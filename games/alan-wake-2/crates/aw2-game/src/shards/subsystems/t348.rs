#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn fn_142111670(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; fl_r = 0x1;
    if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { rax = 0xffffffff; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x1;
    if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { rax = 0xffffffff; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; fl_r = 0x3;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { xmm0 = ((crate::simd_ops::ru64(r8)) as u64) as u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64)).sqrt().to_bits() as u128); crate::simd_ops::wu64(rdx, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rdx.wrapping_add(P0), (0x3u32) as u32); rax = 0x1; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = 0xffffffff;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142111750(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x143426920;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; fl_r = 0x1;
    if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { rax = 0xffffffff; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x1;
    if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { rax = 0xffffffff; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; fl_r = 0x3;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { xmm0 = ((crate::simd_ops::ru64(r8)) as u64) as u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) * f64::from_bits(crate::simd_ops::ru64(P1))).to_bits() as u128); crate::simd_ops::wu64(rdx, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rdx.wrapping_add(P0), (0x3u32) as u32); rax = 0x1; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = 0xffffffff;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421117f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1426465a8; const P2: u64 = 0x18; const P3: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x68u64))) as u32 as u64; fl_r = 0x1;
    rbx = rdx;
    if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { rax = 0xffffffff; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x2;
    if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { rax = 0xffffffff; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; fl_r = 0x3;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { xmm0 = ((crate::simd_ops::ru64(r8)) as u64) as u128; rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64) as u128; crate::simd_ops::wu64(rbx, ((xmm1 as u64)) as u64); crate::simd_ops::wu64(rbx.wrapping_add(P2), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rbx.wrapping_add(P0), (0x3u32) as u32); rax = 0x2; crate::simd_ops::wu32(rbx.wrapping_add(P3), (0x3u32) as u32); rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = 0xffffffff;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142111860(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (((crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as i32 as i64) as u64) as u64; r10 = rdx; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 9; continue; } bb = 1; continue; }
            1 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 9; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 4; continue; }
            4 => { rdx = rcx; xmm0 = ((crate::simd_ops::ru64(rax)) as u64) as u128; xmm1 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((if f64::from_bits(xmm0 as u64) < f64::from_bits(crate::simd_ops::ru64(r8)) { f64::from_bits(xmm0 as u64) } else { f64::from_bits(crate::simd_ops::ru64(r8)) }).to_bits() as u128); fl_l = rcx; fl_r = (0x3u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rcx = 0x3; fl_l = ((rax).wrapping_add(0x18u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 7; continue; }
            7 => { xmm0 = ((crate::simd_ops::ru64(rax)) as u64) as u128; rax = ((rax).wrapping_add(0x18u64)) as u64; rcx = ((rcx).wrapping_add(1)) as u64; xmm1 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((if f64::from_bits(xmm0 as u64) < f64::from_bits(xmm1 as u64) { f64::from_bits(xmm0 as u64) } else { f64::from_bits(xmm1 as u64) }).to_bits() as u128); fl_l = rcx; fl_r = rdx; if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { bb = 6; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(r10, ((xmm1 as u64)) as u64); crate::simd_ops::wu32(r10.wrapping_add(P0), (0x3u32) as u32); rax = 0x1; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            9 => { rax = 0xffffffff; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421118e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (((crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as i32 as i64) as u64) as u64; r10 = rdx; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 9; continue; } bb = 1; continue; }
            1 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 9; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 4; continue; }
            4 => { rdx = rcx; xmm0 = ((crate::simd_ops::ru64(rax)) as u64) as u128; xmm1 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((if f64::from_bits(xmm0 as u64) > f64::from_bits(crate::simd_ops::ru64(r8)) { f64::from_bits(xmm0 as u64) } else { f64::from_bits(crate::simd_ops::ru64(r8)) }).to_bits() as u128); fl_l = rcx; fl_r = (0x3u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rcx = 0x3; fl_l = ((rax).wrapping_add(0x18u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 7; continue; }
            7 => { xmm0 = ((crate::simd_ops::ru64(rax)) as u64) as u128; rax = ((rax).wrapping_add(0x18u64)) as u64; rcx = ((rcx).wrapping_add(1)) as u64; xmm1 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((if f64::from_bits(xmm0 as u64) > f64::from_bits(xmm1 as u64) { f64::from_bits(xmm0 as u64) } else { f64::from_bits(xmm1 as u64) }).to_bits() as u128); fl_l = rcx; fl_r = rdx; if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { bb = 6; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(r10, ((xmm1 as u64)) as u64); crate::simd_ops::wu32(r10.wrapping_add(P0), (0x3u32) as u32); rax = 0x1; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            9 => { rax = 0xffffffff; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142111960(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x14183ad7c; const P2: u64 = 0x143426ac0; const P3: u64 = 0x1425476ca; const P4: u64 = 0x143426b90; const P5: u64 = 0x14183ad5e;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x78u64))) as u32 as u64; rbx = rdx; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 14; continue; } bb = 1; continue; }
            1 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 14; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 3; continue; }
            3 => { xmm0 = ((crate::simd_ops::ru64(r8)) as u64) as u128; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx, ((xmm0 as u64)) as u64); rax = 0x1; crate::simd_ops::wu32(rbx.wrapping_add(P0), (0x3u32) as u32); rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm7) as u128); xmm7 = ((crate::simd_ops::ru64(rax)) as u64) as u128; fl_l = ((xmm7 as u64)) as u64; fl_r = (crate::simd_ops::ru64(P2)) as u64; if (f64::from_bits(fl_l).is_nan() || f64::from_bits(fl_r).is_nan()) { bb = 9; continue; } bb = 7; continue; }
            7 => { if (f64::from_bits(fl_l) != f64::from_bits(fl_r) && !f64::from_bits(fl_l).is_nan() && !f64::from_bits(fl_r).is_nan()) { bb = 9; continue; } bb = 8; continue; }
            8 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            9 => { fl_l = ((xmm7 as u64)) as u64; fl_r = (crate::simd_ops::ru64(P4)) as u64; if (f64::from_bits(fl_l).is_nan() || f64::from_bits(fl_r).is_nan()) { bb = 13; continue; } bb = 10; continue; }
            10 => { if (f64::from_bits(fl_l) != f64::from_bits(fl_r) && !f64::from_bits(fl_l).is_nan() && !f64::from_bits(fl_r).is_nan()) { bb = 13; continue; } bb = 11; continue; }
            11 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rax = 0x1; crate::simd_ops::wu64(rbx, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rbx.wrapping_add(P0), (0x3u32) as u32); rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            13 => { crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm6) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm6 = (xmm0) as u128; xmm0 = (xmm7) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rax = 0x1; xmm1 = (xmm6 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm6 as u64) / f64::from_bits(xmm0 as u64)).to_bits() as u128); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu64(rbx, ((xmm1 as u64)) as u64); crate::simd_ops::wu32(rbx.wrapping_add(P0), (0x3u32) as u32); rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            14 => { rax = 0xffffffff; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142111ab0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x142646720;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x58u64))) as u32 as u64; fl_r = 0x2;
    rbx = rdx;
    if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { rax = 0xffffffff; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x1;
    if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { rax = 0xffffffff; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; fl_r = 0x3;
    if (((fl_l as u32)) != ((fl_r as u32))) { rax = 0xffffffff; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; fl_r = 0x3;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rdx = (crate::simd_ops::cvt_f64_i32((f64::from_bits(crate::simd_ops::ru64(rax))))) as u32 as u64; xmm0 = ((crate::simd_ops::ru64(r8)) as u64) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(P0), (0x3u32) as u32); rax = 0x1; crate::simd_ops::wu64(rbx, ((xmm0 as u64)) as u64); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = 0xffffffff;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142111b10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1426465b0; const P2: u64 = 0x18; const P3: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x58u64))) as u32 as u64; fl_r = 0x1;
    rbx = rdx;
    if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { rax = 0xffffffff; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x2;
    if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { rax = 0xffffffff; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; fl_r = 0x3;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { xmm0 = ((crate::simd_ops::ru64(r8)) as u64) as u128; rdx = rsp.wrapping_add(0x58); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(P0), (0x3u32) as u32); rax = 0x2; crate::simd_ops::wu64(rbx, ((xmm0 as u64)) as u64); xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((crate::simd_ops::ru32(rsp.wrapping_add(0x58u64))) as i32) as f64).to_bits() as u128); crate::simd_ops::wu64(rbx.wrapping_add(P2), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rbx.wrapping_add(P3), (0x3u32) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = 0xffffffff;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142111be0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; fl_r = 0x1;
    if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { rax = 0xffffffff; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x1;
    if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { rax = 0xffffffff; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; fl_r = 0x3;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { xmm0 = 0u128; xmm0 = crate::simd_ops::vroundsd(xmm0, crate::simd_ops::ru64(r8), 0x1u8); crate::simd_ops::wu64(rdx, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rdx.wrapping_add(P0), (0x3u32) as u32); rax = 0x1; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = 0xffffffff;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142111c70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x143426920;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; fl_r = 0x1;
    if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { rax = 0xffffffff; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x1;
    if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { rax = 0xffffffff; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; fl_r = 0x3;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { xmm0 = ((crate::simd_ops::ru64(r8)) as u64) as u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) / f64::from_bits(crate::simd_ops::ru64(P1))).to_bits() as u128); crate::simd_ops::wu64(rdx, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rdx.wrapping_add(P0), (0x3u32) as u32); rax = 0x1; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = 0xffffffff;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142111d50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; fl_r = 0x1;
    if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { rax = 0xffffffff; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x1;
    if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { rax = 0xffffffff; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; fl_r = 0x3;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { xmm0 = 0u128; xmm0 = crate::simd_ops::vroundsd(xmm0, crate::simd_ops::ru64(r8), 0x2u8); crate::simd_ops::wu64(rdx, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rdx.wrapping_add(P0), (0x3u32) as u32); rax = 0x1; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = 0xffffffff;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142111ee0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x143428730;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; fl_r = 0x1;
    if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { rax = 0xffffffff; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x1;
    if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { rax = 0xffffffff; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; fl_r = 0x3;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { xmm0 = ((crate::simd_ops::ru64(r8)) as u64) as u128; xmm0 = (xmm0) & (crate::simd_ops::ru128(P1)); crate::simd_ops::wu64(rdx, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rdx.wrapping_add(P0), (0x3u32) as u32); rax = 0x1; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = 0xffffffff;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142111f20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; fl_r = 0x1;
    if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { rax = 0xffffffff; return rax; }
    fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64;
    if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { rax = 0xffffffff; return rax; }
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if !((((fl_l as u32)) != ((fl_r as u32)))) { rax = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_l = 0; fl_r = ((rax as u32)) as u32 as u64; rax = (0u32.wrapping_sub(fl_r as u32)) as u32 as u64; rax = (if (((fl_l as u32)) < ((fl_r as u32))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64; rax = (0u32.wrapping_sub((rax as u32))) as u32 as u64; fl_l = (((rax as u32)).wrapping_sub(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; return rax; } rax = 0; fl_l = 0; fl_r = 0; return rax; }
    rax = 0xffffffff;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142111f50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1429868d0; const P2: u64 = 0x8; const P3: u64 = 0x1420ecd00; const P4: u64 = 0x20; const P5: u64 = 0x2; const P6: u64 = 0x1421158a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rcx; rcx = P1; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rcx }) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::cvt_f64_i64((f64::from_bits(crate::simd_ops::ru64(rcx))))) as u64; rax = 0; fl_l = 0; fl_r = 0; r8 = ((rax.wrapping_add(P4)) as u32) as u32 as u64; bb = 3; continue; }
            3 => { fl_l = ((((rdx as u32)) >> ((((rax as u32)) as u32 & 31))) & 1) as u64; if ((fl_l as u32) != 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 3; continue; } bb = 5; continue; }
            5 => { bb = 7; continue; }
            6 => { r8 = ((rax as u32)) as u32 as u64; bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rax = ((r8 as u32)) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); crate::simd_ops::wu64(rcx, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rcx.wrapping_add(P0), (0x3u32) as u32); rax = 0x1; crate::simd_ops::lk_add_u64(rbx.wrapping_add(P2), (0x18u64) as u64); rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            8 => { rdx = 0x1; rcx = rbx; r8 = ((rdx.wrapping_add(P5)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x142111fdd, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142111ff0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1429868d0; const P2: u64 = 0x8; const P3: u64 = 0x1420ecd00; const P4: u64 = 0x20; const P5: u64 = 0x2; const P6: u64 = 0x1421158a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rcx; rcx = P1; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rcx }) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::cvt_f64_i64((f64::from_bits(crate::simd_ops::ru64(rcx))))) as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = 0x1f; r8 = ((rcx.wrapping_add(P4)) as u32) as u32 as u64; bb = 3; continue; }
            3 => { fl_l = ((((rdx as u32)) >> ((((rax as u32)) as u32 & 31))) & 1) as u64; if ((fl_l as u32) != 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffffffff; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 3; continue; } bb = 5; continue; }
            5 => { bb = 7; continue; }
            6 => { r8 = ((rcx as u32)) as u32 as u64; bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rax = ((r8 as u32)) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); crate::simd_ops::wu64(rcx, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rcx.wrapping_add(P0), (0x3u32) as u32); rax = 0x1; crate::simd_ops::lk_add_u64(rbx.wrapping_add(P2), (0x18u64) as u64); rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            8 => { rdx = 0x1; rcx = rbx; r8 = ((rdx.wrapping_add(P5)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x142112083, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142112090(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1429868d0; const P1: u64 = 0x1420ecd00; const P2: u64 = 0x142112270; const P3: u64 = 0x1421158a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rbx = P0; rdi = rcx; rcx = rbx; fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; fl_r = rdx; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { crate::simd_ops::ru64(rdi.wrapping_add(0x10u64)) } else { rcx }) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(0x18u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rsi); rsi = (crate::simd_ops::cvt_f64_i64((f64::from_bits(crate::simd_ops::ru64(rcx))))) as u64; fl_l = rax; fl_r = rdx; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rbx }) as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = rsp.wrapping_add(0x20); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::cvt_f64_i64((f64::from_bits(crate::simd_ops::ru64(rbx))))) as u64; r8 = rsp.wrapping_add(0x50); rdx = 0x3; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rdx = 0xfffffffe; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x50u64))) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub(1)) as u32 as u64; r8 = (((rdx as u32)).wrapping_shl(((((rcx as u32)) as u32) & 31))) as u32 as u64; r8 = (!((r8 as u32))) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(0x10u64), (0x3u32) as u32); rcx = (((r8 as u32)).wrapping_shl(((((rax as u32)) as u32) & 31))) as u32 as u64; rbx = ((((rbx as u32)) & ((r8 as u32)))) as u32 as u64; rax = (((rbx as u32)).wrapping_shl(((((rax as u32)) as u32) & 31))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_l = ((!((rcx as u32))) & ((rsi as u32))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = ((((rax as u32)) | ((rdx as u32)))) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); crate::simd_ops::wu64(r9, ((xmm0 as u64)) as u64); crate::simd_ops::lk_add_u64(rdi.wrapping_add(0x8u64), (0x18u64) as u64); rax = 0x1; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { rdx = 0x2; rcx = rdi; r8 = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x14211218b, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { rdx = 0x1; rcx = rdi; r8 = ((rdx.wrapping_add(0x2u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x14211219d, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421121b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1429868d0; const P2: u64 = 0x8; const P3: u64 = 0x1420ecd00; const P4: u64 = 0x142112270; const P5: u64 = 0x2; const P6: u64 = 0x1421158a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = rcx; rcx = P1; fl_l = rax; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rcx }) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbx); r8 = rsp.wrapping_add(0x50); rbx = (crate::simd_ops::cvt_f64_i64((f64::from_bits(crate::simd_ops::ru64(rcx))))) as u64; rcx = rdi; rdx = 0x2; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; rdx = 0xfffffffe; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x50u64))) as u32 as u64; rax = (((rbx as u32)).wrapping_shr(((((rax as u32)) as u32) & 31))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rcx = (((rcx as u32)).wrapping_sub(1)) as u32 as u64; rdx = (((rdx as u32)).wrapping_shl(((((rcx as u32)) as u32) & 31))) as u32 as u64; fl_l = ((!((rdx as u32))) & ((rax as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; rax = ((rax as u32)) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); crate::simd_ops::wu64(r8, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(r8.wrapping_add(P0), (0x3u32) as u32); rax = 0x1; crate::simd_ops::lk_add_u64(rdi.wrapping_add(P2), (0x18u64) as u64); rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { rdx = 0x1; rcx = rdi; r8 = ((rdx.wrapping_add(P5)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x14211225c, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142112270(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142116a10; const P1: u64 = 0x142118e10; const P2: u64 = 0x1433e0f58; const P3: u64 = 0x142116bc0; const P4: u64 = 0x1433e0f18; const P5: u64 = 0x142116b30; const P6: u64 = 0x1433e0f40;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = r8; rbp = ((rdx as u32)) as u32 as u64; rbx = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbp.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = rbx; rdi = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) > 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x1; bb = 3; continue; }
            2 => { rdx = ((rbp.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 9; continue; } bb = 5; continue; }
            5 => { rcx = ((rax.wrapping_add(rdi)) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x20; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 8; continue; } bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu32(r14, ((rax as u32)) as u32); rax = ((rdi as u32)) as u32 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { r8 = P2; rdx = ((rbp as u32)) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1421122fb, rcx, rsp); return rax; }
            8 => { rdx = P4; rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14211230b, rcx, rsp); return rax; }
            9 => { r8 = P6; rcx = rbx; rdx = ((rbp.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14211231e, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142112330(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1429868d0; const P2: u64 = 0x8; const P3: u64 = 0x1420ecd00; const P4: u64 = 0x142112410; const P5: u64 = 0x1; const P6: u64 = 0x1421158a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut xmm0: u128 = 0; let mut xmm1: u128 = 0; let mut xmm2: u128 = 0; let mut xmm3: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rcx; rax = ((rax).wrapping_add(0x18u64)) as u64; rcx = P1; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rcx }) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::cvt_f64_i32((f64::from_bits(crate::simd_ops::ru64(rcx))))) as u32 as u64; rdx = (0u32.wrapping_sub((rdx as u32))) as u32 as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { rdx = 0x2; rcx = rbx; r8 = ((rdx.wrapping_add(P5)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x14211238d, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421123a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1429868d0; const P2: u64 = 0x8; const P3: u64 = 0x1420ecd00; const P4: u64 = 0x142112410; const P5: u64 = 0x1; const P6: u64 = 0x1421158a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut xmm0: u128 = 0; let mut xmm1: u128 = 0; let mut xmm2: u128 = 0; let mut xmm3: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rcx; rax = ((rax).wrapping_add(0x18u64)) as u64; rcx = P1; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rcx }) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::cvt_f64_i32((f64::from_bits(crate::simd_ops::ru64(rcx))))) as u32 as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { rdx = 0x2; rcx = rbx; r8 = ((rdx.wrapping_add(P5)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1421123fb, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142112410(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1429868d0; const P2: u64 = 0x8; const P3: u64 = 0x1420ecd00; const P4: u64 = 0x2; const P5: u64 = 0x1421158a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rcx; rcx = P1; rdi = ((rdx as u32)) as u32 as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; r8 = ((rbx.wrapping_add(P2)) as u64) as u64; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rcx }) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { r8 = ((rbx.wrapping_add(P2)) as u64) as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::cvt_f64_i64((f64::from_bits(crate::simd_ops::ru64(rcx))))) as u64; fl_l = ((((rdi as u32)) & (0x1fu32))) as u32 as u64; fl_r = fl_l; rdi = ((fl_l as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (((rdi as u8)) as u32) as u32 as u64; r8 = ((rbx.wrapping_add(P2)) as u64) as u64; rax = (((rax as u32)).rotate_left(((rcx as u32) & 31))) as u32 as u64; bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = ((rax as u32)) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); crate::simd_ops::wu64(rcx, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rcx.wrapping_add(P0), (0x3u32) as u32); rax = 0x1; crate::simd_ops::lk_add_u64(r8, (0x18u64) as u64); rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { rdx = 0x1; rcx = rbx; r8 = ((rdx.wrapping_add(P4)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1421124af, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421124c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1429868d0; const P1: u64 = 0x1420ecd00; const P2: u64 = 0x1421158a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rdi = P0; rbx = rcx; rcx = rdi; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_r = rdx; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { crate::simd_ops::ru64(rbx.wrapping_add(0x10u64)) } else { rcx }) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(0x18u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rsi); rsi = (crate::simd_ops::cvt_f64_i64((f64::from_bits(crate::simd_ops::ru64(rcx))))) as u64; fl_l = rax; fl_r = rdx; rdi = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdi }) as u64; fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = rsp.wrapping_add(0x20); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::cvt_f64_i32((f64::from_bits(crate::simd_ops::ru64(rdi))))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x20; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = 0xffffffff; bb = 14; continue; }
            9 => { rax = 0xffffffff; rax = (((rax as u32)).wrapping_shr(((((rdx as u32)) as u32) & 31))) as u32 as u64; rax = (!((rax as u32))) as u32 as u64; rcx = (((rsi as u32)).wrapping_shr(((((rdx as u32)) as u32) & 31))) as u32 as u64; fl_l = ((((rax as u32)) | ((rcx as u32)))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; bb = 14; continue; }
            10 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = 0; fl_r = ((rcx as u32)) as u32 as u64; rcx = (0u32.wrapping_sub(fl_r as u32)) as u32 as u64; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) >= 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { r8 = 0; fl_l = 0; fl_r = 0; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x20; rax = (((rsi as u32)).wrapping_shr(((((rdx as u32)) as u32) & 31))) as u32 as u64; bb = 13; continue; }
            12 => { r8 = 0; fl_l = 0; fl_r = 0; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x20; rax = (((rsi as u32)).wrapping_shl(((((rcx as u32)) as u32) & 31))) as u32 as u64; bb = 13; continue; }
            13 => { r8 = (if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { (rax as u32) } else { (r8 as u32) }) as u32 as u64; rax = ((r8 as u32)) as u32 as u64; bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); crate::simd_ops::wu64(rcx, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), (0x3u32) as u32); rax = 0x1; crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0x18u64) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            15 => { rdx = 0x2; rcx = rbx; r8 = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1421125d1, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            16 => { rdx = 0x1; rcx = rbx; r8 = ((rdx.wrapping_add(0x2u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1421125e3, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421125f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1429868d0; const P2: u64 = 0x8; const P3: u64 = 0x1420ecd00; const P4: u64 = 0x1; const P5: u64 = 0x1421158a0; const P6: u64 = 0x2;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rsi = P1; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = rax; fl_r = rdx; rbx = rcx; rcx = rsi; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rcx }) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); rdi = (crate::simd_ops::cvt_f64_i32((f64::from_bits(crate::simd_ops::ru64(rcx))))) as u32 as u64; rdi = (0u32.wrapping_sub((rdi as u32))) as u32 as u64; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_r = rdx; rsi = (if (((fl_l as u64)) < ((fl_r as u64))) { crate::simd_ops::ru64(rbx.wrapping_add(P0)) } else { rsi }) as u64; fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = rsp.wrapping_add(0x20); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; bb = 6; continue; }
            6 => { rax = (crate::simd_ops::cvt_f64_i64((f64::from_bits(crate::simd_ops::ru64(rsi))))) as u64; rcx = 0; fl_l = 0; fl_r = 0; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = 0; fl_r = ((rdi as u32)) as u32 as u64; rdi = (0u32.wrapping_sub(fl_r as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(((((rdi as u32)) as u32) & 31))) as u32 as u64; bb = 9; continue; }
            8 => { rax = (((rax as u32)).wrapping_shl(((((rdi as u32)) as u32) & 31))) as u32 as u64; bb = 9; continue; }
            9 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x20; crate::simd_ops::wu32(rdx.wrapping_add(P0), (0x3u32) as u32); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rcx = (if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); crate::simd_ops::wu64(rdx, ((xmm0 as u64)) as u64); crate::simd_ops::lk_add_u64(rbx.wrapping_add(P2), (0x18u64) as u64); rax = 0x1; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            10 => { rdx = 0x2; rcx = rbx; r8 = ((rdx.wrapping_add(P4)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1421126cd, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            11 => { rdx = 0x1; rcx = rbx; r8 = ((rdx.wrapping_add(P6)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1421126df, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421126f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1429868d0; const P2: u64 = 0x8; const P3: u64 = 0x1420ecd00; const P4: u64 = 0x2; const P5: u64 = 0x1421158a0; const P6: u64 = 0x1;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = P1; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = rax; fl_r = rdx; rbx = rcx; rcx = rdi; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rcx }) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_r = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rsi); rdi = (if (((fl_l as u64)) < ((fl_r as u64))) { crate::simd_ops::ru64(rbx.wrapping_add(P0)) } else { rdi }) as u64; rsi = (crate::simd_ops::cvt_f64_i32((f64::from_bits(crate::simd_ops::ru64(rcx))))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = rsp.wrapping_add(0x20); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::cvt_f64_i64((f64::from_bits(crate::simd_ops::ru64(rdi))))) as u64; rdx = 0; fl_l = 0; fl_r = 0; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { fl_l = 0; fl_r = ((rsi as u32)) as u32 as u64; rsi = (0u32.wrapping_sub(fl_r as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(((((rsi as u32)) as u32) & 31))) as u32 as u64; bb = 8; continue; }
            7 => { rax = (((rax as u32)).wrapping_shl(((((rsi as u32)) as u32) & 31))) as u32 as u64; bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x20; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdx = (if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { (rax as u32) } else { (rdx as u32) }) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); crate::simd_ops::wu64(rcx, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rcx.wrapping_add(P0), (0x3u32) as u32); rax = 0x1; crate::simd_ops::lk_add_u64(rbx.wrapping_add(P2), (0x18u64) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            9 => { rdx = 0x1; rcx = rbx; r8 = ((rdx.wrapping_add(P4)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1421127cb, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            10 => { rdx = 0x2; rcx = rbx; r8 = ((rdx.wrapping_add(P6)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1421127dd, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421127f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    r9 = rcx;
    fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64;
    if !((((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0)) { fl_l = 0; fl_r = ((r8 as u32)) as u32 as u64; r8 = (0u32.wrapping_sub(fl_r as u32)) as u32 as u64; rax = (((rdx as u32)).wrapping_shr(((((r8 as u32)) as u32) & 31))) as u32 as u64; } else { rax = (((rdx as u32)).wrapping_shl(((((r8 as u32)) as u32) & 31))) as u32 as u64; }
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x20;
    xmm0 = 0u128;
    rdx = (if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { (rax as u32) } else { (rdx as u32) }) as u32 as u64;
    rax = ((rdx as u32)) as u32 as u64;
    xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128);
    crate::simd_ops::wu64(rcx, ((xmm0 as u64)) as u64);
    crate::simd_ops::wu32(rcx.wrapping_add(P1), (0x3u32) as u32);
    rax = 0x1;
    fl_l = (crate::simd_ops::lk_add_u64(r9.wrapping_add(P0), (0x18u64) as u64)) as u64; fl_r = fl_l;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142112840(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1429868d0; const P2: u64 = 0x8; const P3: u64 = 0x1420ecd00; const P4: u64 = 0x2; const P5: u64 = 0x1421158a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rcx; rcx = P1; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rcx }) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::cvt_f64_i64((f64::from_bits(crate::simd_ops::ru64(rcx))))) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rax = (!((rax as u32))) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); rax = 0x1; crate::simd_ops::wu64(rcx, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rcx.wrapping_add(P0), (0x3u32) as u32); crate::simd_ops::lk_add_u64(rbx.wrapping_add(P2), (0x18u64) as u64); rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { rdx = 0x1; rcx = rbx; r8 = ((rdx.wrapping_add(P4)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1421128b4, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421128c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x142115950;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(rcx.wrapping_add(P1)))) as u64; rdi = 0; fl_l = 0; fl_r = 0; let prod = (rax as i64 as i128).wrapping_mul((rdx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rbx = 0x1; rsi = rcx; rbp = rdx; rbp = ((((rbp) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rbp; rax = ((rax).wrapping_shr(63u32)) as u64; rbp = ((rbp).wrapping_add(rax)) as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 3; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rdx = ((rbx as u32)) as u32 as u64; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((((rdi as u32)) ^ ((rax as u32)))) as u32 as u64; rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64; rax = 0x1; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rcx = ((rdi as u32)) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rcx) as i64) as f64).to_bits() as u128); crate::simd_ops::wu64(rdx, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rdx.wrapping_add(P1), (0x3u32) as u32); crate::simd_ops::lk_add_u64(rsi.wrapping_add(P0), (0x18u64) as u64); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142112970(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x142115950;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(rcx.wrapping_add(P1)))) as u64; rdi = 0; fl_l = 0; fl_r = 0; let prod = (rax as i64 as i128).wrapping_mul((rdx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rbx = 0x1; rsi = rcx; rbp = rdx; rbp = ((((rbp) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rbp; rax = ((rax).wrapping_shr(63u32)) as u64; rbp = ((rbp).wrapping_add(rax)) as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 3; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rdx = ((rbx as u32)) as u32 as u64; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((((rdi as u32)) | ((rax as u32)))) as u32 as u64; rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64; rax = 0x1; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rcx = ((rdi as u32)) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rcx) as i64) as f64).to_bits() as u128); crate::simd_ops::wu64(rdx, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rdx.wrapping_add(P1), (0x3u32) as u32); crate::simd_ops::lk_add_u64(rsi.wrapping_add(P0), (0x18u64) as u64); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142112a20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142112aa0; const P1: u64 = 0x8; const P2: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    r8 = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    rax = 0x1;
    rdx = (rdx & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { 1u8 } else { 0u8 }) as u8 as u64));
    crate::simd_ops::wu32(r8, ((rdx as u32)) as u32);
    crate::simd_ops::wu32(r8.wrapping_add(P2), (0x1u32) as u32);
    crate::simd_ops::lk_add_u64(rbx.wrapping_add(P1), (0x18u64) as u64);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142112a60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142112aa0; const P1: u64 = 0x8; const P2: u64 = 0x10;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    rax = ((rax as u32)) as u32 as u64;
    xmm0 = 0u128;
    xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128);
    crate::simd_ops::wu64(rdx, ((xmm0 as u64)) as u64);
    crate::simd_ops::wu32(rdx.wrapping_add(P2), (0x3u32) as u32);
    rax = 0x1;
    crate::simd_ops::lk_add_u64(rbx.wrapping_add(P1), (0x18u64) as u64);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142112aa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x142115950;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(rcx.wrapping_add(P1)))) as u64; rbx = 0x1; let prod = (rax as i64 as i128).wrapping_mul((rdx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rbp = rcx; rdi = 0xffffffff; rsi = rdx; rsi = ((((rsi) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rsi; rax = ((rax).wrapping_shr(63u32)) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 3; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rdx = ((rbx as u32)) as u32 as u64; rcx = rbp; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdi = ((((rdi as u32)) & ((rax as u32)))) as u32 as u64; rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = ((rdi as u32)) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142112da0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142118cd0; const P1: u64 = 0x8; const P2: u64 = 0x142103330; const P3: u64 = 0xffffffffffffffd0; const P4: u64 = 0x1420ec690;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rax = r9; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (0x0u64) as u64); rbx = r8; rdi = rdx; rdx = rax; r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rsi = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r9 = 0x1; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (0x0u64) as u64); r8 = rdi; rdx = rbx; rcx = rsi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsi.wrapping_add(P1))) as u64; r8 = (0xffffffffffffffffu64) as u64; bb = 1; continue; }
            1 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(r8))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rdx = rdi; rcx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r9 = (crate::simd_ops::ru64(rsi.wrapping_add(P1))) as u64; r8 = rsp.wrapping_add(0x30); r9 = ((r9).wrapping_sub(0x18u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rdx = ((rbx.wrapping_add(P3)) as u64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x5u32) as u32); rcx = rsi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_add_u64(rsi.wrapping_add(P1), (0xffffffffffffffe8u64) as u64); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142112e60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1429868d0; const P2: u64 = 0x8; const P3: u64 = 0x142117870; const P4: u64 = 0x142119080; const P5: u64 = 0x1421184a0; const P6: u64 = 0x1433e1060; const P7: u64 = 0x1421168e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r8 = P1; rbx = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = rdx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 10; continue; } bb = 3; continue; }
            3 => { fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = r8; bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x81; rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r8 = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = 0xfffffffe; rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = 0x1; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            9 => { rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x81; rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = 0x1; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            10 => { r8 = P6; rdx = 0x1; rcx = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x142112f09, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142112f10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x1429868d0; const P3: u64 = 0x1; const P4: u64 = 0x142115b50; const P5: u64 = 0x1433e0168; const P6: u64 = 0x142116b30;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdx = P2; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { r8 = 0; fl_l = 0; fl_r = 0; rdx = ((r8.wrapping_add(P3)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = 0x1; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            4 => { r8 = 0x1; rdx = P5; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x142112f58, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142112f60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14210d8d0; const P1: u64 = 0x1429868d0; const P2: u64 = 0x142113140; const P3: u64 = 0x14210dbb0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x28u64))) as u64; rbx = rcx; rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rcx.wrapping_add(0x8u64)))) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = (0x18u64) as u64; if ((((fl_l as u64)) as i64) > (((fl_r as u64)) as i64)) { bb = 6; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0x48u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = (((rdx as u32)).wrapping_add((rdx as u32))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; bb = 5; continue; }
            4 => { fl_l = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rdx = ((fl_l as u32)) as u32 as u64; bb = 5; continue; }
            5 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu32(rax, (0x1u32) as u32); crate::simd_ops::wu32(rax.wrapping_add(0x10u64), (0x1u32) as u32); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0x18u64) as u64); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x1u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rbx); bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = P1; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_r = rcx; rax = (0x2aaaaaaaaaaaaaabu64) as u64; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { crate::simd_ops::ru64(rbx.wrapping_add(0x10u64)) } else { rdx }) as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_sub(0x18u64))) as u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = ((rcx).wrapping_sub(0x18u64)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rcx); rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rbx.wrapping_add(0x10u64)))) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rax = ((rax).wrapping_add(rdx)) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            11 => { fl_l = rax; fl_r = (0x48u64) as u64; if ((((fl_l as u64)) as i64) > (((fl_r as u64)) as i64)) { bb = 16; continue; } bb = 12; continue; }
            12 => { rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0x48u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x3; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = (((rdx as u32)).wrapping_add((rdx as u32))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; bb = 15; continue; }
            14 => { fl_l = (((rdx as u32)).wrapping_add(0x3u32)) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; bb = 15; continue; }
            15 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_add(0x48u64)) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 18; continue; } bb = 17; continue; }
            17 => { crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu32(rax, (0x0u32) as u32); crate::simd_ops::wu32(rax.wrapping_add(0x10u64), (0x1u32) as u32); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0x18u64) as u64); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x1u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rbx); bb = 20; continue; }
            20 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = P1; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_r = rcx; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { crate::simd_ops::ru64(rbx.wrapping_add(0x10u64)) } else { rdx }) as u64; xmm0 = (crate::simd_ops::ru128(rdx)) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0x18u64) as u64); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x1u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rbx); bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = P2; xmm0 = (crate::simd_ops::ru128(rax.wrapping_sub(0x48u64))) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_sub(0x38u64))) as u64) as u128; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = ((rcx).wrapping_add(0x18u64)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rcx); r8 = rcx.wrapping_sub(0x30); rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rbx.wrapping_add(0x30u64)))) as u64; rcx = ((rcx).wrapping_sub(0x48u64)) as u64; r9 = r8; r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(rbx.wrapping_add(0x30u64)))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0x2; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142113160(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1429868d0; const P1: u64 = 0x1421137a0; const P2: u64 = 0x14210dbb0; const P3: u64 = 0x14210d8d0; const P4: u64 = 0x1421158a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rbx = rcx; rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 27; continue; } bb = 1; continue; }
            1 => { rdi = P0; fl_l = rax; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 27; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x7; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 27; continue; } bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x1u64))) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x1u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rbx); bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = rdi; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_r = rdx; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { crate::simd_ops::ru64(rbx.wrapping_add(0x10u64)) } else { rcx }) as u64; xmm0 = (crate::simd_ops::ru128(rcx)) as u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0x18u64) as u64); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x1u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rbx); bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = rdi; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = rax; fl_r = rdx; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rcx }) as u64; xmm0 = (crate::simd_ops::ru128(rcx)) as u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0x18u64) as u64); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x1u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rbx); bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = rdi; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_r = rcx; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { crate::simd_ops::ru64(rbx.wrapping_add(0x10u64)) } else { rdx }) as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_sub(0x18u64))) as u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); crate::simd_ops::lk_sub_u64(rbx.wrapping_add(0x8u64), (0x18u64) as u64); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x1u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rbx); bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = rdi; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = rax; fl_r = rcx; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_sub(0x18u64))) as u128; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; crate::simd_ops::wu128(rdx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; rdx = P1; crate::simd_ops::lk_sub_u64(rbx.wrapping_add(0x8u64), (0x18u64) as u64); crate::simd_ops::lk_or_u32(rax.wrapping_add(0x24u64), (0x2u32) as u32); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; crate::simd_ops::lk_add_u16(rbx.wrapping_add(0x52u64), (1) as u16); r8 = rcx.wrapping_add(0x18); rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rbx.wrapping_add(0x30u64)))) as u64; r9 = r8; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(rbx.wrapping_add(0x30u64)))) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rsi = ((rax as u32)) as u32 as u64; rax = 0xffff; crate::simd_ops::lk_add_u16(rbx.wrapping_add(0x52u64), ((rax as u16)) as u16); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rcx); bb = 13; continue; }
            13 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 17; continue; } bb = 14; continue; }
            14 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x3u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 16; continue; } bb = 15; continue; }
            15 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x6u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = 0xffffffff; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            17 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rbx.wrapping_add(0x8u64)))) as u64; fl_l = rax; fl_r = (0x18u64) as u64; if ((((fl_l as u64)) as i64) > (((fl_r as u64)) as i64)) { bb = 22; continue; } bb = 18; continue; }
            18 => { rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32 as u64; rcx = rbx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 20; continue; } bb = 19; continue; }
            19 => { fl_l = (((rdx as u32)).wrapping_add((rdx as u32))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; bb = 21; continue; }
            20 => { fl_l = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rdx = ((fl_l as u32)) as u32 as u64; bb = 21; continue; }
            21 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 22; continue; }
            22 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 24; continue; } bb = 23; continue; }
            23 => { crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); bb = 24; continue; }
            24 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), (0x1u32) as u32); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0x18u64) as u64); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 26; continue; } bb = 25; continue; }
            25 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x1u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rbx); bb = 26; continue; }
            26 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_r = rcx; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (if (((fl_l as u64)) < ((fl_r as u64))) { crate::simd_ops::ru64(rbx.wrapping_add(0x10u64)) } else { rdi }) as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_sub(0x18u64))) as u128; crate::simd_ops::wu128(rdi, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64) as u128; crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = ((rcx).wrapping_add(0xffffffffffffffe8u64)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rcx); rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rbx.wrapping_add(0x10u64)))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rax = ((rax).wrapping_add(rdx)) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            27 => { rdx = 0x2; r8 = ((rdx.wrapping_add(0x5u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x14211340a, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142113420(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14210d8d0; const P1: u64 = 0x1429868d0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x28u64))) as u64; rbx = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 14; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = (0x18u64) as u64; if ((((fl_l as u64)) as i64) > (((fl_r as u64)) as i64)) { bb = 6; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32 as u64; rcx = rbx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = (((rdx as u32)).wrapping_add((rdx as u32))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; bb = 5; continue; }
            4 => { fl_l = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rdx = ((fl_l as u32)) as u32 as u64; bb = 5; continue; }
            5 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu32(rax, (0x1u32) as u32); crate::simd_ops::wu32(rax.wrapping_add(0x10u64), (0x1u32) as u32); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0x18u64) as u64); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x1u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rbx); bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = P1; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_r = rcx; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { crate::simd_ops::ru64(rbx.wrapping_add(0x10u64)) } else { rdx }) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 13; continue; } bb = 11; continue; }
            11 => { rax = rcx.wrapping_sub(0x18); bb = 12; continue; }
            12 => { xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rcx = ((rcx).wrapping_sub(0x18u64)) as u64; rax = rax.wrapping_sub(0x18); fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 12; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rbx.wrapping_add(0x10u64)))) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rax = ((rax).wrapping_add(rdx)) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            14 => { fl_l = rax; fl_r = (0x18u64) as u64; if ((((fl_l as u64)) as i64) > (((fl_r as u64)) as i64)) { bb = 19; continue; } bb = 15; continue; }
            15 => { rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32 as u64; rcx = rbx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 17; continue; } bb = 16; continue; }
            16 => { fl_l = (((rdx as u32)).wrapping_add((rdx as u32))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; bb = 18; continue; }
            17 => { fl_l = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rdx = ((fl_l as u32)) as u32 as u64; bb = 18; continue; }
            18 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 19; continue; }
            19 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 21; continue; } bb = 20; continue; }
            20 => { crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu32(rax, (0x0u32) as u32); crate::simd_ops::wu32(rax.wrapping_add(0x10u64), (0x1u32) as u32); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0x18u64) as u64); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x1u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rbx); bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = rax.wrapping_sub(0x30); fl_l = rax; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 26; continue; } bb = 24; continue; }
            24 => { rcx = rax.wrapping_sub(0x18); bb = 25; continue; }
            25 => { xmm0 = (crate::simd_ops::ru128(rcx)) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rax = ((rax).wrapping_sub(0x18u64)) as u64; rcx = rcx.wrapping_sub(0x18); fl_l = rax; fl_r = rdx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 25; continue; } bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rax = 0x2; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421135f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1429868d0; const P1: u64 = 0x1421137a0; const P2: u64 = 0x14210dbb0; const P3: u64 = 0x14210d8d0; const P4: u64 = 0x1433e0168; const P5: u64 = 0x142116b30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rbx = rcx; fl_l = r8; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 23; continue; } bb = 1; continue; }
            1 => { rdi = P0; fl_l = r8; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 23; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(r8.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 23; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; rdx = P1; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); r9 = r8; rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); crate::simd_ops::lk_or_u32(rax.wrapping_add(0x24u64), (0x2u32) as u32); r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(rcx.wrapping_add(0x30u64)))) as u64; crate::simd_ops::lk_add_u16(rcx.wrapping_add(0x52u64), (1) as u16); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rsi = ((rax as u32)) as u32 as u64; rax = 0xffff; crate::simd_ops::lk_add_u16(rbx.wrapping_add(0x52u64), ((rax as u16)) as u16); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rcx); bb = 5; continue; }
            5 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x3u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x6u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rax = 0xffffffff; bb = 9; continue; }
            9 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            10 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rbx.wrapping_add(0x8u64)))) as u64; fl_l = rax; fl_r = (0x18u64) as u64; if ((((fl_l as u64)) as i64) > (((fl_r as u64)) as i64)) { bb = 15; continue; } bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32 as u64; rcx = rbx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 13; continue; } bb = 12; continue; }
            12 => { fl_l = (((rdx as u32)).wrapping_add((rdx as u32))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; bb = 14; continue; }
            13 => { fl_l = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rdx = ((fl_l as u32)) as u32 as u64; bb = 14; continue; }
            14 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 17; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; rbp = (rbp & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu32(rax, ((rbp as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(0x10u64), (0x1u32) as u32); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0x18u64) as u64); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x1u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rbx); bb = 19; continue; }
            19 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_r = rcx; rdi = (if (((fl_l as u64)) < ((fl_r as u64))) { crate::simd_ops::ru64(rbx.wrapping_add(0x10u64)) } else { rdi }) as u64; fl_l = rcx; fl_r = rdi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 22; continue; } bb = 20; continue; }
            20 => { rax = rcx.wrapping_sub(0x18); bb = 21; continue; }
            21 => { xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rcx = ((rcx).wrapping_sub(0x18u64)) as u64; rax = rax.wrapping_sub(0x18); fl_l = rcx; fl_r = rdi; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 21; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rdi, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rbx.wrapping_add(0x10u64)))) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; fl_l = ((rax).wrapping_add(rdx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 9; continue; }
            23 => { r8 = 0x1; rdx = P4; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x142113791, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421137a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14210dd80;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    r8 = 0xffffffff;
    return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421137c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1429868d0; const P1: u64 = 0x1420ecd00; const P2: u64 = 0x1433dfd40; const P3: u64 = 0x142116bc0; const P4: u64 = 0x1421158a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rdx = r8; rdx = ((rdx).wrapping_sub(r9)) as u64; rdi = rcx; let prod = (rax as i64 as i128).wrapping_mul((rdx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rcx = P0; rbx = rdx; rbx = ((((rbx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rbx; rax = ((rax).wrapping_shr(63u32)) as u64; rbx = ((rbx).wrapping_add(rax)) as u64; fl_l = r9; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 1; continue; }
            1 => { fl_l = r9; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(r9.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x5; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(r9)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x18u64))) as u8 as u64; fl_r = (0x23u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = ((rbx.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x10u64), (0x3u32) as u32); xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | (((((rax as u32)) as i32) as f64).to_bits() as u128); crate::simd_ops::wu64(r8, ((xmm0 as u64)) as u64); crate::simd_ops::lk_add_u64(rdi.wrapping_add(0x8u64), (0x18u64) as u64); rax = 0x1; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { fl_l = r9; fl_r = r8; bb = 6; continue; }
            6 => { rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { r9 } else { rcx }) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::cvt_f64_i32((f64::from_bits(crate::simd_ops::ru64(rcx))))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = (((rax as u32)).wrapping_add((rbx as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; bb = 11; continue; }
            10 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; rax = (if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { (rbx as u32) } else { (rax as u32) }) as u32 as u64; bb = 11; continue; }
            11 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 13; continue; } bb = 12; continue; }
            12 => { rbx = (((rbx as u32)).wrapping_sub((rax as u32))) as u32 as u64; rax = ((rbx as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            13 => { r8 = P2; rdx = 0x1; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x14211389f, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            14 => { rdx = 0x1; rcx = rdi; r8 = ((rdx.wrapping_add(0x2u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1421138b1, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421138c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x1429868d0; const P3: u64 = 0x1433e0168; const P4: u64 = 0x142116b30; const P5: u64 = 0x1433e1070; const P6: u64 = 0x2; const P7: u64 = 0x142116420; const P8: u64 = 0x1431edc44;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 7; continue; } bb = 1; continue; }
            1 => { rdx = P2; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 4; continue; }
            4 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rcx = ((rcx).wrapping_sub(rax)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rax = ((rax).wrapping_add(rdx)) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { r8 = 0x1; rdx = P3; rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x142113932, rcx, rsp); return rax; }
            8 => { r9 = 0; fl_l = 0; fl_r = 0; r8 = P5; rcx = rbx; rdx = ((r9.wrapping_add(P6)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); r8 = rax; rdx = P8; rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14211395b, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142113970(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x1429868d0; const P3: u64 = 0x1; const P4: u64 = 0x18; const P5: u64 = 0x30; const P6: u64 = 0x68; const P7: u64 = 0x142116d40; const P8: u64 = 0x5; const P9: u64 = 0x1421158a0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r10 = rcx; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 9; continue; } bb = 1; continue; }
            1 => { rbx = P2; fl_l = rax; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; fl_r = 0x6; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P3))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64; r11 = ((r10.wrapping_add(P1)) as u64) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(r10.wrapping_add(P3), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(P5))) as u64; crate::simd_ops::wu64(r10.wrapping_add(P6), rax); crate::simd_ops::wu64(rcx.wrapping_add(P5), r10); bb = 6; continue; }
            5 => { r11 = ((rcx.wrapping_add(P1)) as u64) as u64; bb = 6; continue; }
            6 => { rdx = 0xffffd8ed; rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(r10.wrapping_add(P1))) as u64; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(P0), ((xmm1 as u64)) as u64); crate::simd_ops::lk_add_u64(r10.wrapping_add(P1), (0x18u64) as u64); rax = ((crate::simd_ops::ru8(r10.wrapping_add(P3))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(r10.wrapping_add(P4))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(r10.wrapping_add(P3), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(P5))) as u64; crate::simd_ops::wu64(r10.wrapping_add(P6), rax); crate::simd_ops::wu64(rcx.wrapping_add(P5), r10); bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(r11)) as u64; rax = 0x3; fl_l = (crate::simd_ops::ru64(r10.wrapping_add(P0))) as u64; fl_r = rcx; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { crate::simd_ops::ru64(r10.wrapping_add(P0)) } else { rbx }) as u64; xmm0 = (crate::simd_ops::ru128(rbx)) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64) as u128; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P0), ((xmm1 as u64)) as u64); crate::simd_ops::lk_add_u64(r11, (0x18u64) as u64); rcx = (crate::simd_ops::ru64(r11)) as u64; crate::simd_ops::wu64(rcx, (0x0u64) as u64); crate::simd_ops::wu32(rcx.wrapping_add(P0), (0x3u32) as u32); crate::simd_ops::lk_add_u64(r11, (0x18u64) as u64); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            9 => { rdx = 0x1; r8 = ((rdx.wrapping_add(P8)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x142113a69, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142113a70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1429868d0; const P1: u64 = 0x1420ecd00; const P2: u64 = 0x1420fc9b0; const P3: u64 = 0x1421158a0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rdi = P0; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = rax; fl_r = rdx; rbx = rcx; rcx = rdi; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rcx }) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; r8 = (crate::simd_ops::cvt_f64_i32((f64::from_bits(crate::simd_ops::ru64(rcx))))) as u32 as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 11; continue; } bb = 4; continue; }
            4 => { fl_l = rax; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x6; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 6; continue; }
            6 => { r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x10u64), (0x3u32) as u32); xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | (((((r8 as u32)) as i32) as f64).to_bits() as u128); crate::simd_ops::wu64(rdx, ((xmm0 as u64)) as u64); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0x18u64) as u64); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x1u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rbx); bb = 8; continue; }
            8 => { r11 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = rdi; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_r = r11; rdx = ((r8 as u32)) as u32 as u64; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { crate::simd_ops::ru64(rbx.wrapping_add(0x10u64)) } else { rcx }) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(r11, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(r11.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = rcx.wrapping_add(0x18); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); rax = 0xffffffff; fl_l = rcx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = 0; fl_r = ((rax as u32)) as u32 as u64; rax = (0u32.wrapping_sub(fl_r as u32)) as u32 as u64; rax = (if (((fl_l as u32)) < ((fl_r as u32))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64; rax = ((((rax as u32)) & (0x2u32))) as u32 as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            11 => { rdx = 0x1; rcx = rbx; r8 = ((rdx.wrapping_add(0x5u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x142113b82, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            12 => { rdx = 0x2; rcx = rbx; r8 = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x142113b94, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142113ba0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x1429868d0; const P3: u64 = 0x1; const P4: u64 = 0x18; const P5: u64 = 0x30; const P6: u64 = 0x68; const P7: u64 = 0x142116d40; const P8: u64 = 0x5; const P9: u64 = 0x1421158a0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r10 = rcx; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 9; continue; } bb = 1; continue; }
            1 => { rbx = P2; fl_l = rax; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; fl_r = 0x6; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P3))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64; r11 = ((r10.wrapping_add(P1)) as u64) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(r10.wrapping_add(P3), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(P5))) as u64; crate::simd_ops::wu64(r10.wrapping_add(P6), rax); crate::simd_ops::wu64(rcx.wrapping_add(P5), r10); bb = 6; continue; }
            5 => { r11 = ((rcx.wrapping_add(P1)) as u64) as u64; bb = 6; continue; }
            6 => { rdx = 0xffffd8ed; rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(r10.wrapping_add(P1))) as u64; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(P0), ((xmm1 as u64)) as u64); crate::simd_ops::lk_add_u64(r10.wrapping_add(P1), (0x18u64) as u64); rax = ((crate::simd_ops::ru8(r10.wrapping_add(P3))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(r10.wrapping_add(P4))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(r10.wrapping_add(P3), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(P5))) as u64; crate::simd_ops::wu64(r10.wrapping_add(P6), rax); crate::simd_ops::wu64(rcx.wrapping_add(P5), r10); bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(r11)) as u64; fl_l = (crate::simd_ops::ru64(r10.wrapping_add(P0))) as u64; fl_r = rcx; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { crate::simd_ops::ru64(r10.wrapping_add(P0)) } else { rbx }) as u64; xmm0 = (crate::simd_ops::ru128(rbx)) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64) as u128; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P0), ((xmm1 as u64)) as u64); crate::simd_ops::lk_add_u64(r11, (0x18u64) as u64); rax = (crate::simd_ops::ru64(r11)) as u64; crate::simd_ops::wu32(rax.wrapping_add(P0), (0x0u32) as u32); rax = 0x3; crate::simd_ops::lk_add_u64(r11, (0x18u64) as u64); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            9 => { rdx = 0x1; r8 = ((rdx.wrapping_add(P8)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x142113c92, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142113ca0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1429868d0; const P1: u64 = 0x1420fc210; const P2: u64 = 0x1421158a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rbx = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 11; continue; } bb = 1; continue; }
            1 => { r9 = P0; fl_l = rdx; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x6; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rax = rdx.wrapping_add(0x30); rdi = 0; fl_l = 0; fl_r = 0; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { bb = 5; continue; }
            5 => { crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), ((rdi as u32)) as u32); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = ((rcx).wrapping_add(0x18u64)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rcx); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = rdx.wrapping_add(0x30); fl_l = rcx; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { rax = rdx.wrapping_add(0x30); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x1u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rbx); bb = 8; continue; }
            8 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = rbx; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_r = r8; r9 = (if (((fl_l as u64)) < ((fl_r as u64))) { crate::simd_ops::ru64(rbx.wrapping_add(0x10u64)) } else { r9 }) as u64; r8 = ((r8).wrapping_add(0xffffffffffffffe8u64)) as u64; rdx = (crate::simd_ops::ru64(r9)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = rcx.wrapping_add(0x18); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); rax = 0x2; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            10 => { rax = rcx.wrapping_sub(0x18); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); crate::simd_ops::wu32(rax.wrapping_add(0x10u64), ((rdi as u32)) as u32); rax = 0x1; crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0x18u64) as u64); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            11 => { rdx = 0x1; rcx = rbx; r8 = ((rdx.wrapping_add(0x5u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x142113d9e, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142113db0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x1429868d0; const P3: u64 = 0x1420f83f0; const P4: u64 = 0x142118a60; const P5: u64 = 0x1433e0168; const P6: u64 = 0x142116b30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rcx; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 9; continue; } bb = 1; continue; }
            1 => { rax = P2; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rsi = 0; fl_l = 0; fl_r = 0; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) == ((fl_r as u64))) { rsi } else { rdx }) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax).wrapping_add(0x18u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu32(rdi.wrapping_add(P0), ((rsi as u32)) as u32); fl_l = (crate::simd_ops::lk_add_u64(rbx.wrapping_add(P1), (0x18u64) as u64)) as u64; fl_r = fl_l; bb = 8; continue; }
            5 => { r8 = (0xffffffffffffffffu64) as u64; bb = 6; continue; }
            6 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r8))) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 7; continue; }
            7 => { rdx = rax; rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0x1; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            9 => { r8 = 0x1; rdx = P5; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x142113e41, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142113e50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x1429868d0; const P3: u64 = 0x142986510; const P4: u64 = 0x1; const P5: u64 = 0x142118a60; const P6: u64 = 0x1433e0168; const P7: u64 = 0x142116b30;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 8; continue; } bb = 1; continue; }
            1 => { rdx = P2; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 2; continue; }
            2 => { rdx = (((crate::simd_ops::ru32(rax.wrapping_add(P0))) as i32 as i64) as u64) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 3; continue; }
            3 => { rax = rdx; rdx = P3; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu32(r8.wrapping_add(P0), ((rdx as u32)) as u32); rax = ((rdx.wrapping_add(P4)) as u32) as u32 as u64; crate::simd_ops::lk_add_u64(rcx.wrapping_add(P1), (0x18u64) as u64); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            5 => { r8 = (0xffffffffffffffffu64) as u64; bb = 6; continue; }
            6 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 7; continue; }
            7 => { rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = 0x1; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            8 => { r8 = 0x1; rdx = P6; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x142113ecc, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142113ee0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x48; const P2: u64 = 0x8; const P3: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    xmm0 = 0u128;
    rdx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64;
    rdx = ((rdx).wrapping_shr(10u32)) as u64;
    xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | (((((rdx as u32)) as i32) as f64).to_bits() as u128);
    crate::simd_ops::wu64(rax, ((xmm0 as u64)) as u64);
    crate::simd_ops::wu32(rax.wrapping_add(P3), (0x3u32) as u32);
    rax = 0x1;
    fl_l = (crate::simd_ops::lk_add_u64(rcx.wrapping_add(P2), (0x18u64) as u64)) as u64; fl_r = fl_l;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142113f20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x1429868d0; const P3: u64 = 0x4; const P4: u64 = 0x1420fcf40; const P5: u64 = 0x14; const P6: u64 = 0x1433e1088; const P7: u64 = 0x142116bc0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 12; continue; } bb = 1; continue; }
            1 => { rax = P2; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x6; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x5; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 4; continue; }
            4 => { rdx = ((rax as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x5; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 5; continue; }
            5 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 6; continue; }
            6 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 11; continue; }
            8 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(P3))) as u32 as u64; bb = 11; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((rax as u32)) as u32 as u64; bb = 11; continue; }
            10 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(P5))) as u32 as u64; bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | (((((rcx as u32)) as i32) as f64).to_bits() as u128); crate::simd_ops::wu64(rax, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rax.wrapping_add(P0), (0x3u32) as u32); rax = 0x1; crate::simd_ops::lk_add_u64(rbx.wrapping_add(P1), (0x18u64) as u64); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            12 => { r8 = P6; rdx = 0x1; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x142113fb6, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142113fc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x1429868d0; const P3: u64 = 0x18; const P4: u64 = 0x30; const P5: u64 = 0x48; const P6: u64 = 0x142118280; const P7: u64 = 0x5; const P8: u64 = 0x1421158a0; const P9: u64 = 0x1433e0168; const P10: u64 = 0x142116b30;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = r8; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 13; continue; } bb = 1; continue; }
            1 => { r9 = P2; fl_l = r8; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; fl_r = 0x6; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 3; continue; }
            3 => { rax = ((r8.wrapping_add(P3)) as u64) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 15; continue; } bb = 4; continue; }
            4 => { fl_l = rax; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 15; continue; } bb = 6; continue; }
            6 => { rax = ((r8.wrapping_add(P4)) as u64) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 14; continue; } bb = 7; continue; }
            7 => { fl_l = rax; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 9; continue; }
            9 => { rax = ((r8.wrapping_add(P5)) as u64) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 12; continue; } bb = 10; continue; }
            10 => { r9 = 0; fl_l = 0; fl_r = 0; bb = 11; continue; }
            11 => { crate::simd_ops::wu32(rdx.wrapping_add(P0), ((r9 as u32)) as u32); rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rdx = ((rdx).wrapping_add(0x18u64)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P1), rdx); r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = ((r8.wrapping_add(P5)) as u64) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 11; continue; } bb = 12; continue; }
            12 => { rax = ((r8.wrapping_add(P5)) as u64) as u64; rdx = 0x1; crate::simd_ops::wu64(rcx.wrapping_add(P1), rax); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rax = 0x1; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            13 => { rdx = 0x1; r8 = ((rdx.wrapping_add(P7)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x142114077, rcx, rsp); return rax; }
            14 => { r8 = 0x3; rdx = P9; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14211408a, rcx, rsp); return rax; }
            15 => { r8 = 0x2; rdx = P9; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14211409d, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421140b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1429868d0; const P1: u64 = 0x1420fd0a0; const P2: u64 = 0x1421158a0; const P3: u64 = 0x1433e0168; const P4: u64 = 0x142116b30;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = r8; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 12; continue; } bb = 1; continue; }
            1 => { r9 = P0; fl_l = r8; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(r8.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x6; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 3; continue; }
            3 => { rax = r8.wrapping_add(0x18); fl_l = rax; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 13; continue; } bb = 4; continue; }
            4 => { fl_l = rax; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 6; continue; }
            6 => { rax = r8.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 9; continue; } bb = 7; continue; }
            7 => { r10 = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { crate::simd_ops::wu32(rdx.wrapping_add(0x10u64), ((r10 as u32)) as u32); rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_add(0x18u64)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rdx); r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rax = r8.wrapping_add(0x30); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { rax = r8.wrapping_add(0x30); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rax); rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x1u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rcx.wrapping_add(0x1u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rdx.wrapping_add(0x30u64), rcx); bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_r = rdx; r9 = (if (((fl_l as u64)) < ((fl_r as u64))) { crate::simd_ops::ru64(rcx.wrapping_add(0x10u64)) } else { r9 }) as u64; rbx = rdx.wrapping_sub(0x18); rdx = rbx; rcx = (crate::simd_ops::ru64(r9)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = 0x1; rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            12 => { rdx = 0x1; r8 = ((rdx.wrapping_add(0x5u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1421141ad, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            13 => { r8 = 0x2; rdx = P3; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1421141c0, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421141d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1429868d0; const P2: u64 = 0x18; const P3: u64 = 0x142105fa0; const P4: u64 = 0x1433e0168; const P5: u64 = 0x142116b30;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r11 = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rbx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 7; continue; } bb = 1; continue; }
            1 => { rax = P1; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 3; continue; }
            3 => { rdx = ((rcx.wrapping_add(P2)) as u64) as u64; fl_l = rdx; fl_r = rbx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 8; continue; } bb = 4; continue; }
            4 => { fl_l = rdx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(P0), (0x1u32) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rax = 0x1; rcx = (rcx & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu32(rbx, ((rcx as u32)) as u32); crate::simd_ops::lk_add_u64(r11.wrapping_add(0x8u64), (0x18u64) as u64); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { r8 = 0x1; rdx = P4; rcx = r11; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x142114246, rcx, rsp); return rax; }
            8 => { r8 = 0x2; rdx = P4; rcx = r11; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14211425c, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142114270(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1429868d0; const P1: u64 = 0x142114550; const P2: u64 = 0x1420ecd00; const P3: u64 = 0x1421171a0; const P4: u64 = 0x142117070; const P5: u64 = 0x1421158a0; const P6: u64 = 0x1433e10a8; const P7: u64 = 0x142116b30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rbx = rcx; rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 23; continue; } bb = 1; continue; }
            1 => { rdi = P0; fl_l = rax; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 23; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x6; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 23; continue; } bb = 3; continue; }
            3 => { rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x1u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rbx); bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = rdi; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = rax; fl_r = rdx; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rcx }) as u64; xmm0 = (crate::simd_ops::ru128(rcx)) as u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = rcx.wrapping_add(0x18); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = rdi; crate::simd_ops::wu8(rax.wrapping_add(0x5u64), (0x0u8) as u8); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rcx }) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = 0x1; bb = 8; continue; }
            8 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 19; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdi = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdi }) as u64; fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdx = rsp.wrapping_add(0x20); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 11; continue; }
            11 => { xmm1 = ((crate::simd_ops::ru64(rdi)) as u64) as u128; xmm0 = 0u128; fl_l = ((xmm1 as u64)) as u64; fl_r = ((xmm0 as u64)) as u64; if (f64::from_bits(fl_l).is_nan() || f64::from_bits(fl_r).is_nan()) { bb = 19; continue; } bb = 12; continue; }
            12 => { if (f64::from_bits(fl_l) != f64::from_bits(fl_r) && !f64::from_bits(fl_l).is_nan() && !f64::from_bits(fl_r).is_nan()) { bb = 19; continue; } bb = 13; continue; }
            13 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = ((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as u32) as u32 as u64; fl_l = ((r9 as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = ((((r9 as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; r9 = (r9 & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x1u64), ((r9 as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rbx); bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = rax.wrapping_sub(0x30); fl_l = rax; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 18; continue; } bb = 16; continue; }
            16 => { rcx = rax.wrapping_sub(0x18); bb = 17; continue; }
            17 => { xmm0 = (crate::simd_ops::ru128(rcx)) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rax = ((rax).wrapping_sub(0x18u64)) as u64; rcx = rcx.wrapping_sub(0x18); fl_l = rax; fl_r = rdx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 17; continue; } bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = rbx; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            19 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_sub(0x20u64))) as u32 as u64; fl_r = 0x7; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 21; continue; } bb = 20; continue; }
            20 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x30u64))) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x3u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 24; continue; } bb = 21; continue; }
            21 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 24; continue; } bb = 22; continue; }
            22 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = 0x1; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            23 => { rdx = 0x2; r8 = ((rdx.wrapping_add(0x4u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x142114448, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            24 => { rdx = P6; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x142114458, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142114460(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142114550; const P1: u64 = 0x8; const P2: u64 = 0xfffffffffffffff8; const P3: u64 = 0xffffffffffffffe8; const P4: u64 = 0x3; const P5: u64 = 0x1; const P6: u64 = 0x18; const P7: u64 = 0x30; const P8: u64 = 0x68; const P9: u64 = 0x142116d40; const P10: u64 = 0x10; const P11: u64 = 0x58;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = 0x1; rbx = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; fl_r = 0x7; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P4))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P5))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(P5), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(P7))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P8), rax); crate::simd_ops::wu64(rcx.wrapping_add(P7), rbx); bb = 4; continue; }
            4 => { rdx = 0xffffd8ee; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(P10))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(P10), ((xmm1 as u64)) as u64); bb = 13; continue; }
            5 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P5))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(P5), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(P7))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P8), rax); crate::simd_ops::wu64(rcx.wrapping_add(P7), rbx); bb = 7; continue; }
            7 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rax = (crate::simd_ops::ru32(r8.wrapping_add(P2))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 8; continue; }
            8 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x9; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu32(r8.wrapping_add(P10), (0x0u32) as u32); bb = 13; continue; }
            10 => { rax = (crate::simd_ops::ru64(r8.wrapping_add(P3))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P11))) as u64; crate::simd_ops::wu64(r8, rcx); bb = 12; continue; }
            11 => { rcx = (crate::simd_ops::ru64(r8.wrapping_sub(0x18u64))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(r8, rdx); bb = 12; continue; }
            12 => { crate::simd_ops::wu32(r8.wrapping_add(0x10u64), (0x6u32) as u32); bb = 13; continue; }
            13 => { crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0x18u64) as u64); rax = 0x1; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x18u64))) as u64; crate::simd_ops::wu8(rcx.wrapping_add(0x5u64), (0x0u8) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142114550(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1429868d0; const P1: u64 = 0x1420ecd00; const P2: u64 = 0x14336ec04; const P3: u64 = 0x14210df50; const P4: u64 = 0x1421158a0; const P5: u64 = 0x1433e1118; const P6: u64 = 0x142116bc0; const P7: u64 = 0x1433e10e0; const P8: u64 = 0x142116b30; const P9: u64 = 0x1433e1128;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 49>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x180u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rsi = P0; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rbx = rcx; fl_l = rax; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x7; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x1u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x1u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rbx); bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_r = rcx; rsi = (if (((fl_l as u64)) < ((fl_r as u64))) { crate::simd_ops::ru64(rbx.wrapping_add(0x10u64)) } else { rsi }) as u64; xmm0 = (crate::simd_ops::ru128(rsi)) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0x18u64) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x198u64))) as u64; rsp = ((rsp).wrapping_add(0x180u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x190u64), rdi); fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 7; continue; }
            7 => { fl_l = rax; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 13; continue; } bb = 8; continue; }
            8 => { fl_l = rax; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 13; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rdx = rsp.wrapping_add(0x20); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 12; continue; }
            12 => { rdi = (crate::simd_ops::cvt_f64_i32((f64::from_bits(crate::simd_ops::ru64(rax))))) as u32 as u64; bb = 17; continue; }
            13 => { rdi = 0x1; bb = 17; continue; }
            14 => { fl_l = rax; fl_r = r8; rcx = rsi; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rcx }) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 16; continue; }
            16 => { rdi = (crate::simd_ops::cvt_f64_i32((f64::from_bits(crate::simd_ops::ru64(rcx))))) as u32 as u64; bb = 17; continue; }
            17 => { rcx = rbx; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 25; continue; } bb = 18; continue; }
            18 => { r9 = rsp.wrapping_add(0x40); rdx = ((rdi as u32)) as u32 as u64; r8 = P2; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 23; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_sub(0x18u64)) as u64; fl_l = rax; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 21; continue; } bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 24; continue; } bb = 21; continue; }
            21 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x190u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x198u64))) as u64; rsp = ((rsp).wrapping_add(0x180u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            22 => { rdx = 0x1; rcx = rbx; r8 = ((rdx.wrapping_add(0x2u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x142114693, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            23 => { r8 = P5; rdx = 0x1; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1421146a8, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            24 => { r8 = ((rdi as u32)) as u32 as u64; rdx = P7; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1421146bb, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            25 => { r8 = P9; rdx = 0x1; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1421146cd, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421146e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1429868d0; const P1: u64 = 0x142117aa0; const P2: u64 = 0x1433dfbb8; const P3: u64 = 0x142118a60; const P4: u64 = 0x1420fd0a0; const P5: u64 = 0x1421184a0; const P6: u64 = 0x1421158a0; const P7: u64 = 0x1433e1170; const P8: u64 = 0x1421168e0; const P9: u64 = 0x1433e1148; const P10: u64 = 0x142116b30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rsi = P0; rbx = rcx; rcx = rax.wrapping_add(0x18); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = rcx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; bb = 4; continue; }
            3 => { rcx = 0xffffffff; bb = 4; continue; }
            4 => { fl_l = rax; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 23; continue; } bb = 5; continue; }
            5 => { fl_l = rax; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 23; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x6; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 23; continue; } bb = 7; continue; }
            7 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x6; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 9; continue; }
            9 => { rdx = 0x1; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 15; continue; } bb = 10; continue; }
            10 => { r8 = 0xb; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rdx = P2; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x1u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rbx); bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdi = rcx.wrapping_sub(0x18); rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x30u64))) as u64; rdx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rdi, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = rdx.wrapping_sub(0x18); fl_l = rax; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 19; continue; } bb = 14; continue; }
            14 => { rax = rdx.wrapping_sub(0x30); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); bb = 15; continue; }
            15 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = r8.wrapping_add(0x30); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 18; continue; } bb = 16; continue; }
            16 => { r9 = 0; fl_l = 0; fl_r = 0; bb = 17; continue; }
            17 => { crate::simd_ops::wu32(rdx.wrapping_add(0x10u64), ((r9 as u32)) as u32); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_add(0x18u64)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rdx); r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = r8.wrapping_add(0x30); fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 17; continue; } bb = 18; continue; }
            18 => { rcx = r8.wrapping_add(0x30); rdx = 0x1; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rcx); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = 0x1; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            19 => { fl_l = rax; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 25; continue; } bb = 20; continue; }
            20 => { rcx = rax.wrapping_sub(0x18); bb = 21; continue; }
            21 => { xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = rcx.wrapping_add(0x18); rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 25; continue; } bb = 22; continue; }
            22 => { bb = 21; continue; }
            23 => { rdx = 0x1; rcx = rbx; r8 = ((rdx.wrapping_add(0x5u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x142114866, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            24 => { r8 = P7; rdx = 0x2; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x14211487b, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            25 => { rax = rdx.wrapping_sub(0x18); rcx = rbx; rdx = P9; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x142114893, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421148a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1429868d0; const P1: u64 = 0x142117aa0; const P2: u64 = 0x1433dfbb8; const P3: u64 = 0x142118a60; const P4: u64 = 0x1420fd0a0; const P5: u64 = 0x1433e0168; const P6: u64 = 0x142116b30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rdi = rcx; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 16; continue; } bb = 1; continue; }
            1 => { rsi = P0; fl_l = rax; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 3; continue; }
            3 => { rdx = 0x1; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), ((rax as u32)) as u32); fl_l = (crate::simd_ops::lk_add_u64(rdi.wrapping_add(0x8u64), (0x18u64) as u64)) as u64; fl_r = fl_l; bb = 5; continue; }
            5 => { rax = 0x1; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { rdx = 0x1; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 7; continue; }
            7 => { r8 = 0xb; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rdx = P2; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x1u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rdi.wrapping_add(0x1u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rdi); bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rbx = rcx.wrapping_sub(0x18); rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x30u64))) as u64; rdx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = rdx.wrapping_sub(0x18); fl_l = rax; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = rdx.wrapping_sub(0x30); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rax); rax = 0x1; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            12 => { fl_l = rax; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 15; continue; } bb = 13; continue; }
            13 => { rcx = rax.wrapping_sub(0x18); bb = 14; continue; }
            14 => { xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rcx = rcx.wrapping_add(0x18); rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 14; continue; } bb = 15; continue; }
            15 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = rdx.wrapping_sub(0x18); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rax); rax = 0x1; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            16 => { r8 = 0x1; rdx = P5; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1421149fd, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142114a10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xffffffffffffffff; const P1: u64 = 0x142116b80; const P2: u64 = 0x142119980; const P3: u64 = 0x1421193a0; const P4: u64 = 0x1421159f0; const P5: u64 = 0x142118910; const P6: u64 = 0x142117120; const P7: u64 = 0x1421173b0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = 0x2; rbx = rcx; r8 = ((rdx.wrapping_add(P0)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = 0x1; rcx = rbx; r11 = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = 0x1; rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = ((r11 as u32)) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = 0x1; rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rcx = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x142114a75, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142114a80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1429868d0; const P1: u64 = 0x1420ecd00; const P2: u64 = 0x1420ecc90; const P3: u64 = 0x14210c770; const P4: u64 = 0x1426464f8; const P5: u64 = 0x142646980; const P6: u64 = 0x1421158a0; const P7: u64 = 0x1433e1180; const P8: u64 = 0x142116bc0; const P9: u64 = 0x1433e0168; const P10: u64 = 0x142116b30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rsi = P0; rbx = rcx; rax = ((rax).wrapping_add(0x18u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 24; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 24; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 24; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = rsp.wrapping_add(0x20); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 34; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; bb = 6; continue; }
            6 => { rdi = (crate::simd_ops::cvt_f64_i32((f64::from_bits(crate::simd_ops::ru64(rax))))) as u32 as u64; r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0xa; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 25; continue; } bb = 7; continue; }
            7 => { fl_l = r8; fl_r = rcx; rdx = rsi; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { r8 } else { rdx }) as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x5; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 8; continue; }
            8 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x1u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rbx); bb = 10; continue; }
            10 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 33; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x40u64))) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x48u64))) as u64; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdx = rsi; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; bb = 14; continue; }
            14 => { rsi = (crate::simd_ops::ru64(rdx)) as u64; fl_l = ((rsi).wrapping_add(0x18u64)) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 33; continue; } bb = 15; continue; }
            15 => { rax = ((rdi.wrapping_sub(0x2u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x22; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 35; continue; } bb = 16; continue; }
            16 => { r8 = ((rdi as u32)) as u32 as u64; rdx = rsp.wrapping_add(0x50); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 30; continue; } bb = 17; continue; }
            17 => { rcx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 20; continue; } bb = 18; continue; }
            18 => { bb = 19; continue; }
            19 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rcx = ((rcx).wrapping_add(1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rcx); rcx = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 19; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 30; continue; } bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; xmm0 = 0u128; fl_l = rdi; fl_r = rdi; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rdi) as i64) as f64).to_bits() as u128); crate::simd_ops::wu64(rcx, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), (0x3u32) as u32); bb = 32; continue; }
            23 => { rax = rdi; crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), (0x3u32) as u32); rax = ((rax).wrapping_shr(1u32)) as u64; rdi = ((((rdi as u32)) & (0x1u32))) as u32 as u64; fl_l = (((rax) | (rdi))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) + f64::from_bits(xmm0 as u64)).to_bits() as u128); crate::simd_ops::wu64(rcx, ((xmm0 as u64)) as u64); bb = 32; continue; }
            24 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; bb = 25; continue; }
            25 => { fl_l = r8; fl_r = rcx; rax = rsi; rax = (if (((fl_l as u64)) < ((fl_r as u64))) { r8 } else { rax }) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 31; continue; } bb = 26; continue; }
            26 => { rdx = rsp.wrapping_add(0x20); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 31; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 36; continue; } bb = 28; continue; }
            28 => { fl_l = rax; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 36; continue; } bb = 29; continue; }
            29 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 36; continue; } bb = 30; continue; }
            30 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(0x10u64), (0x0u32) as u32); bb = 32; continue; }
            31 => { xmm0 = ((crate::simd_ops::ru64(rax)) as u64) as u128; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rax, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rax.wrapping_add(0x10u64), (0x3u32) as u32); bb = 32; continue; }
            32 => { crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0x18u64) as u64); rax = 0x1; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            33 => { rdx = 0x1; rcx = rbx; r8 = ((rdx.wrapping_add(0x4u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x142114c8b, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            34 => { rdx = 0x2; rcx = rbx; r8 = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x142114c9d, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            35 => { r8 = P7; rdx = 0x2; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x142114cb2, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            36 => { r8 = 0x1; rdx = P9; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x142114cc8, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142114cd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x142115b50; const P3: u64 = 0x142646870; const P4: u64 = 0x1433e1194; const P5: u64 = 0x1426468f8; const P6: u64 = 0x143231690;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(rcx.wrapping_add(P1)))) as u64; rbx = 0x1; let prod = (rax as i64 as i128).wrapping_mul((rdx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdi = rcx; rbp = rdx; rbp = ((((rbp) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rbp; rax = ((rax).wrapping_shr(63u32)) as u64; rbp = ((rbp).wrapping_add(rax)) as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 6; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); bb = 2; continue; }
            2 => { r8 = rsp.wrapping_add(0x30); rdx = ((rbx as u32)) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rsi = rax; fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = 0x1; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rdx = 0x1; rcx = P4; r8 = ((rdx as u32)) as u32 as u64; r9 = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rcx = 0x1; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdx = 0x1; r9 = rax; rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_add_u64(rdi.wrapping_add(P0), (0xffffffffffffffe8u64) as u64); rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 5; continue; }
            5 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 6; continue; }
            6 => { rcx = 0x1; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rdx = 0x1; rcx = P6; r8 = ((rdx as u32)) as u32 as u64; r9 = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142114dd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646870; const P1: u64 = 0x1426468f8;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rcx;
    rbx = rdx;
    rcx = 0x1;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    r8 = rbx;
    rdx = 0x1;
    r9 = rax;
    rcx = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142114e10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142116d40; const P1: u64 = 0x1433e0fac; const P2: u64 = 0x142103330; const P3: u64 = 0x1420ec690; const P4: u64 = 0x142987c70; const P5: u64 = 0x1433e1218; const P6: u64 = 0x1421153b0; const P7: u64 = 0x1420ec940; const P8: u64 = 0x1429868d0; const P9: u64 = 0x1433e1058; const P10: u64 = 0x142118cd0; const P11: u64 = 0x14210c770; const P12: u64 = 0x1433e0fa4; const P13: u64 = 0x1433e0f98; const P14: u64 = 0x142113a70; const P15: u64 = 0x1433e0f8c; const P16: u64 = 0x142113970; const P17: u64 = 0x142113ca0; const P18: u64 = 0x1433e0f84; const P19: u64 = 0x142113ba0; const P20: u64 = 0x142113420; const P21: u64 = 0x1433e0f7c; const P22: u64 = 0x1421135f0; const P23: u64 = 0x142112f60; const P24: u64 = 0x1433e0f74; const P25: u64 = 0x142113160; const P26: u64 = 0x1433e11f8; const P27: u64 = 0x142116b30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp; rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x1u64))) as u32) as u32 as u64; rbx = rcx; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x1u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rbx); bb = 2; continue; }
            2 => { rdx = 0xffffd8ee; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = 0xffffd8ee; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0x18u64) as u64); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = 0x2; rdx = P1; rcx = rbx; rdi = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; r8 = rbp.wrapping_sub(0x20); r9 = ((r9).wrapping_sub(0x18u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), rax); rdx = rdi; crate::simd_ops::wu32(rbp.wrapping_sub(0x10u64), (0x5u32) as u32); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0xffffffffffffffe8u64) as u64); rsi = P4; r15 = 0; fl_l = 0; fl_r = 0; rax = rsi; r14 = ((r15 as u32)) as u32 as u64; bb = 3; continue; }
            3 => { r14 = (((r14 as u32)).wrapping_add(1)) as u32 as u64; rax = rax.wrapping_add(0x10); fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = r15; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { r9 = 0x1; r8 = P5; rdx = 0xffffd8f0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x1u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rbx); bb = 6; continue; }
            6 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = P1; r8 = 0x2; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; r8 = rbp.wrapping_sub(0x20); rdx = rdi.wrapping_sub(0x18); crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), rax); rcx = rbx; crate::simd_ops::wu32(rbp.wrapping_sub(0x10u64), (0x5u32) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = P8; rdx = rax.wrapping_add(0x18); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rdx); fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x6; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 8; continue; }
            8 => { r9 = ((r14 as u32)) as u32 as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); r8 = P1; rdx = 0xffffd8ee; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 24; continue; } bb = 9; continue; }
            9 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x1u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rbx); bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = P1; r8 = 0x2; rcx = rbx; xmm0 = (crate::simd_ops::ru128(rax.wrapping_sub(0x18u64))) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_sub(0x8u64))) as u64) as u128; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdi = ((rdi).wrapping_add(0x18u64)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rdi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = rdi.wrapping_sub(0x48); r9 = ((r9).wrapping_sub(0x18u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), rax); r8 = rbp.wrapping_sub(0x20); crate::simd_ops::wu32(rbp.wrapping_sub(0x10u64), (0x5u32) as u32); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = ((rdx).wrapping_add(0xffffffffffffffe8u64)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rdx); bb = 12; continue; }
            12 => { rax = rdx.wrapping_sub(0x18); fl_l = rax; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 15; continue; } bb = 13; continue; }
            13 => { rcx = rax.wrapping_sub(0x18); bb = 14; continue; }
            14 => { xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = rcx.wrapping_add(0x18); rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 14; continue; } bb = 15; continue; }
            15 => { rax = rdx.wrapping_sub(0x18); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); rax = P9; bb = 16; continue; }
            16 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; r9 = 0; fl_l = 0; fl_r = 0; r8 = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rsi)) as u64; r8 = (0xffffffffffffffffu64) as u64; rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; bb = 17; continue; }
            17 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 17; continue; } bb = 18; continue; }
            18 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; r8 = rbp.wrapping_sub(0x20); r9 = ((r9).wrapping_sub(0x18u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), rax); rdx = rdi.wrapping_sub(0x30); crate::simd_ops::wu32(rbp.wrapping_sub(0x10u64), (0x5u32) as u32); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0xffffffffffffffe8u64) as u64); rsi = ((rsi).wrapping_add(0x10u64)) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 16; continue; } bb = 19; continue; }
            19 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x40u64))) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x48u64))) as u64; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 21; continue; } bb = 20; continue; }
            20 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 21; continue; }
            21 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x1u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rbx); bb = 23; continue; }
            23 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = P12; r8 = 0x4; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi, rax); rdx = 0xffffd8ee; crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), (0x5u32) as u32); rcx = rbx; crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0x18u64) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = 0x8; rdx = P13; rcx = rbx; rdi = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; r8 = rbp.wrapping_sub(0x20); r9 = ((r9).wrapping_sub(0x18u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), rax); rdx = rdi; crate::simd_ops::wu32(rbp.wrapping_sub(0x10u64), (0x5u32) as u32); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::lk_sub_u64(rbx.wrapping_add(0x8u64), (0x18u64) as u64); rdx = P14; r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); r8 = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = 0x1; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); r8 = P15; rcx = rbx; rdx = P16; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = P15; r8 = 0x6; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; r8 = rbp.wrapping_sub(0x20); r9 = ((r9).wrapping_sub(0x18u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), rax); rdx = rdi.wrapping_sub(0x30); crate::simd_ops::wu32(rbp.wrapping_sub(0x10u64), (0x5u32) as u32); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::lk_sub_u64(rbx.wrapping_add(0x8u64), (0x18u64) as u64); rdx = P17; r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); r8 = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = 0x1; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); r8 = P18; rcx = rbx; rdx = P19; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = P18; r8 = 0x5; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; r8 = rbp.wrapping_sub(0x20); r9 = ((r9).wrapping_sub(0x18u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), rax); rdx = rdi.wrapping_sub(0x30); crate::simd_ops::wu32(rbp.wrapping_sub(0x10u64), (0x5u32) as u32); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::lk_sub_u64(rbx.wrapping_add(0x8u64), (0x18u64) as u64); rax = P20; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r8 = P21; r9 = 0; fl_l = 0; fl_r = 0; rdx = P22; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = P21; r8 = 0x5; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; r8 = rbp.wrapping_sub(0x20); r9 = ((r9).wrapping_sub(0x18u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), rax); rdx = rdi.wrapping_sub(0x30); crate::simd_ops::wu32(rbp.wrapping_sub(0x10u64), (0x5u32) as u32); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::lk_sub_u64(rbx.wrapping_add(0x8u64), (0x18u64) as u64); rax = P23; r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r8 = P24; rcx = rbx; rdx = P25; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = P24; r8 = 0x6; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; r8 = rbp.wrapping_sub(0x20); r9 = ((r9).wrapping_sub(0x18u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), rax); rdx = rdi.wrapping_sub(0x30); crate::simd_ops::wu32(rbp.wrapping_sub(0x10u64), (0x5u32) as u32); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::lk_sub_u64(rbx.wrapping_add(0x8u64), (0x18u64) as u64); rax = 0x1; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            24 => { r8 = P1; rcx = rbx; rdx = P26; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P27, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x14211536c, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142115370(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1433e11e0; const P1: u64 = 0x142116b30;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; r9 = rdx; rax = r8; r9 = ((r9).wrapping_shr(1u32)) as u64; rax = (!(rax)) as u64; r9 = ((r9).wrapping_add(rdx)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = r9; fl_r = r8; r9 = (if (((fl_l as u64)) < ((fl_r as u64))) { r8 } else { r9 }) as u64; rax = r9; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            2 => { rdx = P0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1421153a3, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421153b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142118910; const P1: u64 = 0x142646320; const P2: u64 = 0x142118a60; const P3: u64 = 0x1420fd0a0; const P4: u64 = 0x1429868d0; const P5: u64 = 0x14210c770; const P6: u64 = 0x142107140; const P7: u64 = 0x142986720; const P8: u64 = 0x1420fbda0; const P9: u64 = 0x1420ec690;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r9 as u32)) as u32); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = ((rsp).wrapping_sub(0x28u64)) as u64; r13 = ((r9 as u32)) as u32 as u64; rbp = r8; rbx = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); bb = 1; continue; }
            1 => { rdx = 0x2e; rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { rsi = (0xffffffffffffffffu64) as u64; bb = 3; continue; }
            3 => { rsi = ((rsi).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(rbp))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 4; continue; }
            4 => { fl_l = ((rsi).wrapping_add(rbp)) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; bb = 5; continue; }
            5 => { r12 = rsi; rdx = rbp; r12 = ((r12).wrapping_sub(rbp)) as u64; rcx = rbx; r8 = r12; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as u32) as u32 as u64; r14 = rbx.wrapping_add(0x18); r15 = rbx.wrapping_add(0x68); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(r14)) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x1u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(r15, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rbx); bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdi = rcx.wrapping_sub(0x18); rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x30u64))) as u64; rdx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = P4; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rdi, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = rdx.wrapping_sub(0x18); fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 24; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 18; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); rax = 0x1; fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = (0x2eu8) as u8 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r14); r13 = (if (((fl_l as u8)) == ((fl_r as u8))) { (rax as u32) } else { (r13 as u32) }) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r15); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x40u64))) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x48u64))) as u64; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(r14)) as u64; rax = rbx.wrapping_add(0x18); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); rax = rbx.wrapping_add(0x68); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); bb = 11; continue; }
            11 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x1u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(r15, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rbx); bb = 13; continue; }
            13 => { r8 = ((crate::simd_ops::ru8(rbx.wrapping_add(0x4u64))) as u32) as u32 as u64; rdx = 0x30; r15 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(r14)) as u64; rdi = rax; r14 = 0; fl_l = 0; fl_r = 0; rdx = ((crate::simd_ops::ru8(rcx.wrapping_add(0x20u64))) as u32) as u32 as u64; rdx = (rdx & !0xffu64) | ((((((rdx as u8)) & (0x3u8))) as u8 as u64)); crate::simd_ops::wu8(rax, (0x6u8) as u8); crate::simd_ops::wu8(rax.wrapping_add(0x1u64), ((rdx as u8)) as u8); rcx = ((crate::simd_ops::ru8(rbx.wrapping_add(0x4u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rax.wrapping_add(0x2u64), ((rcx as u8)) as u8); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), r14); crate::simd_ops::wu32(rax.wrapping_add(0x3u64), (0xffu32) as u32); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), r14); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), r14); crate::simd_ops::wu8(rax.wrapping_add(0x7u64), ((r14 as u8)) as u8); rax = P7; crate::simd_ops::wu64(rdi.wrapping_add(0x20u64), rax); fl_l = ((r13 as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { r8 = ((r13 as u32)) as u32 as u64; rdx = rdi; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 15; continue; }
            15 => { crate::simd_ops::wu64(r15, rdi); r8 = r12; crate::simd_ops::wu32(r15.wrapping_add(0x10u64), (0x6u32) as u32); rdx = rbp; crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0x18u64) as u64); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x1u64), ((rax as u8)) as u8); rcx = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rdx, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rbx); bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = rbx; xmm0 = (crate::simd_ops::ru128(rax.wrapping_sub(0x30u64))) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_sub(0x20u64))) as u64) as u128; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); r9 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = r9.wrapping_add(0x18); r8 = r9.wrapping_sub(0x18); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); rdx = r9.wrapping_sub(0x48); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0xffffffffffffffd0u64) as u64)) as u64; fl_r = fl_l; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; r13 = (crate::simd_ops::ru32(rsp.wrapping_add(0x78u64))) as u32 as u64; bb = 19; continue; }
            18 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x6; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 19; continue; }
            19 => { rax = rdx.wrapping_sub(0x18); fl_l = rax; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 22; continue; } bb = 20; continue; }
            20 => { rcx = rax.wrapping_sub(0x18); bb = 21; continue; }
            21 => { xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = rcx.wrapping_add(0x18); rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 21; continue; } bb = 22; continue; }
            22 => { rax = rdx.wrapping_sub(0x18); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); rbp = rsi.wrapping_add(0x1); fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = (0x2eu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 1; continue; } bb = 23; continue; }
            23 => { rax = 0; fl_l = 0; fl_r = 0; bb = 25; continue; }
            24 => { rax = rdx.wrapping_sub(0x30); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); rax = rbp; bb = 25; continue; }
            25 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142115660(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = rax;
    if (((fl_l as u64)) == ((fl_r as u64))) { return rax; }
    loop { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; rcx = ((rcx.wrapping_add(P0)) as u64) as u64; fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = (0x0u64) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142115690(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x270f; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x1433e11d0; const P4: u64 = 0x142116310; const P5: u64 = 0x142118910; const P6: u64 = 0x14210dd80;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = ((rdx.wrapping_add(P0)) as u32) as u32 as u64; rsi = ((rdx as u32)) as u32 as u64; rdi = rcx; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x270f; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rcx.wrapping_add(P2)))) as u64; rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = (((rsi as u32)).wrapping_add((rdx as u32))) as u32 as u64; fl_r = fl_l; rsi = ((fl_l as u32)) as u32 as u64; bb = 2; continue; }
            2 => { r8 = P3; rdx = ((rsi as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rdx = ((rsi as u32)) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; r8 = 0x1; rdx = ((rdx).wrapping_sub(0x30u64)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0x1; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142115740(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142116d40; const P1: u64 = 0x14339902c; const P2: u64 = 0x142103330; const P3: u64 = 0x1420ec940; const P4: u64 = 0x1429868d0; const P5: u64 = 0x142119080; const P6: u64 = 0x1420ec690;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x1u64))) as u32) as u32 as u64; rdi = rcx; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rcx.wrapping_add(0x1u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rdx.wrapping_add(0x30u64), rcx); bb = 2; continue; }
            2 => { rdx = 0xffffd8f0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = 0x6; rdx = P1; rcx = rdi; rbx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r8 = rsp.wrapping_add(0x20); rdx = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rcx = rdi; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x5u32) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rax = rcx.wrapping_add(0x18); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rax); rax = P4; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rcx); r8 = 0; fl_l = 0; fl_r = 0; rcx = rdi; rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x1u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rdi.wrapping_add(0x1u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rdi); bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rdx = 0xffffd8f0; rcx = rdi; xmm0 = (crate::simd_ops::ru128(rax.wrapping_sub(0x18u64))) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_sub(0x8u64))) as u64) as u128; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); crate::simd_ops::lk_add_u64(rdi.wrapping_add(0x8u64), (0x18u64) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = 0x6; rdx = P1; rcx = rdi; rbx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r8 = rsp.wrapping_add(0x20); r9 = ((r9).wrapping_sub(0x18u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rdx = rbx; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x5u32) as u32); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::lk_add_u64(rdi.wrapping_add(0x8u64), (0xffffffffffffffe8u64) as u64)) as u64; fl_r = fl_l; rax = 0x1; bb = 8; continue; }
            7 => { rax = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421158a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142119760; const P1: u64 = 0x1421168e0;
    let (mut rax, mut r10) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; r9 = ((rdx as u32)) as u32 as u64; r10 = rcx; rdx = ((r8 as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r8 = rax; rcx = r10; rdx = ((r9 as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1421158c0, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421158d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x40; const P2: u64 = 0x8; const P3: u64 = 0x3; const P4: u64 = 0x28; const P5: u64 = 0x1433dfc48; const P6: u64 = 0x14183ad52; const P7: u64 = 0x70; const P8: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = rcx; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; rbx = (crate::simd_ops::ru64(rax)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P3))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { rdx = P5; rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P7))) as u64; rbx = 0; fl_l = 0; fl_r = 0; fl_l = rcx; fl_r = rcx; rax = ((rcx.wrapping_add(P8)) as u64) as u64; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbx } else { rax }) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 7; continue; }
            7 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142115950(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0xffffffffffffffff; const P2: u64 = 0x8; const P3: u64 = 0x1429868d0; const P4: u64 = 0x142116d40; const P5: u64 = 0x1420ecd00; const P6: u64 = 0x1421158a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut xmm0: u128 = 0; let mut xmm1: u128 = 0; let mut xmm2: u128 = 0; let mut xmm3: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbx = ((((rdx as u32)) as i32 as i64) as u64) as u64; rdi = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r9 = ((rbx.wrapping_add(P1)) as u64) as u64; r9 = ((r9.wrapping_add(r9.wrapping_mul(2))) as u64) as u64; r9 = ((rax.wrapping_add(r9.wrapping_mul(8))) as u64) as u64; fl_l = r9; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rax = P3; rax = (if (((fl_l as u64)) < ((fl_r as u64))) { r9 } else { rax }) as u64; bb = 5; continue; }
            2 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0xffffd8f0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; rcx = ((rbx.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; bb = 5; continue; }
            4 => { rdx = ((rbx as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = rsp.wrapping_add(0x20); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::cvt_f64_i64((f64::from_bits(crate::simd_ops::ru64(rax))))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            8 => { r8 = 0x3; rdx = ((rbx as u32)) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1421159da, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421159f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1433e1240; const P1: u64 = 0x14210df50; const P2: u64 = 0x1433e1238; const P3: u64 = 0x1421177d0; const P4: u64 = 0x18; const P5: u64 = 0x40; const P6: u64 = 0x48; const P7: u64 = 0x14210c770; const P8: u64 = 0x1; const P9: u64 = 0x30; const P10: u64 = 0x68; const P11: u64 = 0x8; const P12: u64 = 0x1426cdcb0; const P13: u64 = 0x142103330;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 45>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x160u64)) as u64; r9 = rsp.wrapping_add(0x20); rdi = rcx; r8 = P0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x44u64))) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdx = P2; rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x160u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x170u64), rbx); rax = (crate::simd_ops::ru64(rcx.wrapping_add(P5))) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P6))) as u64; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = rdi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(P8))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rdi.wrapping_add(P8), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(P9))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P10), rax); crate::simd_ops::wu64(rcx.wrapping_add(P9), rdi); bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(P11))) as u64; rdx = P12; r8 = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), (0x5u32) as u32); crate::simd_ops::lk_add_u64(rdi.wrapping_add(0x8u64), (0x18u64) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x170u64))) as u64; rsp = ((rsp).wrapping_add(0x160u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142115ab0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1421164b0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::lk_add_u64(rcx, (rdx) as u64)) as u64; fl_r = fl_l;
    return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142115ac0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x8; const P4: u64 = 0xfffffffffffffe00; const P5: u64 = 0x1421160a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = ((rdx.wrapping_add(P0)) as u64) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rcx);
    crate::simd_ops::wu64(rdx, rax);
    rbx = rdx;
    rax = ((rax).wrapping_add(0x200u64)) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P2), (0x0u64) as u64);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rax);
    fl_l = r8; fl_r = (0x200u64) as u64;
    if !((((fl_l as u64)) <= ((fl_r as u64)))) { rdx = ((r8.wrapping_add(P4)) as u64) as u64; rcx = rbx; r8 = 0xffffffff; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx)) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142115b20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1421160a0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    r8 = rdx;
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    rax = r9;
    rax = ((rax).wrapping_sub(rdx)) as u64;
    fl_l = rax; fl_r = r8;
    if !((((fl_l as u64)) >= ((fl_r as u64)))) { rdx = ((rdx).wrapping_sub(r9)) as u64; fl_l = ((rdx).wrapping_add(r8)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; r8 = 0xffffffff; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142115b50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142117aa0; const P1: u64 = 0x1429868d0; const P2: u64 = 0x1433e11d0; const P3: u64 = 0x142118a60; const P4: u64 = 0x1420fd0a0; const P5: u64 = 0x142118e10; const P6: u64 = 0x142116d40; const P7: u64 = 0x142117750; const P8: u64 = 0x1420f83f0; const P9: u64 = 0x1433e1198; const P10: u64 = 0x1421177d0; const P11: u64 = 0x142118910; const P12: u64 = 0x142119110; const P13: u64 = 0x142106590; const P14: u64 = 0x14210c770; const P15: u64 = 0x142103330; const P16: u64 = 0x1420ecd00; const P17: u64 = 0x142119790; const P18: u64 = 0x1430782bc; const P19: u64 = 0x1430782b4; const P20: u64 = 0x14336f5a4; const P21: u64 = 0x1420ecc90; const P22: u64 = 0x14210dd80; const P23: u64 = 0x1433e11a8; const P24: u64 = 0x142116b30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 43>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x130u64)) as u64; rbp = ((((rdx as u32)) as i32 as i64) as u64) as u64; r14 = rcx.wrapping_add(0x8); r12 = r8; rdi = rcx; r15 = ((rbp as u32)) as u32 as u64; rax = ((rbp.wrapping_add(0x270fu64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x270f; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { r9 = (crate::simd_ops::ru64(r14)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(rcx.wrapping_add(0x10u64)))) as u64; let prod = (rax as i64 as i128).wrapping_mul((r9) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; r15 = rdx; r15 = ((r15).wrapping_shr(63u32)) as u64; r15 = ((r15).wrapping_add(rdx)) as u64; r15 = (((r15 as u32)).wrapping_add(1)) as u32 as u64; fl_l = (((r15 as u32)).wrapping_add((rbp as u32))) as u32 as u64; fl_r = fl_l; r15 = ((fl_l as u32)) as u32 as u64; bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x160u64), rbx); rdx = ((r15 as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x168u64), r13); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = P1; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 3; continue; }
            3 => { r8 = 0xa; rdx = P2; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x1u64))) as u32) as u32 as u64; r13 = r14; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; r13 = rdi.wrapping_add(0x8); fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rdi.wrapping_add(0x1u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rdi); bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(r14)) as u64; rbx = rcx.wrapping_sub(0x18); rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x30u64))) as u64; rdx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rdx = (crate::simd_ops::ru64(r14)) as u64; rax = rdx.wrapping_sub(0x18); fl_l = rax; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 65; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 65; continue; } bb = 7; continue; }
            7 => { rax = rdx.wrapping_sub(0x30); crate::simd_ops::wu64(r14, rax); r14 = r13; bb = 8; continue; }
            8 => { rdx = ((rbp as u32)) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = 0; fl_l = 0; fl_r = 0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 50; continue; } bb = 9; continue; }
            9 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 47; continue; } bb = 10; continue; }
            10 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 37; continue; } bb = 11; continue; }
            11 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 28; continue; } bb = 12; continue; }
            12 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 27; continue; } bb = 13; continue; }
            13 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rdx = ((rbp.wrapping_mul(2)) as u64) as u64; rdx = ((rdx).wrapping_add(0xfffffffffffffffdu64)) as u64; rdx = ((rdx).wrapping_add(rbp)) as u64; rdx = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = (crate::simd_ops::ru64(r14)) as u64; rsi = (if (((fl_l as u64)) < ((fl_r as u64))) { rdx } else { rsi }) as u64; bb = 18; continue; }
            15 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = 0xffffd8f0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(r14)) as u64; rcx = ((rbp.wrapping_mul(2)) as u64) as u64; fl_l = ((rcx).wrapping_add(rbp)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; rsi = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; bb = 18; continue; }
            17 => { rdx = ((rbp as u32)) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 25; continue; } bb = 19; continue; }
            19 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x4; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 25; continue; } bb = 20; continue; }
            20 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 25; continue; } bb = 21; continue; }
            21 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 24; continue; } bb = 22; continue; }
            22 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 25; continue; } bb = 23; continue; }
            23 => { rcx = r15; bb = 26; continue; }
            24 => { rcx = (crate::simd_ops::ru64(rsi)) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 26; continue; }
            25 => { rcx = (crate::simd_ops::ru64(rsi)) as u64; bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rbx = rcx; rdx = ((rbp as u32)) as u32 as u64; rbx = (((rbx) as i64).wrapping_mul((crate::simd_ops::ru64(rax.wrapping_add(0xc30u64))) as i64) as u64) as u64; rcx = (((rcx) as i64).wrapping_mul((crate::simd_ops::ru64(rax.wrapping_add(0xc28u64))) as i64) as u64) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(rax.wrapping_add(0xc40u64)))) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rax.wrapping_add(0xc38u64)))) as u64; rbx = (((rbx) ^ (rcx))) as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = rbx; rdx = P9; rcx = rdi; r8 = rax.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 52; continue; }
            27 => { rdx = ((rbp as u32)) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 52; continue; }
            28 => { rdx = ((rbp as u32)) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = rax; rsi = rsp.wrapping_add(0x70); rbx = r15; bb = 29; continue; }
            29 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 30; continue; }
            30 => { crate::simd_ops::wu16(rsi, (0x202cu16) as u16); fl_l = ((rsi).wrapping_add(0x2u64)) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; bb = 31; continue; }
            31 => { xmm1 = ((crate::simd_ops::ru32(rbp.wrapping_add(rbx.wrapping_mul(4)))) as u32) as u128; xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | (((f32::from_bits(xmm1 as u32)) as f64).to_bits() as u128); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(1)) as u64; rsi = rax; fl_l = rbx; fl_r = (0x4u64) as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 29; continue; } bb = 32; continue; }
            32 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x40u64))) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x48u64))) as u64; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 34; continue; } bb = 33; continue; }
            33 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 34; continue; }
            34 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x1u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 36; continue; } bb = 35; continue; }
            35 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rdi.wrapping_add(0x1u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rdi); bb = 36; continue; }
            36 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rax = rsp.wrapping_add(0x70); rsi = ((rsi).wrapping_sub(rax)) as u64; rdx = rsp.wrapping_add(0x70); r8 = rsi; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P15, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), (0x5u32) as u32); fl_l = (crate::simd_ops::lk_add_u64(rdi.wrapping_add(0x8u64), (0x18u64) as u64)) as u64; fl_r = fl_l; bb = 52; continue; }
            37 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 39; continue; } bb = 38; continue; }
            38 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rdx = ((rbp.wrapping_mul(2)) as u64) as u64; rdx = ((rdx).wrapping_add(0xfffffffffffffffdu64)) as u64; rdx = ((rdx).wrapping_add(rbp)) as u64; rdx = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = (crate::simd_ops::ru64(r14)) as u64; rsi = (if (((fl_l as u64)) < ((fl_r as u64))) { rdx } else { rsi }) as u64; bb = 42; continue; }
            39 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = 0xffffd8f0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 41; continue; } bb = 40; continue; }
            40 => { rax = (crate::simd_ops::ru64(r14)) as u64; rcx = ((rbp.wrapping_mul(2)) as u64) as u64; fl_l = ((rcx).wrapping_add(rbp)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; rsi = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; bb = 42; continue; }
            41 => { rdx = ((rbp as u32)) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; bb = 42; continue; }
            42 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 45; continue; } bb = 43; continue; }
            43 => { rdx = rsp.wrapping_add(0x50); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P16, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 45; continue; } bb = 44; continue; }
            44 => { xmm1 = 0u128; bb = 46; continue; }
            45 => { xmm1 = ((crate::simd_ops::ru64(rsi)) as u64) as u128; bb = 46; continue; }
            46 => { rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x20); fl_l = rax; fl_r = rcx; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rdx = rsp.wrapping_add(0x20); r8 = rax; bb = 51; continue; }
            47 => { rdx = ((rbp as u32)) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P17, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rcx = P18; rdx = P19; r8 = (0xffffffffffffffffu64) as u64; rdx = (if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { rcx } else { rdx }) as u64; bb = 48; continue; }
            48 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 48; continue; } bb = 49; continue; }
            49 => { bb = 51; continue; }
            50 => { r8 = 0x3; rdx = P20; bb = 51; continue; }
            51 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 52; continue; }
            52 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rbx = rdi.wrapping_add(0x8); rdx = ((rdx).wrapping_sub(0x18u64)) as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x5; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 60; continue; } bb = 53; continue; }
            53 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x1u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 55; continue; } bb = 54; continue; }
            54 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rsi = rdi.wrapping_add(0x8); fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rdi.wrapping_add(0x1u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rdi); bb = 56; continue; }
            55 => { rsi = rbx; bb = 56; continue; }
            56 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P21, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 76; continue; } bb = 57; continue; }
            57 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x40u64))) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x48u64))) as u64; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 59; continue; } bb = 58; continue; }
            58 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rsi; bb = 59; continue; }
            59 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdx; fl_r = (0x18u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 60; continue; }
            60 => { fl_l = r12; fl_r = r12; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 62; continue; } bb = 61; continue; }
            61 => { rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x14u64))) as u32 as u64; crate::simd_ops::wu64(r12, rcx); bb = 62; continue; }
            62 => { r15 = (crate::simd_ops::ru64(rdx)) as u64; bb = 63; continue; }
            63 => { fl_l = ((r15).wrapping_add(0x18u64)) as u64; fl_r = fl_l; r15 = ((fl_l as u64)) as u64; bb = 64; continue; }
            64 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x168u64))) as u64; rax = r15; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x160u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x170u64))) as u64; rsp = ((rsp).wrapping_add(0x130u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            65 => { fl_l = rax; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 69; continue; } bb = 66; continue; }
            66 => { rcx = rax.wrapping_sub(0x18); bb = 67; continue; }
            67 => { xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rdx = (crate::simd_ops::ru64(r14)) as u64; rcx = rcx.wrapping_add(0x18); rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 67; continue; } bb = 68; continue; }
            68 => { r14 = rdi.wrapping_add(0x8); bb = 69; continue; }
            69 => { rax = rdx.wrapping_sub(0x18); rcx = rdi; rdx = ((r15 as u32)) as u32 as u64; crate::simd_ops::wu64(r14, rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(r14)) as u64; r8 = 0x1; rdx = ((rdx).wrapping_sub(0x30u64)) as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P22, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = (crate::simd_ops::ru64(r14)) as u64; r15 = ((r15).wrapping_add(0xffffffffffffffe8u64)) as u64; fl_l = r15; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 84; continue; } bb = 70; continue; }
            70 => { rax = (crate::simd_ops::ru32(r15.wrapping_add(0x10u64))) as u32 as u64; rax = (((rax as u32)).wrapping_add(0xfffffffdu32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xfffffffd; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 84; continue; } bb = 71; continue; }
            71 => { fl_l = (crate::simd_ops::ru32(r15.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x5; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 81; continue; } bb = 72; continue; }
            72 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x1u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 74; continue; } bb = 73; continue; }
            73 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rdi.wrapping_add(0x1u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rdi); bb = 74; continue; }
            74 => { rdx = r15; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P21, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 78; continue; } bb = 75; continue; }
            75 => { r15 = 0; fl_l = 0; fl_r = 0; bb = 76; continue; }
            76 => { fl_l = r12; fl_r = r12; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 64; continue; } bb = 77; continue; }
            77 => { crate::simd_ops::wu64(r12, r15); bb = 64; continue; }
            78 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x40u64))) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x48u64))) as u64; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 80; continue; } bb = 79; continue; }
            79 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 80; continue; }
            80 => { r15 = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = r15; fl_r = (0x18u64) as u64; r15 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 81; continue; }
            81 => { fl_l = r12; fl_r = r12; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 83; continue; } bb = 82; continue; }
            82 => { rax = (crate::simd_ops::ru64(r15)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x14u64))) as u32 as u64; crate::simd_ops::wu64(r12, rcx); bb = 83; continue; }
            83 => { r15 = (crate::simd_ops::ru64(r15)) as u64; bb = 63; continue; }
            84 => { rdx = P23; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P24, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x142116097, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421160a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1421034a0; const P1: u64 = 0x14183acbc; const P2: u64 = 0x142118840; const P3: u64 = 0x1433e11e0; const P4: u64 = 0x142116b30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; r12 = rcx.wrapping_add(0x20); rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_l = rax; fl_r = rax; rdi = rcx; r15 = ((((r8 as u32)) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rbp = rdx; r14 = rax.wrapping_add(0x18); r14 = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r12 } else { r14 }) as u64; rcx = ((rcx).wrapping_sub(r14)) as u64; r8 = ((rcx.wrapping_add(rdx)) as u64) as u64; rdx = rcx; rax = r8; rdx = ((rdx).wrapping_shr(1u32)) as u64; rax = (!(rax)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = ((rdx.wrapping_add(rcx)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); fl_l = rax; fl_r = r8; rbp = (if (((fl_l as u64)) >= ((fl_r as u64))) { rdx } else { rbp }) as u64; rbp = ((rbp).wrapping_add(rcx)) as u64; rcx = rbx; rdx = rbp; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(rdi)) as u64; rdx = r14; r8 = ((r8).wrapping_sub(r14)) as u64; rsi = rax; rcx = rax.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = r14; fl_r = r12; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = ((r15 as u32)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), (0x0u32) as u32); rcx = rbx; crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0x18u64) as u64); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = ((r15.wrapping_add(r15.wrapping_mul(2))) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), rsi); crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x10u64), (0x5u32) as u32); rcx = rbp.wrapping_add(0x18); rax = (crate::simd_ops::ru64(rdi)) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = ((rax).wrapping_sub(r14)) as u64; rax = ((rax).wrapping_add(0x18u64)) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), rsi); rax = ((rax).wrapping_add(rsi)) as u64; rcx = ((rcx).wrapping_add(rsi)) as u64; crate::simd_ops::wu64(rdi, rax); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rcx); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rdx = P3; rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1421161ad, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421161c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142118e10; const P1: u64 = 0x1433e0168; const P2: u64 = 0x142116b30;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; r10 = ((rdx as u32)) as u32 as u64; r11 = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            2 => { r8 = ((r10 as u32)) as u32 as u64; rdx = P1; rcx = r11; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1421161eb, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142116200(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142117750; const P1: u64 = 0x1420f83f0;
    let (mut rax, mut r11) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    r11 = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rdx = rax;
    rcx = r11;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = ((rax).wrapping_add(0x18u64)) as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142116230(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x1; const P3: u64 = 0x18; const P4: u64 = 0x30; const P5: u64 = 0x68; const P6: u64 = 0x1420ecc90; const P7: u64 = 0x40; const P8: u64 = 0x48; const P9: u64 = 0x14210c770; const P10: u64 = 0x14; const P11: u64 = 0x1421160a0; const P12: u64 = 0x14183acbc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = rcx; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rdx = ((rdx).wrapping_sub(0x18u64)) as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = 0x5; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 1; continue; }
            1 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(P2), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P5), rax); crate::simd_ops::wu64(rcx.wrapping_add(P4), rbx); bb = 3; continue; }
            3 => { rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P7))) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P8))) as u64; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = rbx; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = (0x18u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); rsi = (crate::simd_ops::ru32(rax.wrapping_add(P10))) as u32 as u64; rbp = ((rax.wrapping_add(P3)) as u64) as u64; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rdx = (crate::simd_ops::ru64(rdi)) as u64; rax = rcx; rax = ((rax).wrapping_sub(rdx)) as u64; fl_l = rax; fl_r = rsi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = ((rdx).wrapping_sub(rcx)) as u64; r8 = 0xfffffffe; rdx = ((rdx).wrapping_add(rsi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; r8 = rsi; rdx = rbp; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_add_u64(rdi, (rsi) as u64); fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_r = (0x18u64) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64); bb = 11; continue; }
            11 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142116310(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142117aa0; const P1: u64 = 0x142118a60; const P2: u64 = 0x1420fd0a0; const P3: u64 = 0x1429868d0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r11 = r8; rdi = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); fl_l = r11; fl_r = r11; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(0x10u64), ((r11 as u32)) as u32); fl_l = (crate::simd_ops::lk_add_u64(rdi.wrapping_add(0x8u64), (0x18u64) as u64)) as u64; fl_r = fl_l; bb = 6; continue; }
            3 => { r8 = (0xffffffffffffffffu64) as u64; bb = 4; continue; }
            4 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r11.wrapping_add(r8))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 5; continue; }
            5 => { rdx = r11; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x1u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rdi.wrapping_add(0x1u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rdi); bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rbx = rcx.wrapping_sub(0x18); rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x30u64))) as u64; rdx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = P3; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = rdx.wrapping_sub(0x18); fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rax = rdx.wrapping_sub(0x30); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rax); bb = 11; continue; }
            11 => { rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            12 => { fl_l = rax; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 15; continue; } bb = 13; continue; }
            13 => { rcx = rax.wrapping_sub(0x18); bb = 14; continue; }
            14 => { xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rcx = rcx.wrapping_add(0x18); rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 14; continue; } bb = 15; continue; }
            15 => { rax = rdx.wrapping_sub(0x18); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rax); rax = 0x1; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142116420(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142118e10; const P1: u64 = 0x1421191c0; const P2: u64 = 0x5; const P3: u64 = 0x1421158a0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r11 = r9; r10 = r8; rbx = ((rdx as u32)) as u32 as u64; rdi = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) > 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { fl_l = r11; fl_r = r11; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 2; continue; }
            2 => { fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = (0xffffffffffffffffu64) as u64; bb = 4; continue; }
            4 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r10.wrapping_add(rax))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(r11, rax); rax = r10; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r11, rax); bb = 7; continue; }
            7 => { rax = r10; bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            9 => { r8 = r11; rdx = ((rbx as u32)) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 10; continue; }
            10 => { r8 = ((rax.wrapping_add(P2)) as u32) as u32 as u64; rdx = ((rbx as u32)) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14211649e, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421164b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x10; const P2: u64 = 0x40; const P3: u64 = 0x48; const P4: u64 = 0x14210c770; const P5: u64 = 0x8; const P6: u64 = 0x142103230; const P7: u64 = 0xffffffffffffffe8; const P8: u64 = 0xfffffffffffffff8; const P9: u64 = 0x142103330; const P10: u64 = 0x20; const P11: u64 = 0x142118a60;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rcx; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { r8 = (crate::simd_ops::ru64(rbx)) as u64; rcx = rdi; rsi = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64; fl_l = r8; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = rbp; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsi.wrapping_add(P7), rax); crate::simd_ops::wu32(rsi.wrapping_add(P8), (0x5u32) as u32); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { r8 = ((r8).wrapping_sub(rbp)) as u64; rdx = rbp.wrapping_add(0x18); r8 = ((r8).wrapping_sub(0x18u64)) as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            7 => { r8 = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((rcx.wrapping_add(P10)) as u64) as u64; r8 = ((r8).wrapping_sub(rbx)) as u64; rcx = rdi; r8 = ((r8).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P11, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142116560(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142117650; const P1: u64 = 0x1433e1220; const P2: u64 = 0x142116b30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = r8; rbx = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { r8 = rdi; rdx = P1; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x142116596, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421165a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1421160a0; const P2: u64 = 0x14183acbc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rsi = rdx;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdi = r8;
    rax = rcx;
    rdx = (crate::simd_ops::ru64(rbx)) as u64;
    rax = ((rax).wrapping_sub(rdx)) as u64;
    fl_l = rax; fl_r = r8;
    if !((((fl_l as u64)) >= ((fl_r as u64)))) { rdx = ((rdx).wrapping_sub(rcx)) as u64; rcx = rbx; rdx = ((rdx).wrapping_add(r8)) as u64; r8 = ((r9 as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rcx = (crate::simd_ops::ru64(rbx)) as u64;
    r8 = rdi;
    rdx = rsi;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::lk_add_u64(rbx, (rdi) as u64);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142116610(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1433e1218; const P1: u64 = 0x1421153b0; const P2: u64 = 0x142103330; const P3: u64 = 0x1420ec940; const P4: u64 = 0x1429868d0; const P5: u64 = 0x1420ec690; const P6: u64 = 0x142118cd0; const P7: u64 = 0x1433e11f8; const P8: u64 = 0x142116b30;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rsi = r8; rbp = rdx; rdi = rcx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 1; continue; }
            1 => { r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), r13); rax = r8; fl_l = (crate::simd_ops::ru64(r8)) as u64; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { r14 = (((r14 as u32)).wrapping_add(1)) as u32 as u64; rax = rax.wrapping_add(0x10); fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { r9 = 0x1; r8 = P0; rdx = 0xffffd8f0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x1u64))) as u32) as u32 as u64; r12 = rdi.wrapping_add(0x1); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rdi.wrapping_add(0x1u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rdi); bb = 6; continue; }
            6 => { r13 = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rbx = rdi.wrapping_add(0x8); r8 = (0xffffffffffffffffu64) as u64; bb = 7; continue; }
            7 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r8.wrapping_add(rbp))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 8; continue; }
            8 => { rdx = rbp; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(rbx)) as u64; r8 = rsp.wrapping_add(0x30); rdx = r13.wrapping_sub(0x18); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rcx = rdi; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x5u32) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = P4; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rdx = rax.wrapping_add(0x18); crate::simd_ops::wu64(rbx, rdx); fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x6; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 10; continue; }
            10 => { r9 = ((r14 as u32)) as u32 as u64; crate::simd_ops::wu64(rbx, rax); r8 = rbp; rdx = 0xffffd8ee; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 27; continue; } bb = 11; continue; }
            11 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x1u64))) as u32) as u32 as u64; r15 = rbx; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; r15 = rdi.wrapping_add(0x8); fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(r12, ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rdi); bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rbx)) as u64; r8 = (0xffffffffffffffffu64) as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_sub(0x18u64))) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_sub(0x8u64))) as u64) as u128; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); r14 = (crate::simd_ops::ru64(rbx)) as u64; fl_l = ((r14).wrapping_add(0x18u64)) as u64; fl_r = fl_l; r14 = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rbx, r14); bb = 14; continue; }
            14 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r8.wrapping_add(rbp))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 14; continue; } bb = 15; continue; }
            15 => { rdx = rbp; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(rbx)) as u64; rdx = r14.wrapping_sub(0x48); r9 = ((r9).wrapping_sub(0x18u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); r8 = rsp.wrapping_add(0x30); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x5u32) as u32); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::lk_add_u64(rbx, (0xffffffffffffffe8u64) as u64)) as u64; fl_r = fl_l; rdx = (crate::simd_ops::ru64(rbx)) as u64; rbx = r15; bb = 16; continue; }
            16 => { rax = rdx.wrapping_sub(0x18); fl_l = rax; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 20; continue; } bb = 17; continue; }
            17 => { rcx = rax.wrapping_sub(0x18); bb = 18; continue; }
            18 => { xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = rcx.wrapping_add(0x18); rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 18; continue; } bb = 19; continue; }
            19 => { rbx = rdi.wrapping_add(0x8); bb = 20; continue; }
            20 => { rax = rdx.wrapping_sub(0x18); crate::simd_ops::wu64(rbx, rax); bb = 22; continue; }
            21 => { rbx = rcx.wrapping_add(0x8); bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru64(rsi)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 23; continue; }
            23 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; r9 = 0; fl_l = 0; fl_r = 0; r8 = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (0x0u64) as u64); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rsi)) as u64; r8 = (0xffffffffffffffffu64) as u64; rbp = (crate::simd_ops::ru64(rbx)) as u64; bb = 24; continue; }
            24 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 24; continue; } bb = 25; continue; }
            25 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(rbx)) as u64; r8 = rsp.wrapping_add(0x30); r9 = ((r9).wrapping_sub(0x18u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rdx = rbp.wrapping_sub(0x30); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x5u32) as u32); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::lk_add_u64(rbx, (0xffffffffffffffe8u64) as u64); rsi = ((rsi).wrapping_add(0x10u64)) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 23; continue; } bb = 26; continue; }
            26 => { r11 = rsp.wrapping_add(0x50); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            27 => { r8 = rbp; rdx = P7; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1421168a2, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421168b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x10; const P2: u64 = 0x8; const P3: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((rdx.wrapping_add(P0)) as u64) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rcx);
    crate::simd_ops::wu64(rdx, rax);
    fl_l = ((rax).wrapping_add(0x200u64)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), (0x0u64) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421168e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1421158d0; const P1: u64 = 0x142117750; const P2: u64 = 0x1420f83d0; const P3: u64 = 0x1433e12d0; const P4: u64 = 0x142116b30; const P5: u64 = 0x1433e12a0; const P6: u64 = 0x1433e1270; const P7: u64 = 0x1433e1248;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbp = r8; rsi = ((rdx as u32)) as u32 as u64; rbx = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi as u32)) as u32 as u64; rcx = rbx; rdi = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdx = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r8 = ((rsi as u32)) as u32 as u64; rcx = rbx; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rdx = P3; r9 = rdi; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x142116942, rcx, rsp); return rax; }
            3 => { r9 = rbp; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rdx = P5; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x142116957, rcx, rsp); return rax; }
            4 => { r8 = ((rsi as u32)) as u32 as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { r9 = rdi; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rdx = P6; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x142116974, rcx, rsp); return rax; }
            6 => { r9 = rbp; rdx = P7; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x142116984, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142116990(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1421191c0; const P1: u64 = 0x1421158a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = ((rdx as u32)) as u32 as u64; rdi = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { r8 = 0x5; rdx = ((rbx as u32)) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1421169c4, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421169d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142118e10; const P1: u64 = 0x1421158a0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r10 = ((r8 as u32)) as u32 as u64; r11 = ((rdx as u32)) as u32 as u64; rbx = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { r8 = ((r10 as u32)) as u32 as u64; rdx = ((r11 as u32)) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1421169fd, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142116a10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0xffffffffffffffff; const P2: u64 = 0x8; const P3: u64 = 0x1429868d0; const P4: u64 = 0x142116d40; const P5: u64 = 0x1420ecd00; const P6: u64 = 0x1421158a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut xmm0: u128 = 0; let mut xmm1: u128 = 0; let mut xmm2: u128 = 0; let mut xmm3: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbx = ((((rdx as u32)) as i32 as i64) as u64) as u64; rdi = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r9 = ((rbx.wrapping_add(P1)) as u64) as u64; r9 = ((r9.wrapping_add(r9.wrapping_mul(2))) as u64) as u64; r9 = ((rax.wrapping_add(r9.wrapping_mul(8))) as u64) as u64; fl_l = r9; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rax = P3; rax = (if (((fl_l as u64)) < ((fl_r as u64))) { r9 } else { rax }) as u64; bb = 5; continue; }
            2 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0xffffd8f0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; rcx = ((rbx.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; bb = 5; continue; }
            4 => { rdx = ((rbx as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = rsp.wrapping_add(0x20); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::cvt_f64_i32((f64::from_bits(crate::simd_ops::ru64(rax))))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            8 => { r8 = 0x3; rdx = ((rbx as u32)) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x142116a99, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142116aa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142118e10; const P1: u64 = 0x142116c20;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128);
    r10 = ((rdx as u32)) as u32 as u64;
    xmm6 = (xmm2) as u128;
    r11 = rcx;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !((((((fl_l as u32)) & ((fl_r as u32))) as i32) > 0)) { xmm0 = (xmm6) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x38u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rdx = ((r10 as u32)) as u32 as u64;
    rcx = r11;
    xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128;
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return crate::simd_ops::tail_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142116af0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142119110; const P1: u64 = 0x1421158a0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; r10 = ((rdx as u32)) as u32 as u64; r11 = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            2 => { r8 = 0x4; rdx = ((r10 as u32)) as u32 as u64; rcx = r11; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x142116b1a, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142116b30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1421159f0; const P1: u64 = 0x142116e40; const P2: u64 = 0x142117120; const P3: u64 = 0x1421173b0;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rdx = 0x1; rdi = rsp.wrapping_add(0x50); rbx = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; r8 = rdi; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x142116b77, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142116b80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142118e10; const P1: u64 = 0x142116a10;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    r10 = ((r8 as u32)) as u32 as u64;
    r11 = ((rdx as u32)) as u32 as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !((((((fl_l as u32)) & ((fl_r as u32))) as i32) > 0)) { rax = ((r10 as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rdx = ((r11 as u32)) as u32 as u64;
    rcx = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142116bc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1421158d0; const P1: u64 = 0x1433e1328; const P2: u64 = 0x142116b30; const P3: u64 = 0x1433e1308;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = r8; rsi = ((rdx as u32)) as u32 as u64; rbx = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r8 = ((rsi as u32)) as u32 as u64; rcx = rbx; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { r9 = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rdx = P1; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x142116bfb, rcx, rsp); return rax; }
            2 => { r9 = rdi; rdx = P3; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x142116c0b, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142116c20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0xffffffffffffffff; const P2: u64 = 0x8; const P3: u64 = 0x1429868d0; const P4: u64 = 0x142116d40; const P5: u64 = 0x1420ecd00; const P6: u64 = 0x1421158a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbx = ((((rdx as u32)) as i32 as i64) as u64) as u64; rdi = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r9 = ((rbx.wrapping_add(P1)) as u64) as u64; r9 = ((r9.wrapping_add(r9.wrapping_mul(2))) as u64) as u64; r9 = ((rax.wrapping_add(r9.wrapping_mul(8))) as u64) as u64; fl_l = r9; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rax = P3; rax = (if (((fl_l as u64)) < ((fl_r as u64))) { r9 } else { rax }) as u64; bb = 5; continue; }
            2 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0xffffd8f0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; rcx = ((rbx.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; bb = 5; continue; }
            4 => { rdx = ((rbx as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = rsp.wrapping_add(0x20); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { xmm0 = ((crate::simd_ops::ru64(rax)) as u64) as u128; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            8 => { r8 = 0x3; rdx = ((rbx as u32)) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x142116ca9, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142116cb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = ((rax).wrapping_add(0x1048u64)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142116cd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14210dd80;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    r8 = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    rdx = (crate::simd_ops::ru64(rdx)) as u64;
    return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142116cf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1429868d0; const P2: u64 = 0x8; const P3: u64 = 0x142116d40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = rcx;
    fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64;
    if !((((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0)) { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rcx = P1; rax = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; rax = ((rax).wrapping_add(0xffffffffffffffe8u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r8.wrapping_add(P2))) as u64; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { rcx } else { rax }) as u64; return rax; }
    fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffd8f0;
    if !(((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32))) { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(P2))) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; return rax; }
    return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142116d40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x8; const P2: u64 = 0x4; const P3: u64 = 0xffffffffffffffff; const P4: u64 = 0x1429868d0; const P5: u64 = 0x18; const P6: u64 = 0x40; const P7: u64 = 0x58; const P8: u64 = 0x10; const P9: u64 = 0xbe0; const P10: u64 = 0xbf0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r9 = 0xffffd8ee; r8 = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 1; continue; }
            1 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffd8ef; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 2; continue; }
            2 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffd8f0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r9 = (((r9 as u32)).wrapping_sub((rdx as u32))) as u32 as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; r8 = (crate::simd_ops::ru64(rcx)) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(P2))) as u32) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = ((r9.wrapping_add(P3)) as u32) as u32 as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = ((rcx).wrapping_add(0x2u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; rax = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rax = ((r8.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; return rax; }
            5 => { rax = P4; return rax; }
            6 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P5))) as u64; fl_l = ((rax).wrapping_add(0xbf8u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; return rax; }
            7 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P5))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P6))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P7))) as u64; bb = 10; continue; }
            9 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P8))) as u64; bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rdx.wrapping_add(P9), rax); crate::simd_ops::wu32(rdx.wrapping_add(P10), (0x6u32) as u32); rax = (crate::simd_ops::ru64(r8.wrapping_add(P5))) as u64; fl_l = ((rax).wrapping_add(0xbe0u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; return rax; }
            11 => { rax = (crate::simd_ops::ru64(r8.wrapping_add(P7))) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P5))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P9), rax); crate::simd_ops::wu32(rcx.wrapping_add(P10), (0x6u32) as u32); rax = (crate::simd_ops::ru64(r8.wrapping_add(P5))) as u64; fl_l = ((rax).wrapping_add(0xbe0u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142116e10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x40; const P2: u64 = 0x58; const P3: u64 = 0x8; const P4: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; return rax; }
    rax = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64;
    rax = (crate::simd_ops::ru64(rax)) as u64;
    rax = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142116e40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x40; const P2: u64 = 0x48; const P3: u64 = 0x14210c770; const P4: u64 = 0x1; const P5: u64 = 0x30; const P6: u64 = 0x68; const P7: u64 = 0x142105b10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdi = r8;
    rsi = rdx;
    rbx = rcx;
    rax = (crate::simd_ops::ru64(r9.wrapping_add(P1))) as u64;
    fl_l = (crate::simd_ops::ru64(r9.wrapping_add(P2))) as u64; fl_r = rax;
    if !((((fl_l as u64)) < ((fl_r as u64)))) { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P4))) as u32) as u32 as u64;
    fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(P4), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(P5))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P6), rax); crate::simd_ops::wu64(rcx.wrapping_add(P5), rbx); }
    r8 = rdi;
    rdx = rsi;
    rcx = rbx;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P7, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142116eb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0xfffffffffffffffd; const P2: u64 = 0x8; const P3: u64 = 0x1429868d0; const P4: u64 = 0x142116d40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; r9 = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r8 = ((r8.wrapping_add(P1)) as u64) as u64; r8 = ((rax.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rax = P3; rax = (if (((fl_l as u64)) < ((fl_r as u64))) { r8 } else { rax }) as u64; bb = 5; continue; }
            2 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffd8f0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(r9.wrapping_add(P2))) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; bb = 5; continue; }
            4 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x2; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 6; continue; }
            6 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x4; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 7; continue; }
            7 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 8; continue; }
            8 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 9; continue; }
            9 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            11 => { rax = (crate::simd_ops::ru64(rax)) as u64; rax = ((rax).wrapping_add(0x10u64)) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            12 => { rax = (crate::simd_ops::ru64(rax)) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142116f40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0xc30; const P2: u64 = 0xc28; const P3: u64 = 0xc40; const P4: u64 = 0xc38;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = rdx;
    rax = (((rax) as i64).wrapping_mul((crate::simd_ops::ru64(r8.wrapping_add(P1))) as i64) as u64) as u64;
    rdx = (((rdx) as i64).wrapping_mul((crate::simd_ops::ru64(r8.wrapping_add(P2))) as i64) as u64) as u64;
    rax = ((rax).wrapping_add(crate::simd_ops::ru64(r8.wrapping_add(P3)))) as u64;
    rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(r8.wrapping_add(P4)))) as u64;
    fl_l = (((rax) ^ (rdx))) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142116f70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1; const P1: u64 = 0x18; const P2: u64 = 0x30; const P3: u64 = 0x68; const P4: u64 = 0x8; const P5: u64 = 0xfffffffffffffff8; const P6: u64 = 0x10; const P7: u64 = 0xffffffffffffffe8; const P8: u64 = 0x58;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    r8 = rcx;
    fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rcx.wrapping_add(P0), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P3), rax); crate::simd_ops::wu64(rdx.wrapping_add(P2), rcx); }
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7;
    if !((((fl_l as u32)) == ((fl_r as u32)))) { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x9; if !((((fl_l as u32)) == ((fl_r as u32)))) { crate::simd_ops::wu32(rdx.wrapping_add(P6), (0x0u32) as u32); fl_l = (crate::simd_ops::lk_add_u64(rcx.wrapping_add(P4), (0x18u64) as u64)) as u64; fl_r = fl_l; return rax; } rax = (crate::simd_ops::ru64(rdx.wrapping_add(P7))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P8))) as u64; crate::simd_ops::wu64(rdx, rcx); crate::simd_ops::wu32(rdx.wrapping_add(P6), (0x6u32) as u32); fl_l = (crate::simd_ops::lk_add_u64(r8.wrapping_add(P4), (0x18u64) as u64)) as u64; fl_r = fl_l; return rax; }
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P7))) as u64;
    rcx = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64;
    crate::simd_ops::wu64(rdx, rcx);
    crate::simd_ops::wu32(rdx.wrapping_add(P6), (0x6u32) as u32);
    fl_l = (crate::simd_ops::lk_add_u64(r8.wrapping_add(P4), (0x18u64) as u64)) as u64; fl_r = fl_l;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142116ff0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0xfffffffffffffffd; const P2: u64 = 0x8; const P3: u64 = 0x1429868d0; const P4: u64 = 0x142116d40; const P5: u64 = 0x3;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; r9 = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r8 = ((r8.wrapping_add(P1)) as u64) as u64; r8 = ((rax.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rax = P3; rax = (if (((fl_l as u64)) < ((fl_r as u64))) { r8 } else { rax }) as u64; bb = 5; continue; }
            2 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffd8f0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(r9.wrapping_add(P2))) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; bb = 5; continue; }
            4 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; fl_r = 0x7; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P5))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = 0x1; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            8 => { rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142117070(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xffffffffffffffe0; const P2: u64 = 0xffffffffffffffd0; const P3: u64 = 0xffffffffffffffe8; const P4: u64 = 0x58; const P5: u64 = 0x10; const P6: u64 = 0x1; const P7: u64 = 0x18; const P8: u64 = 0x21; const P9: u64 = 0x20; const P10: u64 = 0x14210c5f0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r9 = rcx; r10 = 0x1; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x9; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { r10 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::lk_add_u64(rcx.wrapping_add(P0), (0xffffffffffffffe8u64) as u64); rax = ((r10 as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            3 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P4), rax); bb = 5; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P5), rax); bb = 5; continue; }
            5 => { r11 = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; rcx = ((crate::simd_ops::ru8(r11.wrapping_add(0x1u64))) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(r9.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P6))) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { r8 = (crate::simd_ops::ru64(r9.wrapping_add(P7))) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(P8))) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_sub(1)) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) <= ((fl_r as u8))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = ((crate::simd_ops::ru8(r8.wrapping_add(P9))) as u32) as u32 as u64; rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0xf8u8))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x3u8))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) | ((rcx as u8)))) as u8 as u64)); crate::simd_ops::wu8(r11.wrapping_add(0x1u64), ((rax as u8)) as u8); rax = ((r10 as u32)) as u32 as u64; crate::simd_ops::lk_add_u64(r9.wrapping_add(P0), (0xffffffffffffffe8u64) as u64); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            9 => { rcx = r8; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { crate::simd_ops::lk_add_u64(r9.wrapping_add(P0), (0xffffffffffffffe8u64) as u64); rax = ((r10 as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142117120(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x40; const P2: u64 = 0x48; const P3: u64 = 0x14210c770; const P4: u64 = 0x1; const P5: u64 = 0x30; const P6: u64 = 0x68; const P7: u64 = 0x8; const P8: u64 = 0x10; const P9: u64 = 0x1420ebca0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rcx; rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P4))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(P4), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(P5))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P6), rax); crate::simd_ops::wu64(rcx.wrapping_add(P5), rbx); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P7))) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rbx.wrapping_add(P8)))) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rcx = rbx; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; r8 = rdx; r8 = ((r8).wrapping_shr(63u32)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rdx = 0x2; r8 = (((r8 as u32)).wrapping_sub(1)) as u32 as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_add_u64(rbx.wrapping_add(P7), (0xffffffffffffffe8u64) as u64); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421171a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1; const P1: u64 = 0x18; const P2: u64 = 0x30; const P3: u64 = 0x68; const P4: u64 = 0x8; const P5: u64 = 0x10; const P6: u64 = 0xa70;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    r8 = rcx;
    fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rcx.wrapping_add(P0), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P3), rax); crate::simd_ops::wu64(rdx.wrapping_add(P2), rcx); }
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64;
    crate::simd_ops::wu64(rax, r8);
    crate::simd_ops::wu32(rax.wrapping_add(P5), (0x9u32) as u32);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::lk_add_u64(rcx.wrapping_add(P4), (0x18u64) as u64);
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P6))) as u64; fl_r = r8;
    rax = (rax & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421171f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x3; const P1: u64 = 0x40; const P2: u64 = 0x20; const P3: u64 = 0x10; const P4: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = rdx; fl_r = rcx;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { rax = 0; fl_l = 0; fl_r = 0; return rax; }
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64;
    fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x6u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { rax = 0x2; return rax; } fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rax = 0x4; return rax; } rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; fl_r = rax; if !((((fl_l as u64)) != ((fl_r as u64)))) { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; rax = 0x1; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64; fl_r = rcx; r8 = 0x3; rax = (if (((fl_l as u64)) == ((fl_r as u64))) { (r8 as u32) } else { (rax as u32) }) as u32 as u64; return rax; } rax = 0x2; return rax; }
    rax = 0x1;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142117250(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1; const P1: u64 = 0x18; const P2: u64 = 0x30; const P3: u64 = 0x68; const P4: u64 = 0x10; const P5: u64 = 0x1429868d0; const P6: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64;
    r9 = rcx;
    fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rdx.wrapping_add(P0), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(r8.wrapping_add(P2))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P3), rax); crate::simd_ops::wu64(r8.wrapping_add(P2), rdx); }
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64;
    rcx = P5;
    fl_l = rax; fl_r = (crate::simd_ops::ru64(r9.wrapping_add(P6))) as u64;
    rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rcx }) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P6))) as u64;
    xmm0 = (crate::simd_ops::ru128(rcx)) as u128;
    crate::simd_ops::wu128(rax, (xmm0) as u128);
    xmm1 = ((crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64) as u128;
    crate::simd_ops::wu64(rax.wrapping_add(P4), ((xmm1 as u64)) as u64);
    fl_l = (crate::simd_ops::lk_add_u64(rdx.wrapping_add(P6), (0x18u64) as u64)) as u64; fl_r = fl_l;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421172b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14210c770; const P1: u64 = 0x142107140; const P2: u64 = 0x1421042d0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; rbx = rcx; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x40u64))) as u64; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x48u64))) as u64; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x1u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rbx); bb = 4; continue; }
            4 => { r8 = ((crate::simd_ops::ru8(rbx.wrapping_add(0x4u64))) as u32) as u32 as u64; rdx = 0x80; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rdi = rax; rdx = ((crate::simd_ops::ru8(rcx.wrapping_add(0x20u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rax, (0x9u8) as u8); rdx = (rdx & !0xffu64) | ((((((rdx as u8)) & (0x3u8))) as u8 as u64)); crate::simd_ops::wu8(rax.wrapping_add(0x1u64), ((rdx as u8)) as u8); rdx = rbx; rcx = ((crate::simd_ops::ru8(rbx.wrapping_add(0x4u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rax.wrapping_add(0x2u64), ((rcx as u8)) as u8); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rcx); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(0x3u64), ((rax as u32)) as u32); rcx = rdi; crate::simd_ops::wu64(rdi.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rdi.wrapping_add(0x48u64), rax); crate::simd_ops::wu64(rdi.wrapping_add(0x58u64), rax); crate::simd_ops::wu64(rdi.wrapping_add(0x60u64), rax); crate::simd_ops::wu64(rdi.wrapping_add(0x50u64), rax); crate::simd_ops::wu64(rdi.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rdi.wrapping_add(0x40u64), rax); crate::simd_ops::wu64(rdi.wrapping_add(0x70u64), rax); crate::simd_ops::wu64(rdi.wrapping_add(0x78u64), rax); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x4u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x4u64), ((rax as u8)) as u8); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x58u64), rax); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x6u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x6u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rax, rdi); crate::simd_ops::wu32(rax.wrapping_add(0x10u64), (0x9u32) as u32); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0x18u64) as u64); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_add(0x1060u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = rdi; rcx = rbx; rax = crate::simd_ops::call_stk(r8, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = rdi; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421173b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14210d870;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rdx = 0x2; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1421173be, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421173d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x8; const P2: u64 = 0x48; const P3: u64 = 0x14210d8d0; const P4: u64 = 0x20; const P5: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rcx; r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rcx.wrapping_add(P1)))) as u64; rdi = ((((rdx as u32)) as i32 as i64) as u64) as u64; rax = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; rax = (rax as u32 as i32 as i64) as u64; fl_l = r8; fl_r = rax; if ((((fl_l as u64)) as i64) > (((fl_r as u64)) as i64)) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; rdx = ((rax.wrapping_add(rax)) as u32) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = ((rax.wrapping_add(rdi)) as u32) as u32 as u64; bb = 3; continue; }
            3 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rcx = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rax.wrapping_add(P5), rdx); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142117440(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1; const P1: u64 = 0x18; const P2: u64 = 0x30; const P3: u64 = 0x68; const P4: u64 = 0x8; const P5: u64 = 0x10;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x8u64)) as u64; r10 = rcx; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 1; continue; }
            1 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64; crate::simd_ops::wu64(rsp, rbx); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { r9 = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rdx.wrapping_add(P0), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(r9.wrapping_add(P2))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P3), rax); crate::simd_ops::wu64(r9.wrapping_add(P2), rdx); bb = 3; continue; }
            3 => { r9 = (crate::simd_ops::ru64(r10.wrapping_add(P4))) as u64; r11 = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64; rcx = ((((r8 as u32)) as i32 as i64) as u64) as u64; rax = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rbx = ((rax.wrapping_mul(8)) as u64) as u64; r9 = ((r9).wrapping_sub(rbx)) as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { r8 = r9; rax = r11; fl_l = r8; fl_r = r11; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 5; continue; }
            5 => { xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rax))) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(r8.wrapping_add(rax).wrapping_add(P5))) as u64) as u128; crate::simd_ops::wu64(rax.wrapping_add(P5), ((xmm1 as u64)) as u64); rax = ((rax.wrapping_add(P1)) as u64) as u64; fl_l = rcx; fl_r = (0x1u64) as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { rax = ((rbx.wrapping_add(r11)) as u64) as u64; crate::simd_ops::wu64(r10.wrapping_add(P4), r9); rbx = (crate::simd_ops::ru64(rsp)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P4), rax); bb = 7; continue; }
            7 => { rsp = ((rsp).wrapping_add(0x8u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421174d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0xfffffffffffffffd; const P2: u64 = 0x8; const P3: u64 = 0x1429868d0; const P4: u64 = 0x142116d40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; r9 = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r8 = ((r8.wrapping_add(P1)) as u64) as u64; r8 = ((rax.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rax = P3; rax = (if (((fl_l as u64)) < ((fl_r as u64))) { r8 } else { rax }) as u64; bb = 5; continue; }
            2 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffd8f0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(r9.wrapping_add(P2))) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; bb = 5; continue; }
            4 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; fl_r = 0x9; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            7 => { rax = (crate::simd_ops::ru64(rax)) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142117540(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    crate::simd_ops::wu128(rax, (xmm0) as u128);
    xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64) as u128;
    crate::simd_ops::wu64(rax.wrapping_add(P1), ((xmm1 as u64)) as u64);
    fl_l = (crate::simd_ops::lk_add_u64(rcx.wrapping_add(P0), (0x18u64) as u64)) as u64; fl_r = fl_l;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142117570(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1429868d0; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x142116d40; const P4: u64 = 0xffffffffffffffff; const P5: u64 = 0x1420ec240;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = ((((r8 as u32)) as i32 as i64) as u64) as u64; r10 = rcx; rdi = P0; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; r9 = ((rcx.wrapping_add(P1)) as u64) as u64; r11 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; r11 = r11.wrapping_sub(0x3); r11 = ((rax.wrapping_add(r11.wrapping_mul(8))) as u64) as u64; fl_l = r11; fl_r = (crate::simd_ops::ru64(r9)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 2; continue; }
            2 => { r11 = rdi; bb = 6; continue; }
            3 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffd8f0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; r9 = ((rcx.wrapping_add(P1)) as u64) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(r9)) as u64; r11 = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; bb = 6; continue; }
            5 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r11 = rax; r9 = ((r10.wrapping_add(P1)) as u64) as u64; bb = 6; continue; }
            6 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(r10.wrapping_add(P2))) as u64; r8 = ((rbx.wrapping_add(P4)) as u64) as u64; r8 = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; rax = rdi; r8 = ((rcx.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = (crate::simd_ops::ru64(r9)) as u64; rax = (if (((fl_l as u64)) < ((fl_r as u64))) { r8 } else { rax }) as u64; bb = 11; continue; }
            8 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0xffffd8f0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(r9)) as u64; rcx = ((rbx.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; bb = 11; continue; }
            10 => { rdx = ((rbx as u32)) as u32 as u64; rcx = r10; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { fl_l = r11; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 12; continue; }
            12 => { fl_l = rax; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { r8 = rax; rdx = r11; rcx = r10; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P5, rax, rcx, rdx, r8, r9, rsp); }
            14 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142117650(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x28; const P3: u64 = 0x48; const P4: u64 = 0x14210d8d0; const P5: u64 = 0x20;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r9 = ((((rdx as u32)) as i32 as i64) as u64) as u64; rbx = rcx; fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x1f40; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 10; continue; } bb = 1; continue; }
            1 => { r10 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; r8 = r10; rdi = r9; r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rcx.wrapping_add(P1)))) as u64; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rdx = ((rdx).wrapping_add(r9)) as u64; fl_l = rdx; fl_r = (0x1f40u64) as u64; if ((((fl_l as u64)) as i64) > (((fl_r as u64)) as i64)) { bb = 10; continue; } bb = 2; continue; }
            2 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 9; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rax = ((r9.wrapping_add(r9.wrapping_mul(2))) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; rcx = ((rcx).wrapping_sub(r10)) as u64; rax = (rax as u32 as i32 as i64) as u64; fl_l = rcx; fl_r = rax; if ((((fl_l as u64)) as i64) > (((fl_r as u64)) as i64)) { bb = 7; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; rcx = rbx; rdx = ((rax.wrapping_add(rax)) as u32) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = ((rax.wrapping_add(r9)) as u32) as u32 as u64; bb = 6; continue; }
            6 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rdx = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rax = 0x1; r8 = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 11; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rcx.wrapping_add(P1), r8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            9 => { rax = 0x1; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            10 => { rax = 0; fl_l = 0; fl_r = 0; bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142117720(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1429868d0; const P2: u64 = 0x8; const P3: u64 = 0x4;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdx = P1;
    fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64;
    rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64;
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    rax = ((crate::simd_ops::ru8(rax.wrapping_add(P3))) as u32) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142117750(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1429868d0; const P1: u64 = 0x10; const P2: u64 = 0xfffffffffffffffd; const P3: u64 = 0x8; const P4: u64 = 0x142116d40;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; r10 = P0; r9 = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; r8 = ((r8.wrapping_add(P2)) as u64) as u64; r8 = ((rax.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; rax = r10; fl_l = r8; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; rax = (if (((fl_l as u64)) < ((fl_r as u64))) { r8 } else { rax }) as u64; rcx = 0; fl_l = 0; fl_r = 0; fl_l = rax; fl_r = r10; rax = (if (((fl_l as u64)) == ((fl_r as u64))) { rcx } else { rax }) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            2 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffd8f0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(r9.wrapping_add(P3))) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rcx = 0; fl_l = 0; fl_r = 0; fl_l = rax; fl_r = r10; rax = (if (((fl_l as u64)) == ((fl_r as u64))) { rcx } else { rax }) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            4 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = 0; fl_l = 0; fl_r = 0; fl_l = rax; fl_r = r10; rax = (if (((fl_l as u64)) == ((fl_r as u64))) { rcx } else { rax }) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421177d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x40; const P2: u64 = 0x48; const P3: u64 = 0x14210c770; const P4: u64 = 0x1; const P5: u64 = 0x30; const P6: u64 = 0x68; const P7: u64 = 0x142105b10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rbx = rcx;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; fl_r = rax;
    if !((((fl_l as u64)) < ((fl_r as u64)))) { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P4))) as u32) as u32 as u64;
    fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(P4), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(P5))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P6), rax); crate::simd_ops::wu64(rcx.wrapping_add(P5), rbx); }
    rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    r8 = rsp.wrapping_add(0x40);
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142117840(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1429868d0; const P2: u64 = 0x8; const P3: u64 = 0x3;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdx = P1;
    fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64;
    rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64;
    fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = 0x8;
    if (((fl_l as u32)) != ((fl_r as u32))) { rax = 0; fl_l = 0; fl_r = 0; return rax; }
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P3))) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { fl_l = ((rax).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; return rax; }
    rax = 0;
    fl_l = 0; fl_r = 0;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142117870(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x40; const P2: u64 = 0x48; const P3: u64 = 0x14210c770; const P4: u64 = 0x1; const P5: u64 = 0x30; const P6: u64 = 0x68; const P7: u64 = 0x1420f8360; const P8: u64 = 0x8; const P9: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = ((r8 as u32)) as u32 as u64; rsi = rdx; rbx = rcx; rax = (crate::simd_ops::ru64(r9.wrapping_add(P1))) as u64; fl_l = (crate::simd_ops::ru64(r9.wrapping_add(P2))) as u64; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P4))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(P4), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(P5))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P6), rax); crate::simd_ops::wu64(rcx.wrapping_add(P5), rbx); bb = 4; continue; }
            4 => { r8 = ((rdi as u32)) as u32 as u64; rdx = rsi; rcx = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P8))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu32(rcx.wrapping_add(P9), (0x8u32) as u32); crate::simd_ops::lk_add_u64(rbx.wrapping_add(P8), (0x18u64) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = ((rax).wrapping_add(0x10u64)) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142117900(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1429868d0; const P1: u64 = 0x142116d40; const P2: u64 = 0x14210c5f0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x1u64))) as u32) as u32 as u64; r10 = rcx; r11 = ((((rdx as u32)) as i32 as i64) as u64) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rcx.wrapping_add(0x1u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(r8.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(r8.wrapping_add(0x30u64), rcx); bb = 2; continue; }
            2 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 11; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rdx = r11.wrapping_sub(0x1); rdx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; r9 = rcx.wrapping_add(0x8); rdx = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(r9)) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = P0; bb = 5; continue; }
            5 => { xmm0 = (crate::simd_ops::ru128(rax.wrapping_sub(0x18u64))) as u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_sub(0x8u64))) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); fl_l = ((r11 as u32)) as u32 as u64; fl_r = 0xffffd8ee; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 21; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(r9)) as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_sub(0x8u64))) as u32 as u64; fl_r = 0x5; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 21; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(r10.wrapping_add(0x20u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; r11 = (crate::simd_ops::ru64(rcx)) as u64; rcx = ((crate::simd_ops::ru8(r11.wrapping_add(0x1u64))) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 21; continue; } bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rdx.wrapping_sub(0x18u64))) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x1u64))) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 21; continue; } bb = 9; continue; }
            9 => { r8 = (crate::simd_ops::ru64(r10.wrapping_add(0x18u64))) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x21u64))) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_sub(1)) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) <= ((fl_r as u8))) { bb = 20; continue; } bb = 10; continue; }
            10 => { rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x20u64))) as u32) as u32 as u64; rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0xf8u8))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x3u8))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) | ((rcx as u8)))) as u8 as u64)); crate::simd_ops::wu8(r11.wrapping_add(0x1u64), ((rax as u8)) as u8); crate::simd_ops::lk_add_u64(r9, (0xffffffffffffffe8u64) as u64); rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            11 => { fl_l = ((r11 as u32)) as u32 as u64; fl_r = 0xffffd8f0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; r9 = rcx.wrapping_add(0x8); rcx = ((r11.wrapping_add(r11.wrapping_mul(2))) as u64) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; bb = 5; continue; }
            13 => { rdx = ((r11 as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = r10.wrapping_add(0x8); rdx = rax; fl_l = ((r11 as u32)) as u32 as u64; fl_r = 0xffffd8ef; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 22; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(r10.wrapping_add(0x20u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(r9)) as u64; rdx = (crate::simd_ops::ru64(rcx)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_sub(0x18u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), rcx); r11 = (crate::simd_ops::ru64(r9)) as u64; fl_l = (crate::simd_ops::ru32(r11.wrapping_sub(0x8u64))) as u32 as u64; fl_r = 0x5; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 21; continue; } bb = 15; continue; }
            15 => { rcx = ((crate::simd_ops::ru8(rdx.wrapping_add(0x1u64))) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 21; continue; } bb = 16; continue; }
            16 => { r11 = (crate::simd_ops::ru64(r11.wrapping_sub(0x18u64))) as u64; fl_l = (crate::simd_ops::ru8(r11.wrapping_add(0x1u64))) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 21; continue; } bb = 17; continue; }
            17 => { r8 = (crate::simd_ops::ru64(r10.wrapping_add(0x18u64))) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x21u64))) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_sub(1)) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) <= ((fl_r as u8))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x20u64))) as u32) as u32 as u64; rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0xf8u8))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x3u8))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) | ((rcx as u8)))) as u8 as u64)); crate::simd_ops::wu8(rdx.wrapping_add(0x1u64), ((rax as u8)) as u8); crate::simd_ops::lk_add_u64(r9, (0xffffffffffffffe8u64) as u64); rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            19 => { rdx = r11; bb = 20; continue; }
            20 => { rcx = r8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 21; continue; }
            21 => { crate::simd_ops::lk_add_u64(r9, (0xffffffffffffffe8u64) as u64); rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            22 => { rax = (crate::simd_ops::ru64(r9)) as u64; fl_l = ((r11 as u32)) as u32 as u64; fl_r = 0xffffd8ee; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru64(rax.wrapping_sub(0x18u64))) as u64; crate::simd_ops::lk_add_u64(r9, (0xffffffffffffffe8u64) as u64); crate::simd_ops::wu64(r10.wrapping_add(0x58u64), rax); rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142117aa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1; const P1: u64 = 0x18; const P2: u64 = 0x30; const P3: u64 = 0x68; const P4: u64 = 0x10; const P5: u64 = 0x8; const P6: u64 = 0x1429868d0; const P7: u64 = 0x142116d40; const P8: u64 = 0xaa0;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64; r10 = rcx; r9 = ((((rdx as u32)) as i32 as i64) as u64) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rcx.wrapping_add(P0), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(r8.wrapping_add(P2))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P3), rax); crate::simd_ops::wu64(r8.wrapping_add(P2), rcx); bb = 2; continue; }
            2 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(r10.wrapping_add(P4))) as u64; rcx = ((r9.wrapping_add(r9.wrapping_mul(2))) as u64) as u64; rdx = ((r10.wrapping_add(P5)) as u64) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = ((rax).wrapping_add(0xffffffffffffffe8u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rdx)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 4; continue; }
            4 => { rax = P6; bb = 8; continue; }
            5 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0xffffd8f0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P5))) as u64; rdx = ((rcx.wrapping_add(P5)) as u64) as u64; rcx = ((r9.wrapping_add(r9.wrapping_mul(2))) as u64) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; bb = 8; continue; }
            7 => { rdx = ((r9 as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rdx = ((r10.wrapping_add(P5)) as u64) as u64; bb = 8; continue; }
            8 => { rcx = (((crate::simd_ops::ru32(rax.wrapping_add(P4))) as i32 as i64) as u64) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x6; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 9; continue; }
            9 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x8; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(r10.wrapping_add(P1))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P8))) as u64; bb = 13; continue; }
            11 => { rax = (crate::simd_ops::ru64(rax)) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; bb = 13; continue; }
            12 => { rax = (crate::simd_ops::ru64(rax)) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; bb = 13; continue; }
            13 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rax, r8); crate::simd_ops::wu32(rax.wrapping_add(P4), (0x6u32) as u32); fl_l = (crate::simd_ops::lk_add_u64(rdx, (0x18u64) as u64)) as u64; fl_r = fl_l; bb = 15; continue; }
            15 => { rax = 0; fl_l = 0; fl_r = 0; fl_l = r8; fl_r = r8; rax = (rax & !0xffu64) | (((if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142117b70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1; const P1: u64 = 0x18; const P2: u64 = 0x30; const P3: u64 = 0x68; const P4: u64 = 0x8; const P5: u64 = 0x142103330; const P6: u64 = 0xffffffffffffffe8; const P7: u64 = 0x1420fcb60; const P8: u64 = 0x10; const P9: u64 = 0xfffffffffffffff8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64; r9 = r8; rbx = rcx; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rcx.wrapping_add(P0), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P3), rax); crate::simd_ops::wu64(rdx.wrapping_add(P2), rcx); bb = 2; continue; }
            2 => { rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64; r8 = (0xffffffffffffffffu64) as u64; bb = 3; continue; }
            3 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r9.wrapping_add(r8))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rdx = r9; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P6))) as u64; rdx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(P8))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(P8), ((xmm1 as u64)) as u64); crate::simd_ops::lk_add_u64(rbx.wrapping_add(P4), (0x18u64) as u64); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P9))) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142117c20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1; const P1: u64 = 0x18; const P2: u64 = 0x30; const P3: u64 = 0x68; const P4: u64 = 0x8; const P5: u64 = 0x10; const P6: u64 = 0x1429868d0; const P7: u64 = 0x142116d40; const P8: u64 = 0x1420ec940; const P9: u64 = 0xfffffffffffffff8;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64; r10 = rcx; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rcx.wrapping_add(P0), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(r8.wrapping_add(P2))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P3), rax); crate::simd_ops::wu64(r8.wrapping_add(P2), rcx); bb = 2; continue; }
            2 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; rbx = ((r10.wrapping_add(P4)) as u64) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(r10.wrapping_add(P5))) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = ((rax).wrapping_add(0xffffffffffffffe8u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 4; continue; }
            4 => { rax = P6; bb = 8; continue; }
            5 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffd8f0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; rbx = ((rcx.wrapping_add(P4)) as u64) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; bb = 8; continue; }
            7 => { rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rbx = ((r10.wrapping_add(P4)) as u64) as u64; bb = 8; continue; }
            8 => { r8 = (crate::simd_ops::ru64(rbx)) as u64; rdx = rax; r8 = ((r8).wrapping_add(0xffffffffffffffe8u64)) as u64; rcx = r10; r9 = r8; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P9))) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142117cc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140000000; const P1: u64 = 0x14210ab30; const P2: u64 = 0x14210b360; const P3: u64 = 0x142103040; const P4: u64 = 0x14210c770;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; rbp = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdi = (rcx) as u64; rsi = ((rbp as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x9u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 41; continue; } bb = 1; continue; }
            1 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; rdx = ((P0) as u64) as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x2117f54u64))) as u32 as u64; fl_l = ((rcx).wrapping_add(rdx)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; j_tgt = (rcx) as u64; bb = 0xffff_fffe; continue; }
            2 => { crate::simd_ops::wu64(rbx.wrapping_add(0x40u64), (0xffffffffffffffffu64) as u64); bb = 42; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x40u64), (rax) as u64); bb = 42; continue; }
            4 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x21u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 9; continue; } bb = 5; continue; }
            5 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 9; continue; } bb = 6; continue; }
            6 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 9; continue; } bb = 7; continue; }
            7 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { bb = 10; continue; }
            9 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x260u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x268u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), (rbp) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x30u64), (rbp) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), (rbp) as u64); crate::simd_ops::wu8(rbx.wrapping_add(0x21u64), (0x4u8) as u8); bb = 10; continue; }
            10 => { rdx = (0xffffffffffffffffu64) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x21u64))) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 10; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0xa90u64))) as u64; rcx = ((rbx.wrapping_add(0xa78u64)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 12; continue; }
            12 => { bb = 13; continue; }
            13 => { crate::simd_ops::wu8(rax.wrapping_add(0x3u64), ((rsi as u8)) as u8); rax = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 14; continue; }
            14 => { rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x21u64))) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 17; continue; } bb = 15; continue; }
            15 => { bb = 16; continue; }
            16 => { rdx = (0xffffffffffffffffu64) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x21u64))) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 16; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; r9 = (crate::simd_ops::ru32(rax.wrapping_add(0xcu64))) as u32 as u64; rcx = ((r9 as u32)) as u32 as u64; r8 = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; rax = ((r9 as u32)) as u32 as u64; rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64; rdx = ((((rdx as u32)) & (0x3u32))) as u32 as u64; rax = (((rax as u32)).wrapping_add((rdx as u32))) as u32 as u64; rax = (((((rax as u32)) as i32).wrapping_shr(2u32) as u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 22; continue; } bb = 18; continue; }
            18 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x40u32) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 20; continue; } bb = 19; continue; }
            19 => { rax = ((rcx as u32)) as u32 as u64; rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64; rax = (((rax as u32)).wrapping_sub((rdx as u32))) as u32 as u64; rax = (((((rax as u32)) as i32).wrapping_shr(1u32) as u32)) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64; rdx = ((((rdx as u32)) & (0x3u32))) as u32 as u64; rax = (((rax as u32)).wrapping_add((rdx as u32))) as u32 as u64; rax = (((((rax as u32)) as i32).wrapping_shr(2u32) as u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 18; continue; } bb = 20; continue; }
            20 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 22; continue; } bb = 21; continue; }
            21 => { rdx = ((rcx as u32)) as u32 as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 22; continue; }
            22 => { r11 = (crate::simd_ops::ru64(rbx.wrapping_add(0x48u64))) as u64; rax = (0x47ae147ae147ae15u64) as u64; r8 = (((crate::simd_ops::ru32(rbx.wrapping_add(0x54u64))) as i32 as i64) as u64) as u64; r10 = (r11) as u64; r9 = (((crate::simd_ops::ru32(rbx.wrapping_add(0x50u64))) as i32 as i64) as u64) as u64; rcx = ((r8 as u32)) as u32 as u64; let prod = (rax as u128).wrapping_mul((r11) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rcx = ((((rcx as u32)) as i32).wrapping_mul(((r9 as u32)) as i32) as u32) as u32 as u64; rax = (0x51eb851fu32) as u32 as u64; r10 = ((r10).wrapping_sub(rdx)) as u64; r10 = ((r10).wrapping_shr(1u32)) as u64; r10 = ((r10).wrapping_add(rdx)) as u64; r10 = ((r10).wrapping_shr(6u32)) as u64; let prod = (rax as u32 as i32 as i64).wrapping_mul(((rcx as u32)) as i32 as i64) as u64; rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; r10 = (((r10) as i64).wrapping_mul((r9) as i64) as u64) as u64; rdx = (((((rdx as u32)) as i32).wrapping_shr(5u32) as u32)) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add(0xffffff9cu32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(31u32)) as u32 as u64; rax = (((rax as u32)).wrapping_add((rdx as u32))) as u32 as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rax = (rax as u32 as i32 as i64) as u64; rax = (((rax) as i64).wrapping_mul((r11) as i64) as u64) as u64; let d = (r8) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } crate::simd_ops::wu64(rbx.wrapping_add(0x40u64), (rax) as u64); fl_l = (rax) as u64 as u64; fl_r = (r11) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 24; continue; } bb = 23; continue; }
            23 => { crate::simd_ops::wu64(rbx.wrapping_add(0x40u64), (r11) as u64); bb = 24; continue; }
            24 => { crate::simd_ops::wu64(rbx.wrapping_add(0x1128u64), (r10) as u64); bb = 42; continue; }
            25 => { rsi = (crate::simd_ops::ru64(rbx.wrapping_add(0x48u64))) as u64; fl_l = ((rsi).wrapping_shr(10u32)) as u64 as u64; rsi = ((fl_l as u64)) as u64; bb = 42; continue; }
            26 => { rsi = (crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32 as u64; fl_l = ((((rsi as u32)) & (0x3ffu32))) as u32 as u64; fl_r = fl_l; rsi = ((fl_l as u32)) as u32 as u64; bb = 42; continue; }
            27 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64 as u64; fl_r = (0xffffffffffffffffu64) as u64 as u64; rbp = (rbp & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rsi = ((rbp as u32)) as u32 as u64; bb = 42; continue; }
            28 => { r9 = ((((r8 as u32)) as i32 as i64) as u64) as u64; r9 = ((r9).wrapping_shl(10u32)) as u64; fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x21u64))) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 30; continue; } bb = 29; continue; }
            29 => { r15 = (rbp) as u64; bb = 31; continue; }
            30 => { r15 = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; fl_l = (r15) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x48u64))) as u64 as u64; r15 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 31; continue; }
            31 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x48u64))) as u64; rax = (rbp) as u64; rcx = (rdx) as u64; r14 = (rbp) as u64; rcx = ((rcx).wrapping_sub(r9)) as u64; fl_l = (r9) as u64 as u64; fl_r = (rdx) as u64 as u64; rax = (if (((fl_l as u64)) <= ((fl_r as u64))) { rcx } else { rax }) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x40u64), (rax) as u64); fl_l = (rax) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 36; continue; } bb = 32; continue; }
            32 => { rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r14 = ((r14).wrapping_add(rax)) as u64; fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x21u64))) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 35; continue; } bb = 33; continue; }
            33 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x48u64))) as u64; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 32; continue; } bb = 34; continue; }
            34 => { bb = 37; continue; }
            35 => { rsi = (0x1u32) as u32 as u64; bb = 42; continue; }
            36 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x21u64))) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 42; continue; } bb = 37; continue; }
            37 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x48u64))) as u64; rax = ((rax).wrapping_add(r14)) as u64; fl_l = ((rax).wrapping_add(r15)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; rax = (if ((((fl_l as u64)) as i64) < 0) { rbp } else { rax }) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x40u64), (rax) as u64); bb = 42; continue; }
            38 => { rsi = (crate::simd_ops::ru32(rbx.wrapping_add(0x50u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x50u64), ((r8 as u32)) as u32); bb = 42; continue; }
            39 => { rsi = (crate::simd_ops::ru32(rbx.wrapping_add(0x54u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x54u64), ((r8 as u32)) as u32); bb = 42; continue; }
            40 => { rsi = (crate::simd_ops::ru32(rbx.wrapping_add(0x58u64))) as u32 as u64; rsi = (((((rsi as u32)) as i32).wrapping_shr(10u32) as u32)) as u32 as u64; fl_l = (((r8 as u32)).wrapping_shl(10u32)) as u32 as u64; r8 = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x58u64), ((r8 as u32)) as u32); bb = 42; continue; }
            41 => { rsi = (0xffffffffu32) as u32 as u64; bb = 42; continue; }
            42 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = ((rsi as u32)) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x142117cc0u64) { 0x2c => { bb = 1; continue; } 0x42 => { bb = 2; continue; } 0x4f => { bb = 3; continue; } 0x5c => { bb = 4; continue; } 0x64 => { bb = 5; continue; } 0x68 => { bb = 6; continue; } 0x6c => { bb = 7; continue; } 0x70 => { bb = 8; continue; } 0x72 => { bb = 9; continue; } 0x90 => { bb = 10; continue; } 0xa5 => { bb = 11; continue; } 0xb8 => { bb = 12; continue; } 0xc0 => { bb = 13; continue; } 0xcd => { bb = 14; continue; } 0xdb => { bb = 15; continue; } 0xe0 => { bb = 16; continue; } 0xf5 => { bb = 17; continue; } 0x115 => { bb = 18; continue; } 0x11a => { bb = 19; continue; } 0x131 => { bb = 20; continue; } 0x136 => { bb = 21; continue; } 0x140 => { bb = 22; continue; } 0x19c => { bb = 23; continue; } 0x1a0 => { bb = 24; continue; } 0x1ac => { bb = 25; continue; } 0x1b9 => { bb = 26; continue; } 0x1c7 => { bb = 27; continue; } 0x1d7 => { bb = 28; continue; } 0x1e4 => { bb = 29; continue; } 0x1e9 => { bb = 30; continue; } 0x1f1 => { bb = 31; continue; } 0x211 => { bb = 32; continue; } 0x222 => { bb = 33; continue; } 0x22c => { bb = 34; continue; } 0x22e => { bb = 35; continue; } 0x235 => { bb = 36; continue; } 0x23b => { bb = 37; continue; } 0x24f => { bb = 38; continue; } 0x258 => { bb = 39; continue; } 0x261 => { bb = 40; continue; } 0x271 => { bb = 41; continue; } 0x276 => { bb = 42; continue; } _ => { let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(j_tgt, rcx, rdx, r8, r9, s5, s6, s7, s8); } } }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142117f90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0xfffffffffffffffd; const P2: u64 = 0x8; const P3: u64 = 0x1429868d0; const P4: u64 = 0x5; const P5: u64 = 0x142116d40;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; r11 = ((r8 as u32)) as u32 as u64; r10 = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r9 = ((r9.wrapping_add(P1)) as u64) as u64; r9 = ((rax.wrapping_add(r9.wrapping_mul(8))) as u64) as u64; fl_l = r9; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rax = P3; rax = (if (((fl_l as u64)) < ((fl_r as u64))) { r9 } else { rax }) as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; rcx = (rcx & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); rax = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu8(rax.wrapping_add(P4), ((rcx as u8)) as u8); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            2 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffd8f0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(r10.wrapping_add(P2))) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rcx = (rcx & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); rax = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu8(rax.wrapping_add(P4), ((rcx as u8)) as u8); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            4 => { rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; rcx = (rcx & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); rax = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu8(rax.wrapping_add(P4), ((rcx as u8)) as u8); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142118020(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x270f; const P1: u64 = 0x8; const P2: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = ((rdx as u32)) as u32 as u64;
    rdx = rcx;
    rax = ((r8.wrapping_add(P0)) as u32) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x270f;
    if !((((fl_l as u32)) > ((fl_r as u32)))) { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdx.wrapping_add(P2)))) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rax = ((rax).wrapping_add(rdx)) as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((r8 as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; return rax; }
    rax = ((r8 as u32)) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142118070(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1; const P1: u64 = 0x18; const P2: u64 = 0x30; const P3: u64 = 0x68; const P4: u64 = 0x8; const P5: u64 = 0x10; const P6: u64 = 0x1429868d0; const P7: u64 = 0x142116d40; const P8: u64 = 0x1420fc210;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64; r10 = rcx; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rcx.wrapping_add(P0), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(r8.wrapping_add(P2))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P3), rax); crate::simd_ops::wu64(r8.wrapping_add(P2), rcx); bb = 2; continue; }
            2 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; rbx = ((r10.wrapping_add(P4)) as u64) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(r10.wrapping_add(P5))) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = ((rax).wrapping_add(0xffffffffffffffe8u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 4; continue; }
            4 => { rax = P6; bb = 8; continue; }
            5 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffd8f0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; rbx = ((rcx.wrapping_add(P4)) as u64) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; bb = 8; continue; }
            7 => { rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rbx = ((r10.wrapping_add(P4)) as u64) as u64; bb = 8; continue; }
            8 => { r8 = (crate::simd_ops::ru64(rbx)) as u64; rcx = r10; rdx = (crate::simd_ops::ru64(rax)) as u64; r8 = ((r8).wrapping_sub(0x18u64)) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rcx = 0x18; rdx = (0xffffffffffffffe8u64) as u64; rcx = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { rdx } else { rcx }) as u64; crate::simd_ops::lk_add_u64(rbx, (rcx) as u64); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142118120(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x1429868d0; const P3: u64 = 0x142116d40; const P4: u64 = 0x18;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; r10 = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; rax = ((rax).wrapping_add(0xffffffffffffffe8u64)) as u64; rdx = ((rcx.wrapping_add(P1)) as u64) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rdx)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 2; continue; }
            2 => { rax = P2; bb = 6; continue; }
            3 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffffd8f0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = ((rcx.wrapping_add(P1)) as u64) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; bb = 6; continue; }
            5 => { rdx = ((rax as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = ((r10.wrapping_add(P1)) as u64) as u64; bb = 6; continue; }
            6 => { r8 = (crate::simd_ops::ru64(rdx)) as u64; rcx = rax; rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(P0), ((xmm1 as u64)) as u64); r8 = (crate::simd_ops::ru64(rdx)) as u64; rcx = ((rcx.wrapping_add(P4)) as u64) as u64; rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 8; continue; }
            8 => { r8 = ((r8).wrapping_add(0xffffffffffffffe8u64)) as u64; crate::simd_ops::wu64(rdx, r8); rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421181c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1; const P1: u64 = 0x18; const P2: u64 = 0x30; const P3: u64 = 0x68; const P4: u64 = 0x8; const P5: u64 = 0x10; const P6: u64 = 0x1429868d0; const P7: u64 = 0x142116d40; const P8: u64 = 0xffffffffffffffe8; const P9: u64 = 0x1420fd0a0; const P10: u64 = 0xfffffffffffffff8;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64; r10 = rcx; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rcx.wrapping_add(P0), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(r8.wrapping_add(P2))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P3), rax); crate::simd_ops::wu64(r8.wrapping_add(P2), rcx); bb = 2; continue; }
            2 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; rdi = ((r10.wrapping_add(P4)) as u64) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(r10.wrapping_add(P5))) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = ((rax).wrapping_add(0xffffffffffffffe8u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rdi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 4; continue; }
            4 => { rax = P6; bb = 8; continue; }
            5 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffd8f0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; rdi = ((rcx.wrapping_add(P4)) as u64) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; bb = 8; continue; }
            7 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((r10.wrapping_add(P4)) as u64) as u64; bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rdi)) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; rdx = ((rbx.wrapping_add(P8)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rbx.wrapping_add(P8), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64) as u128; crate::simd_ops::wu64(rbx.wrapping_add(P10), ((xmm1 as u64)) as u64); rax = (crate::simd_ops::ru64(rdi)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P10))) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142118280(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1429868d0; const P1: u64 = 0x142116d40; const P2: u64 = 0x1420fd0a0; const P3: u64 = 0x1420fceb0; const P4: u64 = 0x14210c5f0; const P5: u64 = 0x14210e620;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r12 = P0; rdi = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; rbx = rcx.wrapping_add(0x8); rsi = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rsi = rsi.wrapping_sub(0x3); rsi = ((rax.wrapping_add(rsi.wrapping_mul(8))) as u64) as u64; fl_l = rsi; fl_r = (crate::simd_ops::ru64(rbx)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 2; continue; }
            2 => { rsi = r12; bb = 6; continue; }
            3 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffd8f0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; rbx = rcx.wrapping_add(0x8); rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rsi = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; bb = 6; continue; }
            5 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; rbx = rdi.wrapping_add(0x8); bb = 6; continue; }
            6 => { rbp = (crate::simd_ops::ru64(rsi)) as u64; fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x4u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 16; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); rcx = rbp; r14 = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r15); rdx = r14.wrapping_sub(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(rbp.wrapping_add(0x3u64), (0x0u8) as u8); fl_l = rax; fl_r = r12; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { r8 = r14.wrapping_sub(0x30); rdx = rbp; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { xmm0 = (crate::simd_ops::ru128(r14.wrapping_sub(0x18u64))) as u128; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu128(rax, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(r14.wrapping_sub(0x8u64))) as u64) as u128; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_sub(0x8u64))) as u32 as u64; fl_r = 0x5; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 15; continue; } bb = 10; continue; }
            10 => { r8 = (crate::simd_ops::ru64(rsi)) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x1u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 15; continue; } bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(rdx.wrapping_sub(0x18u64))) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x1u64))) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 15; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x21u64))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 14; continue; } bb = 13; continue; }
            13 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 15; continue; }
            14 => { fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(r8.wrapping_add(0x1u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x28u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), r8); bb = 15; continue; }
            15 => { crate::simd_ops::lk_add_u64(rbx, (0xffffffffffffffd0u64) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            16 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1421183a7, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421183b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0xfffffffffffffffd; const P3: u64 = 0x1429868d0; const P4: u64 = 0x30; const P5: u64 = 0x142116d40; const P6: u64 = 0x142116cd0; const P7: u64 = 0x14210dbb0; const P8: u64 = 0x20;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r10 = 0; fl_l = 0; fl_r = 0; rsi = ((r8 as u32)) as u32 as u64; r11 = ((rdx as u32)) as u32 as u64; rdi = rcx; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = ((((r9 as u32)) as i32 as i64) as u64) as u64; rbx = ((rcx.wrapping_add(P0)) as u64) as u64; r10 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; r10 = ((r10.wrapping_add(P2)) as u64) as u64; r10 = ((rax.wrapping_add(r10.wrapping_mul(8))) as u64) as u64; fl_l = r10; fl_r = (crate::simd_ops::ru64(rbx)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 3; continue; }
            3 => { r10 = P3; fl_l = r10; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 9; continue; }
            4 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0xffffd8f0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = ((((r9 as u32)) as i32 as i64) as u64) as u64; rbx = ((rcx.wrapping_add(P0)) as u64) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; r10 = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = r10; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 9; continue; }
            6 => { rdx = ((r9 as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); r10 = rax; rbx = ((rdi.wrapping_add(P0)) as u64) as u64; bb = 7; continue; }
            7 => { fl_l = r10; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 9; continue; }
            8 => { rbx = ((rcx.wrapping_add(P0)) as u64) as u64; bb = 9; continue; }
            9 => { r9 = (crate::simd_ops::ru64(rbx)) as u64; rax = ((r11.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; r8 = rsp.wrapping_add(0x30); rdx = P6; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rsi as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r10); rax = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = rdi; rax = ((rax).wrapping_shl(3u32)) as u64; r9 = ((r9).wrapping_sub(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r9); r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(rdi.wrapping_add(P4)))) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P8))) as u64; rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdx; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rcx.wrapping_add(P1), rdx); bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421184a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1429868d0; const P1: u64 = 0x142116d40; const P2: u64 = 0x14210c5f0; const P3: u64 = 0x14210e620;
    let (mut rax, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; r11 = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; r8 = r8.wrapping_sub(0x3); r8 = ((rax.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 2; continue; }
            2 => { r8 = P0; bb = 6; continue; }
            3 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffd8f0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(r11.wrapping_add(0x8u64))) as u64; r8 = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; bb = 6; continue; }
            5 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r8 = rax; bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(r11.wrapping_add(0x8u64))) as u64; rdx = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rax.wrapping_sub(0x8u64))) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rax.wrapping_sub(0x18u64))) as u64; bb = 8; continue; }
            8 => { rax = (((crate::simd_ops::ru32(r8.wrapping_add(0x10u64))) as i32 as i64) as u64) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x6; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 9; continue; }
            9 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x8; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = rax; rax = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0xaa0u64), rdx); rax = 0x1; crate::simd_ops::lk_add_u64(r11.wrapping_add(0x8u64), (0xffffffffffffffe8u64) as u64); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            11 => { rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rdx); bb = 14; continue; }
            12 => { rax = (crate::simd_ops::ru64(r8)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x4u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 21; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rdx); bb = 14; continue; }
            14 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 15; continue; }
            15 => { r9 = (crate::simd_ops::ru64(r8)) as u64; r8 = ((crate::simd_ops::ru8(r9.wrapping_add(0x1u64))) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 20; continue; } bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x1u64))) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 20; continue; } bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x21u64))) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_sub(1)) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) <= ((fl_r as u8))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x20u64))) as u32) as u32 as u64; r8 = (r8 & !0xffu64) | ((((((r8 as u8)) & (0xf8u8))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x3u8))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) | ((r8 as u8)))) as u8 as u64)); crate::simd_ops::wu8(r9.wrapping_add(0x1u64), ((rax as u8)) as u8); rax = 0x1; crate::simd_ops::lk_add_u64(r11.wrapping_add(0x8u64), (0xffffffffffffffe8u64) as u64); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            19 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            20 => { crate::simd_ops::lk_add_u64(r11.wrapping_add(0x8u64), (0xffffffffffffffe8u64) as u64); rax = 0x1; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            21 => { rcx = r11; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1421185a2, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421185b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1429868d0; const P2: u64 = 0x8; const P3: u64 = 0xfffffffffffffffd; const P4: u64 = 0x142116d40; const P5: u64 = 0x142105fa0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r11 = P1; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; r10 = r11; fl_l = rdx; fl_r = r9; r10 = (if (((fl_l as u64)) < ((fl_r as u64))) { rdx } else { r10 }) as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((((r8 as u32)) as i32 as i64) as u64) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((rax.wrapping_add(P3)) as u64) as u64; rax = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rdx = r11; fl_l = rax; fl_r = r9; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; bb = 5; continue; }
            2 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0xffffd8f0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = ((((r8 as u32)) as i32 as i64) as u64) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = ((r9.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; bb = 5; continue; }
            4 => { rdx = ((r8 as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = rax; bb = 5; continue; }
            5 => { fl_l = r10; fl_r = r11; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = rdx; fl_r = r11; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = r10; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return crate::simd_ops::tail_stk(P5, rax, rcx, rdx, r8, r9, rsp); }
            8 => { rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142118640(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1429868d0; const P1: u64 = 0x142116d40; const P2: u64 = 0x1420fcf40; const P3: u64 = 0x1420fb250; const P4: u64 = 0x14210c5f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; rsi = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; rdi = P0; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; r8 = r8.wrapping_sub(0x3); r8 = ((rax.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rdi = (if (((fl_l as u64)) < ((fl_r as u64))) { r8 } else { rdi }) as u64; bb = 5; continue; }
            2 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffd8f0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rdi = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; bb = 5; continue; }
            4 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 17; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = (crate::simd_ops::ru32(r14.wrapping_add(0xc10u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbp); rbp = (crate::simd_ops::ru64(rax.wrapping_add(0xbf8u64))) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; bb = 8; continue; }
            8 => { r8 = ((rbx as u32)) as u32 as u64; rdx = rbp; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(r14.wrapping_add(0xc10u64))) as u32 as u64; fl_r = 0x0; rcx = rax; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::cvt_f64_i32((f64::from_bits(crate::simd_ops::ru64(rax))))) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0xc10u64), ((rax as u32)) as u32); bb = 10; continue; }
            10 => { xmm0 = (crate::simd_ops::ru128(rdi)) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x5; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 15; continue; } bb = 11; continue; }
            11 => { rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x1u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 15; continue; } bb = 12; continue; }
            12 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x1u64))) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 15; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x21u64))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 16; continue; } bb = 14; continue; }
            14 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 15; continue; }
            15 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = ((rbx as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            16 => { fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbp.wrapping_add(0x1u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rbp); bb = 15; continue; }
            17 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142118780(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1; const P1: u64 = 0x8; const P2: u64 = 0x14210dd80; const P3: u64 = 0x20; const P4: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = ((rdx.wrapping_add(P0)) as u32) as u32 as u64;
    rbx = ((r8 as u32)) as u32 as u64;
    rdx = ((((rax as u32)) as i32 as i64) as u64) as u64;
    rdi = rcx;
    rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    rax = ((rax).wrapping_shl(3u32)) as u64;
    rdx = ((rdx).wrapping_sub(rax)) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0xffffffff;
    if (((fl_l as u32)) != ((fl_r as u32))) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64;
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64;
    if !((((fl_l as u64)) < ((fl_r as u64)))) { crate::simd_ops::wu64(rax.wrapping_add(P4), rcx); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421187e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0xbf8; const P2: u64 = 0x1420fb250; const P3: u64 = 0xc10; const P4: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64;
    if !((((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0)) { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r8 = ((rdx as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((crate::simd_ops::ru32(rbx.wrapping_add(P3))) as i32) as f64).to_bits() as u128); crate::simd_ops::wu64(rax, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rax.wrapping_add(P4), (0x3u32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(P3), ((rdi as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); }
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142118840(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1; const P1: u64 = 0x18; const P2: u64 = 0x30; const P3: u64 = 0x68; const P4: u64 = 0x8; const P5: u64 = 0x10; const P6: u64 = 0xfffffffffffffffd; const P7: u64 = 0x1429868d0; const P8: u64 = 0x142116d40; const P9: u64 = 0xffffffffffffffe8;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64; r10 = rcx; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rcx.wrapping_add(P0), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(r8.wrapping_add(P2))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P3), rax); crate::simd_ops::wu64(r8.wrapping_add(P2), rcx); bb = 2; continue; }
            2 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; r9 = ((rcx.wrapping_add(P4)) as u64) as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P5))) as u64; r8 = ((r8.wrapping_add(P6)) as u64) as u64; r8 = ((rax.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = (crate::simd_ops::ru64(r9)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 4; continue; }
            4 => { r8 = P7; bb = 8; continue; }
            5 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffd8f0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; r9 = ((rcx.wrapping_add(P4)) as u64) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(r9)) as u64; r8 = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; bb = 8; continue; }
            7 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rax; r9 = ((r10.wrapping_add(P4)) as u64) as u64; bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(r9)) as u64; fl_l = rcx; fl_r = r8; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rdx = ((rcx.wrapping_add(P9)) as u64) as u64; bb = 10; continue; }
            10 => { xmm0 = (crate::simd_ops::ru128(rdx)) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(P5))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(P5), ((xmm1 as u64)) as u64); rcx = ((rcx).wrapping_sub(0x18u64)) as u64; rdx = ((rdx.wrapping_add(P9)) as u64) as u64; fl_l = rcx; fl_r = r8; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 10; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(r9)) as u64; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(r8, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64) as u128; crate::simd_ops::wu64(r8.wrapping_add(P5), ((xmm1 as u64)) as u64); rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142118910(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1; const P1: u64 = 0x18; const P2: u64 = 0x30; const P3: u64 = 0x68; const P4: u64 = 0x10; const P5: u64 = 0x8; const P6: u64 = 0x1429868d0; const P7: u64 = 0x142116d40;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64; r10 = rcx; r9 = ((((rdx as u32)) as i32 as i64) as u64) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rcx.wrapping_add(P0), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(r8.wrapping_add(P2))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P3), rax); crate::simd_ops::wu64(r8.wrapping_add(P2), rcx); bb = 2; continue; }
            2 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(r10.wrapping_add(P4))) as u64; rcx = ((r9.wrapping_add(r9.wrapping_mul(2))) as u64) as u64; rdx = ((r10.wrapping_add(P5)) as u64) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = ((rax).wrapping_add(0xffffffffffffffe8u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rdx)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 4; continue; }
            4 => { rax = P6; bb = 8; continue; }
            5 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0xffffd8f0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P5))) as u64; rdx = ((rcx.wrapping_add(P5)) as u64) as u64; rcx = ((r9.wrapping_add(r9.wrapping_mul(2))) as u64) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; bb = 8; continue; }
            7 => { rdx = ((r9 as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = ((r10.wrapping_add(P5)) as u64) as u64; bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rdx)) as u64; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(P4), ((xmm1 as u64)) as u64); crate::simd_ops::lk_add_u64(rdx, (0x18u64) as u64); rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421189e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0xfffffffffffffffd; const P2: u64 = 0x8; const P3: u64 = 0x1429868d0; const P4: u64 = 0x142116d40; const P5: u64 = 0x1420ecd00;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x48u64)) as u64; r9 = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r8 = ((r8.wrapping_add(P1)) as u64) as u64; r8 = ((rax.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rax = P3; rax = (if (((fl_l as u64)) < ((fl_r as u64))) { r8 } else { rax }) as u64; bb = 5; continue; }
            2 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffd8f0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(r9.wrapping_add(P2))) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; bb = 5; continue; }
            4 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rdx = rsp.wrapping_add(0x20); rcx = rax; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rsp = ((rsp).wrapping_add(0x48u64)) as u64; return rax; }
            8 => { rax = 0x1; rsp = ((rsp).wrapping_add(0x48u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142118b20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0xfffffffffffffffd; const P2: u64 = 0x8; const P3: u64 = 0x1429868d0; const P4: u64 = 0x142116d40; const P5: u64 = 0x4; const P6: u64 = 0x18; const P7: u64 = 0x1068; const P8: u64 = 0x14;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = r8; r10 = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r9 = ((r9.wrapping_add(P1)) as u64) as u64; r9 = ((rax.wrapping_add(r9.wrapping_mul(8))) as u64) as u64; fl_l = r9; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rax = P3; rax = (if (((fl_l as u64)) < ((fl_r as u64))) { r9 } else { rax }) as u64; bb = 5; continue; }
            2 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffd8f0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(r10.wrapping_add(P2))) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; bb = 5; continue; }
            4 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; fl_r = 0x5; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = (crate::simd_ops::ru64(rax)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 8; continue; }
            8 => { rax = ((crate::simd_ops::ru16(rbx.wrapping_add(P5))) as u32) as u32 as u64; rcx = 0xffff8000; fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rcx as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 13; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(r10.wrapping_add(P6))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_add(P7))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P8))) as u32 as u64; rcx = ((rbx.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(r8, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            11 => { rax = 0xffffffff; bb = 12; continue; }
            12 => { crate::simd_ops::wu16(rbx.wrapping_add(P5), ((rax as u16)) as u16); bb = 13; continue; }
            13 => { rax = (rax as u16 as i16 as i32 as u32) as u64; crate::simd_ops::wu32(rdi, ((rax as u32)) as u32); bb = 14; continue; }
            14 => { rax = ((rbx.wrapping_add(P6)) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142118be0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0xfffffffffffffffd; const P2: u64 = 0x8; const P3: u64 = 0x1429868d0; const P4: u64 = 0x4; const P5: u64 = 0x142116d40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    r9 = rcx;
    fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64;
    if !((((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0)) { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r8 = ((r8.wrapping_add(P1)) as u64) as u64; r8 = ((rax.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rax = P3; rax = (if (((fl_l as u64)) < ((fl_r as u64))) { r8 } else { rax }) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(P4), (0x1u8) as u8); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffd8f0;
    if !(((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32))) { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(r9.wrapping_add(P2))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; crate::simd_ops::wu8(rcx.wrapping_add(P4), (0x1u8) as u8); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rax)) as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(P4), (0x1u8) as u8);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142118c50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x1429868d0; const P3: u64 = 0x142116d40; const P4: u64 = 0xffffffffffffffe8; const P5: u64 = 0x1420ec690;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r10 = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; rbx = ((rcx.wrapping_add(P0)) as u64) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rax = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; rax = ((rax).wrapping_add(0xffffffffffffffe8u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 2; continue; }
            2 => { rax = P2; bb = 6; continue; }
            3 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffd8f0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; rbx = ((rcx.wrapping_add(P0)) as u64) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; bb = 6; continue; }
            5 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = ((r10.wrapping_add(P0)) as u64) as u64; bb = 6; continue; }
            6 => { r8 = (crate::simd_ops::ru64(rbx)) as u64; rdx = rax; rcx = r10; r9 = ((r8.wrapping_add(P4)) as u64) as u64; r8 = ((r8).wrapping_add(0xffffffffffffffd0u64)) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_add_u64(rbx, (0xffffffffffffffd0u64) as u64); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142118cd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14210c770; const P1: u64 = 0x142107140;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rsi = ((((r9 as u32)) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; r14 = r8; r15 = rdx; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x40u64))) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x48u64))) as u64; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; bb = 2; continue; }
            2 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x1u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rbx); bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rbp = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; bb = 7; continue; }
            6 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; rbp = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; bb = 7; continue; }
            7 => { r8 = ((crate::simd_ops::ru8(rbx.wrapping_add(0x4u64))) as u32) as u32 as u64; rdx = rsi.wrapping_add(0x2); rdx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = rbx; rdx = ((rdx).wrapping_shl(3u32)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; r8 = rax; rdx = ((crate::simd_ops::ru8(rcx.wrapping_add(0x20u64))) as u32) as u32 as u64; rdx = (rdx & !0xffu64) | ((((((rdx as u8)) & (0x3u8))) as u8 as u64)); crate::simd_ops::wu8(rax, (0x7u8) as u8); crate::simd_ops::wu8(rax.wrapping_add(0x1u64), ((rdx as u8)) as u8); rcx = ((crate::simd_ops::ru8(rbx.wrapping_add(0x4u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rax.wrapping_add(0x2u64), ((rcx as u8)) as u8); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rcx); rcx = ((rsi.wrapping_add(rsi.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(3u32)) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x3u64), (0x1u8) as u8); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbp); crate::simd_ops::wu8(rax.wrapping_add(0x4u64), ((rsi as u8)) as u8); crate::simd_ops::wu16(rax.wrapping_add(0x5u64), (0x28u16) as u16); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), r15); crate::simd_ops::wu64(rax.wrapping_add(0x28u64), r14); crate::simd_ops::lk_sub_u64(rbx.wrapping_add(0x8u64), (rcx) as u64); fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = rcx.wrapping_sub(0x18); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rax))) as u128; crate::simd_ops::wu128(r8.wrapping_add(rcx).wrapping_add(0x30u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rcx.wrapping_add(rax).wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(r8.wrapping_add(rcx).wrapping_add(0x40u64), ((xmm1 as u64)) as u64); fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x1; rsi = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rax, r8); crate::simd_ops::wu32(rax.wrapping_add(0x10u64), (0x7u32) as u32); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0x18u64) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142118e10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x142116d40; const P3: u64 = 0x1429868d0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; r8 = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; rax = ((rax).wrapping_add(0xffffffffffffffe8u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 7; continue; } bb = 2; continue; }
            2 => { bb = 6; continue; }
            3 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffd8f0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; bb = 6; continue; }
            5 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rcx = P3; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = 0xffffffff; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            8 => { rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142118ea0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0xfffffffffffffffd; const P3: u64 = 0x1429868d0; const P4: u64 = 0x142116d40; const P5: u64 = 0x142103330; const P6: u64 = 0x1420ec690;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r10 = r8; rdi = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; rbx = ((rcx.wrapping_add(P0)) as u64) as u64; rsi = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rsi = ((rsi.wrapping_add(P2)) as u64) as u64; rsi = ((rax.wrapping_add(rsi.wrapping_mul(8))) as u64) as u64; fl_l = rsi; fl_r = (crate::simd_ops::ru64(rbx)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 2; continue; }
            2 => { rsi = P3; bb = 6; continue; }
            3 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffd8f0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; rbx = ((rcx.wrapping_add(P0)) as u64) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rsi = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; bb = 6; continue; }
            5 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rsi = rax; rbx = ((rdi.wrapping_add(P0)) as u64) as u64; bb = 6; continue; }
            6 => { r8 = (0xffffffffffffffffu64) as u64; bb = 7; continue; }
            7 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r10.wrapping_add(r8))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 8; continue; }
            8 => { rdx = r10; rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); r9 = (crate::simd_ops::ru64(rbx)) as u64; r8 = rsp.wrapping_add(0x20); r9 = ((r9).wrapping_sub(0x18u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rdx = rsi; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x5u32) as u32); rcx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_add_u64(rbx, (0xffffffffffffffe8u64) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142118f70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0xfffffffffffffffd; const P2: u64 = 0x8; const P3: u64 = 0x1429868d0; const P4: u64 = 0x142116d40; const P5: u64 = 0x1420ecd00;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut xmm0: u128 = 0; let mut xmm1: u128 = 0; let mut xmm2: u128 = 0; let mut xmm3: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbx = r8; r8 = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r9 = ((r9.wrapping_add(P1)) as u64) as u64; r9 = ((rax.wrapping_add(r9.wrapping_mul(8))) as u64) as u64; fl_l = r9; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rax = P3; rax = (if (((fl_l as u64)) < ((fl_r as u64))) { r9 } else { rax }) as u64; bb = 5; continue; }
            2 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffd8f0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(P2))) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; bb = 5; continue; }
            4 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 6; continue; }
            6 => { rdx = rsp.wrapping_add(0x20); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); bb = 9; continue; }
            9 => { rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            10 => { rax = (crate::simd_ops::cvt_f64_i32((f64::from_bits(crate::simd_ops::ru64(rax))))) as u32 as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu32(rbx, (0x1u32) as u32); bb = 12; continue; }
            12 => { rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142119010(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0xfffffffffffffffd; const P2: u64 = 0x8; const P3: u64 = 0x1429868d0; const P4: u64 = 0x142116d40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; r9 = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r8 = ((r8.wrapping_add(P1)) as u64) as u64; r8 = ((rax.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rax = P3; rax = (if (((fl_l as u64)) < ((fl_r as u64))) { r8 } else { rax }) as u64; bb = 5; continue; }
            2 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffd8f0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(r9.wrapping_add(P2))) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; bb = 5; continue; }
            4 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            7 => { rax = (crate::simd_ops::ru64(rax)) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142119080(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x40; const P2: u64 = 0x48; const P3: u64 = 0x14210c770; const P4: u64 = 0x1; const P5: u64 = 0x30; const P6: u64 = 0x68; const P7: u64 = 0x8; const P8: u64 = 0x1420fcda0; const P9: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rsi = ((r8 as u32)) as u32 as u64; rbp = ((rdx as u32)) as u32 as u64; rdi = rcx; rax = (crate::simd_ops::ru64(r9.wrapping_add(P1))) as u64; fl_l = (crate::simd_ops::ru64(r9.wrapping_add(P2))) as u64; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(P4))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rdi.wrapping_add(P4), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(P5))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P6), rax); crate::simd_ops::wu64(rcx.wrapping_add(P5), rdi); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(P7))) as u64; r8 = ((rsi as u32)) as u32 as u64; rdx = ((rbp as u32)) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu32(rbx.wrapping_add(P9), (0x6u32) as u32); crate::simd_ops::lk_add_u64(rdi.wrapping_add(P7), (0x18u64) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142119110(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0xfffffffffffffffd; const P2: u64 = 0x8; const P3: u64 = 0x1429868d0; const P4: u64 = 0x142116d40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; r9 = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r8 = ((r8.wrapping_add(P1)) as u64) as u64; r8 = ((rax.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rax = P3; rax = (if (((fl_l as u64)) < ((fl_r as u64))) { r8 } else { rax }) as u64; rcx = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; fl_r = 0x4; rax = (if (((fl_l as u32)) != ((fl_r as u32))) { rcx } else { rax }) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            2 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffd8f0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(r9.wrapping_add(P2))) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rcx = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; fl_r = 0x4; rax = (if (((fl_l as u32)) != ((fl_r as u32))) { rcx } else { rax }) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            4 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; fl_r = 0x4; rax = (if (((fl_l as u32)) != ((fl_r as u32))) { rcx } else { rax }) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142119190(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64;
    rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { 1u8 } else { 0u8 }) as u8 as u64));
    crate::simd_ops::wu32(r8, ((rax as u32)) as u32);
    crate::simd_ops::wu32(r8.wrapping_add(P1), (0x1u32) as u32);
    fl_l = (crate::simd_ops::lk_add_u64(rcx.wrapping_add(P0), (0x18u64) as u64)) as u64; fl_r = fl_l;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421191c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1429868d0; const P1: u64 = 0x142116d40; const P2: u64 = 0x1420ecc90; const P3: u64 = 0x14210c770;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = ((((rdx as u32)) as i32 as i64) as u64) as u64; rsi = r8; rbx = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; r10 = rdi.wrapping_sub(0x1); r10 = ((r10.wrapping_add(r10.wrapping_mul(2))) as u64) as u64; r8 = P0; r10 = ((rax.wrapping_add(r10.wrapping_mul(8))) as u64) as u64; fl_l = r10; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; r8 = (if (((fl_l as u64)) < ((fl_r as u64))) { r10 } else { r8 }) as u64; bb = 5; continue; }
            2 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0xffffd8f0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; r8 = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; bb = 5; continue; }
            4 => { rdx = ((rdi as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r8 = rax; bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(r8.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x5; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 19; continue; } bb = 6; continue; }
            6 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x1u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rbx); bb = 8; continue; }
            8 => { rdx = r8; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rsi, (0x0u64) as u64); bb = 11; continue; }
            11 => { rax = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            12 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x40u64))) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x48u64))) as u64; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdx = rdi.wrapping_sub(0x1); rdx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; r8 = P0; rdx = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; r8 = (if (((fl_l as u64)) < ((fl_r as u64))) { rdx } else { r8 }) as u64; bb = 19; continue; }
            16 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0xffffd8f0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 18; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; r8 = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; bb = 19; continue; }
            18 => { rdx = ((rdi as u32)) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r8 = rax; bb = 19; continue; }
            19 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(r8)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x14u64))) as u32 as u64; crate::simd_ops::wu64(rsi, rcx); bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru64(r8)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = ((rax).wrapping_add(0x18u64)) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142119300(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0xfffffffffffffffd; const P2: u64 = 0x8; const P3: u64 = 0x1429868d0; const P4: u64 = 0x142116d40; const P5: u64 = 0x1420ecd00;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut xmm0: u128 = 0; let mut xmm1: u128 = 0; let mut xmm2: u128 = 0; let mut xmm3: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbx = r8; r8 = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r9 = ((r9.wrapping_add(P1)) as u64) as u64; r9 = ((rax.wrapping_add(r9.wrapping_mul(8))) as u64) as u64; fl_l = r9; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rax = P3; rax = (if (((fl_l as u64)) < ((fl_r as u64))) { r9 } else { rax }) as u64; bb = 5; continue; }
            2 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffd8f0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(P2))) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; bb = 5; continue; }
            4 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 6; continue; }
            6 => { rdx = rsp.wrapping_add(0x20); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); bb = 9; continue; }
            9 => { rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            10 => { rax = (crate::simd_ops::cvt_f64_i64((f64::from_bits(crate::simd_ops::ru64(rax))))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu32(rbx, (0x1u32) as u32); bb = 12; continue; }
            12 => { rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421193a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142118e10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = (((rax as u32)).wrapping_add(0xfffffffdu32)) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xfffffffd;
    rax = 0x0;
    rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { 1u8 } else { 0u8 }) as u8 as u64));
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421193d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0xfffffffffffffffd; const P2: u64 = 0x8; const P3: u64 = 0x1429868d0; const P4: u64 = 0x142116d40; const P5: u64 = 0x4; const P6: u64 = 0x1420fcf40; const P7: u64 = 0x14;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; r9 = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r8 = ((r8.wrapping_add(P1)) as u64) as u64; r8 = ((rax.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rax = P3; rax = (if (((fl_l as u64)) < ((fl_r as u64))) { r8 } else { rax }) as u64; bb = 5; continue; }
            2 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffd8f0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(r9.wrapping_add(P2))) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; bb = 5; continue; }
            4 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x5; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 6; continue; }
            6 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 7; continue; }
            7 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            9 => { rax = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P5))) as u32 as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            10 => { rcx = (crate::simd_ops::ru64(rax)) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return crate::simd_ops::tail_stk(P6, rax, rcx, rdx, r8, r9, rsp); }
            11 => { rax = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P7))) as u32 as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142119460(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x4; const P3: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32) as u128;
    crate::simd_ops::wu32(rax.wrapping_add(P1), (0x4u32) as u32);
    fl_l = (crate::simd_ops::lk_add_u64(rcx.wrapping_add(P0), (0x18u64) as u64)) as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rax, ((xmm1 as u32)) as u32);
    crate::simd_ops::wu32(rax.wrapping_add(P2), ((xmm2 as u32)) as u32);
    crate::simd_ops::wu32(rax.wrapping_add(P0), ((xmm3 as u32)) as u32);
    crate::simd_ops::wu32(rax.wrapping_add(P3), ((xmm0 as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142119490(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1; const P1: u64 = 0x18; const P2: u64 = 0x30; const P3: u64 = 0x68; const P4: u64 = 0x1429868d0; const P5: u64 = 0x142116d40; const P6: u64 = 0x1420fc9b0; const P7: u64 = 0x10; const P8: u64 = 0xfffffffffffffff8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64; r10 = ((r8 as u32)) as u32 as u64; r11 = rcx; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rcx.wrapping_add(P0), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(r9.wrapping_add(P2))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P3), rax); crate::simd_ops::wu64(r9.wrapping_add(P2), rcx); bb = 2; continue; }
            2 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; r11 = ((r11).wrapping_add(0x8u64)) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = ((rax).wrapping_add(0xffffffffffffffe8u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r11)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 4; continue; }
            4 => { rax = P4; bb = 8; continue; }
            5 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffd8f0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; fl_l = ((r11).wrapping_add(0x8u64)) as u64; fl_r = fl_l; r11 = ((fl_l as u64)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(r11)) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; bb = 8; continue; }
            7 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((r11).wrapping_add(0x8u64)) as u64; fl_r = fl_l; r11 = ((fl_l as u64)) as u64; bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rax)) as u64; rdx = ((r10 as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(r11)) as u64; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(P7))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(P7), ((xmm1 as u64)) as u64); crate::simd_ops::lk_add_u64(r11, (0x18u64) as u64); rax = (crate::simd_ops::ru64(r11)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P8))) as u32 as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142119540(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x142118a60;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; crate::simd_ops::wu32(rax.wrapping_add(P1), ((rdx as u32)) as u32); fl_l = (crate::simd_ops::lk_add_u64(rcx.wrapping_add(P0), (0x18u64) as u64)) as u64; fl_r = fl_l; return rax; }
    r8 = (0xffffffffffffffffu64) as u64;
    loop { r8 = ((r8).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if !((((fl_l as u8)) != ((fl_r as u8)))) { break; } }
    return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142119580(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0xfffffffffffffffd; const P2: u64 = 0x8; const P3: u64 = 0x1429868d0; const P4: u64 = 0x142116d40; const P5: u64 = 0x1420ecd00;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbx = r8; r8 = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r9 = ((r9.wrapping_add(P1)) as u64) as u64; r9 = ((rax.wrapping_add(r9.wrapping_mul(8))) as u64) as u64; fl_l = r9; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rax = P3; rax = (if (((fl_l as u64)) < ((fl_r as u64))) { r9 } else { rax }) as u64; bb = 5; continue; }
            2 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffd8f0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(P2))) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; bb = 5; continue; }
            4 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 6; continue; }
            6 => { rdx = rsp.wrapping_add(0x20); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); bb = 9; continue; }
            9 => { xmm0 = 0u128; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            10 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu32(rbx, (0x1u32) as u32); bb = 12; continue; }
            12 => { xmm0 = ((crate::simd_ops::ru64(rax)) as u64) as u128; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142119620(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rax, ((xmm1 as u64)) as u64);
    crate::simd_ops::wu32(rax.wrapping_add(P1), (0x3u32) as u32);
    fl_l = (crate::simd_ops::lk_add_u64(rcx.wrapping_add(P0), (0x18u64) as u64)) as u64; fl_r = fl_l;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142119640(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = (0x2aaaaaaaaaaaaaabu64) as u64;
    rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(rcx.wrapping_add(P1)))) as u64;
    let prod = (rax as i64 as i128).wrapping_mul((rdx) as i64 as i128) as u128;
    rax = prod as u64; rdx = (prod >> 64) as u64;
    rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64;
    rax = rdx;
    rax = ((rax).wrapping_shr(63u32)) as u64;
    fl_l = ((rax).wrapping_add(rdx)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142119670(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1; const P1: u64 = 0x18; const P2: u64 = 0x30; const P3: u64 = 0x68; const P4: u64 = 0x8; const P5: u64 = 0x10; const P6: u64 = 0xfffffffffffffffd; const P7: u64 = 0x1429868d0; const P8: u64 = 0x142116d40; const P9: u64 = 0x142103330; const P10: u64 = 0x1420ec940; const P11: u64 = 0xfffffffffffffff8;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64; r10 = r8; rbx = rcx; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rcx.wrapping_add(P0), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(r9.wrapping_add(P2))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P3), rax); crate::simd_ops::wu64(r9.wrapping_add(P2), rcx); bb = 2; continue; }
            2 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; rdi = ((rcx.wrapping_add(P4)) as u64) as u64; rsi = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P5))) as u64; rsi = ((rsi.wrapping_add(P6)) as u64) as u64; rsi = ((rax.wrapping_add(rsi.wrapping_mul(8))) as u64) as u64; fl_l = rsi; fl_r = (crate::simd_ops::ru64(rdi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 4; continue; }
            4 => { rsi = P7; bb = 8; continue; }
            5 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffd8f0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; rdi = ((rcx.wrapping_add(P4)) as u64) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rsi = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; bb = 8; continue; }
            7 => { rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rsi = rax; rdi = ((rbx.wrapping_add(P4)) as u64) as u64; bb = 8; continue; }
            8 => { r8 = (0xffffffffffffffffu64) as u64; bb = 9; continue; }
            9 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r10.wrapping_add(r8))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 10; continue; }
            10 => { rdx = r10; rcx = rbx; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); r9 = (crate::simd_ops::ru64(rdi)) as u64; r8 = rsp.wrapping_add(0x20); rdx = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rcx = rbx; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x5u32) as u32); rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_add_u64(rdi, (0x18u64) as u64); rax = (crate::simd_ops::ru64(rdi)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P11))) as u32 as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142119760(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1433e1350; const P1: u64 = 0x142986510;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffffff;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rax = P0; return rax; }
    rax = ((((rdx as u32)) as i32 as i64) as u64) as u64;
    rcx = P1;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)))) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142119790(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0xfffffffffffffffd; const P2: u64 = 0x8; const P3: u64 = 0x1429868d0; const P4: u64 = 0x142116d40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; r9 = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r8 = ((r8.wrapping_add(P1)) as u64) as u64; r8 = ((rax.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rax = P3; rax = (if (((fl_l as u64)) < ((fl_r as u64))) { r8 } else { rax }) as u64; bb = 5; continue; }
            2 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffd8f0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(r9.wrapping_add(P2))) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; bb = 5; continue; }
            4 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = 0x1; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            9 => { rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142119810(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1429868d0; const P1: u64 = 0x142116d40; const P2: u64 = 0x1420fb250; const P3: u64 = 0x14210c5f0; const P4: u64 = 0x14210e620;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r10 = ((r8 as u32)) as u32 as u64; rsi = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; rdi = rcx.wrapping_add(0x8); r14 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; r14 = r14.wrapping_sub(0x3); r14 = ((rax.wrapping_add(r14.wrapping_mul(8))) as u64) as u64; fl_l = r14; fl_r = (crate::simd_ops::ru64(rdi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 2; continue; }
            2 => { r14 = P0; bb = 6; continue; }
            3 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffd8f0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; rdi = rcx.wrapping_add(0x8); rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; r14 = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; bb = 6; continue; }
            5 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rax; rdi = rsi.wrapping_add(0x8); bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(r14)) as u64; rcx = rsi; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x4u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 14; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); r8 = ((r10 as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rdi)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rbx.wrapping_sub(0x18u64))) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rbx.wrapping_sub(0x8u64))) as u64) as u128; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rdx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_sub(0x8u64))) as u32 as u64; fl_r = 0x5; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 13; continue; } bb = 8; continue; }
            8 => { r8 = (crate::simd_ops::ru64(r14)) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x1u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(rdx.wrapping_sub(0x18u64))) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x1u64))) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x21u64))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 12; continue; } bb = 11; continue; }
            11 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 13; continue; }
            12 => { fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(r8.wrapping_add(0x1u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x28u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), r8); bb = 13; continue; }
            13 => { crate::simd_ops::lk_add_u64(rdi, (0xffffffffffffffe8u64) as u64); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            14 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x142119907, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142119910(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0xfffffffffffffffd; const P2: u64 = 0x8; const P3: u64 = 0x1429868d0; const P4: u64 = 0x142116d40; const P5: u64 = 0x4; const P6: u64 = 0x1420fc910; const P7: u64 = 0x14210e620;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; r10 = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r8 = ((r8.wrapping_add(P1)) as u64) as u64; r8 = ((rax.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rax = P3; rax = (if (((fl_l as u64)) < ((fl_r as u64))) { r8 } else { rax }) as u64; bb = 5; continue; }
            2 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffd8f0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(r10.wrapping_add(P2))) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; bb = 5; continue; }
            4 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P5))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; return crate::simd_ops::tail_stk(P6, rax, rcx, rdx, r8, r9, rsp); }
            7 => { rcx = r10; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x142119977, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142119980(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    rax = ((((rdx as u32)) as i32 as i64) as u64) as u64;
    r8 = rcx;
    fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64;
    if !((((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0)) { rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r9 = ((rdx.wrapping_mul(8)) as u64) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rax = ((rdx.wrapping_add(r9)) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { rax = ((r9.wrapping_add(rdx)) as u64) as u64; crate::simd_ops::wu64(r8.wrapping_add(P1), rax); return rax; } r10 = 0; fl_l = 0; fl_r = 0; loop { crate::simd_ops::wu32(rcx.wrapping_add(P0), ((r10 as u32)) as u32); rcx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; rcx = ((rcx).wrapping_add(0x18u64)) as u64; crate::simd_ops::wu64(r8.wrapping_add(P1), rcx); rdx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rax = ((rdx.wrapping_add(r9)) as u64) as u64; fl_l = rcx; fl_r = rax; if !((((fl_l as u64)) < ((fl_r as u64)))) { break; } } rax = ((r9.wrapping_add(rdx)) as u64) as u64; crate::simd_ops::wu64(r8.wrapping_add(P1), rax); return rax; }
    fl_l = ((rax).wrapping_add(1)) as u64;
    rax = ((fl_l as u64)) as u64;
    rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64;
    rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P1), rcx);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421199f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu32(rax.wrapping_add(P1), (0x0u32) as u32);
    fl_l = (crate::simd_ops::lk_add_u64(rcx.wrapping_add(P0), (0x18u64) as u64)) as u64; fl_r = fl_l;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142119b30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401dec00; const P1: u64 = 0x14183acc8; const P2: u64 = 0x1401de830; const P3: u64 = 0x143932190; const P4: u64 = 0x1433e1380; const P5: u64 = 0x14169edf0; const P6: u64 = 0x1433e13b8; const P7: u64 = 0x14169e5d0; const P8: u64 = 0x14169f6b0; const P9: u64 = 0x14169fa50; const P10: u64 = 0x14169edb0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; r13 = 0; fl_l = 0; fl_r = 0; r12 = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 1; continue; }
            1 => { r15 = rcx.wrapping_add(0x8); r14 = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); bb = 2; continue; }
            2 => { rbp = (crate::simd_ops::ru32(r15.wrapping_add(0x8u64))) as u32 as u64; rsi = ((rbp.wrapping_mul(8)) as u64) as u64; rcx = (crate::simd_ops::ru32(r15.wrapping_add(0xcu64))) as u32 as u64; rdi = (crate::simd_ops::ru64(r15)) as u64; rax = ((rcx.wrapping_mul(8)) as u64) as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = rbp.wrapping_add(0x1); rbx = ((rcx as u32)) as u32 as u64; rbx = ((rbx).wrapping_shr(1u32)) as u64; rbx = ((rbx).wrapping_add(rcx)) as u64; fl_l = rbx; fl_r = rax; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rbx }) as u64; rdx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdi = rax; r8 = (crate::simd_ops::ru32(r15.wrapping_add(0x8u64))) as u32 as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; rdx = (crate::simd_ops::ru64(r15)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = r15; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(r15, rdi); crate::simd_ops::wu32(r15.wrapping_add(0x8u64), ((rbp as u32)) as u32); crate::simd_ops::wu32(r15.wrapping_add(0xcu64), ((rbx as u32)) as u32); bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsi.wrapping_add(rdi), (0x0u64) as u64); rax = (crate::simd_ops::ru32(r15.wrapping_add(0x8u64))) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(r15.wrapping_add(0x8u64), ((rax as u32)) as u32); rdi = ((rax as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(r15)) as u64; rbp = (crate::simd_ops::ru64(r14)) as u64; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(P3)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = 0xd0; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r14 = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu32(rax.wrapping_add(0x8u64), (0x1u32) as u32); rax = P4; crate::simd_ops::wu64(r14, rax); crate::simd_ops::wu64(r14.wrapping_add(0x20u64), (0xffffffffffffffffu64) as u64); rcx = r14; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r14.wrapping_add(0x10u64), rax); rcx = P6; crate::simd_ops::wu64(r14, rcx); crate::simd_ops::wu64(r14.wrapping_add(0x38u64), rax); crate::simd_ops::wu32(r14.wrapping_add(0x40u64), ((rax as u32)) as u32); crate::simd_ops::wu64(r14.wrapping_add(0x48u64), rax); crate::simd_ops::wu64(r14.wrapping_add(0x50u64), rax); crate::simd_ops::wu64(r14.wrapping_add(0x58u64), rax); crate::simd_ops::wu64(r14.wrapping_add(0x60u64), rax); crate::simd_ops::wu64(r14.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(r14.wrapping_add(0x70u64), rax); crate::simd_ops::wu64(r14.wrapping_add(0x78u64), rax); crate::simd_ops::wu64(r14.wrapping_add(0x80u64), rax); crate::simd_ops::wu64(r14.wrapping_add(0x88u64), rax); crate::simd_ops::wu64(r14.wrapping_add(0x90u64), rax); crate::simd_ops::wu32(r14.wrapping_add(0x98u64), (0x7fffffffu32) as u32); crate::simd_ops::wu32(r14.wrapping_add(0x9cu64), (0x7fffffffu32) as u32); crate::simd_ops::wu32(r14.wrapping_add(0xa0u64), (0x80000000u32) as u32); crate::simd_ops::wu64(r14.wrapping_add(0xa4u64), (0xffffffff80000000u64) as u64); crate::simd_ops::wu32(r14.wrapping_add(0xacu64), (0xffffffffu32) as u32); rsi = r14.wrapping_add(0xb0); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); rcx = rsi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = rsi.wrapping_add(0x8); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rsi.wrapping_add(0x10u64), (0x0u32) as u32); rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(r14.wrapping_add(0xc8u64), ((rsi as u32)) as u32); r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = r14; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            7 => { rsi = 0; fl_l = 0; fl_r = 0; r14 = ((rsi as u32)) as u32 as u64; bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rbx.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x8u64), r14); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; r8 = ((r8).wrapping_add(r13)) as u64; rdx = rsp.wrapping_add(0xa0); rcx = r14; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0xa0u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x40u64))) as u64; r8 = (crate::simd_ops::ru64(r8.wrapping_add(r12.wrapping_mul(8)))) as u64; rdx = rsp.wrapping_add(0x21); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x8u64))) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            10 => { rbx = (crate::simd_ops::ru64(r15)) as u64; rax = (crate::simd_ops::ru32(r15.wrapping_add(0x8u64))) as u32 as u64; rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rsi); bb = 13; continue; }
            13 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 14; continue; }
            14 => { fl_l = ((r13).wrapping_add(rbp)) as u64; fl_r = fl_l; r13 = ((fl_l as u64)) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; bb = 15; continue; }
            15 => { r12 = (((r12 as u32)).wrapping_add(1)) as u32 as u64; r14 = ((r14).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); rax = ((((r12 as u32)) as i32 as i64) as u64) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142119dd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x14169edb0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rsi = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rsi); bb = 4; continue; }
            4 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142119e20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x1434268d0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm10: u128 = 0; let mut xmm11: u128 = 0; let mut xmm12: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x58u64)) as u64; xmm0 = ((crate::simd_ops::ru32(rdx)) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32) as u128; xmm3 = ((crate::simd_ops::ru32(r8)) as u32) as u128; xmm5 = ((crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm6) as u128); xmm6 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(crate::simd_ops::ru32(rcx))).to_bits() as u128); xmm0 = ((crate::simd_ops::ru32(r9)) as u32) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm7) as u128); xmm7 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(P0)))).to_bits() as u128); xmm1 = ((crate::simd_ops::ru32(r9.wrapping_add(P0))) as u32) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm10) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x10u64), (xmm11) as u128); crate::simd_ops::wu128(rsp, (xmm12) as u128); xmm12 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm0 = (xmm12 & !0xffff_ffffu128) | ((f32::from_bits(xmm12 as u32) * f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm10 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm11 = 0u128; fl_l = ((xmm10 as u32)) as u32 as u64; fl_r = ((xmm11 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 2; continue; } bb = 1; continue; }
            1 => { if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 7; continue; } bb = 2; continue; }
            2 => { xmm4 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(crate::simd_ops::ru32(rcx))).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) - f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(P0)))).to_bits() as u128); xmm1 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm0 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(xmm12 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm10 as u32)).to_bits() as u128); fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm11 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 7; continue; } bb = 3; continue; }
            3 => { xmm3 = ((crate::simd_ops::ru32(P1)) as u32) as u128; fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 7; continue; } bb = 4; continue; }
            4 => { xmm1 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm0 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm4 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm10 as u32)).to_bits() as u128); fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = ((xmm11 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 7; continue; } bb = 5; continue; }
            5 => { fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm4 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; xmm0 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm3 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx))).to_bits() as u128); xmm1 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(P0)))).to_bits() as u128); xmm0 = crate::simd_ops::vunpcklps(xmm3, xmm2); crate::simd_ops::wu64(rax, ((xmm0 as u64)) as u64); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 8; continue; }
            7 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; xmm10 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm11 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; xmm12 = (crate::simd_ops::ru128(rsp)) as u128; rsp = ((rsp).wrapping_add(0x58u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142119f30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x14183ad94;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut xmm4: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm3 = ((crate::simd_ops::ru32(rcx)) as u32) as u128;
    xmm4 = ((crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32) as u128;
    xmm1 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(P0)))).to_bits() as u128);
    xmm0 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(crate::simd_ops::ru32(rdx))).to_bits() as u128);
    xmm2 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(crate::simd_ops::ru32(rdx))).to_bits() as u128);
    xmm0 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128);
    xmm1 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(P0)))).to_bits() as u128);
    xmm1 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128);
    return crate::simd_ops::tail_fwd_xmm(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]);
}
