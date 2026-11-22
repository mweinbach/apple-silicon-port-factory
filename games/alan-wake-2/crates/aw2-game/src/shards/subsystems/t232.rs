#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn fn_14160cae0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931d48; const P1: u64 = 0x143931d50; const P2: u64 = 0x143931d60; const P3: u64 = 0x143931d68; const P4: u64 = 0x143931d58; const P5: u64 = 0x1415242e0; const P6: u64 = 0x143931d6c; const P7: u64 = 0x1415940e0; const P8: u64 = 0x1415ee0d0; const P9: u64 = 0x143931d44;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(P0)) as u64; rsi = 0; fl_l = 0; fl_r = 0; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); bb = 2; continue; }
            2 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = rbx.wrapping_add(0x8); fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x298u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 23; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xcu64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) > ((fl_r as u8))) { bb = 23; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x304u64))) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 14; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x378u64))) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 6; continue; }
            6 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(P0)) as u64; rbp = (crate::simd_ops::ru64(rbx)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(P0, rbp); bb = 9; continue; }
            8 => { crate::simd_ops::wu64(rsi, rbp); bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(P1)) as u64; rdx = (crate::simd_ops::ru64(P2)) as u64; fl_l = rbx; fl_r = rax; rax = (if (((fl_l as u64)) == ((fl_r as u64))) { rsi } else { rax }) as u64; crate::simd_ops::wu64(P1, rax); fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru32(P3)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((rdx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(P4)) as u64; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(P4, rbx); bb = 13; continue; }
            12 => { rdx = rbx; rcx = 0x5; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { crate::simd_ops::lk_sub_u32(P6, (1) as u32); rdx = 0x1; rcx = rdi; rbx = rbp; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 24; continue; }
            14 => { rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rbx)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 22; continue; } bb = 15; continue; }
            15 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(P0)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu64(P0, rdi); bb = 18; continue; }
            17 => { crate::simd_ops::wu64(rsi, rdi); bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(P1)) as u64; rdx = (crate::simd_ops::ru64(P2)) as u64; fl_l = rbx; fl_r = rax; rax = (if (((fl_l as u64)) == ((fl_r as u64))) { rsi } else { rax }) as u64; crate::simd_ops::wu64(P1, rax); fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 21; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru32(P3)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((rdx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 21; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(P4)) as u64; crate::simd_ops::wu64(rbx, rax); fl_l = (crate::simd_ops::lk_sub_u32(P6, (1) as u32)) as u32 as u64; crate::simd_ops::wu64(P4, rbx); rbx = rdi; bb = 24; continue; }
            21 => { rdx = rbx; rcx = 0x5; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_sub_u32(P6, (1) as u32)) as u32 as u64; rbx = rdi; bb = 24; continue; }
            22 => { rsi = rbx; rbx = rdi; bb = 24; continue; }
            23 => { rsi = rbx; rbx = (crate::simd_ops::ru64(rbx)) as u64; bb = 24; continue; }
            24 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 25; continue; }
            25 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu8(P9, ((rbx as u8)) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            26 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu8(P9, ((rsi as u8)) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160cca0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931d58; const P1: u64 = 0x143931d78; const P2: u64 = 0x143931d70; const P3: u64 = 0x143931d48; const P4: u64 = 0x143931d60; const P5: u64 = 0x143931d50; const P6: u64 = 0x143931d68; const P7: u64 = 0x143931d6c; const P8: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru64(P0)) as u64; rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(P1, rdi); crate::simd_ops::wu64(P2, rdi); bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru64(P3)) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(r8)) as u64; r9 = (crate::simd_ops::ru64(P4)) as u64; crate::simd_ops::wu64(P3, rax); rax = (crate::simd_ops::ru64(P5)) as u64; fl_l = r8; fl_r = rax; rax = (if (((fl_l as u64)) == ((fl_r as u64))) { rdi } else { rax }) as u64; crate::simd_ops::wu64(P5, rax); fl_l = r8; fl_r = r9; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(P6)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((r9.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(r8, rdx); rdx = r8; fl_l = (crate::simd_ops::lk_sub_u32(P7, (1) as u32)) as u32 as u64; crate::simd_ops::wu64(P0, rdx); bb = 1; continue; }
            5 => { rdx = r8; rcx = 0x5; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(P0)) as u64; fl_l = (crate::simd_ops::lk_sub_u32(P7, (1) as u32)) as u32 as u64; bb = 1; continue; }
            6 => { r8 = (crate::simd_ops::ru64(P4)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(P6)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((r8.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = 0x5; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(P4)) as u64; bb = 11; continue; }
            11 => { rdx = rbx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 13; continue; }
            13 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rdx = r8; rcx = 0x5; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(P4, rdi); bb = 15; continue; }
            15 => { xmm0 = 0u128; crate::simd_ops::wu32(P6, ((rdi as u32)) as u32); crate::simd_ops::wu128(P3, (xmm0) as u128); crate::simd_ops::wu64(P0, rdi); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160cdc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x18; const P2: u64 = 0x20; const P3: u64 = 0x8; const P4: u64 = 0x10; const P5: u64 = 0x28; const P6: u64 = 0x141612300; const P7: u64 = 0x24;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    crate::simd_ops::lk_and_u8(rcx.wrapping_add(P0), (0xf0u8) as u8);
    rbx = rcx;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rdx);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), r8);
    crate::simd_ops::wu32(rcx.wrapping_add(P3), (0x1u32) as u32);
    crate::simd_ops::wu64(rcx.wrapping_add(P4), (0x0u64) as u64);
    crate::simd_ops::wu16(rcx.wrapping_add(P5), (0xff00u16) as u16);
    rcx = rdx;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); }
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(P7), (1) as u32)) as u32 as u64; }
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160ce10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; rdi = rcx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = 0xffffffff; rcx = ((rcx).wrapping_add(0x10u64)) as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 5; continue; }
            5 => { rbp = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x20u64), rax); fl_l = rcx; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rdi.wrapping_add(0x28u64), rbp); bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rcx, rax); fl_l = (crate::simd_ops::lk_sub_u32(rdi.wrapping_add(0x44u64), (1) as u32)) as u32 as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x30u64), rcx); bb = 6; continue; }
            10 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 11; continue; }
            11 => { bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x38u64))) as u64; rbx = rdi.wrapping_add(0x40); rsi = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            15 => { rdx = rsi; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 16; continue; }
            16 => { bb = 18; continue; }
            17 => { rbx = rdi.wrapping_add(0x40); bb = 18; continue; }
            18 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x38u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(0x38u64), rbp); bb = 20; continue; }
            20 => { crate::simd_ops::wu32(rbx, ((rbp as u32)) as u32); crate::simd_ops::wu64(rdi.wrapping_add(0x20u64), rbp); crate::simd_ops::wu64(rdi.wrapping_add(0x28u64), rbp); crate::simd_ops::wu64(rdi.wrapping_add(0x30u64), rbp); bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x48u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x48u64), rax); fl_l = rcx; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x50u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 24; continue; } bb = 23; continue; }
            23 => { crate::simd_ops::wu64(rdi.wrapping_add(0x50u64), rbp); bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu64(rcx, rax); fl_l = (crate::simd_ops::lk_sub_u32(rdi.wrapping_add(0x6cu64), (1) as u32)) as u32 as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x58u64), rcx); bb = 21; continue; }
            25 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x58u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 26; continue; }
            26 => { bb = 27; continue; }
            27 => { r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; rbx = rdi.wrapping_add(0x68); rsi = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 29; continue; } bb = 28; continue; }
            28 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((r8.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 30; continue; } bb = 29; continue; }
            29 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 30; continue; }
            30 => { rdx = rsi; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 27; continue; } bb = 31; continue; }
            31 => { bb = 33; continue; }
            32 => { rbx = rdi.wrapping_add(0x68); bb = 33; continue; }
            33 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 35; continue; } bb = 34; continue; }
            34 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(0x60u64), rbp); bb = 35; continue; }
            35 => { crate::simd_ops::wu32(rbx, ((rbp as u32)) as u32); crate::simd_ops::wu64(rdi.wrapping_add(0x48u64), rbp); crate::simd_ops::wu64(rdi.wrapping_add(0x50u64), rbp); crate::simd_ops::wu64(rdi.wrapping_add(0x58u64), rbp); bb = 36; continue; }
            36 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x70u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 40; continue; } bb = 37; continue; }
            37 => { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x70u64), rax); fl_l = rcx; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x78u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 39; continue; } bb = 38; continue; }
            38 => { crate::simd_ops::wu64(rdi.wrapping_add(0x78u64), rbp); bb = 39; continue; }
            39 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu64(rcx, rax); fl_l = (crate::simd_ops::lk_sub_u32(rdi.wrapping_add(0x94u64), (1) as u32)) as u32 as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x80u64), rcx); bb = 36; continue; }
            40 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x80u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 47; continue; } bb = 41; continue; }
            41 => { bb = 42; continue; }
            42 => { r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x88u64))) as u64; rbx = rdi.wrapping_add(0x90); rsi = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 44; continue; } bb = 43; continue; }
            43 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((r8.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 45; continue; } bb = 44; continue; }
            44 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 45; continue; }
            45 => { rdx = rsi; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 42; continue; } bb = 46; continue; }
            46 => { bb = 48; continue; }
            47 => { rbx = rdi.wrapping_add(0x90); bb = 48; continue; }
            48 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x88u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 50; continue; } bb = 49; continue; }
            49 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(0x88u64), rbp); bb = 50; continue; }
            50 => { crate::simd_ops::wu32(rbx, ((rbp as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x70u64), rbp); crate::simd_ops::wu64(rdi.wrapping_add(0x78u64), rbp); crate::simd_ops::wu64(rdi.wrapping_add(0x80u64), rbp); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160d040(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    rax = (crate::simd_ops::ru32(rcx)) as u32 as u64;
    fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64;
    rax = ((fl_l as u32)) as u32 as u64;
    crate::simd_ops::wu32(rcx, ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160d050(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x2d; const P2: u64 = 0x38;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x90u8) as u8 as u64;
    if (((fl_l as u8)) != ((fl_r as u8))) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if (((fl_l as u8)) <= ((fl_r as u8))) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P2))) as u32) as u32 as u64;
    fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64;
    if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
    fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xfcu8))) as u8 as u64)); crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x80u8) as u8); fl_l = ((((rax as u8)) | (0x4u8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rcx.wrapping_add(P2), ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160d080(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x29; const P1: u64 = 0x28; const P2: u64 = 0x80; const P3: u64 = 0x2c; const P4: u64 = 0x38; const P5: u64 = 0x2d; const P6: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 18; continue; } bb = 1; continue; }
            1 => { r8 = ((crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u32) as u32 as u64; rax = ((r8.wrapping_add(P2)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xcfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 18; continue; } bb = 2; continue; }
            2 => { fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0xb0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 18; continue; } bb = 3; continue; }
            3 => { r8 = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64; rax = ((r8.wrapping_add(P2)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xcfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 18; continue; } bb = 4; continue; }
            4 => { fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0xb0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 18; continue; } bb = 5; continue; }
            5 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P3))) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P3))) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 18; continue; } bb = 6; continue; }
            6 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P4))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 18; continue; } bb = 7; continue; }
            7 => { r8 = ((crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x90u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 13; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P5))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 13; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u8 as u64; fl_r = (0x90u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 13; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P5))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) <= ((fl_r as u8))) { bb = 13; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu8(rcx.wrapping_add(P1), (0x80u8) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P5))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(P5), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P6))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P6), rax); fl_l = (crate::simd_ops::lk_add_u32(rax, (1) as u32)) as u32 as u64; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P4))) as u32) as u32 as u64; bb = 13; continue; }
            13 => { r8 = ((crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 17; continue; } bb = 14; continue; }
            14 => { fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x90u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 16; continue; } bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P5))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xfbu8))) as u8 as u64)); rcx = (rcx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; fl_l = ((((rax as u8)) | ((rcx as u8)))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rdx.wrapping_add(P4), ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
            17 => { rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xfbu8))) as u8 as u64)); rcx = (rcx & !0xffu64) | (((0x4u8) as u8 as u64)); fl_l = ((((rax as u8)) | ((rcx as u8)))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rdx.wrapping_add(P4), ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
            18 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160d150(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14152ec40; const P1: u64 = 0x1415864b0; const P2: u64 = 0x1415945c0; const P3: u64 = 0x1415bfb60; const P4: u64 = 0x1415c0d60; const P5: u64 = 0x1415c0fe0; const P6: u64 = 0x14152ede0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 109>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x240); rsp = ((rsp).wrapping_sub(0x340u64)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; r15 = r8.wrapping_add(0x10); rsi = rcx; r14 = r9; rcx = rdx.wrapping_add(0x10); rdi = r8; rbx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xb; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 15; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x370u64), r12); rcx = rbp.wrapping_sub(0x10); r12 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu16(rbp.wrapping_sub(0x28u64), (0xff00u16) as u16); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r12); crate::simd_ops::wu32(rbp.wrapping_sub(0x44u64), ((r12 as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), r12); crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), r12); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x290u64))) as u64; rax = ((r12 as u32)) as u32 as u64; rdx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x238u64))) as u32) as u32 as u64; crate::simd_ops::lk_and_u8(rbp.wrapping_add(0x234u64), (0xfeu8) as u8); rdx = (rdx & !0xffu64) | ((((((rdx as u8)) & (0xf3u8))) as u8 as u64)); rdx = (rdx & !0xffu64) | ((((((rdx as u8)) | (0x3u8))) as u8 as u64)); crate::simd_ops::wu32(rbp.wrapping_add(0x230u64), ((r12 as u32)) as u32); fl_l = rdi; fl_r = rdi; crate::simd_ops::wu8(rbp.wrapping_add(0x238u64), ((rdx as u8)) as u8); rdi = (crate::simd_ops::ru64(rcx)) as u64; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { r15 } else { rax }) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x78u64), ((r12 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x14u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x74u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rax); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((r12 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x30u64), ((r12 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((rax as u32)) as u32); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x238u64))) as u32) as u32 as u64; bb = 3; continue; }
            3 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::lk_add_u32(rdi, (1) as u32)) as u32 as u64; rdx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x238u64))) as u32) as u32 as u64; bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x34u64))) as u32 as u64; rdx = (rdx & !0xffu64) | ((((((rdx as u8)) | (0x2u8))) as u8 as u64)); fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x28u64))) as u8 as u64; fl_r = (0xfeu8) as u8 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x3cu64), ((rax as u32)) as u32); rax = rsp.wrapping_add(0x30); crate::simd_ops::wu32(rbp.wrapping_sub(0x40u64), ((r12 as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x38u64), r12); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r12 as u32)) as u32); r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x370u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rdi); crate::simd_ops::wu8(rbp.wrapping_add(0x238u64), ((rdx as u8)) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), (0x4u32) as u32); if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x298u64))) as u64; xmm0 = ((crate::simd_ops::ru64(rax)) as u64) as u128; rax = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x20u64), ((rax as u32)) as u32); bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_sub(0x28u64))) as u8 as u64; fl_r = (0x90u8) as u8 as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), rax); if (((fl_l as u8)) != ((fl_r as u8))) { bb = 11; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x28u64))) as u64; rax = ((rax).wrapping_shr(40u32)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { rdx = (rdx & !0xffu64) | ((((((rdx as u8)) & (0x1u8))) as u8 as u64)); rcx = rsp.wrapping_add(0x40); crate::simd_ops::wu8(rbp.wrapping_add(0x278u64), ((rdx as u8)) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = ((rax as u32)) as u32 as u64; r9 = rbp.wrapping_add(0x278); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x74u64))) as u32 as u64; rdx = rbp.wrapping_sub(0x28); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); rcx = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 10; continue; }
            10 => { rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x238u64))) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x278u64))) as u32) as u32 as u64; rcx = (rcx & !0xffu64) | ((((((rcx as u8)) ^ ((rax as u8)))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0x1u8))) as u8 as u64)); fl_l = ((((rax as u8)) ^ ((rcx as u8)))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbp.wrapping_add(0x238u64), ((rax as u8)) as u8); bb = 11; continue; }
            11 => { rdx = rsp.wrapping_add(0x40); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = rsp.wrapping_add(0x40); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x250u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 13; continue; }
            13 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_sub(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x380u64))) as u64; rsp = ((rsp).wrapping_add(0x340u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160d370(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18; const P2: u64 = 0x141612370; const P3: u64 = 0x20; const P4: u64 = 0x14; const P5: u64 = 0x10; const P6: u64 = 0x14160ce10; const P7: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x1; crate::simd_ops::wu32(rcx.wrapping_add(P0), ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); rbx = rcx; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { rcx = ((rcx).wrapping_add(0x10u64)) as u64; rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(P4), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_r = 0x1; crate::simd_ops::wu32(rdi, ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdx = rdi; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rdx = rbx; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = ((rsi as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160d400(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14160ce10; const P1: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    fl_l = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_r = 0x1;
    crate::simd_ops::wu32(rcx, ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32);
    rbx = rcx;
    rdi = (crate::simd_ops::ru32(rcx)) as u32 as u64;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = ((rdi as u32)) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160d430(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524330;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x38u64))) as u32) as u32 as u64; rsi = rcx; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xfdu8))) as u8 as u64)); r15 = ((r8 as u32)) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(0x38u64), ((rax as u8)) as u8); r14 = rdx; rcx = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rbp = (rbp & !0xffu64) | (((0x1u8) as u8 as u64)); fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x90u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x5u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) <= ((fl_r as u8))) { bb = 20; continue; } bb = 2; continue; }
            2 => { bb = 5; continue; }
            3 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xfeu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 17; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru16(rdx.wrapping_add(0x4u64))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 23; continue; } bb = 5; continue; }
            5 => { rdx = 0x98; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = 0; fl_l = 0; fl_r = 0; rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; xmm0 = ((crate::simd_ops::ru64(r14)) as u64) as u128; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x4u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rdx as u32)) as u32); rdx = 0x80; crate::simd_ops::wu32(rbx, (0x1u32) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rcx); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x30u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x40u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x48u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x50u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x58u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x60u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x68u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x70u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x78u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x80u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x88u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x90u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), rax); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rbx.wrapping_add(0x30u64), rax); r8 = ((rdi as u32)) as u32 as u64; fl_l = ((rax).wrapping_add(0x18u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x40u64), (0x4u32) as u32); bb = 8; continue; }
            8 => { rcx = rax.wrapping_sub(0x10); r9 = rdx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rax.wrapping_sub(0x8u64), rdi); crate::simd_ops::wu16(rax, (0xffffu16) as u16); bb = 10; continue; }
            10 => { rcx = rax.wrapping_add(0x8); r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu64(rdx, rcx); rax = ((rax).wrapping_add(0x20u64)) as u64; rdx = ((rdx).wrapping_add(0x20u64)) as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x40u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 8; continue; } bb = 11; continue; }
            11 => { rdx = 0x60; crate::simd_ops::wu64(r9, rdi); rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0x60u64), rax); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu32(rbx.wrapping_add(0x68u64), (0x4u32) as u32); rdx = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x58u64), rax); bb = 13; continue; }
            13 => { rax = rcx.wrapping_add(0x18); rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu64(rcx, rax); r8 = rcx; rcx = rax; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x68u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 13; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu64(r8, rdi); bb = 28; continue; }
            15 => { crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), rdi); bb = 16; continue; }
            16 => { rbp = (rbp & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 28; continue; }
            17 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 26; continue; } bb = 18; continue; }
            18 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x90u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 22; continue; } bb = 19; continue; }
            19 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x5u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; bb = 20; continue; }
            20 => { if (((fl_l as u8)) == ((fl_r as u8))) { bb = 26; continue; } bb = 21; continue; }
            21 => { fl_l = ((((rax as u8)) | (0x7u8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); bb = 27; continue; }
            22 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xfeu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 21; continue; } bb = 23; continue; }
            23 => { rax = ((crate::simd_ops::ru16(rdx.wrapping_add(0x4u64))) as u32) as u32 as u64; fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) == 0) { bb = 28; continue; } bb = 24; continue; }
            24 => { rax = (rax & !0xffffu64) | (((((rax as u16)).wrapping_sub(1)) as u16 as u64)); fl_l = ((rax as u16)) as u16 as u64; fl_r = (0x4u16) as u16 as u64; if (((fl_l as u16)) <= ((fl_r as u16))) { bb = 28; continue; } bb = 25; continue; }
            25 => { rbp = (rbp & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 28; continue; }
            26 => { fl_l = ((((rax as u8)) | (0x4u8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); bb = 27; continue; }
            27 => { crate::simd_ops::wu8(rsi.wrapping_add(0x38u64), ((rax as u8)) as u8); bb = 28; continue; }
            28 => { xmm0 = ((crate::simd_ops::ru64(r14)) as u64) as u128; rax = (((rbp as u8)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x28u64), ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x30u64), ((rcx as u32)) as u32); crate::simd_ops::wu32(rsi.wrapping_add(0x34u64), ((r15 as u32)) as u32); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160d630(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x48; const P2: u64 = 0x378; const P3: u64 = 0x1415970c0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    fl_l = rbx; fl_r = rbx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi);
    rdi = 0;
    fl_l = 0; fl_r = 0;
    loop { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rdx = rsp.wrapping_add(0x30); r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rcx.wrapping_add(P2), (0xffffffffu32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rdi as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), (0x4u32) as u32); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rbx; fl_r = rbx; if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { break; } }
    rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160d690(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1438152b4; const P1: u64 = 0x1415c2af0; const P2: u64 = 0x14160dc20; const P3: u64 = 0x1415970c0; const P4: u64 = 0x143930f08; const P5: u64 = 0x1415a5db0; const P6: u64 = 0x14160d150;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x88u64)) as u64; r10 = ((crate::simd_ops::ru8(rcx.wrapping_add(0x28u64))) as u32) as u32 as u64; r14 = rcx.wrapping_add(0x28); r12 = ((rdx as u32)) as u32 as u64; rdi = rcx; fl_l = ((r10 as u8)) as u8 as u64; fl_r = (0x90u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 14; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x5u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) <= ((fl_r as u8))) { bb = 16; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; r15 = 0; fl_l = 0; fl_r = 0; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 70; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = r15; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 70; continue; } bb = 4; continue; }
            4 => { fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 70; continue; } bb = 5; continue; }
            5 => { rcx = ((crate::simd_ops::ru8(rcx.wrapping_add(0x38u64))) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), rbx); fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 48; continue; } bb = 6; continue; }
            6 => { rax = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 48; continue; } bb = 7; continue; }
            7 => { fl_l = ((r10 as u8)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = ((r10 as u8)) as u8 as u64; fl_r = (0x90u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 47; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x2du64))) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 47; continue; } bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x48u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 44; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rsi); bb = 12; continue; }
            12 => { rsi = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rbp = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x90u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 35; continue; } bb = 13; continue; }
            13 => { rdx = ((rbp as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 36; continue; }
            14 => { fl_l = ((r10 as u8)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 15; continue; }
            15 => { fl_l = ((r10 as u8)) as u8 as u64; fl_r = (0x90u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 18; continue; } bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x2du64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 17; continue; }
            17 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x88u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            18 => { fl_l = ((r10 as u8)) as u8 as u64; fl_r = (0xfeu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 17; continue; } bb = 19; continue; }
            19 => { rdx = ((crate::simd_ops::ru8(rcx.wrapping_add(0x38u64))) as u32) as u32 as u64; rcx = (((rdx as u8)) as u32) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 26; continue; } bb = 20; continue; }
            20 => { rax = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 26; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 22; continue; }
            22 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 25; continue; } bb = 23; continue; }
            23 => { fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 25; continue; } bb = 24; continue; }
            24 => { rdx = r9; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::lk_or_u8(rdi.wrapping_add(0x38u64), (0x2u8) as u8)) as u8 as u64; fl_r = fl_l; rcx = ((crate::simd_ops::ru8(rdi.wrapping_add(0x38u64))) as u32) as u32 as u64; bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; r15 = 0x0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; r15 = (if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) >= 0) { (rax as u32) } else { (r15 as u32) }) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x34u64), ((r15 as u32)) as u32); bb = 26; continue; }
            26 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 34; continue; } bb = 27; continue; }
            27 => { fl_l = (crate::simd_ops::ru8(r14)) as u8 as u64; fl_r = (0xfeu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 17; continue; } bb = 28; continue; }
            28 => { fl_l = (crate::simd_ops::ru16(r14.wrapping_add(0x4u64))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 33; continue; } bb = 29; continue; }
            29 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x8u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 17; continue; } bb = 30; continue; }
            30 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 31; continue; }
            31 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x6cu64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 33; continue; } bb = 32; continue; }
            32 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x94u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 17; continue; } bb = 33; continue; }
            33 => { fl_l = (crate::simd_ops::ru16(rdi.wrapping_add(0x2cu64))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 17; continue; } bb = 34; continue; }
            34 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x88u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            35 => { crate::simd_ops::wu32(rsi.wrapping_add(0x98u64), ((rbp as u32)) as u32); bb = 36; continue; }
            36 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 38; continue; } bb = 37; continue; }
            37 => { crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), (0x3u32) as u32); rbp = 0xffffffff; bb = 38; continue; }
            38 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 41; continue; } bb = 39; continue; }
            39 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 42; continue; } bb = 40; continue; }
            40 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = 0xffffffff; crate::simd_ops::wu32(rsi.wrapping_add(0x378u64), ((rbp as u32)) as u32); rdx = rsp.wrapping_add(0xb0); crate::simd_ops::wu32(rsp.wrapping_add(0xb0u64), ((r15 as u32)) as u32); r8 = (r8 & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu32(rsp.wrapping_add(0xb4u64), (0x4u32) as u32); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 42; continue; }
            41 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = rsi; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 42; continue; }
            42 => { rbx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 43; continue; }
            43 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; bb = 44; continue; }
            44 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x70u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 47; continue; } bb = 45; continue; }
            45 => { bb = 46; continue; }
            46 => { r9 = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(P4)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 46; continue; } bb = 47; continue; }
            47 => { fl_l = (crate::simd_ops::lk_or_u8(rdi.wrapping_add(0x38u64), (0x2u8) as u8)) as u8 as u64; fl_r = fl_l; rcx = ((crate::simd_ops::ru8(rdi.wrapping_add(0x38u64))) as u32) as u32 as u64; bb = 48; continue; }
            48 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 62; continue; } bb = 49; continue; }
            49 => { fl_l = (crate::simd_ops::ru8(r14)) as u8 as u64; fl_r = (0x90u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 52; continue; } bb = 50; continue; }
            50 => { fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x5u64))) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if (((fl_l as u8)) <= ((fl_r as u8))) { bb = 52; continue; } bb = 51; continue; }
            51 => { r9 = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rcx = rdi; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); r8 = rdx; rax = r9.wrapping_add(0x18); r9 = (crate::simd_ops::ru64(r9.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 61; continue; }
            52 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 56; continue; } bb = 53; continue; }
            53 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 55; continue; } bb = 54; continue; }
            54 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rcx = rsp.wrapping_add(0x30); r9 = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; xmm0 = ((crate::simd_ops::ru64(r14)) as u64) as u128; r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x19u64))) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x31u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x18u64))) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); rcx = rdi; crate::simd_ops::wu8(rsp.wrapping_add(0x34u64), ((rax as u8)) as u8); rax = r9.wrapping_add(0x18); r9 = (crate::simd_ops::ru64(r9.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 55; continue; }
            55 => { rbx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 53; continue; } bb = 56; continue; }
            56 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x48u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 61; continue; } bb = 57; continue; }
            57 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 60; continue; } bb = 58; continue; }
            58 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; r10 = (crate::simd_ops::ru64(r8.wrapping_add(0x1d8u64))) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 60; continue; } bb = 59; continue; }
            59 => { rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x349u64))) as u32) as u32 as u64; r9 = (r9 & !0xffu64) | (((0x1u8) as u8 as u64)); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; xmm0 = ((crate::simd_ops::ru64(r14)) as u64) as u128; rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x31u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x34cu64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x34u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(r8.wrapping_add(0x358u64))) as u64; xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); rax = (crate::simd_ops::ru32(r8.wrapping_add(0x30u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x2cu64))) as u32 as u64; rdx = rsp.wrapping_add(0x40); crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((rcx as u32)) as u32); rcx = r10; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r8); r8 = ((r8).wrapping_add(0x90u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x64u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x6cu64), (0x3u32) as u32); rax = (crate::simd_ops::ru64(r10)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x138u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 60; continue; }
            60 => { rbx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 57; continue; } bb = 61; continue; }
            61 => { fl_l = (crate::simd_ops::lk_or_u8(rdi.wrapping_add(0x38u64), (0x1u8) as u8)) as u8 as u64; fl_r = fl_l; rcx = ((crate::simd_ops::ru8(rdi.wrapping_add(0x38u64))) as u32) as u32 as u64; bb = 62; continue; }
            62 => { fl_l = (crate::simd_ops::ru8(r14)) as u8 as u64; fl_r = (0x90u8) as u8 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 70; continue; } bb = 63; continue; }
            63 => { fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x5u64))) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if (((fl_l as u8)) <= ((fl_r as u8))) { bb = 70; continue; } bb = 64; continue; }
            64 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 70; continue; } bb = 65; continue; }
            65 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 70; continue; } bb = 66; continue; }
            66 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x6cu64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 70; continue; } bb = 67; continue; }
            67 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x94u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 70; continue; } bb = 68; continue; }
            68 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 70; continue; } bb = 69; continue; }
            69 => { fl_l = (crate::simd_ops::lk_or_u8(rdi.wrapping_add(0x38u64), (0x4u8) as u8)) as u8 as u64; fl_r = fl_l; bb = 70; continue; }
            70 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; r15 = (if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) >= 0) { (rax as u32) } else { (r15 as u32) }) as u32 as u64; rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x38u64))) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x6u8))) as u8 as u64)); crate::simd_ops::wu32(rdi.wrapping_add(0x34u64), ((r15 as u32)) as u32); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x6u8) as u8 as u64; rax = (rax & !0xffu64) | (((if (((fl_l as u8)) == ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64)); rsp = ((rsp).wrapping_add(0x88u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160db00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x48; const P2: u64 = 0x141596ba0; const P3: u64 = 0x70; const P4: u64 = 0x143930f08; const P5: u64 = 0x1415a6110;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); rdi = rcx; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); rsi = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rdx = rsp.wrapping_add(0x30); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), (0x4u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rsi as u32)) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 3; continue; }
            3 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rcx = (crate::simd_ops::ru64(P4)) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 7; continue; }
            7 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160db80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x48; const P2: u64 = 0x328; const P3: u64 = 0x141596f00; const P4: u64 = 0x70; const P5: u64 = 0x143930f08; const P6: u64 = 0x1415aa9b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); rsi = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); rdi = ((rdx as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbp); rbp = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rdx = rsp.wrapping_add(0x30); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), (0x4u32) as u32); r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rbp as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rdi as u32)) as u32); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 3; continue; }
            3 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rcx = (crate::simd_ops::ru64(P5)) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 7; continue; }
            7 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160dc20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14160d150; const P1: u64 = 0x1415970c0; const P2: u64 = 0x141596ba0; const P3: u64 = 0x143930f08; const P4: u64 = 0x1415a6110; const P5: u64 = 0x141596f00; const P6: u64 = 0x1415aa9b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x58u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x28u64))) as u8 as u64; fl_r = (0xfeu8) as u8 as u64; r15 = rcx.wrapping_add(0x28); rbp = rcx; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru16(r15.wrapping_add(0x4u64))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 3; continue; } bb = 2; continue; }
            2 => { r9 = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r15); r8 = rdx; rax = r9.wrapping_add(0x18); r9 = (crate::simd_ops::ru64(r9.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x58u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r12); r12 = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_l = r12; fl_r = r12; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 52; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r13); r13 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r14); bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(r12.wrapping_add(0x28u64))) as u8 as u64; fl_r = (0xfeu8) as u8 as u64; r14 = r12; r12 = (crate::simd_ops::ru64(r12)) as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 50; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru16(r14.wrapping_add(0x2cu64))) as u16 as u64; fl_r = ((r13 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 50; continue; } bb = 7; continue; }
            7 => { rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x38u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 50; continue; } bb = 8; continue; }
            8 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x8u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 50; continue; } bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 50; continue; } bb = 10; continue; }
            10 => { rax = ((crate::simd_ops::ru8(r15)) as u32) as u32 as u64; rdi = r15.wrapping_add(0x4); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xfeu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 50; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru16(rdi)) as u16 as u64; fl_r = (0x4u16) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 16; continue; } bb = 12; continue; }
            12 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 50; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru16(rdi)) as u16 as u64; fl_r = (0x5u16) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 16; continue; } bb = 14; continue; }
            14 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 50; continue; } bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru16(rbp.wrapping_add(0x2cu64))) as u16 as u64; fl_r = (0x1u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 35; continue; } bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x48u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 50; continue; } bb = 17; continue; }
            17 => { bb = 18; continue; }
            18 => { rdx = ((crate::simd_ops::ru8(r15)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rbx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0xfeu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 24; continue; } bb = 19; continue; }
            19 => { fl_l = (crate::simd_ops::ru16(rdi)) as u16 as u64; fl_r = (0x1u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 21; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x34u64))) as u32 as u64; rdx = rsp.wrapping_add(0x70); r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rcx.wrapping_add(0x378u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((r13 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x74u64), (0x4u32) as u32); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_or_u8(r14.wrapping_add(0x38u64), (0x8u8) as u8)) as u8 as u64; fl_r = fl_l; bb = 33; continue; }
            21 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0xfeu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 24; continue; } bb = 22; continue; }
            22 => { fl_l = (crate::simd_ops::ru16(rbp.wrapping_add(0x2cu64))) as u16 as u64; fl_r = (0x4u16) as u16 as u64; rax = rbp.wrapping_add(0x2c); if (((fl_l as u16)) != ((fl_r as u16))) { bb = 25; continue; } bb = 23; continue; }
            23 => { bb = 27; continue; }
            24 => { rax = rbp.wrapping_add(0x2c); bb = 25; continue; }
            25 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0xfeu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 33; continue; } bb = 26; continue; }
            26 => { fl_l = (crate::simd_ops::ru16(rax)) as u16 as u64; fl_r = (0x5u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 33; continue; } bb = 27; continue; }
            27 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0xfeu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 33; continue; } bb = 28; continue; }
            28 => { fl_l = (crate::simd_ops::ru16(rax)) as u16 as u64; fl_r = (0x4u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 30; continue; } bb = 29; continue; }
            29 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0x30u64))) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 33; continue; }
            30 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0xfeu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 33; continue; } bb = 31; continue; }
            31 => { fl_l = (crate::simd_ops::ru16(rax)) as u16 as u64; fl_r = (0x5u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 33; continue; } bb = 32; continue; }
            32 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0x30u64))) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 33; continue; }
            33 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 18; continue; } bb = 34; continue; }
            34 => { bb = 50; continue; }
            35 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xfeu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 50; continue; } bb = 36; continue; }
            36 => { fl_l = (crate::simd_ops::ru16(rbp.wrapping_add(0x2cu64))) as u16 as u64; fl_r = (0x2u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 43; continue; } bb = 37; continue; }
            37 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x48u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 39; continue; } bb = 38; continue; }
            38 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdx = rsp.wrapping_add(0x80); crate::simd_ops::wu32(rsp.wrapping_add(0x84u64), (0x4u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x80u64), ((r13 as u32)) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 38; continue; } bb = 39; continue; }
            39 => { rbx = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x70u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 50; continue; } bb = 40; continue; }
            40 => { bb = 41; continue; }
            41 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(P3)) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 41; continue; } bb = 42; continue; }
            42 => { bb = 50; continue; }
            43 => { fl_l = (crate::simd_ops::ru16(rbp.wrapping_add(0x2cu64))) as u16 as u64; fl_r = (0x3u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 50; continue; } bb = 44; continue; }
            44 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x48u64))) as u64; rdi = (crate::simd_ops::ru32(rbp.wrapping_add(0x34u64))) as u32 as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 47; continue; } bb = 45; continue; }
            45 => { bb = 46; continue; }
            46 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdx = rsp.wrapping_add(0x88); crate::simd_ops::wu32(rsp.wrapping_add(0x8cu64), (0x4u32) as u32); r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x88u64), ((r13 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x328u64), ((rdi as u32)) as u32); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 46; continue; } bb = 47; continue; }
            47 => { rbx = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x70u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 50; continue; } bb = 48; continue; }
            48 => { bb = 49; continue; }
            49 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(P3)) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 49; continue; } bb = 50; continue; }
            50 => { fl_l = r12; fl_r = r12; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 51; continue; }
            51 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; bb = 52; continue; }
            52 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x58u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160df00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x28; const P2: u64 = 0x2; const P3: u64 = 0x141524330; const P4: u64 = 0x10; const P5: u64 = 0xc; const P6: u64 = 0x18; const P7: u64 = 0x14; const P8: u64 = 0x20; const P9: u64 = 0x1c; const P10: u64 = 0x24; const P11: u64 = 0x4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = 0; fl_l = 0; fl_r = 0; rbp = ((rdx as u32)) as u32 as u64; rsi = (((r8 as u8)) as u32) as u32 as u64; rdi = rcx; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 2; continue; }
            2 => { rdx = ((rbx.wrapping_add(P1)) as u32) as u32 as u64; rcx = ((rbx.wrapping_add(P2)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r9 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rax, rbx); crate::simd_ops::wu16(rax.wrapping_add(P0), ((rbx as u16)) as u16); crate::simd_ops::wu16(rax.wrapping_add(P4), ((rbx as u16)) as u16); crate::simd_ops::wu32(rax.wrapping_add(P5), ((rbx as u32)) as u32); crate::simd_ops::wu16(rax.wrapping_add(P6), ((rbx as u16)) as u16); crate::simd_ops::wu32(rax.wrapping_add(P7), ((rbx as u32)) as u32); crate::simd_ops::wu16(rax.wrapping_add(P8), ((rbx as u16)) as u16); crate::simd_ops::wu32(rax.wrapping_add(P9), ((rbx as u32)) as u32); crate::simd_ops::wu8(rax.wrapping_add(P10), ((rbx as u8)) as u8); crate::simd_ops::wu64(rdi.wrapping_add(P0), rax); bb = 4; continue; }
            4 => { rcx = ((rbp.wrapping_mul(8)) as u64) as u64; crate::simd_ops::wu8(rcx.wrapping_add(r9).wrapping_add(P0), ((rsi as u8)) as u8); rax = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(rax).wrapping_add(P11))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu8(rcx.wrapping_add(rax).wrapping_add(0x9u64), ((rbx as u8)) as u8); crate::simd_ops::wu32(rcx.wrapping_add(rax).wrapping_add(0x4u64), ((rbx as u32)) as u32); rcx = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi)) as u64; rdx = ((rbp as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rax = 0x1; bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            9 => { crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rbx); rax = 0x34; bb = 8; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160dfc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x28; const P2: u64 = 0x2; const P3: u64 = 0x141524330; const P4: u64 = 0x10; const P5: u64 = 0xc; const P6: u64 = 0x18; const P7: u64 = 0x14; const P8: u64 = 0x20; const P9: u64 = 0x1c; const P10: u64 = 0x24; const P11: u64 = 0x9;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = ((rdx as u32)) as u32 as u64; rbp = (((r9 as u8)) as u32) as u32 as u64; rsi = ((r8 as u32)) as u32 as u64; rbx = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x1f; bb = 10; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rdx = ((rax.wrapping_add(P1)) as u32) as u32 as u64; rcx = ((rax.wrapping_add(P2)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = 0; fl_l = 0; fl_r = 0; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rax, rcx); crate::simd_ops::wu16(rax.wrapping_add(P0), ((rcx as u16)) as u16); crate::simd_ops::wu16(rax.wrapping_add(P4), ((rcx as u16)) as u16); crate::simd_ops::wu32(rax.wrapping_add(P5), ((rcx as u32)) as u32); crate::simd_ops::wu16(rax.wrapping_add(P6), ((rcx as u16)) as u16); crate::simd_ops::wu32(rax.wrapping_add(P7), ((rcx as u32)) as u32); crate::simd_ops::wu16(rax.wrapping_add(P8), ((rcx as u16)) as u16); crate::simd_ops::wu32(rax.wrapping_add(P9), ((rcx as u32)) as u32); crate::simd_ops::wu8(rax.wrapping_add(P10), ((rcx as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(P0), rax); bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x50u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 9; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu32(rax, ((rcx as u32)) as u32); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(rdi.wrapping_mul(8)).wrapping_add(P11))) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(rdi.wrapping_mul(8)).wrapping_add(0x4u64))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu8(rax.wrapping_add(rdi.wrapping_mul(8)).wrapping_add(0x9u64), ((rbp as u8)) as u8); crate::simd_ops::wu32(rax.wrapping_add(rdi.wrapping_mul(8)).wrapping_add(0x4u64), ((rsi as u32)) as u32); rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = ((rdi as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rax = 0x1; bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            11 => { crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rcx); rax = 0x34; bb = 10; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160e090(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x28; const P2: u64 = 0xffffffffffffffda; const P3: u64 = 0x141524330; const P4: u64 = 0x10; const P5: u64 = 0xc; const P6: u64 = 0x18; const P7: u64 = 0x14; const P8: u64 = 0x20; const P9: u64 = 0x1c; const P10: u64 = 0x24;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = ((r8 as u32)) as u32 as u64; rsi = ((rdx as u32)) as u32 as u64; rdi = rcx; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { rdx = ((r9.wrapping_add(P1)) as u32) as u32 as u64; rcx = ((rdx.wrapping_add(P2)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r9 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu16(r9.wrapping_add(P0), ((rax as u16)) as u16); crate::simd_ops::wu16(r9.wrapping_add(P4), ((rax as u16)) as u16); crate::simd_ops::wu32(r9.wrapping_add(P5), ((rax as u32)) as u32); crate::simd_ops::wu16(r9.wrapping_add(P6), ((rax as u16)) as u16); crate::simd_ops::wu32(r9.wrapping_add(P7), ((rax as u32)) as u32); crate::simd_ops::wu16(r9.wrapping_add(P8), ((rax as u16)) as u16); crate::simd_ops::wu32(r9.wrapping_add(P9), ((rax as u32)) as u32); crate::simd_ops::wu8(r9.wrapping_add(P10), ((rax as u8)) as u8); crate::simd_ops::wu64(rdi.wrapping_add(P0), r9); bb = 4; continue; }
            4 => { rax = (((rbx as u8)) as u32) as u32 as u64; rbx = (rbx & !0xffu64) | ((((((rbx as u8)) & ((rsi as u8)))) as u8 as u64)); rax = (rax & !0xffu64) | (((!((rax as u8))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) & (crate::simd_ops::ru8(r9.wrapping_add(P10))))) as u8 as u64)); fl_l = ((((rax as u8)) | ((rbx as u8)))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(r9.wrapping_add(P10), ((rax as u8)) as u8); bb = 5; continue; }
            5 => { rax = 0x1; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P0), rax); rax = 0x34; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160e140(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524330;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; r14 = rdx; rsi = rcx; rdi = 0x1; r8 = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(rdx, rax); r13 = ((r8 as u32)) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 29; continue; } bb = 1; continue; }
            1 => { rbx = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rdx = ((rdi.wrapping_add(0x27u64)) as u32) as u32 as u64; rcx = ((rdi.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rax, rbx); crate::simd_ops::wu16(rax.wrapping_add(0x8u64), ((rbx as u16)) as u16); crate::simd_ops::wu16(rax.wrapping_add(0x10u64), ((rbx as u16)) as u16); crate::simd_ops::wu32(rax.wrapping_add(0xcu64), ((rbx as u32)) as u32); crate::simd_ops::wu16(rax.wrapping_add(0x18u64), ((rbx as u16)) as u16); crate::simd_ops::wu32(rax.wrapping_add(0x14u64), ((rbx as u32)) as u32); crate::simd_ops::wu16(rax.wrapping_add(0x20u64), ((rbx as u16)) as u16); crate::simd_ops::wu32(rax.wrapping_add(0x1cu64), ((rbx as u32)) as u32); crate::simd_ops::wu8(rax.wrapping_add(0x24u64), ((rbx as u8)) as u8); crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rax); bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r15); r15 = ((rbx as u32)) as u32 as u64; rbp = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rcx = rax.wrapping_add(0x1); crate::simd_ops::wu8(rsp.wrapping_add(0x58u64), ((rbp as u8)) as u8); crate::simd_ops::wu64(r14, rcx); fl_l = ((r13 as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 23; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r12); bb = 6; continue; }
            6 => { rbp = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; rdi = 0x1; rcx = ((rcx).wrapping_add(1)) as u64; crate::simd_ops::wu64(r14, rcx); rax = rcx.wrapping_add(0x4); r12 = (crate::simd_ops::ru32(rcx)) as u32 as u64; rcx = rax.wrapping_add(0x2); crate::simd_ops::wu64(r14, rax); rdx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; crate::simd_ops::wu64(r14, rcx); fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 17; continue; } bb = 7; continue; }
            7 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; rdi = (rdi & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = ((rbp as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 21; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { rdx = ((rax.wrapping_add(0x28u64)) as u32) as u32 as u64; rcx = ((rax.wrapping_add(0x2u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rax, rbx); crate::simd_ops::wu16(rax.wrapping_add(0x8u64), ((rbx as u16)) as u16); crate::simd_ops::wu16(rax.wrapping_add(0x10u64), ((rbx as u16)) as u16); crate::simd_ops::wu32(rax.wrapping_add(0xcu64), ((rbx as u32)) as u32); crate::simd_ops::wu16(rax.wrapping_add(0x18u64), ((rbx as u16)) as u16); crate::simd_ops::wu32(rax.wrapping_add(0x14u64), ((rbx as u32)) as u32); crate::simd_ops::wu16(rax.wrapping_add(0x20u64), ((rbx as u16)) as u16); crate::simd_ops::wu32(rax.wrapping_add(0x1cu64), ((rbx as u32)) as u32); crate::simd_ops::wu8(rax.wrapping_add(0x24u64), ((rbx as u8)) as u8); crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rax); rcx = (crate::simd_ops::ru64(r14)) as u64; bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 16; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu32(rax, ((rbx as u32)) as u32); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rbp.wrapping_mul(8)).wrapping_add(0x9u64))) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(rbp.wrapping_mul(8)).wrapping_add(0x4u64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 15; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu8(rcx.wrapping_add(rbp.wrapping_mul(8)).wrapping_add(0x9u64), ((rdi as u8)) as u8); crate::simd_ops::wu32(rcx.wrapping_add(rbp.wrapping_mul(8)).wrapping_add(0x4u64), ((r12 as u32)) as u32); rcx = (crate::simd_ops::ru64(rsi)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi)) as u64; rdx = ((rbp as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(r14)) as u64; bb = 16; continue; }
            16 => { rdi = 0x1; bb = 17; continue; }
            17 => { r15 = (((r15 as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 6; continue; } bb = 18; continue; }
            18 => { bb = 22; continue; }
            19 => { crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rbx); rax = 0x34; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            20 => { crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rbx); rdi = 0x34; bb = 22; continue; }
            21 => { rdi = 0x1f; bb = 22; continue; }
            22 => { rbp = ((crate::simd_ops::ru8(rsp.wrapping_add(0x58u64))) as u32) as u32 as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 27; continue; } bb = 24; continue; }
            24 => { fl_l = ((rbp as u8)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 28; continue; } bb = 25; continue; }
            25 => { rdx = ((rax.wrapping_add(0x28u64)) as u32) as u32 as u64; rcx = ((rax.wrapping_add(0x2u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 26; continue; }
            26 => { crate::simd_ops::wu64(rax, rbx); crate::simd_ops::wu16(rax.wrapping_add(0x8u64), ((rbx as u16)) as u16); crate::simd_ops::wu16(rax.wrapping_add(0x10u64), ((rbx as u16)) as u16); crate::simd_ops::wu32(rax.wrapping_add(0xcu64), ((rbx as u32)) as u32); crate::simd_ops::wu16(rax.wrapping_add(0x18u64), ((rbx as u16)) as u16); crate::simd_ops::wu32(rax.wrapping_add(0x14u64), ((rbx as u32)) as u32); crate::simd_ops::wu16(rax.wrapping_add(0x20u64), ((rbx as u16)) as u16); crate::simd_ops::wu32(rax.wrapping_add(0x1cu64), ((rbx as u32)) as u32); crate::simd_ops::wu8(rax.wrapping_add(0x24u64), ((rbx as u8)) as u8); crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rax); bb = 27; continue; }
            27 => { crate::simd_ops::wu8(rax.wrapping_add(0x24u64), ((rbp as u8)) as u8); bb = 28; continue; }
            28 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 29; continue; }
            29 => { rax = ((rdi as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            30 => { crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rbx); bb = 28; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160e340(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rbx = rcx;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = 0x2; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P0), (0x0u64) as u64); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160e370(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f88; const P1: u64 = 0x1426452b0; const P2: u64 = 0x1426452a8; const P3: u64 = 0x142645560;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; crate::simd_ops::wu64(rdx, (0x0u64) as u64); r14 = rdx; rbp = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 54; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x5u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = (crate::simd_ops::ru64(P0)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); if (((fl_l as u8)) == ((fl_r as u8))) { bb = 26; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x3c8u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 14; continue; } bb = 4; continue; }
            4 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x3c8u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 5; continue; }
            5 => { rsi = rbx.wrapping_add(0x3d0); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x410u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 16; continue; } bb = 7; continue; }
            7 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbp as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x408u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 11; continue; }
            11 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 12; continue; }
            12 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 17; continue; }
            14 => { rsi = rbx.wrapping_add(0x3d0); rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x3c8u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 6; continue; } bb = 15; continue; }
            15 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            16 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x3c8u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 25; continue; } bb = 18; continue; }
            18 => { bb = 19; continue; }
            19 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x3c8u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 22; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x3c8u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 19; continue; } bb = 21; continue; }
            21 => { bb = 49; continue; }
            22 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 50; continue; } bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x3f8u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 50; continue; } bb = 24; continue; }
            24 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x400u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 50; continue; }
            25 => { rsi = rbx.wrapping_add(0x3d0); bb = 49; continue; }
            26 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x370u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 37; continue; } bb = 27; continue; }
            27 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x370u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 26; continue; } bb = 28; continue; }
            28 => { rsi = rbx.wrapping_add(0x378); bb = 29; continue; }
            29 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x3b8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 39; continue; } bb = 30; continue; }
            30 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbp as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x3b0u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 34; continue; } bb = 31; continue; }
            31 => { bb = 32; continue; }
            32 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 35; continue; } bb = 33; continue; }
            33 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 32; continue; } bb = 34; continue; }
            34 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 35; continue; }
            35 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 40; continue; } bb = 36; continue; }
            36 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 40; continue; }
            37 => { rsi = rbx.wrapping_add(0x378); rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x370u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 29; continue; } bb = 38; continue; }
            38 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            39 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 40; continue; }
            40 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x370u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 48; continue; } bb = 41; continue; }
            41 => { bb = 42; continue; }
            42 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x370u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 45; continue; } bb = 43; continue; }
            43 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x370u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 42; continue; } bb = 44; continue; }
            44 => { bb = 49; continue; }
            45 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 50; continue; } bb = 46; continue; }
            46 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x3a0u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 50; continue; } bb = 47; continue; }
            47 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x3a8u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 50; continue; }
            48 => { rsi = rbx.wrapping_add(0x378); bb = 49; continue; }
            49 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 50; continue; }
            50 => { rcx = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(r14, rdi); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 53; continue; } bb = 51; continue; }
            51 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 53; continue; } bb = 52; continue; }
            52 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 53; continue; }
            53 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = r14; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            54 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = r14; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160e5c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14268f100; const P1: u64 = 0x18; const P2: u64 = 0x1; const P3: u64 = 0x143930f80; const P4: u64 = 0x1415adef0; const P5: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = P0; rsi = rcx; crate::simd_ops::wu64(rcx, rax); rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rdx = ((rcx.wrapping_add(P2)) as u64) as u64; rbx = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; rax = 0xfffffff8; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); rbx = ((rbx).wrapping_add(0x8u64)) as u64; rdi = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; rbx = (((rbx) & (rax))) as u64; rdi = ((rdi).wrapping_add(1)) as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rdi = ((rdi).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 2; continue; }
            2 => { rbx = ((rbx).wrapping_add(rcx)) as u64; fl_l = rdi; fl_r = rdx; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(P3)) as u64; r8 = rsi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rsi.wrapping_add(P1), (0x0u64) as u64); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160e660(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426894c0; const P1: u64 = 0x14159df30; const P2: u64 = 0x141524330; const P3: u64 = 0x14183acbc; const P4: u64 = 0x1415242e0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r10 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rsi = rcx; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); xmm6 = (xmm2) as u128; rbp = ((((rdx as u32)) as i32 as i64) as u64) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; r12 = 0xfffffff8; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; bb = 3; continue; }
            3 => { rcx = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(r8))) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 9; continue; } bb = 4; continue; }
            4 => { rdx = ((rcx as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 3; continue; } bb = 5; continue; }
            5 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { rcx = P0; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(rbp.wrapping_mul(4)))) as u32 as u64; rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 17; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { rdi = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; bb = 12; continue; }
            9 => { rax = ((r9.wrapping_add(0x8u64)) as u32) as u32 as u64; rbx = ((rdx as u32)) as u32 as u64; rax = (((rax) & (r12))) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(r8)) as u64; fl_l = ((rbx).wrapping_add(rax)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; if (((fl_l as u64)) != 0) { bb = 17; continue; } bb = 10; continue; }
            10 => { bb = 6; continue; }
            11 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 12; continue; }
            12 => { rdx = ((rdi.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rdx = (((rdx as u32)).wrapping_shl(4u32)) as u32 as u64; r15 = ((rdi.wrapping_add(0x9u64)) as u32) as u32 as u64; rax = ((r15 as u32)) as u32 as u64; rax = ((((rax as u32)) & ((r12 as u32)))) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 13; continue; }
            13 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; rbx = ((rdi as u32)) as u32 as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = rax.wrapping_add(0x1); r8 = ((rbx as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = ((rdi.wrapping_add(0x8u64)) as u32) as u32 as u64; rcx = ((r15 as u32)) as u32 as u64; rdx = (((rdx) & (r12))) as u64; rcx = (((rcx) & (r12))) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0x18u64)))) as u64; r8 = ((rbx as u32)) as u32 as u64; rcx = ((rcx).wrapping_add(r14)) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 15; continue; }
            15 => { r15 = (((r15) & (r12))) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rax = ((rdi.wrapping_add(0x1u64)) as u32) as u32 as u64; rdi = (rdi & !0xffu64) | (((((rdi as u8)).wrapping_add(1)) as u8 as u64)); crate::simd_ops::wu8(rax.wrapping_add(r14), ((rbp as u8)) as u8); crate::simd_ops::wu8(r14, ((rdi as u8)) as u8); rcx = ((r14.wrapping_add(r15)) as u64) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x18u64), r14); fl_l = ((rbx).wrapping_add(rcx)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 18; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu32(rbx, ((xmm6 as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), (0x0u64) as u64); bb = 17; continue; }
            17 => { rax = rbx; bb = 19; continue; }
            18 => { rax = 0; fl_l = 0; fl_r = 0; bb = 19; continue; }
            19 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160e7d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426894c0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_r = (0x0u64) as u64; rsi = (((r9 as u8)) as u32) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm6) as u128); r11 = rdx; xmm6 = (xmm2) as u128; rdi = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 1; continue; }
            1 => { r10 = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { rdx = ((crate::simd_ops::ru8(r10)) as u32) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; bb = 3; continue; }
            3 => { rcx = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(r10))) as u8 as u64; fl_r = ((r11 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 4; continue; }
            4 => { r8 = ((rcx as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 3; continue; } bb = 5; continue; }
            5 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = ((r11 as u32)) as u32 as u64; fl_r = 0xb; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 7; continue; }
            7 => { r9 = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rdx = P0; xmm0 = (xmm6) as u128; rax = ((((r11 as u32)) as i32 as i64) as u64) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(crate::simd_ops::ru32(rbx))).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x0u32) as u32); r8 = rsp.wrapping_add(0x30); crate::simd_ops::wu16(rsp.wrapping_add(0x34u64), (0x1u16) as u16); rdx = (crate::simd_ops::ru32(rdx.wrapping_add(rax.wrapping_mul(4)))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x0u32) as u32); r10 = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm6 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(r10.wrapping_add(0x228u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            8 => { rax = ((rdx.wrapping_add(0x8u64)) as u32) as u32 as u64; rbx = ((r8 as u32)) as u32 as u64; rcx = 0xfffffff8; rbx = ((rbx).wrapping_shl(4u32)) as u64; rax = (((rax) & (rcx))) as u64; rax = ((rax).wrapping_add(r10)) as u64; fl_l = ((rbx).wrapping_add(rax)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 6; continue; }
            9 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x38u64))) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; xmm1 = (xmm6) as u128; fl_l = r8; fl_r = r8; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx); rdx = (rdx & !0xffu64) | (((if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); rax = ((((rax as u32)) & (0xffffff3cu32))) as u32 as u64; rdx = ((((rdx as u32)) & (0xffffff3fu32))) as u32 as u64; rdx = ((((rdx as u32)) | ((rax as u32)))) as u32 as u64; rdx = ((((rdx as u32)) | (0x3cu32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rdx as u32)) as u32); rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { r9 = rsp.wrapping_add(0x30); crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0x0u8) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0xb8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            11 => { r9 = 0; fl_l = 0; fl_r = 0; r8 = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0xc0u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { crate::simd_ops::wu32(rbx, ((xmm6 as u32)) as u32); fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), (0x0u64) as u64); bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; bb = 15; continue; }
            15 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160e910(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14268f100; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x24; const P6: u64 = 0x28; const P7: u64 = 0x2c; const P8: u64 = 0x30; const P9: u64 = 0x34; const P10: u64 = 0x38; const P11: u64 = 0x3c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rcx.wrapping_add(P0), rdx); rax = P1; crate::simd_ops::wu64(rcx, rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P2), rax); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx.wrapping_add(P3), rax); crate::simd_ops::wu8(rcx.wrapping_add(P4), ((r8 as u8)) as u8); crate::simd_ops::wu32(rcx.wrapping_add(P5), ((rax as u32)) as u32); crate::simd_ops::wu16(rcx.wrapping_add(P6), ((rax as u16)) as u16); crate::simd_ops::wu32(rcx.wrapping_add(P7), ((rax as u32)) as u32); crate::simd_ops::wu16(rcx.wrapping_add(P8), ((rax as u16)) as u16); crate::simd_ops::wu32(rcx.wrapping_add(P9), ((rax as u32)) as u32); crate::simd_ops::wu16(rcx.wrapping_add(P10), ((rax as u16)) as u16); crate::simd_ops::wu32(rcx.wrapping_add(P11), ((rax as u32)) as u32); crate::simd_ops::wu16(rcx.wrapping_add(0x40u64), ((rax as u16)) as u16); fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(r9)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x24u64), rax); rax = (crate::simd_ops::ru64(r9.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x2cu64), rax); rax = (crate::simd_ops::ru64(r9.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x34u64), rax); rax = (crate::simd_ops::ru64(r9.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x3cu64), rax); rax = rcx; return rax; }
            2 => { bb = 3; continue; }
            3 => { crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); crate::simd_ops::wu16(rsp.wrapping_add(0x2cu64), ((rax as u16)) as u16); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x24u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x2cu64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x34u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x3cu64), rax); rax = rcx; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160e9b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x14160e5c0; const P3: u64 = 0x1415242e0; const P4: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbp = 0; fl_l = 0; fl_r = 0; rsi = rcx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rdi = (crate::simd_ops::ru64(rbx)) as u64; rax = ((rax).wrapping_add(rdi)) as u64; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0x4; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64; rdi = ((rdi).wrapping_add(0x10u64)) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 6; continue; }
            6 => { rdi = rcx; bb = 7; continue; }
            7 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = rdi; crate::simd_ops::wu32(rbx.wrapping_add(P1), ((rbp as u32)) as u32); rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rbp); crate::simd_ops::wu32(rbx.wrapping_add(P4), ((rbp as u32)) as u32); bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = 0x4; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rsi.wrapping_add(P0), rbp); bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(P1))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0x4; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi.wrapping_add(P1), rbp); bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160eaa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x20;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rcx; r10 = ((rdx as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { r9 = (((r8 as u8)) as u32) as u32 as u64; rax = (((r10 as u8)) as u32) as u32 as u64; r9 = (r9 & !0xffu64) | (((!((r9 as u8))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) & ((r8 as u8)))) as u8 as u64)); r9 = (r9 & !0xffu64) | ((((((r9 as u8)) & (crate::simd_ops::ru8(rcx.wrapping_add(P1))))) as u8 as u64)); fl_l = ((((r9 as u8)) | ((rax as u8)))) as u8 as u64; fl_r = fl_l; r9 = (r9 & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rcx.wrapping_add(P1), ((r9 as u8)) as u8); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { r9 = (((r8 as u8)) as u32) as u32 as u64; fl_l = ((((r10 as u8)) & ((r8 as u8)))) as u8 as u64; fl_r = fl_l; r10 = (r10 & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); r9 = (r9 & !0xffu64) | (((!((r9 as u8))) as u8 as u64)); bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; rax = ((rax).wrapping_add(0x10u64)) as u64; rcx = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64; rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & ((r9 as u8)))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) | ((r10 as u8)))) as u8 as u64)); crate::simd_ops::wu8(rdx.wrapping_add(P1), ((rcx as u8)) as u8); rcx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rcx))) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160eb20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14159b8a0; const P1: u64 = 0x143930fa0; const P2: u64 = 0x14153ad70;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rbx = r8; rdi = rdx; rsi = rcx; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(r8.wrapping_add(0x58u64))) as u64; rbp = (0x200000000u64) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { r14 = r8.wrapping_add(0x28); bb = 5; continue; }
            4 => { rax = (crate::simd_ops::ru64(r8.wrapping_add(0x28u64))) as u64; r14 = r8.wrapping_add(0x28); rdx = 0x21; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 5; continue; }
            5 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm6) as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); rdx = 0x21; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); rcx = r14; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); xmm6 = 0u128; rax = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0xffu8) as u8); crate::simd_ops::wu8(rsp.wrapping_add(0x40u64), (0xffu8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = 0x21; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm6 = (xmm0) as u128; bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rdx = rbx.wrapping_add(0x48); r9 = rsp.wrapping_add(0x20); r8 = 0x21; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 10; continue; }
            10 => { rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm6 as u32)))) as u32 as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; bb = 23; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 14; continue; }
            14 => { rax = ((rax).wrapping_add(0x10u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 16; continue; }
            16 => { rax = ((crate::simd_ops::ru8(rax.wrapping_add(0x20u64))) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shr(4u32)) as u8 as u64)); fl_l = ((((rax as u8)) & (0x1u8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); rax = (((rax as u8)) as u32) as u32 as u64; bb = 23; continue; }
            17 => { fl_l = ((rax).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 15; continue; } bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(rax)) as u64; rcx = ((crate::simd_ops::ru8(rax.wrapping_add(0x20u64))) as u32) as u32 as u64; rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_shr(4u32)) as u8 as u64)); fl_l = ((((rcx as u8)) & (0x1u8))) as u8 as u64; fl_r = fl_l; rcx = (rcx & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); rax = (((rcx as u8)) as u32) as u32 as u64; bb = 23; continue; }
            19 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x24u64))) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { rax = 0x1; bb = 23; continue; }
            22 => { rax = 0; fl_l = 0; fl_r = 0; bb = 23; continue; }
            23 => { r11 = rsp.wrapping_add(0x60); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160eca0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14159b8a0; const P1: u64 = 0x143930fa0; const P2: u64 = 0x14153ad70;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rbp = ((rdx as u32)) as u32 as u64; rbx = r9; rsi = r8; r14 = rcx; rdi = ((rbp.wrapping_add(0x1du64)) as u32) as u32 as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(rbp.wrapping_mul(8)).wrapping_add(0x4u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(r9.wrapping_add(0x58u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rax)) as u64; rcx = ((rdi as u32)) as u32 as u64; fl_l = (((rax) >> (((rcx) as u32 & 63))) & 1) as u64; if ((fl_l as u64) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { r15 = r9.wrapping_add(0x28); bb = 7; continue; }
            6 => { rax = (crate::simd_ops::ru64(r9.wrapping_add(0x28u64))) as u64; r15 = r9.wrapping_add(0x28); rdx = ((rdi as u32)) as u32 as u64; rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 7; continue; }
            7 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm6) as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); rdx = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); rcx = r15; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); xmm6 = 0u128; rax = (crate::simd_ops::ru64(r15)) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0xffu8) as u8); crate::simd_ops::wu8(rsp.wrapping_add(0x40u64), (0xffu8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = ((rdi as u32)) as u32 as u64; rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm6 = (xmm0) as u128; bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rax)) as u64; rcx = ((rdi as u32)) as u32 as u64; fl_l = (((rax) >> (((rcx) as u32 & 63))) & 1) as u64; if ((fl_l as u64) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rdx = rbx.wrapping_add(0x48); r9 = rsp.wrapping_add(0x20); r8 = ((rdi as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 12; continue; }
            12 => { rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm6 as u32)))) as u32 as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; bb = 25; continue; }
            13 => { rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 16; continue; }
            16 => { rax = ((rax).wrapping_add(0x10u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 18; continue; }
            18 => { rax = ((crate::simd_ops::ru8(rax.wrapping_add(0x20u64))) as u32) as u32 as u64; bb = 24; continue; }
            19 => { fl_l = ((rax).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 17; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(rax)) as u64; rcx = (((rbp as u8)) as u32) as u32 as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(0x20u64))) as u32) as u32 as u64; rdx = (rdx & !0xffu64) | (((crate::lift_ops::shr8((rdx as u8), ((rcx as u32) & 31))) as u8 as u64)); fl_l = ((((rdx as u8)) & (0x1u8))) as u8 as u64; fl_r = fl_l; rdx = (rdx & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); rax = (((rdx as u8)) as u32) as u32 as u64; bb = 25; continue; }
            21 => { rax = (crate::simd_ops::ru64(r14)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x24u64))) as u32) as u32 as u64; bb = 24; continue; }
            23 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 24; continue; }
            24 => { rcx = (((rbp as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((crate::lift_ops::shr8((rax as u8), ((rcx as u32) & 31))) as u8 as u64)); fl_l = ((((rax as u8)) & (0x1u8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); rax = (((rax as u8)) as u32) as u32 as u64; bb = 25; continue; }
            25 => { r11 = rsp.wrapping_add(0x60); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160ee30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f78; const P1: u64 = 0x1415f47f0; const P2: u64 = 0x14160eca0; const P3: u64 = 0x141524330; const P4: u64 = 0x14268f100;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x48u64)) as u64; r15 = rdx; r14 = rcx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 35; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(r15)) as u64; rcx = r15; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x70u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(r15)) as u64; rcx = r15; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x70u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xa; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 25; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbp); rsi = rsp.wrapping_add(0x20); rbp = (rbp & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rdi as u32)) as u32); rbx = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x24u64), ((rdi as u16)) as u16); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rdi as u32)) as u32); crate::simd_ops::wu16(rsp.wrapping_add(0x2cu64), ((rdi as u16)) as u16); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rdi as u32)) as u32); crate::simd_ops::wu16(rsp.wrapping_add(0x34u64), ((rdi as u16)) as u16); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rdi as u32)) as u32); crate::simd_ops::wu16(rsp.wrapping_add(0x3cu64), ((rdi as u16)) as u16); bb = 7; continue; }
            7 => { r9 = r15; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((rbx as u32)) as u32 as u64; rcx = r14; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; rcx = ((rbx as u32)) as u32 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u16)) & ((fl_r as u16))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); rax = (rax & !0xffu64) | (((crate::lift_ops::shl8((rax as u8), ((rcx as u32) & 31))) as u8 as u64)); rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; rbp = (rbp & !0xffu64) | ((((((rbp as u8)) | ((rax as u8)))) as u8 as u64)); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 10; continue; }
            10 => { rax = ((rax).wrapping_add(0x10u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 12; continue; }
            12 => { rax = ((rbx as u32)) as u32 as u64; rax = ((rax.wrapping_mul(8).wrapping_add(0x24u64)) as u64) as u64; fl_l = ((rax).wrapping_add(rcx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 18; continue; }
            13 => { fl_l = ((rax).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 11; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rax)) as u64; rcx = ((rbx as u32)) as u32 as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = ((rax).wrapping_add(0x24u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 18; continue; }
            15 => { rax = (crate::simd_ops::ru64(r14)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 16; continue; }
            16 => { fl_l = ((rcx).wrapping_add(0x4u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 20; continue; } bb = 17; continue; }
            17 => { rax = ((rbx as u32)) as u32 as u64; rax = ((rcx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; bb = 18; continue; }
            18 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(rax)) as u64; bb = 21; continue; }
            20 => { crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rdi as u32)) as u32); crate::simd_ops::wu16(rsp.wrapping_add(0x64u64), ((rdi as u16)) as u16); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; bb = 21; continue; }
            21 => { crate::simd_ops::wu64(rsi, rax); rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; rsi = ((rsi).wrapping_add(0x8u64)) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 7; continue; } bb = 22; continue; }
            22 => { rdx = 0x48; rcx = ((rdx.wrapping_sub(0x44u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { crate::simd_ops::wu8(rcx.wrapping_add(0x20u64), ((rbp as u8)) as u8); rax = P4; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(rcx, rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), r15); crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x24u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x2cu64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x34u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x3cu64), rax); bb = 34; continue; }
            24 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rcx = rdi; bb = 34; continue; }
            25 => { rax = (crate::simd_ops::ru64(r14)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 26; continue; }
            26 => { rsi = ((crate::simd_ops::ru8(rcx.wrapping_add(0x24u64))) as u32) as u32 as u64; bb = 28; continue; }
            27 => { rsi = (rsi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 28; continue; }
            28 => { rdi = 0; fl_l = 0; fl_r = 0; rbx = rcx.wrapping_add(0x4); fl_l = rcx; fl_r = rcx; rbx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rdi } else { rbx }) as u64; rdx = ((rdi.wrapping_add(0x48u64)) as u32) as u32 as u64; rcx = ((rdi.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 29; continue; }
            29 => { crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), r15); rax = P4; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), rdi); crate::simd_ops::wu8(rcx.wrapping_add(0x20u64), ((rsi as u8)) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x24u64), ((rdi as u32)) as u32); crate::simd_ops::wu16(rcx.wrapping_add(0x28u64), ((rdi as u16)) as u16); crate::simd_ops::wu32(rcx.wrapping_add(0x2cu64), ((rdi as u32)) as u32); crate::simd_ops::wu16(rcx.wrapping_add(0x30u64), ((rdi as u16)) as u16); crate::simd_ops::wu32(rcx.wrapping_add(0x34u64), ((rdi as u32)) as u32); crate::simd_ops::wu16(rcx.wrapping_add(0x38u64), ((rdi as u16)) as u16); crate::simd_ops::wu32(rcx.wrapping_add(0x3cu64), ((rdi as u32)) as u32); crate::simd_ops::wu16(rcx.wrapping_add(0x40u64), ((rdi as u16)) as u16); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 30; continue; }
            30 => { rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x24u64), rax); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x2cu64), rax); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x34u64), rax); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x3cu64), rax); bb = 34; continue; }
            31 => { bb = 32; continue; }
            32 => { crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rdi as u32)) as u32); crate::simd_ops::wu16(rsp.wrapping_add(0x64u64), ((rdi as u16)) as u16); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x24u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x2cu64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x34u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x3cu64), rax); bb = 34; continue; }
            33 => { rcx = rdi; bb = 34; continue; }
            34 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rax = rcx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x8u64), rcx); bb = 35; continue; }
            35 => { rsp = ((rsp).wrapping_add(0x48u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160f0e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14160ee30; const P1: u64 = 0x141524330; const P2: u64 = 0x14160e910; const P3: u64 = 0x141540670; const P4: u64 = 0x141604ac0; const P5: u64 = 0x1415242e0; const P6: u64 = 0x1416058a0; const P7: u64 = 0x14183acc8; const P8: u64 = 0x14160e5c0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r12 = r8; rbx = rdx; r14 = rcx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = r8; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P0, rax, rcx, rdx, r8, r9, rsp); }
            2 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbp); rbp = 0; fl_l = 0; fl_r = 0; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rdx = ((rbp.wrapping_add(0x10u64)) as u32) as u32 as u64; rcx = ((rbp.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rax, rbp); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbp); crate::simd_ops::wu64(r14.wrapping_add(0x10u64), rax); bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rsi); fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 6; continue; }
            6 => { bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = ((rax).wrapping_add(0x10u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 9; continue; }
            9 => { fl_l = rax; fl_r = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r15); rsi = rax.wrapping_add(0x8); rsi = (if (((fl_l as u64)) == ((fl_r as u64))) { rbp } else { rsi }) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { rsi = (crate::simd_ops::ru64(rsi)) as u64; bb = 46; continue; }
            11 => { crate::simd_ops::wu64(r14.wrapping_add(0x10u64), rbp); rax = 0; fl_l = 0; fl_r = 0; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            12 => { rax = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rdi); rdi = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rsi = ((crate::simd_ops::ru8(rdi.wrapping_add(0x24u64))) as u32) as u32 as u64; bb = 15; continue; }
            14 => { rsi = (rsi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 15; continue; }
            15 => { rdx = 0x48; rcx = ((rdx.wrapping_sub(0x44u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 44; continue; } bb = 16; continue; }
            16 => { fl_l = rdi; fl_r = rdi; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); r9 = rdi.wrapping_add(0x4); r8 = (((rsi as u8)) as u32) as u32 as u64; r9 = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbp } else { r9 }) as u64; rdx = r12; rcx = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 45; continue; } bb = 17; continue; }
            17 => { rdi = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(rdi)) as u64; r15 = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rdx = ((r15 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rcx = ((rcx).wrapping_add(0x10u64)) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 18; continue; } bb = 20; continue; }
            20 => { fl_l = rcx; fl_r = rdx; rax = rcx.wrapping_add(0x8); rax = (if (((fl_l as u64)) == ((fl_r as u64))) { rbp } else { rax }) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { crate::simd_ops::wu64(rax, rsi); bb = 27; continue; }
            22 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; fl_l = r15; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 24; continue; } bb = 23; continue; }
            23 => { rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 43; continue; } bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; fl_l = r15; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 43; continue; } bb = 25; continue; }
            25 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = ((rcx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; fl_l = ((rax).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 43; continue; } bb = 26; continue; }
            26 => { crate::simd_ops::wu64(rax, rbx); fl_l = ((rax).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rax, rsi); if (((fl_l as u64)) == 0) { bb = 43; continue; } bb = 27; continue; }
            27 => { fl_l = r12; fl_r = r12; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 45; continue; } bb = 28; continue; }
            28 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 34; continue; } bb = 29; continue; }
            29 => { rdx = ((rdi.wrapping_add(0x48u64)) as u32) as u32 as u64; rcx = ((rdi.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 35; continue; } bb = 30; continue; }
            30 => { rcx = rax; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 31; continue; }
            31 => { crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), rax); rcx = rax; r8 = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 34; continue; } bb = 32; continue; }
            32 => { r8 = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r8)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rdi; rcx = 0x4; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), rbp); bb = 35; continue; }
            33 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 35; continue; } bb = 34; continue; }
            34 => { rdx = r12; rcx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 45; continue; } bb = 35; continue; }
            35 => { rdi = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; r8 = (crate::simd_ops::ru64(rdi)) as u64; rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rcx = r8; rax = ((rdx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(r8)) as u64; fl_l = r8; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 43; continue; } bb = 36; continue; }
            36 => { bb = 37; continue; }
            37 => { fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 40; continue; } bb = 38; continue; }
            38 => { rcx = ((rcx).wrapping_add(0x10u64)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 37; continue; } bb = 39; continue; }
            39 => { bb = 43; continue; }
            40 => { rdx = (((rdx as u32)).wrapping_sub(1)) as u32 as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 42; continue; } bb = 41; continue; }
            41 => { rdx = ((rdx).wrapping_sub(rcx)) as u64; rdx = ((((rdx as u32)) & (0xfffffff0u32))) as u32 as u64; r8 = ((rdx as u32)) as u32 as u64; rdx = rcx.wrapping_add(0x10); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 42; continue; }
            42 => { fl_l = (crate::simd_ops::lk_sub_u32(rdi.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 43; continue; }
            43 => { rcx = rsi; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rdx = rsi; rcx = 0x4; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 44; continue; }
            44 => { rsi = rbp; bb = 45; continue; }
            45 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; bb = 46; continue; }
            46 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rax = rsi; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160f380(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x24; const P3: u64 = 0x4;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); r10 = ((rdx as u32)) as u32 as u64; rbx = 0; fl_l = 0; fl_r = 0; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r11 = rcx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdx)) as u64; r9 = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; r9 = ((r9).wrapping_shl(4u32)) as u64; r9 = ((r9).wrapping_add(rax)) as u64; fl_l = rax; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 4; continue; }
            4 => { rax = ((rax).wrapping_add(0x10u64)) as u64; fl_l = rax; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { rax = ((r10.wrapping_mul(8).wrapping_add(P2)) as u64) as u64; fl_l = ((rax).wrapping_add(rcx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; return rax; }
            7 => { fl_l = ((rax).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 5; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rax)) as u64; rax = ((rax.wrapping_add(r10.wrapping_mul(8))) as u64) as u64; fl_l = ((rax).wrapping_add(0x24u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; return rax; }
            9 => { rax = (crate::simd_ops::ru64(r11)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; fl_l = rcx; fl_r = rcx; rdx = ((rcx.wrapping_add(P3)) as u64) as u64; rdx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbx } else { rdx }) as u64; rax = ((rdx.wrapping_add(r10.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = rbx; bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160f420(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x28; const P2: u64 = 0x24; const P3: u64 = 0x29; const P4: u64 = 0x10;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r11 = ((r8 as u32)) as u32 as u64; r10 = rcx; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P2), ((r8 as u32)) as u32); rax = (crate::simd_ops::ru64(r10.wrapping_add(P0))) as u64; crate::simd_ops::wu8(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P3), ((r9 as u8)) as u8); bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(r10.wrapping_add(P4))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 5; continue; }
            5 => { r8 = ((rdx as u32)) as u32 as u64; bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(r8.wrapping_mul(8)).wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu32(rcx.wrapping_add(r8.wrapping_mul(8)).wrapping_add(P2), ((r11 as u32)) as u32); rcx = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; crate::simd_ops::wu8(rcx.wrapping_add(r8.wrapping_mul(8)).wrapping_add(P3), ((r9 as u8)) as u8); bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(r10.wrapping_add(P4))) as u64; rax = ((rax).wrapping_add(0x10u64)) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rcx))) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 9; continue; }
            9 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160f4a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415bd140; const P1: u64 = 0x14268f108; const P2: u64 = 0x34; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x28; const P6: u64 = 0x30; const P7: u64 = 0x36;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = ((rdx as u32)) as u32 as u64;
    rdi = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = P1;
    crate::simd_ops::wu16(rdi.wrapping_add(P2), ((rbx as u16)) as u16);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu64(rdi, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdi.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rdi.wrapping_add(P4), rax);
    crate::simd_ops::wu64(rdi.wrapping_add(P5), rax);
    crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rax as u32)) as u32);
    rax = rdi;
    crate::simd_ops::wu8(rdi.wrapping_add(P7), (0x4u8) as u8);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160f4f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x1415242e0; const P2: u64 = 0x20; const P3: u64 = 0x1415bd160; const P4: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = ((rdx as u32)) as u32 as u64;
    rbx = rcx;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = 0x1; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    crate::simd_ops::wu64(rbx.wrapping_add(P0), (0x0u64) as u64);
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = 0x1; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rcx = rbx;
    crate::simd_ops::wu64(rbx.wrapping_add(P2), (0x0u64) as u64);
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x38; rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
    rax = rbx;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160f560(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14164de30; const P1: u64 = 0x14164e400; const P2: u64 = 0x14164e210; const P3: u64 = 0x141610ae0; const P4: u64 = 0x14164ea60; const P5: u64 = 0x14164e760; const P6: u64 = 0x14164f610; const P7: u64 = 0x14164eed0; const P8: u64 = 0x14164f010; const P9: u64 = 0x14164f140; const P10: u64 = 0x14164fe50; const P11: u64 = 0x14164e610; const P12: u64 = 0x14164f700; const P13: u64 = 0x14164ff70; const P14: u64 = 0x1416503c0; const P15: u64 = 0x141650240; const P16: u64 = 0x1416505e0; const P17: u64 = 0x141650830;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rdx = ((rcx as u32)) as u32 as u64; rdx = ((((rdx as u32)) & (0xff00u32))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1200; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 29; continue; } bb = 1; continue; }
            1 => { if (((fl_l as u32)) == ((fl_r as u32))) { bb = 28; continue; } bb = 2; continue; }
            2 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xa00; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 18; continue; } bb = 3; continue; }
            3 => { if (((fl_l as u32)) == ((fl_r as u32))) { bb = 23; continue; } bb = 4; continue; }
            4 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x600; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 14; continue; } bb = 5; continue; }
            5 => { if (((fl_l as u32)) == ((fl_r as u32))) { bb = 17; continue; } bb = 6; continue; }
            6 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x100; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 7; continue; }
            7 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x200; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 8; continue; }
            8 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x300; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 9; continue; }
            9 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x400; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 54; continue; } bb = 10; continue; }
            10 => { return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            12 => { return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            13 => { return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            14 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x700; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 17; continue; } bb = 15; continue; }
            15 => { rax = ((rdx.wrapping_sub(0x800u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xfffffeff; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 54; continue; } bb = 16; continue; }
            16 => { rdx = 0x2; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            17 => { return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            18 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xe00; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 24; continue; } bb = 19; continue; }
            19 => { if (((fl_l as u32)) == ((fl_r as u32))) { bb = 27; continue; } bb = 20; continue; }
            20 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xb00; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 23; continue; } bb = 21; continue; }
            21 => { rax = ((rdx.wrapping_sub(0xc00u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xfffffeff; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 54; continue; } bb = 22; continue; }
            22 => { rdx = 0x5; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            23 => { rdx = 0; fl_l = 0; fl_r = 0; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            24 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xf00; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 27; continue; } bb = 25; continue; }
            25 => { rax = ((rdx.wrapping_sub(0x1000u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xfffffeff; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 54; continue; } bb = 26; continue; }
            26 => { return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            27 => { rdx = 0x3; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            28 => { return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            29 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1e00; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 44; continue; } bb = 30; continue; }
            30 => { if (((fl_l as u32)) == ((fl_r as u32))) { bb = 43; continue; } bb = 31; continue; }
            31 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1a00; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 37; continue; } bb = 32; continue; }
            32 => { if (((fl_l as u32)) == ((fl_r as u32))) { bb = 42; continue; } bb = 33; continue; }
            33 => { rax = ((rdx.wrapping_sub(0x1300u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xfffffeff; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 36; continue; } bb = 34; continue; }
            34 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1900; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 54; continue; } bb = 35; continue; }
            35 => { return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            36 => { return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            37 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1b00; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 42; continue; } bb = 38; continue; }
            38 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1c00; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 41; continue; } bb = 39; continue; }
            39 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1d00; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 54; continue; } bb = 40; continue; }
            40 => { return crate::simd_ops::call_fwd(P10, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            41 => { return crate::simd_ops::call_fwd(P11, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            42 => { return crate::simd_ops::call_fwd(P12, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            43 => { return crate::simd_ops::call_fwd(P13, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            44 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x2200; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 52; continue; } bb = 45; continue; }
            45 => { if (((fl_l as u32)) == ((fl_r as u32))) { bb = 51; continue; } bb = 46; continue; }
            46 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1f00; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 50; continue; } bb = 47; continue; }
            47 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x2000; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 56; continue; } bb = 48; continue; }
            48 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x2100; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 54; continue; } bb = 49; continue; }
            49 => { return crate::simd_ops::call_fwd(P14, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            50 => { return crate::simd_ops::call_fwd(P15, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            51 => { return crate::simd_ops::call_fwd(P16, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            52 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x3000; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 56; continue; } bb = 53; continue; }
            53 => { rax = ((rdx.wrapping_sub(0x3100u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xfffffeff; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 55; continue; } bb = 54; continue; }
            54 => { rax = 0; fl_l = 0; fl_r = 0; return rax; }
            55 => { return crate::simd_ops::call_fwd(P17, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            56 => { rdx = 0x4; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160f710(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f88; const P1: u64 = 0x1426452b0; const P2: u64 = 0x1426452a8; const P3: u64 = 0x142645560;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x36u64))) as u8 as u64; fl_r = (0x40u8) as u8 as u64; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(0x30u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(P0)) as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 23; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 12; continue; } bb = 3; continue; }
            3 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rsi = rbx.wrapping_add(0x8); bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32 as u64; r14 = 0; fl_l = 0; fl_r = 0; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 6; continue; }
            6 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbp as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 9; continue; }
            9 => { rdi = r14; bb = 10; continue; }
            10 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 15; continue; }
            12 => { rsi = rbx.wrapping_add(0x8); rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 5; continue; } bb = 13; continue; }
            13 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            14 => { rdi = r14; bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 22; continue; } bb = 16; continue; }
            16 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 19; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 16; continue; } bb = 18; continue; }
            18 => { bb = 45; continue; }
            19 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 46; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 46; continue; } bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 46; continue; }
            22 => { rsi = rbx.wrapping_add(0x8); bb = 45; continue; }
            23 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x58u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 34; continue; } bb = 24; continue; }
            24 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x58u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 23; continue; } bb = 25; continue; }
            25 => { rsi = rbx.wrapping_add(0x60); bb = 26; continue; }
            26 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0xa0u64))) as u32 as u64; r14 = 0; fl_l = 0; fl_r = 0; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 36; continue; } bb = 27; continue; }
            27 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbp as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x98u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 28; continue; }
            28 => { bb = 29; continue; }
            29 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 32; continue; } bb = 30; continue; }
            30 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 29; continue; } bb = 31; continue; }
            31 => { rdi = r14; bb = 32; continue; }
            32 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 37; continue; } bb = 33; continue; }
            33 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 37; continue; }
            34 => { rsi = rbx.wrapping_add(0x60); rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x58u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 26; continue; } bb = 35; continue; }
            35 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 23; continue; }
            36 => { rdi = r14; bb = 37; continue; }
            37 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x58u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 44; continue; } bb = 38; continue; }
            38 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x58u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 41; continue; } bb = 39; continue; }
            39 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x58u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 38; continue; } bb = 40; continue; }
            40 => { bb = 45; continue; }
            41 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 46; continue; } bb = 42; continue; }
            42 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x88u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 46; continue; } bb = 43; continue; }
            43 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x90u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 46; continue; }
            44 => { rsi = rbx.wrapping_add(0x60); bb = 45; continue; }
            45 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 46; continue; }
            46 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = rdi.wrapping_sub(0x10); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = rdi; fl_r = rdi; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r14 } else { rax }) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160f8e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x143814fac; const P2: u64 = 0x1; const P3: u64 = 0x4; const P4: u64 = 0x1415ac1a0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru32(P1)) as u32 as u64; r11 = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); rdi = 0xfffffffc; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { r10 = ((crate::simd_ops::ru8(r9)) as u32) as u32 as u64; r8 = ((rdx as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rcx = ((r8.wrapping_add(P2)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(r9))) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 3; continue; }
            3 => { r8 = ((rcx as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 4; continue; }
            4 => { r8 = rdx; bb = 5; continue; }
            5 => { r9 = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rax = rsp.wrapping_add(0x30); fl_l = r8; fl_r = r8; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { r8 } else { rax }) as u64; rbx = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { rax = ((crate::simd_ops::ru8(r9)) as u32) as u32 as u64; bb = 7; continue; }
            7 => { r8 = ((rdx.wrapping_add(P2)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(r8.wrapping_add(r9))) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } bb = 8; continue; }
            8 => { rdx = ((r8 as u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 7; continue; } bb = 9; continue; }
            9 => { rax = ((rbx as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            10 => { rax = ((r10.wrapping_add(P3)) as u32) as u32 as u64; rcx = ((r8 as u32)) as u32 as u64; rax = (((rax) & (rdi))) as u64; fl_l = ((rax).wrapping_add(r9)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; r8 = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; bb = 5; continue; }
            11 => { rax = (((rax as u32)).wrapping_add(0x4u32)) as u32 as u64; rcx = ((rdx as u32)) as u32 as u64; rax = (((rax) & (rdi))) as u64; rax = ((rax).wrapping_add(r9)) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 12; continue; }
            12 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = (((rax as u32)).wrapping_add((rbx as u32))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160f9a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x143814fb0; const P2: u64 = 0x1; const P3: u64 = 0x4; const P4: u64 = 0x1415ac1a0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru32(P1)) as u32 as u64; r11 = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); rdi = 0xfffffffc; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { r10 = ((crate::simd_ops::ru8(r9)) as u32) as u32 as u64; r8 = ((rdx as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rcx = ((r8.wrapping_add(P2)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(r9))) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 3; continue; }
            3 => { r8 = ((rcx as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 4; continue; }
            4 => { r8 = rdx; bb = 5; continue; }
            5 => { r9 = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rax = rsp.wrapping_add(0x30); fl_l = r8; fl_r = r8; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { r8 } else { rax }) as u64; rbx = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { rax = ((crate::simd_ops::ru8(r9)) as u32) as u32 as u64; bb = 7; continue; }
            7 => { r8 = ((rdx.wrapping_add(P2)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(r8.wrapping_add(r9))) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } bb = 8; continue; }
            8 => { rdx = ((r8 as u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 7; continue; } bb = 9; continue; }
            9 => { rax = ((rbx as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            10 => { rax = ((r10.wrapping_add(P3)) as u32) as u32 as u64; rcx = ((r8 as u32)) as u32 as u64; rax = (((rax) & (rdi))) as u64; fl_l = ((rax).wrapping_add(r9)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; r8 = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; bb = 5; continue; }
            11 => { rax = (((rax as u32)).wrapping_add(0x4u32)) as u32 as u64; rcx = ((rdx as u32)) as u32 as u64; rax = (((rax) & (rdi))) as u64; rax = ((rax).wrapping_add(r9)) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 12; continue; }
            12 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = (((rax as u32)).wrapping_add((rbx as u32))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160fa60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x36;
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    fl_l = (((rax as u8)).wrapping_shr(7u32)) as u8 as u64;
    rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160fa70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415bd1d0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rdx = 0x4;
    return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160fa80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524330; const P1: u64 = 0x14183acbc; const P2: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = r8; rdi = 0; fl_l = 0; fl_r = 0; r8 = (crate::simd_ops::ru64(rcx)) as u64; rbp = (((rdx as u8)) as u32) as u32 as u64; r12 = rcx; r13 = 0xfffffffc; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rax = ((rdi as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rdx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 4; continue; }
            4 => { bb = 7; continue; }
            5 => { rcx = ((rax as u32)) as u32 as u64; rdx = ((r9.wrapping_add(0x4u64)) as u32) as u32 as u64; rdx = (((rdx) & (r13))) as u64; rax = ((r8.wrapping_add(rdx)) as u64) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 6; continue; }
            6 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdi = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; bb = 8; continue; }
            8 => { r15 = ((rdi.wrapping_add(0x5u64)) as u32) as u32 as u64; rcx = 0x1; rdx = ((r15 as u32)) as u32 as u64; rdx = ((((rdx as u32)) & ((r13 as u32)))) as u32 as u64; rdx = (((rdx as u32)).wrapping_add(0x8u32)) as u32 as u64; rdx = ((rdx.wrapping_add(rdi.wrapping_mul(8))) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(r12)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); r14 = ((rdi as u32)) as u32 as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = rax.wrapping_add(0x1); r8 = ((r14 as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = ((rdi.wrapping_add(0x4u64)) as u32) as u32 as u64; rcx = ((r15 as u32)) as u32 as u64; rdx = (((rdx) & (r13))) as u64; r8 = ((rdi.wrapping_mul(8)) as u64) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(r12))) as u64; rcx = (((rcx) & (r13))) as u64; rcx = ((rcx).wrapping_add(rsi)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(r12)) as u64; rcx = 0x1; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { r15 = (((r15) & (r13))) as u64; rax = ((rdi.wrapping_add(0x1u64)) as u32) as u32 as u64; rdi = (rdi & !0xffu64) | (((((rdi as u8)).wrapping_add(1)) as u8 as u64)); crate::simd_ops::wu8(rax.wrapping_add(rsi), ((rbp as u8)) as u8); crate::simd_ops::wu8(rsi, ((rdi as u8)) as u8); crate::simd_ops::wu64(r12, rsi); rcx = ((rsi.wrapping_add(r15)) as u64) as u64; rdx = ((rcx.wrapping_add(r14.wrapping_mul(8))) as u64) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rdx, rbx); rax = 0x1; bb = 14; continue; }
            13 => { rax = 0x2; bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160fbb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524330; const P1: u64 = 0x14183acbc; const P2: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = ((r8 as u32)) as u32 as u64; rdi = 0; fl_l = 0; fl_r = 0; r8 = (crate::simd_ops::ru64(rcx)) as u64; rbp = (((rdx as u8)) as u32) as u32 as u64; r12 = rcx; r13 = 0xfffffffc; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rax = ((rdi as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rdx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 4; continue; }
            4 => { bb = 7; continue; }
            5 => { rcx = ((rax as u32)) as u32 as u64; rdx = ((r9.wrapping_add(0x4u64)) as u32) as u32 as u64; rdx = (((rdx) & (r13))) as u64; rax = ((r8.wrapping_add(rdx)) as u64) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 6; continue; }
            6 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdi = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; bb = 8; continue; }
            8 => { r15 = ((rdi.wrapping_add(0x5u64)) as u32) as u32 as u64; rcx = 0x1; rdx = ((r15 as u32)) as u32 as u64; rdx = ((((rdx as u32)) & ((r13 as u32)))) as u32 as u64; rdx = (((rdx as u32)).wrapping_add(0x4u32)) as u32 as u64; rdx = ((rdx.wrapping_add(rdi.wrapping_mul(4))) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(r12)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); r14 = ((rdi as u32)) as u32 as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = rax.wrapping_add(0x1); r8 = ((r14 as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = ((rdi.wrapping_add(0x4u64)) as u32) as u32 as u64; rcx = ((r15 as u32)) as u32 as u64; rdx = (((rdx) & (r13))) as u64; r8 = ((rdi.wrapping_mul(4)) as u64) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(r12))) as u64; rcx = (((rcx) & (r13))) as u64; rcx = ((rcx).wrapping_add(rsi)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(r12)) as u64; rcx = 0x1; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { r15 = (((r15) & (r13))) as u64; rax = ((rdi.wrapping_add(0x1u64)) as u32) as u32 as u64; rdi = (rdi & !0xffu64) | (((((rdi as u8)).wrapping_add(1)) as u8 as u64)); crate::simd_ops::wu8(rax.wrapping_add(rsi), ((rbp as u8)) as u8); crate::simd_ops::wu8(rsi, ((rdi as u8)) as u8); crate::simd_ops::wu64(r12, rsi); rcx = ((rsi.wrapping_add(r15)) as u64) as u64; rdx = ((rcx.wrapping_add(r14.wrapping_mul(4))) as u64) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu32(rdx, ((rbx as u32)) as u32); rax = 0x1; bb = 14; continue; }
            13 => { rax = 0x2; bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160fce0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143814f70; const P1: u64 = 0x14160fbb0; const P2: u64 = 0x14160fa80;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbp = rcx; rdi = ((((rdx as u32)) as i32 as i64) as u64) as u64; rcx = P0; rbx = 0; fl_l = 0; fl_r = 0; r15 = ((r9 as u32)) as u32 as u64; rsi = ((r8 as u32)) as u32 as u64; r10 = 0x1; r12 = 0xfffffffc; r11 = (crate::simd_ops::ru64(rbp.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(rdi.wrapping_mul(4)))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); fl_l = r11; fl_r = r11; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { r8 = ((crate::simd_ops::ru8(r11)) as u32) as u32 as u64; rdx = ((rbx as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rcx = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(r11))) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 14; continue; } bb = 3; continue; }
            3 => { rdx = ((rcx as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rdx = rbx; bb = 5; continue; }
            5 => { fl_l = rdx; fl_r = rdx; rax = rsp.wrapping_add(0x20); rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rdx } else { rax }) as u64; fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { r8 = ((rsi as u32)) as u32 as u64; rcx = rbp.wrapping_add(0x20); rdx = (((rdi as u8)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r10 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 17; continue; } bb = 7; continue; }
            7 => { r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x80u64))) as u32 as u64; fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 16; continue; } bb = 8; continue; }
            8 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 16; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 10; continue; }
            10 => { r8 = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; bb = 11; continue; }
            11 => { rcx = ((rbx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rdx))) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 15; continue; } bb = 12; continue; }
            12 => { rbx = ((rcx as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 11; continue; } bb = 13; continue; }
            13 => { rax = ((r10 as u32)) as u32 as u64; bb = 17; continue; }
            14 => { rax = ((r8.wrapping_add(0x4u64)) as u32) as u32 as u64; rcx = ((rdx as u32)) as u32 as u64; rax = (((rax) & (r12))) as u64; fl_l = ((rax).wrapping_add(r11)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; bb = 5; continue; }
            15 => { rax = ((r8.wrapping_add(0x4u64)) as u32) as u32 as u64; rcx = ((rbx as u32)) as u32 as u64; rax = (((rax) & (r12))) as u64; rax = ((rax).wrapping_add(rdx)) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu32(rsp.wrapping_add(0x64u64), ((r9 as u32)) as u32); rcx = rbp.wrapping_add(0x28); crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((r15 as u32)) as u32); rdx = (((rdi as u8)) as u32) as u32 as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            17 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160fe00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143814f70; const P1: u64 = 0x14160fbb0; const P2: u64 = 0x14160fa80;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rsi = rcx; rdi = ((((rdx as u32)) as i32 as i64) as u64) as u64; rcx = P0; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); rbx = 0; fl_l = 0; fl_r = 0; xmm6 = (xmm3) as u128; r8 = 0x1; rbp = 0xfffffffc; r9 = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(rdi.wrapping_mul(4)))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), ((rax as u32)) as u32); fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { r10 = ((crate::simd_ops::ru8(r9)) as u32) as u32 as u64; rdx = ((rbx as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rcx = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(r9))) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 14; continue; } bb = 3; continue; }
            3 => { rdx = ((rcx as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rdx = rbx; bb = 5; continue; }
            5 => { fl_l = rdx; fl_r = rdx; rax = rsp.wrapping_add(0x58); rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rdx } else { rax }) as u64; xmm0 = ((crate::simd_ops::ru32(rax)) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), ((xmm2 as u32)) as u32); rcx = rsi.wrapping_add(0x20); r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x58u64))) as u32 as u64; rdx = (((rdi as u8)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 17; continue; } bb = 7; continue; }
            7 => { xmm1 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x70u64))) as u32) as u128; xmm0 = 0u128; fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 16; continue; } bb = 8; continue; }
            8 => { fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 16; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 10; continue; }
            10 => { r9 = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; bb = 11; continue; }
            11 => { rcx = ((rbx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rdx))) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 15; continue; } bb = 12; continue; }
            12 => { rbx = ((rcx as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 11; continue; } bb = 13; continue; }
            13 => { rax = ((r8 as u32)) as u32 as u64; bb = 17; continue; }
            14 => { rax = ((r10.wrapping_add(0x4u64)) as u32) as u32 as u64; rcx = ((rdx as u32)) as u32 as u64; rax = (((rax) & (rbp))) as u64; fl_l = ((rax).wrapping_add(r9)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; bb = 5; continue; }
            15 => { rax = ((r9.wrapping_add(0x4u64)) as u32) as u32 as u64; rcx = ((rbx as u32)) as u32 as u64; rax = (((rax) & (rbp))) as u64; rax = ((rax).wrapping_add(rdx)) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((xmm6 as u32)) as u32); rcx = rsi.wrapping_add(0x28); crate::simd_ops::wu32(rsp.wrapping_add(0x54u64), ((xmm1 as u32)) as u32); rdx = (((rdi as u8)) as u32) as u32 as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 17; continue; }
            17 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160ff20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x36;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rdx as u32)) as u32);
    rdx = ((rdx).wrapping_shr(32u32)) as u64;
    rdx = (rdx & !0xffu64) | (((((rdx as u8)).wrapping_shl(6u32)) as u8 as u64));
    rdx = (rdx & !0xffu64) | ((((((rdx as u8)) ^ (crate::simd_ops::ru8(rcx.wrapping_add(P1))))) as u8 as u64));
    rdx = (rdx & !0xffu64) | ((((((rdx as u8)) & (0x40u8))) as u8 as u64));
    fl_l = (crate::simd_ops::lk_xor_u8(rcx.wrapping_add(P1), ((rdx as u8)) as u8)) as u8 as u64; fl_r = fl_l;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160ff40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415242e0; const P1: u64 = 0x141524340; const P2: u64 = 0x14183acbc; const P3: u64 = 0x1438152ac;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu32(rsp.wrapping_add(0x18u64), ((r8 as u32)) as u32); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdx = ((rdx).wrapping_add(0x4u64)) as u64; rsi = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rdx); rax = ((crate::simd_ops::ru16(rdx)) as u32) as u32 as u64; rdx = ((rdx).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rdx); fl_l = (crate::simd_ops::ru16(rcx.wrapping_add(0x34u64))) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x1; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; rdx = ((rdx).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rdx); crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rbx); rcx = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rax = rdx.wrapping_add(0x1); fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = rsi; crate::simd_ops::wu8(rsp.wrapping_add(0x54u64), (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { 1u8 } else { 0u8 }) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbp); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = 0x1; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsi.wrapping_add(0x20u64), rdi); rdx = ((rdi as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = rax; rcx = ((rdi.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = 0xfffffffc; crate::simd_ops::wu64(rsi.wrapping_add(0x28u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); rbx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); fl_l = ((rbx as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { r14 = ((rbx.wrapping_add(0x4u64)) as u32) as u32 as u64; rcx = 0x1; r14 = ((((r14 as u32)) & ((rbp as u32)))) as u32 as u64; r8 = ((r14.wrapping_add(rbx.wrapping_mul(4))) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r15 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { r10 = ((rax.wrapping_add(0x34u64)) as u32) as u32 as u64; bb = 19; continue; }
            9 => { crate::simd_ops::wu8(rax, ((rbx as u8)) as u8); rcx = rax.wrapping_add(0x1); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; r8 = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rdx = ((rdx).wrapping_add(rbx)) as u64; rcx = ((r14 as u32)) as u32 as u64; rbx = ((rbx.wrapping_mul(4)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rdx); r8 = rbx; rcx = ((rcx).wrapping_add(r15)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; fl_l = ((rax).wrapping_add(rbx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x20u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); bb = 12; continue; }
            10 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = 0x1; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x20u64), rdi); bb = 12; continue; }
            12 => { rbx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rax = ((rax).wrapping_add(1)) as u64; rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); fl_l = ((rbx as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 16; continue; } bb = 13; continue; }
            13 => { r14 = ((rbx.wrapping_add(0x4u64)) as u32) as u32 as u64; rcx = 0x1; r14 = ((((r14 as u32)) & ((rbp as u32)))) as u32 as u64; r8 = ((r14.wrapping_add(rbx.wrapping_mul(8))) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r15 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { r10 = ((rax.wrapping_add(0x34u64)) as u32) as u32 as u64; bb = 19; continue; }
            15 => { crate::simd_ops::wu8(rax, ((rbx as u8)) as u8); rcx = rax.wrapping_add(0x1); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; r8 = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rdx = ((rdx).wrapping_add(rbx)) as u64; rcx = ((r14 as u32)) as u32 as u64; rbx = ((rbx.wrapping_mul(8)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rdx); r8 = rbx; rcx = ((rcx).wrapping_add(r15)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_add_u64(rsp.wrapping_add(0x58u64), (rbx) as u64)) as u64; fl_r = fl_l; crate::simd_ops::wu64(rsi.wrapping_add(0x28u64), r15); bb = 18; continue; }
            16 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { rcx = 0x1; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi.wrapping_add(0x28u64), rdi); bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(rsi)) as u64; r8 = rsp.wrapping_add(0x60); rdx = rsp.wrapping_add(0x58); rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r10 = ((rax as u32)) as u32 as u64; bb = 19; continue; }
            19 => { r8 = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; r11 = (0x20c49ba5e353f7cfu64) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 20; continue; }
            20 => { r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((rdi as u32)) as u32 as u64; bb = 21; continue; }
            21 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 24; continue; } bb = 22; continue; }
            22 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 21; continue; } bb = 23; continue; }
            23 => { bb = 26; continue; }
            24 => { rax = ((r9.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = (((rax) & (rbp))) as u64; rax = ((rax).wrapping_add(r8)) as u64; r8 = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru32(P3)) as u32 as u64; rcx = (((crate::simd_ops::ru32(r8)) as i32 as i64) as u64) as u64; rcx = (((rcx) as i64).wrapping_mul((rax) as i64) as u64) as u64; rax = r11; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(7u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; fl_l = ((rdx).wrapping_add(rax)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; crate::simd_ops::wu32(r8, ((rdx as u32)) as u32); bb = 26; continue; }
            26 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 27; continue; }
            27 => { r8 = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; bb = 28; continue; }
            28 => { rcx = ((rdi.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rdx))) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 31; continue; } bb = 29; continue; }
            29 => { rdi = ((rcx as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 28; continue; } bb = 30; continue; }
            30 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = ((r10 as u32)) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            31 => { rax = ((r8.wrapping_add(0x4u64)) as u32) as u32 as u64; rcx = ((rdi as u32)) as u32 as u64; rax = (((rax) & (rbp))) as u64; rax = ((rax).wrapping_add(rdx)) as u64; r8 = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 32; continue; }
            32 => { rax = (crate::simd_ops::ru32(P3)) as u32 as u64; rcx = (((crate::simd_ops::ru32(r8)) as i32 as i64) as u64) as u64; rcx = (((rcx) as i64).wrapping_mul((rax) as i64) as u64) as u64; rax = r11; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rax = r11; rdx = ((((rdx) as i64).wrapping_shr(7u32) as u64)) as u64; rcx = rdx; rcx = ((rcx).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; crate::simd_ops::wu32(r8, ((rdx as u32)) as u32); rdx = (((crate::simd_ops::ru32(r8.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru32(P3)) as u32 as u64; rdx = (((rdx) as i64).wrapping_mul((rcx) as i64) as u64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rdx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(7u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; fl_l = ((rdx).wrapping_add(rax)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; crate::simd_ops::wu32(r8.wrapping_add(0x4u64), ((rdx as u32)) as u32); bb = 33; continue; }
            33 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = ((r10 as u32)) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141610260(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x36;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7fu8))) as u8 as u64));
    rdx = (rdx & !0xffu64) | (((((rdx as u8)).wrapping_shl(7u32)) as u8 as u64));
    fl_l = ((((rax as u8)) | ((rdx as u8)))) as u8 as u64; fl_r = fl_l;
    rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64));
    crate::simd_ops::wu8(rcx.wrapping_add(P0), ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141610290(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14268f158; const P1: u64 = 0x142677588; const P2: u64 = 0x8; const P3: u64 = 0x14268f3c0; const P4: u64 = 0x10; const P5: u64 = 0x14268f3e8; const P6: u64 = 0x28; const P7: u64 = 0x1415c6110; const P8: u64 = 0x1415ac250; const P9: u64 = 0x141834d30;
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
    rax = P3;
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rax);
    rax = P5;
    crate::simd_ops::wu64(rcx.wrapping_add(P6), rax);
    rcx = ((rcx).wrapping_add(0xf0u64)) as u64;
    rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp);
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x110; rcx = rdi; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141610300(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524330; const P1: u64 = 0x1415ac200; const P2: u64 = 0x14268f158; const P3: u64 = 0xf0; const P4: u64 = 0x142677588; const P5: u64 = 0x8; const P6: u64 = 0x14268f3c0; const P7: u64 = 0x10; const P8: u64 = 0x14268f3e8; const P9: u64 = 0x28; const P10: u64 = 0x1415c60e0; const P11: u64 = 0x70; const P12: u64 = 0x14020bf70;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = 0x110; rcx = 0x2; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = P2; crate::simd_ops::wu64(rbx, rax); rcx = ((rbx.wrapping_add(P3)) as u64) as u64; rax = P4; crate::simd_ops::wu64(rbx.wrapping_add(P5), rax); rax = P6; crate::simd_ops::wu64(rbx.wrapping_add(P7), rax); rax = P8; crate::simd_ops::wu64(rbx.wrapping_add(P9), rax); rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(P11))) as u64; rax = P12; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rcx = rbx; rax = crate::simd_ops::call_stk(rdx, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xa; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (rax & !0xffu64) | (((0x80u8) as u8 as u64)); bb = 6; continue; }
            5 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { rcx = ((crate::simd_ops::ru8(rbx.wrapping_add(0xd4u64))) as u32) as u32 as u64; rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0x7fu8))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) | ((rax as u8)))) as u8 as u64)); rax = rbx; crate::simd_ops::wu8(rbx.wrapping_add(0xd4u64), ((rcx as u8)) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416103c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1f; const P1: u64 = 0x1d; const P2: u64 = 0x1415bdd20; const P3: u64 = 0x141610420;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rbx = rdx; rcx = ((rcx).wrapping_add(0x28u64)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x1; r8 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 4; continue; }
            4 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rdx = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rdx = rbx; rcx = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
            8 => { rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 6; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141610420(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415ac910; const P1: u64 = 0x1434268d0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x70u64))) as u64; rdi = rdx; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru16(rbx.wrapping_add(0xd0u64))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if ((((fl_l as u16)) as i16) <= (((fl_r as u16)) as i16)) { bb = 24; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            4 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x3cu64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 15; continue; } bb = 5; continue; }
            5 => { xmm0 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x38u64))) as u32) as u128; xmm6 = 0u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { xmm6 = (xmm0) as u128; xmm6 = (xmm6 & !0xffff_ffffu128) | ((if f32::from_bits(xmm6 as u32) < f32::from_bits(crate::simd_ops::ru32(P1)) { f32::from_bits(xmm6 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P1)) }).to_bits() as u128); bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = rcx; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 10; continue; }
            10 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x258u64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { r8 = ((crate::simd_ops::ru8(rdi.wrapping_add(0x3cu64))) as u32) as u32 as u64; rcx = rbx.wrapping_sub(0x18); rax = (crate::simd_ops::ru64(rbx.wrapping_sub(0x18u64))) as u64; xmm1 = (xmm6) as u128; r8 = (r8 & !0xffu64) | (((((r8 as u8)).wrapping_shr(1u32)) as u8 as u64)); r8 = (r8 & !0xffu64) | ((((((r8 as u8)) & (0x1u8))) as u8 as u64)); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x70u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            15 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x38u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); rsi = 0; fl_l = 0; fl_r = 0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rsi = (if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { (rax as u32) } else { (rsi as u32) }) as u32 as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 16; continue; }
            16 => { bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { fl_l = rcx; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 22; continue; } bb = 19; continue; }
            19 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x258u64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 22; continue; } bb = 21; continue; }
            21 => { r8 = ((crate::simd_ops::ru8(rdi.wrapping_add(0x3cu64))) as u32) as u32 as u64; rcx = rbx.wrapping_sub(0x18); rax = (crate::simd_ops::ru64(rbx.wrapping_sub(0x18u64))) as u64; rdx = ((rsi as u32)) as u32 as u64; r8 = (r8 & !0xffu64) | (((((r8 as u8)).wrapping_shr(1u32)) as u8 as u64)); r8 = (r8 & !0xffu64) | ((((((r8 as u8)) & (0x1u8))) as u8 as u64)); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 22; continue; }
            22 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x70u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 17; continue; } bb = 23; continue; }
            23 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 24; continue; }
            24 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141610560(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1f; const P1: u64 = 0x1d; const P2: u64 = 0x1415bdd20; const P3: u64 = 0x141610420;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = 0x0;
    rbx = rdx;
    rdi = rcx;
    if (((fl_l as u32)) != ((fl_r as u32))) { rdx = rbx; rcx = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rcx = ((rcx).wrapping_add(0x28u64)) as u64;
    fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if (((fl_l as u8)) == ((fl_r as u8))) { rdx = rbx; rcx = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rdx = rbx;
    rcx = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416105b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xfc; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x14153e940; const P4: u64 = 0x1; const P5: u64 = 0xf0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rbx = r8; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7cu8))) as u8 as u64)); rdi = rdx; rsi = rcx; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 12; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x50u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rdx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 4; continue; }
            4 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 12; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; rax = ((rcx.wrapping_add(P4)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P1), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdi)) as u64; bb = 10; continue; }
            6 => { rdi = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r8.wrapping_add(P2))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 12; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; rax = ((rcx.wrapping_add(P4)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P1), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rbx)) as u64; bb = 10; continue; }
            10 => { rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = ((rsi.wrapping_add(P5)) as u64) as u64; crate::simd_ops::wu64(rdx, rax); bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141610650(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415c0040; const P1: u64 = 0x18; const P2: u64 = 0x1415a06c0; const P3: u64 = 0x20; const P4: u64 = 0x143426ea4; const P5: u64 = 0x4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rsi = r9; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rdi = rdx; rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x80u64))) as u32) as u32 as u64; rbp = rcx; crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(P1))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; rbx = ((rax as u32)) as u32 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rdx = rsp.wrapping_add(0x68); crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), (0x0u32) as u32); rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rax = ((rbx as u32)) as u32 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = ((crate::simd_ops::ru8(rsi.wrapping_add(P3))) as u32) as u32 as u64; xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x68u64))) as u32) as u128; xmm0 = crate::simd_ops::vcvtdq2ps(xmm0); xmm1 = (((rcx as u32)) as u32) as u128; xmm1 = crate::simd_ops::vcvtdq2ps(xmm1); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(P4))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(crate::simd_ops::ru32(rdi.wrapping_add(P5)))).to_bits() as u128); crate::simd_ops::wu32(rdi.wrapping_add(P5), ((xmm1 as u32)) as u32); bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141610710(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415c0b00; const P1: u64 = 0x141524330; const P2: u64 = 0x1415945c0; const P3: u64 = 0x143930fa0; const P4: u64 = 0x1415c4f90; const P5: u64 = 0x143426abc; const P6: u64 = 0x14268f46c; const P7: u64 = 0x14268f468; const P8: u64 = 0x14183ad46; const P9: u64 = 0x14268f470; const P10: u64 = 0x1415c0b50;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x98u64))) as u32) as u32 as u64; rdi = rdx; rsi = rcx; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 22; continue; } bb = 1; continue; }
            1 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x90u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 22; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x9du64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) <= ((fl_r as u8))) { bb = 22; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbp); crate::simd_ops::wu64(r11.wrapping_add(0x18u64), r14); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = ((crate::simd_ops::ru8(rdi.wrapping_add(0x98u64))) as u32) as u32 as u64; rbp = ((rax as u32)) as u32 as u64; r8 = ((crate::simd_ops::ru8(rdi.wrapping_add(0x99u64))) as u32) as u32 as u64; rcx = ((rdx.wrapping_add(0x80u64)) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xcfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0xb0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = ((crate::simd_ops::ru8(rdi.wrapping_add(0x9cu64))) as u32) as u32 as u64; bb = 7; continue; }
            6 => { rcx = (rcx & !0xffu64) | (((0xffu8) as u8 as u64)); bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0xa8u64))) as u64; r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rsp.wrapping_add(0x48u64), ((rcx as u8)) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); crate::simd_ops::wu8(rsp.wrapping_add(0x49u64), ((r8 as u8)) as u8); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { rdx = ((rcx.wrapping_add(0x20u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 9; continue; }
            9 => { fl_l = ((rax).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rax.wrapping_add(0x8u64), r14); crate::simd_ops::wu16(rax.wrapping_add(0x10u64), (0xffffu16) as u16); bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu64(rbx.wrapping_add(0x30u64), rcx); bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), rcx); bb = 15; continue; }
            14 => { crate::simd_ops::wu64(rax, rcx); bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), rcx); rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rcx, r14); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x44u64), (1) as u32); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; fl_l = ((rax).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu32(rax, ((rbp as u32)) as u32); crate::simd_ops::wu128(rax.wrapping_add(0x8u64), (xmm0) as u128); fl_l = (crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x24u64), (1) as u32)) as u32 as u64; bb = 17; continue; }
            17 => { rcx = ((crate::simd_ops::ru8(rdi.wrapping_add(0x98u64))) as u32) as u32 as u64; rbp = ((crate::simd_ops::ru8(rdi.wrapping_add(0x99u64))) as u32) as u32 as u64; rax = ((rcx.wrapping_add(0x80u64)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xcfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 20; continue; } bb = 18; continue; }
            18 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xb0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rbx = ((crate::simd_ops::ru8(rdi.wrapping_add(0x9cu64))) as u32) as u32 as u64; bb = 21; continue; }
            20 => { rbx = (rbx & !0xffu64) | (((0xffu8) as u8 as u64)); bb = 21; continue; }
            21 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; r8 = rsp.wrapping_add(0x50); crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x9cu64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x70u64), ((rbx as u8)) as u8); rbx = 0x1; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rcx); rcx = (crate::simd_ops::ru64(P3)) as u64; xmm3 = (((rax as u32)) as u32) as u128; crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), ((r14 as u8)) as u8); xmm3 = crate::simd_ops::vcvtdq2ps(xmm3); crate::simd_ops::wu32(rsp.wrapping_add(0xa8u64), (0x85u32) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0xacu64), (0x1u8) as u8); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rbx as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), r14); crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), ((r14 as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x68u64), ((rbp as u8)) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x9cu64))) as u32) as u32 as u64; xmm0 = ((crate::simd_ops::ru32(P5)) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0xa8u64), (0x84u32) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0xacu64), ((rbx as u8)) as u8); xmm1 = (((rax as u32)) as u32) as u128; xmm1 = crate::simd_ops::vcvtdq2ps(xmm1); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(crate::simd_ops::ru32(P6))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(P7))).to_bits() as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P9))).to_bits() as u128); r8 = rsp.wrapping_add(0x50); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; rcx = (crate::simd_ops::ru64(P3)) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), ((r14 as u8)) as u8); xmm3 = (xmm0) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rbx as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rcx = 0x52; rbx = (if (((fl_l as u32)) != ((fl_r as u32))) { (rcx as u32) } else { (rbx as u32) }) as u32 as u64; rax = ((rbx as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x80u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            22 => { rax = 0x1; rsp = ((rsp).wrapping_add(0x80u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141610970(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141610710; const P1: u64 = 0xf0; const P2: u64 = 0x141598a00;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rdx;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rdx = ((rbx.wrapping_add(P1)) as u64) as u64; r8 = rdi; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416109c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70; const P1: u64 = 0x20; const P2: u64 = 0x143930f00; const P3: u64 = 0x1401de410; const P4: u64 = 0x1415b8430; const P5: u64 = 0xf0; const P6: u64 = 0x1415c6420; const P7: u64 = 0x1415c6220; const P8: u64 = 0x1415a2910;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu32(rsp.wrapping_add(0x18u64), ((r8 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdi = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbx); rbx = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(P2)) as u64; r8 = 0x3; r9 = ((rax as u32)) as u32 as u64; rdx = ((rbx as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = 0x5b; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { crate::simd_ops::lk_add_u64(rsp.wrapping_add(0x78u64), (0x4u64) as u64); r8 = rsp.wrapping_add(0x30); rdx = rsp.wrapping_add(0x80); rcx = rsp.wrapping_add(0x78); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; fl_r = (0x0u64) as u64; rcx = ((rdi.wrapping_add(P5)) as u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x38u64))) as u128; rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; r8 = rsp.wrapping_add(0x20); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            5 => { rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x38u64))) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { r9 = ((crate::simd_ops::ru8(rsp.wrapping_add(0x90u64))) as u32) as u32 as u64; r8 = rsp.wrapping_add(0x80); rdx = rsp.wrapping_add(0x78); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            7 => { rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141610ae0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xffffffffffffffb1; const P1: u64 = 0x141524330; const P2: u64 = 0x141651390; const P3: u64 = 0x48; const P4: u64 = 0x14268f478;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdx = 0x50;
    rdi = ((rcx as u32)) as u32 as u64;
    rcx = ((rdx.wrapping_add(P0)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rbx = rax;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rdx = ((rdi as u32)) as u32 as u64; rcx = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_and_u8(rbx.wrapping_add(P3), (0xfcu8) as u8); rax = P4; crate::simd_ops::wu64(rbx, rax); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141610b40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141651410; const P1: u64 = 0x14160f9a0; const P2: u64 = 0x141651590; const P3: u64 = 0x1415995a0; const P4: u64 = 0x143930f08; const P5: u64 = 0x1415ab990; const P6: u64 = 0x14160f710; const P7: u64 = 0x1416514c0; const P8: u64 = 0x1415ab5b0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; r9 = ((crate::simd_ops::ru16(rcx.wrapping_add(0x34u64))) as u32) as u32 as u64; rdi = rdx; rsi = (crate::simd_ops::ru64(rdx.wrapping_add(0x40u64))) as u64; rbx = rcx; fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x102; r9 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 1; continue; }
            1 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x1; r9 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 2; continue; }
            2 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x1; r9 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x40u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { r9 = (crate::simd_ops::ru32(rdx.wrapping_add(0x38u64))) as u32 as u64; r8 = rsi; rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            6 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x36u64))) as u32) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rcx); fl_l = rsi; fl_r = rsi; crate::simd_ops::wu8(rsp.wrapping_add(0x3cu64), ((rcx as u8)) as u8); crate::simd_ops::wu8(rsp.wrapping_add(0x3du64), (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { 1u8 } else { 0u8 }) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rcx as u32)) as u32); rax = ((((rax as u32)) & (0x1fu32))) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x3eu64), (0x0u16) as u16); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rax as u32)) as u32); rcx = rbx; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x38u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x40u64), (0x0u8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((rax as u32)) as u32); rdx = rsp.wrapping_add(0x20); rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x68u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rdx = rsp.wrapping_add(0x50); rcx = rbx; xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm1) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = 0; fl_l = 0; fl_r = 0; rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            7 => { r9 = (crate::simd_ops::ru32(rdi.wrapping_add(0x38u64))) as u32 as u64; r8 = rbx.wrapping_add(0x38); rcx = (crate::simd_ops::ru64(P4)) as u64; rdx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { rax = 0x1; bb = 15; continue; }
            9 => { crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rbp); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 10; continue; }
            10 => { r9 = (crate::simd_ops::ru32(rdi.wrapping_add(0x38u64))) as u32 as u64; r8 = rsi; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru32(rdi.wrapping_add(0x38u64))) as u32 as u64; r8 = rsi; rcx = (crate::simd_ops::ru64(P4)) as u64; rdx = rbp; rbx = ((rax as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x10); rdx = 0xffffffff; let xa_src: u32 = (rdx as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rdx = (xa_old) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { rax = ((rbx as u32)) as u32 as u64; bb = 14; continue; }
            13 => { rax = 0x1; bb = 14; continue; }
            14 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; bb = 15; continue; }
            15 => { r11 = rsp.wrapping_add(0x80); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141610cd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x48; const P1: u64 = 0x1f; const P2: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x1u8))) as u8 as u64));
    crate::simd_ops::wu8(rdx.wrapping_add(P1), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shr(1u32)) as u8 as u64));
    fl_l = ((((rax as u8)) & (0x1u8))) as u8 as u64; fl_r = fl_l;
    rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64));
    crate::simd_ops::wu8(rdx.wrapping_add(P2), ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141610cf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x48;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    r8 = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(1)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0x1;
    r8 = (r8 & !0xffu64) | (((((r8 as u8)).wrapping_shr(1u32)) as u8 as u64));
    r8 = (r8 & !0xffu64) | ((((((r8 as u8)) ^ (crate::simd_ops::ru8(rcx.wrapping_add(P0))))) as u8 as u64));
    r8 = (r8 & !0xffu64) | ((((((r8 as u8)) & (0x3u8))) as u8 as u64));
    fl_l = (crate::simd_ops::lk_xor_u8(rcx.wrapping_add(P0), ((r8 as u8)) as u8)) as u8 as u64; fl_r = fl_l;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141610d20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14268f4e8; const P1: u64 = 0x143930f80; const P2: u64 = 0x1415adef0; const P3: u64 = 0x143930fa8; const P4: u64 = 0x1415f8290; const P5: u64 = 0x14160d400; const P6: u64 = 0x1415c2070; const P7: u64 = 0x141620d50; const P8: u64 = 0x14164dd50;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x40u64))) as u64; rax = P0; crate::simd_ops::wu64(rcx, rax); rbx = rcx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0xd8u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x48u64))) as u64; rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx.wrapping_add(0x40u64), rsi); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0xd8u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x48u64), rsi); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(P3)) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0xf0u64))) as u64; rdi = 0xffffffff; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { rcx = ((rcx).wrapping_add(0x10u64)) as u64; rax = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rbx.wrapping_add(0xf0u64), rsi); bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0xe0u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 13; continue; }
            13 => { let xa_src: u32 = (rdi as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rdi = (xa_old) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            15 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x110u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 16; continue; }
            16 => { r9 = (crate::simd_ops::ru64(rdx.wrapping_add(0x70u64))) as u64; r8 = rsi; rcx = r9; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 18; continue; }
            18 => { r8 = rcx; rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 17; continue; } bb = 19; continue; }
            19 => { bb = 26; continue; }
            20 => { fl_l = rcx; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 22; continue; } bb = 21; continue; }
            21 => { crate::simd_ops::wu64(rdx.wrapping_add(0x70u64), rax); bb = 23; continue; }
            22 => { crate::simd_ops::wu64(r8, rax); bb = 23; continue; }
            23 => { fl_l = rcx; fl_r = (crate::simd_ops::ru64(rdx.wrapping_add(0x78u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 25; continue; } bb = 24; continue; }
            24 => { crate::simd_ops::wu64(rdx.wrapping_add(0x78u64), r8); bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu64(rcx, rax); fl_l = (crate::simd_ops::lk_sub_u32(rdx.wrapping_add(0x94u64), (1) as u32)) as u32 as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x80u64), rcx); bb = 26; continue; }
            26 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 27; continue; }
            27 => { fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x6cu64))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 30; continue; } bb = 28; continue; }
            28 => { fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x94u64))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 30; continue; } bb = 29; continue; }
            29 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 30; continue; }
            30 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x110u64))) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x110u64), rsi); bb = 31; continue; }
            31 => { rcx = rbx.wrapping_add(0x118); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x60u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 32; continue; }
            32 => { rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x60u64), rsi); bb = 33; continue; }
            33 => { rcx = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P8, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141610ef0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x100; const P1: u64 = 0x8; const P2: u64 = 0x108; const P3: u64 = 0x110; const P4: u64 = 0x14160d040; const P5: u64 = 0x80; const P6: u64 = 0x18; const P7: u64 = 0x141524330; const P8: u64 = 0x78; const P9: u64 = 0x70; const P10: u64 = 0x94;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; xmm0 = ((crate::simd_ops::ru64(r9)) as u64) as u128; rsi = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu64(rcx.wrapping_add(P0), ((xmm0 as u64)) as u64); rdi = rcx; rax = (crate::simd_ops::ru32(r9.wrapping_add(P1))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_add(P3), r8); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 1; continue; }
            1 => { rcx = r8; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rdx = ((rax.wrapping_add(P6)) as u32) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; crate::simd_ops::wu64(rax, rcx); crate::simd_ops::wu64(rbx.wrapping_add(P5), rax); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P8))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbx.wrapping_add(P9), rax); bb = 7; continue; }
            6 => { crate::simd_ops::wu64(rcx, rax); bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P8), rcx); rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P5), rax); crate::simd_ops::wu64(rcx, (0x0u64) as u64); crate::simd_ops::lk_add_u32(rbx.wrapping_add(P10), (1) as u32); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P8))) as u64; fl_l = ((rax).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu32(rax, ((rsi as u32)) as u32); crate::simd_ops::wu64(rax.wrapping_add(P1), rdi); bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 10; continue; }
            10 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141610fd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415c2680;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = rcx;
    rcx = (crate::simd_ops::ru64(rdx)) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { fl_l = ((r8).wrapping_add(0x118u64)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; rdx = 0x1; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141610ff0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415c1150; const P1: u64 = 0x141620bb0; const P2: u64 = 0x141621270; const P3: u64 = 0x141620d00; const P4: u64 = 0x141620d50; const P5: u64 = 0x1415fa780;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(0x60u64))) as u64; rbp = r8; rsi = rdx; rdi = rcx; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 1; continue; }
            1 => { r14 = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(r9.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 18; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(r9.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(r9)) as u64; rbx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x10u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_sub(0x28u64))) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 24; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x68u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x68u64), ((rax as u32)) as u32); if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 6; continue; }
            6 => { bb = 7; continue; }
            7 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x20; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 11; continue; } bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0xacu64))) as u32 as u64; fl_l = (((rdx) >> (((rcx) as u32 & 63))) & 1) as u64; if ((fl_l as u64) != 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu32(rdi.wrapping_add(0x68u64), ((rcx as u32)) as u32); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 7; continue; } bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rcx.wrapping_sub(0x5); rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_sub_u32(rbx.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 17; continue; }
            11 => { crate::simd_ops::wu32(rdi.wrapping_add(0x68u64), ((rcx as u32)) as u32); bb = 7; continue; }
            12 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_sub(0x8u64))) as u64; r8 = rsp.wrapping_add(0x38); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r14); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rcx.wrapping_sub(0x5); rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_sub_u32(rbx.wrapping_add(0x8u64), (1) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 13; continue; }
            13 => { rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x60u64), rbx); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            17 => { r9 = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; fl_l = (crate::simd_ops::ru32(r9.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 22; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::wu64(rdi.wrapping_add(0x60u64), r14); bb = 20; continue; }
            20 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 22; continue; }
            22 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 23; continue; }
            23 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            24 => { rax = rbx.wrapping_sub(0x18); rdx = rbp; rcx = rbx.wrapping_sub(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); r9 = rsp.wrapping_add(0x30); rcx = (crate::simd_ops::ru64(rbx.wrapping_sub(0x28u64))) as u64; r8 = rsp.wrapping_add(0x60); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 25; continue; }
            25 => { rcx = ((crate::simd_ops::ru16(rsp.wrapping_add(0x60u64))) as u32) as u32 as u64; rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x68u64))) as u32 as u64; rdx = (((rdx as u32)).wrapping_sub(1)) as u32 as u64; crate::simd_ops::wu16(rdi.wrapping_add(rdx.wrapping_mul(2)).wrapping_add(0x6cu64), ((rcx as u16)) as u16); rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x20u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x30u64), ((rcx as u32)) as u32); rcx = rax.wrapping_add(0x10); rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 27; continue; } bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 27; continue; }
            27 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 23; continue; }
            28 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x60u64), r14); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 29; continue; }
            29 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 20; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416111e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524330; const P1: u64 = 0x14164dd20; const P2: u64 = 0x14268f4e8; const P3: u64 = 0x40; const P4: u64 = 0x48; const P5: u64 = 0x60; const P6: u64 = 0x141620d00; const P7: u64 = 0xcc; const P8: u64 = 0xac; const P9: u64 = 0xb0; const P10: u64 = 0xb8; const P11: u64 = 0xc0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rdx; rdi = ((rcx as u32)) as u32 as u64; rdx = 0x128; rcx = 0x1; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdx = ((rdi as u32)) as u32 as u64; rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdi = 0; fl_l = 0; fl_r = 0; rax = P2; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(P3), rdi); crate::simd_ops::wu64(rbx.wrapping_add(P4), rdi); rcx = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P5), rcx); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { crate::simd_ops::lk_and_u8(rbx.wrapping_add(P7), (0xfeu8) as u8); rax = rbx; crate::simd_ops::wu32(rbx.wrapping_add(P8), ((rdi as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(P9), rdi); crate::simd_ops::wu64(rbx.wrapping_add(P10), rdi); crate::simd_ops::wu32(rbx.wrapping_add(P11), ((rdi as u32)) as u32); crate::simd_ops::wu16(rbx.wrapping_add(0xc4u64), (0x1u16) as u16); crate::simd_ops::wu32(rbx.wrapping_add(0xc8u64), ((rdi as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0xd0u64), (0x1u32) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0xd8u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0xe0u64), rdi); crate::simd_ops::wu32(rbx.wrapping_add(0xe8u64), ((rdi as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0xf0u64), rdi); crate::simd_ops::wu32(rbx.wrapping_add(0xf8u64), ((rdi as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0xfcu64), (0x4u32) as u32); crate::simd_ops::wu16(rbx.wrapping_add(0x100u64), (0xff00u16) as u16); crate::simd_ops::wu64(rbx.wrapping_add(0x110u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x118u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x120u64), rdi); crate::simd_ops::wu32(rbx.wrapping_add(0x68u64), ((rdi as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x50u64), rdi); crate::simd_ops::wu32(rbx.wrapping_add(0x58u64), ((rdi as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdi = 0; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = ((rdi as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141611300(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415970c0; const P1: u64 = 0x14160f710; const P2: u64 = 0x141620d00; const P3: u64 = 0x141620d50; const P4: u64 = 0x14152ec40; const P5: u64 = 0x1415864b0; const P6: u64 = 0x14160fa60; const P7: u64 = 0x1415945c0; const P8: u64 = 0x1415bfb60; const P9: u64 = 0x1415c0d60; const P10: u64 = 0x143931d40; const P11: u64 = 0x143930fa8; const P12: u64 = 0x1415f8290; const P13: u64 = 0x14152ede0; const P14: u64 = 0x1415c2070;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 117>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x280); rsp = ((rsp).wrapping_sub(0x380u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0xb8u64))) as u32 as u64; r8 = 0x4; rbx = rcx; crate::simd_ops::wu32(rbp.wrapping_add(0x2c0u64), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0xb0u64))) as u64; r15 = rdx; crate::simd_ops::wu32(rbp.wrapping_add(0x2c4u64), ((r8 as u32)) as u32); r13 = ((r8.wrapping_sub(0x3u64)) as u32) as u32 as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x3d8u64))) as u32) as u32 as u64; rdx = rbp.wrapping_add(0x2c0); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xfu8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; r8 = (if (((fl_l as u8)) == ((fl_r as u8))) { (r13 as u32) } else { (r8 as u32) }) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x2c4u64), ((r8 as u32)) as u32); r8 = (((r13 as u8)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0xf8u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x2c0u64), rax); bb = 3; continue; }
            3 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 35; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x3b8u64), rsi); rsi = rbx.wrapping_add(0x50); crate::simd_ops::wu64(rsp.wrapping_add(0x378u64), rdi); rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x370u64), r12); r12 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rcx = ((r12 as u32)) as u32 as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xe8u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rcx); crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r12); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(0x68u64))) as u128; rcx = rbp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), r12); xmm1 = (crate::simd_ops::ru128(rbx.wrapping_add(0x78u64))) as u128; crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), r12); crate::simd_ops::wu128(rbp.wrapping_sub(0x58u64), (xmm0) as u128); crate::simd_ops::wu64(rbp.wrapping_sub(0x68u64), r12); xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(0x88u64))) as u128; crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), r12); crate::simd_ops::wu128(rbp.wrapping_sub(0x48u64), (xmm1) as u128); crate::simd_ops::wu64(rbp, r12); xmm1 = (crate::simd_ops::ru128(rbx.wrapping_add(0x98u64))) as u128; crate::simd_ops::wu16(rbp.wrapping_add(0x8u64), (0xff00u16) as u16); crate::simd_ops::wu128(rbp.wrapping_sub(0x38u64), (xmm0) as u128); crate::simd_ops::wu64(rbp.wrapping_add(0x18u64), r12); xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(0xa8u64))) as u64) as u128; crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu128(rbp.wrapping_sub(0x28u64), (xmm1) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0xc8u64))) as u64; rdx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x268u64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x70u64))) as u64; rdx = (rdx & !0xffu64) | ((((((rdx as u8)) & (0xf3u8))) as u8 as u64)); rdi = (crate::simd_ops::ru64(r15.wrapping_add(0x1cu64))) as u64; rdx = (rdx & !0xffu64) | ((((((rdx as u8)) | (0x3u8))) as u8 as u64)); crate::simd_ops::wu64(rbp.wrapping_add(0x260u64), rax); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xd0u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0xe0u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rax); rax = (crate::simd_ops::ru64(r15.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rax); rax = (crate::simd_ops::ru32(r15.wrapping_add(0x30u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x5cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(r15.wrapping_add(0x24u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x68u64), rax); rax = (crate::simd_ops::ru32(r15.wrapping_add(0x2cu64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x60u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x268u64), ((rdx as u8)) as u8); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x268u64))) as u32) as u32 as u64; bb = 10; continue; }
            10 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::lk_add_u32(rdi, (1) as u32)) as u32 as u64; rdx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x268u64))) as u32) as u32 as u64; bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xbcu64))) as u32 as u64; xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x100u64))) as u64) as u128; rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x10u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r15.wrapping_add(0x14u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0xcu64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0xc5u64))) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_add((rax as u8))) as u8 as u64)); crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), rdi); rax = (rax & !0xffu64) | ((((((rax as u8)) ^ ((rdx as u8)))) as u8 as u64)); crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), ((xmm0 as u64)) as u64); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x2u8))) as u8 as u64)); rdx = (rdx & !0xffu64) | ((((((rdx as u8)) ^ ((rax as u8)))) as u8 as u64)); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xc0u64))) as u32 as u64; crate::simd_ops::wu32(rbp, ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x108u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x110u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x18u64), rax); rax = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rbp.wrapping_sub(0x8u64), rax); rax = rbp.wrapping_add(0x2c0); crate::simd_ops::wu64(rbp.wrapping_sub(0x78u64), rax); crate::simd_ops::wu8(rbp.wrapping_add(0x268u64), ((rdx as u8)) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (((rax as u8)) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x268u64))) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xfbu8))) as u8 as u64)); rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_shl(2u32)) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) | ((rax as u8)))) as u8 as u64)); fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0x90u8) as u8 as u64; crate::simd_ops::wu8(rbp.wrapping_add(0x268u64), ((rcx as u8)) as u8); if (((fl_l as u8)) != ((fl_r as u8))) { bb = 16; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0xdu64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) <= ((fl_r as u8))) { bb = 16; continue; } bb = 14; continue; }
            14 => { rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & ((r13 as u8)))) as u8 as u64)); crate::simd_ops::wu8(rbp.wrapping_add(0x2b0u64), ((rcx as u8)) as u8); rcx = rsp.wrapping_add(0x70); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = ((rax as u32)) as u32 as u64; r9 = rbp.wrapping_add(0x2b0); rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x5cu64))) as u32 as u64; rdx = rbp.wrapping_add(0x8); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); rcx = r14; rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x80u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 20; continue; } bb = 15; continue; }
            15 => { rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x268u64))) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x2b0u64))) as u32) as u32 as u64; rcx = (rcx & !0xffu64) | ((((((rcx as u8)) ^ ((rax as u8)))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & ((r13 as u8)))) as u8 as u64)); fl_l = ((((rax as u8)) ^ ((rcx as u8)))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbp.wrapping_add(0x268u64), ((rax as u8)) as u8); bb = 16; continue; }
            16 => { rdx = rsp.wrapping_add(0x70); rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 18; continue; } bb = 17; continue; }
            17 => { rdi = ((r13 as u32)) as u32 as u64; bb = 20; continue; }
            18 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(r14)) as u64; rdx = rsp.wrapping_add(0x70); rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x250u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rax as u32)) as u32 as u64; bb = 20; continue; }
            20 => { crate::simd_ops::lk_add_u32(P10, (1) as u32); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x60u64), r12); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 22; continue; }
            22 => { rdx = (crate::simd_ops::ru64(rsi)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru64(P11)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi, r12); crate::simd_ops::wu32(rbx.wrapping_add(0x58u64), ((r12 as u32)) as u32); bb = 24; continue; }
            24 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0xf0u64))) as u64; rsi = 0xffffffff; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 25; continue; }
            25 => { rcx = ((rcx).wrapping_add(0x10u64)) as u64; rax = ((rsi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 27; continue; } bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 27; continue; }
            27 => { crate::simd_ops::wu64(rbx.wrapping_add(0xf0u64), r12); bb = 28; continue; }
            28 => { rcx = r14.wrapping_add(0x10); let xa_src: u32 = (rsi as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rsi = (xa_old) as u32 as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x370u64))) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x3b8u64))) as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 30; continue; } bb = 29; continue; }
            29 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 30; continue; }
            30 => { rcx = rbp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x70u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 31; continue; }
            31 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 32; continue; }
            32 => { rcx = rsp.wrapping_add(0x58); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 34; continue; } bb = 33; continue; }
            33 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 34; continue; }
            34 => { rax = ((rdi as u32)) as u32 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x378u64))) as u64; rsp = ((rsp).wrapping_add(0x380u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            35 => { rax = 0xf; rsp = ((rsp).wrapping_add(0x380u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416116f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x68; const P1: u64 = 0x78; const P2: u64 = 0x10; const P3: u64 = 0x88; const P4: u64 = 0x20; const P5: u64 = 0x98; const P6: u64 = 0x30; const P7: u64 = 0xa8; const P8: u64 = 0x40;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(P0))) as u128;
    crate::simd_ops::wu128(rdx, (xmm0) as u128);
    xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(P1))) as u128;
    crate::simd_ops::wu128(rdx.wrapping_add(P2), (xmm1) as u128);
    xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(P3))) as u128;
    crate::simd_ops::wu128(rdx.wrapping_add(P4), (xmm0) as u128);
    xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(P5))) as u128;
    crate::simd_ops::wu128(rdx.wrapping_add(P6), (xmm1) as u128);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P7))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P8), ((rax as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141611740(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x48; const P1: u64 = 0x1415ad830; const P2: u64 = 0xd8; const P3: u64 = 0x143930f80; const P4: u64 = 0x1415adef0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64;
    if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { crate::simd_ops::wu64(rbx.wrapping_add(P0), (0x0u64) as u64); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    r8 = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64;
    fl_l = r8; fl_r = r8;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = (crate::simd_ops::ru64(P3)) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
    crate::simd_ops::wu64(rbx.wrapping_add(P0), (0x0u64) as u64);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141611790(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xb0; const P1: u64 = 0xb8; const P2: u64 = 0xc4;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rdx);
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((r8 as u32)) as u32);
    crate::simd_ops::wu8(rcx.wrapping_add(P2), (0x0u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416117b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x68; const P1: u64 = 0x10; const P2: u64 = 0x78; const P3: u64 = 0x20; const P4: u64 = 0x88; const P5: u64 = 0x30; const P6: u64 = 0x98; const P7: u64 = 0x40; const P8: u64 = 0xa8; const P9: u64 = 0x44; const P10: u64 = 0xac;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P0), (xmm0) as u128);
    xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(P1))) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P2), (xmm1) as u128);
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P3))) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P4), (xmm0) as u128);
    xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(P5))) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P6), (xmm1) as u128);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P7))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P8), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P9))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P10), ((rax as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141611800(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xe0; const P1: u64 = 0x14;
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rdx);
    fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P1), (1) as u32)) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141611820(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930fa8; const P1: u64 = 0x1415f8000; const P2: u64 = 0x143930f88; const P3: u64 = 0x1426452b0; const P4: u64 = 0x1426452a8; const P5: u64 = 0x142645560;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r15 = (crate::simd_ops::ru64(rcx.wrapping_add(0xf0u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rbx = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r12); r12 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); r14 = rcx; crate::simd_ops::wu64(rcx.wrapping_add(0x50u64), r12); crate::simd_ops::wu32(rcx.wrapping_add(0x58u64), ((r12 as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0xf0u64), r12); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 29; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbp); crate::simd_ops::wu64(r14.wrapping_add(0x50u64), rax); rbp = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x58u64), ((rbp as u32)) as u32); rbx = (crate::simd_ops::ru64(P2)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rsi); bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 15; continue; } bb = 5; continue; }
            5 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 6; continue; }
            6 => { rsi = rbx.wrapping_add(0x8); bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 17; continue; } bb = 8; continue; }
            8 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbp as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 11; continue; }
            11 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 12; continue; }
            12 => { rdi = r12; bb = 13; continue; }
            13 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 18; continue; }
            15 => { rsi = rbx.wrapping_add(0x8); rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 7; continue; } bb = 16; continue; }
            16 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            17 => { rdi = r12; bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 26; continue; } bb = 19; continue; }
            19 => { bb = 20; continue; }
            20 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 23; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 20; continue; } bb = 22; continue; }
            22 => { bb = 27; continue; }
            23 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 28; continue; } bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 28; continue; } bb = 25; continue; }
            25 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); bb = 28; continue; }
            26 => { rsi = rbx.wrapping_add(0x8); bb = 27; continue; }
            27 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 28; continue; }
            28 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = rdi.wrapping_sub(0x10); fl_l = rdi; fl_r = rdi; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r12 } else { rax }) as u64; crate::simd_ops::wu64(r14.wrapping_add(0xf0u64), rax); bb = 29; continue; }
            29 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 30; continue; }
            30 => { rcx = r15.wrapping_add(0x10); rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 32; continue; } bb = 31; continue; }
            31 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
            32 => { rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141611aa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xbc;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu32(rcx.wrapping_add(P0), (0x1u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141611ab0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xb0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = rdx; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { crate::simd_ops::wu64(rcx.wrapping_add(P0), (0x0u64) as u64); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141611ad0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931d88; const P1: u64 = 0x143931d80; const P2: u64 = 0x10; const P3: u64 = 0x8; const P4: u64 = 0x18; const P5: u64 = 0xffffffffffffffff;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rax = (crate::simd_ops::ru32(P0)) as u32 as u64; r10 = rdx; rdi = (crate::simd_ops::ru64(P1)) as u64; r11 = ((rcx as u32)) as u32 as u64; rbp = 0; fl_l = 0; fl_r = 0; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rbx = ((rdx.wrapping_mul(8)) as u64) as u64; rsi = ((rbx.wrapping_add(rdi)) as u64) as u64; bb = 1; continue; }
            1 => { rdx = ((rbx.wrapping_add(rdi)) as u64) as u64; rax = rdi; fl_l = rdi; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = ((rax).wrapping_add(0x28u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 7; continue; }
            7 => { fl_l = rax; fl_r = rsi; rcx = ((rax.wrapping_add(P4)) as u64) as u64; rcx = (if (((fl_l as u64)) == ((fl_r as u64))) { rbp } else { rcx }) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { fl_l = r10; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { r10 = ((rcx.wrapping_add(P5)) as u64) as u64; bb = 1; continue; }
            10 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(r9, rbp); crate::simd_ops::wu32(rax, ((rbp as u32)) as u32); bb = 12; continue; }
            11 => { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(r9, rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu32(rax, ((rcx as u32)) as u32); bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141611b90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524340; const P1: u64 = 0x141524330; const P2: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rdi = rcx; rbp = ((rdx as u32)) as u32 as u64; rcx = 0x1; rbp = (((rbp as u32)).wrapping_shr(1u32)) as u32 as u64; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; rsi = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rbp = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rcx as u32) } else { (rbp as u32) }) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi)) as u64; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; rdx = ((rbp.wrapping_mul(4)) as u64) as u64; rdx = ((rdx).wrapping_add(rbp)) as u64; rdx = ((rdx).wrapping_shl(3u32)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = rdx; rdx = rcx; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 2; continue; }
            2 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 13; continue; }
            3 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 5; continue; }
            5 => { fl_l = rax; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 6; continue; }
            6 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { rax = rbx; r8 = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu32(rax, ((r8 as u32)) as u32); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), r8); crate::simd_ops::wu32(rax.wrapping_add(0x10u64), ((r8 as u32)) as u32); bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rdx = rax; rdx = ((rdx).wrapping_sub(rbx)) as u64; xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(rcx))) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(rcx).wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(rax.wrapping_add(0x10u64), (xmm1) as u128); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(rcx).wrapping_add(0x20u64))) as u64) as u128; crate::simd_ops::wu64(rax.wrapping_add(0x20u64), ((xmm0 as u64)) as u64); rax = ((rax).wrapping_add(0x28u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rdi, rbx); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((rbp as u32)) as u32); bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141611c80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931d80; const P1: u64 = 0x143931d88; const P2: u64 = 0x1415242e0; const P3: u64 = 0x14183acc8; const P4: u64 = 0x143931d8c; const P5: u64 = 0x141611b90;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbx = (crate::simd_ops::ru32(rsp.wrapping_add(0x80u64))) as u32 as u64; rsi = ((rcx as u32)) as u32 as u64; rbx = (((rbx as u32)).wrapping_shl(12u32)) as u32 as u64; rdi = rdx; rbx = (((rbx as u32)).wrapping_add((r9 as u32))) as u32 as u64; rax = (((r8 as u8)) as u32) as u32 as u64; r9 = (crate::simd_ops::ru64(P0)) as u64; r8 = (crate::simd_ops::ru32(P1)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rcx as u32)) as u32); r15 = ((r8 as u32)) as u32 as u64; rbx = (((rbx as u32)).wrapping_shl(4u32)) as u32 as u64; rbx = (((rbx as u32)).wrapping_add((rax as u32))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdx); rcx = ((r8.wrapping_add(r8.wrapping_mul(4))) as u64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rbx as u32)) as u32); rdx = ((r9.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = r9; rcx = ((r9.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = r9; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = ((rax).wrapping_add(0x28u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 5; continue; }
            5 => { r12 = 0; fl_l = 0; fl_r = 0; r14 = rax.wrapping_add(0x18); fl_l = rax; fl_r = rdx; r14 = (if (((fl_l as u64)) == ((fl_r as u64))) { r12 } else { r14 }) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0xcu64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(r14)) as u64; rcx = ((r12.wrapping_add(0x5u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru32(P1)) as u32 as u64; r9 = (crate::simd_ops::ru64(P0)) as u64; bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 19; continue; } bb = 9; continue; }
            9 => { rax = ((r8 as u32)) as u32 as u64; rcx = r9; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((r9.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; r10 = ((r9.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = r9; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 10; continue; }
            10 => { bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rcx = ((rcx).wrapping_add(0x28u64)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 15; continue; }
            15 => { fl_l = rcx; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 16; continue; }
            16 => { rax = ((r8.wrapping_sub(0x1u64)) as u32) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rdx = ((r9.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 18; continue; } bb = 17; continue; }
            17 => { rdx = ((rdx).wrapping_sub(rcx)) as u64; rax = (0x6666666666666667u64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rdx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(4u32) as u64)) as u64; r8 = rdx; r8 = ((r8).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; r8 = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(3u32)) as u32 as u64; rdx = rcx.wrapping_add(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru32(P1)) as u32 as u64; bb = 18; continue; }
            18 => { fl_l = (((r8 as u32)).wrapping_sub(1)) as u32 as u64; r8 = ((fl_l as u32)) as u32 as u64; rax = 0x1; crate::simd_ops::wu32(P1, ((r8 as u32)) as u32); bb = 41; continue; }
            19 => { crate::simd_ops::wu64(r14, rax); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x90u64))) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x98u64))) as u32) as u32 as u64; crate::simd_ops::wu8(r14.wrapping_add(0xcu64), ((rax as u8)) as u8); bb = 20; continue; }
            20 => { rax = 0x1; bb = 41; continue; }
            21 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbp); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 39; continue; } bb = 22; continue; }
            22 => { rax = r9; fl_l = r9; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 28; continue; } bb = 23; continue; }
            23 => { bb = 24; continue; }
            24 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 27; continue; } bb = 25; continue; }
            25 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 27; continue; } bb = 26; continue; }
            26 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 28; continue; } bb = 27; continue; }
            27 => { rax = ((rax).wrapping_add(0x28u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 24; continue; } bb = 28; continue; }
            28 => { fl_l = rax; fl_r = rdx; rcx = rax.wrapping_add(0x18); rcx = (if (((fl_l as u64)) == ((fl_r as u64))) { r12 } else { rcx }) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 38; continue; } bb = 29; continue; }
            29 => { rcx = (crate::simd_ops::ru32(P4)) as u32 as u64; fl_l = r15; fl_r = rcx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 32; continue; } bb = 30; continue; }
            30 => { rcx = P0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 35; continue; } bb = 31; continue; }
            31 => { rcx = (crate::simd_ops::ru32(P4)) as u32 as u64; r8 = (crate::simd_ops::ru32(P1)) as u32 as u64; r9 = (crate::simd_ops::ru64(P0)) as u64; bb = 32; continue; }
            32 => { rax = ((rcx as u32)) as u32 as u64; fl_l = r15; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 35; continue; } bb = 33; continue; }
            33 => { rax = ((r8 as u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(P1, ((r8 as u32)) as u32); rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((r9.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 35; continue; } bb = 34; continue; }
            34 => { xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rcx = rax.wrapping_add(0x18); crate::simd_ops::wu32(rax, ((r12 as u32)) as u32); xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64) as u128; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), r12); crate::simd_ops::wu32(rax.wrapping_add(0x10u64), ((r12 as u32)) as u32); crate::simd_ops::wu128(rax, (xmm0) as u128); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 38; continue; } bb = 35; continue; }
            35 => { fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x98u64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 37; continue; } bb = 36; continue; }
            36 => { rdx = rbp; rcx = 0x5; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 37; continue; }
            37 => { rax = 0x34; bb = 40; continue; }
            38 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x90u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x98u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(0xcu64), ((rax as u8)) as u8); crate::simd_ops::wu64(rcx, rbp); bb = 39; continue; }
            39 => { rax = 0x1; bb = 40; continue; }
            40 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; bb = 41; continue; }
            41 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141611f20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931d80; const P1: u64 = 0x143931d88; const P2: u64 = 0x24; const P3: u64 = 0x18; const P4: u64 = 0x1415242e0; const P5: u64 = 0x143931d8c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r8 = (crate::simd_ops::ru64(P0)) as u64; r9 = (crate::simd_ops::ru32(P1)) as u32 as u64; rbx = r8; rcx = ((r9.wrapping_add(r9.wrapping_mul(4))) as u64) as u64; rax = ((r8.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = 0x5; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); r9 = (crate::simd_ops::ru32(P1)) as u32 as u64; r8 = (crate::simd_ops::ru64(P0)) as u64; bb = 4; continue; }
            4 => { rax = ((r9 as u32)) as u32 as u64; rbx = ((rbx).wrapping_add(0x28u64)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((r8.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 5; continue; }
            5 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rbx = 0; fl_l = 0; fl_r = 0; rdx = r8; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(P1, ((rbx as u32)) as u32); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P0, rbx); crate::simd_ops::wu32(P5, ((rbx as u32)) as u32); bb = 7; continue; }
            7 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141611fb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14268f540; const P1: u64 = 0x10; const P2: u64 = 0x8; const P3: u64 = 0x18; const P4: u64 = 0x14; const P5: u64 = 0x2c; const P6: u64 = 0x20; const P7: u64 = 0x28; const P8: u64 = 0x40; const P9: u64 = 0x48; const P10: u64 = 0x50; const P11: u64 = 0x30; const P12: u64 = 0x1415af330;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rax = P0; crate::simd_ops::wu64(rcx, rax); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx.wrapping_add(P1), rdx); crate::simd_ops::wu64(rbx.wrapping_add(P2), rcx); crate::simd_ops::wu64(rbx.wrapping_add(P3), rcx); rax = (crate::simd_ops::ru32(r9.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P5), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(r9.wrapping_add(P2))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P6), rax); rax = (crate::simd_ops::ru32(r9.wrapping_add(P1))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P7), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(r9)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::lk_add_u32(rax, (1) as u32)) as u32 as u64; bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rbx.wrapping_add(P3), rax); rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P8), rcx); crate::simd_ops::wu8(rbx.wrapping_add(P9), ((rcx as u8)) as u8); crate::simd_ops::wu8(rbx.wrapping_add(P10), ((rcx as u8)) as u8); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P11), rax); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x50u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x38u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x4cu64), (0x1u32) as u32); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x60u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 8; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x64u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x60u64), ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x64u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rax = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            8 => { fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x60u64), ((rax as u32)) as u32); bb = 9; continue; }
            9 => { rax = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141612060(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x14268f540; const P2: u64 = 0x60; const P3: u64 = 0x64; const P4: u64 = 0x1415af340; const P5: u64 = 0x1415242e0; const P6: u64 = 0x40; const P7: u64 = 0x14; const P8: u64 = 0x18; const P9: u64 = 0x1415864b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = P1; crate::simd_ops::wu64(rcx, rax); rdi = rcx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P3))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (rdx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 5; continue; }
            4 => { rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 5; continue; }
            5 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P2), ((rcx as u32)) as u32); if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 6; continue; }
            6 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P3))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0x4; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P6))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 13; continue; }
            13 => { rcx = ((rcx).wrapping_add(0x10u64)) as u64; rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(P7), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P8))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P8))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            17 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141612110(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14268f540; const P1: u64 = 0x1415af340; const P2: u64 = 0x1415242e0; const P3: u64 = 0x1415864b0; const P4: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rax = P0; crate::simd_ops::wu64(rcx, rax); rsi = ((rdx as u32)) as u32 as u64; rbx = rcx; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x60u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x64u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (rdx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 5; continue; }
            4 => { rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 5; continue; }
            5 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x60u64), ((rcx as u32)) as u32); if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 6; continue; }
            6 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x64u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x60u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rdi; rcx = 0x4; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 13; continue; }
            13 => { rcx = ((rcx).wrapping_add(0x10u64)) as u64; rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            17 => { fl_l = ((rsi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { rdx = 0x58; rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 19; continue; }
            19 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416121e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524330;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r10 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rbp = ((r9 as u32)) as u32 as u64; rdi = r8; rsi = rdx; rbx = rcx; fl_l = ((r10 as u8)) as u8 as u64; fl_r = ((r10 as u8)) as u8 as u64; if (((((fl_l as u8)) & ((fl_r as u8))) as i8) >= 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (((r10 as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xf0u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xf0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = ((r10 as u8)) as u8 as u64; fl_r = (0xfeu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 19; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x50u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 19; continue; } bb = 4; continue; }
            4 => { rdx = 0x28; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rax, rbx); r10 = 0; fl_l = 0; fl_r = 0; xmm0 = ((crate::simd_ops::ru64(rdi)) as u64) as u128; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(0x10u64), ((rcx as u32)) as u32); rcx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x58u64))) as u32) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(0x14u64), ((rbp as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x50u64))) as u32 as u64; rax = ((((rax as u32)) & (0x3fffffffu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(30u32)) as u32 as u64; rcx = ((((rcx as u32)) | ((rax as u32)))) as u32 as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r10); rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x60u64))) as u32) as u32 as u64; fl_l = ((((rcx as u32)) >> (31u32)) & 1) as u64; rcx = (((rcx as u32)) & !((1u32) << (31u32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(31u32)) as u32 as u64; rcx = ((((rcx as u32)) | ((rax as u32)))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x18u64), ((rcx as u32)) as u32); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x4cu64), (1) as u32); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 6; continue; }
            6 => { r8 = (crate::simd_ops::ru32(rdx.wrapping_add(0x14u64))) as u32 as u64; bb = 7; continue; }
            7 => { r9 = rax; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x14u64))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 14; continue; } bb = 8; continue; }
            8 => { if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x18u64))) as u32 as u64; fl_l = ((((rcx as u32)) >> (30u32)) & 1) as u64; if ((fl_l as u32) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x18u64))) as u32 as u64; fl_r = 0x40000000; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 11; continue; }
            11 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x18u64))) as u32 as u64; fl_r = 0x0; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; r10 = r9; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), rax); fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rdx); bb = 17; continue; }
            16 => { crate::simd_ops::wu64(r10.wrapping_add(0x20u64), rdx); bb = 17; continue; }
            17 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { crate::simd_ops::wu64(rsi, rdx); bb = 19; continue; }
            19 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141612310(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x40; const P2: u64 = 0x141612750;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_r = (0x0u64) as u64;
    if (((fl_l as u64)) == ((fl_r as u64))) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rdx = rbx; rcx = rax; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141612350(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    fl_l = 0; fl_r = ((rax as u8)) as u8 as u64;
    rax = (rax & !0xffu64) | (((0u8.wrapping_sub(fl_r as u8)) as u8 as u64));
    rax = (if (((fl_l as u8)) < ((fl_r as u8))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64;
    rax = (((rax as u32)).wrapping_add(0x2u32)) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141612370(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4c; const P1: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x1;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32);
    rbx = rcx;
    rdi = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { r8 = (crate::simd_ops::ru64(rcx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r8)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = ((rdi as u32)) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416123b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14268f578; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x28; const P5: u64 = 0x30;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rcx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P5), rax);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416123e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14268f578; const P1: u64 = 0x14183acc8; const P2: u64 = 0x1415242e0; const P3: u64 = 0x14160d630; const P4: u64 = 0x14160d370;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; rax = P0; crate::simd_ops::wu64(rcx, rax); rbx = rcx; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0x20u64))) as u32 as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rdi)) as u64; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 1; continue; }
            1 => { r8 = rdi; r9 = rdi; rsi = (0x2aaaaaaaaaaaaaabu64) as u64; bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = ((rdx.wrapping_sub(0x1u64)) as u32) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; r8 = r9; fl_l = rdi; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = ((rdx).wrapping_sub(rdi)) as u64; rax = rsi; let prod = (rax as i64 as i128).wrapping_mul((rdx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rcx = rdi; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; r8 = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(4u32)) as u32 as u64; rdx = rdi.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::lk_sub_u32(rbx.wrapping_add(0x20u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32 as u64; bb = 7; continue; }
            6 => { fl_l = ((rdi).wrapping_add(0x30u64)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; rax = ((rdx as u32)) as u32 as u64; r8 = r9; bb = 7; continue; }
            7 => { rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; r9 = r8; rcx = ((rcx).wrapping_add(r8)) as u64; fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 8; continue; }
            8 => { rdi = r8; bb = 9; continue; }
            9 => { r14 = 0; fl_l = 0; fl_r = 0; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdx = rdi; crate::simd_ops::wu32(rbx.wrapping_add(0x20u64), ((r14 as u32)) as u32); rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), r14); crate::simd_ops::wu32(rbx.wrapping_add(0x24u64), ((r14 as u32)) as u32); bb = 11; continue; }
            11 => { rsi = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); rdi = rsi; bb = 13; continue; }
            13 => { rbp = (crate::simd_ops::ru64(rsi)) as u64; rsi = rbp; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rdi; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rbx.wrapping_add(0x30u64), rax); bb = 16; continue; }
            15 => { crate::simd_ops::wu64(r14, rax); bb = 16; continue; }
            16 => { fl_l = rdi; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 18; continue; } bb = 17; continue; }
            17 => { crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), r14); bb = 18; continue; }
            18 => { crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), r14); rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdi = rbp; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 13; continue; } bb = 19; continue; }
            19 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 20; continue; }
            20 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x30u64), r14); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), r14); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141612550(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { crate::simd_ops::wu64(rcx.wrapping_add(P0), rdx); crate::simd_ops::wu64(rdx.wrapping_add(P1), rax); return rax; }
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rdx);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141612570(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930fa0; const P1: u64 = 0x1415c55d0; const P2: u64 = 0x1415c4f90; const P3: u64 = 0x1434264d0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rdi = 0; fl_l = 0; fl_r = 0; rbp = rdx; rsi = r9; rdx = ((rdi as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru8(r8.wrapping_add(0x48u64))) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; bb = 2; continue; }
            2 => { rcx = ((crate::simd_ops::ru8(r9)) as u32) as u32 as u64; r10 = ((crate::simd_ops::ru8(r9.wrapping_add(0x1u64))) as u32) as u32 as u64; rax = ((rcx.wrapping_add(0x80u64)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xcfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xb0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } bb = 4; continue; }
            4 => { r9 = ((crate::simd_ops::ru8(r9.wrapping_add(0x4u64))) as u32) as u32 as u64; bb = 6; continue; }
            5 => { r9 = (r9 & !0xffu64) | (((0xffu8) as u8 as u64)); bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(r8.wrapping_add(0x38u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdx); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rdi as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x58u64), ((r10 as u8)) as u8); crate::simd_ops::wu8(rsp.wrapping_add(0x60u64), ((r9 as u8)) as u8); fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xb0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 15; continue; } bb = 7; continue; }
            7 => { rax = ((crate::simd_ops::ru8(rsi.wrapping_add(0x4u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x79u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 11; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu16(rbp.wrapping_add(0x10u64), ((rdi as u16)) as u16); rbx = 0x1; bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(P0)) as u64; r8 = rsp.wrapping_add(0x40); crate::simd_ops::wu32(rsp.wrapping_add(0x90u64), ((rbx as u32)) as u32); r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rsp.wrapping_add(0x94u64), (0x1u8) as u8); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0x1u8) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x86; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 9; continue; } bb = 10; continue; }
            10 => { bb = 17; continue; }
            11 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x7bu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 17; continue; } bb = 12; continue; }
            12 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x78u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 17; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(P0)) as u64; r8 = rsp.wrapping_add(0x40); rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), ((rdi as u8)) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x90u64), ((rax as u32)) as u32); rbx = 0x1; rax = ((crate::simd_ops::ru8(rsi.wrapping_add(0x5u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x94u64), (0x1u8) as u8); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rbx as u32)) as u32); xmm3 = (((rax as u32)) as u32) as u128; xmm3 = crate::simd_ops::vcvtdq2ps(xmm3); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x4u64))) as u8 as u64; fl_r = (0x40u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 17; continue; } bb = 14; continue; }
            14 => { rdx = ((crate::simd_ops::ru16(rbp.wrapping_add(0x10u64))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rsi.wrapping_add(0x1u64))) as u32) as u32 as u64; fl_l = ((((rdx as u32)) >> ((((rax as u32)) as u32 & 31))) & 1) as u64; rdx = (((rdx as u32)) & !((1u32) << ((((rax as u32)) as u32 & 31)))) as u32 as u64; crate::simd_ops::wu16(rbp.wrapping_add(0x10u64), ((rdx as u16)) as u16); fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x5u64))) as u8 as u64; fl_r = (0x40u8) as u8 as u64; rcx = ((crate::simd_ops::ru8(rsi.wrapping_add(0x1u64))) as u32) as u32 as u64; rdi = (rdi & !0xffu64) | (((if (((fl_l as u8)) >= ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64)); rdi = (rdi & !0xffffu64) | (((crate::lift_ops::shl16((rdi as u16), ((rcx as u32) & 31))) as u16 as u64)); fl_l = ((((rdi as u16)) | ((rdx as u16)))) as u16 as u64; fl_r = fl_l; rdi = (rdi & !0xffffu64) | ((((fl_l as u16)) as u16 as u64)); crate::simd_ops::wu16(rbp.wrapping_add(0x10u64), ((rdi as u16)) as u16); bb = 17; continue; }
            15 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xe0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = ((crate::simd_ops::ru8(rsi.wrapping_add(0x5u64))) as u32) as u32 as u64; r8 = rsp.wrapping_add(0x40); rax = ((crate::simd_ops::ru8(rsi.wrapping_add(0x4u64))) as u32) as u32 as u64; rbx = 0x1; rcx = (((rcx as u32)).wrapping_shl(7u32)) as u32 as u64; rcx = ((((rcx as u32)) | ((rax as u32)))) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), ((rdi as u8)) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x90u64), (0x83u32) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x94u64), (0x1u8) as u8); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; xmm3 = (((rcx as u32)) as u32) as u128; rcx = (crate::simd_ops::ru64(P0)) as u64; xmm3 = crate::simd_ops::vcvtdq2ps(xmm3); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rbx as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(crate::simd_ops::ru32(P3))).to_bits() as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 17; continue; }
            17 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; r11 = rsp.wrapping_add(0x70); rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141612750(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141612570; const P1: u64 = 0x14160d370; const P2: u64 = 0x14160d050; const P3: u64 = 0x141612e10; const P4: u64 = 0x1416131c0; const P5: u64 = 0x14160d630;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r10 = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; rbp = rcx; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x40u64))) as u64; r14 = rdx; r11 = (crate::simd_ops::ru32(rdx.wrapping_add(0x2cu64))) as u32 as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; r8 = (crate::simd_ops::ru32(rbp.wrapping_add(0x20u64))) as u32 as u64; r8 = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; r8 = ((r8).wrapping_add(rax)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = rcx; fl_r = (crate::simd_ops::ru64(rax)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = r10; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = ((r11 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = ((rax).wrapping_add(0x30u64)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 5; continue; }
            5 => { r15 = 0; fl_l = 0; fl_r = 0; rdi = rax.wrapping_add(0x18); fl_l = rax; fl_r = r8; rdi = (if (((fl_l as u64)) == ((fl_r as u64))) { r15 } else { rdi }) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 35; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); rbx = ((r15 as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rsi); bb = 7; continue; }
            7 => { r9 = rsp.wrapping_add(0x30); crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0xb0u8) as u8); r8 = r14; crate::simd_ops::wu8(rsp.wrapping_add(0x31u64), ((rbx as u8)) as u8); rdx = rdi; crate::simd_ops::wu16(rsp.wrapping_add(0x34u64), (0x40u16) as u16); rcx = rbp; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x10; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 7; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu16(rdi.wrapping_add(0x10u64), ((r15 as u16)) as u16); rcx = r15; rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 9; continue; }
            9 => { bb = 10; continue; }
            10 => { rdx = rbx; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 22; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu32(rbx.wrapping_add(0x34u64), ((r15 as u32)) as u32); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x28u64))) as u8 as u64; fl_r = (0x90u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 22; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x2du64))) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if (((fl_l as u8)) <= ((fl_r as u8))) { bb = 22; continue; } bb = 13; continue; }
            13 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x38u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 22; continue; } bb = 14; continue; }
            14 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 21; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rax); bb = 18; continue; }
            17 => { crate::simd_ops::wu64(rcx, rax); bb = 18; continue; }
            18 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdi)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 20; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::wu64(rdi, rcx); bb = 20; continue; }
            20 => { rcx = rdx; rbx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 22; continue; }
            21 => { rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 22; continue; }
            22 => { rcx = rbx; rbx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 23; continue; }
            23 => { rax = 0x1; crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((r15 as u8)) as u8); r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu16(rsp.wrapping_add(0x34u64), ((rax as u16)) as u16); r8 = rsp.wrapping_add(0x30); crate::simd_ops::wu16(rsp.wrapping_add(0x30u64), (0xfff0u16) as u16); rdx = r14; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((r15 as u8)) as u8); rcx = rbp; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = 0; fl_l = 0; fl_r = 0; rcx = rbp; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64; rsi = r15; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 34; continue; } bb = 24; continue; }
            24 => { rbx = rax; bb = 25; continue; }
            25 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdi = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 32; continue; } bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rbx; fl_r = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 28; continue; } bb = 27; continue; }
            27 => { crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), rax); bb = 29; continue; }
            28 => { crate::simd_ops::wu64(rsi, rax); bb = 29; continue; }
            29 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 31; continue; } bb = 30; continue; }
            30 => { crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rsi); bb = 31; continue; }
            31 => { crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), r15); rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 33; continue; }
            32 => { rsi = rax; bb = 33; continue; }
            33 => { rbx = rdi; rax = rdi; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 25; continue; } bb = 34; continue; }
            34 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; bb = 35; continue; }
            35 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141612930(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x40; const P2: u64 = 0x10; const P3: u64 = 0x2c; const P4: u64 = 0x20; const P5: u64 = 0x8; const P6: u64 = 0xffffffffffffffff; const P7: u64 = 0x30; const P8: u64 = 0x14183acc8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r11 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rcx; r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; rcx = r11; r9 = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; r10 = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(r11)) as u64; fl_l = r11; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { fl_l = r8; fl_r = (crate::simd_ops::ru64(rcx)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = r9; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P5))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = ((r10 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = ((rcx).wrapping_add(0x30u64)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 6; continue; }
            6 => { fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 8; continue; }
            8 => { rax = ((rdx.wrapping_add(P6)) as u32) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; r8 = ((r8).wrapping_add(r11)) as u64; fl_l = rcx; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { r8 = ((r8).wrapping_sub(rcx)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; r8 = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(4u32)) as u32 as u64; rdx = ((rcx.wrapping_add(P7)) as u64) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::lk_sub_u32(rbx.wrapping_add(P4), (1) as u32)) as u32 as u64; bb = 11; continue; }
            11 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416129e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r10 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r8 = 0; fl_l = 0; fl_r = 0; r11 = rcx; rax = r10; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { r8 = rax; rax = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } bb = 3; continue; }
            3 => { return rax; }
            4 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; fl_l = rax; fl_r = r10; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rcx); return rax; }
            6 => { crate::simd_ops::wu64(r8.wrapping_add(P1), rcx); bb = 7; continue; }
            7 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141612a20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524340; const P1: u64 = 0x141524330; const P2: u64 = 0x1415242e0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rbx = rcx; rbp = ((rdx as u32)) as u32 as u64; rcx = 0x1; rbp = (((rbp as u32)).wrapping_shr(1u32)) as u32 as u64; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; rsi = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rbp = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rcx as u32) } else { (rbp as u32) }) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; rdx = ((rbp.wrapping_mul(2)) as u64) as u64; rdx = ((rdx).wrapping_add(rbp)) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = rdx; rdx = rcx; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 2; continue; }
            2 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 13; continue; }
            3 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 5; continue; }
            5 => { fl_l = rax; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 6; continue; }
            6 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { r9 = (0xffffffffffffffd8u64) as u64; rax = rdi.wrapping_add(0x28); r9 = ((r9).wrapping_sub(rdi)) as u64; r10 = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { r8 = rax.wrapping_sub(0x28); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rax.wrapping_sub(0x10u64), r10); crate::simd_ops::wu64(rax.wrapping_sub(0x8u64), r10); crate::simd_ops::wu16(rax, ((r10 as u16)) as u16); bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = ((r9.wrapping_add(rax)) as u64) as u64; rax = ((rax).wrapping_add(0x30u64)) as u64; xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(rcx))) as u128; crate::simd_ops::wu128(r8, (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(rcx).wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x10u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(rcx).wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x20u64), (xmm0) as u128); fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rbx, rdi); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rbp as u32)) as u32); bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141612b20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1438152b4; const P1: u64 = 0x141612b30;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    r8 = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64;
    r9 = (r9 & !0xffu64) | (((0xffu8) as u8 as u64));
    return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141612b30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141612e10;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; rsi = rdx; r10 = (crate::simd_ops::ru32(rcx.wrapping_add(0x20u64))) as u32 as u64; rdi = (((r9 as u8)) as u32) as u32 as u64; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x40u64))) as u64; rbp = ((r8 as u32)) as u32 as u64; r14 = rcx; r9 = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; rbx = (crate::simd_ops::ru32(rsi.wrapping_add(0x2cu64))) as u32 as u64; r10 = ((r10.wrapping_add(r10.wrapping_mul(2))) as u64) as u64; r10 = ((r10).wrapping_shl(4u32)) as u64; r10 = ((r10).wrapping_add(rax)) as u64; fl_l = rax; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { fl_l = rdx; fl_r = (crate::simd_ops::ru64(rax)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = r9; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = ((rax).wrapping_add(0x30u64)) as u64; fl_l = rax; fl_r = r10; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 6; continue; }
            6 => { rcx = 0; fl_l = 0; fl_r = 0; rbx = rax.wrapping_add(0x18); fl_l = rax; fl_r = r10; rbx = (if (((fl_l as u64)) == ((fl_r as u64))) { rcx } else { rbx }) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x29u64))) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } bb = 10; continue; }
            10 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 16; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x28u64))) as u8 as u64; fl_r = (0x90u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 16; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x2du64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) <= ((fl_r as u8))) { bb = 16; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x38u64))) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 16; continue; } bb = 14; continue; }
            14 => { r9 = (crate::simd_ops::ru32(rbx.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 16; continue; } bb = 15; continue; }
            15 => { xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64) as u128; r8 = rsp.wrapping_add(0x30); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; rdx = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm0 as u64)) as u64); rcx = r14; crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x0u8) as u8); crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x80u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0x1u8) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 17; continue; }
            17 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141612c40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x10; const P2: u64 = 0x28; const P3: u64 = 0x18; const P4: u64 = 0x14160d370;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r8 = rcx; rax = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = rcx; rcx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { return rax; }
            5 => { r9 = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(r8.wrapping_add(P0), r9); bb = 8; continue; }
            7 => { crate::simd_ops::wu64(rax, r9); bb = 8; continue; }
            8 => { fl_l = rcx; fl_r = (crate::simd_ops::ru64(r8.wrapping_add(P2))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(r8.wrapping_add(P2), rax); bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rdx.wrapping_add(P3), (0x0u64) as u64); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141612e10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f90; const P1: u64 = 0x14159ad70; const P2: u64 = 0x141612a20; const P3: u64 = 0x141524330; const P4: u64 = 0x14160cdc0; const P5: u64 = 0x14160d430; const P6: u64 = 0x14160d080; const P7: u64 = 0x14160d370; const P8: u64 = 0x141612b30;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rdi = rdx; r13 = ((r9 as u32)) as u32 as u64; rdx = (crate::simd_ops::ru32(rdx.wrapping_add(0x2cu64))) as u32 as u64; r12 = r8; rbx = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(P0)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; r15 = (crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32 as u64; rcx = rax; r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; r9 = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; r10 = (crate::simd_ops::ru32(rdi.wrapping_add(0x2cu64))) as u32 as u64; rdx = ((r15.wrapping_add(r15.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 3; continue; }
            3 => { fl_l = r8; fl_r = (crate::simd_ops::ru64(rcx)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = r9; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = ((r10 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = ((rcx).wrapping_add(0x30u64)) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 7; continue; }
            7 => { r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm6) as u128); fl_l = rcx; fl_r = rdx; rsi = rcx.wrapping_add(0x18); rsi = (if (((fl_l as u64)) == ((fl_r as u64))) { r14 } else { rsi }) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 23; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r8); xmm6 = 0u128; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r9); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r10 as u32)) as u32); crate::simd_ops::wu16(rsp.wrapping_add(0x48u64), ((r14 as u16)) as u16); fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 9; continue; }
            9 => { fl_l = r8; fl_r = (crate::simd_ops::ru64(rax)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 10; continue; }
            10 => { fl_l = r9; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = ((r10 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = ((rax).wrapping_add(0x30u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 13; continue; }
            13 => { fl_l = rax; fl_r = rdx; rsi = rax.wrapping_add(0x18); rsi = (if (((fl_l as u64)) == ((fl_r as u64))) { r14 } else { rsi }) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64) as u128; crate::simd_ops::wu128(rsi, (xmm6) as u128); crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), ((xmm0 as u64)) as u64); bb = 23; continue; }
            15 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x24u64))) as u32 as u64; fl_l = r15; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = rbx.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 40; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x24u64))) as u32 as u64; fl_l = r15; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 40; continue; } bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x20u64), ((rax as u32)) as u32); fl_l = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x18u64)))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), r14); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), r14); crate::simd_ops::wu16(rcx.wrapping_add(0x28u64), ((r14 as u16)) as u16); bb = 20; continue; }
            20 => { rsi = r14; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsi = rcx.wrapping_add(0x18); xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64) as u128; crate::simd_ops::wu128(rsi, (xmm6) as u128); crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); bb = 22; continue; }
            22 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 40; continue; } bb = 23; continue; }
            23 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; rcx = rbx; rax = (crate::simd_ops::ru64(rbx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x70u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 25; continue; } bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x70u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xa; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 26; continue; } bb = 25; continue; }
            25 => { fl_l = (crate::simd_ops::ru8(r12)) as u8 as u64; fl_r = (0xb0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 40; continue; } bb = 26; continue; }
            26 => { rdx = 0x40; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 40; continue; } bb = 27; continue; }
            27 => { r8 = rbx; rdx = rdi; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 40; continue; } bb = 28; continue; }
            28 => { r8 = ((r13 as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rbp); rdx = r12; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 38; continue; } bb = 29; continue; }
            29 => { rax = ((crate::simd_ops::ru8(r12)) as u32) as u32 as u64; rbp = r12.wrapping_add(0x5); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 32; continue; } bb = 30; continue; }
            30 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x90u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 41; continue; } bb = 31; continue; }
            31 => { fl_l = (crate::simd_ops::ru8(rbp)) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 41; continue; } bb = 32; continue; }
            32 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 38; continue; } bb = 33; continue; }
            33 => { r15 = ((crate::simd_ops::ru8(rsp.wrapping_add(0xb0u64))) as u32) as u32 as u64; bb = 34; continue; }
            34 => { fl_l = ((r15 as u8)) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 36; continue; } bb = 35; continue; }
            35 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 37; continue; } bb = 36; continue; }
            36 => { rdx = rbx; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 41; continue; } bb = 37; continue; }
            37 => { rbx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 34; continue; } bb = 38; continue; }
            38 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 39; continue; }
            39 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; bb = 40; continue; }
            40 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; r11 = rsp.wrapping_add(0x60); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x48u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            41 => { rax = ((crate::simd_ops::ru8(r12)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x90u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 43; continue; } bb = 42; continue; }
            42 => { fl_l = (crate::simd_ops::ru8(r12.wrapping_add(0x5u64))) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if (((fl_l as u8)) > ((fl_r as u8))) { bb = 50; continue; } bb = 43; continue; }
            43 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 50; continue; } bb = 44; continue; }
            44 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x90u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 46; continue; } bb = 45; continue; }
            45 => { fl_l = (crate::simd_ops::ru8(rbp)) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 50; continue; } bb = 46; continue; }
            46 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x28u64))) as u8 as u64; fl_r = (0xb0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 50; continue; } bb = 47; continue; }
            47 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x2cu64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x7bu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 49; continue; } bb = 48; continue; }
            48 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x78u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 50; continue; } bb = 49; continue; }
            49 => { r9 = ((crate::simd_ops::ru8(rdi.wrapping_add(0x29u64))) as u32) as u32 as u64; r8 = ((r13 as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 50; continue; }
            50 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 54; continue; } bb = 51; continue; }
            51 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; bb = 52; continue; }
            52 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x34u64))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 54; continue; } bb = 53; continue; }
            53 => { r14 = rax; rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 52; continue; } bb = 54; continue; }
            54 => { crate::simd_ops::wu64(rdi, rax); fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 56; continue; } bb = 55; continue; }
            55 => { crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rdi); bb = 57; continue; }
            56 => { crate::simd_ops::wu64(r14, rdi); bb = 57; continue; }
            57 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 39; continue; } bb = 58; continue; }
            58 => { crate::simd_ops::wu64(rsi, rdi); bb = 39; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141613100(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x20; const P2: u64 = 0x18; const P3: u64 = 0x14; const P4: u64 = 0x141612e10; const P5: u64 = 0x141612370; const P6: u64 = 0x1415242e0; const P7: u64 = 0x1416131c0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rbp = ((r8 as u32)) as u32 as u64; r14 = rdx; rsi = rcx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r15); r15 = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rcx = ((r14.wrapping_add(P0)) as u64) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(r14, r15); rax = r15; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rcx, rax); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { fl_l = ((((rax as u32)) >> (30u32)) & 1) as u64; if ((fl_l as u32) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsi)) as u64; r8 = rbx; rdx = r14; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { r9 = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; r8 = ((rbx.wrapping_add(P0)) as u64) as u64; rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = rsi; crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x1u8) as u8); crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((r15 as u8)) as u8); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(r14.wrapping_add(P0))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 8; continue; }
            8 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 9; continue; }
            9 => { rdx = ((rbp as u32)) as u32 as u64; rcx = rsi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P7, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416131c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14160d690; const P1: u64 = 0x141612570; const P2: u64 = 0x14160d370; const P3: u64 = 0x14160d630;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; rbp = rcx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x20u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r13); r13 = ((rdx as u32)) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; r8 = ((r8).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 35; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r14); r14 = rbx.wrapping_add(0x18); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); bb = 2; continue; }
            2 => { rdi = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; r12 = 0; fl_l = 0; fl_r = 0; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 3; continue; }
            3 => { rcx = ((crate::simd_ops::ru8(rdi.wrapping_add(0x29u64))) as u32) as u32 as u64; rsi = rdi.wrapping_add(0x28); rax = ((crate::simd_ops::ru16(r14.wrapping_add(0x10u64))) as u32) as u32 as u64; r15 = rdi; fl_l = ((((rax as u32)) >> ((((rcx as u32)) as u32 & 31))) & 1) as u64; if ((fl_l as u32) == 0) { bb = 8; continue; } bb = 4; continue; }
            4 => { rax = ((crate::simd_ops::ru8(rsi)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } bb = 5; continue; }
            5 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x90u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x5u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 7; continue; }
            7 => { r8 = (r8 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 9; continue; }
            8 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 9; continue; }
            9 => { r9 = r14; rdx = ((r13 as u32)) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 31; continue; } bb = 10; continue; }
            10 => { rax = ((crate::simd_ops::ru8(rsi)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x90u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 13; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x5u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) <= ((fl_r as u8))) { bb = 15; continue; } bb = 12; continue; }
            12 => { bb = 19; continue; }
            13 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 19; continue; } bb = 14; continue; }
            14 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x90u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 16; continue; } bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x2du64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; bb = 17; continue; }
            16 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xfeu8) as u8 as u64; bb = 17; continue; }
            17 => { if (((fl_l as u8)) == ((fl_r as u8))) { bb = 19; continue; } bb = 18; continue; }
            18 => { r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; r9 = rsi; rdx = r14; rcx = rbp; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rdi; fl_r = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 20; continue; }
            20 => { crate::simd_ops::wu64(r14.wrapping_add(0x8u64), rax); bb = 22; continue; }
            21 => { crate::simd_ops::wu64(r12, rax); bb = 22; continue; }
            22 => { fl_l = rdi; fl_r = (crate::simd_ops::ru64(r14)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 24; continue; } bb = 23; continue; }
            23 => { crate::simd_ops::wu64(r14, r12); bb = 24; continue; }
            24 => { rdi = rax; rax = ((crate::simd_ops::ru8(rsi)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 28; continue; } bb = 25; continue; }
            25 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x90u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 27; continue; } bb = 26; continue; }
            26 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x5u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 28; continue; } bb = 27; continue; }
            27 => { rcx = r15; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 32; continue; }
            28 => { rax = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(r15, (0x0u64) as u64); fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 30; continue; } bb = 29; continue; }
            29 => { crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), r15); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), r15); bb = 32; continue; }
            30 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rax, r15); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), r15); bb = 32; continue; }
            31 => { r12 = rdi; rdi = (crate::simd_ops::ru64(rdi)) as u64; bb = 32; continue; }
            32 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 33; continue; }
            33 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x20u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x30u64)) as u64; r14 = ((r14).wrapping_add(0x30u64)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbp.wrapping_add(0x18u64)))) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 34; continue; }
            34 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 35; continue; }
            35 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64; rsi = 0; fl_l = 0; fl_r = 0; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 47; continue; } bb = 36; continue; }
            36 => { rbx = rax; bb = 37; continue; }
            37 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x6cu64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 45; continue; } bb = 38; continue; }
            38 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x94u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 45; continue; } bb = 39; continue; }
            39 => { rdi = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rbx; fl_r = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 41; continue; } bb = 40; continue; }
            40 => { crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), rax); bb = 42; continue; }
            41 => { crate::simd_ops::wu64(rsi, rax); bb = 42; continue; }
            42 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 44; continue; } bb = 43; continue; }
            43 => { crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rsi); bb = 44; continue; }
            44 => { crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), (0x0u64) as u64); rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 46; continue; }
            45 => { rdi = (crate::simd_ops::ru64(rax)) as u64; rsi = rax; bb = 46; continue; }
            46 => { rbx = rdi; rax = rdi; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 37; continue; } bb = 47; continue; }
            47 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416133c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1426452b8; const P2: u64 = 0x38; const P3: u64 = 0x40; const P4: u64 = 0x48; const P5: u64 = 0x50;
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
    rcx = ((rcx).wrapping_add(0x10u64)) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    crate::simd_ops::wu64(rbx.wrapping_add(P2), rdi);
    crate::simd_ops::wu32(rbx.wrapping_add(P3), ((rdi as u32)) as u32);
    crate::simd_ops::wu64(rbx.wrapping_add(P4), rdi);
    crate::simd_ops::wu64(rbx.wrapping_add(P5), rdi);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416135f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14158f360; const P1: u64 = 0x14158f430; const P2: u64 = 0x141524320; const P3: u64 = 0x1426452b0; const P4: u64 = 0x1426452a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbp = ((r8 as u32)) as u32 as u64; r15 = ((rdx as u32)) as u32 as u64; rdi = rcx; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 17; continue; } bb = 1; continue; }
            1 => { rax = ((rbp.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 17; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rcx.wrapping_add(0x3cu64), ((rbp as u32)) as u32); rax = rbp.wrapping_sub(0x1); rax = (!(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); crate::simd_ops::wu64(rcx.wrapping_add(0x48u64), rax); rax = ((rbp.wrapping_sub(0x18u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x40u64), ((rax as u32)) as u32); rcx = ((rcx).wrapping_add(0x38u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = 0x9; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            4 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x38u64))) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r13); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r13 = 0; fl_l = 0; fl_r = 0; fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 16; continue; } bb = 5; continue; }
            5 => { r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0x3cu64))) as u32 as u64; rcx = ((r13.wrapping_add(0x12u64)) as u32) as u32 as u64; rdx = ((r8 as u32)) as u32 as u64; rdx = ((((rdx as u32)) as i32).wrapping_mul(((r15 as u32)) as i32) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r12 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = ((r13.wrapping_add(0x34u64)) as u32) as u32 as u64; bb = 7; continue; }
            7 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            8 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rsi); rsi = ((r15.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 15; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru32(rdi.wrapping_add(0x40u64))) as u32 as u64; rcx = rdi.wrapping_add(0x10); rax = ((rsi as u32)) as u32 as u64; xmm0 = 0u128; rax = ((((rax as u32)) as i32).wrapping_mul(((rbp as u32)) as i32) as u32) as u32 as u64; rax = ((rax).wrapping_add(r12)) as u64; rbx = ((rbx).wrapping_add(rax)) as u64; rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu128(rbx, (xmm0) as u128); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x40u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rbx, r13); bb = 13; continue; }
            12 => { crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rbx); bb = 13; continue; }
            13 => { rcx = rdi.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x1; rsi = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) >= 0) { bb = 10; continue; } bb = 14; continue; }
            14 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 15; continue; }
            15 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; bb = 16; continue; }
            16 => { crate::simd_ops::wu32(rdi.wrapping_add(0x50u64), ((r15 as u32)) as u32); rax = 0x1; crate::simd_ops::wu32(rdi.wrapping_add(0x54u64), ((r13 as u32)) as u32); bb = 7; continue; }
            17 => { rax = 0x1f; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141613740(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x14158f3e0; const P2: u64 = 0x14; const P3: u64 = 0x14158f430;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rdx = 0; fl_l = 0; fl_r = 0; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rax.wrapping_add(P2), ((rdx as u32)) as u32); rdx = (tmp_xchg) as u32 as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141613770(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x54; const P2: u64 = 0x8; const P3: u64 = 0x48; const P4: u64 = 0x1415242e0; const P5: u64 = 0x50; const P6: u64 = 0x10; const P7: u64 = 0x1426452a8; const P8: u64 = 0x38; const P9: u64 = 0x14158f3b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rcx = ((rcx).wrapping_add(0x10u64)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P2), rax); bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; rdx = (((rdx) & (rcx))) as u64; rcx = 0x12; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(P1), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 1; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; rcx = 0x12; rdx = (((rdx) & (crate::simd_ops::ru64(rbx.wrapping_add(P2))))) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbx.wrapping_add(P5), (0x0u32) as u32); bb = 6; continue; }
            6 => { rcx = ((rbx.wrapping_add(P6)) as u64) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P2), (0x0u64) as u64); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P8))) as u32 as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu32(rbx.wrapping_add(P8), (0x0u32) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141613820(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xe; const P1: u64 = 0x141651900;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdx = 0x2;
    rbx = rcx;
    r8 = ((rdx.wrapping_add(P0)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141613840(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141651a60;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rdx = 0x10;
    return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141613850(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1416517f0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    r8 = 0x10;
    return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141613870(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1416518c0; const P1: u64 = 0x1416519a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbp);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, r14);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rsi = rdx;
    rbp = r9;
    rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64;
    r14 = ((r8 as u32)) as u32 as u64;
    rdi = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rdx = rbp; r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x50u64))) as u32 as u64; rcx = ((r14 as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(rsi, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    r14 = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416138e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931d98; const P1: u64 = 0x1426452b0; const P2: u64 = 0x8; const P3: u64 = 0x4; const P4: u64 = 0x10; const P5: u64 = 0x1426452a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = rcx; rdi = rdx; rcx = P0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; rax = rdx; rbx = (crate::simd_ops::ru64(rbx)) as u64; xmm0 = (crate::simd_ops::ru128(rdi)) as u128; rax = ((rax).wrapping_shr(32u32)) as u64; rax = (((rax as u32)).wrapping_add((rcx as u32))) as u32 as u64; rax = (((rax as u32)).wrapping_add((rdx as u32))) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rdx = 0; fl_l = 0; fl_r = 0; let d = ((r8 as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rbx = (crate::simd_ops::ru64(rbx.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x24u64))) as u32 as u64; rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 7; continue; }
            7 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { rcx = P0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141613980(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x14159a6d0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = 0; fl_l = 0; fl_r = 0; rbx = rcx; crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rdi as u32)) as u32); fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; fl_r = 0xb; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rcx.wrapping_add(P1), (0xbu32) as u32); bb = 6; continue; }
            2 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xb; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rdx = 0xb; rdx = (((rdx as u32)).wrapping_sub((rax as u32))) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { crate::simd_ops::wu32(rbx.wrapping_add(P1), (0xbu32) as u32); bb = 6; continue; }
            6 => { rax = ((rdi as u32)) as u32 as u64; bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = ((rax as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu64(rcx.wrapping_add(rdx.wrapping_mul(8)), rdi); fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 7; continue; } bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416139f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931d98; const P1: u64 = 0x1426452b0; const P2: u64 = 0x8; const P3: u64 = 0x4; const P4: u64 = 0x10; const P5: u64 = 0x1426452a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rbx = rdx; rcx = P0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); r9 = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rax = r8; rax = ((rax).wrapping_shr(32u32)) as u64; rax = (((rax as u32)).wrapping_add((r8 as u32))) as u32 as u64; rax = (((rax as u32)).wrapping_add((rcx as u32))) as u32 as u64; let d = ((r9 as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rdi)) as u64; r9 = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(r9)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P3))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rdx = rax; rax = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 7; continue; }
            7 => { bb = 12; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rdx.wrapping_add(P4), rcx); bb = 11; continue; }
            10 => { crate::simd_ops::wu64(r9, rcx); bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::lk_sub_u32(rdi.wrapping_add(P4), (1) as u32)) as u32 as u64; bb = 12; continue; }
            12 => { rcx = P0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141613aa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14268f590; const P1: u64 = 0x141524330; const P2: u64 = 0x1415242e0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x38u64)) as u64; rax = 0; fl_l = 0; fl_r = 0; r8 = P0; rdi = rcx; rcx = ((rax as u32)) as u32 as u64; bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; r8 = ((r8).wrapping_add(0x4u64)) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1b; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rax = 0x2; rsp = ((rsp).wrapping_add(0x38u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 3; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbp); rdx = ((rbx.wrapping_mul(8)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rsi); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r12); rsi = rbx; r12 = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r13); r13 = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); r15 = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(rdi, rax); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rdi)) as u64; rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { r8 = 0; fl_l = 0; fl_r = 0; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((r8 as u32)) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rdi, r13); rax = 0x2; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((r15 as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((r12 as u32)) as u32); bb = 26; continue; }
            9 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { fl_l = rdx; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rdx = rdi; rcx = rdi.wrapping_add(0xc); rax = ((rbx as u32)) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdx, rbp); rbp = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu32(rcx, ((rbx as u32)) as u32); bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 16; continue; } bb = 14; continue; }
            14 => { rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rdx, r8); rbx = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; bb = 16; continue; }
            16 => { fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 17; continue; }
            17 => { fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 23; continue; } bb = 18; continue; }
            18 => { r10 = r13; r11 = r15; bb = 19; continue; }
            19 => { r9 = (crate::simd_ops::ru64(r10)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 20; continue; }
            20 => { bb = 21; continue; }
            21 => { rdx = (crate::simd_ops::ru64(r9)) as u64; rcx = (crate::simd_ops::ru64(r9.wrapping_add(0x8u64))) as u64; rax = rdx; r8 = (crate::simd_ops::ru64(r9.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_shr(32u32)) as u64; rax = (((rax as u32)).wrapping_add((rcx as u32))) as u32 as u64; rax = (((rax as u32)).wrapping_add((rdx as u32))) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; let d = ((rbp as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = ((rdx.wrapping_mul(8)) as u64) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(rax))) as u64; crate::simd_ops::wu64(r9.wrapping_add(0x10u64), rcx); rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(rax), r9); r9 = r8; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 21; continue; } bb = 22; continue; }
            22 => { r10 = ((r10).wrapping_add(0x8u64)) as u64; fl_l = r11; fl_r = (0x1u64) as u64; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 19; continue; } bb = 23; continue; }
            23 => { fl_l = r13; fl_r = r13; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 24; continue; }
            24 => { rdx = r13; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 25; continue; }
            25 => { rax = 0x1; bb = 26; continue; }
            26 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rsp = ((rsp).wrapping_add(0x38u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141613c30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931d98; const P1: u64 = 0x1426452b0; const P2: u64 = 0x143426890; const P3: u64 = 0x141613aa0; const P4: u64 = 0x1426452a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = rcx; rsi = r8; rcx = P0; rdi = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rax = rdx; xmm0 = (crate::simd_ops::ru128(rdi)) as u128; rax = ((rax).wrapping_shr(32u32)) as u64; rax = (((rax as u32)).wrapping_add((rcx as u32))) as u32 as u64; rax = (((rax as u32)).wrapping_add((rdx as u32))) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rdx = 0; fl_l = 0; fl_r = 0; let d = ((r9 as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x24u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 7; continue; }
            7 => { rdx = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 9; continue; }
            9 => { xmm0 = (crate::simd_ops::ru128(rdi)) as u128; rdi = rbx.wrapping_add(0x10); crate::simd_ops::wu128(rsi, (xmm0) as u128); rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; xmm1 = 0u128; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rdx) as i64) as f32).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P2)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rdx = (crate::simd_ops::ru64(rsi)) as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rax = rdx; rax = ((rax).wrapping_shr(32u32)) as u64; rax = (((rax as u32)).wrapping_add((rcx as u32))) as u32 as u64; rax = (((rax as u32)).wrapping_add((rdx as u32))) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; let d = ((r8 as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx)) as u64; r8 = ((rdx as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), rdx); rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rax.wrapping_add(r8.wrapping_mul(8)), rsi); fl_l = (crate::simd_ops::lk_add_u32(rdi, (1) as u32)) as u32 as u64; bb = 14; continue; }
            14 => { rcx = P0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141613d50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524340; const P1: u64 = 0x141524330; const P2: u64 = 0x1415242e0; const P3: u64 = 0x143930f44; const P4: u64 = 0x1415c2bf0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r15 = rdx; rbx = rcx; r12 = 0x1; r9 = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rax = ((rax).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rcx, rax); rbp = ((r9 as u32)) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 25; continue; } bb = 1; continue; }
            1 => { r14 = r8.wrapping_add(0x210); fl_l = ((rbp as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r14.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 15; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(r14)) as u64; rdx = ((r9 as u32)) as u32 as u64; rsi = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rdx = ((rdx).wrapping_shl(3u32)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { r8 = rdx; rdx = rax; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 4; continue; }
            4 => { bb = 13; continue; }
            5 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(r14)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 7; continue; }
            7 => { fl_l = rax; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 8; continue; }
            8 => { rdx = 0; fl_l = 0; fl_r = 0; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(r14)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(rdx.wrapping_mul(8)), rcx); rdx = ((rdx).wrapping_add(1)) as u64; fl_l = rdx; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 10; continue; } bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(r14)) as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { crate::simd_ops::wu64(r14, rdi); rax = (((r12 as u8)) as u32) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0xcu64), ((rbp as u32)) as u32); bb = 13; continue; }
            13 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rcx = 0x34; rcx = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (r12 as u32) } else { (rcx as u32) }) as u32 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = ((rcx as u32)) as u32 as u64; bb = 26; continue; }
            15 => { rsi = 0; fl_l = 0; fl_r = 0; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 25; continue; } bb = 16; continue; }
            16 => { bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rbx)) as u64; r12 = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = ((rax).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu64(rbx, rax); r13 = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(rbx, rax); r11 = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(rbx, rax); rcx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rax = ((rax).wrapping_add(1)) as u64; r9 = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu64(rbx, rax); r9 = ((((r9 as u32)) & (0x7fu32))) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if (((((fl_l as u8)) & ((fl_r as u8))) as i8) >= 0) { bb = 20; continue; } bb = 18; continue; }
            18 => { bb = 19; continue; }
            19 => { r8 = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rcx = ((r9 as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(7u32)) as u32 as u64; rax = ((rax).wrapping_add(1)) as u64; rdx = ((r8 as u32)) as u32 as u64; crate::simd_ops::wu64(rbx, rax); rdx = ((((rdx as u32)) & (0x7fu32))) as u32 as u64; r9 = ((rdx as u32)) as u32 as u64; r9 = ((((r9 as u32)) | ((rcx as u32)))) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if (((((fl_l as u8)) & ((fl_r as u8))) as i8) < 0) { bb = 19; continue; } bb = 20; continue; }
            20 => { r8 = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = ((rax).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu64(rbx, rax); rdx = rax.wrapping_add(0x1); r10 = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; crate::simd_ops::wu64(rbx, rdx); rdi = ((crate::simd_ops::ru16(rdx)) as u32) as u32 as u64; rdx = ((rdx).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rbx, rdx); fl_l = ((r11 as u8)) as u8 as u64; fl_r = (0x6u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 22; continue; } bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::ru32(P3)) as u32 as u64; fl_r = 0x1; rax = 0x2; rcx = 0x5; rax = (if (((fl_l as u32)) == ((fl_r as u32))) { (rcx as u32) } else { (rax as u32) }) as u32 as u64; bb = 23; continue; }
            22 => { rax = (((r11 as u8)) as u32) as u32 as u64; bb = 23; continue; }
            23 => { rcx = rsp.wrapping_add(0xa0); crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), (0x0u64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rcx); rcx = r14; crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((rdi as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdx); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((r10 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((r8 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r9 as u32)) as u32); r9 = ((r12 as u32)) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((rax as u8)) as u8); r12 = 0x1; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rdx = ((r12 as u32)) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((r13 as u8)) as u8); r8 = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 26; continue; } bb = 24; continue; }
            24 => { rax = ((((rdi as u32)) as i32).wrapping_mul((0xfffffff4u32) as i32) as u32) as u32 as u64; rcx = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(2u32)) as u64; rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::lk_add_u64(rbx, (rcx) as u64); crate::simd_ops::lk_add_u32(r15, ((rax as u32)) as u32); fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 17; continue; } bb = 25; continue; }
            25 => { rax = ((r12 as u32)) as u32 as u64; bb = 26; continue; }
            26 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141613f80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14268f600; const P1: u64 = 0x1415c2b30; const P2: u64 = 0x30; const P3: u64 = 0x1415242e0; const P4: u64 = 0x8; const P5: u64 = 0x10; const P6: u64 = 0x210; const P7: u64 = 0x1415bd160; const P8: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = P0; rdi = rcx; crate::simd_ops::wu64(rcx, rax); r14 = ((rdx as u32)) as u32 as u64; rcx = ((rcx).wrapping_add(0x210u64)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r15 = 0; fl_l = 0; fl_r = 0; rbx = ((rdi.wrapping_add(P2)) as u64) as u64; rsi = 0x13; bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rbx, r15); crate::simd_ops::wu64(rbx.wrapping_add(P4), r15); crate::simd_ops::wu32(rbx.wrapping_add(P5), (0x1u32) as u32); rbx = ((rbx).wrapping_add(0x18u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 4; continue; }
            4 => { rcx = ((rdi.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); fl_l = ((r14 as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = 0x220; rcx = rdi; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = rdi; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141614030(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524330; const P1: u64 = 0x1415bd140; const P2: u64 = 0x14268f600; const P3: u64 = 0x30; const P4: u64 = 0x8; const P5: u64 = 0x10; const P6: u64 = 0x18; const P7: u64 = 0x210; const P8: u64 = 0x3c; const P9: u64 = 0x218; const P10: u64 = 0x1f8; const P11: u64 = 0xfffffffffffffff4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = 0x220; rcx = 0x2; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = P2; rdx = 0x13; crate::simd_ops::wu64(rbx, rax); rcx = ((rbx.wrapping_add(P3)) as u64) as u64; rax = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu64(rcx.wrapping_add(P4), rax); crate::simd_ops::wu32(rcx.wrapping_add(P5), (0x1u32) as u32); rcx = ((rcx.wrapping_add(P6)) as u64) as u64; fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rbx.wrapping_add(P7), rax); rcx = ((rbx.wrapping_add(P8)) as u64) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P9), rax); rdx = rax; bb = 4; continue; }
            4 => { crate::simd_ops::wu8(rbx.wrapping_add(rdx).wrapping_add(P10), (0xffu8) as u8); rdx = ((rdx).wrapping_add(1)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P11), rax); crate::simd_ops::wu64(rcx.wrapping_sub(0x4u64), rax); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), (0x1u32) as u32); rcx = rcx.wrapping_add(0x18); fl_l = rdx; fl_r = (0x13u64) as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 4; continue; } bb = 5; continue; }
            5 => { rax = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416140e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415bd1d0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rdx = 0x6;
    return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416140f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434264e0; const P1: u64 = 0x1434268d0; const P2: u64 = 0x1415242e0; const P3: u64 = 0x141524330; const P4: u64 = 0x14183acbc; const P5: u64 = 0x141540c30; const P6: u64 = 0x141613d50;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu32(rsp.wrapping_add(0x18u64), ((r8 as u32)) as u32); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = ((rsp).wrapping_sub(0x48u64)) as u64; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x4u64))) as u32) as u32 as u64; r12 = ((r8 as u32)) as u32 as u64; fl_l = 0; fl_r = ((rax as u8)) as u8 as u64; rax = (rax & !0xffu64) | (((0u8.wrapping_sub(fl_r as u8)) as u8 as u64)); r8 = rdx.wrapping_add(0x6); rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x20bu64))) as u32) as u32 as u64; rdi = rcx; r9 = (r9 & !0xffu64) | (((if (((fl_l as u8)) < ((fl_r as u8))) { 0xffu8 } else { 0u8 }) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xfdu8))) as u8 as u64)); r9 = (r9 & !0xffu64) | ((((((r9 as u8)) & (0x2u8))) as u8 as u64)); r9 = (r9 & !0xffu64) | ((((((r9 as u8)) | ((rax as u8)))) as u8 as u64)); crate::simd_ops::wu8(rcx.wrapping_add(0x20bu64), ((r9 as u8)) as u8); fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x5u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rdx = r8; rax = (rax & !0xffu64) | (((if (((fl_l as u8)) != ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (r9 & !0xffu64) | ((((((r9 as u8)) & (0xfeu8))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) | ((r9 as u8)))) as u8 as u64)); crate::simd_ops::wu8(rcx.wrapping_add(0x20bu64), ((rax as u8)) as u8); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm0 = ((crate::simd_ops::ru32(r8)) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P0))).to_bits() as u128); crate::simd_ops::wu32(rcx.wrapping_add(0x18u64), ((xmm0 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(r8.wrapping_add(0x4u64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P0))).to_bits() as u128); crate::simd_ops::wu32(rcx.wrapping_add(0x1cu64), ((xmm0 as u32)) as u32); rax = (crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x20u64), ((rax as u32)) as u32); rcx = r8.wrapping_add(0x10); rax = (crate::simd_ops::ru32(r8.wrapping_add(0xcu64))) as u32 as u64; r8 = rcx.wrapping_add(0x4); crate::simd_ops::wu32(rdi.wrapping_add(0x24u64), ((rax as u32)) as u32); rdx = r8; rax = (crate::simd_ops::ru32(rcx)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x28u64), ((rax as u32)) as u32); bb = 2; continue; }
            2 => { rax = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = rdx.wrapping_add(0x14); crate::simd_ops::wu8(rdi.wrapping_add(0x1f8u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x1u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x1f9u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x2u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x1fau64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x3u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x1fbu64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x4u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x1fcu64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x5u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x1fdu64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x6u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x1feu64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x7u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x1ffu64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x8u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x200u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x9u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x201u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0xau64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x202u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0xbu64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x203u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0xcu64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x204u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0xdu64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x205u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x206u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0xfu64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x207u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x10u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x208u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x11u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x209u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x12u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x20au64), ((rax as u8)) as u8); rax = 0; fl_l = 0; fl_r = 0; r13 = ((crate::simd_ops::ru8(rdx.wrapping_add(0x13u64))) as u32) as u32 as u64; rsi = ((rax as u32)) as u32 as u64; fl_l = ((r13 as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 31; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbx); rdx = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); rbx = rdi.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r15); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); xmm6 = ((crate::simd_ops::ru32(P1)) as u32) as u128; bb = 4; continue; }
            4 => { r14 = ((crate::simd_ops::ru16(rdx.wrapping_add(0x1u64))) as u32) as u32 as u64; r15 = rdx.wrapping_add(0x3); rdx = (crate::simd_ops::ru64(rbx)) as u64; rbp = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), r15); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), (0x0u64) as u64); crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), (0x1u32) as u32); fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 7; continue; }
            7 => { fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 30; continue; } bb = 8; continue; }
            8 => { rax = ((r14.wrapping_add(0x2u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), (0x1u32) as u32); rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); rdx = (((rdx as u32)).wrapping_shl(2u32)) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx, rax); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 9; continue; }
            9 => { r8 = ((r14.wrapping_add(r14.wrapping_mul(2))) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rbp as u32)) as u32); r8 = (((r8 as u32)).wrapping_shl(2u32)) as u32 as u64; rcx = ((rcx).wrapping_add(0xcu64)) as u64; rdx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu32(rax.wrapping_add(0x8u64), (0x9u32) as u32); rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu32(rax, (0xff7fffffu32) as u32); rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x8u64), (0x9u32) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(4)), (0x7f7fffffu32) as u32); rcx = rbx; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; r9 = (crate::simd_ops::ru64(rbx)) as u64; rax = ((rdx.wrapping_sub(0x2u64)) as u32) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((rdx.wrapping_sub(0x1u64)) as u32) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru32(r9.wrapping_add(r8.wrapping_mul(4)).wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(0x4u64), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x1f8u64))) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 10; continue; }
            10 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x1f9u64))) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 11; continue; }
            11 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x1fau64))) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 12; continue; }
            12 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x1ffu64))) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 13; continue; }
            13 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x202u64))) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 14; continue; }
            14 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x205u64))) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x208u64))) as u32) as u32 as u64; rbp = 0x1; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 19; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 23; continue; } bb = 17; continue; }
            17 => { crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), (0x4u32) as u32); bb = 18; continue; }
            18 => { rbp = 0x1; bb = 19; continue; }
            19 => { rax = ((((r14 as u32)) as i32).wrapping_mul((0xfffffff4u32) as i32) as u32) as u32 as u64; rcx = ((r14.wrapping_add(r14.wrapping_mul(2))) as u64) as u64; rcx = ((r15.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; rbx = ((rbx).wrapping_add(0x18u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rcx); rdx = rcx; r12 = (((r12 as u32)).wrapping_add((rax as u32))) as u32 as u64; rax = 0x0; crate::simd_ops::wu32(rsp.wrapping_add(0x80u64), ((r12 as u32)) as u32); fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 4; continue; } bb = 20; continue; }
            20 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 28; continue; } bb = 21; continue; }
            21 => { r8 = rdi; rdx = rsp.wrapping_add(0x80); rcx = rsp.wrapping_add(0x78); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 22; continue; }
            22 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x48u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            23 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 18; continue; } bb = 24; continue; }
            24 => { rdx = 0; fl_l = 0; fl_r = 0; rbp = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 27; continue; } bb = 25; continue; }
            25 => { bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x4u64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm6 as u32)).to_bits() as u128); crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x4u64), ((xmm0 as u32)) as u32); fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 26; continue; } bb = 27; continue; }
            27 => { rcx = rbx; crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), (0x0u32) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 19; continue; }
            28 => { rax = ((rbp as u32)) as u32 as u64; bb = 22; continue; }
            29 => { crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), (0x0u32) as u32); rax = 0x34; bb = 22; continue; }
            30 => { rax = 0x1f; bb = 22; continue; }
            31 => { rax = 0x2; rsp = ((rsp).wrapping_add(0x48u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416144d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x11; const P2: u64 = 0x14; const P3: u64 = 0x10; const P4: u64 = 0x1416144d0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; rsi = rdx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rdi = rcx; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(P1))) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P0))) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; xmm0 = ((crate::simd_ops::ru32(rsi.wrapping_add(P2))) as u32) as u128; xmm3 = ((crate::simd_ops::ru32(rsi.wrapping_add(P3))) as u32) as u128; r8 = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64; crate::simd_ops::wu8(rbx.wrapping_add(P0), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x64u64), (0x4u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm0 as u32)) as u32); rdx = (crate::simd_ops::ru32(rax.wrapping_add(P3))) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rdx as u32)) as u32); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(P3))) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; rbx = ((rbx).wrapping_add(0x10u64)) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 5; continue; }
            5 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(P3))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { xmm0 = (crate::simd_ops::ru128(rsi)) as u128; rdx = rsp.wrapping_add(0x30); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xfeu8))) as u8 as u64)); xmm1 = ((crate::simd_ops::ru64(rsi.wrapping_add(P3))) as u64) as u128; rcx = rdi; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); crate::simd_ops::wu8(rdi.wrapping_add(P3), ((rax as u8)) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), ((xmm1 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141614590(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141624420; const P1: u64 = 0x1415fa0e0; const P2: u64 = 0x141616430; const P3: u64 = 0x1416164e0; const P4: u64 = 0x141615b30; const P5: u64 = 0x141616590; const P6: u64 = 0x1416158d0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 33>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu8(rsp.wrapping_add(0x10u64), ((rdx as u8)) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rbp = ((rsp.wrapping_sub(0x3fu64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0xe8u64)) as u64; rbx = (crate::simd_ops::ru64(r8)) as u64; rdi = (((rdx as u8)) as u32) as u32 as u64; rdx = (rcx) as u64; crate::simd_ops::wu8(rbp.wrapping_sub(0x49u64), (0xffu8) as u8); rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rbp.wrapping_sub(0x41u64), (0xffu8) as u8); crate::simd_ops::wu64(rbp.wrapping_sub(0x61u64), (rcx) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x59u64), ((rcx as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x51u64), ((rcx as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), (rcx) as u64); fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { r14 = ((r8.wrapping_add(0x8u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x77u64), (r14) as u64); fl_l = (crate::simd_ops::ru32(r14)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x18u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru64(r14.wrapping_add(0x20u64))) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { r14 = ((r8.wrapping_add(0x8u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x77u64), (r14) as u64); bb = 7; continue; }
            7 => { fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 221; continue; } bb = 8; continue; }
            8 => { rax = (((crate::simd_ops::ru32(rdx.wrapping_add(0x20u64))) as i32 as i64) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x7u64), (rax) as u64); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 223; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rsp.wrapping_add(0xe0u64), (rsi) as u64); rax = (rcx) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xd8u64), (r12) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0xd0u64), (r13) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), (r15) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), (rcx) as u64); bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru32(r14)) as u32 as u64; fl_r = (0x0u32) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; r15 = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x7fu64), (r15) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x61u64), (r15) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), (rdx) as u64); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 18; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; rbx = ((r14.wrapping_add(0x8u64)) as u64) as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 18; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 18; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 18; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 18; continue; } bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 19; continue; } bb = 16; continue; }
            16 => { fl_l = (r15) as u64 as u64; fl_r = (r15) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x19u64))) as u64; bb = 19; continue; }
            18 => { rbx = ((r14.wrapping_add(0x8u64)) as u64) as u64; bb = 19; continue; }
            19 => { rsi = (crate::simd_ops::ru32(r14)) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 217; continue; } bb = 20; continue; }
            20 => { rax = (((crate::simd_ops::ru32(rdx.wrapping_add(0x20u64))) as i32 as i64) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x1u64), (rax) as u64); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 219; continue; } bb = 21; continue; }
            21 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x77u64))) as u64; rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x21u64), (rax) as u64); bb = 22; continue; }
            22 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; r8 = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x69u64), (r8) as u64); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 29; continue; } bb = 23; continue; }
            23 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 29; continue; } bb = 24; continue; }
            24 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x18u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 29; continue; } bb = 25; continue; }
            25 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 29; continue; } bb = 26; continue; }
            26 => { fl_l = (crate::simd_ops::ru8(r8.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 29; continue; } bb = 27; continue; }
            27 => { fl_l = (r15) as u64 as u64; fl_r = (r15) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 28; continue; }
            28 => { rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x69u64))) as u64; bb = 29; continue; }
            29 => { rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 124; continue; } bb = 30; continue; }
            30 => { rax = (((crate::simd_ops::ru32(r8.wrapping_add(0x20u64))) as i32 as i64) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), (rax) as u64); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 212; continue; } bb = 31; continue; }
            31 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x71u64), (rax) as u64); bb = 32; continue; }
            32 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(0x18u64))) as u64; r8 = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), (r8) as u64); if (((fl_l as u8)) != ((fl_r as u8))) { bb = 38; continue; } bb = 33; continue; }
            33 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x18u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 38; continue; } bb = 34; continue; }
            34 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 38; continue; } bb = 35; continue; }
            35 => { fl_l = (crate::simd_ops::ru8(r8.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 38; continue; } bb = 36; continue; }
            36 => { fl_l = (r15) as u64 as u64; fl_r = (r15) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 38; continue; } bb = 37; continue; }
            37 => { rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x29u64))) as u64; bb = 38; continue; }
            38 => { rdx = ((crate::simd_ops::ru8(rbx.wrapping_add(0x10u64))) as u32) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 82; continue; } bb = 39; continue; }
            39 => { rax = (((crate::simd_ops::ru32(r8.wrapping_add(0x20u64))) as i32 as i64) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x31u64), (rax) as u64); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 119; continue; } bb = 40; continue; }
            40 => { rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x79u64), (rcx) as u64); bb = 41; continue; }
            41 => { r13 = (crate::simd_ops::ru64(r8.wrapping_add(0x18u64))) as u64; r13 = ((r13).wrapping_add(rcx)) as u64; fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x18u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 46; continue; } bb = 42; continue; }
            42 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 46; continue; } bb = 43; continue; }
            43 => { fl_l = (crate::simd_ops::ru8(r13.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 46; continue; } bb = 44; continue; }
            44 => { fl_l = (r15) as u64 as u64; fl_r = (r15) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 46; continue; } bb = 45; continue; }
            45 => { rcx = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x79u64))) as u64; bb = 46; continue; }
            46 => { rdx = ((crate::simd_ops::ru8(rbx.wrapping_add(0x18u64))) as u32) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 64; continue; } bb = 47; continue; }
            47 => { r12 = (((crate::simd_ops::ru32(r13.wrapping_add(0x20u64))) as i32 as i64) as u64) as u64; fl_l = (r12) as u64 as u64; fl_r = (r12) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 77; continue; } bb = 48; continue; }
            48 => { rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 49; continue; }
            49 => { rsi = (crate::simd_ops::ru64(r13.wrapping_add(0x18u64))) as u64; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(rsi).wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 52; continue; } bb = 50; continue; }
            50 => { fl_l = (r15) as u64 as u64; fl_r = (r15) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 52; continue; } bb = 51; continue; }
            51 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rsi).wrapping_add(0x8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 52; continue; }
            52 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; r14 = ((rdi.wrapping_add(rsi)) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 61; continue; } bb = 53; continue; }
            53 => { rbx = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(rsi).wrapping_add(0x20u64))) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rbx)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 58; continue; } bb = 54; continue; }
            54 => { bb = 55; continue; }
            55 => { fl_l = (r15) as u64 as u64; fl_r = (r15) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 57; continue; } bb = 56; continue; }
            56 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 57; continue; }
            57 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0x20u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x10u64)) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r14.wrapping_add(0x18u64)))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 55; continue; } bb = 58; continue; }
            58 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x77u64))) as u64; bb = 59; continue; }
            59 => { rdi = ((rdi).wrapping_add(0x28u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), (0x0u64) as u64); fl_l = (r12) as u64 as u64; fl_r = (0x1u64) as u64 as u64; r12 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 49; continue; } bb = 60; continue; }
            60 => { bb = 76; continue; }
            61 => { rcx = ((r14.wrapping_add(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 59; continue; } bb = 62; continue; }
            62 => { fl_l = (r15) as u64 as u64; fl_r = (r15) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 59; continue; } bb = 63; continue; }
            63 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 59; continue; }
            64 => { rcx = ((r13.wrapping_add(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r14 = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 76; continue; } bb = 65; continue; }
            65 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 68; continue; } bb = 66; continue; }
            66 => { fl_l = (r15) as u64 as u64; fl_r = (r15) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 68; continue; } bb = 67; continue; }
            67 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 68; continue; }
            68 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 79; continue; } bb = 69; continue; }
            69 => { rbx = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(0x20u64))) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rbx)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 74; continue; } bb = 70; continue; }
            70 => { bb = 71; continue; }
            71 => { fl_l = (r15) as u64 as u64; fl_r = (r15) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 73; continue; } bb = 72; continue; }
            72 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 73; continue; }
            73 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0x20u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x10u64)) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r14.wrapping_add(0x18u64)))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 71; continue; } bb = 74; continue; }
            74 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x77u64))) as u64; bb = 75; continue; }
            75 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), (0x0u64) as u64); bb = 76; continue; }
            76 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x79u64))) as u64; bb = 77; continue; }
            77 => { r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x29u64))) as u64; rcx = ((rcx).wrapping_add(0x28u64)) as u64; fl_l = (crate::simd_ops::ru64(rbp.wrapping_sub(0x31u64))) as u64 as u64; fl_r = (0x1u64) as u64 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x31u64), ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64); crate::simd_ops::wu8(rbp.wrapping_sub(0x41u64), (0xffu8) as u8); crate::simd_ops::wu64(rbp.wrapping_sub(0x79u64), (rcx) as u64); if (((fl_l as u64)) != ((fl_r as u64))) { bb = 41; continue; } bb = 78; continue; }
            78 => { bb = 119; continue; }
            79 => { rcx = ((r14.wrapping_add(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 75; continue; } bb = 80; continue; }
            80 => { fl_l = (r15) as u64 as u64; fl_r = (r15) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 75; continue; } bb = 81; continue; }
            81 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 75; continue; }
            82 => { rcx = ((r8.wrapping_add(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdi = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 119; continue; } bb = 83; continue; }
            83 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x18u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 88; continue; } bb = 84; continue; }
            84 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 88; continue; } bb = 85; continue; }
            85 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 88; continue; } bb = 86; continue; }
            86 => { fl_l = (r15) as u64 as u64; fl_r = (r15) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 88; continue; } bb = 87; continue; }
            87 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 88; continue; }
            88 => { rdx = ((crate::simd_ops::ru8(rbx.wrapping_add(0x18u64))) as u32) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 106; continue; } bb = 89; continue; }
            89 => { r13 = (((crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as i32 as i64) as u64) as u64; fl_l = (r13) as u64 as u64; fl_r = (r13) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 118; continue; } bb = 90; continue; }
            90 => { r12 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 91; continue; }
            91 => { r14 = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; r14 = ((r14).wrapping_add(r12)) as u64; fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 94; continue; } bb = 92; continue; }
            92 => { fl_l = (r15) as u64 as u64; fl_r = (r15) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 94; continue; } bb = 93; continue; }
            93 => { rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 94; continue; }
            94 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 103; continue; } bb = 95; continue; }
            95 => { rbx = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(0x20u64))) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rbx)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 100; continue; } bb = 96; continue; }
            96 => { bb = 97; continue; }
            97 => { fl_l = (r15) as u64 as u64; fl_r = (r15) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 99; continue; } bb = 98; continue; }
            98 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 99; continue; }
            99 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0x20u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x10u64)) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r14.wrapping_add(0x18u64)))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 97; continue; } bb = 100; continue; }
            100 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x77u64))) as u64; bb = 101; continue; }
            101 => { r12 = ((r12).wrapping_add(0x28u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), (0x0u64) as u64); fl_l = (r13) as u64 as u64; fl_r = (0x1u64) as u64 as u64; r13 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 91; continue; } bb = 102; continue; }
            102 => { bb = 118; continue; }
            103 => { rcx = ((r14.wrapping_add(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 101; continue; } bb = 104; continue; }
            104 => { fl_l = (r15) as u64 as u64; fl_r = (r15) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 101; continue; } bb = 105; continue; }
            105 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 101; continue; }
            106 => { rcx = ((rdi.wrapping_add(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r14 = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 118; continue; } bb = 107; continue; }
            107 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 110; continue; } bb = 108; continue; }
            108 => { fl_l = (r15) as u64 as u64; fl_r = (r15) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 110; continue; } bb = 109; continue; }
            109 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 110; continue; }
            110 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 121; continue; } bb = 111; continue; }
            111 => { rbx = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(0x20u64))) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rbx)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 116; continue; } bb = 112; continue; }
            112 => { bb = 113; continue; }
            113 => { fl_l = (r15) as u64 as u64; fl_r = (r15) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 115; continue; } bb = 114; continue; }
            114 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 115; continue; }
            115 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0x20u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x10u64)) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r14.wrapping_add(0x18u64)))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 113; continue; } bb = 116; continue; }
            116 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x77u64))) as u64; bb = 117; continue; }
            117 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), (0x0u64) as u64); bb = 118; continue; }
            118 => { crate::simd_ops::wu8(rbp.wrapping_sub(0x41u64), (0xffu8) as u8); bb = 119; continue; }
            119 => { rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x71u64))) as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x69u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu8(rbp.wrapping_sub(0x49u64), (0xffu8) as u8); crate::simd_ops::wu64(rbp.wrapping_sub(0x71u64), (rax) as u64); fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbp.wrapping_sub(0x9u64))) as u64 as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 32; continue; } bb = 120; continue; }
            120 => { bb = 212; continue; }
            121 => { rcx = ((r14.wrapping_add(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 117; continue; } bb = 122; continue; }
            122 => { fl_l = (r15) as u64 as u64; fl_r = (r15) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 117; continue; } bb = 123; continue; }
            123 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 117; continue; }
            124 => { rcx = ((r8.wrapping_add(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rbp.wrapping_sub(0x71u64), (rax) as u64); r8 = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 212; continue; } bb = 125; continue; }
            125 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 131; continue; } bb = 126; continue; }
            126 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x18u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 131; continue; } bb = 127; continue; }
            127 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 131; continue; } bb = 128; continue; }
            128 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 131; continue; } bb = 129; continue; }
            129 => { fl_l = (r15) as u64 as u64; fl_r = (r15) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 131; continue; } bb = 130; continue; }
            130 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x71u64))) as u64; bb = 131; continue; }
            131 => { rdx = ((crate::simd_ops::ru8(rbx.wrapping_add(0x10u64))) as u32) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 175; continue; } bb = 132; continue; }
            132 => { rax = (((crate::simd_ops::ru32(r8.wrapping_add(0x20u64))) as i32 as i64) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x69u64), (rax) as u64); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 211; continue; } bb = 133; continue; }
            133 => { rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x79u64), (rcx) as u64); bb = 134; continue; }
            134 => { rdi = (crate::simd_ops::ru64(r8.wrapping_add(0x18u64))) as u64; rdi = ((rdi).wrapping_add(rcx)) as u64; fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x18u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 139; continue; } bb = 135; continue; }
            135 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 139; continue; } bb = 136; continue; }
            136 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 139; continue; } bb = 137; continue; }
            137 => { fl_l = (r15) as u64 as u64; fl_r = (r15) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 139; continue; } bb = 138; continue; }
            138 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x79u64))) as u64; bb = 139; continue; }
            139 => { rdx = ((crate::simd_ops::ru8(rbx.wrapping_add(0x18u64))) as u32) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 157; continue; } bb = 140; continue; }
            140 => { r13 = (((crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as i32 as i64) as u64) as u64; fl_l = (r13) as u64 as u64; fl_r = (r13) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 170; continue; } bb = 141; continue; }
            141 => { r12 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 142; continue; }
            142 => { r14 = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; r14 = ((r14).wrapping_add(r12)) as u64; fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 145; continue; } bb = 143; continue; }
            143 => { fl_l = (r15) as u64 as u64; fl_r = (r15) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 145; continue; } bb = 144; continue; }
            144 => { rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 145; continue; }
            145 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 154; continue; } bb = 146; continue; }
            146 => { rbx = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(0x20u64))) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rbx)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 151; continue; } bb = 147; continue; }
            147 => { bb = 148; continue; }
            148 => { fl_l = (r15) as u64 as u64; fl_r = (r15) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 150; continue; } bb = 149; continue; }
            149 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 150; continue; }
            150 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0x20u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x10u64)) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r14.wrapping_add(0x18u64)))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 148; continue; } bb = 151; continue; }
            151 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x77u64))) as u64; bb = 152; continue; }
            152 => { r12 = ((r12).wrapping_add(0x28u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), (0x0u64) as u64); fl_l = (r13) as u64 as u64; fl_r = (0x1u64) as u64 as u64; r13 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 142; continue; } bb = 153; continue; }
            153 => { bb = 169; continue; }
            154 => { rcx = ((r14.wrapping_add(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 152; continue; } bb = 155; continue; }
            155 => { fl_l = (r15) as u64 as u64; fl_r = (r15) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 152; continue; } bb = 156; continue; }
            156 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 152; continue; }
            157 => { rcx = ((rdi.wrapping_add(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r14 = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 169; continue; } bb = 158; continue; }
            158 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 161; continue; } bb = 159; continue; }
            159 => { fl_l = (r15) as u64 as u64; fl_r = (r15) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 161; continue; } bb = 160; continue; }
            160 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 161; continue; }
            161 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 172; continue; } bb = 162; continue; }
            162 => { rbx = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(0x20u64))) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rbx)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 167; continue; } bb = 163; continue; }
            163 => { bb = 164; continue; }
            164 => { fl_l = (r15) as u64 as u64; fl_r = (r15) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 166; continue; } bb = 165; continue; }
            165 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 166; continue; }
            166 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0x20u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x10u64)) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r14.wrapping_add(0x18u64)))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 164; continue; } bb = 167; continue; }
            167 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x77u64))) as u64; bb = 168; continue; }
            168 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), (0x0u64) as u64); bb = 169; continue; }
            169 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x79u64))) as u64; bb = 170; continue; }
            170 => { r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x71u64))) as u64; rcx = ((rcx).wrapping_add(0x28u64)) as u64; fl_l = (crate::simd_ops::ru64(rbp.wrapping_sub(0x69u64))) as u64 as u64; fl_r = (0x1u64) as u64 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x69u64), ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64); crate::simd_ops::wu8(rbp.wrapping_sub(0x41u64), (0xffu8) as u8); crate::simd_ops::wu64(rbp.wrapping_sub(0x79u64), (rcx) as u64); if (((fl_l as u64)) != ((fl_r as u64))) { bb = 134; continue; } bb = 171; continue; }
            171 => { bb = 211; continue; }
            172 => { rcx = ((r14.wrapping_add(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 168; continue; } bb = 173; continue; }
            173 => { fl_l = (r15) as u64 as u64; fl_r = (r15) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 168; continue; } bb = 174; continue; }
            174 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 168; continue; }
            175 => { rcx = ((r8.wrapping_add(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r13 = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 211; continue; } bb = 176; continue; }
            176 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x18u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 181; continue; } bb = 177; continue; }
            177 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 181; continue; } bb = 178; continue; }
            178 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 181; continue; } bb = 179; continue; }
            179 => { fl_l = (r15) as u64 as u64; fl_r = (r15) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 181; continue; } bb = 180; continue; }
            180 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 181; continue; }
            181 => { rdx = ((crate::simd_ops::ru8(rbx.wrapping_add(0x18u64))) as u32) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 198; continue; } bb = 182; continue; }
            182 => { r12 = (((crate::simd_ops::ru32(r13.wrapping_add(0x20u64))) as i32 as i64) as u64) as u64; fl_l = (r12) as u64 as u64; fl_r = (r12) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 210; continue; } bb = 183; continue; }
            183 => { r15 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 184; continue; }
            184 => { r14 = (crate::simd_ops::ru64(r13.wrapping_add(0x18u64))) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64; r14 = ((r14).wrapping_add(r15)) as u64; fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 187; continue; } bb = 185; continue; }
            185 => { fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 187; continue; } bb = 186; continue; }
            186 => { rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64; bb = 187; continue; }
            187 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 195; continue; } bb = 188; continue; }
            188 => { rbx = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(0x20u64))) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rbx)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 192; continue; } bb = 189; continue; }
            189 => { fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 191; continue; } bb = 190; continue; }
            190 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64; bb = 191; continue; }
            191 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0x20u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x10u64)) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r14.wrapping_add(0x18u64)))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 189; continue; } bb = 192; continue; }
            192 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x77u64))) as u64; bb = 193; continue; }
            193 => { r15 = ((r15).wrapping_add(0x28u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), (0x0u64) as u64); fl_l = (r12) as u64 as u64; fl_r = (0x1u64) as u64 as u64; r12 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 184; continue; } bb = 194; continue; }
            194 => { r15 = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64; bb = 210; continue; }
            195 => { rcx = ((r14.wrapping_add(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 193; continue; } bb = 196; continue; }
            196 => { fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 193; continue; } bb = 197; continue; }
            197 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 193; continue; }
            198 => { rcx = ((r13.wrapping_add(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r14 = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 210; continue; } bb = 199; continue; }
            199 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 202; continue; } bb = 200; continue; }
            200 => { fl_l = (r15) as u64 as u64; fl_r = (r15) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 202; continue; } bb = 201; continue; }
            201 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 202; continue; }
            202 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 214; continue; } bb = 203; continue; }
            203 => { rbx = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(0x20u64))) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rbx)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 208; continue; } bb = 204; continue; }
            204 => { bb = 205; continue; }
            205 => { fl_l = (r15) as u64 as u64; fl_r = (r15) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 207; continue; } bb = 206; continue; }
            206 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 207; continue; }
            207 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0x20u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x10u64)) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r14.wrapping_add(0x18u64)))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 205; continue; } bb = 208; continue; }
            208 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x77u64))) as u64; bb = 209; continue; }
            209 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), (0x0u64) as u64); bb = 210; continue; }
            210 => { crate::simd_ops::wu8(rbp.wrapping_sub(0x41u64), (0xffu8) as u8); bb = 211; continue; }
            211 => { crate::simd_ops::wu8(rbp.wrapping_sub(0x49u64), (0xffu8) as u8); bb = 212; continue; }
            212 => { rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x21u64))) as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x19u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x51u64), (0x0u32) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x21u64), (rax) as u64); fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbp.wrapping_sub(0x1u64))) as u64 as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 22; continue; } bb = 213; continue; }
            213 => { rdi = ((crate::simd_ops::ru8(rbp.wrapping_add(0x6fu64))) as u32) as u32 as u64; r14 = (crate::simd_ops::ru64(rbp.wrapping_add(0x77u64))) as u64; bb = 219; continue; }
            214 => { rcx = ((r14.wrapping_add(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 209; continue; } bb = 215; continue; }
            215 => { fl_l = (r15) as u64 as u64; fl_r = (r15) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 209; continue; } bb = 216; continue; }
            216 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 209; continue; }
            217 => { rcx = ((rdx.wrapping_add(0x18u64)) as u64) as u64; rdx = ((rsi as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 219; continue; } bb = 218; continue; }
            218 => { rcx = ((rbp.wrapping_sub(0x51u64)) as u64) as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x59u64), ((rsi as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rcx) as u64); r9 = ((rbp.wrapping_sub(0x61u64)) as u64) as u64; rcx = (rax) as u64; r8 = (rbx) as u64; rdx = (((rdi as u8)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 219; continue; }
            219 => { rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x11u64))) as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x59u64), (0x0u32) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), (rax) as u64); fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbp.wrapping_add(0x7u64))) as u64 as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 10; continue; } bb = 220; continue; }
            220 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; rsp = ((rsp).wrapping_add(0xe8u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            221 => { rcx = ((rdx.wrapping_add(0x18u64)) as u64) as u64; rdx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 223; continue; } bb = 222; continue; }
            222 => { rcx = ((rbp.wrapping_sub(0x59u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x61u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rcx) as u64); r9 = ((rbp.wrapping_sub(0x61u64)) as u64) as u64; rcx = (rax) as u64; r8 = (r14) as u64; rdx = (((rdi as u8)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 223; continue; }
            223 => { rsp = ((rsp).wrapping_add(0xe8u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141614f90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415c4520;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; rdi = r8; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rsi = rdx; rax = ((rax).wrapping_shl(4u32)) as u64; r14 = rcx; rax = ((rax).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 29; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r15); r15 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 8; continue; }
            8 => { rcx = ((crate::simd_ops::ru8(rdi.wrapping_add(0x18u64))) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x18u64))) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 14; continue; } bb = 10; continue; }
            10 => { rcx = ((crate::simd_ops::ru8(rdi.wrapping_add(0x20u64))) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x20u64))) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 14; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { fl_l = ((rbx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 27; continue; }
            15 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 16; continue; }
            16 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 24; continue; } bb = 18; continue; }
            18 => { rax = ((rax).wrapping_add(0x10u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 19; continue; }
            19 => { rax = ((crate::simd_ops::ru8(rsi.wrapping_add(0x11u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x8u64), ((rax as u8)) as u8); rbp = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsi.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 22; continue; } bb = 20; continue; }
            20 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 24; continue; } bb = 21; continue; }
            21 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsi.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 24; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rax as u32)) as u32); fl_l = ((rcx).wrapping_add(crate::simd_ops::ru64(rsi))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { crate::simd_ops::wu64(rcx, r15); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), ((r15 as u8)) as u8); xmm0 = (crate::simd_ops::ru128(rbx)) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); rcx = ((crate::simd_ops::ru8(rsi.wrapping_add(0x10u64))) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xfcu8) as u8 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0xfeu8))) as u8 as u64)); fl_l = ((((rax as u8)) | ((rcx as u8)))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rsi.wrapping_add(0x10u64), ((rax as u8)) as u8); bb = 24; continue; }
            24 => { rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rdx = (crate::simd_ops::ru64(r14)) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(0xfffffffffffffff0u64)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 26; continue; } bb = 25; continue; }
            25 => { xmm0 = (crate::simd_ops::ru128(rdx)) as u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); bb = 26; continue; }
            26 => { crate::simd_ops::lk_sub_u32(r14.wrapping_add(0x8u64), (1) as u32); rcx = ((crate::simd_ops::ru8(r14.wrapping_add(0x10u64))) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xfcu8) as u8 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0xfeu8))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) | ((rcx as u8)))) as u8 as u64)); crate::simd_ops::wu8(r14.wrapping_add(0x10u64), ((rax as u8)) as u8); rcx = ((crate::simd_ops::ru8(rsi.wrapping_add(0x10u64))) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xfcu8) as u8 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0xfeu8))) as u8 as u64)); fl_l = ((((rax as u8)) | ((rcx as u8)))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rsi.wrapping_add(0x10u64), ((rax as u8)) as u8); bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r14))) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 28; continue; }
            28 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 29; continue; }
            29 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141615120(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524340; const P1: u64 = 0x141524330; const P2: u64 = 0x1415242e0; const P3: u64 = 0x143930f44; const P4: u64 = 0x1415c2bf0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r12 = rdx; rbx = rcx; r13 = 0x1; r9 = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rax = ((rax).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rcx, rax); r15 = ((r9 as u32)) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 26; continue; } bb = 1; continue; }
            1 => { r14 = r8.wrapping_add(0x48); rbp = 0; fl_l = 0; fl_r = 0; fl_l = ((r15 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r14.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 15; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(r14)) as u64; rdx = ((r9 as u32)) as u32 as u64; rsi = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rdx = ((rdx).wrapping_shl(3u32)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { r8 = rdx; rdx = rax; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 4; continue; }
            4 => { bb = 13; continue; }
            5 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(r14)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 7; continue; }
            7 => { fl_l = rax; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 8; continue; }
            8 => { rdx = rbp; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(r14)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(rdx.wrapping_mul(8)), rcx); rdx = ((rdx).wrapping_add(1)) as u64; fl_l = rdx; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 10; continue; } bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(r14)) as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { crate::simd_ops::wu64(r14, rdi); rax = (((r13 as u8)) as u32) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0xcu64), ((r15 as u32)) as u32); bb = 13; continue; }
            13 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rcx = 0x34; rcx = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (r13 as u32) } else { (rcx as u32) }) as u32 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = ((rcx as u32)) as u32 as u64; bb = 27; continue; }
            15 => { fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 26; continue; } bb = 16; continue; }
            16 => { bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rsi = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = ((rax).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu64(rbx, rax); r13 = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(rbx, rax); r11 = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(rbx, rax); rcx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rax = ((rax).wrapping_add(1)) as u64; r9 = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu64(rbx, rax); r9 = ((((r9 as u32)) & (0x7fu32))) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if (((((fl_l as u8)) & ((fl_r as u8))) as i8) >= 0) { bb = 20; continue; } bb = 18; continue; }
            18 => { bb = 19; continue; }
            19 => { r8 = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rcx = ((r9 as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(7u32)) as u32 as u64; rax = ((rax).wrapping_add(1)) as u64; rdx = ((r8 as u32)) as u32 as u64; crate::simd_ops::wu64(rbx, rax); rdx = ((((rdx as u32)) & (0x7fu32))) as u32 as u64; r9 = ((rdx as u32)) as u32 as u64; r9 = ((((r9 as u32)) | ((rcx as u32)))) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if (((((fl_l as u8)) & ((fl_r as u8))) as i8) < 0) { bb = 19; continue; } bb = 20; continue; }
            20 => { r8 = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = ((rax).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu64(rbx, rax); rdx = rax.wrapping_add(0x1); r10 = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; crate::simd_ops::wu64(rbx, rdx); rdi = ((crate::simd_ops::ru16(rdx)) as u32) as u32 as u64; rdx = ((rdx).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rbx, rdx); fl_l = ((r11 as u8)) as u8 as u64; fl_r = (0x6u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 22; continue; } bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::ru32(P3)) as u32 as u64; fl_r = 0x1; rax = 0x2; rcx = 0x5; rax = (if (((fl_l as u32)) == ((fl_r as u32))) { (rcx as u32) } else { (rax as u32) }) as u32 as u64; bb = 23; continue; }
            22 => { rax = (((r11 as u8)) as u32) as u32 as u64; bb = 23; continue; }
            23 => { rcx = rsp.wrapping_add(0xa0); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rcx); rcx = r14; crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((rdi as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdx); rdx = 0x4; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((r10 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((r8 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r9 as u32)) as u32); r9 = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((r13 as u8)) as u8); r8 = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 27; continue; } bb = 24; continue; }
            24 => { rax = ((((rdi as u32)) as i32).wrapping_mul((0xfffffff4u32) as i32) as u32) as u32 as u64; rcx = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(2u32)) as u64; rbp = (((rbp as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::lk_add_u64(rbx, (rcx) as u64); crate::simd_ops::lk_add_u32(r12, ((rax as u32)) as u32); fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 17; continue; } bb = 25; continue; }
            25 => { r13 = 0x1; bb = 26; continue; }
            26 => { rax = ((r13 as u32)) as u32 as u64; bb = 27; continue; }
            27 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141615340(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141624420; const P1: u64 = 0x1415fa0e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = rcx; r15 = 0; fl_l = 0; fl_r = 0; rcx = ((rcx).wrapping_add(0x8u64)) as u64; rbp = r9; rsi = r8; fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x10u64))) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; rcx = (if (((fl_l as u8)) == ((fl_r as u8))) { r15 } else { rcx }) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(r9)) as u64; fl_r = r15; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rsi = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(0x20u64))) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 5; continue; }
            5 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rsi, rax); fl_l = (crate::simd_ops::ru64(rbp)) as u64; fl_r = r15; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0x20u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x10u64)) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r14.wrapping_add(0x18u64)))) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rsi, r15); bb = 15; continue; }
            10 => { rdx = rsi; rcx = r14.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rbx, rsi); fl_l = (crate::simd_ops::ru64(rbp)) as u64; fl_r = r15; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rbx, r15); bb = 15; continue; }
            14 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rax, r15); bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141615420(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141624420; const P1: u64 = 0x1415fa0e0; const P2: u64 = 0x141616430; const P3: u64 = 0x141615340; const P4: u64 = 0x141615700;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu8(rsp.wrapping_add(0x10u64), ((rdx as u8)) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; r12 = r8; rbp = r9; r8 = 0; fl_l = 0; fl_r = 0; rdi = (((rdx as u8)) as u32) as u32 as u64; rax = rcx; fl_l = (crate::simd_ops::ru8(r12)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(r12.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; rsi = r12.wrapping_add(0x8); if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rcx = ((rcx).wrapping_add(0x8u64)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x10u64))) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; rcx = (if (((fl_l as u8)) == ((fl_r as u8))) { r8 } else { rcx }) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(r9)) as u64; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r8 = 0; fl_l = 0; fl_r = 0; bb = 7; continue; }
            6 => { rsi = r12.wrapping_add(0x8); bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; bb = 8; continue; }
            8 => { r14 = ((crate::simd_ops::ru8(r12)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rbx); fl_l = ((r14 as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 40; continue; } bb = 9; continue; }
            9 => { rdx = (((crate::simd_ops::ru32(rax.wrapping_add(0x20u64))) as i32 as i64) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdx); fl_l = rdx; fl_r = rdx; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 42; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r13); rcx = r8; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r15); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), rcx); bb = 11; continue; }
            11 => { rsi = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; rsi = ((rsi).wrapping_add(rcx)) as u64; rax = ((crate::simd_ops::ru8(rsi)) as u32) as u32 as u64; crate::simd_ops::wu8(r15, ((rax as u8)) as u8); fl_l = (crate::simd_ops::ru8(r12.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 16; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru64(r12.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 16; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru64(rbp)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 16; continue; }
            16 => { rbx = ((crate::simd_ops::ru8(r12.wrapping_add(0x8u64))) as u32) as u32 as u64; fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 35; continue; } bb = 17; continue; }
            17 => { r13 = (((crate::simd_ops::ru32(rsi.wrapping_add(0x20u64))) as i32 as i64) as u64) as u64; fl_l = r13; fl_r = r13; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 38; continue; } bb = 18; continue; }
            18 => { r12 = r8; bb = 19; continue; }
            19 => { r14 = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; r14 = ((r14).wrapping_add(r12)) as u64; rax = ((crate::simd_ops::ru8(r14)) as u32) as u32 as u64; crate::simd_ops::wu8(r15.wrapping_add(0x8u64), ((rax as u8)) as u8); fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 22; continue; } bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::ru64(rbp)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 22; continue; } bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r8 = 0; fl_l = 0; fl_r = 0; bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 29; continue; } bb = 23; continue; }
            23 => { rbx = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(0x20u64))) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 33; continue; } bb = 24; continue; }
            24 => { bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(r15.wrapping_add(0x10u64), rax); fl_l = (crate::simd_ops::ru64(rbp)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 27; continue; } bb = 26; continue; }
            26 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0x20u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x10u64)) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r14.wrapping_add(0x18u64)))) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 25; continue; } bb = 28; continue; }
            28 => { bb = 32; continue; }
            29 => { rdx = rbx; rcx = r14.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 30; continue; }
            30 => { crate::simd_ops::wu64(r15.wrapping_add(0x10u64), rbx); fl_l = (crate::simd_ops::ru64(rbp)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 32; continue; } bb = 31; continue; }
            31 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 32; continue; }
            32 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 33; continue; }
            33 => { r12 = ((r12).wrapping_add(0x28u64)) as u64; crate::simd_ops::wu64(r15.wrapping_add(0x10u64), r8); fl_l = r13; fl_r = (0x1u64) as u64; r13 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 19; continue; } bb = 34; continue; }
            34 => { rdi = ((crate::simd_ops::ru8(rsp.wrapping_add(0x88u64))) as u32) as u32 as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; bb = 37; continue; }
            35 => { rcx = rsi.wrapping_add(0x18); rdx = (((rbx as u8)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 37; continue; } bb = 36; continue; }
            36 => { rcx = r15.wrapping_add(0x10); crate::simd_ops::wu8(r15.wrapping_add(0x8u64), ((rbx as u8)) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); r8 = r12.wrapping_add(0x10); rcx = rax; r9 = rbp; rdx = (((rdi as u8)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 37; continue; }
            37 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 38; continue; }
            38 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rcx = ((rcx).wrapping_add(0x28u64)) as u64; fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; crate::simd_ops::wu8(r15.wrapping_add(0x8u64), (0xffu8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdx); r8 = 0x0; crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), rcx); if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 39; continue; }
            39 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu8(r15, (0xffu8) as u8); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 43; continue; }
            40 => { rcx = rax.wrapping_add(0x18); rdx = (((r14 as u8)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 42; continue; } bb = 41; continue; }
            41 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; r9 = rbp; r8 = rsi; rdx = (((rdi as u8)) as u32) as u32 as u64; rcx = rbx.wrapping_add(0x8); crate::simd_ops::wu8(rbx, ((r14 as u8)) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); rcx = rax; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rbx, (0xffu8) as u8); bb = 43; continue; }
            42 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; crate::simd_ops::wu8(rax, (0xffu8) as u8); bb = 43; continue; }
            43 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141615700(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141624420; const P1: u64 = 0x1415fa0e0; const P2: u64 = 0x141616430; const P3: u64 = 0x141615340;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = (((rdx as u8)) as u32) as u32 as u64; rbp = r9; rdx = 0; fl_l = 0; fl_r = 0; r13 = r8; fl_l = (crate::simd_ops::ru8(r8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; rax = rcx; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; fl_r = rdx; r14 = r8.wrapping_add(0x8); if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rcx = ((rcx).wrapping_add(0x8u64)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x10u64))) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; rcx = (if (((fl_l as u8)) == ((fl_r as u8))) { rdx } else { rcx }) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(r9)) as u64; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            5 => { r14 = r8.wrapping_add(0x8); bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; bb = 7; continue; }
            7 => { rsi = ((crate::simd_ops::ru8(r13)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rdi); fl_l = ((rsi as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 27; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r12); r12 = (((crate::simd_ops::ru32(rax.wrapping_add(0x20u64))) as i32 as i64) as u64) as u64; fl_l = r12; fl_r = r12; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 26; continue; } bb = 9; continue; }
            9 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), r15); r15 = rdx; bb = 10; continue; }
            10 => { rsi = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; rsi = ((rsi).wrapping_add(r15)) as u64; rax = ((crate::simd_ops::ru8(rsi)) as u32) as u32 as u64; crate::simd_ops::wu8(r14, ((rax as u8)) as u8); fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 13; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru64(rbp)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = 0; fl_l = 0; fl_r = 0; bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 19; continue; } bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x20u64))) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 23; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x8u64), rax); fl_l = (crate::simd_ops::ru64(rbp)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x20u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x10u64)) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0x18u64)))) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 18; continue; }
            18 => { bb = 22; continue; }
            19 => { rdx = rbx; rcx = rsi.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 20; continue; }
            20 => { crate::simd_ops::wu64(r14.wrapping_add(0x8u64), rbx); fl_l = (crate::simd_ops::ru64(rbp)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 22; continue; } bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 22; continue; }
            22 => { rdx = 0; fl_l = 0; fl_r = 0; bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r15 = ((r15).wrapping_add(0x28u64)) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x8u64), rdx); fl_l = r12; fl_r = (0x1u64) as u64; r12 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 24; continue; }
            24 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu8(r14, (0xffu8) as u8); bb = 25; continue; }
            25 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            26 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu8(rax, (0xffu8) as u8); bb = 25; continue; }
            27 => { rcx = rax.wrapping_add(0x18); rdx = (((rsi as u8)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 28; continue; }
            28 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; r9 = rbp; r8 = r14; rdx = (((rbx as u8)) as u32) as u32 as u64; rcx = rdi.wrapping_add(0x8); crate::simd_ops::wu8(rdi, ((rsi as u8)) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); rcx = rax; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rdi, (0xffu8) as u8); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            29 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu8(rax, (0xffu8) as u8); rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416158d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141624420; const P1: u64 = 0x141615420; const P2: u64 = 0x1416164e0; const P3: u64 = 0x141615b30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rbp = r8; r12 = r9; r8 = 0; fl_l = 0; fl_r = 0; rbx = (((rdx as u8)) as u32) as u32 as u64; rax = rcx; fl_l = (crate::simd_ops::ru32(rbp)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; rsi = rbp.wrapping_add(0x8); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rcx = ((rcx).wrapping_add(0x8u64)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x10u64))) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; rcx = (if (((fl_l as u8)) == ((fl_r as u8))) { r8 } else { rcx }) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru64(r9)) as u64; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r8 = 0; fl_l = 0; fl_r = 0; bb = 9; continue; }
            8 => { rsi = rbp.wrapping_add(0x8); bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; bb = 10; continue; }
            10 => { r14 = (crate::simd_ops::ru32(rbp)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rdi); fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 31; continue; } bb = 11; continue; }
            11 => { rdx = (((crate::simd_ops::ru32(rax.wrapping_add(0x20u64))) as i32 as i64) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdx); fl_l = rdx; fl_r = rdx; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 30; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r13); rcx = r8; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r15); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), rcx); bb = 13; continue; }
            13 => { rsi = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; rdi = rbp.wrapping_add(0x10); rsi = ((rsi).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru32(rsi)) as u32 as u64; crate::simd_ops::wu32(r15, ((rax as u32)) as u32); fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x8u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 20; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru8(rdi)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 20; continue; } bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 20; continue; } bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 20; continue; } bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 20; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru64(r12)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 20; continue; }
            20 => { r14 = (crate::simd_ops::ru32(rbp.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 25; continue; } bb = 21; continue; }
            21 => { rdi = (((crate::simd_ops::ru32(rsi.wrapping_add(0x20u64))) as i32 as i64) as u64) as u64; fl_l = rdi; fl_r = rdi; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 28; continue; } bb = 22; continue; }
            22 => { r13 = rbp.wrapping_add(0x10); r14 = r8; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; r12 = r15.wrapping_add(0x10); bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; r9 = rbp; rcx = ((rcx).wrapping_add(r14)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r12); r8 = r13; rdx = (((rbx as u8)) as u32) as u32 as u64; rax = (crate::simd_ops::ru32(rcx)) as u32 as u64; crate::simd_ops::wu32(r15.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r14 = ((r14).wrapping_add(0x28u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 23; continue; } bb = 24; continue; }
            24 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; bb = 27; continue; }
            25 => { rcx = rsi.wrapping_add(0x18); rdx = ((r14 as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 26; continue; }
            26 => { rcx = r15.wrapping_add(0x10); crate::simd_ops::wu32(r15.wrapping_add(0x8u64), ((r14 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); r9 = r12; rcx = rax; r8 = rdi; rdx = (((rbx as u8)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 27; continue; }
            27 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 28; continue; }
            28 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rcx = ((rcx).wrapping_add(0x28u64)) as u64; fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; crate::simd_ops::wu32(r15.wrapping_add(0x8u64), ((r8 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), rcx); if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 29; continue; }
            29 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu32(r15, ((r8 as u32)) as u32); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 34; continue; }
            30 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; crate::simd_ops::wu32(rax, ((r8 as u32)) as u32); bb = 34; continue; }
            31 => { rcx = rax.wrapping_add(0x18); rdx = ((r14 as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 32; continue; }
            32 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; r9 = r12; r8 = rsi; rdx = (((rbx as u8)) as u32) as u32 as u64; rcx = rdi.wrapping_add(0x8); crate::simd_ops::wu32(rdi, ((r14 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); rcx = rax; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rdi, (0x0u32) as u32); bb = 34; continue; }
            33 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; crate::simd_ops::wu32(rax, (0x0u32) as u32); bb = 34; continue; }
            34 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141615b30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141624420; const P1: u64 = 0x141615420; const P2: u64 = 0x1416164e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r12 = 0; fl_l = 0; fl_r = 0; r15 = r9; rdi = r8; rbx = (((rdx as u8)) as u32) as u32 as u64; rbp = rcx; fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(r8.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; rsi = r8.wrapping_add(0x8); if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; fl_r = r12; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 4; continue; }
            4 => { rcx = ((rcx).wrapping_add(0x8u64)) as u64; fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x10u64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; rcx = (if (((fl_l as u8)) == ((fl_r as u8))) { r12 } else { rcx }) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru64(r9)) as u64; fl_r = r12; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            7 => { rsi = r8.wrapping_add(0x8); bb = 8; continue; }
            8 => { r14 = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 13; continue; } bb = 9; continue; }
            9 => { rsi = (((crate::simd_ops::ru32(rbp.wrapping_add(0x20u64))) as i32 as i64) as u64) as u64; fl_l = rsi; fl_r = rsi; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 15; continue; } bb = 10; continue; }
            10 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r13); r13 = rdi.wrapping_add(0x8); rdi = 0; fl_l = 0; fl_r = 0; r12 = r14.wrapping_add(0x8); bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; r9 = r15; rcx = ((rcx).wrapping_add(rdi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r12); r8 = r13; rdx = (((rbx as u8)) as u32) as u32 as u64; rax = (crate::simd_ops::ru32(rcx)) as u32 as u64; crate::simd_ops::wu32(r14, ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdi = ((rdi).wrapping_add(0x28u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 12; continue; }
            12 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu32(r14, ((rsi as u32)) as u32); bb = 16; continue; }
            13 => { rcx = rbp.wrapping_add(0x18); rdx = ((r14 as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; r9 = r15; r8 = rsi; rdx = (((rbx as u8)) as u32) as u32 as u64; rcx = rdi.wrapping_add(0x8); crate::simd_ops::wu32(rdi, ((r14 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rdi, ((r12 as u32)) as u32); bb = 16; continue; }
            15 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu32(rax, ((r12 as u32)) as u32); bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141615c60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415bd140; const P1: u64 = 0x14268f670; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x28; const P5: u64 = 0x30; const P6: u64 = 0x38; const P7: u64 = 0x40; const P8: u64 = 0x48; const P9: u64 = 0x50; const P10: u64 = 0x58; const P11: u64 = 0x68;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = (((r9 as u8)) as u32) as u32 as u64; rdi = (((r8 as u8)) as u32) as u32 as u64; rbx = ((rdx as u32)) as u32 as u64; rsi = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = 0; fl_l = 0; fl_r = 0; rax = P1; crate::simd_ops::wu64(rsi, rax); xmm0 = 0u128; crate::simd_ops::wu64(rsi.wrapping_add(P2), rcx); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsi.wrapping_add(P3), rcx); rbp = (rbp & !0xffu64) | (((((rbp as u8)).wrapping_add((rbp as u8))) as u8 as u64)); crate::simd_ops::wu16(rsi.wrapping_add(P4), ((rcx as u16)) as u16); crate::simd_ops::wu64(rsi.wrapping_add(P5), rcx); crate::simd_ops::wu64(rsi.wrapping_add(P6), rcx); crate::simd_ops::wu16(rsi.wrapping_add(P7), ((rcx as u16)) as u16); crate::simd_ops::wu64(rsi.wrapping_add(P8), rcx); crate::simd_ops::wu64(rsi.wrapping_add(P9), rcx); crate::simd_ops::wu128(rsi.wrapping_add(P10), (xmm0) as u128); crate::simd_ops::wu128(rsi.wrapping_add(P11), (xmm0) as u128); crate::simd_ops::wu8(rsi.wrapping_add(0x68u64), ((rax as u8)) as u8); crate::simd_ops::wu64(rsi.wrapping_add(0x60u64), rcx); crate::simd_ops::wu64(rsi.wrapping_add(0x70u64), rcx); crate::simd_ops::wu64(rsi.wrapping_add(0x78u64), rcx); rax = ((crate::simd_ops::ru8(rsi.wrapping_add(0x94u64))) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xfeu8))) as u8 as u64)); crate::simd_ops::wu32(rsi.wrapping_add(0x90u64), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) | ((rdi as u8)))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xfdu8))) as u8 as u64)); crate::simd_ops::wu64(rsi.wrapping_add(0x80u64), rcx); rax = (rax & !0xffu64) | ((((((rax as u8)) | ((rbp as u8)))) as u8 as u64)); crate::simd_ops::wu64(rsi.wrapping_add(0x88u64), rcx); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x94u64), ((rax as u8)) as u8); rax = rsi; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141615d20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14268f670; const P1: u64 = 0x1415c2b30; const P2: u64 = 0x141619960; const P3: u64 = 0x1415242e0; const P4: u64 = 0x1415bd160;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = P0; rbx = rcx; crate::simd_ops::wu64(rcx, rax); rcx = ((rcx).wrapping_add(0x48u64)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru32(rbx.wrapping_add(0x78u64))) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1; rdi = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) < 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x70u64))) as u64; rcx = ((rdi.wrapping_add(rdi.wrapping_mul(4))) as u64) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1; rdi = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) >= 0) { bb = 2; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x70u64))) as u64; rdi = 0; fl_l = 0; fl_r = 0; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(0x78u64), ((rdi as u32)) as u32); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x70u64), rdi); crate::simd_ops::wu32(rbx.wrapping_add(0x7cu64), ((rdi as u32)) as u32); bb = 5; continue; }
            5 => { crate::simd_ops::wu8(rbx.wrapping_add(0x68u64), ((rdi as u8)) as u8); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x88u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rbx.wrapping_add(0x88u64), rdi); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x80u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rcx = rbx.wrapping_add(0x48); crate::simd_ops::wu64(rbx.wrapping_add(0x80u64), rdi); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(0x38u64), ((rdi as u32)) as u32); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x30u64), rdi); crate::simd_ops::wu32(rbx.wrapping_add(0x3cu64), ((rdi as u32)) as u32); bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(0x20u64), ((rdi as u32)) as u32); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rdi); crate::simd_ops::wu32(rbx.wrapping_add(0x24u64), ((rdi as u32)) as u32); bb = 13; continue; }
            13 => { rcx = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141615e10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18; const P2: u64 = 0x20; const P3: u64 = 0x1415242e0; const P4: u64 = 0x24;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rbx = rdx; crate::simd_ops::wu8(rcx, ((rax as u8)) as u8); rbp = 0; fl_l = 0; fl_r = 0; xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P0))) as u128; rdi = rcx; crate::simd_ops::wu128(rcx.wrapping_add(P0), (xmm0) as u128); rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rbp as u32)) as u32); rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi.wrapping_add(P1), rbp); crate::simd_ops::wu32(rdi.wrapping_add(P4), ((rbp as u32)) as u32); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P1), rax); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P2), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P4), ((rax as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(P1), rbp); crate::simd_ops::wu64(rbx.wrapping_add(P2), rbp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141615e90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18; const P2: u64 = 0x20; const P3: u64 = 0x1415242e0; const P4: u64 = 0x24;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; rdi = rdx; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rbp = 0; fl_l = 0; fl_r = 0; xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P0))) as u128; rbx = rcx; crate::simd_ops::wu128(rcx.wrapping_add(P0), (xmm0) as u128); rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rbp as u32)) as u32); rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(P1), rbp); crate::simd_ops::wu32(rbx.wrapping_add(P4), ((rbp as u32)) as u32); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P1), rax); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P2), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P4), ((rax as u32)) as u32); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P1), rbp); crate::simd_ops::wu64(rdi.wrapping_add(P2), rbp); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141615f10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18; const P2: u64 = 0x20; const P3: u64 = 0x1415242e0; const P4: u64 = 0x24;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rdi = rdx; crate::simd_ops::wu8(rcx, ((rax as u8)) as u8); rbp = 0; fl_l = 0; fl_r = 0; xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P0))) as u128; rbx = rcx; crate::simd_ops::wu128(rcx.wrapping_add(P0), (xmm0) as u128); rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rbp as u32)) as u32); rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(P1), rbp); crate::simd_ops::wu32(rbx.wrapping_add(P4), ((rbp as u32)) as u32); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P1), rax); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P2), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P4), ((rax as u32)) as u32); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P1), rbp); crate::simd_ops::wu64(rdi.wrapping_add(P2), rbp); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141615f90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18; const P2: u64 = 0x20; const P3: u64 = 0x1415242e0; const P4: u64 = 0x24;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rdi = rdx; crate::simd_ops::wu64(rcx, rax); rbp = 0; fl_l = 0; fl_r = 0; xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P0))) as u128; rbx = rcx; crate::simd_ops::wu128(rcx.wrapping_add(P0), (xmm0) as u128); rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rbp as u32)) as u32); rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(P1), rbp); crate::simd_ops::wu32(rbx.wrapping_add(P4), ((rbp as u32)) as u32); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P1), rax); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P2), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P4), ((rax as u32)) as u32); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P1), rbp); crate::simd_ops::wu64(rdi.wrapping_add(P2), rbp); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141616050(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x11; const P2: u64 = 0xc; const P3: u64 = 0x1415c4520; const P4: u64 = 0x1; const P5: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rsi = rdx; r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rbx = rcx; r8 = ((r8).wrapping_shl(4u32)) as u64; r8 = ((r8).wrapping_add(rax)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rdx)) as u64; bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 3; continue; }
            3 => { rax = ((rax).wrapping_add(0x10u64)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P1))) as u32) as u32 as u64; crate::simd_ops::wu8(rdx.wrapping_add(P0), ((rax as u8)) as u8); rdi = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 10; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = ((rax.wrapping_add(P4)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P0), ((rax as u32)) as u32); fl_l = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rcx, (0x0u64) as u64); crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x0u8) as u8); xmm0 = (crate::simd_ops::ru128(rsi)) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); rcx = ((crate::simd_ops::ru8(rbx.wrapping_add(P5))) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xfcu8) as u8 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0xfeu8))) as u8 as u64)); fl_l = ((((rax as u8)) | ((rcx as u8)))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(P5), ((rax as u8)) as u8); bb = 9; continue; }
            9 => { rax = 0x1; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0x34; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}
