#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f8d40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524800; const P1: u64 = 0x142645790; const P2: u64 = 0x143931c90; const P3: u64 = 0xf8; const P4: u64 = 0x142645748; const P5: u64 = 0x100; const P6: u64 = 0x30; const P7: u64 = 0x141524860; const P8: u64 = 0x141524850;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu32(P2, ((rax as u32)) as u32);
    rdx = 0xffffffff;
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp);
    fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P5))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if (((fl_l as u8)) != ((fl_r as u8))) { rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    loop { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; rdx = 0xffffffff; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P5))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if !((((fl_l as u8)) == ((fl_r as u8)))) { break; } }
    rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp);
    rax = 0;
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f8dc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x154; const P2: u64 = 0x130; const P3: u64 = 0x8; const P4: u64 = 0x18; const P5: u64 = 0x28; const P6: u64 = 0x38; const P7: u64 = 0x48; const P8: u64 = 0x138; const P9: u64 = 0x148; const P10: u64 = 0x150; const P11: u64 = 0x140; const P12: u64 = 0x1415242e0; const P13: u64 = 0x1426452a8; const P14: u64 = 0x1415b6870; const P15: u64 = 0x1415bbdf0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rbx = rcx; rcx = ((rcx).wrapping_add(0x108u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rsi); rsi = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P3))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(P4))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P5))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(P6))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P7))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm0) as u128); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P2), rax); fl_l = rdx; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(P8))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rbx.wrapping_add(P8), rsi); bb = 5; continue; }
            5 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(P9))) as u64; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P10))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x58u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P11))) as u64; crate::simd_ops::wu64(rdx, rax); crate::simd_ops::wu64(rbx.wrapping_add(0x140u64), rdx); bb = 9; continue; }
            8 => { rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::lk_sub_u32(rbx.wrapping_add(0x154u64), (1) as u32)) as u32 as u64; bb = 10; continue; }
            10 => { rcx = rbx.wrapping_add(0x108); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P13), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rsp.wrapping_add(0x20); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P15, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx.wrapping_add(0x108); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x154u64))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 11; continue; }
            11 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; bb = 12; continue; }
            12 => { rcx = rbx.wrapping_add(0x108); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P13), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rax = 0x1; rsp = ((rsp).wrapping_add(0x70u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f8f00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415b7470; const P1: u64 = 0x141524330; const P2: u64 = 0x143931c88; const P3: u64 = 0x142645290; const P4: u64 = 0x143931bc8; const P5: u64 = 0x1415f8d40; const P6: u64 = 0x1426456a0; const P7: u64 = 0x14268e078; const P8: u64 = 0x14152c1e0; const P9: u64 = 0x143931bc0; const P10: u64 = 0x1426456b0; const P11: u64 = 0x1426455b0; const P12: u64 = 0x143931bc4; const P13: u64 = 0x1426456b8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 19; continue; } bb = 1; continue; }
            1 => { rdx = 0x370; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x148u64), rax); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = ((rdx.wrapping_add(0x34u64)) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); rcx = rax.wrapping_add(0x30); rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(0x150u64), (0xau32) as u32); r8 = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x140u64), rax); bb = 4; continue; }
            4 => { rax = rcx.wrapping_sub(0x28); r9 = rdx; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu32(rax, (0xffffffffu32) as u32); crate::simd_ops::wu64(rcx.wrapping_sub(0x24u64), rdi); crate::simd_ops::wu64(rcx.wrapping_sub(0x18u64), rdi); crate::simd_ops::wu8(rcx.wrapping_sub(0x10u64), ((rdi as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_sub(0x8u64), rdi); crate::simd_ops::wu64(rcx, rdi); bb = 6; continue; }
            6 => { rax = rcx.wrapping_add(0x28); r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu64(rdx, rax); rcx = ((rcx).wrapping_add(0x58u64)) as u64; rdx = ((rdx).wrapping_add(0x58u64)) as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x150u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 4; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(r9, rdi); fl_l = (crate::simd_ops::ru64(P2)) as u64; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 8; continue; }
            8 => { r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rbx.wrapping_add(0x100u64), ((rdi as u8)) as u8); r8 = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0xf8u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru32(P4)) as u32 as u64; rax = rsp.wrapping_add(0x48); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); r8 = P5; r9 = rbx; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rdi as u32)) as u32); rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P2, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; r8 = P7; rcx = rax; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(P9)) as u32 as u64; rcx = (crate::simd_ops::ru64(P2)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 14; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru32(P9)) as u32 as u64; fl_r = 0x10000; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(P2)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P11), rax), rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { crate::simd_ops::wu64(P2, rdi); rax = 0x2; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            14 => { rax = (crate::simd_ops::ru32(P12)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(P2)) as u64; rdx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P13), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru64(P2)) as u64; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 18; continue; } bb = 17; continue; }
            17 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = 0x2; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            18 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = 0x1; bb = 19; continue; }
            19 => { rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f90b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645290; const P1: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    r9 = 0;
    fl_l = 0; fl_r = 0;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    r8 = 0;
    fl_l = 0; fl_r = 0;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rbx.wrapping_add(P1), rax);
    fl_l = 0; fl_r = rax;
    rax = (0u64.wrapping_sub(fl_r as u64)) as u64;
    rax = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64;
    rax = (((rax as u32)).wrapping_add(0x2u32)) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f90f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x141524330; const P2: u64 = 0x141613850; const P3: u64 = 0x1415242e0; const P4: u64 = 0x1401de410; const P5: u64 = 0x142645560; const P6: u64 = 0x1415bbdf0; const P7: u64 = 0x1426452a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; r14 = rdx; rcx = ((rcx).wrapping_add(0x108u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x140u64))) as u64; rsi = 0; fl_l = 0; fl_r = 0; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rdx = ((rsi.wrapping_add(0x58u64)) as u32) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 2; continue; }
            2 => { rcx = rax.wrapping_add(0x8); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu32(rcx, (0xffffffffu32) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x4u64), rsi); crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rsi); crate::simd_ops::wu8(rcx.wrapping_add(0x18u64), ((rsi as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), rsi); crate::simd_ops::wu64(rcx.wrapping_add(0x28u64), rsi); bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x140u64))) as u64; crate::simd_ops::wu64(r8, rax); crate::simd_ops::wu64(rdi.wrapping_add(0x140u64), r8); bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x138u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rdi.wrapping_add(0x130u64), r8); bb = 8; continue; }
            7 => { crate::simd_ops::wu64(rax, r8); bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x140u64))) as u64; rbp = 0x34; crate::simd_ops::wu64(rdi.wrapping_add(0x138u64), rcx); rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x140u64), rax); crate::simd_ops::wu64(rcx, rsi); rcx = 0x1; crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x154u64), (1) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x138u64))) as u64; xmm0 = (crate::simd_ops::ru128(r14)) as u128; fl_l = ((rax).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; rbp = (if (((fl_l as u64)) != 0) { (rcx as u32) } else { (rbp as u32) }) as u32 as u64; crate::simd_ops::wu128(rax, (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(r14.wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(rax.wrapping_add(0x10u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(r14.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rax.wrapping_add(0x20u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(r14.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rax.wrapping_add(0x30u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(r14.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(rax.wrapping_add(0x40u64), (xmm0) as u128); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru64(r14.wrapping_add(0x20u64))) as u64; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 23; continue; } bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x28u64))) as u64; rcx = rdi.wrapping_add(0xf0); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 24; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x154u64))) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub(1)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x154u64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 26; continue; } bb = 12; continue; }
            12 => { r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x130u64))) as u64; rdx = r8; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rsi = rdx; rdx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = (((rcx as u32)).wrapping_add(0xffffffffu32)) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 13; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rdi.wrapping_add(0x130u64), rax); bb = 17; continue; }
            16 => { crate::simd_ops::wu64(rsi, rax); bb = 17; continue; }
            17 => { fl_l = rdx; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x138u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 19; continue; } bb = 18; continue; }
            18 => { crate::simd_ops::wu64(rdi.wrapping_add(0x138u64), rsi); bb = 19; continue; }
            19 => { r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x148u64))) as u64; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 22; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x150u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x58u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 22; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x140u64))) as u64; crate::simd_ops::wu64(rdx, rax); fl_l = (crate::simd_ops::lk_sub_u32(rdi.wrapping_add(0x154u64), (1) as u32)) as u32 as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x140u64), rdx); bb = 26; continue; }
            22 => { rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::lk_sub_u32(rdi.wrapping_add(0x154u64), (1) as u32)) as u32 as u64; bb = 26; continue; }
            23 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 26; continue; } bb = 24; continue; }
            24 => { rdx = r14; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0xf8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 27; continue; }
            25 => { rbp = 0x34; bb = 26; continue; }
            26 => { rcx = r14.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 27; continue; }
            27 => { rcx = rdi.wrapping_add(0x108); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = ((rbp as u32)) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f9310(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x100; const P1: u64 = 0xf8; const P2: u64 = 0x143931c88; const P3: u64 = 0x142645560; const P4: u64 = 0x142645748; const P5: u64 = 0x1426455b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x1u8) as u8);
    rbx = ((rcx.wrapping_add(P1)) as u64) as u64;
    fl_l = (crate::simd_ops::ru64(P2)) as u64; fl_r = (0x0u64) as u64;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P2)) as u64; rdx = 0xffffffff; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P2)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P2, (0x0u64) as u64); }
    rcx = (crate::simd_ops::ru64(rbx)) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); }
    crate::simd_ops::wu64(rbx, (0x0u64) as u64);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f9380(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415242e0; const P1: u64 = 0x1415bbdf0; const P2: u64 = 0x1415bbe30;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; r14 = 0; fl_l = 0; fl_r = 0; rbx = rcx.wrapping_add(0x130); rbp = rcx; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x154u64))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(0x8u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(0x18u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(0x28u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(0x38u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(0x48u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm0) as u128); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rbx, rax); fl_l = rdx; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), r14); bb = 5; continue; }
            5 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x58u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rdx, rax); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); bb = 9; continue; }
            8 => { rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::lk_sub_u32(rbx.wrapping_add(0x24u64), (1) as u32)) as u32 as u64; bb = 10; continue; }
            10 => { rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x154u64))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 11; continue; }
            11 => { bb = 12; continue; }
            12 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rbx, rax); fl_l = rdx; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), r14); bb = 15; continue; }
            15 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 18; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x58u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 18; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rdx, rax); fl_l = (crate::simd_ops::lk_sub_u32(rbx.wrapping_add(0x24u64), (1) as u32)) as u32 as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); bb = 12; continue; }
            18 => { rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::lk_sub_u32(rbx.wrapping_add(0x24u64), (1) as u32)) as u32 as u64; bb = 12; continue; }
            19 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 20; continue; }
            20 => { crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rsi); bb = 21; continue; }
            21 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rdi = rbx.wrapping_add(0x20); rsi = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x58u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 24; continue; } bb = 23; continue; }
            23 => { rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 24; continue; }
            24 => { rdx = rsi; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 21; continue; } bb = 25; continue; }
            25 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; bb = 27; continue; }
            26 => { rdi = rbx.wrapping_add(0x20); bb = 27; continue; }
            27 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 28; continue; }
            28 => { rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), r14); bb = 29; continue; }
            29 => { rcx = rbp; crate::simd_ops::wu32(rdi, ((r14 as u32)) as u32); crate::simd_ops::wu64(rbx, r14); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), r14); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), r14); r11 = rsp.wrapping_add(0x70); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f9520(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1426455b0; const P2: u64 = 0x142645748;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = ((r8 as u32)) as u32 as u64;
    rbx = rdx;
    fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x1;
    if !((((fl_l as u32)) == ((fl_r as u32)))) { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = rcx; if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); } crate::simd_ops::wu64(rbx.wrapping_add(P0), (0x0u64) as u64); rax = ((rdi as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    rdx = 0xffffffff;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); }
    crate::simd_ops::wu64(rbx.wrapping_add(P0), (0x0u64) as u64);
    rax = (crate::simd_ops::ru32(rbx)) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f9590(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x48; const P2: u64 = 0x4; const P3: u64 = 0x141524330; const P4: u64 = 0x141604ac0; const P5: u64 = 0x8; const P6: u64 = 0x58; const P7: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = rcx; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rdx = ((rax.wrapping_add(P1)) as u32) as u32 as u64; rcx = ((rax.wrapping_add(P2)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 2; continue; }
            2 => { rcx = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rdi.wrapping_add(P0), rax); rcx = rbx; crate::simd_ops::wu64(rax.wrapping_add(P5), rdi); rax = (crate::simd_ops::ru64(rax)) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P6))) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0x4; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi.wrapping_add(P0), (0x0u64) as u64); bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 6; continue; }
            6 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            8 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f9640(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14268e090; const P1: u64 = 0x142677588; const P2: u64 = 0x8; const P3: u64 = 0x14268e308; const P4: u64 = 0x10; const P5: u64 = 0x14268e330; const P6: u64 = 0x28; const P7: u64 = 0x108; const P8: u64 = 0x18; const P9: u64 = 0x38; const P10: u64 = 0x13d; const P11: u64 = 0x1415f9e60; const P12: u64 = 0x1415242e0; const P13: u64 = 0x141649250; const P14: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = P0; rbx = rcx; crate::simd_ops::wu64(rcx, rax); rdi = ((rdx as u32)) as u32 as u64; rax = P1; crate::simd_ops::wu64(rcx.wrapping_add(P2), rax); rax = P3; crate::simd_ops::wu64(rcx.wrapping_add(P4), rax); rax = P5; crate::simd_ops::wu64(rcx.wrapping_add(P6), rax); rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P7))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P8))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P7))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P9))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_and_u8(rbx.wrapping_add(P10), (0xf7u8) as u8); rcx = rbx; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x108u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x110u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(0x118u64), (0x0u32) as u32); rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x110u64), (0x0u64) as u64); crate::simd_ops::wu32(rbx.wrapping_add(0x11cu64), (0x0u32) as u32); bb = 6; continue; }
            6 => { rcx = rbx; rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = 0x140; rcx = rbx; rax = crate::simd_ops::call_stk(P14, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f9720(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415fab80; const P1: u64 = 0x141620b90; const P2: u64 = 0x1439310e8; const P3: u64 = 0x14268a040; const P4: u64 = 0x1434269f8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x88u64))) as u64; rdi = rcx; rsi = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; rbx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsi.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 2; continue; }
            2 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsi.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 18; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; rbx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rax as u32)) as u32); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 4; continue; }
            4 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 5; continue; }
            5 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x24u64), (1) as u32)) as u32 as u64; bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rbx, rdi); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { rcx = ((rcx).wrapping_add(0x10u64)) as u64; rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru16(rdi.wrapping_add(0x134u64))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; rdx = ((crate::simd_ops::ru8(rbx.wrapping_add(0x12u64))) as u32) as u32 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 13; continue; } bb = 11; continue; }
            11 => { fl_l = ((((rdx as u8)) | (0x3u8))) as u8 as u64; fl_r = fl_l; rdx = (rdx & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); rax = 0x1; crate::simd_ops::wu8(rbx.wrapping_add(0x12u64), ((rdx as u8)) as u8); crate::simd_ops::wu16(rbx.wrapping_add(0x10u64), ((rax as u16)) as u16); bb = 12; continue; }
            12 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            13 => { rdx = (rdx & !0xffu64) | ((((((rdx as u8)) & (0xfdu8))) as u8 as u64)); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rbx.wrapping_add(0x12u64), ((rdx as u8)) as u8); r8 = ((crate::simd_ops::ru16(rdi.wrapping_add(0x138u64))) as u32) as u32 as u64; fl_l = ((r8 as u16)) as u16 as u64; fl_r = (crate::simd_ops::ru16(rdi.wrapping_add(0x136u64))) as u16 as u64; r8 = (r8 & !0xffffu64) | ((((fl_l as u16).wrapping_sub(fl_r as u16)) as u16 as u64)); if (((fl_l as u16)) == ((fl_r as u16))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(P2)) as u64; rcx = (0x5851f42d4c957f2du64) as u64; rax = (((rax) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(P2, rax); rdx = ((crate::simd_ops::ru8(rbx.wrapping_add(0x12u64))) as u32) as u32 as u64; fl_l = ((rax).wrapping_shr(33u32)) as u64; rax = ((fl_l as u64)) as u64; xmm1 = (((rax as u32)) as u32) as u128; xmm1 = crate::simd_ops::vcvtdq2pd(xmm1); rax = ((((r8 as u16)) as i16 as i32) as u32) as u32 as u64; xmm0 = (((rax as u32)) as u32) as u128; xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(crate::simd_ops::ru64(P3))).to_bits() as u128); xmm0 = crate::simd_ops::vcvtdq2pd(xmm0); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm0 as u64)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) + f64::from_bits(crate::simd_ops::ru64(P4))).to_bits() as u128); rcx = (crate::simd_ops::cvt_f64_i32((f64::from_bits(xmm1 as u64)))) as u32 as u64; bb = 15; continue; }
            15 => { rax = ((crate::simd_ops::ru16(rdi.wrapping_add(0x136u64))) as u32) as u32 as u64; rax = (rax & !0xffffu64) | (((((rax as u16)).wrapping_add(crate::simd_ops::ru16(rdi.wrapping_add(0x134u64)))) as u16 as u64)); rax = (rax & !0xffffu64) | (((((rax as u16)).wrapping_add((rcx as u16))) as u16 as u64)); fl_l = ((rax as u16)) as u16 as u64; fl_r = (0x1u16) as u16 as u64; if ((((fl_l as u16)) as i16) > (((fl_r as u16)) as i16)) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = 0x1; fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; bb = 17; continue; }
            17 => { rcx = (rcx & !0xffu64) | (((if ((((fl_l as u16)) as i16) > (((fl_r as u16)) as i16)) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu16(rbx.wrapping_add(0x10u64), ((rax as u16)) as u16); rdx = (rdx & !0xffu64) | ((((((rdx as u8)) & (0xfeu8))) as u8 as u64)); rax = rbx; rcx = (rcx & !0xffu64) | ((((((rcx as u8)) | ((rdx as u8)))) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x12u64), ((rcx as u8)) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            18 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = 0; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = ((rcx as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f98b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x13c; const P1: u64 = 0x13a; const P2: u64 = 0x1415cd4e0; const P3: u64 = 0x1415cd500;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x30u8) as u8 as u64; rbx = (((r8 as u16)) as u32) as u32 as u64; rdi = rdx; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rdx = (((rbx as u16)) as u32) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { rdx = (((rbx as u16)) as u32) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = (((rbx as u16)) as u32) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f9930(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524330; const P1: u64 = 0x1416491f0; const P2: u64 = 0x14268e090; const P3: u64 = 0x142677588; const P4: u64 = 0x14268e308; const P5: u64 = 0x14268e330; const P6: u64 = 0x14027d060; const P7: u64 = 0x141649350; const P8: u64 = 0x141649330; const P9: u64 = 0x141536a70; const P10: u64 = 0x1415bd1d0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = ((rcx as u32)) as u32 as u64; rdx = 0x140; rcx = 0x2; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 1; continue; }
            1 => { rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_and_u8(rbx.wrapping_add(0x13eu64), (0xfeu8) as u8); rax = P2; crate::simd_ops::lk_and_u8(rbx.wrapping_add(0x13cu64), (0xc0u8) as u8); rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx, rax); rsi = (rsi & !0xffu64) | ((((((rsi as u8)) & (0x7u8))) as u8 as u64)); rax = P3; crate::simd_ops::wu64(rbx.wrapping_add(0x108u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); rsi = (rsi & !0xffu64) | ((((((rsi as u8)) | (0x90u8))) as u8 as u64)); rax = P4; crate::simd_ops::wu64(rbx.wrapping_add(0x110u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rax = P5; crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), rax); rax = 0x1; crate::simd_ops::wu16(rbx.wrapping_add(0x134u64), ((rax as u16)) as u16); crate::simd_ops::wu64(rbx.wrapping_add(0x118u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x120u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x128u64), rdi); crate::simd_ops::wu32(rbx.wrapping_add(0x130u64), ((rdi as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x136u64), ((rdi as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x13du64), ((rsi as u8)) as u8); crate::simd_ops::wu16(rbx.wrapping_add(0x13au64), ((rdi as u16)) as u16); rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x70u64))) as u64; rax = P6; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rcx = rbx; rax = crate::simd_ops::call_stk(rdx, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xa; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (rcx & !0xffu64) | (((0x80u8) as u8 as u64)); bb = 6; continue; }
            5 => { rcx = (rcx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0xd4u64))) as u32) as u32 as u64; rdx = 0x18; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7fu8))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) | ((rcx as u8)))) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0xd4u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x13du64))) as u32) as u32 as u64; rcx = ((rdx.wrapping_sub(0x16u64)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { rcx = rax; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            9 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = rax; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            11 => { rax = rdi; bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rbx.wrapping_add(0x108u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 13; continue; }
            13 => { rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            15 => { rcx = rbx.wrapping_add(0x10); rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 20; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; rax = P9; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 18; continue; } bb = 17; continue; }
            17 => { rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            18 => { rax = crate::simd_ops::call_stk(rdx, rax, rcx, rdx, r8, r9, rsp); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            19 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 20; continue; }
            20 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = rdi; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f9b00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524330; const P1: u64 = 0x141649730; const P2: u64 = 0x1415beae0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; fl_l = (crate::simd_ops::ru32(r9)) as u32 as u64; fl_r = 0x1; rbx = r9; rdi = r8; rsi = rdx; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdx = 0x3e0; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { r9 = rdi; r8 = rsi; rcx = rax; rdx = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f9b80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524330; const P1: u64 = 0x1415ccd20; const P2: u64 = 0x13a; const P3: u64 = 0x1415cd430; const P4: u64 = 0x8; const P5: u64 = 0x13d; const P6: u64 = 0x108; const P7: u64 = 0x60; const P8: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (((rdx as u16)) as u32) as u32 as u64; rdi = rcx; rdx = 0x38; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdx = (((rbx as u16)) as u32) as u32 as u64; rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rdx = ((crate::simd_ops::ru16(rdi.wrapping_add(P2))) as u32) as u32 as u64; rcx = rax; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rax = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(P5))) as u8 as u64; fl_r = (0x8u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P6))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P7))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbx.wrapping_add(P8), ((rax as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(P4), ((rax as u32)) as u32); bb = 7; continue; }
            7 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f9c10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524330; const P1: u64 = 0x1415ccd60;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rdx = 0x10;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = rax; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f9c40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f78; const P1: u64 = 0x1415f3790; const P2: u64 = 0x141524330; const P3: u64 = 0x1415ccd60; const P4: u64 = 0x1415f9b80; const P5: u64 = 0x1415242e0; const P6: u64 = 0x1415fa260; const P7: u64 = 0x1415fa180;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; r9 = r11.wrapping_add(0x20); r12 = 0; fl_l = 0; fl_r = 0; rsi = rdx; rdi = rcx; crate::simd_ops::wu32(r11.wrapping_add(0x18u64), ((r12 as u32)) as u32); rdx = r11.wrapping_add(0x10); crate::simd_ops::wu8(r11.wrapping_add(0x10u64), ((r12 as u8)) as u8); rcx = rsi; r8 = ((r12.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 25; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rsi)) as u64; r9 = rsp.wrapping_add(0x88); r8 = ((r12.wrapping_add(0x4u64)) as u32) as u32 as u64; rcx = rsi; rdx = rsp.wrapping_add(0x80); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 25; continue; } bb = 2; continue; }
            2 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x13du64))) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x78u64))) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 25; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x108u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r15 = ((rax as u32)) as u32 as u64; bb = 6; continue; }
            5 => { r15 = (((r12 as u16)) as u32) as u32 as u64; bb = 6; continue; }
            6 => { rbp = ((r12 as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x80u64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 23; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rsi)) as u64; r9 = rsp.wrapping_add(0x88); r8 = 0x8; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (0xffffffffffffffffu64) as u64); rdx = rsp.wrapping_add(0x20); rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 25; continue; } bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rbx = r12; fl_l = rdx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 17; continue; } bb = 10; continue; }
            10 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x13du64))) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 13; continue; } bb = 11; continue; }
            11 => { rdx = ((rbx.wrapping_add(0x10u64)) as u32) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 12; continue; }
            12 => { rcx = rax; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            13 => { rdx = (((r15 as u16)) as u32) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rax)) as u64; r8 = (((r15 as u16)) as u32) as u32 as u64; rdx = rsi; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; r14 = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r8)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = ((r14 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 22; continue; } bb = 16; continue; }
            16 => { rax = ((r14 as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            17 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x13du64))) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            19 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x108u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r8 = rbx; rcx = rdi; rdx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            20 => { rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru64(rax)) as u64; r8 = (((r15 as u16)) as u32) as u32 as u64; rdx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 26; continue; } bb = 22; continue; }
            22 => { rbp = (((rbp as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsp.wrapping_add(0x80u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 7; continue; } bb = 23; continue; }
            23 => { rax = 0x1; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            24 => { rax = 0x34; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            25 => { rax = 0x2; bb = 26; continue; }
            26 => { rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f9e60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x110; const P1: u64 = 0x118; const P2: u64 = 0x8; const P3: u64 = 0x120;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = rcx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; rbx = ((rbx).wrapping_add(0x10u64)) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(P0)))) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(P1), ((rbx as u32)) as u32); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(P3), rbx); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f9ee0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415936b0; const P1: u64 = 0x141594710; const P2: u64 = 0x1415fb140; const P3: u64 = 0x14159a6d0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x230u64))) as u64; r13 = rdx; crate::simd_ops::wu64(rcx.wrapping_add(0x230u64), rax); r12 = rcx; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x238u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x238u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(r13.wrapping_add(0xc8u64))) as u128; crate::simd_ops::wu128(r12.wrapping_add(0xc8u64), (xmm0) as u128); rax = (crate::simd_ops::ru32(r13.wrapping_add(0xd8u64))) as u32 as u64; crate::simd_ops::wu32(r12.wrapping_add(0xd8u64), ((rax as u32)) as u32); rbx = (crate::simd_ops::ru32(r13.wrapping_add(0xc0u64))) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r12.wrapping_add(0xc0u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(r12.wrapping_add(0xc4u64))) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = ((rbx as u32)) as u32 as u64; rcx = r12.wrapping_add(0xb8); rdx = (((rdx as u32)).wrapping_sub((rax as u32))) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu32(r12.wrapping_add(0xc0u64), ((rbx as u32)) as u32); rdx = 0; fl_l = 0; fl_r = 0; r8 = rbx; bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(r13.wrapping_add(0xb8u64))) as u64; rdx = rdx.wrapping_add(0x18); rax = (crate::simd_ops::ru64(r12.wrapping_add(0xb8u64))) as u64; xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(rcx).wrapping_sub(0x18u64))) as u128; crate::simd_ops::wu128(rdx.wrapping_add(rax).wrapping_sub(0x18u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(rcx).wrapping_sub(0x8u64))) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_add(rax).wrapping_sub(0x8u64), ((xmm1 as u64)) as u64); fl_l = r8; fl_r = (0x1u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru32(r13.wrapping_add(0x228u64))) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 19; continue; } bb = 7; continue; }
            7 => { rcx = r12.wrapping_add(0xe0); rdx = ((rbx as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 19; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbp); rbp = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r14); r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r15); bb = 9; continue; }
            9 => { rdi = (crate::simd_ops::ru64(r13.wrapping_add(0x220u64))) as u64; r15 = (crate::simd_ops::ru64(r12.wrapping_add(0x220u64))) as u64; rdi = ((rdi).wrapping_add(r14)) as u64; r15 = ((r15).wrapping_add(r14)) as u64; xmm0 = (crate::simd_ops::ru128(rdi)) as u128; crate::simd_ops::wu128(r15, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(r15.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rax = (crate::simd_ops::ru32(r15.wrapping_add(0x24u64))) as u32 as u64; crate::simd_ops::wu32(r15.wrapping_add(0x20u64), (0x0u32) as u32); rsi = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdx = ((rsi as u32)) as u32 as u64; rcx = r15.wrapping_add(0x18); rdx = (((rdx as u32)).wrapping_sub((rax as u32))) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 17; continue; } bb = 11; continue; }
            11 => { r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(r15.wrapping_add(0x20u64), ((rsi as u32)) as u32); fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 14; continue; } bb = 12; continue; }
            12 => { bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rdx = ((r8.wrapping_mul(8)) as u64) as u64; rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(rax))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(rcx), rax); fl_l = ((r8 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 13; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; rcx = (crate::simd_ops::ru32(r15.wrapping_add(0x20u64))) as u32 as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 15; continue; }
            15 => { bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x14u64), (1) as u32); rdx = (crate::simd_ops::ru32(r15.wrapping_add(0x20u64))) as u32 as u64; rax = ((rax).wrapping_add(0x8u64)) as u64; rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; rdx = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 17; continue; }
            17 => { r14 = ((r14).wrapping_add(0x28u64)) as u64; fl_l = rbp; fl_r = (0x1u64) as u64; rbp = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 18; continue; }
            18 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 19; continue; }
            19 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415fa0e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xffffffffffffffff;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdi); r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; r11 = 0; fl_l = 0; fl_r = 0; r10 = rdx; rdi = rcx; r9 = ((r11 as u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rcx)) as u64; bb = 2; continue; }
            2 => { rax = ((r8 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; rax = (((rax as u32)).wrapping_add((r9 as u32))) as u32 as u64; rdx = ((((rax as u32)) as i32 as i64) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rbx)) as u64; rcx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rcx; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = rcx; fl_r = r10; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { (r9 as u32) } else { (rax as u32) }) as u32 as u64; r9 = ((rax as u32)) as u32 as u64; rax = ((r8.wrapping_add(P1)) as u32) as u32 as u64; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { (r8 as u32) } else { (rax as u32) }) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; fl_l = (((r8 as u32)).wrapping_shr(1u32)) as u32 as u64; r8 = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rdx = ((((r9 as u32)) as i32 as i64) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; r11 = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { rdx } else { r11 }) as u64; rax = r11; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; return rax; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; r11 = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { rdx } else { r11 }) as u64; rax = r11; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415fa180(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x13d; const P1: u64 = 0x120; const P2: u64 = 0x1415f9b80; const P3: u64 = 0x1415fa0e0; const P4: u64 = 0x1415f9590; const P5: u64 = 0x1416058a0; const P6: u64 = 0x110; const P7: u64 = 0x1415fb990; const P8: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = 0; fl_l = 0; fl_r = 0; rsi = r8; rbp = (((rdx as u16)) as u32) as u32 as u64; rdi = rcx; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if ((((fl_l as u8)) as i8) >= (((fl_r as u8)) as i8)) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(P1), rax); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 4; continue; }
            4 => { rbx = rax; bb = 14; continue; }
            5 => { rdx = rsi; rcx = ((rcx).wrapping_add(0x110u64)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 13; continue; } bb = 6; continue; }
            6 => { rdx = (((rbp as u16)) as u32) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 7; continue; }
            7 => { rcx = rsi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { rdx = rdi; rcx = rax; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rdx = rsi; rcx = ((rdi.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rax.wrapping_add(P8), rbx); bb = 14; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P8))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rax = 0; fl_l = 0; fl_r = 0; bb = 15; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rax.wrapping_add(P8))) as u64; bb = 14; continue; }
            14 => { rax = rbx; bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415fa260(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524330; const P1: u64 = 0x1415ccd60; const P2: u64 = 0x1415fa0e0; const P3: u64 = 0x1415f9590; const P4: u64 = 0x1416058a0; const P5: u64 = 0x1415fb990;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x13du64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rsi = rdx; rdi = rcx; if ((((fl_l as u8)) as i8) >= (((fl_r as u8)) as i8)) { bb = 5; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x120u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 15; continue; } bb = 2; continue; }
            2 => { rdx = ((rbx.wrapping_add(0x10u64)) as u32) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = rax; crate::simd_ops::wu64(rdi.wrapping_add(0x120u64), rax); bb = 15; continue; }
            4 => { rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi.wrapping_add(0x120u64), rbx); bb = 15; continue; }
            5 => { rcx = ((rcx).wrapping_add(0x110u64)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 6; continue; }
            6 => { rdx = ((rax.wrapping_add(0x10u64)) as u32) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 7; continue; }
            7 => { rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 8; continue; }
            8 => { rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { rdx = rdi; rcx = rax; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rdx = rsi; rcx = rdi.wrapping_add(0x110); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); bb = 15; continue; }
            12 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { rax = 0; fl_l = 0; fl_r = 0; bb = 16; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; bb = 15; continue; }
            15 => { rax = rbx; bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415fa360(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415fa260; const P1: u64 = 0x1415f98b0; const P2: u64 = 0x1415fc1f0; const P3: u64 = 0x1415fa180; const P4: u64 = 0x1415fb3a0; const P5: u64 = 0x143930f88; const P6: u64 = 0x1426452b0; const P7: u64 = 0x1426452a8; const P8: u64 = 0x142645560;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r13 = 0; fl_l = 0; fl_r = 0; r15 = rcx; crate::simd_ops::wu16(r8, ((r13 as u16)) as u16); r14 = r9; crate::simd_ops::wu32(r9, ((r13 as u32)) as u32); rdi = rdx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x108u64))) as u64; rbx = ((r13 as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rsi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 89; continue; } bb = 1; continue; }
            1 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 63; continue; } bb = 2; continue; }
            2 => { rax = ((crate::simd_ops::ru8(r15.wrapping_add(0x13du64))) as u32) as u32 as u64; rbp = 0; fl_l = 0; fl_r = 0; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((r13 as u32)) as u32); r12 = (r12 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbp); r14 = (((r13 as u16)) as u32) as u32 as u64; rcx = r15; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 3; continue; }
            3 => { rdx = rdi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r13 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 4; continue; }
            4 => { rcx = 0x1; crate::simd_ops::wu8(rsp.wrapping_add(0x70u64), ((rcx as u8)) as u8); fl_l = ((r12 as u8)) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 5; continue; }
            5 => { r14 = (r14 & !0xffffu64) | (((((r14 as u16)).wrapping_add(1)) as u16 as u64)); rax = (((r14 as u16)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = 0; fl_l = 0; fl_r = 0; r14 = ((rax as u32)) as u32 as u64; bb = 7; continue; }
            7 => { r8 = (((r14 as u16)) as u32) as u32 as u64; rdx = rbp; rcx = r15; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rsp.wrapping_add(0x70u64), ((rax as u8)) as u8); rdi = (((rax as u8)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 55; continue; } bb = 8; continue; }
            8 => { r8 = (((r14 as u16)) as u32) as u32 as u64; rdx = rbp; rcx = r15; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 22; continue; }
            9 => { r8 = rdi; rdx = (((rsi as u16)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 10; continue; }
            10 => { rax = 0; fl_l = 0; fl_r = 0; bb = 90; continue; }
            11 => { rax = ((crate::simd_ops::ru8(r15.wrapping_add(0x13du64))) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 21; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru8(r13.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 18; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x108u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (((crate::simd_ops::ru16(r13.wrapping_add(0xau64))) as i16 as i32) as u32) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 17; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru8(r15.wrapping_add(0x13du64))) as u8 as u64; fl_r = (0x20u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { fl_l = (((rdx as u16)).wrapping_sub(1)) as u16 as u64; rdx = (rdx & !0xffffu64) | ((((fl_l as u16)) as u16 as u64)); crate::simd_ops::wu8(r13.wrapping_add(0x8u64), (0x0u8) as u8); crate::simd_ops::wu16(r13.wrapping_add(0xau64), ((rdx as u16)) as u16); r14 = (((rdx as u16)) as u32) as u32 as u64; bb = 23; continue; }
            16 => { rax = 0; fl_l = 0; fl_r = 0; rdx = ((rax as u32)) as u32 as u64; crate::simd_ops::wu16(r13.wrapping_add(0xau64), ((rdx as u16)) as u16); r14 = (((rdx as u16)) as u32) as u32 as u64; bb = 23; continue; }
            17 => { fl_l = (((rdx as u16)).wrapping_add(1)) as u16 as u64; rdx = (rdx & !0xffffu64) | ((((fl_l as u16)) as u16 as u64)); crate::simd_ops::wu16(r13.wrapping_add(0xau64), ((rdx as u16)) as u16); r14 = (((rdx as u16)) as u32) as u32 as u64; bb = 23; continue; }
            18 => { rax = ((crate::simd_ops::ru16(r13.wrapping_add(0xau64))) as u32) as u32 as u64; fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) != 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::wu16(r13.wrapping_add(0xau64), ((rcx as u16)) as u16); rdx = (((rcx as u16)) as u32) as u32 as u64; crate::simd_ops::wu8(r13.wrapping_add(0x8u64), (0x1u8) as u8); r14 = (((rcx as u16)) as u32) as u32 as u64; bb = 23; continue; }
            20 => { fl_l = (((rax as u16)).wrapping_sub(1)) as u16 as u64; rax = (rax & !0xffffu64) | ((((fl_l as u16)) as u16 as u64)); crate::simd_ops::wu16(r13.wrapping_add(0xau64), ((rax as u16)) as u16); rdx = (((rax as u16)) as u32) as u32 as u64; r14 = (((rax as u16)) as u32) as u32 as u64; bb = 23; continue; }
            21 => { r9 = 0; fl_l = 0; fl_r = 0; r8 = rsp.wrapping_add(0x70); rdx = rbp; rcx = r15; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdi = ((crate::simd_ops::ru8(rsp.wrapping_add(0x70u64))) as u32) as u32 as u64; r14 = (((rax as u16)) as u32) as u32 as u64; bb = 22; continue; }
            22 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 60; continue; } bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x108u64))) as u64; rdx = (((r14 as u16)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbp = ((rax as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; crate::simd_ops::wu32(rax, ((rbp as u32)) as u32); rbx = (crate::simd_ops::ru64(P5)) as u64; bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 35; continue; } bb = 25; continue; }
            25 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 26; continue; }
            26 => { rsi = rbx.wrapping_add(0x8); bb = 27; continue; }
            27 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 38; continue; } bb = 28; continue; }
            28 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbp as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 29; continue; }
            29 => { bb = 30; continue; }
            30 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 37; continue; } bb = 31; continue; }
            31 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 30; continue; } bb = 32; continue; }
            32 => { rbp = 0; fl_l = 0; fl_r = 0; rdi = ((rbp as u32)) as u32 as u64; bb = 33; continue; }
            33 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 39; continue; } bb = 34; continue; }
            34 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 39; continue; }
            35 => { rsi = rbx.wrapping_add(0x8); rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 27; continue; } bb = 36; continue; }
            36 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); bb = 24; continue; }
            37 => { rbp = 0; fl_l = 0; fl_r = 0; bb = 33; continue; }
            38 => { rbp = 0; fl_l = 0; fl_r = 0; rdi = ((rbp as u32)) as u32 as u64; bb = 39; continue; }
            39 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 47; continue; } bb = 40; continue; }
            40 => { bb = 41; continue; }
            41 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 44; continue; } bb = 42; continue; }
            42 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 41; continue; } bb = 43; continue; }
            43 => { bb = 48; continue; }
            44 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 49; continue; } bb = 45; continue; }
            45 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 49; continue; } bb = 46; continue; }
            46 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); bb = 49; continue; }
            47 => { rsi = rbx.wrapping_add(0x8); bb = 48; continue; }
            48 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); bb = 49; continue; }
            49 => { fl_l = rdi; fl_r = rdi; rbx = rdi.wrapping_sub(0x10); rbx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbp } else { rbx }) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 54; continue; } bb = 50; continue; }
            50 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x78u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 62; continue; } bb = 51; continue; }
            51 => { rcx = rbx.wrapping_add(0x10); rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 53; continue; } bb = 52; continue; }
            52 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 53; continue; }
            53 => { rbx = rbp; bb = 54; continue; }
            54 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; bb = 55; continue; }
            55 => { rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x24u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 58; continue; } bb = 56; continue; }
            56 => { fl_l = (crate::simd_ops::ru8(r15.wrapping_add(0x13du64))) as u8 as u64; fl_r = (0x7u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 58; continue; } bb = 57; continue; }
            57 => { rax = ((crate::simd_ops::ru8(r15.wrapping_add(0x13cu64))) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x30u8))) as u8 as u64)); r12 = (((r12 as u8)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x10u8) as u8 as u64; rax = 0x1; r12 = (if (((fl_l as u8)) != ((fl_r as u8))) { (rax as u32) } else { (r12 as u32) }) as u32 as u64; bb = 58; continue; }
            58 => { rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((rcx as u32)) as u32); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 89; continue; } bb = 59; continue; }
            59 => { bb = 4; continue; }
            60 => { fl_l = ((r12 as u8)) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 89; continue; } bb = 61; continue; }
            61 => { bb = 55; continue; }
            62 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu16(rax, ((r14 as u16)) as u16); bb = 89; continue; }
            63 => { rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x108u64))) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(r14, ((rax as u32)) as u32); rbp = ((rax as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(P5)) as u64; bb = 64; continue; }
            64 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 75; continue; } bb = 65; continue; }
            65 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 64; continue; } bb = 66; continue; }
            66 => { rsi = rbx.wrapping_add(0x8); bb = 67; continue; }
            67 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 77; continue; } bb = 68; continue; }
            68 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbp as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 72; continue; } bb = 69; continue; }
            69 => { bb = 70; continue; }
            70 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 73; continue; } bb = 71; continue; }
            71 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 70; continue; } bb = 72; continue; }
            72 => { rdi = r13; bb = 73; continue; }
            73 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 78; continue; } bb = 74; continue; }
            74 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 78; continue; }
            75 => { rsi = rbx.wrapping_add(0x8); rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 67; continue; } bb = 76; continue; }
            76 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); bb = 64; continue; }
            77 => { rdi = r13; bb = 78; continue; }
            78 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 85; continue; } bb = 79; continue; }
            79 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 82; continue; } bb = 80; continue; }
            80 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 79; continue; } bb = 81; continue; }
            81 => { bb = 86; continue; }
            82 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 87; continue; } bb = 83; continue; }
            83 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 87; continue; } bb = 84; continue; }
            84 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); bb = 87; continue; }
            85 => { rsi = rbx.wrapping_add(0x8); bb = 86; continue; }
            86 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); bb = 87; continue; }
            87 => { rbx = rdi.wrapping_sub(0x10); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 89; continue; } bb = 88; continue; }
            88 => { rbx = r13; bb = 89; continue; }
            89 => { rax = rbx; bb = 90; continue; }
            90 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415fa780(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415fa260; const P1: u64 = 0x1415f9c10; const P2: u64 = 0x1415fb720; const P3: u64 = 0x1415fa0e0; const P4: u64 = 0x1415f9b80; const P5: u64 = 0x1415fb3a0; const P6: u64 = 0x143930f88; const P7: u64 = 0x1426452b0; const P8: u64 = 0x1426452a8; const P9: u64 = 0x142645560;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = 0; fl_l = 0; fl_r = 0; rbx = rcx; crate::simd_ops::wu32(r9, ((rbp as u32)) as u32); r12 = r9; crate::simd_ops::wu16(r8, ((rbp as u16)) as u16); r13 = r8; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x108u64))) as u64; rsi = rdx; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r8 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 78; continue; } bb = 1; continue; }
            1 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 50; continue; } bb = 2; continue; }
            2 => { rcx = ((crate::simd_ops::ru8(rbx.wrapping_add(0x13du64))) as u32) as u32 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x50u64), (0x1u8) as u8); fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 8; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(rdi)) as u64; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 4; continue; }
            4 => { rax = (((rcx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rdx = rsi; rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r14 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 6; continue; }
            6 => { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 7; continue; }
            7 => { rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x8u64))) as u32) as u32 as u64; r8 = rsp.wrapping_add(0x50); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rdx = rsi; crate::simd_ops::wu8(rsi.wrapping_add(0x8u64), ((rax as u8)) as u8); rcx = rbx; rax = ((crate::simd_ops::ru16(r14.wrapping_add(0xau64))) as u32) as u32 as u64; crate::simd_ops::wu16(rsi.wrapping_add(0xau64), ((rax as u16)) as u16); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rsi); bb = 28; continue; }
            8 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0x7u8))) as u8 as u64)); fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 23; continue; } bb = 9; continue; }
            9 => { r14 = 0xffffffff; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 13; continue; } bb = 10; continue; }
            10 => { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rax); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x120u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = ((crate::simd_ops::ru16(rcx.wrapping_add(0xau64))) as u32) as u32 as u64; crate::simd_ops::wu16(rdx.wrapping_add(0xau64), ((rax as u16)) as u16); crate::simd_ops::wu16(rcx.wrapping_add(0xau64), ((r14 as u16)) as u16); bb = 13; continue; }
            13 => { r15 = (crate::simd_ops::ru64(rdi)) as u64; r8 = rsp.wrapping_add(0x50); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rdx = r15; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r14 = (((rax as u16)) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x13du64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 29; continue; } bb = 14; continue; }
            14 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((((fl_l as u8)) & ((fl_r as u8))) as i8) >= 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x120u64))) as u64; bb = 17; continue; }
            16 => { rcx = rbx.wrapping_add(0x110); rdx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; bb = 17; continue; }
            17 => { rax = ((crate::simd_ops::ru8(r15.wrapping_add(0x8u64))) as u32) as u32 as u64; rdx = 0x1; crate::simd_ops::wu8(rdi.wrapping_add(0x8u64), ((rax as u8)) as u8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rsi = 0xffffffff; rcx = ((rsi as u32)) as u32 as u64; rcx = (rcx & !0xffffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (rdx as u16) } else { (rcx as u16) }) as u16 as u64)); rcx = (rcx & !0xffffu64) | (((((rcx as u16)).wrapping_add(crate::simd_ops::ru16(r15.wrapping_add(0xau64)))) as u16 as u64)); crate::simd_ops::wu16(rdi.wrapping_add(0xau64), ((rcx as u16)) as u16); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x108u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (((crate::simd_ops::ru16(rdi.wrapping_add(0xau64))) as i16 as i32) as u32) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 20; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x13du64))) as u8 as u64; fl_r = (0x20u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::wu16(rdi.wrapping_add(0xau64), ((rsi as u16)) as u16); bb = 29; continue; }
            20 => { fl_l = ((rdx as u16)) as u16 as u64; fl_r = ((rdx as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) != 0) { bb = 29; continue; } bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x8u64))) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 29; continue; } bb = 22; continue; }
            22 => { crate::simd_ops::wu8(rdi.wrapping_add(0x8u64), (0x1u8) as u8); bb = 29; continue; }
            23 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 27; continue; } bb = 24; continue; }
            24 => { rdx = (((r8 as u16)) as u32) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 26; continue; } bb = 25; continue; }
            25 => { rax = 0; fl_l = 0; fl_r = 0; bb = 79; continue; }
            26 => { crate::simd_ops::wu64(rdi, rax); bb = 27; continue; }
            27 => { r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; r8 = rsp.wrapping_add(0x50); rdx = rax; rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 28; continue; }
            28 => { r14 = (((rax as u16)) as u32) as u32 as u64; bb = 29; continue; }
            29 => { fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x50u64))) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 78; continue; } bb = 30; continue; }
            30 => { crate::simd_ops::wu16(r13, ((r14 as u16)) as u16); rdx = (((r14 as u16)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x108u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(r12, ((rax as u32)) as u32); r14 = ((rax as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(P6)) as u64; bb = 31; continue; }
            31 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 42; continue; } bb = 32; continue; }
            32 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 31; continue; } bb = 33; continue; }
            33 => { rsi = rbx.wrapping_add(0x8); bb = 34; continue; }
            34 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 44; continue; } bb = 35; continue; }
            35 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r14 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 39; continue; } bb = 36; continue; }
            36 => { bb = 37; continue; }
            37 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 40; continue; } bb = 38; continue; }
            38 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 37; continue; } bb = 39; continue; }
            39 => { rdi = rbp; bb = 40; continue; }
            40 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 45; continue; } bb = 41; continue; }
            41 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 45; continue; }
            42 => { rsi = rbx.wrapping_add(0x8); rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 34; continue; } bb = 43; continue; }
            43 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); bb = 31; continue; }
            44 => { rdi = rbp; bb = 45; continue; }
            45 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 74; continue; } bb = 46; continue; }
            46 => { bb = 47; continue; }
            47 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 71; continue; } bb = 48; continue; }
            48 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 47; continue; } bb = 49; continue; }
            49 => { bb = 75; continue; }
            50 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64; fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((((fl_l as u16)) & ((fl_r as u16))) as i16) <= 0) { bb = 78; continue; } bb = 51; continue; }
            51 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x2u64))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 53; continue; } bb = 52; continue; }
            52 => { fl_l = (((rax as u16)).wrapping_sub(1)) as u16 as u64; rax = (rax & !0xffffu64) | ((((fl_l as u16)) as u16 as u64)); crate::simd_ops::wu16(rcx, ((rax as u16)) as u16); bb = 53; continue; }
            53 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x108u64))) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(r12, ((rax as u32)) as u32); r14 = ((rax as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(P6)) as u64; bb = 54; continue; }
            54 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 64; continue; } bb = 55; continue; }
            55 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 54; continue; } bb = 56; continue; }
            56 => { rsi = rbx.wrapping_add(0x8); bb = 57; continue; }
            57 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 66; continue; } bb = 58; continue; }
            58 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r14 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 61; continue; } bb = 59; continue; }
            59 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 62; continue; } bb = 60; continue; }
            60 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 59; continue; } bb = 61; continue; }
            61 => { rdi = rbp; bb = 62; continue; }
            62 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 67; continue; } bb = 63; continue; }
            63 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 67; continue; }
            64 => { rsi = rbx.wrapping_add(0x8); rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 57; continue; } bb = 65; continue; }
            65 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); bb = 54; continue; }
            66 => { rdi = rbp; bb = 67; continue; }
            67 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 74; continue; } bb = 68; continue; }
            68 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 71; continue; } bb = 69; continue; }
            69 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 68; continue; } bb = 70; continue; }
            70 => { bb = 75; continue; }
            71 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 76; continue; } bb = 72; continue; }
            72 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 76; continue; } bb = 73; continue; }
            73 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp); bb = 76; continue; }
            74 => { rsi = rbx.wrapping_add(0x8); bb = 75; continue; }
            75 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); bb = 76; continue; }
            76 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 78; continue; } bb = 77; continue; }
            77 => { rbp = rdi.wrapping_sub(0x10); bb = 78; continue; }
            78 => { rax = rbp; bb = 79; continue; }
            79 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415fab80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524340; const P1: u64 = 0x141524330; const P2: u64 = 0x141620b90; const P3: u64 = 0x141620c30; const P4: u64 = 0x141620bb0; const P5: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rsi = rcx; r15 = ((rdx as u32)) as u32 as u64; rcx = 0x1; r15 = (((r15 as u32)).wrapping_shr(1u32)) as u32 as u64; r15 = (((r15 as u32)).wrapping_add((rdx as u32))) as u32 as u64; r14 = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; r15 = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rcx as u32) } else { (r15 as u32) }) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi)) as u64; r15 = (((r15 as u32)).wrapping_add((rdx as u32))) as u32 as u64; rdx = ((r15.wrapping_add(r15.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(3u32)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = rdx; rdx = rcx; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 13; continue; } bb = 2; continue; }
            2 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rsi)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 5; continue; }
            5 => { fl_l = rax; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 6; continue; }
            6 => { fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rbx = rbp; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); bb = 8; continue; }
            8 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru64(rsi)) as u64; rdi = rbx; rdi = ((rdi).wrapping_sub(rbp)) as u64; rcx = rbx; rdx = ((rdx).wrapping_add(rdi)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi)) as u64; rcx = ((rcx).wrapping_add(rdi)) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x28u64)) as u64; fl_l = r14; fl_r = (0x1u64) as u64; r14 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 11; continue; }
            11 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 12; continue; }
            12 => { rdx = (crate::simd_ops::ru64(rsi)) as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rsi, rbp); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu32(rsi.wrapping_add(0xcu64), ((r15 as u32)) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415fac80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x13d; const P1: u64 = 0x134; const P2: u64 = 0x1415a0660;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x40u8) as u8 as u64;
    if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    fl_l = (crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u16 as u64; fl_r = (0x0u16) as u16 as u64;
    if !((((fl_l as u16)) != ((fl_r as u16)))) { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
    return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415faca0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x98; const P1: u64 = 0x9d; const P2: u64 = 0x2f8; const P3: u64 = 0x1415945c0; const P4: u64 = 0x34; const P5: u64 = 0x10; const P6: u64 = 0x1415bfb60; const P7: u64 = 0x1415c0d60; const P8: u64 = 0x250;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u8 as u64; fl_r = (0x90u8) as u8 as u64; rbx = rdx; rdi = rcx; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) <= ((fl_r as u8))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P2))) as u32) as u32 as u64; rcx = rdx; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x1u8))) as u8 as u64)); crate::simd_ops::wu8(rsp.wrapping_add(0x48u64), ((rax as u8)) as u8); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r8 = ((rax as u32)) as u32 as u64; r9 = rsp.wrapping_add(0x48); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; rdx = ((rbx.wrapping_add(P0)) as u64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); rcx = rdi; rax = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x48u64))) as u32) as u32 as u64; crate::simd_ops::lk_and_u8(rbx.wrapping_add(P2), (0xfeu8) as u8); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x1u8))) as u8 as u64)); fl_l = (crate::simd_ops::lk_or_u8(rbx.wrapping_add(P2), ((rax as u8)) as u8)) as u8 as u64; fl_r = fl_l; bb = 4; continue; }
            4 => { rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = 0x1; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P8))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415fad70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415fa360; const P1: u64 = 0x1415faca0; const P2: u64 = 0x1415c11f0; const P3: u64 = 0x14158f5a0; const P4: u64 = 0x1415f9ee0; const P5: u64 = 0x141620d10; const P6: u64 = 0x141620d50; const P7: u64 = 0x1415fc410; const P8: u64 = 0x1415fc320; const P9: u64 = 0x1415c2070; const P10: u64 = 0x14152ede0; const P11: u64 = 0x1415864b0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 111>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rbp = rsp.wrapping_sub(0x260); rsp = ((rsp).wrapping_sub(0x360u64)) as u64; crate::simd_ops::lk_or_u8(rcx.wrapping_add(0x13eu64), (0x1u8) as u8); rdi = rdx; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x13du64))) as u8 as u64; fl_r = (0x40u8) as u8 as u64; rsi = rcx; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; r9 = rbp.wrapping_add(0x290); r8 = rbp.wrapping_add(0x280); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x38u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x20; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x7cu64))) as u32 as u64; fl_l = (((rdx) >> (((rax) as u32 & 63))) & 1) as u64; rdx = ((rdx) & !((1u64) << (((rax) as u32 & 63)))) as u64; rax = ((crate::simd_ops::ru16(rbp.wrapping_add(0x280u64))) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x7cu64), ((rdx as u32)) as u32); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x38u64))) as u32 as u64; crate::simd_ops::wu16(rdi.wrapping_add(rcx.wrapping_mul(2)).wrapping_add(0x3cu64), ((rax as u16)) as u16); fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x38u64), (1) as u32)) as u32 as u64; bb = 5; continue; }
            4 => { fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x38u64), ((rax as u32)) as u32); bb = 5; continue; }
            5 => { rdx = rdi; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; rcx = r14.wrapping_add(0x10); rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((rbx as u32)) as u32 as u64; bb = 35; continue; }
            7 => { fl_l = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rbx = 0x2; rax = ((rbx as u32)) as u32 as u64; bb = 35; continue; }
            9 => { rdx = rdi; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; bb = 10; continue; }
            10 => { rax = ((rbx as u32)) as u32 as u64; bb = 35; continue; }
            11 => { fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 26; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rax); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x34u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x6cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x78u64), rax); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x30u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x70u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((r14 as u32)) as u32); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::lk_add_u32(rax, (1) as u32)) as u32 as u64; bb = 14; continue; }
            14 => { xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(0x38u64))) as u128; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rax); rcx = rbp.wrapping_add(0x10); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x80u64))) as u32 as u64; xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(0x48u64))) as u128; crate::simd_ops::wu32(rbp.wrapping_sub(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x84u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x1cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x88u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), rax); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x90u64))) as u32 as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x68u64), (xmm0) as u128); crate::simd_ops::wu32(rbp.wrapping_sub(0x10u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x94u64))) as u32 as u64; xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(0x58u64))) as u128; crate::simd_ops::wu32(rbp.wrapping_sub(0xcu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0xa0u64))) as u32 as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x48u64), (xmm0) as u128); crate::simd_ops::wu32(rbp, ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(0x78u64))) as u64) as u128; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0xa8u64))) as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x58u64), (xmm1) as u128); crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), rax); xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(0x68u64))) as u128; crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), ((xmm0 as u64)) as u64); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(0x98u64))) as u64) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x38u64), (xmm1) as u128); crate::simd_ops::wu64(rbp.wrapping_sub(0x8u64), ((xmm0 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x2e0u64))) as u64; rdx = rdi.wrapping_add(0xb0); crate::simd_ops::wu64(rbp.wrapping_add(0x240u64), rax); rcx = rbp.wrapping_add(0x10); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x2e8u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x248u64), rax); crate::simd_ops::wu64(rbp.wrapping_add(0xc8u64), r14); crate::simd_ops::wu64(rbp.wrapping_add(0xd0u64), r14); crate::simd_ops::wu64(rbp.wrapping_add(0xd8u64), r14); crate::simd_ops::wu64(rbp.wrapping_add(0xe0u64), r14); crate::simd_ops::wu32(rbp.wrapping_add(0xe8u64), ((r14 as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x230u64), r14); crate::simd_ops::wu64(rbp.wrapping_add(0x238u64), r14); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x2f0u64))) as u64; xmm0 = 0u128; rcx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x258u64))) as u32) as u32 as u64; xmm1 = 0u128; crate::simd_ops::wu64(rbp.wrapping_add(0x250u64), rax); rax = (((rcx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) ^ (crate::simd_ops::ru8(rdi.wrapping_add(0x2f8u64))))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xfu8))) as u8 as u64)); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((r14 as u32)) as u32); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) ^ ((rax as u8)))) as u8 as u64)); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r14); rax = rsi.wrapping_add(0x10); crate::simd_ops::wu8(rbp.wrapping_add(0x258u64), ((rcx as u8)) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rax); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm1) as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), (0x1u32) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 16; continue; }
            16 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { rbx = ((rax.wrapping_add(0x2u64)) as u32) as u32 as u64; bb = 22; continue; }
            18 => { rax = rsp.wrapping_add(0x20); rcx = rsi; crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), rax); rdx = rsp.wrapping_add(0x60); rax = ((crate::simd_ops::ru8(rsi.wrapping_add(0x13cu64))) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xfu8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x5u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 20; continue; } bb = 19; continue; }
            19 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 21; continue; }
            20 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 21; continue; }
            21 => { rbx = ((rax as u32)) as u32 as u64; bb = 22; continue; }
            22 => { rcx = rsp.wrapping_add(0x48); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r14); bb = 24; continue; }
            24 => { rcx = rbp.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x80u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 25; continue; }
            25 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((rbx as u32)) as u32 as u64; bb = 35; continue; }
            26 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x88u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 27; continue; }
            27 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 32; continue; } bb = 28; continue; }
            28 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x88u64))) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x18u64), rax); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 29; continue; }
            29 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 30; continue; }
            30 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x88u64))) as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 32; continue; } bb = 31; continue; }
            31 => { rax = 0x2; bb = 35; continue; }
            32 => { rax = ((crate::simd_ops::ru8(rsi.wrapping_add(0x13cu64))) as u32) as u32 as u64; rdx = rdi; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xfu8))) as u8 as u64)); rcx = rsi; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x5u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 34; continue; } bb = 33; continue; }
            33 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 35; continue; }
            34 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 35; continue; }
            35 => { r11 = rsp.wrapping_add(0x360); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415fb0e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415f9e60; const P1: u64 = 0x13e; const P2: u64 = 0x143930f08; const P3: u64 = 0x1415aa290; const P4: u64 = 0x1415aa460; const P5: u64 = 0x1415a34b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P1))) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = (crate::simd_ops::ru64(P2)) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rdx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P2)) as u64; rdx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
    r9 = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x4u32) as u32);
    r8 = 0;
    fl_l = 0; fl_r = 0;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415fb140(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415242e0; const P1: u64 = 0x141524320; const P2: u64 = 0x1415bf1b0; const P3: u64 = 0x1415bf250;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu32(rsp.wrapping_add(0x10u64), ((rdx as u32)) as u32); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x148u64))) as u32 as u64; rbp = rcx; r12 = ((rdx as u32)) as u32 as u64; r14 = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 12; continue; } bb = 1; continue; }
            1 => { rax = (((rax as u32)).wrapping_sub((r12 as u32))) as u32 as u64; r14 = ((r12.wrapping_add(r12.wrapping_mul(4))) as u64) as u64; r14 = ((r14).wrapping_shl(3u32)) as u64; rdi = 0; fl_l = 0; fl_r = 0; r15 = ((rax as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x140u64))) as u64; rsi = ((rsi).wrapping_add(r14)) as u64; rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x20u64))) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x20u64))) as u32 as u64; rcx = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 7; continue; }
            7 => { rbx = rdx; bb = 8; continue; }
            8 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = rbx; crate::simd_ops::wu32(rsi.wrapping_add(0x20u64), ((rdi as u32)) as u32); rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi.wrapping_add(0x18u64), rdi); crate::simd_ops::wu32(rsi.wrapping_add(0x24u64), ((rdi as u32)) as u32); bb = 10; continue; }
            10 => { r14 = ((r14).wrapping_add(0x28u64)) as u64; fl_l = r15; fl_r = (0x1u64) as u64; r15 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 11; continue; }
            11 => { bb = 34; continue; }
            12 => { rdi = 0; fl_l = 0; fl_r = 0; r15 = rax; fl_l = ((r12 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(0x14cu64))) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 29; continue; } bb = 13; continue; }
            13 => { rdx = ((r12.wrapping_add(r12.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(3u32)) as u64; fl_l = rdx; fl_r = (0x140u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { r13 = rbp; bb = 16; continue; }
            15 => { rcx = 0; fl_l = 0; fl_r = 0; r8 = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r13 = rax; bb = 16; continue; }
            16 => { fl_l = r13; fl_r = r13; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 35; continue; }
            18 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x140u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 19; continue; }
            19 => { fl_l = rax; fl_r = r13; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 28; continue; } bb = 20; continue; }
            20 => { fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 21; continue; }
            21 => { r12 = (0xffffffffffffffdcu64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r15); fl_l = r12; fl_r = r13; r12 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rsi = r13.wrapping_add(0x24); bb = 22; continue; }
            22 => { rcx = rsi.wrapping_sub(0x24); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { crate::simd_ops::wu64(rcx, rdi); crate::simd_ops::wu64(rsi.wrapping_sub(0x1cu64), rdi); crate::simd_ops::wu32(rsi.wrapping_sub(0x14u64), ((rdi as u32)) as u32); crate::simd_ops::wu64(rsi.wrapping_sub(0xcu64), rdi); crate::simd_ops::wu64(rsi.wrapping_sub(0x4u64), rdi); bb = 24; continue; }
            24 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x140u64))) as u64; rbx = ((r12.wrapping_add(rsi)) as u64) as u64; rdx = ((rdx).wrapping_add(rbx)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x140u64))) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = ((rcx).wrapping_add(rbx)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rsi = ((rsi).wrapping_add(0x28u64)) as u64; fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_r = (0x1u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64); if (((fl_l as u64)) != ((fl_r as u64))) { bb = 22; continue; } bb = 25; continue; }
            25 => { r12 = (crate::simd_ops::ru32(rsp.wrapping_add(0x68u64))) as u32 as u64; bb = 26; continue; }
            26 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x140u64))) as u64; fl_l = rbp; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 28; continue; } bb = 27; continue; }
            27 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 28; continue; }
            28 => { crate::simd_ops::wu64(rbp.wrapping_add(0x140u64), r13); crate::simd_ops::wu32(rbp.wrapping_add(0x14cu64), ((r12 as u32)) as u32); bb = 29; continue; }
            29 => { fl_l = r15; fl_r = r14; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 34; continue; } bb = 30; continue; }
            30 => { rdx = ((r15.wrapping_add(r15.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(3u32)) as u64; fl_l = r14; fl_r = r15; r14 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 31; continue; }
            31 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x140u64))) as u64; fl_l = ((rcx).wrapping_add(rdx)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 33; continue; } bb = 32; continue; }
            32 => { crate::simd_ops::wu64(rcx, rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rdi); crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), ((rdi as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), rdi); bb = 33; continue; }
            33 => { rdx = ((rdx).wrapping_add(0x28u64)) as u64; fl_l = r14; fl_r = (0x1u64) as u64; r14 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 31; continue; } bb = 34; continue; }
            34 => { crate::simd_ops::wu32(rbp.wrapping_add(0x148u64), ((r12 as u32)) as u32); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 35; continue; }
            35 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415fb340(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x13d; const P1: u64 = 0x1415f9e60; const P2: u64 = 0x110; const P3: u64 = 0x1415fa0e0; const P4: u64 = 0x8; const P5: u64 = 0x1415fc1a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rbx = rdx;
    fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x40u8) as u8 as u64;
    if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64;
    if (((((fl_l as u8)) & ((fl_r as u8))) as i8) < 0) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi);
    rdi = ((rcx.wrapping_add(P2)) as u64) as u64;
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); }
    rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415fb3a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415cd520; const P1: u64 = 0x1415cd2f0; const P2: u64 = 0x1415cd350; const P3: u64 = 0x1415cd500; const P4: u64 = 0x1415cd4e0; const P5: u64 = 0x1439310e8; const P6: u64 = 0x1415cd330; const P7: u64 = 0x1415fc1f0; const P8: u64 = 0x1415422a0; const P9: u64 = 0x14183acc8; const P10: u64 = 0x1415cd310;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r15 = rcx; crate::simd_ops::wu8(r8, (0x1u8) as u8); rcx = 0; fl_l = 0; fl_r = 0; rdi = rdx; rbp = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), ((rcx as u32)) as u32); r13 = 0xffff; r12 = (crate::simd_ops::ru64(r15.wrapping_add(0x108u64))) as u64; r14 = ((rcx.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru16(rdx.wrapping_add(0x12u64))) as u16 as u64; fl_r = ((rcx as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 13; continue; } bb = 1; continue; }
            1 => { fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { rax = ((crate::simd_ops::ru8(r9.wrapping_add(0x2u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru16(r9)) as u16 as u64; fl_r = (0x1u16) as u16 as u64; crate::simd_ops::wu16(r9, ((fl_l as u16).wrapping_sub(fl_r as u16)) as u16); if (((fl_l as u16)) != ((fl_r as u16))) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu8(r8, ((rcx as u8)) as u8); rax = (((rcx as u16)) as u32) as u32 as u64; bb = 49; continue; }
            6 => { rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x108u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu16(rdi.wrapping_add(0x12u64), ((rax as u16)) as u16); rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x108u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = ((rax as u32)) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; r13 = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rdx = ((crate::simd_ops::ru16(rdi.wrapping_add(0x10u64))) as u32) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 7; continue; }
            7 => { r14 = (((rdx as u16)) as u32) as u32 as u64; rbp = 0xffff; bb = 8; continue; }
            8 => { rsi = ((crate::simd_ops::ru16(rbx)) as u32) as u32 as u64; rcx = r12; rax = (crate::simd_ops::ru64(r12)) as u64; rdx = (((rsi as u16)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (((rsi as u16)) as u32) as u32 as u64; rcx = rdi; r13 = (((r13 as u32)).wrapping_sub((rax as u32))) as u32 as u64; r14 = (r14 & !0xffffu64) | (((((r14 as u16)).wrapping_add((rbp as u16))) as u16 as u64)); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x2u64)) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { rbp = (crate::simd_ops::ru32(rsp.wrapping_add(0x68u64))) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x60u64), ((r14 as u16)) as u16); r14 = 0xffffffff; rdx = ((crate::simd_ops::ru16(rsp.wrapping_add(0x60u64))) as u32) as u32 as u64; bb = 10; continue; }
            10 => { rax = ((crate::simd_ops::ru8(r15.wrapping_add(0x13cu64))) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x30u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((r13 as u32)) as u32); bb = 12; continue; }
            12 => { crate::simd_ops::wu16(rdi.wrapping_add(0x10u64), ((rdx as u16)) as u16); r13 = 0xffff; bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru8(r15.wrapping_add(0x13du64))) as u8 as u64; fl_r = (0x8u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 25; continue; } bb = 14; continue; }
            14 => { rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rsi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffffffff; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 34; continue; } bb = 15; continue; }
            15 => { bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru8(r15.wrapping_add(0x13cu64))) as u8 as u64; fl_r = (0x30u8) as u8 as u64; rbx = (((rbp as u16)) as u32) as u32 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 19; continue; } bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru16(r15.wrapping_add(0x13au64))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 22; continue; } bb = 18; continue; }
            18 => { rdx = (((rbp as u16)) as u32) as u32 as u64; bb = 21; continue; }
            19 => { rdx = (((rbp as u16)) as u32) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 23; continue; } bb = 20; continue; }
            20 => { rdx = (((rbx as u16)) as u32) as u32 as u64; bb = 21; continue; }
            21 => { rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru64(r12)) as u64; rdx = (((rbx as u16)) as u32) as u32 as u64; rcx = r12; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (((r14 as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_r = fl_l; r14 = ((fl_l as u32)) as u32 as u64; bb = 23; continue; }
            23 => { rbp = (((rbp as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 16; continue; } bb = 24; continue; }
            24 => { bb = 34; continue; }
            25 => { rax = (crate::simd_ops::ru64(P5)) as u64; rcx = (0x5851f42d4c957f2du64) as u64; rax = (((rax) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(P5, rax); rcx = ((crate::simd_ops::ru16(rdi.wrapping_add(0x10u64))) as u32) as u32 as u64; rax = ((rax).wrapping_shr(33u32)) as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rbx = (((rdx as u16)) as u32) as u32 as u64; bb = 26; continue; }
            26 => { fl_l = (crate::simd_ops::ru8(r15.wrapping_add(0x13cu64))) as u8 as u64; fl_r = (0x30u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 29; continue; } bb = 27; continue; }
            27 => { fl_l = (crate::simd_ops::ru16(r15.wrapping_add(0x13au64))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 31; continue; } bb = 28; continue; }
            28 => { rdx = (((rbp as u16)) as u32) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rcx = (rcx & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 33; continue; }
            29 => { rdx = (((rbp as u16)) as u32) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 32; continue; } bb = 30; continue; }
            30 => { rdx = (((rbp as u16)) as u32) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 32; continue; } bb = 31; continue; }
            31 => { rcx = (rcx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 33; continue; }
            32 => { rcx = (rcx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 33; continue; }
            33 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; rax = ((r14.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (r14 as u32) } else { (rax as u32) }) as u32 as u64; rbp = (((rbp as u32)).wrapping_add(1)) as u32 as u64; r14 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 26; continue; } bb = 34; continue; }
            34 => { rbp = (rbp & !0xffffu64) | (((((rbp as u16)).wrapping_sub(1)) as u16 as u64)); fl_l = (crate::simd_ops::ru8(r15.wrapping_add(0x13cu64))) as u8 as u64; fl_r = (0x30u8) as u8 as u64; rdx = (((rbp as u16)) as u32) as u32 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 38; continue; } bb = 35; continue; }
            35 => { rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 37; continue; } bb = 36; continue; }
            36 => { rdx = (((rbp as u16)) as u32) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_add_u16(rdi.wrapping_add(0x12u64), ((r13 as u16)) as u16)) as u16 as u64; fl_r = fl_l; bb = 37; continue; }
            37 => { r8 = (((rbp as u16)) as u32) as u32 as u64; rdx = rdi; rcx = r15; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 48; continue; }
            38 => { rax = ((crate::simd_ops::ru16(r15.wrapping_add(0x13au64))) as u32) as u32 as u64; rsi = 0x1; fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; rcx = r12; rsi = (rsi & !0xffffu64) | (((if ((((fl_l as u16)) & ((fl_r as u16))) != 0) { (rax as u16) } else { (rsi as u16) }) as u16 as u64)); rax = (crate::simd_ops::ru64(r12)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_add_u16(rdi.wrapping_add(0x10u64), ((r13 as u16)) as u16); rdx = (((rbp as u16)) as u32) as u32 as u64; crate::simd_ops::lk_sub_u32(rdi.wrapping_add(0xcu64), ((rax as u32)) as u32); rcx = rdi; crate::simd_ops::lk_add_u16(rdi.wrapping_add(0x12u64), ((r13 as u16)) as u16); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); r14 = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x24u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 41; continue; } bb = 39; continue; }
            39 => { rcx = rdi.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 47; continue; } bb = 40; continue; }
            40 => { fl_l = ((r14 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x24u64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 47; continue; } bb = 41; continue; }
            41 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 47; continue; } bb = 42; continue; }
            42 => { crate::simd_ops::wu16(rdx, ((rbp as u16)) as u16); rcx = rdi; rdx = (((rbp as u16)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x108u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; rax = (rax & !0xffffu64) | (((((rax as u16)).wrapping_sub(1)) as u16 as u64)); fl_l = ((rsi as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; rax = (rax & !0xffffu64) | (((if (((fl_l as u16)) < ((fl_r as u16))) { (rsi as u16) } else { (rax as u16) }) as u16 as u64)); rax = (((rax as u16)) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 48; continue; } bb = 43; continue; }
            43 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rbx = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 45; continue; } bb = 44; continue; }
            44 => { r8 = ((rdx.wrapping_mul(2).wrapping_sub(0x2u64)) as u32) as u32 as u64; rdx = rcx.wrapping_add(0x2); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 45; continue; }
            45 => { crate::simd_ops::lk_sub_u32(rdi.wrapping_add(0x20u64), (1) as u32); rdx = (((rbx as u16)) as u32) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rdx = (((rbx as u16)) as u32) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 48; continue; } bb = 46; continue; }
            46 => { rax = (crate::simd_ops::ru64(r12)) as u64; rdx = (((rbx as u16)) as u32) as u32 as u64; rcx = r12; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_add_u32(rdi.wrapping_add(0xcu64), ((rax as u32)) as u32); fl_l = (crate::simd_ops::lk_add_u16(rdi.wrapping_add(0x10u64), (1) as u16)) as u16 as u64; bb = 48; continue; }
            47 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu16(rdi.wrapping_add(0x12u64), ((rax as u16)) as u16); bb = 48; continue; }
            48 => { rax = (((rbp as u16)) as u32) as u32 as u64; bb = 49; continue; }
            49 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415fb720(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(r8, (0x1u8) as u8); rdi = r9; r14 = r8; rbx = rdx; rsi = rcx; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x8u64))) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x108u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (((crate::simd_ops::ru16(rbx.wrapping_add(0xau64))) as i16 as i32) as u32) as u32 as u64; rcx = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x13du64))) as u8 as u64; fl_r = (0x20u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = (((rdx as u16)).wrapping_sub(1)) as u16 as u64; rdx = (rdx & !0xffffu64) | ((((fl_l as u16)) as u16 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x8u64), ((rbp as u8)) as u8); crate::simd_ops::wu16(rbx.wrapping_add(0xau64), ((rdx as u16)) as u16); bb = 17; continue; }
            4 => { crate::simd_ops::wu16(rbx.wrapping_add(0xau64), ((rbp as u16)) as u16); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 5; continue; }
            5 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x2u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 15; continue; } bb = 6; continue; }
            6 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 17; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru16(rdi)) as u16 as u64; fl_r = (0x1u16) as u16 as u64; crate::simd_ops::wu16(rdi, ((fl_l as u16).wrapping_sub(fl_r as u16)) as u16); if (((fl_l as u16)) == ((fl_r as u16))) { bb = 15; continue; } bb = 8; continue; }
            8 => { bb = 17; continue; }
            9 => { fl_l = (((rdx as u16)).wrapping_add(1)) as u16 as u64; rdx = (rdx & !0xffffu64) | ((((fl_l as u16)) as u16 as u64)); crate::simd_ops::wu16(rbx.wrapping_add(0xau64), ((rdx as u16)) as u16); bb = 17; continue; }
            10 => { rax = ((crate::simd_ops::ru16(rdx.wrapping_add(0xau64))) as u32) as u32 as u64; fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) != 0) { bb = 16; continue; } bb = 11; continue; }
            11 => { rax = 0x1; crate::simd_ops::wu16(rdx.wrapping_add(0xau64), ((rax as u16)) as u16); crate::simd_ops::wu8(rdx.wrapping_add(0x8u64), ((rax as u8)) as u8); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 12; continue; }
            12 => { rcx = ((crate::simd_ops::ru8(r9.wrapping_add(0x2u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 15; continue; } bb = 13; continue; }
            13 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 17; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru16(r9)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; crate::simd_ops::wu16(r9, ((fl_l as u16).wrapping_sub(fl_r as u16)) as u16); if (((fl_l as u16)) != ((fl_r as u16))) { bb = 17; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu8(r14, ((rbp as u8)) as u8); rax = (((rbp as u16)) as u32) as u32 as u64; bb = 18; continue; }
            16 => { fl_l = (((rax as u16)).wrapping_sub(1)) as u16 as u64; rax = (rax & !0xffffu64) | ((((fl_l as u16)) as u16 as u64)); crate::simd_ops::wu16(rdx.wrapping_add(0xau64), ((rax as u16)) as u16); bb = 17; continue; }
            17 => { rax = ((crate::simd_ops::ru16(rbx.wrapping_add(0xau64))) as u32) as u32 as u64; bb = 18; continue; }
            18 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415fb810(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14268e088;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rbp = 0; fl_l = 0; fl_r = 0; rcx = ((crate::simd_ops::ru8(rcx.wrapping_add(0x13du64))) as u32) as u32 as u64; rsi = rdx; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if (((((fl_l as u8)) & ((fl_r as u8))) as i8) >= 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((rbp as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x120u64))) as u64; fl_r = rax; rax = (rax & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 3; continue; }
            2 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x118u64))) as u32 as u64; bb = 3; continue; }
            3 => { rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0x7u8))) as u8 as u64)); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdx)) as u64; r9 = rsp.wrapping_add(0x50); crate::simd_ops::wu8(rsp.wrapping_add(0x40u64), ((rcx as u8)) as u8); rdx = rsp.wrapping_add(0x40); r8 = 0x1; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 16; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rsi)) as u64; r9 = rsp.wrapping_add(0x40); r8 = 0x4; rdx = rsp.wrapping_add(0x48); rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 16; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x108u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbp = ((rax as u32)) as u32 as u64; bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x120u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rsi)) as u64; r9 = rsp.wrapping_add(0x40); r8 = 0x8; rdx = P0; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 16; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x120u64))) as u64; r8 = (((rbp as u16)) as u32) as u32 as u64; rdx = rsi; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 15; continue; } bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            11 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x110u64))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x118u64))) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rbx)) as u64; r9 = rsp.wrapping_add(0x40); r8 = 0x8; rdx = rsp.wrapping_add(0x50); rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x58u64))) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rcx); rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 16; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; r8 = (((rbp as u16)) as u32) as u32 as u64; rdx = rsi; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 17; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x118u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x10u64)) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x110u64)))) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 15; continue; }
            15 => { rax = 0x1; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            16 => { rax = 0x2; bb = 17; continue; }
            17 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415fb990(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141540670; const P1: u64 = 0x14183acc8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rdi = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rsi = rdx; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { r9 = (crate::simd_ops::ru64(rbx)) as u64; bb = 2; continue; }
            2 => { rdx = ((rcx as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rdi as u32))) as u32 as u64; rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(r9)) as u64; fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 3; continue; }
            3 => { rax = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { (rdi as u32) } else { (rax as u32) }) as u32 as u64; rdi = ((rax as u32)) as u32 as u64; rax = ((rcx.wrapping_sub(0x1u64)) as u32) as u32 as u64; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { (rcx as u32) } else { (rax as u32) }) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; fl_l = (((rcx as u32)).wrapping_shr(1u32)) as u32 as u64; rcx = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = ((((rdi as u32)) as i32 as i64) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; fl_l = ((rax).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbp); rbp = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; if (((fl_l as u64)) == 0) { bb = 11; continue; } bb = 5; continue; }
            5 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 17; continue; } bb = 7; continue; }
            7 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 17; continue; } bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); r14 = ((rdi as u32)) as u32 as u64; r14 = ((r14).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(r14)) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (((rax as u32)).wrapping_sub((rdi as u32))) as u32 as u64; rcx = rdx.wrapping_add(0x10); rax = (((rax as u32)).wrapping_shl(4u32)) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x8u64), (1) as u32); rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = ((rax).wrapping_add(r14)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 15; continue; }
            11 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 17; continue; } bb = 13; continue; }
            13 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 17; continue; } bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = ((rcx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; fl_l = ((rax).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 15; continue; }
            15 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 16; continue; }
            16 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rax, rsi); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            17 => { rax = 0; fl_l = 0; fl_r = 0; bb = 18; continue; }
            18 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 19; continue; }
            19 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415fba90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f00; const P1: u64 = 0x1401de410; const P2: u64 = 0x1415a2910; const P3: u64 = 0x141524330; const P4: u64 = 0x141649350; const P5: u64 = 0x141649330; const P6: u64 = 0x1415fb0e0; const P7: u64 = 0x14153be40; const P8: u64 = 0x1415fbeb0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm0: u128 = 0; let mut xmm1: u128 = 0; let mut xmm2: u128 = 0; let mut xmm3: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu32(rsp.wrapping_add(0x18u64), ((r8 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rbp = rsp; rsp = ((rsp).wrapping_sub(0x68u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdi = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x70u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rdi; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rbx = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x70u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(P0)) as u64; r8 = 0x2; r9 = ((rax as u32)) as u32 as u64; rdx = ((rbx as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0x5b; rsp = ((rsp).wrapping_add(0x68u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { crate::simd_ops::lk_add_u64(rbp.wrapping_add(0x30u64), (0x4u64) as u64); r8 = rbp.wrapping_add(0x38); r9 = 0; fl_l = 0; fl_r = 0; rdx = rbp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 41; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64; rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0xa8u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r13); rcx = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rax = ((rax).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r15); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm6) as u128); r8 = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rax = ((rax).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), rax); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm7) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm8) as u128); rdx = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rax = ((rax).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), rax); fl_l = ((rcx as u16)) as u16 as u64; fl_r = ((rcx as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = (((rbx as u16)) as u32) as u32 as u64; r8 = (((rbx as u16)) as u32) as u32 as u64; rcx = (((rbx as u16)) as u32) as u32 as u64; bb = 5; continue; }
            5 => { crate::simd_ops::wu16(rdi.wrapping_add(0x134u64), ((rcx as u16)) as u16); rsi = 0x1; crate::simd_ops::wu16(rdi.wrapping_add(0x136u64), ((r8 as u16)) as u16); r14 = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu16(rdi.wrapping_add(0x138u64), ((rdx as u16)) as u16); xmm6 = ((crate::simd_ops::ru32(rax)) as u32) as u128; rax = ((rax).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), rax); xmm7 = ((crate::simd_ops::ru32(rax)) as u32) as u128; rax = ((rax).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), rax); xmm8 = ((crate::simd_ops::ru32(rax)) as u32) as u128; rax = ((rax).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), rax); r13 = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rax = ((rax).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), rax); rcx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), rax); crate::simd_ops::wu8(rbp.wrapping_add(0x28u64), ((rcx as u8)) as u8); rcx = ((crate::simd_ops::ru8(rdi.wrapping_add(0x13du64))) as u32) as u32 as u64; r12 = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), rax); rdx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), rax); r15 = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), rax); rax = (((rcx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 18; continue; } bb = 6; continue; }
            6 => { rax = (((rcx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) ^ ((rdx as u8)))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) ^ ((rcx as u8)))) as u8 as u64)); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x108u64))) as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x13du64), ((rax as u8)) as u8); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x13du64))) as u32) as u32 as u64; r14 = 0x2; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); rdx = 0x18; rcx = ((r14 as u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 11; continue; } bb = 9; continue; }
            9 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 10; continue; }
            10 => { rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; bb = 14; continue; }
            11 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; bb = 14; continue; }
            13 => { rcx = rbx; bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rdi.wrapping_add(0x108u64), rcx); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x108u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi.wrapping_add(0x108u64), rbx); bb = 17; continue; }
            17 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 18; continue; }
            18 => { fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 40; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::wu32(rdi.wrapping_add(0x128u64), ((xmm6 as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0x12cu64), ((xmm7 as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0x130u64), ((xmm8 as u32)) as u32); fl_l = (crate::simd_ops::ru16(rdi.wrapping_add(0x13au64))) as u16 as u64; fl_r = ((r13 as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 21; continue; } bb = 20; continue; }
            20 => { rcx = rdi; crate::simd_ops::wu16(rdi.wrapping_add(0x13au64), ((r13 as u16)) as u16); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 21; continue; }
            21 => { rcx = ((crate::simd_ops::ru8(rdi.wrapping_add(0x13cu64))) as u32) as u32 as u64; rdx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x28u64))) as u32) as u32 as u64; rax = (((rcx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xfu8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rax = (((rcx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) ^ ((rdx as u8)))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xfu8))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) ^ ((rcx as u8)))) as u8 as u64)); rcx = rdi; crate::simd_ops::wu8(rdi.wrapping_add(0x13cu64), ((rax as u8)) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 23; continue; }
            23 => { rcx = ((crate::simd_ops::ru8(rdi.wrapping_add(0x13cu64))) as u32) as u32 as u64; rax = (((rcx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shr(4u32)) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x3u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 25; continue; } bb = 24; continue; }
            24 => { r12 = (r12 & !0xffu64) | (((((r12 as u8)).wrapping_shl(4u32)) as u8 as u64)); r12 = (r12 & !0xffu64) | ((((((r12 as u8)) ^ ((rcx as u8)))) as u8 as u64)); r12 = (r12 & !0xffu64) | ((((((r12 as u8)) & (0x30u8))) as u8 as u64)); r12 = (r12 & !0xffu64) | ((((((r12 as u8)) ^ ((rcx as u8)))) as u8 as u64)); rcx = rdi; crate::simd_ops::wu8(rdi.wrapping_add(0x13cu64), ((r12 as u8)) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 25; continue; }
            25 => { rdx = ((crate::simd_ops::ru8(rdi.wrapping_add(0x13du64))) as u32) as u32 as u64; rcx = (((r15 as u8)) as u32) as u32 as u64; rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_shr(1u32)) as u8 as u64)); rax = (((rdx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shr(4u32)) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & ((rsi as u8)))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) & ((rsi as u8)))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 27; continue; } bb = 26; continue; }
            26 => { rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_shl(4u32)) as u8 as u64)); rdx = (rdx & !0xffu64) | ((((((rdx as u8)) & (0xefu8))) as u8 as u64)); rdx = (rdx & !0xffu64) | ((((((rdx as u8)) | ((rcx as u8)))) as u8 as u64)); rcx = rdi; crate::simd_ops::wu8(rdi.wrapping_add(0x13du64), ((rdx as u8)) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 27; continue; }
            27 => { rcx = (((r15 as u8)) as u32) as u32 as u64; rdx = (((r15 as u8)) as u32) as u32 as u64; rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_shl(3u32)) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) ^ (crate::simd_ops::ru8(rdi.wrapping_add(0x13du64))))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0x20u8))) as u8 as u64)); rdx = (rdx & !0xffu64) | (((((rdx as u8)).wrapping_shr(3u32)) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) ^ (crate::simd_ops::ru8(rdi.wrapping_add(0x13du64))))) as u8 as u64)); rdx = (rdx & !0xffu64) | ((((((rdx as u8)) & ((rsi as u8)))) as u8 as u64)); rax = (((rcx as u8)) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x13du64), ((rcx as u8)) as u8); rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shr(6u32)) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) & ((rsi as u8)))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 29; continue; } bb = 28; continue; }
            28 => { rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0xbfu8))) as u8 as u64)); rdx = (rdx & !0xffu64) | (((((rdx as u8)).wrapping_shl(6u32)) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) | ((rdx as u8)))) as u8 as u64)); crate::simd_ops::wu8(rdi.wrapping_add(0x13du64), ((rcx as u8)) as u8); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 29; continue; }
            29 => { rcx = ((crate::simd_ops::ru8(rdi.wrapping_add(0x13du64))) as u32) as u32 as u64; r15 = (r15 & !0xffu64) | (((((r15 as u8)).wrapping_shr(4u32)) as u8 as u64)); rax = (((rcx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shr(7u32)) as u8 as u64)); r15 = (r15 & !0xffu64) | ((((((r15 as u8)) & ((rsi as u8)))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 31; continue; } bb = 30; continue; }
            30 => { rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0x7fu8))) as u8 as u64)); r15 = (r15 & !0xffu64) | (((((r15 as u8)).wrapping_shl(7u32)) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) | ((r15 as u8)))) as u8 as u64)); crate::simd_ops::wu8(rdi.wrapping_add(0x13du64), ((rcx as u8)) as u8); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 31; continue; }
            31 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64; r15 = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::lk_add_u64(rbp.wrapping_add(0x30u64), (0x4u64) as u64); fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 38; continue; } bb = 32; continue; }
            32 => { rcx = rdi.wrapping_add(0xf0); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x14u64))) as u32 as u64; fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 34; continue; } bb = 33; continue; }
            33 => { rdx = ((r15 as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_sub((rax as u32))) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; r14 = 0x34; r14 = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (rsi as u32) } else { (r14 as u32) }) as u32 as u64; rsi = ((r14 as u32)) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 40; continue; } bb = 34; continue; }
            34 => { fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 38; continue; } bb = 35; continue; }
            35 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64; rcx = rdi; crate::simd_ops::wu8(rbp.wrapping_add(0x2cu64), (0x0u8) as u8); rax = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::lk_add_u64(rbp.wrapping_add(0x30u64), (0x4u64) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x28u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 37; continue; } bb = 36; continue; }
            36 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xf; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 38; continue; } bb = 37; continue; }
            37 => { rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 35; continue; } bb = 38; continue; }
            38 => { r14 = ((rsi as u32)) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 40; continue; } bb = 39; continue; }
            39 => { r8 = rbp.wrapping_add(0x38); rcx = rdi; rdx = rbp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = ((rax as u32)) as u32 as u64; bb = 40; continue; }
            40 => { xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; bb = 41; continue; }
            41 => { rax = ((r14 as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x68u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415fbeb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415fb0e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rdi = 0; fl_l = 0; fl_r = 0; r14 = rdx; rbp = rcx; r8 = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rax = ((rax).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rdx, rax); r13 = ((r8 as u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); r15 = rcx.wrapping_add(0x13d); rsi = r15; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(r14)) as u64; r12 = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = ((rax).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu64(r14, rax); rbx = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = ((rax).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu64(r14, rax); fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0xc350; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::lk_or_u8(rsi, (0x8u8) as u8)) as u8 as u64; fl_r = fl_l; r15 = rsi; bb = 4; continue; }
            4 => { rax = ((crate::simd_ops::ru8(r15)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x108u64))) as u64; rdx = ((r12 as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 13; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x108u64))) as u64; r8 = ((rbx as u32)) as u32 as u64; rdx = ((r12 as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = rbp; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            8 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xf; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 9; continue; }
            9 => { rbx = ((rax.wrapping_sub(0xeu64)) as u32) as u32 as u64; bb = 10; continue; }
            10 => { rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 11; continue; }
            11 => { rax = ((rbx as u32)) as u32 as u64; bb = 12; continue; }
            12 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            13 => { rax = 0x23; bb = 12; continue; }
            14 => { rax = 0x1; rsp = ((rsp).wrapping_add(0x28u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415fbfb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x13c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    fl_l = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_r = fl_l;
    rax = ((fl_l as u32)) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415fbfc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1439310e8; const P1: u64 = 0x14268a040; const P2: u64 = 0x14159b8a0; const P3: u64 = 0x143930fa0; const P4: u64 = 0x14153ad70; const P5: u64 = 0x143426f34;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x130u64))) as u32) as u128; rsi = rdx; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0x12cu64)))).to_bits() as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm6) as u128); rbx = rcx; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm8) as u128); xmm6 = 0u128; xmm8 = 0u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm8 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(P0)) as u64; rcx = (0x5851f42d4c957f2du64) as u64; rax = (((rax) as i64).wrapping_mul((rcx) as i64) as u64) as u64; xmm0 = crate::simd_ops::vcvtps2pd(xmm0); rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(P0, rax); fl_l = ((rax).wrapping_shr(33u32)) as u64; rax = ((fl_l as u64)) as u64; xmm1 = (((rax as u32)) as u32) as u128; xmm1 = crate::simd_ops::vcvtdq2pd(xmm1); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(crate::simd_ops::ru64(P1))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm0 as u64)).to_bits() as u128); xmm6 = crate::simd_ops::vcvtpd2ps(xmm1); bb = 2; continue; }
            2 => { xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_add(0x12cu64)))).to_bits() as u128); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rdi); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_add(0x128u64)))).to_bits() as u128); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_l = (((rax) >> (15u32)) & 1) as u64; if ((fl_l as u64) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdi = rbx.wrapping_add(0x28); bb = 6; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; rdi = rbx.wrapping_add(0x28); rdx = 0xf; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0xf; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm7) as u128); rcx = rdi; xmm7 = 0u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = 0xf; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm7 = (xmm0) as u128; bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_l = (((rax) >> (15u32)) & 1) as u64; if ((fl_l as u64) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(P3)) as u64; rdx = rbx.wrapping_add(0x48); r9 = rsi; r8 = 0xf; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 11; continue; }
            11 => { xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) * f32::from_bits(crate::simd_ops::ru32(P5))).to_bits() as u128); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; bb = 12; continue; }
            12 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; xmm6 = (xmm6 & !0xffff_ffffu128) | ((if f32::from_bits(xmm6 as u32) > f32::from_bits(xmm8 as u32) { f32::from_bits(xmm6 as u32) } else { f32::from_bits(xmm8 as u32) }).to_bits() as u128); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm0 = (xmm6) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415fc110(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415a39e0; const P1: u64 = 0x110; const P2: u64 = 0x1415fa0e0; const P3: u64 = 0x8; const P4: u64 = 0x118; const P5: u64 = 0x10; const P6: u64 = 0x14183acc8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rdx; rbx = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = rsi; rcx = ((rbx.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rsi; rcx = ((rbx.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; rdx = (((rdx as u32)).wrapping_sub(1)) as u32 as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(P1)))) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = ((rdx).wrapping_sub(rax)) as u64; rcx = rax; rdx = ((((rdx as u32)) & (0xfffffff0u32))) as u32 as u64; r8 = ((rdx as u32)) as u32 as u64; rdx = ((rax.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::lk_sub_u32(rbx.wrapping_add(P4), (1) as u32)) as u32 as u64; bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415fc1a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415fa0e0; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x14183acc8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { r8 = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; r8 = (((r8 as u32)).wrapping_sub(1)) as u32 as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; r8 = ((r8).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; fl_l = rax; fl_r = r8; if !((((fl_l as u64)) >= ((fl_r as u64)))) { r8 = ((r8).wrapping_sub(rax)) as u64; rdx = ((rax.wrapping_add(P2)) as u64) as u64; r8 = ((((r8 as u32)) & (0xfffffff0u32))) as u32 as u64; rcx = rax; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); } crate::simd_ops::lk_sub_u32(rbx.wrapping_add(P1), (1) as u32); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415fc1f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415422a0; const P1: u64 = 0x1415cd2f0; const P2: u64 = 0x1415cd310; const P3: u64 = 0x14183acc8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru16(rcx.wrapping_add(0x13au64))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; r15 = (((r8 as u16)) as u32) as u32 as u64; r14 = (crate::simd_ops::ru64(rcx.wrapping_add(0x108u64))) as u64; rdi = rdx; rbx = rcx; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 9; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::lk_sub_u16(rdx.wrapping_add(0x10u64), (1) as u16); rbp = (crate::simd_ops::ru32(rdx.wrapping_add(0x20u64))) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx.wrapping_add(0x24u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = rdx.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 3; continue; }
            3 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x24u64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 11; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu16(rdx, ((r15 as u16)) as u16); rcx = rdi; rdx = (((r15 as u16)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(r14)) as u64; rdx = (((r15 as u16)) as u32) as u32 as u64; rcx = r14; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_sub_u32(rdi.wrapping_add(0xcu64), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x108u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = ((crate::simd_ops::ru16(rbx.wrapping_add(0x13au64))) as u32) as u32 as u64; rax = (rax & !0xffffu64) | (((((rax as u16)).wrapping_sub(1)) as u16 as u64)); fl_l = ((rdx as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; rdx = (rdx & !0xffffu64) | (((if (((fl_l as u16)) >= ((fl_r as u16))) { (rax as u16) } else { (rdx as u16) }) as u16 as u64)); rax = (((rdx as u16)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 9; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rcx = rdi; rbx = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rdx = (((rbx as u16)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(r14)) as u64; rdx = (((rbx as u16)) as u32) as u32 as u64; rcx = r14; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_add_u32(rdi.wrapping_add(0xcu64), ((rax as u32)) as u32); crate::simd_ops::lk_add_u16(rdi.wrapping_add(0x10u64), (1) as u16); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { r8 = ((rax.wrapping_mul(2).wrapping_sub(0x2u64)) as u32) as u32 as u64; rdx = rcx.wrapping_add(0x2); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::lk_sub_u32(rdi.wrapping_add(0x20u64), (1) as u32)) as u32 as u64; bb = 9; continue; }
            9 => { rax = 0x1; bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            11 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu16(rdi.wrapping_add(0x12u64), ((rax as u16)) as u16); rax = 0x2; bb = 10; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415fc320(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415f9720; const P1: u64 = 0x1415fa780; const P2: u64 = 0x1415faca0; const P3: u64 = 0x141620bb0; const P4: u64 = 0x1415c11f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = rdx; rsi = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; r8 = rax.wrapping_add(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r8); r9 = rsp.wrapping_add(0x68); rax = ((rax).wrapping_add(0x8u64)) as u64; r8 = rsp.wrapping_add(0x60); rcx = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x38u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x20; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x7cu64))) as u32 as u64; fl_l = (((rdx) >> (((rax) as u32 & 63))) & 1) as u64; rdx = ((rdx) | ((1u64) << (((rax) as u32 & 63)))) as u64; rax = ((crate::simd_ops::ru16(rsp.wrapping_add(0x60u64))) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x7cu64), ((rdx as u32)) as u32); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x38u64))) as u32 as u64; crate::simd_ops::wu16(rdi.wrapping_add(rcx.wrapping_mul(2)).wrapping_add(0x3cu64), ((rax as u16)) as u16); fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x38u64), (1) as u32)) as u32 as u64; bb = 5; continue; }
            4 => { fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x38u64), ((rax as u32)) as u32); bb = 5; continue; }
            5 => { rdx = rdi; rcx = rbp; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = ((rax as u32)) as u32 as u64; rcx = rbp.wrapping_add(0x10); rdx = 0xffffffff; let xa_src: u32 = (rdx as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rdx = (xa_old) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rax = ((rbx as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            8 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x88u64))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rcx.wrapping_sub(0x5); rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_sub_u32(rbx.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 9; continue; }
            9 => { rdx = rdi; rcx = rsi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415fc410(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415f9720; const P1: u64 = 0x1415fa780; const P2: u64 = 0x143930f88; const P3: u64 = 0x1426452b0; const P4: u64 = 0x1426452a8; const P5: u64 = 0x142645560; const P6: u64 = 0x141620d00; const P7: u64 = 0x141620d50; const P8: u64 = 0x1415faca0; const P9: u64 = 0x1439310e8; const P10: u64 = 0x14268a040; const P11: u64 = 0x14159b8a0; const P12: u64 = 0x143930fa0; const P13: u64 = 0x14153ad70; const P14: u64 = 0x143426f34; const P15: u64 = 0x143426494; const P16: u64 = 0x143815b08; const P17: u64 = 0x1415bf880; const P18: u64 = 0x141620bb0; const P19: u64 = 0x1415c11f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 39>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x118u64)) as u64; rsi = 0; fl_l = 0; fl_r = 0; rbp = rdx; r15 = rcx; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x90u64))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rdx.wrapping_add(0x90u64), ((rsi as u32)) as u32); bb = 2; continue; }
            2 => { r12 = (crate::simd_ops::ru32(rdx.wrapping_add(0x2f0u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x140u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x110u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x108u64), r13); crate::simd_ops::wu64(rsp.wrapping_add(0x100u64), r14); fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 10; continue; } bb = 3; continue; }
            3 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r13 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = ((r12.wrapping_add(0x2u64)) as u32) as u32 as u64; bb = 76; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; rcx = r13.wrapping_add(0x8); rax = ((rax).wrapping_add(0x10u64)) as u64; r9 = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); r8 = rsp.wrapping_add(0x148); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x38u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x20; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x7cu64))) as u32 as u64; fl_l = (((rcx) >> (((rax) as u32 & 63))) & 1) as u64; rcx = ((rcx) & !((1u64) << (((rax) as u32 & 63)))) as u64; rax = ((crate::simd_ops::ru16(rsp.wrapping_add(0x148u64))) as u32) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x7cu64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x38u64))) as u32 as u64; crate::simd_ops::wu16(rbp.wrapping_add(rcx.wrapping_mul(2)).wrapping_add(0x3cu64), ((rax as u16)) as u16); crate::simd_ops::lk_add_u32(rbp.wrapping_add(0x38u64), (1) as u32); r12 = 0; fl_l = 0; fl_r = 0; bb = 37; continue; }
            8 => { fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x38u64), ((rax as u32)) as u32); bb = 9; continue; }
            9 => { r12 = 0; fl_l = 0; fl_r = 0; bb = 37; continue; }
            10 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x88u64))) as u64; rbx = (crate::simd_ops::ru64(P2)) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; r13 = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; r13 = r13.wrapping_sub(0x5); r13 = ((rax.wrapping_add(r13.wrapping_mul(8))) as u64) as u64; bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 22; continue; } bb = 12; continue; }
            12 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 13; continue; }
            13 => { r14 = rbx.wrapping_add(0x8); bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 25; continue; } bb = 15; continue; }
            15 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r12 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 16; continue; }
            16 => { bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 24; continue; } bb = 18; continue; }
            18 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 17; continue; } bb = 19; continue; }
            19 => { r12 = 0; fl_l = 0; fl_r = 0; rdi = ((r12 as u32)) as u32 as u64; bb = 20; continue; }
            20 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 26; continue; }
            22 => { r14 = rbx.wrapping_add(0x8); rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 14; continue; } bb = 23; continue; }
            23 => { rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            24 => { r12 = 0; fl_l = 0; fl_r = 0; bb = 20; continue; }
            25 => { r12 = 0; fl_l = 0; fl_r = 0; rdi = ((r12 as u32)) as u32 as u64; bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 34; continue; } bb = 27; continue; }
            27 => { bb = 28; continue; }
            28 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 31; continue; } bb = 29; continue; }
            29 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 28; continue; } bb = 30; continue; }
            30 => { bb = 35; continue; }
            31 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 36; continue; } bb = 32; continue; }
            32 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 36; continue; } bb = 33; continue; }
            33 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 36; continue; }
            34 => { r14 = rbx.wrapping_add(0x8); bb = 35; continue; }
            35 => { rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 36; continue; }
            36 => { fl_l = rdi; fl_r = rdi; crate::simd_ops::wu32(rbp.wrapping_add(0x2f0u64), ((r12 as u32)) as u32); rbx = rdi.wrapping_sub(0x10); rbx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r12 } else { rbx }) as u64; bb = 37; continue; }
            37 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x88u64))) as u64; rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 39; continue; } bb = 38; continue; }
            38 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 39; continue; }
            39 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 66; continue; } bb = 40; continue; }
            40 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; rax = r13.wrapping_add(0x10); rcx = r13.wrapping_add(0x8); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); r9 = rsp.wrapping_add(0x158); rcx = r15; r8 = rsp.wrapping_add(0x150); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = (crate::simd_ops::ru64(rbp.wrapping_add(0x88u64))) as u64; rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 65; continue; } bb = 41; continue; }
            41 => { rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0xf0u64), (xmm6) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0xe0u64), (xmm7) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0xd0u64), (xmm8) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0xc0u64), (xmm9) as u128); crate::simd_ops::wu64(r14.wrapping_add(0x18u64), r12); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 43; continue; } bb = 42; continue; }
            42 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 43; continue; }
            43 => { xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_add(0x78u64))) as u64) as u128; rdx = rbp; xmm6 = (crate::simd_ops::ru128(rbp.wrapping_add(0x38u64))) as u128; rcx = rbx; xmm7 = (crate::simd_ops::ru128(rbp.wrapping_add(0x48u64))) as u128; xmm8 = (crate::simd_ops::ru128(rbp.wrapping_add(0x58u64))) as u128; xmm9 = (crate::simd_ops::ru128(rbp.wrapping_add(0x68u64))) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), ((xmm0 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 45; continue; } bb = 44; continue; }
            44 => { fl_l = (crate::simd_ops::lk_and_u8(rbp.wrapping_add(0x2f8u64), (0xfdu8) as u8)) as u8 as u64; fl_r = fl_l; bb = 45; continue; }
            45 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xb0u64))) as u32 as u64; crate::simd_ops::wu128(rbp.wrapping_add(0x38u64), (xmm6) as u128); crate::simd_ops::wu128(rbp.wrapping_add(0x48u64), (xmm7) as u128); crate::simd_ops::wu128(rbp.wrapping_add(0x58u64), (xmm8) as u128); crate::simd_ops::wu128(rbp.wrapping_add(0x68u64), (xmm9) as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x78u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xb4u64))) as u32 as u64; xmm9 = (crate::simd_ops::ru128(rsp.wrapping_add(0xc0u64))) as u128; crate::simd_ops::wu32(rbp.wrapping_add(0x7cu64), ((rax as u32)) as u32); crate::simd_ops::lk_sub_u32(rbp.wrapping_add(0x38u64), (1) as u32); r14 = (crate::simd_ops::ru64(rbp.wrapping_add(0x88u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 47; continue; } bb = 46; continue; }
            46 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 47; continue; }
            47 => { rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x18u64), rdi); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 49; continue; } bb = 48; continue; }
            48 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 49; continue; }
            49 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x158u64))) as u32 as u64; rdx = ((crate::simd_ops::ru16(rsp.wrapping_add(0x150u64))) as u32) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x2f0u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x38u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x20; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 51; continue; } bb = 50; continue; }
            50 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x7cu64))) as u32 as u64; fl_l = (((rcx) >> (((rax) as u32 & 63))) & 1) as u64; rcx = ((rcx) & !((1u64) << (((rax) as u32 & 63)))) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x7cu64), ((rcx as u32)) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x38u64))) as u32 as u64; crate::simd_ops::wu16(rbp.wrapping_add(rax.wrapping_mul(2)).wrapping_add(0x3cu64), ((rdx as u16)) as u16); fl_l = (crate::simd_ops::lk_add_u32(rbp.wrapping_add(0x38u64), (1) as u32)) as u32 as u64; bb = 52; continue; }
            51 => { fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x38u64), ((rax as u32)) as u32); bb = 52; continue; }
            52 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; xmm8 = 0u128; xmm0 = ((crate::simd_ops::ru32(r15.wrapping_add(0x130u64))) as u32) as u128; xmm6 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(crate::simd_ops::ru32(r15.wrapping_add(0x12cu64)))).to_bits() as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x34u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((r12 as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x50u64), (0xffu8) as u8); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm8 as u32)) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x58u64), (0xffu8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r12); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rax as u32)) as u32); if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 54; continue; } bb = 53; continue; }
            53 => { rax = (crate::simd_ops::ru64(P9)) as u64; rcx = (0x5851f42d4c957f2du64) as u64; rax = (((rax) as i64).wrapping_mul((rcx) as i64) as u64) as u64; xmm0 = crate::simd_ops::vcvtps2pd(xmm0); rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(P9, rax); fl_l = ((rax).wrapping_shr(33u32)) as u64; rax = ((fl_l as u64)) as u64; xmm1 = (((rax as u32)) as u32) as u128; xmm1 = crate::simd_ops::vcvtdq2pd(xmm1); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(crate::simd_ops::ru64(P10))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm0 as u64)).to_bits() as u128); xmm6 = crate::simd_ops::vcvtpd2ps(xmm1); bb = 54; continue; }
            54 => { xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(crate::simd_ops::ru32(r15.wrapping_add(0x12cu64)))).to_bits() as u128); rax = (crate::simd_ops::ru64(r15.wrapping_add(0x58u64))) as u64; xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(crate::simd_ops::ru32(r15.wrapping_add(0x128u64)))).to_bits() as u128); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 57; continue; } bb = 55; continue; }
            55 => { rax = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_l = (((rax) >> (15u32)) & 1) as u64; if ((fl_l as u64) == 0) { bb = 57; continue; } bb = 56; continue; }
            56 => { r14 = r15.wrapping_add(0x28); bb = 58; continue; }
            57 => { rax = (crate::simd_ops::ru64(r15.wrapping_add(0x28u64))) as u64; r14 = r15.wrapping_add(0x28); rdx = 0xf; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 64; continue; } bb = 58; continue; }
            58 => { rax = (crate::simd_ops::ru64(r14)) as u64; rdx = 0xf; rcx = r14; xmm7 = 0u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 60; continue; } bb = 59; continue; }
            59 => { rdx = 0xf; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm7 = (xmm0) as u128; bb = 60; continue; }
            60 => { rax = (crate::simd_ops::ru64(r15.wrapping_add(0x58u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 63; continue; } bb = 61; continue; }
            61 => { rax = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_l = (((rax) >> (15u32)) & 1) as u64; if ((fl_l as u64) == 0) { bb = 63; continue; } bb = 62; continue; }
            62 => { rcx = (crate::simd_ops::ru64(P12)) as u64; rdx = r15.wrapping_add(0x48); r9 = rsp.wrapping_add(0x38); r8 = 0xf; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 63; continue; }
            63 => { xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) * f32::from_bits(crate::simd_ops::ru32(P14))).to_bits() as u128); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(xmm7 as u32)).to_bits() as u128); bb = 64; continue; }
            64 => { xmm6 = (xmm6 & !0xffff_ffffu128) | ((if f32::from_bits(xmm6 as u32) > f32::from_bits(xmm8 as u32) { f32::from_bits(xmm6 as u32) } else { f32::from_bits(xmm8 as u32) }).to_bits() as u128); r8 = rbp; rcx = r15; xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(crate::simd_ops::ru32(P15))).to_bits() as u128); xmm6 = (xmm6 & !0xffff_ffffu128) | ((if f32::from_bits(xmm6 as u32) > f32::from_bits(crate::simd_ops::ru32(P16)) { f32::from_bits(xmm6 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P16)) }).to_bits() as u128); crate::simd_ops::wu64(rbp.wrapping_add(0x18u64), r12); xmm1 = (xmm6) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P17, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0xd0u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0xe0u64))) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0xf0u64))) as u128; bb = 67; continue; }
            65 => { r14 = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(r14)) as u64; rcx = rcx.wrapping_sub(0x5); rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P18, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::lk_sub_u32(r14.wrapping_add(0x8u64), (1) as u32); rdx = rbp; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 67; continue; }
            66 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x88u64))) as u64; r14 = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(r14)) as u64; rcx = rcx.wrapping_sub(0x5); rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P18, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::lk_sub_u32(r14.wrapping_add(0x8u64), (1) as u32); rdx = rbp; rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P19, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 67; continue; }
            67 => { rbp = ((rax as u32)) as u32 as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 69; continue; } bb = 68; continue; }
            68 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 69; continue; }
            69 => { rdi = 0xffffffff; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 72; continue; } bb = 70; continue; }
            70 => { rcx = rsi.wrapping_add(0x10); rax = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 72; continue; } bb = 71; continue; }
            71 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 72; continue; }
            72 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 75; continue; } bb = 73; continue; }
            73 => { rcx = rbx.wrapping_add(0x10); let xa_src: u32 = (rdi as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rdi = (xa_old) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 75; continue; } bb = 74; continue; }
            74 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 75; continue; }
            75 => { rax = ((rbp as u32)) as u32 as u64; bb = 76; continue; }
            76 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x100u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x108u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x110u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x140u64))) as u64; rsp = ((rsp).wrapping_add(0x118u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415fc9c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f88; const P1: u64 = 0x1426452b0; const P2: u64 = 0x1426452a8; const P3: u64 = 0x142645560; const P4: u64 = 0x1415b16f0; const P5: u64 = 0x141534be0; const P6: u64 = 0x141524330; const P7: u64 = 0x1415bd140; const P8: u64 = 0x14268e420; const P9: u64 = 0x141534ef0; const P10: u64 = 0x1415fcd90; const P11: u64 = 0x1415bd1d0; const P12: u64 = 0x1415b1830;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbx = (crate::simd_ops::ru64(P0)) as u64; r13 = rdx; r14 = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); r12 = 0x1; bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x478u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 12; continue; } bb = 2; continue; }
            2 => { rdx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rdx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x478u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rsi = rbx.wrapping_add(0x480); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x4c0u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbp); rbp = 0; fl_l = 0; fl_r = 0; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 5; continue; }
            5 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r14 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x4b8u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 9; continue; }
            9 => { rdi = rbp; bb = 10; continue; }
            10 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 15; continue; }
            12 => { rsi = rbx.wrapping_add(0x480); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x478u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 4; continue; } bb = 13; continue; }
            13 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 1; continue; }
            14 => { rdi = rbp; bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x478u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 22; continue; } bb = 16; continue; }
            16 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x478u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 19; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x478u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 16; continue; } bb = 18; continue; }
            18 => { bb = 23; continue; }
            19 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 24; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x4a8u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 24; continue; } bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x4b0u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 24; continue; }
            22 => { rsi = rbx.wrapping_add(0x480); bb = 23; continue; }
            23 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 24; continue; }
            24 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 50; continue; } bb = 25; continue; }
            25 => { rcx = rsp.wrapping_add(0x88); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r15); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (crate::simd_ops::ru64(P0)) as u64; rcx = rsp.wrapping_add(0x20); rdx = rsi.wrapping_add(0x478); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = ((rdi.wrapping_add(0x38u64)) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x80u64), ((rdi as u8)) as u8); rcx = ((rdi.wrapping_add(0x2u64)) as u32) as u32 as u64; r15 = (r15 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 26; continue; }
            26 => { rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = P8; rdx = rbx; crate::simd_ops::wu64(rbx, rax); rcx = rsi.wrapping_add(0x4b8); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rdi); r8 = rsp.wrapping_add(0x80); crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), rdi); crate::simd_ops::wu32(rbx.wrapping_add(0x30u64), ((rbp as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), ((r14 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 28; continue; } bb = 27; continue; }
            27 => { r15 = (((r12 as u8)) as u32) as u32 as u64; bb = 43; continue; }
            28 => { crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), ((rbp as u32)) as u32); rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 32; continue; } bb = 29; continue; }
            29 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; rax = P10; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 31; continue; } bb = 30; continue; }
            30 => { rdx = 0xd; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 32; continue; }
            31 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(rdx, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 32; continue; }
            32 => { rdi = ((crate::simd_ops::ru8(rsp.wrapping_add(0x80u64))) as u32) as u32 as u64; bb = 33; continue; }
            33 => { rbx = rbp; fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 43; continue; } bb = 34; continue; }
            34 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x4c0u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 42; continue; } bb = 35; continue; }
            35 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r14 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x4b8u64))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 39; continue; } bb = 36; continue; }
            36 => { bb = 37; continue; }
            37 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 40; continue; } bb = 38; continue; }
            38 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 37; continue; } bb = 39; continue; }
            39 => { rbx = rbp; bb = 40; continue; }
            40 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 43; continue; } bb = 41; continue; }
            41 => { fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 43; continue; }
            42 => { rbx = rbp; bb = 43; continue; }
            43 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = (crate::simd_ops::lk_add_u32(rax, (1) as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; if (((fl_l as u32)) != 0) { bb = 45; continue; } bb = 44; continue; }
            44 => { crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((rbp as u32)) as u32); bb = 45; continue; }
            45 => { rcx = ((rcx).wrapping_add(0x8u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 47; continue; } bb = 46; continue; }
            46 => { r12 = ((rbx.wrapping_add(0x34u64)) as u32) as u32 as u64; bb = 49; continue; }
            47 => { fl_l = ((r15 as u8)) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 49; continue; } bb = 48; continue; }
            48 => { xmm0 = (crate::simd_ops::ru128(r13)) as u128; crate::simd_ops::wu128(rbx.wrapping_add(0x18u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(r13.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), ((xmm1 as u64)) as u64); rcx = (crate::simd_ops::ru32(r13.wrapping_add(0x18u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x30u64), ((rcx as u32)) as u32); bb = 49; continue; }
            49 => { rcx = rsp.wrapping_add(0x88); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 50; continue; }
            50 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = ((r12 as u32)) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415fcc70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f88; const P1: u64 = 0x1426452b0; const P2: u64 = 0x1426452a8; const P3: u64 = 0x142645560;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = (crate::simd_ops::ru64(P0)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbp); rbp = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x478u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 12; continue; } bb = 2; continue; }
            2 => { rdx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rdx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x478u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rsi = rbx.wrapping_add(0x480); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x4c0u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 5; continue; }
            5 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbp as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x4b8u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 9; continue; }
            9 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            10 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 15; continue; }
            12 => { rsi = rbx.wrapping_add(0x480); rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x478u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 4; continue; } bb = 13; continue; }
            13 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            14 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x478u64))) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 22; continue; } bb = 16; continue; }
            16 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x478u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 19; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x478u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 16; continue; } bb = 18; continue; }
            18 => { bb = 23; continue; }
            19 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 24; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x4a8u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 24; continue; } bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x4b0u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 24; continue; }
            22 => { rsi = rbx.wrapping_add(0x480); bb = 23; continue; }
            23 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 24; continue; }
            24 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 25; continue; }
            25 => { rax = rdi.wrapping_add(0x18); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            26 => { rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415fcd90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415bd1d0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rdx = 0xd;
    return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415fcda0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140000000;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x5u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 8; continue; } bb = 1; continue; }
            1 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; r8 = ((P0) as u64) as u64; rdx = (crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x15fcde0u64))) as u32 as u64; fl_l = ((rdx).wrapping_add(r8)) as u64 as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; j_tgt = (rdx) as u64; bb = 0xffff_fffe; continue; }
            2 => { crate::simd_ops::wu32(rcx.wrapping_add(0x1cu64), ((xmm2 as u32)) as u32); return rax; }
            3 => { crate::simd_ops::wu32(rcx.wrapping_add(0x20u64), ((xmm2 as u32)) as u32); return rax; }
            4 => { crate::simd_ops::wu32(rcx.wrapping_add(0x24u64), ((xmm2 as u32)) as u32); return rax; }
            5 => { crate::simd_ops::wu32(rcx.wrapping_add(0x28u64), ((xmm2 as u32)) as u32); return rax; }
            6 => { crate::simd_ops::wu32(rcx.wrapping_add(0x2cu64), ((xmm2 as u32)) as u32); return rax; }
            7 => { crate::simd_ops::wu32(rcx.wrapping_add(0x30u64), ((xmm2 as u32)) as u32); bb = 8; continue; }
            8 => { return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x1415fcda0u64) { 0x5 => { bb = 1; continue; } 0x1c => { bb = 2; continue; } 0x22 => { bb = 3; continue; } 0x28 => { bb = 4; continue; } 0x2e => { bb = 5; continue; } 0x34 => { bb = 6; continue; } 0x3a => { bb = 7; continue; } 0x3f => { bb = 8; continue; } _ => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(j_tgt, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; } } }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415fce00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rcx, ((rax as u32)) as u32);
    crate::simd_ops::wu16(rcx.wrapping_add(P0), ((rax as u16)) as u16);
    rax = 0x1;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415fce10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434268d0; const P1: u64 = 0x143427068; const P2: u64 = 0x143426780; const P3: u64 = 0x143426c2c; const P4: u64 = 0x14342698c;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; r9 = rcx; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rcx.wrapping_add(0xcu64), (0x4u32) as u32); rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rdi = 0; fl_l = 0; fl_r = 0; xmm3 = ((crate::simd_ops::ru32(P0)) as u32) as u128; rdx = ((rdi as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { bb = 5; continue; }
            5 => { rax = ((rdx as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(r9)) as u64; xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x4u64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm3 as u32)).to_bits() as u128); crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x4u64), ((xmm0 as u32)) as u32); rax = (crate::simd_ops::ru32(r9.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 6; continue; }
            6 => { rdx = ((rax as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(r9)) as u64; crate::simd_ops::wu32(r9.wrapping_add(0xcu64), ((rdi as u32)) as u32); rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; r8 = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rcx = ((rdi as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 24; continue; } bb = 7; continue; }
            7 => { xmm4 = ((crate::simd_ops::ru32(P1)) as u32) as u128; xmm5 = ((crate::simd_ops::ru32(P2)) as u32) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); crate::simd_ops::wu128(rsp.wrapping_add(0x10u64), (xmm6) as u128); xmm6 = ((crate::simd_ops::ru32(P3)) as u32) as u128; crate::simd_ops::wu128(rsp, (xmm7) as u128); xmm7 = ((crate::simd_ops::ru32(P4)) as u32) as u128; bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(r9)) as u64; r10 = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rbx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; xmm1 = (xmm3) as u128; r11 = ((rbx.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; xmm2 = ((crate::simd_ops::ru32(rax.wrapping_add(r11.wrapping_mul(4)).wrapping_add(0x4u64))) as u32) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(crate::simd_ops::ru32(rax.wrapping_add(r10.wrapping_mul(4)).wrapping_add(0x4u64)))).to_bits() as u128); crate::simd_ops::wu32(r8.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x4u64), ((xmm2 as u32)) as u32); rax = (crate::simd_ops::ru64(r9)) as u64; xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(r11.wrapping_mul(4)))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(crate::simd_ops::ru32(rax.wrapping_add(r10.wrapping_mul(4))))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); crate::simd_ops::wu32(r8.wrapping_add(rcx.wrapping_mul(8)), ((xmm1 as u32)) as u32); rax = (crate::simd_ops::ru64(r9)) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(r10.wrapping_mul(4)).wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x9; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu32(r8.wrapping_add(rcx.wrapping_mul(8)), ((rdi as u32)) as u32); bb = 22; continue; }
            10 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(r11.wrapping_mul(4)).wrapping_add(0x4u64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(crate::simd_ops::ru32(rax.wrapping_add(r10.wrapping_mul(4)).wrapping_add(0x4u64)))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); crate::simd_ops::wu32(r8.wrapping_add(rcx.wrapping_mul(8)), ((xmm0 as u32)) as u32); bb = 22; continue; }
            12 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 20; continue; } bb = 13; continue; }
            13 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x7; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 20; continue; } bb = 14; continue; }
            14 => { rax = ((rdx.wrapping_sub(0x3u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xfffffffd; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 19; continue; } bb = 15; continue; }
            15 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); crate::simd_ops::wu32(r8.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x4u64), ((xmm2 as u32)) as u32); bb = 22; continue; }
            17 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x6; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 22; continue; } bb = 18; continue; }
            18 => { xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); crate::simd_ops::wu32(r8.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x4u64), ((xmm2 as u32)) as u32); bb = 22; continue; }
            19 => { xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); bb = 21; continue; }
            20 => { xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm7 as u32)).to_bits() as u128); bb = 21; continue; }
            21 => { crate::simd_ops::wu32(r8.wrapping_add(rcx.wrapping_mul(8)), ((xmm1 as u32)) as u32); bb = 22; continue; }
            22 => { rcx = ((rbx as u32)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r9.wrapping_add(0x8u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 8; continue; } bb = 23; continue; }
            23 => { xmm7 = (crate::simd_ops::ru128(rsp)) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 24; continue; }
            24 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415fcfa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415242e0; const P1: u64 = 0x141524330; const P2: u64 = 0x14183acbc; const P3: u64 = 0x141540c30; const P4: u64 = 0x1415fce10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; rdi = ((r9 as u32)) as u32 as u64; rbp = ((((r8 as u32)) as i32 as i64) as u64) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_add(rbp)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rbx = ((rcx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), r14); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), r14); crate::simd_ops::wu32(rbx.wrapping_add(0x18u64), (0x1u32) as u32); fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 3; continue; }
            3 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 4; continue; }
            4 => { rax = ((rdi.wrapping_add(0x2u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x18u64), (0x1u32) as u32); rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), ((rax as u32)) as u32); rdx = (((rdx as u32)).wrapping_shl(2u32)) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x58u64))) as u32 as u64; r8 = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u32) as u32 as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rcx = ((rcx).wrapping_add(0xcu64)) as u64; r8 = (((r8 as u32)).wrapping_shl(2u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x14u64), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(0x8u64), (0x9u32) as u32); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu32(rax, (0xff7fffffu32) as u32); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x8u64), (0x9u32) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(4)), (0x7f7fffffu32) as u32); rcx = rbx.wrapping_add(0x8); rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; r9 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = ((rdx.wrapping_sub(0x2u64)) as u32) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((rdx.wrapping_sub(0x1u64)) as u32) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru32(r9.wrapping_add(r8.wrapping_mul(4)).wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(0x4u64), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = rbx.wrapping_add(0x8); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rax = 0x1; bb = 10; continue; }
            8 => { crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), ((r14 as u32)) as u32); rax = 0x34; bb = 10; continue; }
            9 => { rax = 0x1f; bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415fd100(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1415242e0; const P2: u64 = 0x10; const P3: u64 = 0x1;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = ((rcx.wrapping_add(P0)) as u64) as u64; rsi = 0x2; rbp = 0; fl_l = 0; fl_r = 0; bb = 1; continue; }
            1 => { rdi = 0x3; bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rbx, rbp); crate::simd_ops::wu64(rbx.wrapping_add(P0), rbp); crate::simd_ops::wu32(rbx.wrapping_add(P2), (0x1u32) as u32); rbx = ((rbx).wrapping_add(0x18u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = ((rdi.wrapping_add(P3)) as u32) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415fd170(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524330; const P1: u64 = 0x14268e4c0; const P2: u64 = 0x1415af330; const P3: u64 = 0x1415fd450; const P4: u64 = 0x143930fa0; const P5: u64 = 0x1415c4950; const P6: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rsi = rcx; rdx = 0x80; r14 = (((r9 as u8)) as u32) as u32 as u64; r15 = (((r8 as u8)) as u32) as u32 as u64; rcx = ((rdx.wrapping_sub(0x70u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbp = 0; fl_l = 0; fl_r = 0; rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), ((rbp as u32)) as u32); rax = P1; crate::simd_ops::wu64(rdi, rax); crate::simd_ops::wu32(rdi.wrapping_add(0x18u64), ((rbp as u32)) as u32); crate::simd_ops::wu8(rdi.wrapping_add(0x20u64), (0xffu8) as u8); crate::simd_ops::wu8(rdi.wrapping_add(0x28u64), (0xffu8) as u8); crate::simd_ops::wu64(rdi.wrapping_add(0x30u64), rbp); crate::simd_ops::wu64(rdi.wrapping_add(0x40u64), rbp); crate::simd_ops::wu64(rdi.wrapping_add(0x48u64), rbp); crate::simd_ops::wu64(rdi.wrapping_add(0x50u64), rbp); crate::simd_ops::wu64(rdi.wrapping_add(0x58u64), rbp); crate::simd_ops::wu64(rdi.wrapping_add(0x60u64), rbp); crate::simd_ops::wu64(rdi.wrapping_add(0x68u64), rbp); crate::simd_ops::wu16(rdi.wrapping_add(0x70u64), ((rbp as u16)) as u16); crate::simd_ops::wu64(rdi.wrapping_add(0x78u64), rbp); crate::simd_ops::wu64(rdi.wrapping_add(0x38u64), rsi); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rbx); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x60u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 7; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x64u64))) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } bb = 4; continue; }
            4 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x60u64), ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x64u64))) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            7 => { fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x60u64), ((rax as u32)) as u32); bb = 8; continue; }
            8 => { r9 = rdi.wrapping_add(0x40); crate::simd_ops::wu8(rdi.wrapping_add(0x70u64), ((r15 as u8)) as u8); r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rdi.wrapping_add(0x71u64), ((r14 as u8)) as u8); rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; rsi = (rsi & !0xffu64) | (((0x1u8) as u8 as u64)); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x48u64))) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 9; continue; }
            9 => { fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; r8 = rdi; rcx = (crate::simd_ops::ru64(P4)) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = (((rsi as u8)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rdx = ((rbp as u32)) as u32 as u64; rdx = (if (((fl_l as u32)) == ((fl_r as u32))) { (rcx as u32) } else { (rdx as u32) }) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; rsi = (((rdx as u8)) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x48u64))) as u32 as u64; rdx = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 11; continue; }
            11 => { fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = rdi; bb = 15; continue; }
            13 => { rdx = rdi; rcx = 0x10; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { rax = rbp; bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415fd2c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143932030; const P1: u64 = 0x143930fa0; const P2: u64 = 0x1415c53e0; const P3: u64 = 0x1415af340; const P4: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x50u64))) as u64; rdi = rcx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x58u64))) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(P0)) as u64; r8 = ((crate::simd_ops::ru8(rdi.wrapping_add(0x70u64))) as u32) as u32 as u64; rdx = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x58u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x4u64)) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x50u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x68u64))) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(P0)) as u64; r8 = ((crate::simd_ops::ru8(rdi.wrapping_add(0x71u64))) as u32) as u32 as u64; rdx = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x68u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x4u64)) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(0x58u64), ((rsi as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0x68u64), ((rsi as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x48u64))) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; r8 = rdi; rcx = (crate::simd_ops::ru64(P1)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x48u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x60u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 11; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x64u64))) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdx = (rdx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 12; continue; }
            11 => { rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 12; continue; }
            12 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x60u64), ((rcx as u32)) as u32); if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 13; continue; }
            13 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 17; continue; } bb = 14; continue; }
            14 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 17; continue; } bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x64u64))) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x60u64))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0x4; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 19; continue; }
            19 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x50u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(0x58u64), ((rsi as u32)) as u32); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(0x50u64), rsi); crate::simd_ops::wu32(rdi.wrapping_add(0x5cu64), ((rsi as u32)) as u32); bb = 21; continue; }
            21 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(0x68u64), ((rsi as u32)) as u32); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(0x60u64), rsi); crate::simd_ops::wu32(rdi.wrapping_add(0x6cu64), ((rsi as u32)) as u32); bb = 23; continue; }
            23 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 24; continue; }
            24 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(0x48u64), ((rsi as u32)) as u32); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(0x40u64), rsi); crate::simd_ops::wu32(rdi.wrapping_add(0x4cu64), ((rsi as u32)) as u32); bb = 25; continue; }
            25 => { rdx = rdi; rcx = 0x10; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415fd450(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141605fa0; const P1: u64 = 0x1415af950; const P2: u64 = 0x14183acc8; const P3: u64 = 0x143932030; const P4: u64 = 0x1415242e0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rbp = rax.wrapping_sub(0x5f); rsp = ((rsp).wrapping_sub(0xc8u64)) as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(0x50u64))) as u128; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rsi); rsi = rcx; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rdi); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), r12); crate::simd_ops::wu64(rax.wrapping_sub(0x18u64), r13); crate::simd_ops::wu64(rax.wrapping_sub(0x20u64), r14); crate::simd_ops::wu64(rax.wrapping_sub(0x28u64), r15); r15 = rcx.wrapping_add(0x60); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r8 as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), rax); r8 = rbp.wrapping_sub(0x29); crate::simd_ops::wu64(rbp.wrapping_sub(0x21u64), rax); crate::simd_ops::wu128(rbp.wrapping_sub(0x19u64), (xmm0) as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rdx as u32)) as u32); rdx = rbp.wrapping_sub(0x39); xmm0 = (crate::simd_ops::ru128(r15)) as u128; crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), rax); crate::simd_ops::wu64(rbp.wrapping_sub(0x31u64), rax); crate::simd_ops::wu128(rbp.wrapping_sub(0x9u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x29u64))) as u128; crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), rax); crate::simd_ops::wu64(rbp.wrapping_sub(0x21u64), rax); crate::simd_ops::wu128(rcx.wrapping_add(0x50u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x29u64))) as u128; crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), rax); crate::simd_ops::wu64(rbp.wrapping_sub(0x21u64), rax); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu128(r15, (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x39u64))) as u64; rdi = (crate::simd_ops::ru32(rbp.wrapping_sub(0x31u64))) as u32 as u64; rcx = ((rbx.wrapping_add(rdi.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 33; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru32(rsi.wrapping_add(0x58u64))) as u32 as u64; r11 = 0; fl_l = 0; fl_r = 0; r13 = (crate::simd_ops::ru64(rbx)) as u64; rdx = ((r8 as u32)) as u32 as u64; r14 = (crate::simd_ops::ru32(r13.wrapping_add(0x4u64))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { r15 = (crate::simd_ops::ru64(rsi.wrapping_add(0x50u64))) as u64; bb = 4; continue; }
            4 => { rcx = ((rdx as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(1u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_add((r11 as u32))) as u32 as u64; rax = ((((rcx as u32)) as i32 as i64) as u64) as u64; fl_l = (crate::simd_ops::ru32(r15.wrapping_add(rax.wrapping_mul(4)))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; r10 = ((r15.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 19; continue; } bb = 5; continue; }
            5 => { rax = ((rdx.wrapping_sub(0x1u64)) as u32) as u32 as u64; rax = (if (((fl_l as u32)) >= ((fl_r as u32))) { (rdx as u32) } else { (rax as u32) }) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = (if (((fl_l as u32)) >= ((fl_r as u32))) { (r11 as u32) } else { (rax as u32) }) as u32 as u64; fl_l = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; rdx = ((fl_l as u32)) as u32 as u64; r11 = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 4; continue; } bb = 6; continue; }
            6 => { r15 = (crate::simd_ops::ru64(rsi.wrapping_add(0x50u64))) as u64; bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru32(r13.wrapping_add(0x8u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 31; continue; } bb = 8; continue; }
            8 => { rdi = 0; fl_l = 0; fl_r = 0; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { bb = 10; continue; }
            10 => { rax = ((r8 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; rax = (((rax as u32)).wrapping_add((rdi as u32))) as u32 as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; rdx = (crate::simd_ops::ru32(r15.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 27; continue; } bb = 11; continue; }
            11 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; rax = (if (((fl_l as u32)) >= ((fl_r as u32))) { (rdi as u32) } else { (rax as u32) }) as u32 as u64; rdi = ((rax as u32)) as u32 as u64; rax = ((r8.wrapping_sub(0x1u64)) as u32) as u32 as u64; rax = (if (((fl_l as u32)) >= ((fl_r as u32))) { (r8 as u32) } else { (rax as u32) }) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; fl_l = (((r8 as u32)).wrapping_shr(1u32)) as u32 as u64; r8 = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 10; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x50u64))) as u64; r15 = (crate::simd_ops::ru32(rsi.wrapping_add(0x58u64))) as u32 as u64; rcx = ((((rdi as u32)) as i32 as i64) as u64) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x5cu64))) as u32 as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 13; continue; }
            13 => { fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rcx = rsi.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 27; continue; } bb = 15; continue; }
            15 => { fl_l = ((r15 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsi.wrapping_add(0x5cu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 27; continue; } bb = 16; continue; }
            16 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x50u64))) as u64; rax = ((rdi as u32)) as u32 as u64; r15 = ((rax.wrapping_mul(4)) as u64) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x58u64))) as u32 as u64; rdx = ((rdx).wrapping_add(r15)) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 18; continue; } bb = 17; continue; }
            17 => { rax = (((rax as u32)).wrapping_sub((rdi as u32))) as u32 as u64; rcx = rdx.wrapping_add(0x4); rax = (((rax as u32)).wrapping_shl(2u32)) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 18; continue; }
            18 => { crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x58u64), (1) as u32); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x50u64))) as u64; fl_l = ((rcx).wrapping_add(r15)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 25; continue; }
            19 => { fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 31; continue; } bb = 20; continue; }
            20 => { bb = 7; continue; }
            21 => { fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rcx = rsi.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 27; continue; } bb = 23; continue; }
            23 => { fl_l = ((r15 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsi.wrapping_add(0x5cu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 27; continue; } bb = 24; continue; }
            24 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x58u64))) as u32 as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x58u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x50u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; bb = 25; continue; }
            25 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 26; continue; }
            26 => { crate::simd_ops::wu32(rcx, ((r14 as u32)) as u32); bb = 27; continue; }
            27 => { rcx = (crate::simd_ops::ru32(r13.wrapping_add(0xcu64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x500; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 29; continue; } bb = 28; continue; }
            28 => { rdx = (crate::simd_ops::ru32(r13.wrapping_add(0x4u64))) as u32 as u64; bb = 30; continue; }
            29 => { rdx = 0xffffffff; bb = 30; continue; }
            30 => { rax = ((crate::simd_ops::ru16(r13.wrapping_add(0x1au64))) as u32) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; r9 = ((crate::simd_ops::ru8(rsi.wrapping_add(0x70u64))) as u32) as u32 as u64; rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0x1u8))) as u8 as u64)); crate::simd_ops::wu8(rbp.wrapping_add(0x1fu64), ((rcx as u8)) as u8); rcx = (crate::simd_ops::ru64(P3)) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0xbu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r13.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x7u64), ((r8 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0xfu64), ((r8 as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x17u64), r8); crate::simd_ops::wu32(rbp.wrapping_add(0x23u64), ((rdx as u32)) as u32); rdx = ((r14 as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x2bu64), ((r8 as u32)) as u32); r8 = rbp.wrapping_add(0x7); crate::simd_ops::wu32(rbp.wrapping_add(0x27u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru32(rbp.wrapping_sub(0x31u64))) as u32 as u64; bb = 31; continue; }
            31 => { rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x39u64))) as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rcx = ((rdi as u32)) as u32 as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 32; continue; }
            32 => { r15 = rsi.wrapping_add(0x60); bb = 33; continue; }
            33 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x29u64))) as u64; r14 = (crate::simd_ops::ru32(rbp.wrapping_sub(0x21u64))) as u32 as u64; rcx = ((rbx.wrapping_add(r14.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 73; continue; } bb = 34; continue; }
            34 => { bb = 35; continue; }
            35 => { r13 = (crate::simd_ops::ru64(rbx)) as u64; r9 = 0; fl_l = 0; fl_r = 0; rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x58u64))) as u32 as u64; rdi = (crate::simd_ops::ru32(r13.wrapping_add(0x4u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 41; continue; } bb = 36; continue; }
            36 => { r11 = (crate::simd_ops::ru64(rsi.wrapping_add(0x50u64))) as u64; bb = 37; continue; }
            37 => { rcx = ((rdx as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(1u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_add((r9 as u32))) as u32 as u64; rax = ((((rcx as u32)) as i32 as i64) as u64) as u64; fl_l = (crate::simd_ops::ru32(r11.wrapping_add(rax.wrapping_mul(4)))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; r10 = ((r11.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 40; continue; } bb = 38; continue; }
            38 => { rax = ((rdx.wrapping_sub(0x1u64)) as u32) as u32 as u64; rax = (if (((fl_l as u32)) >= ((fl_r as u32))) { (rdx as u32) } else { (rax as u32) }) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = (if (((fl_l as u32)) >= ((fl_r as u32))) { (r9 as u32) } else { (rax as u32) }) as u32 as u64; fl_l = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; rdx = ((fl_l as u32)) as u32 as u64; r9 = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 37; continue; } bb = 39; continue; }
            39 => { bb = 41; continue; }
            40 => { fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 72; continue; } bb = 41; continue; }
            41 => { r8 = (crate::simd_ops::ru32(rsi.wrapping_add(0x68u64))) as u32 as u64; r11 = 0; fl_l = 0; fl_r = 0; rdx = ((r8 as u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 45; continue; } bb = 42; continue; }
            42 => { r15 = (crate::simd_ops::ru64(r15)) as u64; bb = 43; continue; }
            43 => { rcx = ((rdx as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(1u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_add((r11 as u32))) as u32 as u64; rax = ((((rcx as u32)) as i32 as i64) as u64) as u64; fl_l = (crate::simd_ops::ru32(r15.wrapping_add(rax.wrapping_mul(4)))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; r10 = ((r15.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 59; continue; } bb = 44; continue; }
            44 => { rax = ((rdx.wrapping_sub(0x1u64)) as u32) as u32 as u64; rax = (if (((fl_l as u32)) >= ((fl_r as u32))) { (rdx as u32) } else { (rax as u32) }) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = (if (((fl_l as u32)) >= ((fl_r as u32))) { (r11 as u32) } else { (rax as u32) }) as u32 as u64; fl_l = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; rdx = ((fl_l as u32)) as u32 as u64; r11 = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 43; continue; } bb = 45; continue; }
            45 => { r15 = (crate::simd_ops::ru64(rsi.wrapping_add(0x60u64))) as u64; r9 = rsi.wrapping_add(0x60); bb = 46; continue; }
            46 => { fl_l = (crate::simd_ops::ru32(r13.wrapping_add(0x8u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 72; continue; } bb = 47; continue; }
            47 => { r14 = 0; fl_l = 0; fl_r = 0; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 51; continue; } bb = 48; continue; }
            48 => { bb = 49; continue; }
            49 => { rcx = ((r8 as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(1u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_add((r14 as u32))) as u32 as u64; rax = ((((rcx as u32)) as i32 as i64) as u64) as u64; fl_l = (crate::simd_ops::ru32(r15.wrapping_add(rax.wrapping_mul(4)))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 68; continue; } bb = 50; continue; }
            50 => { rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = (if (((fl_l as u32)) >= ((fl_r as u32))) { (r14 as u32) } else { (rax as u32) }) as u32 as u64; r14 = ((rax as u32)) as u32 as u64; rax = ((r8.wrapping_sub(0x1u64)) as u32) as u32 as u64; rax = (if (((fl_l as u32)) >= ((fl_r as u32))) { (r8 as u32) } else { (rax as u32) }) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; fl_l = (((r8 as u32)).wrapping_shr(1u32)) as u32 as u64; r8 = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 49; continue; } bb = 51; continue; }
            51 => { rax = (crate::simd_ops::ru64(r9)) as u64; r15 = (crate::simd_ops::ru32(rsi.wrapping_add(0x68u64))) as u32 as u64; rcx = ((((r14 as u32)) as i32 as i64) as u64) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x6cu64))) as u32 as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 61; continue; } bb = 52; continue; }
            52 => { fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 55; continue; } bb = 53; continue; }
            53 => { rcx = r9; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 68; continue; } bb = 54; continue; }
            54 => { r9 = rsi.wrapping_add(0x60); bb = 55; continue; }
            55 => { fl_l = ((r15 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsi.wrapping_add(0x6cu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 68; continue; } bb = 56; continue; }
            56 => { rdx = (crate::simd_ops::ru64(r9)) as u64; rax = ((r14 as u32)) as u32 as u64; r15 = ((rax.wrapping_mul(4)) as u64) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x68u64))) as u32 as u64; rdx = ((rdx).wrapping_add(r15)) as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 58; continue; } bb = 57; continue; }
            57 => { rax = (((rax as u32)).wrapping_sub((r14 as u32))) as u32 as u64; rcx = rdx.wrapping_add(0x4); rax = (((rax as u32)).wrapping_shl(2u32)) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = rsi.wrapping_add(0x60); bb = 58; continue; }
            58 => { crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x68u64), (1) as u32); rcx = (crate::simd_ops::ru64(r9)) as u64; fl_l = ((rcx).wrapping_add(r15)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 66; continue; }
            59 => { fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 72; continue; } bb = 60; continue; }
            60 => { r9 = rsi.wrapping_add(0x60); bb = 46; continue; }
            61 => { fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 64; continue; } bb = 62; continue; }
            62 => { rcx = r9; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 68; continue; } bb = 63; continue; }
            63 => { r9 = rsi.wrapping_add(0x60); bb = 64; continue; }
            64 => { fl_l = ((r15 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsi.wrapping_add(0x6cu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 68; continue; } bb = 65; continue; }
            65 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x68u64))) as u32 as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x68u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(r9)) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; bb = 66; continue; }
            66 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 68; continue; } bb = 67; continue; }
            67 => { crate::simd_ops::wu32(rcx, ((rdi as u32)) as u32); bb = 68; continue; }
            68 => { rcx = (crate::simd_ops::ru32(r13.wrapping_add(0xcu64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x500; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 70; continue; } bb = 69; continue; }
            69 => { rdx = (crate::simd_ops::ru32(r13.wrapping_add(0x4u64))) as u32 as u64; bb = 71; continue; }
            70 => { rdx = 0xffffffff; bb = 71; continue; }
            71 => { rax = ((crate::simd_ops::ru16(r13.wrapping_add(0x1au64))) as u32) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; r9 = ((crate::simd_ops::ru8(rsi.wrapping_add(0x71u64))) as u32) as u32 as u64; rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0x1u8))) as u8 as u64)); crate::simd_ops::wu8(rbp.wrapping_add(0x1fu64), ((rcx as u8)) as u8); rcx = (crate::simd_ops::ru64(P3)) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0xbu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r13.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x7u64), ((r8 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0xfu64), ((r8 as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x17u64), r8); crate::simd_ops::wu32(rbp.wrapping_add(0x23u64), ((rdx as u32)) as u32); rdx = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x2bu64), ((r8 as u32)) as u32); r8 = rbp.wrapping_add(0x7); crate::simd_ops::wu32(rbp.wrapping_add(0x27u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = (crate::simd_ops::ru32(rbp.wrapping_sub(0x21u64))) as u32 as u64; bb = 72; continue; }
            72 => { rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x29u64))) as u64; r15 = rsi.wrapping_add(0x60); rcx = ((r14 as u32)) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 35; continue; } bb = 73; continue; }
            73 => { r15 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x19u64))) as u64; rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x11u64))) as u32 as u64; rbx = r15; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; rdi = ((r15.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; fl_l = r15; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 76; continue; } bb = 74; continue; }
            74 => { bb = 75; continue; }
            75 => { rcx = (crate::simd_ops::ru64(P3)) as u64; r8 = ((crate::simd_ops::ru8(rsi.wrapping_add(0x70u64))) as u32) as u32 as u64; rdx = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x4u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 75; continue; } bb = 76; continue; }
            76 => { r14 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x9u64))) as u64; rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x1u64))) as u32 as u64; rbx = r14; rdi = ((r14.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; fl_l = r14; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 78; continue; } bb = 77; continue; }
            77 => { rcx = (crate::simd_ops::ru64(P3)) as u64; r8 = ((crate::simd_ops::ru8(rsi.wrapping_add(0x71u64))) as u32) as u32 as u64; rdx = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x4u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 77; continue; } bb = 78; continue; }
            78 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xe8u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 80; continue; } bb = 79; continue; }
            79 => { rdx = r15; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 80; continue; }
            80 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 82; continue; } bb = 81; continue; }
            81 => { rdx = r14; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 82; continue; }
            82 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x39u64))) as u64; rbx = 0; fl_l = 0; fl_r = 0; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 84; continue; } bb = 83; continue; }
            83 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbp.wrapping_sub(0x31u64), ((rbx as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), rbx); crate::simd_ops::wu32(rbp.wrapping_sub(0x2du64), ((rbx as u32)) as u32); bb = 84; continue; }
            84 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x29u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 86; continue; } bb = 85; continue; }
            85 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbp.wrapping_sub(0x21u64), ((rbx as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 86; continue; }
            86 => { rsp = ((rsp).wrapping_add(0xc8u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415fd9f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426ea4; const P1: u64 = 0x143931cb0; const P2: u64 = 0x1426452b0; const P3: u64 = 0x143931ce0; const P4: u64 = 0x143931cd8; const P5: u64 = 0x143932030; const P6: u64 = 0x1426452a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbp = rcx; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); xmm6 = ((crate::simd_ops::ru32(P0)) as u32) as u128; rcx = P1; crate::simd_ops::wu8(r9, (0x0u8) as u8); rsi = r9; r14 = r8; crate::simd_ops::wu32(r8, (0x0u32) as u32); rbx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru32(P3)) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 28; continue; } bb = 1; continue; }
            1 => { rdx = 0; fl_l = 0; fl_r = 0; rax = rbx; rax = ((rax).wrapping_shr(4u32)) as u64; rcx = rbp; rcx = ((rcx).wrapping_shr(4u32)) as u64; rax = ((((rax as u32)) ^ ((rcx as u32)))) as u32 as u64; let d = ((r9 as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(P4)) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x38u64))) as u64; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x78u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 5; continue; }
            5 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x50u64))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x58u64))) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(P5)) as u64; r9 = rsp.wrapping_add(0x50); rdx = (crate::simd_ops::ru32(rbx)) as u32 as u64; r8 = rsp.wrapping_add(0x58); rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x58u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 28; continue; } bb = 10; continue; }
            10 => { xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x58u64))) as u32) as u128; fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { xmm6 = (xmm0) as u128; bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 15; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x50u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 16; continue; }
            15 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 16; continue; }
            16 => { crate::simd_ops::wu8(rsi, ((rax as u8)) as u8); rbx = ((rbx).wrapping_add(0x4u64)) as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x58u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x50u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 17; continue; }
            17 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x68u64))) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 27; continue; } bb = 18; continue; }
            18 => { bb = 19; continue; }
            19 => { rcx = (crate::simd_ops::ru64(P5)) as u64; r9 = rsp.wrapping_add(0x50); rdx = (crate::simd_ops::ru32(rbx)) as u32 as u64; r8 = rsp.wrapping_add(0x58); rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x58u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 28; continue; } bb = 20; continue; }
            20 => { xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x58u64))) as u32) as u128; fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 22; continue; } bb = 21; continue; }
            21 => { xmm6 = (xmm0) as u128; bb = 22; continue; }
            22 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 25; continue; } bb = 23; continue; }
            23 => { fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x50u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 25; continue; } bb = 24; continue; }
            24 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 26; continue; }
            25 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 26; continue; }
            26 => { crate::simd_ops::wu8(rsi, ((rax as u8)) as u8); rbx = ((rbx).wrapping_add(0x4u64)) as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x68u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 19; continue; } bb = 27; continue; }
            27 => { crate::simd_ops::wu32(r14, ((xmm6 as u32)) as u32); rbx = 0x1; bb = 29; continue; }
            28 => { rbx = 0x2; bb = 29; continue; }
            29 => { rcx = P1; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rax = ((rbx as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415fdba0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931cb0; const P1: u64 = 0x1426452b0; const P2: u64 = 0x1415fd450; const P3: u64 = 0x1415fde80; const P4: u64 = 0x1426452a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rdi = rcx;
    rbx = rdx;
    rcx = P0;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp);
    r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64;
    r9 = rsp.wrapping_add(0x20);
    rax = 0;
    fl_l = 0; fl_r = 0;
    rdx = ((rbx as u32)) as u32 as u64;
    rcx = rdi;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rdx = rsp.wrapping_add(0x20);
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rcx = P0;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415fdc10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931cb0; const P1: u64 = 0x1426452b0; const P2: u64 = 0x143931ce0; const P3: u64 = 0x143931cd8; const P4: u64 = 0x143932030; const P5: u64 = 0x143931ce8; const P6: u64 = 0x143931ce4; const P7: u64 = 0x14159a6d0; const P8: u64 = 0x1415fd170; const P9: u64 = 0x143426890; const P10: u64 = 0x1415fe0c0; const P11: u64 = 0x1426452a8;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rsi = rcx; rbp = (((r9 as u8)) as u32) as u32 as u64; rcx = P0; r14 = (((r8 as u8)) as u32) as u32 as u64; rbx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru32(P2)) as u32 as u64; r15 = 0; fl_l = 0; fl_r = 0; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 17; continue; } bb = 1; continue; }
            1 => { rax = rsi; rdx = rbx; rax = ((rax).wrapping_shr(4u32)) as u64; rdx = ((rdx).wrapping_shr(4u32)) as u64; rax = ((((rax as u32)) ^ ((rdx as u32)))) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; let d = ((r8 as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(P3)) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x38u64))) as u64; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x78u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 6; continue; }
            6 => { rdi = r15; bb = 7; continue; }
            7 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 8; continue; }
            8 => { fl_l = ((r14 as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rdi.wrapping_add(0x70u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = ((rbp as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rdi.wrapping_add(0x71u64))) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 29; continue; } bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x50u64))) as u64; rsi = rdi; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x58u64))) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 11; continue; }
            11 => { bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(P4)) as u64; r8 = (((r14 as u8)) as u32) as u32 as u64; r9 = ((crate::simd_ops::ru8(rdi.wrapping_add(0x70u64))) as u32) as u32 as u64; rdx = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x58u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x4u64)) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x50u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x60u64))) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x68u64))) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 14; continue; }
            14 => { bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(P4)) as u64; r8 = (((rbp as u8)) as u32) as u32 as u64; r9 = ((crate::simd_ops::ru8(rdi.wrapping_add(0x71u64))) as u32) as u32 as u64; rdx = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x68u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x4u64)) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x60u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu8(rdi.wrapping_add(0x70u64), ((r14 as u8)) as u8); crate::simd_ops::wu8(rdi.wrapping_add(0x71u64), ((rbp as u8)) as u8); bb = 29; continue; }
            17 => { fl_l = (crate::simd_ops::ru32(P5)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 23; continue; } bb = 18; continue; }
            18 => { crate::simd_ops::wu32(P5, ((r15 as u32)) as u32); fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x1d; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 21; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru32(P6)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1d; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 21; continue; } bb = 20; continue; }
            20 => { rdx = 0x1d; rcx = P3; rdx = (((rdx as u32)).wrapping_sub((rax as u32))) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 23; continue; } bb = 21; continue; }
            21 => { crate::simd_ops::wu32(P2, (0x1du32) as u32); rdx = ((r15 as u32)) as u32 as u64; bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru64(P3)) as u64; rcx = ((rdx as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), r15); fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P2)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 22; continue; } bb = 23; continue; }
            23 => { r9 = (((rbp as u8)) as u32) as u32 as u64; r8 = (((r14 as u8)) as u32) as u32 as u64; rdx = rbx; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 24; continue; }
            24 => { r9 = (crate::simd_ops::ru32(P2)) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 26; continue; } bb = 25; continue; }
            25 => { r8 = (crate::simd_ops::ru32(P5)) as u32 as u64; xmm1 = 0u128; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((r9) as i64) as f32).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((((r8) as i64) as f32).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P9)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 27; continue; } bb = 26; continue; }
            26 => { rdx = ((r9 as u32)) as u32 as u64; rcx = P3; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru32(P2)) as u32 as u64; bb = 27; continue; }
            27 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 29; continue; } bb = 28; continue; }
            28 => { xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64) as u128; rdx = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_shr(4u32)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rcx = ((rcx).wrapping_shr(4u32)) as u64; rax = ((((rax as u32)) ^ ((rcx as u32)))) as u32 as u64; let d = ((r9 as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(P3)) as u64; r8 = ((rdx as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x78u64), rdx); rax = (crate::simd_ops::ru64(P3)) as u64; crate::simd_ops::wu64(rax.wrapping_add(r8.wrapping_mul(8)), rbx); fl_l = (crate::simd_ops::lk_add_u32(P5, (1) as u32)) as u32 as u64; bb = 29; continue; }
            29 => { rcx = P0; r11 = rsp.wrapping_add(0x60); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P11), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415fde80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930fa0; const P1: u64 = 0x1415c4950; const P2: u64 = 0x1415c53e0; const P3: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(0x40u64))) as u64; r14 = rdx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x48u64))) as u32 as u64; rsi = rcx; rbx = (crate::simd_ops::ru64(rdx)) as u64; r8 = ((rdi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rdi; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(r14)) as u64; rcx = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru32(rbx)) as u32 as u64; r9 = ((rdx as u32)) as u32 as u64; r8 = ((crate::simd_ops::ru8(rbx.wrapping_add(0x4u64))) as u32) as u32 as u64; rdx = ((crate::simd_ops::ru8(rdi.wrapping_add(0x4u64))) as u32) as u32 as u64; r8 = ((r8).wrapping_shl(32u32)) as u64; r8 = (((r8) | (rcx))) as u64; rdx = ((rdx).wrapping_shl(32u32)) as u64; rcx = (crate::simd_ops::ru32(rdi)) as u32 as u64; rdx = (((rdx) | (rcx))) as u64; fl_l = r8; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; r8 = rsi; rcx = (crate::simd_ops::ru64(P0)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 27; continue; } bb = 5; continue; }
            5 => { r9 = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rax = (crate::simd_ops::ru64(r14)) as u64; rdx = ((r9 as u32)) as u32 as u64; rcx = ((rax.wrapping_add(r9.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 6; continue; }
            6 => { rcx = ((r9 as u32)) as u32 as u64; r8 = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x48u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x40u64))) as u64; r9 = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; bb = 8; continue; }
            8 => { fl_l = rdi; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; rdx = ((crate::simd_ops::ru8(rdi.wrapping_add(0x4u64))) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru8(rbx.wrapping_add(0x4u64))) as u32) as u32 as u64; rdx = ((rdx).wrapping_shl(32u32)) as u64; rdx = (((rdx) | (rax))) as u64; rcx = ((rcx).wrapping_shl(32u32)) as u64; rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rcx = (((rcx) | (rax))) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; rdi = ((rdi).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 11; continue; }
            11 => { r9 = (crate::simd_ops::ru32(rsi.wrapping_add(0x48u64))) as u32 as u64; r8 = (crate::simd_ops::ru64(rsi.wrapping_add(0x40u64))) as u64; rax = ((r8.wrapping_add(r9.wrapping_mul(8))) as u64) as u64; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 12; continue; }
            12 => { rcx = ((r9 as u32)) as u32 as u64; bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; r9 = ((rcx as u32)) as u32 as u64; rdx = ((crate::simd_ops::ru8(rdi.wrapping_add(0x4u64))) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru8(rbx.wrapping_add(0x4u64))) as u32) as u32 as u64; rdx = ((rdx).wrapping_shl(32u32)) as u64; rdx = (((rdx) | (rax))) as u64; rcx = ((rcx).wrapping_shl(32u32)) as u64; rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rcx = (((rcx) | (rax))) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; r8 = rsi; rcx = (crate::simd_ops::ru64(P0)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x48u64))) as u32 as u64; rdi = ((rdi).wrapping_add(0x8u64)) as u64; r8 = (crate::simd_ops::ru64(rsi.wrapping_add(0x40u64))) as u64; r9 = ((rcx as u32)) as u32 as u64; rax = ((r8.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 15; continue; }
            15 => { rax = ((r9 as u32)) as u32 as u64; rcx = ((r8.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(r14)) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 17; continue; }
            17 => { bb = 18; continue; }
            18 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; r8 = rsi; rcx = (crate::simd_ops::ru64(P0)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 27; continue; } bb = 19; continue; }
            19 => { rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rax = (crate::simd_ops::ru64(r14)) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 18; continue; } bb = 20; continue; }
            20 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x40u64))) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x48u64))) as u32 as u64; rcx = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 23; continue; } bb = 21; continue; }
            21 => { bb = 22; continue; }
            22 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; r8 = rsi; rcx = (crate::simd_ops::ru64(P0)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x40u64))) as u64; rdi = ((rdi).wrapping_add(0x8u64)) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x48u64))) as u32 as u64; rcx = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 22; continue; } bb = 23; continue; }
            23 => { rdi = 0; fl_l = 0; fl_r = 0; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 24; continue; }
            24 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsi.wrapping_add(0x48u64), ((rdi as u32)) as u32); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi.wrapping_add(0x40u64), rdi); crate::simd_ops::wu32(rsi.wrapping_add(0x4cu64), ((rdi as u32)) as u32); bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x40u64), rax); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x48u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r14.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x4cu64), ((rax as u32)) as u32); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu64(r14, rdi); crate::simd_ops::wu64(r14.wrapping_add(0x8u64), rdi); bb = 26; continue; }
            26 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            27 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 26; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415fe0c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14268e450; const P1: u64 = 0x141524330; const P2: u64 = 0x1415242e0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x78u64)) as u64; rax = 0; fl_l = 0; fl_r = 0; r8 = P0; rdi = rcx; rcx = ((rax as u32)) as u32 as u64; bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; r8 = ((r8).wrapping_add(0x4u64)) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1b; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rax = 0x2; rsp = ((rsp).wrapping_add(0x78u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            4 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 3; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rbp); rdx = ((rbx.wrapping_mul(8)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rsi); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), r12); rsi = rbx; r12 = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r13); r13 = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r15); r15 = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(rdi, rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rdi)) as u64; rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { r8 = 0; fl_l = 0; fl_r = 0; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((r8 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rdi, r13); rax = 0x2; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((r15 as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((r12 as u32)) as u32); bb = 26; continue; }
            9 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { fl_l = rdx; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { rdx = rdi; rcx = rdi.wrapping_add(0xc); rax = ((rbx as u32)) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdx, rbp); rbp = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu32(rcx, ((rbx as u32)) as u32); bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 16; continue; } bb = 14; continue; }
            14 => { rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rdx, r8); rbx = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; bb = 16; continue; }
            16 => { fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 17; continue; }
            17 => { fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 23; continue; } bb = 18; continue; }
            18 => { r10 = r13; r11 = r15; bb = 19; continue; }
            19 => { r9 = (crate::simd_ops::ru64(r10)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 20; continue; }
            20 => { bb = 21; continue; }
            21 => { xmm0 = ((crate::simd_ops::ru64(r9.wrapping_add(0x38u64))) as u64) as u128; rdx = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(r9.wrapping_add(0x8u64))) as u64; r8 = (crate::simd_ops::ru64(r9.wrapping_add(0x78u64))) as u64; rax = ((rax).wrapping_shr(4u32)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rcx = ((rcx).wrapping_shr(4u32)) as u64; rax = ((((rax as u32)) ^ ((rcx as u32)))) as u32 as u64; let d = ((rbp as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = ((rdx.wrapping_mul(8)) as u64) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(rax))) as u64; crate::simd_ops::wu64(r9.wrapping_add(0x78u64), rcx); rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(rax), r9); r9 = r8; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 21; continue; } bb = 22; continue; }
            22 => { r10 = ((r10).wrapping_add(0x8u64)) as u64; fl_l = r11; fl_r = (0x1u64) as u64; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 19; continue; } bb = 23; continue; }
            23 => { fl_l = r13; fl_r = r13; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 24; continue; }
            24 => { rdx = r13; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 25; continue; }
            25 => { rax = 0x1; bb = 26; continue; }
            26 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x78u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415fe280(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931ce8; const P1: u64 = 0x143931cb0; const P2: u64 = 0x1426452b0; const P3: u64 = 0x143931ce0; const P4: u64 = 0x143931cd8; const P5: u64 = 0x143932030; const P6: u64 = 0x143930fa0; const P7: u64 = 0x1415c53e0; const P8: u64 = 0x1415af340; const P9: u64 = 0x1415242e0; const P10: u64 = 0x143931ce4; const P11: u64 = 0x1426452a8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = 0x0; rbp = rcx; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 50; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rdi); rcx = P1; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r15); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(P3)) as u32 as u64; r15 = 0; fl_l = 0; fl_r = 0; r14 = (crate::simd_ops::ru64(P4)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 46; continue; } bb = 2; continue; }
            2 => { rdi = (crate::simd_ops::ru64(r14)) as u64; r10 = P4; r8 = ((r15 as u32)) as u32 as u64; r11 = ((r15 as u32)) as u32 as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdi = (crate::simd_ops::ru64(r14.wrapping_add(r8.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 5; continue; }
            5 => { bb = 7; continue; }
            6 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 46; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rsi); bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x38u64))) as u64; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 51; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x78u64))) as u64; r9 = ((r8 as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r10); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r8 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r11); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 10; continue; }
            10 => { rsi = (crate::simd_ops::ru32(r10.wrapping_add(0x8u64))) as u32 as u64; bb = 11; continue; }
            11 => { r9 = (((r9 as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(r10)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r15); rdx = (crate::simd_ops::ru64(rax.wrapping_add(r9.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdx); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r9 as u32)) as u32); bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x78u64))) as u64; fl_l = r11; fl_r = r11; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu64(r11.wrapping_add(0x78u64), rax); bb = 17; continue; }
            16 => { rcx = ((r8 as u32)) as u32 as u64; crate::simd_ops::wu64(r14.wrapping_add(rcx.wrapping_mul(8)), rax); bb = 17; continue; }
            17 => { crate::simd_ops::lk_sub_u32(P0, (1) as u32); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x50u64))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x58u64))) as u32 as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; rcx = ((rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm1) as u128); fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 18; continue; }
            18 => { bb = 19; continue; }
            19 => { rcx = (crate::simd_ops::ru64(P5)) as u64; r8 = ((crate::simd_ops::ru8(rdi.wrapping_add(0x70u64))) as u32) as u32 as u64; rdx = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x58u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x4u64)) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x50u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 19; continue; } bb = 20; continue; }
            20 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x68u64))) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 23; continue; } bb = 21; continue; }
            21 => { bb = 22; continue; }
            22 => { rcx = (crate::simd_ops::ru64(P5)) as u64; r8 = ((crate::simd_ops::ru8(rdi.wrapping_add(0x71u64))) as u32) as u32 as u64; rdx = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x68u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x4u64)) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 22; continue; } bb = 23; continue; }
            23 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x58u64), ((r15 as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0x68u64), ((r15 as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x48u64))) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 25; continue; } bb = 24; continue; }
            24 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; r8 = rdi; rcx = (crate::simd_ops::ru64(P6)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x48u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 24; continue; } bb = 25; continue; }
            25 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 37; continue; } bb = 26; continue; }
            26 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x60u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 29; continue; } bb = 27; continue; }
            27 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x64u64))) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 29; continue; } bb = 28; continue; }
            28 => { rdx = (rdx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 30; continue; }
            29 => { rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 30; continue; }
            30 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x60u64), ((rcx as u32)) as u32); if (((fl_l as u32)) == ((fl_r as u32))) { bb = 34; continue; } bb = 31; continue; }
            31 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 35; continue; } bb = 32; continue; }
            32 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 35; continue; } bb = 33; continue; }
            33 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x64u64))) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 35; continue; } bb = 34; continue; }
            34 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 35; continue; }
            35 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x60u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 37; continue; } bb = 36; continue; }
            36 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbx; rcx = 0x4; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 37; continue; }
            37 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x50u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 39; continue; } bb = 38; continue; }
            38 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(0x58u64), ((r15 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi.wrapping_add(0x50u64), r15); crate::simd_ops::wu32(rdi.wrapping_add(0x5cu64), ((r15 as u32)) as u32); bb = 39; continue; }
            39 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 41; continue; } bb = 40; continue; }
            40 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(0x68u64), ((r15 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi.wrapping_add(0x60u64), r15); crate::simd_ops::wu32(rdi.wrapping_add(0x6cu64), ((r15 as u32)) as u32); bb = 41; continue; }
            41 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 43; continue; } bb = 42; continue; }
            42 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(0x48u64), ((r15 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi.wrapping_add(0x40u64), r15); crate::simd_ops::wu32(rdi.wrapping_add(0x4cu64), ((r15 as u32)) as u32); bb = 43; continue; }
            43 => { rdx = rdi; rcx = 0x10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r11 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r14 = (crate::simd_ops::ru64(P4)) as u64; bb = 44; continue; }
            44 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 45; continue; }
            45 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; bb = 46; continue; }
            46 => { fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 49; continue; } bb = 47; continue; }
            47 => { fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 49; continue; } bb = 48; continue; }
            48 => { rdx = r14; crate::simd_ops::wu32(P3, ((r15 as u32)) as u32); rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(P4, r15); crate::simd_ops::wu32(P10, ((r15 as u32)) as u32); bb = 49; continue; }
            49 => { rcx = P1; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P11), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; bb = 50; continue; }
            50 => { rsp = ((rsp).wrapping_add(0x70u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            51 => { r11 = rdi; rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x78u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 52; continue; }
            52 => { rcx = (crate::simd_ops::ru32(r10.wrapping_add(0x8u64))) as u32 as u64; bb = 53; continue; }
            53 => { r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 44; continue; } bb = 54; continue; }
            54 => { rdi = (crate::simd_ops::ru64(r10)) as u64; r11 = r15; rdi = (crate::simd_ops::ru64(rdi.wrapping_add(r8.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 53; continue; } bb = 55; continue; }
            55 => { bb = 8; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415fe5d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931cb0; const P1: u64 = 0x1426452b0; const P2: u64 = 0x143931ce0; const P3: u64 = 0x143931cd8; const P4: u64 = 0x8; const P5: u64 = 0x38; const P6: u64 = 0x78; const P7: u64 = 0x143931ce8; const P8: u64 = 0x1415fd2c0; const P9: u64 = 0x1415242e0; const P10: u64 = 0x143931ce4; const P11: u64 = 0x1426452a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rbx = rdx; rcx = P0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(P2)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { rdx = 0; fl_l = 0; fl_r = 0; rax = rdi; rax = ((rax).wrapping_shr(4u32)) as u64; r8 = rbx; r8 = ((r8).wrapping_shr(4u32)) as u64; rax = ((((rax as u32)) ^ ((r8 as u32)))) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(P3)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; r8 = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; rax = 0; fl_l = 0; fl_r = 0; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P5))) as u64; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P6))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 5; continue; }
            5 => { bb = 11; continue; }
            6 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P6))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rax.wrapping_add(P6), rdx); bb = 10; continue; }
            9 => { crate::simd_ops::wu64(r8, rdx); bb = 10; continue; }
            10 => { crate::simd_ops::lk_sub_u32(P7, (1) as u32); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru32(P7)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 12; continue; }
            12 => { rdx = (crate::simd_ops::ru64(P3)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(P2, (0x0u32) as u32); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P3, (0x0u64) as u64); crate::simd_ops::wu32(P10, (0x0u32) as u32); bb = 14; continue; }
            14 => { rcx = P0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P11), rax), rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415fe6c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931cf0; const P1: u64 = 0x141524330; const P2: u64 = 0x14164abb0; const P3: u64 = 0x141612370; const P4: u64 = 0x1415864b0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; fl_l = (crate::simd_ops::ru64(P0)) as u64; fl_r = (0x0u64) as u64; rdi = r9; rsi = ((rdx as u32)) as u32 as u64; rbp = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 1; continue; }
            1 => { fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 2; continue; }
            2 => { rbx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x90u64))) as u32) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 4; continue; }
            4 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 13; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r14); rcx = 0; fl_l = 0; fl_r = 0; r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x5cu64), ((r8 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), ((r14 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r14); rdx = ((r14.wrapping_add(0x78u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((r14 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), (0x4u32) as u32); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), ((rbx as u8)) as u8); rcx = rsp.wrapping_add(0x48); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rsi as u32)) as u32); r9 = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); r8 = rdi; rcx = rax; rdx = rbp; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rax)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rbx = r14; bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rax = rbx; bb = 14; continue; }
            13 => { rax = 0; fl_l = 0; fl_r = 0; bb = 14; continue; }
            14 => { r11 = rsp.wrapping_add(0x60); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415fe7c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931cf0; const P1: u64 = 0x40; const P2: u64 = 0x141524330; const P3: u64 = 0x1416123b0; const P4: u64 = 0x14268e4d0; const P5: u64 = 0x38; const P6: u64 = 0x1415fea00; const P7: u64 = 0x10; const P8: u64 = 0x1415862a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = (crate::simd_ops::ru64(P0)) as u64;
    fl_l = rax; fl_r = rax;
    if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rsp = ((rsp).wrapping_add(0x38u64)) as u64; return rax; }
    rdx = ((rax.wrapping_add(P1)) as u32) as u32 as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rbx = rax;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = rax; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = P4; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(P0, rbx); crate::simd_ops::wu64(rbx.wrapping_add(P5), (0x0u64) as u64); }
    rax = (crate::simd_ops::ru64(P0)) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x322u32) as u32); r9 = (r9 & !0xffu64) | (((0x8u8) as u8 as u64)); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x0u32) as u32); rcx = P6; rdx = ((r8.wrapping_add(P7)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(P0)) as u64; }
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415fe850(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415fea00; const P1: u64 = 0x14158bd20; const P2: u64 = 0x10; const P3: u64 = 0x8; const P4: u64 = 0x141612750; const P5: u64 = 0x1416129e0; const P6: u64 = 0x141612930; const P7: u64 = 0x38; const P8: u64 = 0x141612370; const P9: u64 = 0x14268e4d0; const P10: u64 = 0x143931cf0; const P11: u64 = 0x1416123e0; const P12: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rdx = 0x10; rcx = P0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); bb = 2; continue; }
            2 => { rsi = rbx; rcx = rdi; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; rdx = rsi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = rsi; rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = rsi; rcx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P7))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 3; continue; }
            3 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 4; continue; }
            4 => { rax = P9; rcx = rdi; crate::simd_ops::wu64(rdi, rax); crate::simd_ops::wu64(P10, (0x0u64) as u64); rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rdx = rdi; rcx = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P12, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415fe900(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14164b030; const P1: u64 = 0x14164aef0; const P2: u64 = 0x141612750; const P3: u64 = 0x1416129e0; const P4: u64 = 0x141612930; const P5: u64 = 0x141612370;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; r14 = r9; rbp = ((r8 as u32)) as u32 as u64; r15 = ((rdx as u32)) as u32 as u64; r13 = rcx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); rsi = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); bb = 2; continue; }
            2 => { rdi = rbx; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x38u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 17; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x58u64))) as u64; bb = 7; continue; }
            6 => { rcx = (0xffffffffffffffffu64) as u64; bb = 7; continue; }
            7 => { fl_l = rcx; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { fl_l = r14; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x2cu64))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 17; continue; } bb = 11; continue; }
            11 => { rcx = ((r15 as u32)) as u32 as u64; fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 16; continue; } bb = 12; continue; }
            12 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 15; continue; } bb = 13; continue; }
            13 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 17; continue; } bb = 14; continue; }
            14 => { rcx = rdi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            15 => { rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            16 => { rdx = rdi; rcx = r13; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = rdi; rcx = r13; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = rdi; rcx = r13; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            17 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 18; continue; }
            18 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 19; continue; }
            19 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415fea00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931cf0; const P1: u64 = 0x143930f08; const P2: u64 = 0x1438152b4; const P3: u64 = 0x14164adf0; const P4: u64 = 0x14164add0; const P5: u64 = 0x141612750; const P6: u64 = 0x1416129e0; const P7: u64 = 0x141612930; const P8: u64 = 0x141612370; const P9: u64 = 0x141613100;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x38u64)) as u64; r8 = (crate::simd_ops::ru64(P0)) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(P1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rsi); rdx = (crate::simd_ops::ru32(rax.wrapping_add(0x88u64))) as u32 as u64; rax = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64; rdx = ((rdx).wrapping_add(1)) as u64; rdx = (((rdx) as i64).wrapping_mul((rax) as i64) as u64) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x38u64), rdx); rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (crate::simd_ops::ru64(P0)) as u64; rbp = (((rax as u16)) as u32) as u32 as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); xmm0 = 0u128; rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); bb = 4; continue; }
            4 => { rdi = rbx; rdx = rsp.wrapping_add(0x20); rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = rdi; r8 = ((rbp as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = rdi; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rdi; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rdi; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            7 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 8; continue; }
            8 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 9; continue; }
            9 => { r8 = ((rbp as u32)) as u32 as u64; rdx = rsp.wrapping_add(0x20); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 10; continue; }
            10 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 11; continue; }
            11 => { rsp = ((rsp).wrapping_add(0x38u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415feb10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931cf0; const P1: u64 = 0x143930f78; const P2: u64 = 0x1415f3790; const P3: u64 = 0x1415fe6c0; const P4: u64 = 0x14397d1e0; const P5: u64 = 0x143931cfc; const P6: u64 = 0x143931cf8; const P7: u64 = 0x14164acd0; const P8: u64 = 0x141834a78; const P9: u64 = 0x1438152ac; const P10: u64 = 0x141834a0c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; fl_l = (crate::simd_ops::ru64(P0)) as u64; fl_r = (0x0u64) as u64; rdi = ((r9 as u32)) as u32 as u64; rsi = ((rdx as u32)) as u32 as u64; r15 = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 21; continue; } bb = 1; continue; }
            1 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 21; continue; } bb = 2; continue; }
            2 => { fl_l = r8; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 21; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rdx = r8; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(P0)) as u64; fl_r = (0x0u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbx); if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x38u64))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x2cu64))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 11; continue; }
            11 => { r9 = rax; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0x0u8) as u8); r8 = ((rdi as u32)) as u32 as u64; rdx = ((rsi as u32)) as u32 as u64; rcx = r15; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 12; continue; }
            12 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); r14 = (crate::simd_ops::ru32(rsp.wrapping_add(0x98u64))) as u32 as u64; rsi = (((rax as u16)) as u32) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 19; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru32(P4)) as u32 as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), r12); r12 = ((crate::simd_ops::ru8(rsp.wrapping_add(0xa0u64))) as u32) as u32 as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r13); r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdx); r8 = 0x74; bb = 14; continue; }
            14 => { rax = (((rsi as u16)) as u32) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rbp = ((rcx.wrapping_mul(4)) as u64) as u64; rbp = ((rbp).wrapping_add(r13)) as u64; rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0xcu64))) as u64; fl_l = ((r12 as u8)) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 17; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = ((r8 as u32)) as u32 as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(rax))) as u32 as u64; fl_l = (crate::simd_ops::ru32(P5)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 22; continue; } bb = 16; continue; }
            16 => { rdi = ((rdi).wrapping_sub(crate::simd_ops::ru64(r15.wrapping_add(0x38u64)))) as u64; rax = (crate::simd_ops::ru32(P6)) as u32 as u64; fl_l = rdi; fl_r = rax; rax = 0x0; rdi = (if (((fl_l as u64)) > ((fl_r as u64))) { rax } else { rdi }) as u64; bb = 17; continue; }
            17 => { r8 = ((rdi as u32)) as u32 as u64; rdx = rbp; rcx = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (rsi & !0xffffu64) | (((((rsi as u16)).wrapping_add(1)) as u16 as u64)); rax = (((rsi as u16)) as u32) as u32 as u64; r8 = 0x74; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 14; continue; } bb = 18; continue; }
            18 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; bb = 19; continue; }
            19 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 20; continue; }
            20 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; bb = 21; continue; }
            21 => { rsp = ((rsp).wrapping_add(0x50u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            22 => { rcx = P5; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(P5)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 16; continue; } bb = 23; continue; }
            23 => { rax = (((crate::simd_ops::ru32(P9)) as i32).wrapping_mul((0xe10u32) as i32) as u32) as u32 as u64; rcx = P5; crate::simd_ops::wu32(P6, ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415fece0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931cf0; const P1: u64 = 0x10; const P2: u64 = 0x8; const P3: u64 = 0x38; const P4: u64 = 0x58; const P5: u64 = 0x2c; const P6: u64 = 0x141612750; const P7: u64 = 0x1416129e0; const P8: u64 = 0x141612930; const P9: u64 = 0x141612370;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru64(P0)) as u64; fl_r = (0x0u64) as u64; rsi = ((r9 as u32)) as u32 as u64; r14 = r8; rbp = ((rdx as u32)) as u32 as u64; r15 = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); bb = 3; continue; }
            3 => { rdi = rbx; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 13; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; bb = 8; continue; }
            7 => { rcx = (0xffffffffffffffffu64) as u64; bb = 8; continue; }
            8 => { fl_l = rcx; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = r14; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(P5))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rdx = rdi; rcx = r15; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdx = rdi; rcx = r15; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rdx = rdi; rcx = r15; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 14; continue; }
            14 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 16; continue; }
            16 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415fedf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140000000; const P1: u64 = 0x143930f48; const P2: u64 = 0x14268e5b0; const P3: u64 = 0x14268e4e0; const P4: u64 = 0x14183acbc;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rax = (((r8 as u8)) as u32) as u32 as u64; rsi = (rcx) as u64; crate::simd_ops::wu64(r9, (rbp) as u64); r14 = (r9) as u64; rcx = ((crate::simd_ops::ru8(rax.wrapping_add(rdx).wrapping_add(0x1f8u64))) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xfeu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 12; continue; } bb = 1; continue; }
            1 => { rax = ((r8.wrapping_sub(0x7u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0xbu32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 8; continue; } bb = 2; continue; }
            2 => { rdx = ((P0) as u64) as u64; rax = (rax as u32 as i32 as i64) as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x15fefa0u64))) as u32 as u64; fl_l = ((rcx).wrapping_add(rdx)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; j_tgt = (rcx) as u64; bb = 0xffff_fffe; continue; }
            3 => { rcx = (0x1u32) as u32 as u64; rax = ((rcx.wrapping_add(0x17u64)) as u32) as u32 as u64; bb = 9; continue; }
            4 => { rcx = (0x2u32) as u32 as u64; rax = ((rcx.wrapping_add(0x2eu64)) as u32) as u32 as u64; bb = 9; continue; }
            5 => { rcx = (0x3u32) as u32 as u64; rax = ((rcx.wrapping_add(0x45u64)) as u32) as u32 as u64; bb = 9; continue; }
            6 => { rcx = (0x4u32) as u32 as u64; rax = ((rcx.wrapping_add(0x5cu64)) as u32) as u32 as u64; bb = 9; continue; }
            7 => { rcx = (0x5u32) as u32 as u64; rax = ((rcx.wrapping_add(0x73u64)) as u32) as u32 as u64; bb = 9; continue; }
            8 => { rax = (rbp) as u64; rcx = (rbp) as u64; bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(P1)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rdx))) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 22; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(rdx).wrapping_add(0x8u64))) as u64 as u64; fl_r = (rbp) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 22; continue; } bb = 11; continue; }
            11 => { rdi = ((rdx.wrapping_add(0x8u64)) as u64) as u64; fl_l = ((rdi).wrapping_add(rax)) as u64 as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 14; continue; }
            12 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 22; continue; } bb = 13; continue; }
            13 => { rax = ((rcx.wrapping_add(0x2u64)) as u64) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(rax.wrapping_mul(8)))) as u64 as u64; fl_r = (rbp) as u64 as u64; rdi = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 22; continue; } bb = 14; continue; }
            14 => { fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rsi)) as u64; r8 = ((P2) as u64) as u64; r9 = (0xdau32) as u32 as u64; rdx = (0x18u32) as u32 as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu64(rax, (rbp) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rbp) as u64); crate::simd_ops::wu32(rax.wrapping_add(0x10u64), (0x1u32) as u32); rdx = (crate::simd_ops::ru64(rax)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 18; continue; }
            18 => { crate::simd_ops::wu64(rbx, (rbp) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), (rbp) as u64); crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), (0x1u32) as u32); fl_l = (crate::simd_ops::ru64(rdi)) as u64 as u64; fl_r = (rbp) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 21; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; r8 = ((P3) as u64) as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); r9 = (0xc6u32) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), (0x1u32) as u32); rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = (((rcx as u32)).wrapping_shl(2u32)) as u32 as u64; rbp = ((rcx as u32)) as u32 as u64; rdx = ((rcx as u32)) as u32 as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rbx, (rax) as u64); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 20; continue; }
            20 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; r8 = ((rbp as u32)) as u32 as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 21; continue; }
            21 => { crate::simd_ops::wu64(r14, (rbx) as u64); bb = 22; continue; }
            22 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 23; continue; }
            23 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            24 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = (rbx) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 25; continue; }
            25 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 23; continue; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x1415fedf0u64) { 0x36 => { bb = 1; continue; } 0x3f => { bb = 2; continue; } 0x54 => { bb = 3; continue; } 0x5e => { bb = 4; continue; } 0x68 => { bb = 5; continue; } 0x72 => { bb = 6; continue; } 0x7c => { bb = 7; continue; } 0x86 => { bb = 8; continue; } 0x8c => { bb = 9; continue; } 0x9d => { bb = 10; continue; } 0xa8 => { bb = 11; continue; } 0xb1 => { bb = 12; continue; } 0xba => { bb = 13; continue; } 0xd0 => { bb = 14; continue; } 0xd9 => { bb = 15; continue; } 0x100 => { bb = 16; continue; } 0x116 => { bb = 17; continue; } 0x11f => { bb = 18; continue; } 0x132 => { bb = 19; continue; } 0x170 => { bb = 20; continue; } 0x17e => { bb = 21; continue; } 0x181 => { bb = 22; continue; } 0x183 => { bb = 23; continue; } 0x19e => { bb = 24; continue; } 0x1aa => { bb = 25; continue; } _ => { let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(j_tgt, rcx, rdx, r8, r9, s5, s6, s7, s8); } } }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415fefd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1439310e8;
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    rax = (crate::simd_ops::ru64(P0)) as u64;
    rcx = (0x5851f42d4c957f2du64) as u64;
    rax = (((rax) as i64).wrapping_mul((rcx) as i64) as u64) as u64;
    fl_l = ((rax).wrapping_add(1)) as u64;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu64(P0, rax);
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ff000(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(r8)) as u64;
    rsi = r8;
    rdi = rdx;
    fl_l = rbx; fl_r = rbx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rdx = (crate::simd_ops::ru64(rbx)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp);
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
    rbp = 0;
    fl_l = 0; fl_r = 0;
    rdx = rbx;
    crate::simd_ops::wu64(rbx, rbp);
    rcx = rdi;
    crate::simd_ops::wu64(rbx.wrapping_add(P1), rbp);
    crate::simd_ops::wu32(rbx.wrapping_add(P0), (0x1u32) as u32);
    rax = (crate::simd_ops::ru64(rdi)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rsi, rbp);
    rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ff070(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x141529ed0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rdx)) as u64;
    fl_l = (crate::simd_ops::ru64(rbx)) as u64; fl_r = (0x0u64) as u64;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; r8 = rsp.wrapping_add(0x38); xmm1 = (xmm2) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rax as u32)) as u32); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x38u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P0), ((rax as u32)) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    xmm0 = 0u128;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ff0b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415fedf0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(0x60u64))) as u64; rsi = ((r9 as u32)) as u32 as u64; rcx = (crate::simd_ops::ru32(r8.wrapping_add(0x68u64))) as u32 as u64; r12 = rdx; r10 = (r10 & !0xffu64) | (((0x1u8) as u8 as u64)); r8 = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = ((rax.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = 0x3850009; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 2; continue; }
            2 => { rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rdi = 0; fl_l = 0; fl_r = 0; rbx = 0x1; rcx = ((rdi as u32)) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 8; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; bb = 5; continue; }
            5 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = ((rcx as u32)) as u32 as u64; fl_l = (((rcx as u32)).wrapping_add(1)) as u32 as u64; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu64(rdx.wrapping_add(rax.wrapping_mul(8)), rdi); bb = 7; continue; }
            7 => { rbx = (((rbx as u32)).wrapping_add((rbx as u32))) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 8; continue; }
            8 => { rax = (((r10 as u8)) as u32) as u32 as u64; bb = 21; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rdx = 0x18c; fl_l = rcx; fl_r = rcx; rax = rcx.wrapping_add(0x124); rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rdx } else { rax }) as u64; fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = (0x40u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } bb = 10; continue; }
            10 => { fl_l = rcx; fl_r = rcx; rax = rcx.wrapping_add(0x128); rdx = 0x190; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rdx } else { rax }) as u64; r14 = (crate::simd_ops::ru64(rax)) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 11; continue; }
            11 => { rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); rbp = ((rdi as u32)) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 22; continue; } bb = 12; continue; }
            12 => { rbx = 0x1; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r15); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = ((rbx as u32)) as u32 as u64; bb = 13; continue; }
            13 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 18; continue; } bb = 14; continue; }
            14 => { fl_l = ((r10 as u8)) as u8 as u64; fl_r = ((r10 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 17; continue; } bb = 15; continue; }
            15 => { rax = ((rbp as u32)) as u32 as u64; r8 = ((rdi as u32)) as u32 as u64; rdx = r14; rcx = r12; r9 = ((r15.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbp = (((rbp as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { r10 = (((rbx as u8)) as u32) as u32 as u64; bb = 18; continue; }
            17 => { r10 = (r10 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 18; continue; }
            18 => { rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; rax = ((rbx as u32)) as u32 as u64; rcx = ((rdi as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(((rcx as u32) & 31))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 13; continue; } bb = 19; continue; }
            19 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = (((r10 as u8)) as u32) as u32 as u64; bb = 20; continue; }
            20 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 21; continue; }
            21 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            22 => { rax = (((r10 as u8)) as u32) as u32 as u64; bb = 20; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ff1e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x60; const P1: u64 = 0x68; const P2: u64 = 0x8; const P3: u64 = 0x124; const P4: u64 = 0x128; const P5: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = 0x3850009; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rax = 0; fl_l = 0; fl_r = 0; return rax; }
            4 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; rdx = 0x18c; fl_l = rcx; fl_r = rcx; rax = ((rcx.wrapping_add(P3)) as u64) as u64; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rdx } else { rax }) as u64; fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = (0x40u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } bb = 5; continue; }
            5 => { fl_l = rcx; fl_r = rcx; rax = ((rcx.wrapping_add(P4)) as u64) as u64; rdx = 0x190; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rdx } else { rax }) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(P5))) as u32 as u64; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ff250(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xfffffffffffffffe;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64;
    rax = ((rcx.wrapping_add(P1)) as u32) as u32 as u64;
    rax = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rdx as u32) } else { (rax as u32) }) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ff490(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = ((r8.wrapping_add(P0)) as u32) as u32 as u64;
    r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    rax = ((rax.wrapping_add(r8.wrapping_mul(4))) as u64) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ff4a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1; const P1: u64 = 0x8; const P2: u64 = 0x90; const P3: u64 = 0xffffffffffffffff;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64;
    if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { return rax; }
    r8 = ((r8 as u32)) as u32 as u64;
    loop { rax = (crate::simd_ops::ru64(rdx)) as u64; r9 = ((r9.wrapping_add(P0)) as u64) as u64; rdx = ((rdx.wrapping_add(P1)) as u64) as u64; rcx = ((crate::simd_ops::ru8(rax.wrapping_add(P2))) as u32) as u32 as u64; crate::simd_ops::wu8(r9.wrapping_add(P3), ((rcx as u8)) as u8); fl_l = r8; fl_r = (0x1u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ff4d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415fce10;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rcx = (crate::simd_ops::ru64(rdx)) as u64;
    return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ff4e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1434268d0; const P2: u64 = 0x14163cd10;
    let (mut rax, mut r10, mut r11, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; r11 = r8; xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x70u64))) as u32) as u128; r9 = ((rax as u32)) as u32 as u64; xmm1 = ((crate::simd_ops::ru32(P1)) as u32) as u128; rcx = ((rax as u32)) as u32 as u64; r9 = (((r9 as u32)).wrapping_shr(12u32)) as u32 as u64; rax = ((((rax as u32)) & (0xf00u32))) as u32 as u64; r10 = ((r9 as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x64u64), ((xmm0 as u32)) as u32); r10 = ((((r10 as u32)) & (0xffff7u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((xmm3 as u32)) as u32); r9 = (((r9 as u32)).wrapping_sub((r10 as u32))) as u32 as u64; rbx = rdx; r9 = (((r9 as u32)).wrapping_shr(3u32)) as u32 as u64; rdx = r11; rcx = (((rcx as u32)).wrapping_sub((r9 as u32))) as u32 as u64; r10 = (((r10 as u32)).wrapping_shl(12u32)) as u32 as u64; r8 = (((rcx as u8)) as u32) as u32 as u64; r9 = rsp.wrapping_add(0x60); r8 = ((((r8 as u32)) | ((rax as u32)))) as u32 as u64; rcx = rbx; rax = ((crate::simd_ops::ru16(r11.wrapping_add(0x10u64))) as u32) as u32 as u64; r8 = ((((r8 as u32)) | ((r10 as u32)))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rax as u32)) as u32); xmm0 = (((rax as u32)) as u32) as u128; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; xmm0 = crate::simd_ops::vcvtdq2ps(xmm0); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((xmm1 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ff590(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14163ccd0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rsp.wrapping_add(0x30u64))) as u32) as u32 as u64;
    xmm2 = (xmm3) as u128;
    r9 = rdx;
    fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64;
    if !(((((fl_l as u16)) & ((fl_r as u16))) == 0)) { xmm3 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32) as u128; rdx = r8; xmm0 = (((rax as u32)) as u32) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm0 = crate::simd_ops::vcvtdq2ps(xmm0); rcx = r9; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); return crate::simd_ops::tail_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ff5d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1434268d0; const P2: u64 = 0x14163ccf0;
    let (mut rax, mut r10) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rax = ((crate::simd_ops::ru16(r8.wrapping_add(P0))) as u32) as u32 as u64;
    r10 = r8;
    xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x70u64))) as u32) as u128;
    r8 = rsp.wrapping_add(0x60);
    r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64;
    rcx = rdx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x64u64), ((xmm0 as u32)) as u32);
    rdx = r10;
    xmm0 = ((crate::simd_ops::ru32(P1)) as u32) as u128;
    xmm1 = (((rax as u32)) as u32) as u128;
    crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64;
    xmm1 = crate::simd_ops::vcvtdq2ps(xmm1);
    crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((xmm3 as u32)) as u32);
    xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(xmm1 as u32)).to_bits() as u128);
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((xmm0 as u32)) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ff640(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1439310e8;
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    rax = (crate::simd_ops::ru64(P0)) as u64;
    rcx = (0x5851f42d4c957f2du64) as u64;
    rax = (((rax) as i64).wrapping_mul((rcx) as i64) as u64) as u64;
    fl_l = ((rax).wrapping_add(1)) as u64;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu64(P0, rax);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ff660(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1; const P2: u64 = 0x90;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64;
    if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { return rax; }
    r8 = ((r8 as u32)) as u32 as u64;
    loop { rcx = (crate::simd_ops::ru64(rdx)) as u64; rdx = ((rdx.wrapping_add(P0)) as u64) as u64; rax = ((crate::simd_ops::ru8(r9)) as u32) as u32 as u64; r9 = ((r9.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu8(rcx.wrapping_add(P2), ((rax as u8)) as u8); fl_l = r8; fl_r = (0x1u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ff690(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14268e950; const P1: u64 = 0x8; const P2: u64 = 0x141646930; const P3: u64 = 0x14268e928; const P4: u64 = 0x48; const P5: u64 = 0x38; const P6: u64 = 0x142689070; const P7: u64 = 0x40; const P8: u64 = 0x50; const P9: u64 = 0x58; const P10: u64 = 0x60; const P11: u64 = 0x68;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = 0; fl_l = 0; fl_r = 0; rax = P0; crate::simd_ops::wu64(rcx.wrapping_add(P1), rdi); rbx = rcx; crate::simd_ops::wu64(rcx, rax); rcx = ((rcx).wrapping_add(0x10u64)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = P3; crate::simd_ops::wu64(rbx.wrapping_add(P4), rdi); crate::simd_ops::wu64(rbx.wrapping_add(P5), rax); rax = P6; crate::simd_ops::wu64(rbx.wrapping_add(P7), rax); rax = rbx; crate::simd_ops::wu32(rbx.wrapping_add(P8), ((rdi as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(P9), ((rdi as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_add(P10), (0xffu8) as u8); crate::simd_ops::wu8(rbx.wrapping_add(P11), (0xffu8) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x70u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x78u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x80u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x88u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x90u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x98u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0xa0u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0xa8u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0xb0u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0xb8u64), rdi); crate::simd_ops::wu32(rbx.wrapping_add(0xc0u64), (0xffffffffu32) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ff740(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415ff690; const P1: u64 = 0x14268eb18; const P2: u64 = 0x120; const P3: u64 = 0xe8; const P4: u64 = 0xe0; const P5: u64 = 0xf8; const P6: u64 = 0x118; const P7: u64 = 0x130; const P8: u64 = 0xec; const P9: u64 = 0x100; const P10: u64 = 0x104; const P11: u64 = 0x108;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = P1;
    crate::simd_ops::wu64(rbx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu8(rbx.wrapping_add(P2), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rbx.wrapping_add(P3), (0x0u8) as u8);
    crate::simd_ops::lk_and_u32(rbx.wrapping_add(P3), (0xffu32) as u32);
    crate::simd_ops::lk_and_u32(rbx.wrapping_add(P2), (0xffu32) as u32);
    crate::simd_ops::wu64(rbx.wrapping_add(P4), rax);
    crate::simd_ops::wu64(rbx.wrapping_add(P5), rax);
    crate::simd_ops::wu64(rbx.wrapping_add(P6), rax);
    crate::simd_ops::wu64(rbx.wrapping_add(P7), rax);
    rax = rbx;
    crate::simd_ops::wu64(rbx.wrapping_add(P8), (0x2bu64) as u64);
    crate::simd_ops::wu32(rbx.wrapping_add(P9), (0xffffffffu32) as u32);
    crate::simd_ops::wu32(rbx.wrapping_add(P10), (0x3f800000u32) as u32);
    crate::simd_ops::wu32(rbx.wrapping_add(P11), (0xffffffffu32) as u32);
    crate::simd_ops::wu32(rbx.wrapping_add(0x10cu64), (0x1u32) as u32);
    crate::simd_ops::wu64(rbx.wrapping_add(0x124u64), (0x2bu64) as u64);
    crate::simd_ops::wu32(rbx.wrapping_add(0x138u64), (0xffffffffu32) as u32);
    crate::simd_ops::wu32(rbx.wrapping_add(0x13cu64), (0x3f800000u32) as u32);
    crate::simd_ops::wu32(rbx.wrapping_add(0x140u64), (0xffffffffu32) as u32);
    crate::simd_ops::wu32(rbx.wrapping_add(0x144u64), (0x1u32) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ff810(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x60; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x14268e898; const P4: u64 = 0x14; const P5: u64 = 0x14268e908; const P6: u64 = 0x68; const P7: u64 = 0x14268e728; const P8: u64 = 0x78; const P9: u64 = 0x18; const P10: u64 = 0x20; const P11: u64 = 0x28; const P12: u64 = 0x1416419b0; const P13: u64 = 0x14268e7d8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64; rdi = 0; fl_l = 0; fl_r = 0; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xf9u8))) as u8 as u64)); crate::simd_ops::wu64(rcx.wrapping_add(P1), rdi); rax = (rax & !0xffu64) | ((((((rax as u8)) | (0x1u8))) as u8 as u64)); crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rdi as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(P0), ((rax as u8)) as u8); rbx = rcx; rax = P3; crate::simd_ops::wu32(rcx.wrapping_add(P4), (0xc5800000u32) as u32); crate::simd_ops::wu64(rcx, rax); rax = P5; crate::simd_ops::wu64(rcx.wrapping_add(P6), rax); rax = P7; crate::simd_ops::wu64(rcx.wrapping_add(P8), rax); crate::simd_ops::wu64(rcx.wrapping_add(P9), rdi); crate::simd_ops::wu64(rcx.wrapping_add(P10), rdi); crate::simd_ops::wu32(rcx.wrapping_add(P11), ((rdi as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x38u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x40u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x48u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x50u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x58u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x80u64), rdi); rcx = ((rcx).wrapping_add(0x88u64)) as u64; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0xd8u64), rdi); rax = P13; crate::simd_ops::wu64(rbx.wrapping_add(0xd0u64), rax); rax = rbx; crate::simd_ops::wu64(rbx.wrapping_add(0xe0u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0xb0u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0xb8u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0xc0u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0xc8u64), rdi); crate::simd_ops::lk_and_u8(rbx.wrapping_add(0x120u64), (0xf8u8) as u8); crate::simd_ops::wu8(rbx.wrapping_add(0x130u64), ((rdi as u8)) as u8); crate::simd_ops::lk_and_u32(rbx.wrapping_add(0x130u64), (0xffu32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x160u64), ((rdi as u8)) as u8); crate::simd_ops::lk_and_u32(rbx.wrapping_add(0x160u64), (0xffu32) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0xf8u64), rdi); crate::simd_ops::wu32(rbx.wrapping_add(0x100u64), ((rdi as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x108u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x110u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x128u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x134u64), (0x2bu64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x140u64), rdi); crate::simd_ops::wu32(rbx.wrapping_add(0x148u64), (0xffffffffu32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x14cu64), (0x3f800000u32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x150u64), (0xffffffffu32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x154u64), (0x1u32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x164u64), (0x7f7fffffu32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x168u64), (0x7f7fffffu32) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0xe8u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0xf0u64), rdi); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ffa20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14158fa60; const P1: u64 = 0x10; const P2: u64 = 0x141646a00; const P3: u64 = 0x14268b678; const P4: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rcx;
    rbx = ((rdx as u32)) as u32 as u64;
    rcx = ((rcx).wrapping_add(0x38u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rax = P3;
    crate::simd_ops::wu64(rdi, rax);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0xe8; rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ffa70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14158fa60; const P1: u64 = 0x10; const P2: u64 = 0x141646a00; const P3: u64 = 0x14268b678; const P4: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rcx;
    rbx = ((rdx as u32)) as u32 as u64;
    rcx = ((rcx).wrapping_add(0x38u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rax = P3;
    crate::simd_ops::wu64(rdi, rax);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0xd0; rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ffac0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14158fa60; const P1: u64 = 0x10; const P2: u64 = 0x141646a00; const P3: u64 = 0x14268b678; const P4: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rcx;
    rbx = ((rdx as u32)) as u32 as u64;
    rcx = ((rcx).wrapping_add(0x38u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rax = P3;
    crate::simd_ops::wu64(rdi, rax);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x168; rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ffb60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14268e898; const P1: u64 = 0x14268e908; const P2: u64 = 0x68; const P3: u64 = 0x141643b60; const P4: u64 = 0x88; const P5: u64 = 0x1401de410; const P6: u64 = 0x14268b678; const P7: u64 = 0x78; const P8: u64 = 0x142689028; const P9: u64 = 0x1416440f0; const P10: u64 = 0x141834d30;
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
    rax = P1;
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    rcx = ((rcx).wrapping_add(0xd0u64)) as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P4)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rax = P6;
    rcx = rdi;
    crate::simd_ops::wu64(rdi.wrapping_add(P7), rax);
    rax = P8;
    crate::simd_ops::wu64(rdi.wrapping_add(P2), rax);
    rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x170; rcx = rdi; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ffc90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524330; const P1: u64 = 0x1438152ac; const P2: u64 = 0x14268e780; const P3: u64 = 0x141643100; const P4: u64 = 0x14164b6a0; const P5: u64 = 0x1416031f0; const P6: u64 = 0x141594550; const P7: u64 = 0x14158e830; const P8: u64 = 0x14158f340; const P9: u64 = 0x143815b50; const P10: u64 = 0x14183acc2; const P11: u64 = 0x1415ff690; const P12: u64 = 0x14268ea80; const P13: u64 = 0x14268e9e8; const P14: u64 = 0x1415ff740; const P15: u64 = 0x1415242e0; const P16: u64 = 0x141642740; const P17: u64 = 0x141644040; const P18: u64 = 0x14164b6d0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 31>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x2f); rsp = ((rsp).wrapping_sub(0xc8u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0xe8u64))) as u64; r15 = 0; fl_l = 0; fl_r = 0; r14 = rcx; rdx = 0xa0; rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x110u64), ((r15 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x114u64), (0x5u32) as u32); rcx = ((r15.wrapping_add(0x5u64)) as u32) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x67u64), rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(r14.wrapping_add(0x108u64), rax); rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); rcx = (crate::simd_ops::ru64(r14.wrapping_add(0xe8u64))) as u64; xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x2a0u64))) as u64) as u128; rsi = (crate::simd_ops::ru32(P1)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x49u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x2a8u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x41u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), rcx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0xa8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rdx = 0xf8; rcx = ((r15.wrapping_add(0x5u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rax.wrapping_add(0x8u64), r15); rcx = rbx.wrapping_add(0x10); rax = P2; crate::simd_ops::wu64(rbx, rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; rdx = rbp.wrapping_sub(0x49); crate::simd_ops::wu8(rbx.wrapping_add(0x88u64), ((r15 as u8)) as u8); r9 = ((rsi as u32)) as u32 as u64; crate::simd_ops::lk_and_u32(rbx.wrapping_add(0x88u64), (0xffu32) as u32); rcx = rbx; crate::simd_ops::wu8(rbx.wrapping_add(0xb8u64), ((r15 as u8)) as u8); crate::simd_ops::lk_and_u32(rbx.wrapping_add(0xb8u64), (0xffu32) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0xe0u64), r14); crate::simd_ops::wu64(rbx.wrapping_add(0xe8u64), r15); crate::simd_ops::wu64(rbx.wrapping_add(0x80u64), r15); crate::simd_ops::wu64(rbx.wrapping_add(0x8cu64), (0x2bu64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x98u64), r15); crate::simd_ops::wu32(rbx.wrapping_add(0xa0u64), (0xffffffffu32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0xa4u64), (0x3f800000u32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0xa8u64), (0xffffffffu32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0xacu64), (0x1u32) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0xb0u64), r15); crate::simd_ops::wu64(rbx.wrapping_add(0xbcu64), (0x2bu64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0xc8u64), r15); crate::simd_ops::wu32(rbx.wrapping_add(0xd0u64), (0xffffffffu32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0xd4u64), (0x3f800000u32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0xd8u64), (0xffffffffu32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0xdcu64), (0x1u32) as u32); crate::simd_ops::wu64(r14.wrapping_add(0xf8u64), rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = ((r15.wrapping_add(0x2u64)) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0xc8u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            4 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0xf8u64))) as u64; rdi = (rdi & !0xffu64) | (((0x2u8) as u8 as u64)); rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x21u64), rax); bb = 5; continue; }
            5 => { crate::simd_ops::lk_or_u8(r14.wrapping_add(0x120u64), (0x2u8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x110u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), r13); fl_l = (crate::simd_ops::ru64(r14.wrapping_add(0xf0u64))) as u64; fl_r = r15; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            7 => { r12 = r14.wrapping_add(0xb0); crate::simd_ops::wu32(rbp.wrapping_sub(0x49u64), ((rsi as u32)) as u32); r13 = r12; rsi = 0xffffffff; bb = 8; continue; }
            8 => { r8 = rbp.wrapping_sub(0x69); crate::simd_ops::wu64(rbp.wrapping_sub(0x69u64), (0x0u64) as u64); rdx = ((r15 as u32)) as u32 as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x69u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 39; continue; } bb = 9; continue; }
            9 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbp.wrapping_sub(0x51u64), (0x1u32) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x6fu64), rcx); r8 = rbp.wrapping_sub(0x59); crate::simd_ops::wu8(rbp.wrapping_sub(0x59u64), ((rcx as u8)) as u8); rdx = rbp.wrapping_add(0x6f); crate::simd_ops::wu32(rbp.wrapping_sub(0x55u64), ((rcx as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_sub(0x4bu64), ((rcx as u8)) as u8); crate::simd_ops::wu16(rbp.wrapping_sub(0x4du64), (0x100u16) as u16); rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x30u64))) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 15; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x69u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 39; continue; } bb = 11; continue; }
            11 => { rax = ((rsi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x69u64), (0x0u64) as u64); bb = 39; continue; }
            14 => { crate::simd_ops::wu64(r14.wrapping_add(0xf8u64), r15); rax = 0x34; rsp = ((rsp).wrapping_add(0xc8u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            15 => { rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x69u64))) as u64; r8 = rbp.wrapping_sub(0x59); rdx = (rdx & !0xffu64) | (((0x3u8) as u8 as u64)); rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x30u64))) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 19; continue; } bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x6fu64))) as u64; rdx = P9; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x69u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 39; continue; } bb = 17; continue; }
            17 => { rax = ((rsi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_sub(0x69u64), (0x0u64) as u64); bb = 39; continue; }
            19 => { rax = ((crate::simd_ops::ru8(rbp.wrapping_sub(0x51u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_sub(0x50u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 16; continue; } bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_sub(0x4du64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 16; continue; } bb = 22; continue; }
            22 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_sub(0x4eu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 26; continue; } bb = 23; continue; }
            23 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 56; continue; } bb = 24; continue; }
            24 => { rdx = 0xe8; rcx = 0x5; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 56; continue; } bb = 25; continue; }
            25 => { rdx = 0; fl_l = 0; fl_r = 0; r8 = 0xe8; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = P12; crate::simd_ops::wu8(rbx.wrapping_add(0xe0u64), (0x0u8) as u8); fl_l = (crate::simd_ops::lk_and_u32(rbx.wrapping_add(0xe0u64), (0xffu32) as u32)) as u32 as u64; fl_r = fl_l; crate::simd_ops::wu64(rbx, rax); bb = 31; continue; }
            26 => { rcx = 0x5; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 29; continue; } bb = 27; continue; }
            27 => { rdx = 0xe8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 56; continue; } bb = 28; continue; }
            28 => { rdx = 0; fl_l = 0; fl_r = 0; r8 = 0xe8; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = P13; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu8(rbx.wrapping_add(0xe0u64), (0x0u8) as u8); fl_l = (crate::simd_ops::lk_and_u32(rbx.wrapping_add(0xe0u64), (0xffu32) as u32)) as u32 as u64; fl_r = fl_l; bb = 31; continue; }
            29 => { rdx = 0x168; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 56; continue; } bb = 30; continue; }
            30 => { rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x168; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 56; continue; } bb = 31; continue; }
            31 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbp.wrapping_sub(0x49); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x6fu64))) as u64; r8 = rbp.wrapping_sub(0x69); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); r9 = ((r15 as u32)) as u32 as u64; rcx = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x58u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 33; continue; } bb = 32; continue; }
            32 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbx; rcx = 0x5; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P15, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 34; continue; }
            33 => { rax = (((rdi as u8)) as u32) as u32 as u64; fl_l = (((rdi as u8)).wrapping_add(1)) as u8 as u64; rdi = (rdi & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu64(r13, rbx); crate::simd_ops::wu64(rbp.wrapping_add(rax.wrapping_mul(8)).wrapping_sub(0x29u64), rbx); bb = 34; continue; }
            34 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x69u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 38; continue; } bb = 35; continue; }
            35 => { rax = ((rsi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 37; continue; } bb = 36; continue; }
            36 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 37; continue; }
            37 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x69u64), (0x0u64) as u64); bb = 38; continue; }
            38 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; bb = 39; continue; }
            39 => { r15 = (((r15 as u32)).wrapping_add(1)) as u32 as u64; r13 = ((r13).wrapping_add(0x8u64)) as u64; fl_l = ((r15 as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 8; continue; } bb = 40; continue; }
            40 => { rdx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x45u64))) as u32 as u64; rcx = r14.wrapping_add(0x88); crate::simd_ops::wu32(r14.wrapping_add(0x160u64), ((rdx as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P16, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 55; continue; } bb = 41; continue; }
            41 => { rax = (((rdi as u8)) as u32) as u32 as u64; rcx = r14.wrapping_add(0x78); r13 = r14.wrapping_add(0xd0); rdx = rbx; rdi = (rdi & !0xffu64) | (((((rdi as u8)).wrapping_add(1)) as u8 as u64)); crate::simd_ops::wu64(rbp.wrapping_add(rax.wrapping_mul(8)).wrapping_sub(0x29u64), rcx); rcx = r13; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P17, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 55; continue; } bb = 42; continue; }
            42 => { rax = (((rdi as u8)) as u32) as u32 as u64; rbx = rbp.wrapping_sub(0x29); crate::simd_ops::wu64(rbx.wrapping_add(rax.wrapping_mul(8)), r13); rbx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 45; continue; } bb = 43; continue; }
            43 => { bb = 44; continue; }
            44 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_sub(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rbx.wrapping_sub(0x8); fl_l = (((rdi as u8)).wrapping_add(0xffu8)) as u8 as u64; fl_r = fl_l; rdi = (rdi & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); if (((fl_l as u8)) != 0) { bb = 44; continue; } bb = 45; continue; }
            45 => { rcx = (crate::simd_ops::ru64(r14.wrapping_add(0xf8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 47; continue; } bb = 46; continue; }
            46 => { rdx = rbp.wrapping_sub(0x49); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P18, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 47; continue; }
            47 => { r13 = 0; fl_l = 0; fl_r = 0; rbx = ((r13 as u32)) as u32 as u64; bb = 48; continue; }
            48 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0xe8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 54; continue; } bb = 49; continue; }
            49 => { r9 = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; rdi = (crate::simd_ops::ru64(r12)) as u64; rcx = (crate::simd_ops::ru64(r9.wrapping_add(0x1d8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 54; continue; } bb = 50; continue; }
            50 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = rbp.wrapping_sub(0x39); r9 = (crate::simd_ops::ru64(r9.wrapping_add(0x20u64))) as u64; rdx = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), r13); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x1b0u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x39u64))) as u64; rax = ((crate::simd_ops::ru16(rbp.wrapping_sub(0x31u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rdi.wrapping_add(0xcau64), ((rax as u16)) as u16); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 54; continue; } bb = 51; continue; }
            51 => { rax = ((rsi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 53; continue; } bb = 52; continue; }
            52 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 53; continue; }
            53 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), r13); bb = 54; continue; }
            54 => { rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; r12 = ((r12).wrapping_add(0x8u64)) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 48; continue; } bb = 55; continue; }
            55 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rax = ((r15 as u32)) as u32 as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x110u64))) as u64; rsp = ((rsp).wrapping_add(0xc8u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            56 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x6fu64))) as u64; rdx = P9; r15 = 0x2; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x69u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 55; continue; } bb = 57; continue; }
            57 => { let xa_src: u32 = (rsi as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rsi = (xa_old) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 55; continue; } bb = 58; continue; }
            58 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 55; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141600270(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415cf650; const P1: u64 = 0x1415940e0; const P2: u64 = 0x1415cfa40; const P3: u64 = 0x1416003b0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(r11.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rdi = (crate::simd_ops::ru64(rdx.wrapping_add(0x290u64))) as u64; r14 = rcx; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x338u64))) as u64; rbp = (((r8 as u8)) as u32) as u32 as u64; rbx = rdx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru32(rdx.wrapping_add(0x340u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 2; continue; }
            2 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rsp.wrapping_add(0x50u64), (0x1u8) as u8); crate::simd_ops::wu64(r11.wrapping_sub(0x50u64), rax); r8 = r11.wrapping_sub(0x30); crate::simd_ops::wu64(r11.wrapping_sub(0x58u64), rax); r9 = 0; fl_l = 0; fl_r = 0; rax = r11.wrapping_add(0x10); crate::simd_ops::wu64(r11.wrapping_sub(0x60u64), rax); rax = r11.wrapping_add(0x20); crate::simd_ops::wu64(r11.wrapping_sub(0x68u64), rax); rax = r11.wrapping_sub(0x38); crate::simd_ops::wu64(r11.wrapping_sub(0x70u64), rax); rax = r11.wrapping_sub(0x34); crate::simd_ops::wu64(r11.wrapping_sub(0x78u64), rax); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rsi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = 0x1; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = ((rsi as u32)) as u32 as u64; bb = 13; continue; }
            4 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rcx = 0xfffe; fl_l = (crate::simd_ops::ru16(rax)) as u16 as u64; fl_r = ((rcx as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu32(rdi.wrapping_add(0x18u64), (0x10001u32) as u32); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0xa8u64))) as u32 as u64; rcx = (((rcx as u32)).wrapping_add(crate::simd_ops::ru32(rsp.wrapping_add(0xb8u64)))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x340u64))) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffff8fu32))) as u32 as u64; fl_l = ((((rax as u32)) | (0xcu32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((rax as u32)) as u32); bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(2u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0x1fu32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; r8 = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { r9 = (r9 & !0xffu64) | (((0x1u8) as u8 as u64)); r8 = (((rbp as u8)) as u32) as u32 as u64; rdx = rax; rcx = r14; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            12 => { rdx = 0x1; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = 0x2; bb = 13; continue; }
            13 => { r11 = rsp.wrapping_add(0x80); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416003b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141594660; const P1: u64 = 0x141524340; const P2: u64 = 0x141524330; const P3: u64 = 0x1415242e0; const P4: u64 = 0x141591ed0; const P5: u64 = 0x141596980; const P6: u64 = 0x1415cfad0; const P7: u64 = 0x1416031f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; r13 = 0; fl_l = 0; fl_r = 0; r12 = (((r9 as u8)) as u32) as u32 as u64; r15 = (((r8 as u8)) as u32) as u32 as u64; r14 = rdx; rbx = rcx; crate::simd_ops::wu64(rdi.wrapping_add(0x298u64), rcx); fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 19; continue; } bb = 1; continue; }
            1 => { fl_l = ((r9 as u8)) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 19; continue; } bb = 2; continue; }
            2 => { rdx = rcx.wrapping_add(0x118); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(0x11cu64), ((rax as u32)) as u32); fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x1bcu64))) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 19; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x1bcu64))) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 19; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x1b0u64))) as u64; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbp); rbp = (crate::simd_ops::ru32(rdi.wrapping_add(0x1b8u64))) as u32 as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { r8 = 0x88; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 16; continue; } bb = 6; continue; }
            6 => { bb = 17; continue; }
            7 => { rdx = 0x88; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x1b0u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 9; continue; }
            9 => { fl_l = rax; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 10; continue; }
            10 => { fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 11; continue; }
            11 => { r9 = (0xffffffffffffffccu64) as u64; rdx = rsi.wrapping_add(0x34); fl_l = r9; fl_r = rsi; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 12; continue; }
            12 => { r8 = rdx.wrapping_sub(0x34); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rdx.wrapping_sub(0x4u64), r13); crate::simd_ops::wu32(rdx.wrapping_add(0x4u64), ((r13 as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), (0x3f800000u32) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0xcu64), (0x3f800000u32) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), (0x3f800000u64) as u64); crate::simd_ops::wu64(rdx.wrapping_add(0x18u64), r13); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r13); crate::simd_ops::wu32(rdx.wrapping_add(0x28u64), (0x42c80000u32) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x2cu64), (0x3f800000u32) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x30u64), (0xffffffffu32) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x34u64), ((r13 as u32)) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0x3cu64), r13); crate::simd_ops::wu64(rdx.wrapping_add(0x44u64), (0x3f800000u64) as u64); crate::simd_ops::wu32(rdx.wrapping_add(0x4cu64), ((r13 as u32)) as u32); bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x1b0u64))) as u64; rcx = ((rcx).wrapping_add(rdx)) as u64; rdx = ((rdx).wrapping_add(0x88u64)) as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(r9))) as u128; crate::simd_ops::wu128(r8, (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(r9).wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x10u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(r9).wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x20u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(r9).wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x30u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(r9).wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x40u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(r9).wrapping_add(0x50u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x50u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(r9).wrapping_add(0x60u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x60u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(r9).wrapping_add(0x70u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x70u64), (xmm1) as u128); xmm0 = ((crate::simd_ops::ru64(rcx.wrapping_add(r9).wrapping_add(0x80u64))) as u64) as u128; crate::simd_ops::wu64(r8.wrapping_add(0x80u64), ((xmm0 as u64)) as u64); fl_l = rbp; fl_r = (0x1u64) as u64; rbp = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 15; continue; }
            15 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x1b0u64))) as u64; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 16; continue; }
            16 => { crate::simd_ops::wu64(rdi.wrapping_add(0x1b0u64), rsi); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rdi.wrapping_add(0x1bcu64), (0x1u32) as u32); bb = 17; continue; }
            17 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { rsi = 0x2; bb = 32; continue; }
            19 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x11cu64))) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 30; continue; } bb = 20; continue; }
            20 => { rcx = rdi.wrapping_add(0x18); rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 30; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x11cu64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rsi = ((rax.wrapping_add(0x2u64)) as u32) as u32 as u64; bb = 32; continue; }
            23 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x118u64))) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 30; continue; } bb = 24; continue; }
            24 => { fl_l = ((r15 as u8)) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 30; continue; } bb = 25; continue; }
            25 => { fl_l = ((r12 as u8)) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 30; continue; } bb = 26; continue; }
            26 => { rsi = 0x1; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 29; continue; } bb = 27; continue; }
            27 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x60u64))) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 29; continue; } bb = 28; continue; }
            28 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 29; continue; }
            29 => { fl_l = (crate::simd_ops::lk_and_u8(rbx.wrapping_add(0x60u64), (0xfeu8) as u8)) as u8 as u64; fl_r = fl_l; bb = 34; continue; }
            30 => { rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 33; continue; } bb = 31; continue; }
            31 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3f; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 33; continue; } bb = 32; continue; }
            32 => { rax = (crate::simd_ops::ru64(r14)) as u64; rdx = 0x1; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x58u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(r14)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = r14; rcx = 0x5; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 37; continue; }
            33 => { fl_l = ((r15 as u8)) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 35; continue; } bb = 34; continue; }
            34 => { crate::simd_ops::wu64(rbx.wrapping_add(0xe8u64), r14); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; fl_l = rax; fl_r = rax; rcx = rax.wrapping_add(0x18); rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r13 } else { rcx }) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rcx); fl_l = (crate::simd_ops::lk_and_u8(rdi.wrapping_add(0x306u64), (0xfdu8) as u8)) as u8 as u64; fl_r = fl_l; bb = 37; continue; }
            35 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x120u64))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; crate::simd_ops::wu64(rbx.wrapping_add(0xf0u64), r14); if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 37; continue; } bb = 36; continue; }
            36 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 37; continue; }
            37 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = ((rsi as u32)) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416006a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xa8; const P1: u64 = 0x18; const P2: u64 = 0x290;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdi = rdx; rsi = rcx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rbx = (((rax as u8)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rbx as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; r8 = (r8 & !0xffu64) | (((if (((fl_l as u8)) == ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64)); if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(P1))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rax = (rax & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (((r8 as u8)) as u32) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141600720(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14164b950; const P1: u64 = 0x1415242e0; const P2: u64 = 0x141642fc0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0xf8u64))) as u64; r14 = 0; fl_l = 0; fl_r = 0; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0xf8u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = ((r14.wrapping_add(0x5u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rdi.wrapping_add(0xf8u64), r14); bb = 4; continue; }
            4 => { rbp = 0x4; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); rbx = rdi.wrapping_add(0xb0); bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rsi = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rsi; rcx = 0x5; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rbx, r14); bb = 9; continue; }
            9 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbp; fl_r = (0x1u64) as u64; rbp = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 10; continue; }
            10 => { rcx = rdi.wrapping_add(0x88); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_and_u8(rdi.wrapping_add(0x120u64), (0xfdu8) as u8); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x108u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 11; continue; }
            11 => { rsi = ((r14 as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x110u64))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 16; continue; } bb = 12; continue; }
            12 => { rbx = r14; bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(rdx).wrapping_add(0x18u64))) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; rax = ((rbx.wrapping_add(rdx)) as u64) as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rcx = 0x5; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x108u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(rbx).wrapping_add(0x8u64), r14); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x108u64))) as u64; bb = 15; continue; }
            15 => { rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; rbx = ((rbx).wrapping_add(0x20u64)) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x110u64))) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 13; continue; } bb = 16; continue; }
            16 => { rcx = 0x5; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(0x108u64), r14); bb = 17; continue; }
            17 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x110u64), r14); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141600860(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xffffffffffffffa0; const P1: u64 = 0xffffffffffffffe8; const P2: u64 = 0x141593cb0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    fl_l = rax; fl_r = rax;
    rcx = ((rax.wrapping_add(P1)) as u64) as u64;
    rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rdx } else { rcx }) as u64;
    return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141600a30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xe8; const P1: u64 = 0x18; const P2: u64 = 0x8; const P3: u64 = 0x20; const P4: u64 = 0x270; const P5: u64 = 0x143930f68; const P6: u64 = 0x1415d1020; const P7: u64 = 0x68; const P8: u64 = 0x12; const P9: u64 = 0x100; const P10: u64 = 0xf0; const P11: u64 = 0x1415cfad0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rdx; rdi = rcx; rsi = (crate::simd_ops::ru64(r9.wrapping_add(P1))) as u64; fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(P2))) as u32 as u64; fl_r = 0x100000; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64; fl_r = 0xffffffff; r8 = ((rdx.wrapping_add(P3)) as u64) as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru32(rsi.wrapping_add(P4))) as u32 as u64; rcx = (crate::simd_ops::ru64(P5)) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P7))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 4; continue; }
            4 => { rcx = ((crate::simd_ops::ru16(rbx.wrapping_add(P8))) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu16(rbx.wrapping_add(P8), ((rax as u16)) as u16); bb = 6; continue; }
            6 => { crate::simd_ops::wu32(rdi.wrapping_add(P9), (0x2u32) as u32); bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P10))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu32(rbx.wrapping_add(0x30u64), ((rax as u32)) as u32); bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141600ad0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x8; const P2: u64 = 0x1415242e0; const P3: u64 = 0x14183acc8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r14); rdi = rcx; r14 = ((rdx as u32)) as u32 as u64; rbp = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); rsi = ((rbp as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r15); r15 = 0; fl_l = 0; fl_r = 0; rbx = ((r15 as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdi)) as u64; fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(rax).wrapping_add(P0))) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(rax).wrapping_add(P1))) as u64; rcx = 0x5; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(rax).wrapping_add(P1), r15); bb = 4; continue; }
            4 => { rbx = ((rbx).wrapping_add(0x20u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; rbx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rbp = ((rbp).wrapping_shl(5u32)) as u64; r8 = ((rbx as u32)) as u32 as u64; r8 = ((r8).wrapping_shl(5u32)) as u64; rdx = ((rcx.wrapping_add(rbp)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { crate::simd_ops::wu32(rdi.wrapping_add(P1), ((rbx as u32)) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141600b70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xe8; const P1: u64 = 0x1415cfad0; const P2: u64 = 0x328; const P3: u64 = 0x141601080; const P4: u64 = 0x1438152b4; const P5: u64 = 0x143931104; const P6: u64 = 0x2ac; const P7: u64 = 0x143426780; const P8: u64 = 0x143427068;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rdi = rdx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3f; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; fl_r = 0x0; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            3 => { rax = 0x3f; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            4 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 5; continue; }
            5 => { rax = ((crate::simd_ops::ru16(P4)) as u32) as u32 as u64; xmm1 = 0u128; rcx = (crate::simd_ops::ru32(P5)) as u32 as u64; xmm0 = 0u128; rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul(((rcx as u32)) as i32) as u32) as u32 as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(rdi.wrapping_add(P6)))).to_bits() as u128); fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { xmm0 = ((crate::simd_ops::ru32(P7)) as u32) as u128; bb = 8; continue; }
            7 => { xmm0 = ((crate::simd_ops::ru32(P8)) as u32) as u128; bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm0 as u32)))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 3; continue; } bb = 9; continue; }
            9 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            11 => { rax = 0x1; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0x2; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141600c40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xf8; const P1: u64 = 0x92; const P2: u64 = 0xe8; const P3: u64 = 0xe0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rdx = ((crate::simd_ops::ru16(rax.wrapping_add(P1))) as u32) as u32 as u64; } else { rdx = 0; fl_l = 0; fl_r = 0; }
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    return crate::simd_ops::call_fwd(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141600c70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1438152b4; const P1: u64 = 0x141601080; const P2: u64 = 0x143426780; const P3: u64 = 0x143427068; const P4: u64 = 0x1415cfad0; const P5: u64 = 0x1416006a0; const P6: u64 = 0x1416027d0; const P7: u64 = 0x141600fe0; const P8: u64 = 0x14164b600; const P9: u64 = 0x14164b270;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = rcx; rbx = rdx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0xf8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 24; continue; } bb = 1; continue; }
            1 => { rax = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; crate::simd_ops::wu16(rdx.wrapping_add(0x10u64), ((rax as u16)) as u16); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0xe8u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x78u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; fl_r = 0x2e; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; fl_r = 0x11; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 31; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0xf0u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 5; continue; }
            5 => { rsi = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0xe8u64))) as u64; rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x328u64))) as u32 as u64; rbp = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 11; continue; } bb = 6; continue; }
            6 => { rax = ((crate::simd_ops::ru16(rbx.wrapping_add(0x12u64))) as u32) as u32 as u64; xmm1 = 0u128; rcx = ((crate::simd_ops::ru16(rbx.wrapping_add(0x10u64))) as u32) as u32 as u64; xmm0 = 0u128; rcx = (((rcx as u32)).wrapping_sub((rax as u32))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(rsi.wrapping_add(0x2acu64)))).to_bits() as u128); fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(crate::simd_ops::ru32(P2))).to_bits() as u128); bb = 9; continue; }
            8 => { xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(crate::simd_ops::ru32(P3))).to_bits() as u128); bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm1 as u32)))) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rax = (if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { (rcx as u32) } else { (rax as u32) }) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x328u64), ((rdx as u32)) as u32); bb = 10; continue; }
            10 => { crate::simd_ops::wu32(rbx.wrapping_add(0x30u64), (0x11u32) as u32); bb = 31; continue; }
            11 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3f; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 12; continue; }
            12 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu32(rbx.wrapping_add(0x30u64), (0x2u32) as u32); bb = 31; continue; }
            14 => { xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_add(0x2a0u64))) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm0 as u64)) as u64); xmm0 = ((crate::simd_ops::ru64(rsi.wrapping_add(0x2a0u64))) as u64) as u128; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdx = ((rdx).wrapping_shr(32u32)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), ((xmm0 as u64)) as u64); rax = ((rdx as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = ((rcx).wrapping_shr(32u32)) as u64; rax = ((((rax as u32)) ^ ((rcx as u32)))) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 10; continue; } bb = 15; continue; }
            15 => { rax = ((rdx as u32)) as u32 as u64; rax = ((((rax as u32)) ^ ((rcx as u32)))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xf00; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 10; continue; } bb = 16; continue; }
            16 => { rcx = ((((rcx as u32)) ^ ((rdx as u32)))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0xfffff000; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 10; continue; } bb = 17; continue; }
            17 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0xf0u64))) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0xe8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; rsi = ((rsi).wrapping_add(0x18u64)) as u64; rcx = rsi; fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x1f8u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x68u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 22; continue; }
            20 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x70u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x1f8u64))) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x78u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 22; continue; }
            22 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = ((crate::simd_ops::ru16(rbx.wrapping_add(0x12u64))) as u32) as u32 as u64; rax = 0x2d; fl_l = ((rdx as u16)) as u16 as u64; fl_r = (crate::simd_ops::ru16(rbx.wrapping_add(0x10u64))) as u16 as u64; r8 = 0x2b; rax = (if (((fl_l as u16)) != ((fl_r as u16))) { (r8 as u32) } else { (rax as u32) }) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x30u64), ((rax as u32)) as u32); if (((fl_l as u16)) == ((fl_r as u16))) { bb = 31; continue; } bb = 23; continue; }
            23 => { rdx = rbx; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 31; continue; }
            24 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; fl_r = 0x2b; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 31; continue; } bb = 25; continue; }
            25 => { bb = 26; continue; }
            26 => { rax = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; rdx = rbx; crate::simd_ops::wu16(rbx.wrapping_add(0x10u64), ((rax as u16)) as u16); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0xe8u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x78u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; fl_r = 0x2e; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 28; continue; } bb = 27; continue; }
            27 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 28; continue; }
            28 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2d; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 30; continue; } bb = 29; continue; }
            29 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x11; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 31; continue; } bb = 30; continue; }
            30 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0xf8u64))) as u64; rdx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; fl_r = 0x2b; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 26; continue; } bb = 31; continue; }
            31 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141600ec0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x58;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141600ed0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x3a;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rcx = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = ((crate::simd_ops::ru8(rax.wrapping_add(P1))) as u32) as u32 as u64;
    fl_l = ((((rax as u8)) & (0x7u8))) as u8 as u64; fl_r = fl_l;
    rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141600f00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x78;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rcx = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32) as u128;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141600f30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x141605350; const P2: u64 = 0x18; const P3: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = r8; rdi = rdx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r9 = rax; rcx = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rbx, ((rcx as u32)) as u32); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx)) as u32 as u64; rcx = (if (((fl_l as u32)) >= ((fl_r as u32))) { crate::simd_ops::ru32(rbx) } else { (rcx as u32) }) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx, ((rcx as u32)) as u32); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(r9.wrapping_add(P3))) as u64; rdx = ((r8.wrapping_mul(8)) as u64) as u64; r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(rax))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(rdi), rcx); rcx = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 4; continue; } bb = 5; continue; }
            5 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r9.wrapping_add(P2))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (rax & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141600fb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x141605370;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141600fc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x38;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rcx = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = ((crate::simd_ops::ru16(rax.wrapping_add(P1))) as u32) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141600fe0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1438152b4; const P1: u64 = 0x10; const P2: u64 = 0xe8; const P3: u64 = 0x78; const P4: u64 = 0x30; const P5: u64 = 0x141601080;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64;
    rdi = rcx;
    crate::simd_ops::wu16(rdx.wrapping_add(P1), ((rax as u16)) as u16);
    rbx = rdx;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; fl_r = 0x2e;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141601030(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xffffffffffffffa0; const P1: u64 = 0x258;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rcx = 0x270;
    fl_l = rdx; fl_r = rdx;
    rax = ((rdx.wrapping_add(P1)) as u64) as u64;
    rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rcx } else { rax }) as u64;
    rax = (crate::simd_ops::ru32(rax)) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141601060(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xffffffffffffffa0; const P1: u64 = 0x2f8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdx = 0x310;
    fl_l = rax; fl_r = rax;
    rcx = ((rax.wrapping_add(P1)) as u64) as u64;
    rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rdx } else { rcx }) as u64;
    rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(crate::simd_ops::ru32(rcx))))) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141601080(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xe8; const P1: u64 = 0x20; const P2: u64 = 0x120; const P3: u64 = 0x8; const P4: u64 = 0x258; const P5: u64 = 0x141591570; const P6: u64 = 0x143930f90; const P7: u64 = 0x14159b210;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P1))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::lk_or_u8(rcx.wrapping_add(P2), (0x1u8) as u8); rdx = 0x270; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; fl_l = rcx; fl_r = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rax = ((rcx.wrapping_add(P4)) as u64) as u64; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rdx } else { rax }) as u64; rbx = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = 0x18; rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax } else { rcx }) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P6)) as u64; r8 = ((rax as u32)) as u32 as u64; rdx = ((rbx as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return crate::simd_ops::tail_stk(P7, rax, rcx, rdx, r8, r9, rsp); }
            2 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 3; continue; }
            3 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416010f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1c; const P1: u64 = 0x110; const P2: u64 = 0x108; const P3: u64 = 0x18; const P4: u64 = 0x143930f90; const P5: u64 = 0x14159ac60; const P6: u64 = 0x40; const P7: u64 = 0x141600ad0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; rdi = rdx; rbp = rcx; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; fl_r = 0x0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = 0; fl_l = 0; fl_r = 0; bb = 4; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64; r8 = ((rsi as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(P4)) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x78u64))) as u64; rcx = rbp.wrapping_add(0x78); rdx = ((rsi as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi as u32)) as u32 as u64; rcx = rbp.wrapping_add(0x108); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P3), (0x0u64) as u64); bb = 5; continue; }
            5 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141601180(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xb0; const P1: u64 = 0x88; const P2: u64 = 0x10; const P3: u64 = 0x141647270; const P4: u64 = 0xe8; const P5: u64 = 0x98;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = 0; fl_l = 0; fl_r = 0; rsi = ((rcx.wrapping_add(P0)) as u64) as u64; rbx = ((rdi as u32)) as u32 as u64; rbp = (((r8 as u8)) as u32) as u32 as u64; r15 = rdx; r13 = rcx; bb = 1; continue; }
            1 => { r14 = (crate::simd_ops::ru64(rsi)) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(r14)) as u64; rcx = r14; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = ((r14.wrapping_add(P2)) as u64) as u64; r9 = (((rbp as u8)) as u32) as u32 as u64; r8 = rax; rdx = r15; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 10; continue; } bb = 3; continue; }
            3 => { rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; rsi = ((rsi).wrapping_add(0x8u64)) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 1; continue; } bb = 4; continue; }
            4 => { rbx = ((r13.wrapping_add(P4)) as u64) as u64; bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = (((rbp as u8)) as u32) as u32 as u64; rdx = r15; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 8; continue; }
            8 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            10 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 9; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141601240(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xb0; const P1: u64 = 0x88; const P2: u64 = 0x10; const P3: u64 = 0x141647310; const P4: u64 = 0xe8; const P5: u64 = 0x90;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = 0; fl_l = 0; fl_r = 0; rsi = ((rcx.wrapping_add(P0)) as u64) as u64; rbx = ((rdi as u32)) as u32 as u64; rbp = rdx; r15 = rcx; bb = 1; continue; }
            1 => { r14 = (crate::simd_ops::ru64(rsi)) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(r14)) as u64; rcx = r14; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = ((r14.wrapping_add(P2)) as u64) as u64; rdx = rbp; r8 = rax; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 10; continue; } bb = 3; continue; }
            3 => { rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; rsi = ((rsi).wrapping_add(0x8u64)) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 1; continue; } bb = 4; continue; }
            4 => { rbx = ((r15.wrapping_add(P4)) as u64) as u64; bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbp; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 8; continue; }
            8 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            10 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 9; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141601300(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x100; const P1: u64 = 0xe8; const P2: u64 = 0x1415cfce0; const P3: u64 = 0x1415cfb10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { crate::simd_ops::wu32(rbx.wrapping_add(P0), (0x2u32) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbx.wrapping_add(P0), (0x3u32) as u32); bb = 6; continue; }
            6 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141601360(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xb0; const P1: u64 = 0x18; const P2: u64 = 0x1d8; const P3: u64 = 0x20; const P4: u64 = 0x1b0; const P5: u64 = 0xca; const P6: u64 = 0x14;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); rbx = ((r14 as u32)) as u32 as u64; rdi = ((rcx.wrapping_add(P0)) as u64) as u64; rbp = rcx; bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0xe8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 2; continue; }
            2 => { r9 = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; rsi = (crate::simd_ops::ru64(rdi)) as u64; rcx = (crate::simd_ops::ru64(r9.wrapping_add(P2))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = rsp.wrapping_add(0x20); r9 = (crate::simd_ops::ru64(r9.wrapping_add(P3))) as u64; rdx = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = ((crate::simd_ops::ru16(rsp.wrapping_add(0x28u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rsi.wrapping_add(P5), ((rax as u16)) as u16); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(P6), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); bb = 7; continue; }
            7 => { rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; rdi = ((rdi).wrapping_add(0x8u64)) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 1; continue; } bb = 8; continue; }
            8 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141601420(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x78; const P1: u64 = 0x10; const P2: u64 = 0x141645b90;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rbx = rcx;
    rcx = ((rcx).wrapping_add(0x78u64)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rcx = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141601470(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xe8; const P1: u64 = 0x58; const P2: u64 = 0x1415242e0; const P3: u64 = 0x141600720; const P4: u64 = 0x60;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_or_u8(rbx.wrapping_add(P4), (0x1u8) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi);
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    fl_l = rdi; fl_r = rdi;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rdi; rcx = 0x5; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu64(rbx.wrapping_add(P0), (0x0u64) as u64);
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::lk_or_u8(rbx.wrapping_add(P4), (0x1u8) as u8);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416014e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14268e670;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; rbx = rcx; rbp = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 13; continue; } bb = 1; continue; }
            1 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(0x14u64))) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 13; continue; } bb = 2; continue; }
            2 => { r10 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rdx = ((rbp.wrapping_mul(2)) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((rdx).wrapping_add(rbp)) as u64; rdx = ((rdx).wrapping_shl(3u32)) as u64; rsi = ((rax as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { r8 = rdx; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0xcdu32) as u32); rdx = r10; r9 = P0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 4; continue; }
            4 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 14; continue; }
            5 => { r9 = 0xcc; r8 = P0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 7; continue; }
            7 => { fl_l = rax; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 8; continue; }
            8 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = rcx.wrapping_add(0x18); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(rcx).wrapping_sub(0x18u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(rdi).wrapping_sub(0x18u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(rcx).wrapping_sub(0x8u64))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(rdi).wrapping_sub(0x8u64), ((xmm1 as u64)) as u64); fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rdi); crate::simd_ops::wu32(rbx.wrapping_add(0x14u64), ((rbp as u32)) as u32); bb = 13; continue; }
            13 => { crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), ((rbp as u32)) as u32); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416015e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x164;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu32(rax.wrapping_add(P1), (0x42ca0000u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416015f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x100; const P1: u64 = 0xe8; const P2: u64 = 0x1415cfce0; const P3: u64 = 0xf8; const P4: u64 = 0x1416431e0; const P5: u64 = 0x1434268d0; const P6: u64 = 0x1415cfb50;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1;
    rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64;
    if (((fl_l as u32)) == ((fl_r as u32))) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2;
    if !((((fl_l as u32)) == ((fl_r as u32)))) { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_l = rcx; fl_r = rcx; if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); } crate::simd_ops::wu32(rbx.wrapping_add(P0), (0x2u32) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = ((rcx).wrapping_add(0x10u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); } else { xmm0 = ((crate::simd_ops::ru32(P5)) as u32) as u128; }
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    xmm1 = (xmm0) as u128;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    crate::simd_ops::wu32(rbx.wrapping_add(P0), (0x1u32) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141601670(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x118; const P1: u64 = 0x11c; const P2: u64 = 0x8; const P3: u64 = 0xffffffffffffffe8; const P4: u64 = 0x305; const P5: u64 = 0x306; const P6: u64 = 0x2fc; const P7: u64 = 0xe8; const P8: u64 = 0x20; const P9: u64 = 0x78; const P10: u64 = 0x10; const P11: u64 = 0x141645b90; const P12: u64 = 0x1415cfce0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x0; rdi = rcx; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rdx = 0; fl_l = 0; fl_r = 0; fl_l = rax; fl_r = rax; rcx = ((rax.wrapping_add(P3)) as u64) as u64; rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rdx } else { rcx }) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::lk_and_u8(rcx.wrapping_add(P4), (0x3fu8) as u8); crate::simd_ops::lk_and_u8(rcx.wrapping_add(P5), (0xfeu8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(P6), ((rdx as u32)) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P7))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P8))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P9))) as u64; rcx = ((rcx).wrapping_add(0x78u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P10))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x78u64))) as u64; rcx = rdi.wrapping_add(0x78); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0xe8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { crate::simd_ops::wu32(rdi.wrapping_add(0x100u64), (0x2u32) as u32); bb = 10; continue; }
            10 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141601720(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xb0; const P1: u64 = 0xca; const P2: u64 = 0xb8; const P3: u64 = 0xc0; const P4: u64 = 0xc8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (((rdx as u16)) as u32) as u32 as u64; fl_l = ((((rax as u16)) & (0x1u16))) as u16 as u64; fl_r = fl_l; rax = (rax & !0xffffu64) | ((((fl_l as u16)) as u16 as u64)); crate::simd_ops::wu16(r9.wrapping_add(P1), ((rax as u16)) as u16); bb = 3; continue; }
            3 => { r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = ((rdx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((((rax as u16)) & (0x1u16))) as u16 as u64; fl_r = fl_l; rax = (rax & !0xffffu64) | ((((fl_l as u16)) as u16 as u64)); crate::simd_ops::wu16(r9.wrapping_add(P1), ((rax as u16)) as u16); bb = 6; continue; }
            6 => { r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = ((rdx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(2u32)) as u32 as u64; fl_l = ((((rax as u16)) & (0x1u16))) as u16 as u64; fl_r = fl_l; rax = (rax & !0xffffu64) | ((((fl_l as u16)) as u16 as u64)); crate::simd_ops::wu16(r9.wrapping_add(P1), ((rax as u16)) as u16); bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x8u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rdx = (((rdx as u32)).wrapping_shr(3u32)) as u32 as u64; fl_l = ((((rdx as u16)) & (0x1u16))) as u16 as u64; fl_r = fl_l; rdx = (rdx & !0xffffu64) | ((((fl_l as u16)) as u16 as u64)); crate::simd_ops::wu16(rax.wrapping_add(P1), ((rdx as u16)) as u16); bb = 12; continue; }
            12 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416017b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x140; const P2: u64 = 0x148; const P3: u64 = 0x128; const P4: u64 = 0x18; const P5: u64 = 0x20;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r10 = rcx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; r8 = (((rcx) as i64).wrapping_mul((0x118u64) as i64) as u64) as u64; r8 = ((r8).wrapping_add(rax)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rcx = ((r10.wrapping_add(P3)) as u64) as u64; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rax.wrapping_add(P4), rcx); rax = ((rax).wrapping_add(0x118u64)) as u64; r8 = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; r9 = (((r8) as i64).wrapping_mul((0x118u64) as i64) as u64) as u64; r9 = ((r9).wrapping_add(crate::simd_ops::ru64(rdx.wrapping_add(P1)))) as u64; fl_l = rax; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P5))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 6; continue; }
            6 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141601820(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x100; const P1: u64 = 0xe8; const P2: u64 = 0x1415cfce0; const P3: u64 = 0x1415cfbb0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rbx = rcx; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { crate::simd_ops::wu32(rbx.wrapping_add(P0), (0x2u32) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbx.wrapping_add(P0), (0x1u32) as u32); bb = 6; continue; }
            6 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416026f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xe8; const P1: u64 = 0x1415cfce0; const P2: u64 = 0x100;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    crate::simd_ops::wu32(rbx.wrapping_add(P2), (0x2u32) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141602720(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xe8; const P1: u64 = 0x18; const P2: u64 = 0x20; const P3: u64 = 0x88; const P4: u64 = 0x1415cfce0; const P5: u64 = 0x100; const P6: u64 = 0xf0; const P7: u64 = 0x58; const P8: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { fl_l = rdx; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P2))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { crate::simd_ops::wu32(rbx.wrapping_add(P5), (0x2u32) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { fl_l = rdx; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P7))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rdi; rcx = 0x5; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P6), (0x0u64) as u64); bb = 12; continue; }
            12 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416027d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14164b8e0; const P1: u64 = 0x141524330; const P2: u64 = 0x1426469f0; const P3: u64 = 0x1415242e0; const P4: u64 = 0x1415cfbb0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rsi = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0xf8u64))) as u64; rdi = 0; fl_l = 0; fl_r = 0; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0xf0u64))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            2 => { rax = ((rdi as u32)) as u32 as u64; rcx = rsi.wrapping_add(0xb0); bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; rcx = ((rcx).wrapping_add(0x8u64)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 3; continue; } bb = 5; continue; }
            5 => { rcx = rsi.wrapping_add(0x78); bb = 7; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0xb0u64))) as u64; bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0xf0u64))) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { r15 = ((rdi as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x110u64))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 24; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); r14 = rdi; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r13); bb = 10; continue; }
            10 => { rbp = (crate::simd_ops::ru64(rsi.wrapping_add(0x108u64))) as u64; rdi = (crate::simd_ops::ru64(rbp.wrapping_add(r14).wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(r14).wrapping_add(0x18u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 22; continue; } bb = 12; continue; }
            12 => { rbx = (0xffffffffffffffffu64) as u64; bb = 13; continue; }
            13 => { rbx = ((rbx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(rbx))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 13; continue; } bb = 14; continue; }
            14 => { r13 = rbx.wrapping_add(0x1); rcx = 0x5; rdx = r13; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbp.wrapping_add(r14).wrapping_add(0x8u64), rax); r12 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 15; continue; }
            15 => { rcx = (0xffffffffffffffffu64) as u64; bb = 16; continue; }
            16 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(rcx))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 16; continue; } bb = 17; continue; }
            17 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 21; continue; } bb = 18; continue; }
            18 => { rbx = (0xffffffffffffffffu64) as u64; bb = 19; continue; }
            19 => { rbx = ((rbx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(rbx))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 19; continue; } bb = 20; continue; }
            20 => { fl_l = ((rbx).wrapping_add(1)) as u64; rbx = ((fl_l as u64)) as u64; bb = 21; continue; }
            21 => { r9 = rbx; r8 = rdi; rdx = r13; rcx = r12; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rbx.wrapping_add(r12), (0x0u8) as u8); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x108u64))) as u64; crate::simd_ops::wu8(r14.wrapping_add(rax).wrapping_add(0x18u64), (0x1u8) as u8); bb = 22; continue; }
            22 => { r15 = (((r15 as u32)).wrapping_add(1)) as u32 as u64; r14 = ((r14).wrapping_add(0x20u64)) as u64; fl_l = ((r15 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsi.wrapping_add(0x110u64))) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 10; continue; } bb = 23; continue; }
            23 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rdi = 0; fl_l = 0; fl_r = 0; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 24; continue; }
            24 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0xe8u64))) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x58u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0xe8u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0x5; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            26 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0xf0u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0xe8u64), rcx); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; fl_l = rax; fl_r = rax; crate::simd_ops::wu64(rsi.wrapping_add(0xf0u64), rdi); rdx = rax.wrapping_add(0x18); rdx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rdi } else { rdx }) as u64; crate::simd_ops::lk_and_u8(rsi.wrapping_add(0x120u64), (0xfbu8) as u8); crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rdx); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0xe8u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0xe0u64), rax); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; crate::simd_ops::lk_and_u8(rax.wrapping_add(0x306u64), (0xfdu8) as u8); rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416029d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xe8; const P1: u64 = 0x58; const P2: u64 = 0x1415242e0; const P3: u64 = 0x141600720; const P4: u64 = 0x60; const P5: u64 = 0xf0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rdi; rcx = 0x5; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rbx.wrapping_add(P0), (0x0u64) as u64); bb = 4; continue; }
            4 => { rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_or_u8(rbx.wrapping_add(P4), (0x1u8) as u8); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rdi; rcx = 0x5; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rbx.wrapping_add(P5), (0x0u64) as u64); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141602a80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426780; const P1: u64 = 0x143427068; const P2: u64 = 0x1415cfad0; const P3: u64 = 0x1416006a0; const P4: u64 = 0x1416027d0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0xe8u64))) as u64; rdi = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0xf0u64))) as u64; rsi = rdx; rbp = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { rax = ((crate::simd_ops::ru16(rdx.wrapping_add(0x12u64))) as u32) as u32 as u64; xmm1 = 0u128; rcx = ((crate::simd_ops::ru16(rdx.wrapping_add(0x10u64))) as u32) as u32 as u64; xmm0 = 0u128; rcx = (((rcx as u32)).wrapping_sub((rax as u32))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_add(0x2acu64)))).to_bits() as u128); fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(crate::simd_ops::ru32(P0))).to_bits() as u128); bb = 4; continue; }
            3 => { xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(crate::simd_ops::ru32(P1))).to_bits() as u128); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm1 as u32)))) as u32 as u64; rax = ((r8 as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; rax = (if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { (rcx as u32) } else { (rax as u32) }) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; r8 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x328u64), ((r8 as u32)) as u32); bb = 5; continue; }
            5 => { rax = 0x11; bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3f; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 8; continue; }
            8 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = 0x2; bb = 6; continue; }
            10 => { xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_add(0x2a0u64))) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm0 as u64)) as u64); xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x2a0u64))) as u64) as u128; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdx = ((rdx).wrapping_shr(32u32)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = ((rcx).wrapping_shr(32u32)) as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((((rax as u32)) ^ ((rdx as u32)))) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 11; continue; }
            11 => { rax = ((rcx as u32)) as u32 as u64; rax = ((((rax as u32)) ^ ((rdx as u32)))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xf00; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 5; continue; } bb = 12; continue; }
            12 => { rcx = ((((rcx as u32)) ^ ((rdx as u32)))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0xfffff000; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 5; continue; } bb = 13; continue; }
            13 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0xf0u64))) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0xe8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rbx = ((rbx).wrapping_add(0x18u64)) as u64; rcx = rbx; fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x1f8u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x68u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 18; continue; }
            16 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x70u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x1f8u64))) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x78u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 18; continue; }
            18 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru16(rsi.wrapping_add(0x10u64))) as u32) as u32 as u64; rax = 0x2d; fl_l = (crate::simd_ops::ru16(rsi.wrapping_add(0x12u64))) as u16 as u64; fl_r = ((rcx as u16)) as u16 as u64; rdx = 0x2b; rax = (if (((fl_l as u16)) != ((fl_r as u16))) { (rdx as u32) } else { (rax as u32) }) as u32 as u64; bb = 6; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141602c00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xb0; const P1: u64 = 0xca;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((rdx as u32)) as u32 as u64;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(P0))) as u64;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { fl_l = (crate::simd_ops::lk_add_u16(rdx.wrapping_add(P1), ((r8 as u16)) as u16)) as u16 as u64; fl_r = fl_l; }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141602c20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1438152ac; const P1: u64 = 0x140348730; const P2: u64 = 0x1415242e0; const P3: u64 = 0x14158e830; const P4: u64 = 0x14158f340; const P5: u64 = 0x143815b50; const P6: u64 = 0x141524330; const P7: u64 = 0x14183acc2; const P8: u64 = 0x1415ff690; const P9: u64 = 0x14268e9e8; const P10: u64 = 0x14268ea80; const P11: u64 = 0x1415ff740; const P12: u64 = 0x141602c20; const P13: u64 = 0x141642fc0; const P14: u64 = 0x141642740;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut fl_k: u32 = 0;
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu32(rax.wrapping_add(0x10u64), ((rdx as u32)) as u32); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rbp = ((rax.wrapping_sub(0x5fu64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; r12 = ((((rdx as u32)) as i32 as i64) as u64) as u64; r13 = (rcx) as u64; fl_l = ((r12 as u32)) as u32 as u64; fl_r = (0xffffffffu32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 57; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rbx) as u64); rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rax.wrapping_sub(0x20u64), (rsi) as u64); r8 = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu64(rax.wrapping_sub(0x28u64), (rdi) as u64); crate::simd_ops::wu64(rax.wrapping_sub(0x38u64), (r15) as u64); rax = ((r12.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = (rax as u32 as i32 as i64) as u64; crate::simd_ops::wu8(rbp.wrapping_add(0x77u64), ((rbx as u8)) as u8); fl_l = (rax) as u64 as u64; fl_r = (0x4u64) as u64 as u64; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { fl_k = 1; bb = 7; continue; } bb = 2; continue; }
            2 => { fl_l = ((rcx).wrapping_add(0xb0u64)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; rcx = ((rcx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_k = 6; bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = ((rax).wrapping_add(1)) as u64; rcx = ((rcx).wrapping_add(0x8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x4u64) as u64 as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { fl_k = 1; bb = 3; continue; } bb = 5; continue; }
            5 => { fl_k = 1; bb = 7; continue; }
            6 => { r8 = (rdx) as u64; crate::simd_ops::wu8(rbp.wrapping_add(0x77u64), (0x1u8) as u8); fl_k = 4; bb = 7; continue; }
            7 => { fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), (r14) as u64); rax = ((r12.wrapping_sub(0x1u64)) as u32) as u32 as u64; r15 = ((r13.wrapping_add(0x78u64)) as u64) as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; r15 = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { r8 } else { r15 }) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), (r15) as u64); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { fl_k = 3; bb = 11; continue; } bb = 8; continue; }
            8 => { rax = ((r13.wrapping_add(0xb0u64)) as u64) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_k = 3; bb = 9; continue; }
            9 => { r14 = (crate::simd_ops::ru64(rax)) as u64; fl_l = (r14) as u64 as u64; fl_r = (r14) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 13; continue; } bb = 10; continue; }
            10 => { rax = ((rax).wrapping_sub(0x8u64)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (0x1u64) as u64 as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((((fl_l as u64)).wrapping_sub((fl_r as u64))) as i64) >= 0) { fl_k = 1; bb = 9; continue; } fl_k = 1; bb = 11; continue; }
            11 => { r14 = (crate::simd_ops::ru64(r13.wrapping_add(0xf8u64))) as u64; fl_l = (r14) as u64 as u64; fl_r = (r14) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(r13.wrapping_add(0xe8u64))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x21u64), (rdx) as u64); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(0x2a0u64))) as u64) as u128; crate::simd_ops::wu64(rbp.wrapping_sub(0x1u64), ((xmm0 as u64)) as u64); rdx = (crate::simd_ops::ru32(rdx.wrapping_add(0x2a8u64))) as u32 as u64; bb = 15; continue; }
            13 => { rax = (crate::simd_ops::ru64(r13.wrapping_add(0xe8u64))) as u64; rdx = ((rbp.wrapping_add(0x7fu64)) as u64) as u64; rcx = (r14) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x21u64), (rbx) as u64); xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x2a0u64))) as u64) as u128; crate::simd_ops::wu64(rbp.wrapping_sub(0x1u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x2a8u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x7u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(r14)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x90u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0x7u64))) as u32 as u64; rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdi = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x3u64), ((rdi as u32)) as u32); fl_k = 3; bb = 16; continue; }
            14 => { r14 = (crate::simd_ops::ru64(r13.wrapping_add(0xe8u64))) as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x21u64), (rax) as u64); xmm0 = ((crate::simd_ops::ru64(rax.wrapping_add(0x2a0u64))) as u64) as u128; crate::simd_ops::wu64(rbp.wrapping_sub(0x1u64), ((xmm0 as u64)) as u64); rdx = (crate::simd_ops::ru32(rax.wrapping_add(0x2a8u64))) as u32 as u64; bb = 15; continue; }
            15 => { rdi = (crate::simd_ops::ru32(rbp.wrapping_add(0x3u64))) as u32 as u64; fl_k = 4; bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru32(P0)) as u32 as u64; rdx = ((((rdx as u32)) & (0xffffffe0u32))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x1u64), ((rax as u32)) as u32); rdx = ((((rdx as u32)) | (0x20u32))) as u32 as u64; rcx = ((rdx as u32)) as u32 as u64; rax = (((rdi as u8)) as u32) as u32 as u64; rcx = ((((rcx as u32)) & (0x3fu32))) as u32 as u64; rdx = ((((rdx as u32)) & (0xfffd003fu32))) as u32 as u64; rcx = ((((rcx as u32)) as i32).wrapping_mul(((rax as u32)) as i32) as u32) as u32 as u64; rax = ((r12 as u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0x1ff8u32))) as u32 as u64; rax = ((rax.wrapping_mul(8).wrapping_add(0xb0u64)) as u64) as u64; rax = ((rax).wrapping_add(r13)) as u64; rcx = ((((rcx as u32)) | (0xa000u32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(3u32)) as u32 as u64; rcx = ((((rcx as u32)) | ((rdx as u32)))) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x7u64), ((rcx as u32)) as u32); rsi = (crate::simd_ops::ru64(rax)) as u64; fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { fl_k = 4; bb = 25; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = ((rbp.wrapping_sub(0x29u64)) as u64) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x90u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = ((r12 as u32)) as u32 as u64; rdi = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_l = ((r12 as u32)) as u32 as u64; fl_r = (0x4u32) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 24; continue; } bb = 18; continue; }
            18 => { r15 = (crate::simd_ops::ru32(rbp.wrapping_add(0x6fu64))) as u32 as u64; rax = (r12) as u64; r12 = ((r13.wrapping_add(0xb0u64)) as u64) as u64; r12 = ((r12.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; bb = 19; continue; }
            19 => { rcx = (crate::simd_ops::ru64(r12)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { fl_k = 4; bb = 22; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x70u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { fl_k = 5; bb = 22; continue; } bb = 21; continue; }
            21 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 23; continue; } fl_k = 0; bb = 22; continue; }
            22 => { rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; r12 = ((r12).wrapping_add(0x8u64)) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = (0x4u32) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 19; continue; } bb = 23; continue; }
            23 => { r15 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x11u64))) as u64; r12 = (crate::simd_ops::ru32(rbp.wrapping_add(0x6fu64))) as u32 as u64; bb = 24; continue; }
            24 => { rcx = (r15) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (rsi) as u64; rcx = (0x5u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x19u64))) as u64; rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rsi = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu64(rax, (rbx) as u64); fl_k = 3; bb = 25; continue; }
            25 => { r9 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x21u64))) as u64; r8 = ((rbp.wrapping_add(0xfu64)) as u64) as u64; rdx = ((r12 as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(r9.wrapping_add(0x1d8u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0xfu64), (rbx) as u64); r9 = (crate::simd_ops::ru64(r9.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x1b0u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0xfu64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 45; continue; } bb = 26; continue; }
            26 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x21u64), (rbx) as u64); r8 = ((rbp.wrapping_sub(0x11u64)) as u64) as u64; crate::simd_ops::wu8(rbp.wrapping_sub(0x11u64), (0x0u8) as u8); rdx = ((rbp.wrapping_sub(0x21u64)) as u64) as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0xdu64), ((rbx as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x9u64), (0x1u32) as u32); crate::simd_ops::wu16(rbp.wrapping_sub(0x5u64), (0x100u16) as u16); crate::simd_ops::wu8(rbp.wrapping_sub(0x3u64), (0x0u8) as u8); rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x30u64))) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 53; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0xfu64))) as u64; r8 = ((rbp.wrapping_sub(0x11u64)) as u64) as u64; rdx = (rdx & !0xffu64) | (((0x3u8) as u8 as u64)); rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x30u64))) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 29; continue; } fl_k = 3; bb = 28; continue; }
            28 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x21u64))) as u64; rdx = ((P5) as u64) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 53; continue; }
            29 => { rcx = ((crate::simd_ops::ru8(rbp.wrapping_sub(0x9u64))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rbp.wrapping_sub(0x6u64))) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 31; continue; } bb = 30; continue; }
            30 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_sub(0x8u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; fl_k = 2; bb = 32; continue; }
            31 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; fl_k = 5; bb = 32; continue; }
            32 => { if (match fl_k { 2 => (((fl_l as u8)) != ((fl_r as u8))), 5 => ((((fl_l as u8)) & ((fl_r as u8))) != 0), _ => false }) { bb = 28; continue; } bb = 33; continue; }
            33 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_sub(0x5u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 28; continue; } bb = 34; continue; }
            34 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; rcx = (0x5u32) as u32 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 40; continue; } bb = 35; continue; }
            35 => { rdx = (0xe8u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 38; continue; } bb = 36; continue; }
            36 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rsi = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { fl_k = 4; bb = 28; continue; } bb = 37; continue; }
            37 => { rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r8 = (0xe8u32) as u32 as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((P9) as u64) as u64; crate::simd_ops::wu64(rsi, (rax) as u64); crate::simd_ops::wu8(rsi.wrapping_add(0xe0u64), (0x0u8) as u8); fl_l = (crate::simd_ops::lk_and_u32(rsi.wrapping_add(0xe0u64), (0xffu32) as u32)) as u32 as u64; fl_r = fl_l; fl_k = 3; bb = 42; continue; }
            38 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rsi = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { fl_k = 4; bb = 28; continue; } bb = 39; continue; }
            39 => { rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r8 = (0xe8u32) as u32 as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((P10) as u64) as u64; crate::simd_ops::wu8(rsi.wrapping_add(0xe0u64), (0x0u8) as u8); fl_l = (crate::simd_ops::lk_and_u32(rsi.wrapping_add(0xe0u64), (0xffu32) as u32)) as u32 as u64; fl_r = fl_l; crate::simd_ops::wu64(rsi, (rax) as u64); fl_k = 3; bb = 42; continue; }
            40 => { rdx = (0x168u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { fl_k = 4; bb = 28; continue; } bb = 41; continue; }
            41 => { rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r8 = (0x168u32) as u32 as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rsi = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { fl_k = 4; bb = 28; continue; } fl_k = 4; bb = 42; continue; }
            42 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = ((rbp.wrapping_sub(0x1u64)) as u64) as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x21u64))) as u64; r8 = ((rbp.wrapping_add(0xfu64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rcx) as u64); r9 = ((r12 as u32)) as u32 as u64; rcx = (rsi) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r13) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x58u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rsi) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 44; continue; } bb = 43; continue; }
            43 => { rax = (crate::simd_ops::ru64(rsi)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (rsi) as u64; rcx = (0x5u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 53; continue; }
            44 => { rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x19u64))) as u64; rdx = (r14) as u64; crate::simd_ops::wu64(rax, (rsi) as u64); rax = ((crate::simd_ops::ru16(rbp.wrapping_add(0x17u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rsi.wrapping_add(0xcau64), ((rax as u16)) as u16); rax = (crate::simd_ops::ru64(rsi)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = ((rbp.wrapping_add(0x6fu64)) as u64) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x90u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru32(rax)) as u32 as u64; bb = 46; continue; }
            45 => { rbx = (crate::simd_ops::ru32(rbp.wrapping_add(0x3u64))) as u32 as u64; bb = 46; continue; }
            46 => { rax = ((rbx as u32)) as u32 as u64; rax = ((((rax as u32)) ^ ((rdi as u32)))) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { fl_k = 5; bb = 49; continue; } bb = 47; continue; }
            47 => { rax = ((rbx as u32)) as u32 as u64; rax = ((((rax as u32)) ^ ((rdi as u32)))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0xf00u32) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { fl_k = 3; bb = 49; continue; } bb = 48; continue; }
            48 => { rax = ((rbx as u32)) as u32 as u64; rax = ((((rax as u32)) ^ ((rdi as u32)))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0xfffff000u32) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 52; continue; } fl_k = 3; bb = 49; continue; }
            49 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x77u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 51; continue; } bb = 50; continue; }
            50 => { rdx = ((r12.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = (r13) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 54; continue; }
            51 => { rcx = ((r13.wrapping_add(0x88u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((rbx as u32)) as u32 as u64; rcx = ((r13.wrapping_add(0x88u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P14) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(r13.wrapping_add(0x160u64), ((rbx as u32)) as u32); bb = 52; continue; }
            52 => { fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; r14 = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rsi } else { r14 }) as u64; bb = 53; continue; }
            53 => { rax = (crate::simd_ops::ru64(r15)) as u64; rdx = (r14) as u64; rcx = (r15) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 54; continue; }
            54 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xfu64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 57; continue; } bb = 55; continue; }
            55 => { rax = (0xffffffffu32) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 57; continue; } bb = 56; continue; }
            56 => { rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 57; continue; }
            57 => { rsp = ((rsp).wrapping_add(0xa0u64)) as u64; r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141603110(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x24c; const P1: u64 = 0xb4; const P2: u64 = 0x164; const P3: u64 = 0x10; const P4: u64 = 0x1434268d0; const P5: u64 = 0x18; const P6: u64 = 0x4; const P7: u64 = 0x156; const P8: u64 = 0x143426564; const P9: u64 = 0x142673708; const P10: u64 = 0x1426736fc; const P11: u64 = 0x142673700; const P12: u64 = 0x1426736ec; const P13: u64 = 0x1426736e8; const P14: u64 = 0x1426736f0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { xmm3 = ((crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32) as u128; r9 = rcx; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(P1)))).to_bits() as u128); fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 8; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(r8.wrapping_add(P3))) as u64; xmm2 = ((crate::simd_ops::ru32(P4)) as u32) as u128; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(r8.wrapping_add(P5))) as u64; xmm2 = ((crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32) as u128; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P7))) as u8 as u64; fl_r = (0x20u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(crate::simd_ops::ru32(rdx))).to_bits() as u128); bb = 4; continue; }
            4 => { xmm0 = (xmm3) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P8))).to_bits() as u128); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P9)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { xmm1 = 0u128; bb = 7; continue; }
            6 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P10))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(P11))).to_bits() as u128); rcx = (crate::simd_ops::cvt_f32_i64((f32::from_bits(xmm0 as u32)))) as u64; rax = ((rcx as u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0x7fffffu32))) as u32 as u64; rax = ((((rax as u32)) & (0xff800000u32))) as u32 as u64; fl_l = (((rcx as u32)).wrapping_add(0x3f800000u32)) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x10u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x8u64), ((rcx as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x8u64))) as u32) as u128; xmm1 = (xmm0) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(P12))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(crate::simd_ops::ru32(P13))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(crate::simd_ops::ru32(P14))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x10u64)))).to_bits() as u128); bb = 7; continue; }
            7 => { xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); crate::simd_ops::wu32(r9.wrapping_add(0x164u64), ((xmm3 as u32)) as u32); crate::simd_ops::wu32(r9.wrapping_add(0x168u64), ((xmm1 as u32)) as u32); bb = 8; continue; }
            8 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416031f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xf0; const P1: u64 = 0xe8; const P2: u64 = 0x1416006a0; const P3: u64 = 0x120; const P4: u64 = 0xf8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rbx = rcx;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((crate::simd_ops::ru8(rbx.wrapping_add(P3))) as u32) as u32 as u64;
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) != 0)) { rdx = (rdx & !0xffu64) | ((((((rdx as u8)) & (0xfbu8))) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(P3), ((rdx as u8)) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = 0;
    fl_l = 0; fl_r = 0;
    rcx = 0x4;
    fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; fl_r = rax;
    rax = (if (((fl_l as u64)) == ((fl_r as u64))) { (rcx as u32) } else { (rax as u32) }) as u32 as u64;
    rdx = (rdx & !0xffu64) | ((((((rdx as u8)) & (0xfbu8))) as u8 as u64));
    rax = (rax & !0xffu64) | ((((((rax as u8)) | ((rdx as u8)))) as u8 as u64));
    crate::simd_ops::wu8(rbx.wrapping_add(P3), ((rax as u8)) as u8);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141603260(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x141524320;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    r8 = 0x10;
    return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141603280(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x141524350;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    r9 = 0x10;
    return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416032b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14158c580; const P1: u64 = 0x14158d2f0; const P2: u64 = 0x141603630; const P3: u64 = 0x141604140; const P4: u64 = 0x1415aaf70; const P5: u64 = 0x1416034e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x48u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xe0u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r15 = rdx; r14 = rcx; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 1; continue; }
            1 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0xe0u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = rax.wrapping_add(0x20); bb = 4; continue; }
            3 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; bb = 4; continue; }
            4 => { rcx = r14.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rcx = r14; rax = 0x20; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu8(r14.wrapping_add(0xe0u64), (0x1u8) as u8); r12 = (r12 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 8; continue; }
            6 => { rax = 0x20; bb = 7; continue; }
            7 => { r12 = (r12 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; rbp = ((rcx.wrapping_add(rax)) as u64) as u64; rax = (crate::simd_ops::ru32(r15.wrapping_add(0x18u64))) as u32 as u64; rsi = (rsi & !0xffu64) | (((0x1u8) as u8 as u64)); rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 23; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r13); r13 = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            10 => { rdi = (crate::simd_ops::ru64(rbx)) as u64; r8 = rsp.wrapping_add(0x80); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rdx = rsp.wrapping_add(0x30); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r13 as u32)) as u32); rcx = rbp; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 11; continue; }
            11 => { fl_l = rdi; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 12; continue; }
            12 => { r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x20u64))) as u64; rcx = rax.wrapping_sub(0x18); fl_l = rcx; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 14; continue; }
            14 => { rdx = rax.wrapping_add(0x18); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x28u64))) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = rax.wrapping_sub(0x3); rax = ((r8.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru64(rdx)) as u64; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = rbp.wrapping_add(0x30); rdx = rdi; r8 = rsp.wrapping_add(0x88); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 17; continue; }
            17 => { fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { rsi = (rsi & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 20; continue; }
            19 => { rsi = (rsi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 20; continue; }
            20 => { rcx = (crate::simd_ops::ru32(r15.wrapping_add(0x18u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 21; continue; }
            21 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { fl_l = ((r12 as u8)) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 24; continue; }
            24 => { rsp = ((rsp).wrapping_add(0x48u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141603420(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x20; const P2: u64 = 0x8; const P3: u64 = 0x1; const P4: u64 = 0xffffffffffffffff;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); r10 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; r11 = 0; fl_l = 0; fl_r = 0; rsi = r8; rbx = rcx; fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { rbp = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; r9 = (crate::simd_ops::ru64(rdx)) as u64; bb = 2; continue; }
            2 => { r8 = ((r10 as u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_shr(1u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_add((r11 as u32))) as u32 as u64; rax = ((((r8 as u32)) as i32 as i64) as u64) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdi = ((rcx.wrapping_mul(8)) as u64) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(rbp))) as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(rbp).wrapping_add(0x8u64))) as u32 as u64; fl_l = rax; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 4; continue; }
            4 => { fl_l = rax; fl_r = r9; bb = 5; continue; }
            5 => { if (((fl_l as u64)) < ((fl_r as u64))) { bb = 9; continue; } bb = 6; continue; }
            6 => { if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = (rcx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 10; continue; }
            9 => { rcx = (rcx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 10; continue; }
            10 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; rax = ((r8.wrapping_add(P3)) as u32) as u32 as u64; rax = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (r11 as u32) } else { (rax as u32) }) as u32 as u64; r11 = ((rax as u32)) as u32 as u64; rax = ((r10.wrapping_add(P4)) as u32) as u32 as u64; rax = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (r10 as u32) } else { (rax as u32) }) as u32 as u64; r10 = ((rax as u32)) as u32 as u64; fl_l = (((r10 as u32)).wrapping_shr(1u32)) as u32 as u64; r10 = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 2; continue; } bb = 11; continue; }
            11 => { rax = ((((r11 as u32)) as i32 as i64) as u64) as u64; crate::simd_ops::wu8(rsi, (0x0u8) as u8); rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; return rax; }
            13 => { crate::simd_ops::wu8(rsi, (0x1u8) as u8); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_l = ((rax).wrapping_add(rdi)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 12; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416034e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14158c580; const P1: u64 = 0x14158d2f0; const P2: u64 = 0x1415e71a0; const P3: u64 = 0x14158c3d0; const P4: u64 = 0x141603710; const P5: u64 = 0x143931d18; const P6: u64 = 0x14392d458;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = 0; fl_l = 0; fl_r = 0; rsi = rcx; crate::simd_ops::wu32(rcx.wrapping_add(0xd8u64), ((rbp as u32)) as u32); fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xe0u64))) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = rcx.wrapping_add(0x20); bb = 3; continue; }
            2 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rdx = rax.wrapping_add(0x30); rcx = rsi.wrapping_add(0xc0); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0xe1u64))) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = rsi.wrapping_add(0x70); bb = 6; continue; }
            5 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rdx = rax.wrapping_add(0x30); rcx = rsi.wrapping_add(0xc0); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rsi.wrapping_add(0xc0); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(P5)) as u64; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(P6)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); bb = 9; continue; }
            9 => { fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0xe1u64))) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0xe0u64))) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 19; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu32(rbx.wrapping_add(0xd8u64), ((rbp as u32)) as u32); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0xe0u64))) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = rbx.wrapping_add(0x20); bb = 15; continue; }
            14 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            15 => { rdx = rax.wrapping_add(0x30); rcx = rbx.wrapping_add(0xc0); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0xe1u64))) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = rbx.wrapping_add(0x70); bb = 18; continue; }
            17 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 18; continue; }
            18 => { rdx = rax.wrapping_add(0x30); rcx = rbx.wrapping_add(0xc0); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0xc0); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 19; continue; }
            19 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 20; continue; }
            20 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 21; continue; }
            21 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141603630(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x141603fd0; const P2: u64 = 0x20; const P3: u64 = 0x10; const P4: u64 = 0x4c; const P5: u64 = 0x48; const P6: u64 = 0x30; const P7: u64 = 0x1415a5850; const P8: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rbx as u32)) as u32); rsi = rcx; rdx = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 1; continue; }
            1 => { r8 = ((rbx as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rax = ((r8 as u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(P2))) as u64; rdx = ((rcx.wrapping_mul(8)) as u64) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(rcx))) as u128; crate::simd_ops::wu128(rax.wrapping_add(rdx), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(rcx).wrapping_add(P3))) as u64) as u128; crate::simd_ops::wu64(rax.wrapping_add(rdx).wrapping_add(P3), ((xmm1 as u64)) as u64); fl_l = ((r8 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(P5), ((rbx as u32)) as u32); rbp = (crate::simd_ops::ru32(rdi.wrapping_add(P5))) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = ((rbp as u32)) as u32 as u64; rcx = ((rsi.wrapping_add(P6)) as u64) as u64; rdx = (((rdx as u32)).wrapping_sub((rax as u32))) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu32(rsi.wrapping_add(P5), ((rbp as u32)) as u32); fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(P5))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 9; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P8))) as u64; rdx = ((rbx.wrapping_mul(8)) as u64) as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P8))) as u64; rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(rax))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(rcx), rax); fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(P5))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 8; continue; } bb = 9; continue; }
            9 => { rax = 0x1; bb = 11; continue; }
            10 => { rax = 0x2; bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141603710(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x18; const P2: u64 = 0x143930f78; const P3: u64 = 0x1415f3790; const P4: u64 = 0x20; const P5: u64 = 0x141524330; const P6: u64 = 0x1415d4390; const P7: u64 = 0x8; const P8: u64 = 0x58; const P9: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbp = rcx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; rdx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru64(P2)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 4; continue; }
            4 => { rdx = 0xa0; rcx = 0x4; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { rcx = rax; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsi.wrapping_add(P4), rax); rcx = rdi; crate::simd_ops::wu64(rax.wrapping_add(P7), rsi); rax = (crate::simd_ops::ru64(rax)) as u64; rdx = (crate::simd_ops::ru64(rsi.wrapping_add(P8))) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P7))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rdi; rcx = 0x4; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi.wrapping_add(P4), r14); bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x18u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 9; continue; }
            9 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141603800(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x20; const P2: u64 = 0x8; const P3: u64 = 0x1; const P4: u64 = 0xffffffffffffffff;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x10u64)) as u64; xmm0 = (crate::simd_ops::ru128(rdx)) as u128; r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rdi = ((rcx.wrapping_add(P1)) as u64) as u64; rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu128(rsp, (xmm0) as u128); r11 = ((rsi as u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { r14 = (crate::simd_ops::ru64(rdi)) as u64; r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x8u64))) as u32 as u64; rdx = (crate::simd_ops::ru64(rsp)) as u64; bb = 2; continue; }
            2 => { r10 = ((r8 as u32)) as u32 as u64; r10 = (((r10 as u32)).wrapping_shr(1u32)) as u32 as u64; r10 = (((r10 as u32)).wrapping_add((r11 as u32))) as u32 as u64; rax = ((((r10 as u32)) as i32 as i64) as u64) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(rcx.wrapping_mul(8)))) as u64; rbx = ((r14.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rcx = (crate::simd_ops::ru32(r14.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P2))) as u32 as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 4; continue; }
            4 => { fl_l = rax; fl_r = rdx; bb = 5; continue; }
            5 => { if (((fl_l as u64)) < ((fl_r as u64))) { bb = 9; continue; } bb = 6; continue; }
            6 => { if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = (rcx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 10; continue; }
            9 => { rcx = (rcx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 10; continue; }
            10 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; rax = ((r10.wrapping_add(P3)) as u32) as u32 as u64; rax = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (r11 as u32) } else { (rax as u32) }) as u32 as u64; r11 = ((rax as u32)) as u32 as u64; rax = ((r8.wrapping_add(P4)) as u32) as u32 as u64; rax = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (r8 as u32) } else { (rax as u32) }) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; fl_l = (((r8 as u32)).wrapping_shr(1u32)) as u32 as u64; r8 = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 2; continue; } bb = 11; continue; }
            11 => { rcx = ((((r11 as u32)) as i32 as i64) as u64) as u64; rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = (crate::simd_ops::ru64(rdi)) as u64; rbx = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; bb = 12; continue; }
            12 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rsi = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { rbx } else { rsi }) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = rsi; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rsp = ((rsp).wrapping_add(0x10u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            13 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 12; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416038d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141603800; const P1: u64 = 0x10; const P2: u64 = 0x1434268d0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    rdx = rsp.wrapping_add(0x20);
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32) as u128; rsp = ((rsp).wrapping_add(0x38u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    xmm0 = ((crate::simd_ops::ru32(P2)) as u32) as u128;
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}
