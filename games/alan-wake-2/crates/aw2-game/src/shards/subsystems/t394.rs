#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn fn_142526490(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143d01598; const P1: u64 = 0x143422e38; const P2: u64 = 0x8; const P3: u64 = 0x143422d98; const P4: u64 = 0xc; const P5: u64 = 0x20; const P6: u64 = 0x28; const P7: u64 = 0x30; const P8: u64 = 0x40; const P9: u64 = 0x50; const P10: u64 = 0x10; const P11: u64 = 0x60;
    let (mut rax, mut r10) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru32(P0)) as u32 as u64;
    r10 = P1;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    rax = P3;
    crate::simd_ops::wu64(rcx, r10);
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    crate::simd_ops::wu64(rcx, rax);
    crate::simd_ops::wu128(rcx.wrapping_add(P4), (xmm0) as u128);
    crate::simd_ops::wu8(rcx.wrapping_add(P5), (0x0u8) as u8);
    rax = (crate::simd_ops::ru64(r8)) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P6), rax);
    xmm0 = (crate::simd_ops::ru128(r9)) as u128;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    crate::simd_ops::wu128(rcx.wrapping_add(P7), (xmm0) as u128);
    xmm0 = (crate::simd_ops::ru128(rax)) as u128;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu128(rcx.wrapping_add(P8), (xmm0) as u128);
    xmm0 = (crate::simd_ops::ru128(rax)) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P9), (xmm0) as u128);
    xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(P10))) as u64) as u128;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P11), ((xmm1 as u64)) as u64);
    xmm0 = (crate::simd_ops::ru128(rax)) as u128;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    crate::simd_ops::wu128(rcx.wrapping_add(0x68u64), (xmm0) as u128);
    xmm0 = (crate::simd_ops::ru128(rax)) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(0x78u64), (xmm0) as u128);
    xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64) as u128;
    crate::simd_ops::wu64(rcx.wrapping_add(0x88u64), ((xmm1 as u64)) as u64);
    rax = rcx;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142526530(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143d01594; const P1: u64 = 0x143422e38; const P2: u64 = 0x8; const P3: u64 = 0x1434228e8; const P4: u64 = 0xc; const P5: u64 = 0x20; const P6: u64 = 0x40; const P7: u64 = 0x30; const P8: u64 = 0x48; const P9: u64 = 0x78; const P10: u64 = 0x58; const P11: u64 = 0x90;
    let (mut rax, mut r10) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru32(P0)) as u32 as u64; r10 = rcx; rcx = P1; crate::simd_ops::wu32(r10.wrapping_add(P2), ((rax as u32)) as u32); rax = P3; crate::simd_ops::wu64(r10, rcx); xmm0 = (crate::simd_ops::ru128(rdx)) as u128; ymm0_hi = 0; crate::simd_ops::wu64(r10, rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu128(r10.wrapping_add(P4), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(r8)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(r10.wrapping_add(P5), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(r9)) as u128; ymm0_hi = 0; crate::simd_ops::wu64(r10.wrapping_add(P6), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu128(r10.wrapping_add(P7), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rax)) as u128; ymm0_hi = 0; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu128(r10.wrapping_add(P8), (xmm0) as u128); let y_addr: u64 = rax; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x38u64))) as u32) as u32 as u64; crate::simd_ops::wu8(r10.wrapping_add(P9), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; let y_addr: u64 = r10.wrapping_add(P10); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm0 = (crate::simd_ops::ru128(rax)) as u128; ymm0_hi = 0; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu64(r10.wrapping_add(P11), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(r10.wrapping_add(0x98u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(r10.wrapping_add(0xa0u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu128(r10.wrapping_add(0x7cu64), (xmm0) as u128); rcx = (crate::simd_ops::ru64(rax)) as u64; rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x70u64))) as u32) as u32 as u64; crate::simd_ops::wu8(r10.wrapping_add(0xb0u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu64(r10.wrapping_add(0xa8u64), rcx); xmm0 = (crate::simd_ops::ru128(rax)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(r10.wrapping_add(0xb4u64), (xmm0) as u128); rax = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(0xc4u64), ((rax as u32)) as u32); rax = r10; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142526660(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143d01590; const P1: u64 = 0x143422e38; const P2: u64 = 0x8; const P3: u64 = 0x143422708; const P4: u64 = 0xc; const P5: u64 = 0x20; const P6: u64 = 0x28; const P7: u64 = 0x30; const P8: u64 = 0x48; const P9: u64 = 0x40;
    let (mut rax, mut r10) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru32(P0)) as u32 as u64;
    r10 = rcx;
    rcx = P1;
    crate::simd_ops::wu32(r10.wrapping_add(P2), ((rax as u32)) as u32);
    rax = P3;
    crate::simd_ops::wu64(r10, rcx);
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    crate::simd_ops::wu64(r10, rax);
    crate::simd_ops::wu128(r10.wrapping_add(P4), (xmm0) as u128);
    crate::simd_ops::wu64(r10.wrapping_add(P5), r8);
    rax = (crate::simd_ops::ru64(r9)) as u64;
    crate::simd_ops::wu64(r10.wrapping_add(P6), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    xmm0 = (crate::simd_ops::ru128(rax)) as u128;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu128(r10.wrapping_add(P7), (xmm0) as u128);
    rcx = (crate::simd_ops::ru64(rax)) as u64;
    rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x38u64))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r10.wrapping_add(P8), ((rax as u8)) as u8);
    rax = r10;
    crate::simd_ops::wu64(r10.wrapping_add(P9), rcx);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142526710(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143d01600; const P1: u64 = 0x143422e38; const P2: u64 = 0x8; const P3: u64 = 0x143422960; const P4: u64 = 0xc; const P5: u64 = 0x20; const P6: u64 = 0x28;
    let (mut rax, mut r10) = (crate::simd_ops::get_rax_in(), 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru32(P0)) as u32 as u64;
    r10 = P1;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    rax = P3;
    crate::simd_ops::wu64(rcx, r10);
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    crate::simd_ops::wu64(rcx, rax);
    rax = rcx;
    crate::simd_ops::wu128(rcx.wrapping_add(P4), (xmm0) as u128);
    crate::simd_ops::wu64(rcx.wrapping_add(P5), r8);
    crate::simd_ops::wu8(rcx.wrapping_add(P6), ((r9 as u8)) as u8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142526790(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143d01618; const P1: u64 = 0x143422e38; const P2: u64 = 0x8; const P3: u64 = 0x143422b18; const P4: u64 = 0xc; const P5: u64 = 0x20; const P6: u64 = 0x38; const P7: u64 = 0x28; const P8: u64 = 0x3c;
    let (mut rax, mut r10) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru32(P0)) as u32 as u64;
    r10 = P1;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    rax = P3;
    crate::simd_ops::wu64(rcx, r10);
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    crate::simd_ops::wu64(rcx, rax);
    rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x28u64))) as u32) as u32 as u64;
    crate::simd_ops::wu128(rcx.wrapping_add(P4), (xmm0) as u128);
    crate::simd_ops::wu64(rcx.wrapping_add(P5), r8);
    xmm0 = (crate::simd_ops::ru128(r9)) as u128;
    crate::simd_ops::wu8(rcx.wrapping_add(P6), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu128(rcx.wrapping_add(P7), (xmm0) as u128);
    xmm0 = (crate::simd_ops::ru128(rax)) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P8), (xmm0) as u128);
    rax = rcx;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142526830(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143d015c4; const P1: u64 = 0x143422e38; const P2: u64 = 0x8; const P3: u64 = 0x143422c58; const P4: u64 = 0xc; const P5: u64 = 0x20; const P6: u64 = 0x28; const P7: u64 = 0x38; const P8: u64 = 0x48; const P9: u64 = 0x10; const P10: u64 = 0x58;
    let (mut rax, mut r10) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru32(P0)) as u32 as u64;
    r10 = P1;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    rax = P3;
    crate::simd_ops::wu64(rcx, r10);
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    crate::simd_ops::wu64(rcx, rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    crate::simd_ops::wu128(rcx.wrapping_add(P4), (xmm0) as u128);
    crate::simd_ops::wu8(rcx.wrapping_add(P5), (0x0u8) as u8);
    xmm0 = (crate::simd_ops::ru128(r8)) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P6), (xmm0) as u128);
    xmm0 = (crate::simd_ops::ru128(r9)) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P7), (xmm0) as u128);
    xmm0 = (crate::simd_ops::ru128(rax)) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P8), (xmm0) as u128);
    rax = (crate::simd_ops::ru32(rax.wrapping_add(P9))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P10), ((rax as u32)) as u32);
    rax = rcx;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425268d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143d015ec; const P1: u64 = 0x143422e38; const P2: u64 = 0x8; const P3: u64 = 0x143422c08; const P4: u64 = 0x38; const P5: u64 = 0xc; const P6: u64 = 0x20; const P7: u64 = 0x28; const P8: u64 = 0x30; const P9: u64 = 0x40; const P10: u64 = 0x10; const P11: u64 = 0x50;
    let (mut rax, mut r10) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru32(P0)) as u32 as u64;
    r10 = P1;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    rax = P3;
    crate::simd_ops::wu64(rcx, r10);
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    crate::simd_ops::wu64(rcx, rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu128(rcx.wrapping_add(P5), (xmm0) as u128);
    crate::simd_ops::wu8(rcx.wrapping_add(P6), (0x0u8) as u8);
    crate::simd_ops::wu64(rcx.wrapping_add(P7), r8);
    crate::simd_ops::wu64(rcx.wrapping_add(P8), r9);
    xmm0 = (crate::simd_ops::ru128(rax)) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P9), (xmm0) as u128);
    rax = (crate::simd_ops::ru32(rax.wrapping_add(P10))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P11), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    xmm0 = (crate::simd_ops::ru128(rax)) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(0x54u64), (xmm0) as u128);
    rax = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(0x64u64), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x40u64))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(0x68u64), ((rax as u8)) as u8);
    rax = rcx;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142526990(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143d01740; const P1: u64 = 0x143422e38; const P2: u64 = 0x8; const P3: u64 = 0x143422aa0; const P4: u64 = 0xc; const P5: u64 = 0x20; const P6: u64 = 0x28;
    let (mut rax, mut r10) = (crate::simd_ops::get_rax_in(), 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru32(P0)) as u32 as u64;
    r10 = P1;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    rax = P3;
    crate::simd_ops::wu64(rcx, r10);
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    crate::simd_ops::wu64(rcx, rax);
    rax = rcx;
    crate::simd_ops::wu128(rcx.wrapping_add(P4), (xmm0) as u128);
    crate::simd_ops::wu32(rcx.wrapping_add(P5), ((r8 as u32)) as u32);
    crate::simd_ops::wu64(rcx.wrapping_add(P6), r9);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425269d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143d01580; const P1: u64 = 0x143422e38; const P2: u64 = 0x10; const P3: u64 = 0x8; const P4: u64 = 0x1434229b0; const P5: u64 = 0xc; const P6: u64 = 0x20; const P7: u64 = 0x30;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru32(P0)) as u32 as u64;
    r9 = P1;
    xmm1 = ((crate::simd_ops::ru64(r8.wrapping_add(P2))) as u64) as u128;
    crate::simd_ops::wu32(rcx.wrapping_add(P3), ((rax as u32)) as u32);
    rax = P4;
    crate::simd_ops::wu64(rcx, r9);
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P5), (xmm0) as u128);
    xmm0 = (crate::simd_ops::ru128(r8)) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P6), (xmm0) as u128);
    crate::simd_ops::wu64(rcx, rax);
    rax = rcx;
    crate::simd_ops::wu64(rcx.wrapping_add(P7), ((xmm1 as u64)) as u64);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142526a20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143d015b8; const P1: u64 = 0x143422e38; const P2: u64 = 0x8; const P3: u64 = 0x143422640; const P4: u64 = 0xc; const P5: u64 = 0x20; const P6: u64 = 0x28; const P7: u64 = 0x30; const P8: u64 = 0x40;
    let (mut rax, mut r10) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru32(P0)) as u32 as u64;
    r10 = P1;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    rax = P3;
    crate::simd_ops::wu64(rcx, r10);
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    crate::simd_ops::wu64(rcx, rax);
    crate::simd_ops::wu128(rcx.wrapping_add(P4), (xmm0) as u128);
    xmm0 = (crate::simd_ops::ru128(r9)) as u128;
    crate::simd_ops::wu8(rcx.wrapping_add(P5), (0x0u8) as u8);
    rax = (crate::simd_ops::ru64(r8)) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P6), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    crate::simd_ops::wu128(rcx.wrapping_add(P7), (xmm0) as u128);
    xmm0 = (crate::simd_ops::ru128(rax)) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P8), (xmm0) as u128);
    rax = rcx;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142526a80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143d015d4; const P1: u64 = 0x143422e38; const P2: u64 = 0x8; const P3: u64 = 0x143422910; const P4: u64 = 0xc; const P5: u64 = 0x20; const P6: u64 = 0x30;
    let (mut rax, mut r10) = (crate::simd_ops::get_rax_in(), 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru32(P0)) as u32 as u64;
    r10 = P1;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    rax = P3;
    crate::simd_ops::wu64(rcx, r10);
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    crate::simd_ops::wu64(rcx, rax);
    rax = rcx;
    crate::simd_ops::wu128(rcx.wrapping_add(P4), (xmm0) as u128);
    xmm0 = (crate::simd_ops::ru128(r8)) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P5), (xmm0) as u128);
    crate::simd_ops::wu64(rcx.wrapping_add(P6), r9);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142526b00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143d015c8; const P1: u64 = 0x143422e38; const P2: u64 = 0x8; const P3: u64 = 0x143422820; const P4: u64 = 0x38; const P5: u64 = 0xc; const P6: u64 = 0x20; const P7: u64 = 0x28; const P8: u64 = 0x30; const P9: u64 = 0x4c; const P10: u64 = 0x3c;
    let (mut rax, mut r10) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru32(P0)) as u32 as u64;
    r10 = P1;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    rax = P3;
    crate::simd_ops::wu64(rcx, r10);
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    crate::simd_ops::wu64(rcx, rax);
    rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x28u64))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(P4), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu128(rcx.wrapping_add(P5), (xmm0) as u128);
    crate::simd_ops::wu8(rcx.wrapping_add(P6), (0x0u8) as u8);
    crate::simd_ops::wu64(rcx.wrapping_add(P7), r8);
    crate::simd_ops::wu64(rcx.wrapping_add(P8), r9);
    xmm0 = (crate::simd_ops::ru128(rax)) as u128;
    rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x38u64))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(P9), ((rax as u8)) as u8);
    rax = rcx;
    crate::simd_ops::wu128(rcx.wrapping_add(P10), (xmm0) as u128);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142526c10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143d015a8; const P1: u64 = 0x143422e38; const P2: u64 = 0x8; const P3: u64 = 0x143422bb8; const P4: u64 = 0x20; const P5: u64 = 0xc; const P6: u64 = 0x28; const P7: u64 = 0x38; const P8: u64 = 0x40;
    let (mut rax, mut r10) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru32(P0)) as u32 as u64;
    r10 = P1;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    rax = P3;
    crate::simd_ops::wu64(rcx, r10);
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    crate::simd_ops::wu64(rcx, rax);
    rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x28u64))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(P4), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu128(rcx.wrapping_add(P5), (xmm0) as u128);
    xmm0 = (crate::simd_ops::ru128(r8)) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P6), (xmm0) as u128);
    crate::simd_ops::wu64(rcx.wrapping_add(P7), r9);
    xmm0 = (crate::simd_ops::ru128(rax)) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P8), (xmm0) as u128);
    rax = rcx;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142526cb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143d01734; const P1: u64 = 0x143422e38; const P2: u64 = 0x8; const P3: u64 = 0x1434229d8; const P4: u64 = 0xc; const P5: u64 = 0x20; const P6: u64 = 0x28; const P7: u64 = 0x38;
    let (mut rax, mut r10) = (crate::simd_ops::get_rax_in(), 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru32(P0)) as u32 as u64;
    r10 = P1;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    rax = P3;
    crate::simd_ops::wu64(rcx, r10);
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    crate::simd_ops::wu64(rcx, rax);
    rax = rcx;
    crate::simd_ops::wu128(rcx.wrapping_add(P4), (xmm0) as u128);
    crate::simd_ops::wu8(rcx.wrapping_add(P5), (0x0u8) as u8);
    xmm0 = (crate::simd_ops::ru128(r8)) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P6), (xmm0) as u128);
    xmm0 = (crate::simd_ops::ru128(r9)) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P7), (xmm0) as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142526d00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x143d01610; const P2: u64 = 0x20; const P3: u64 = 0x143d01600;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    r8 = rcx;
    rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P1)) as u32 as u64;
    rax = (if (((fl_l as u32)) == ((fl_r as u32))) { rdx } else { rax }) as u64;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; crate::simd_ops::wu64(r8, rax); rax = r8; return rax; }
    fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P3)) as u32 as u64;
    rax = (if (((fl_l as u32)) == ((fl_r as u32))) { rdx } else { rax }) as u64;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; rax = r8; crate::simd_ops::wu64(r8, rcx); return rax; }
    crate::simd_ops::wu64(r8, (0x0u64) as u64);
    rax = r8;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142526d50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143d01744; const P1: u64 = 0x143422e38; const P2: u64 = 0x8; const P3: u64 = 0x143422b68; const P4: u64 = 0x38; const P5: u64 = 0x40; const P6: u64 = 0x48; const P7: u64 = 0x50; const P8: u64 = 0xc; const P9: u64 = 0x20; const P10: u64 = 0x28; const P11: u64 = 0x30;
    let (mut rax, mut r10) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru32(P0)) as u32 as u64;
    r10 = P1;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    rax = P3;
    crate::simd_ops::wu64(rcx, r10);
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    crate::simd_ops::wu64(rcx, rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P5), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P6), rax);
    rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x40u64))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(P7), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    crate::simd_ops::wu128(rcx.wrapping_add(P8), (xmm0) as u128);
    crate::simd_ops::wu8(rcx.wrapping_add(P9), (0x0u8) as u8);
    crate::simd_ops::wu64(rcx.wrapping_add(P10), r8);
    crate::simd_ops::wu64(rcx.wrapping_add(P11), r9);
    xmm0 = (crate::simd_ops::ru128(rax)) as u128;
    rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x50u64))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(0x64u64), ((rax as u8)) as u8);
    rax = rcx;
    crate::simd_ops::wu128(rcx.wrapping_add(0x54u64), (xmm0) as u128);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142526dd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143d01610; const P1: u64 = 0x143422e38; const P2: u64 = 0x8; const P3: u64 = 0x1434227f8; const P4: u64 = 0xc; const P5: u64 = 0x20;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru32(P0)) as u32 as u64;
    r9 = P1;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    rax = P3;
    crate::simd_ops::wu64(rcx, r9);
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    crate::simd_ops::wu64(rcx, rax);
    crate::simd_ops::wu128(rcx.wrapping_add(P4), (xmm0) as u128);
    rax = (crate::simd_ops::ru64(r8)) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P5), rax);
    rax = rcx;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142526e10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143d0159c; const P1: u64 = 0x143422e38; const P2: u64 = 0x8; const P3: u64 = 0x143422dc0; const P4: u64 = 0xc; const P5: u64 = 0x20; const P6: u64 = 0x30; const P7: u64 = 0x40; const P8: u64 = 0x10; const P9: u64 = 0x50; const P10: u64 = 0x58; const P11: u64 = 0x68;
    let (mut rax, mut r10) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru32(P0)) as u32 as u64;
    r10 = P1;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    rax = P3;
    crate::simd_ops::wu64(rcx, r10);
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    crate::simd_ops::wu64(rcx, rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    crate::simd_ops::wu128(rcx.wrapping_add(P4), (xmm0) as u128);
    xmm0 = (crate::simd_ops::ru128(r8)) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P5), (xmm0) as u128);
    xmm0 = (crate::simd_ops::ru128(r9)) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P6), (xmm0) as u128);
    xmm0 = (crate::simd_ops::ru128(rax)) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P7), (xmm0) as u128);
    xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(P8))) as u64) as u128;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P9), ((xmm1 as u64)) as u64);
    xmm0 = (crate::simd_ops::ru128(rax)) as u128;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    crate::simd_ops::wu128(rcx.wrapping_add(P10), (xmm0) as u128);
    xmm0 = (crate::simd_ops::ru128(rax)) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P11), (xmm0) as u128);
    xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64) as u128;
    crate::simd_ops::wu64(rcx.wrapping_add(0x78u64), ((xmm1 as u64)) as u64);
    rax = rcx;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142526ea0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143d0156c; const P1: u64 = 0x143422e38; const P2: u64 = 0x8; const P3: u64 = 0x1434227a8; const P4: u64 = 0xc; const P5: u64 = 0x34; const P6: u64 = 0x20; const P7: u64 = 0x30;
    let (mut rax, mut r10) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru32(P0)) as u32 as u64;
    r10 = P1;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    rax = P3;
    crate::simd_ops::wu64(rcx, r10);
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    crate::simd_ops::wu64(rcx, rax);
    rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64;
    crate::simd_ops::wu128(rcx.wrapping_add(P4), (xmm0) as u128);
    xmm0 = (crate::simd_ops::ru128(r8)) as u128;
    crate::simd_ops::wu32(rcx.wrapping_add(P5), ((rax as u32)) as u32);
    rax = rcx;
    crate::simd_ops::wu128(rcx.wrapping_add(P6), (xmm0) as u128);
    crate::simd_ops::wu8(rcx.wrapping_add(P7), ((r9 as u8)) as u8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142526ef0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143d0160c; const P1: u64 = 0x143422e38; const P2: u64 = 0x8; const P3: u64 = 0x143422a00; const P4: u64 = 0x38; const P5: u64 = 0xc; const P6: u64 = 0x20; const P7: u64 = 0x28; const P8: u64 = 0x30;
    let (mut rax, mut r10) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru32(P0)) as u32 as u64;
    r10 = P1;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    rax = P3;
    crate::simd_ops::wu64(rcx, r10);
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    crate::simd_ops::wu64(rcx, rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rax);
    rax = rcx;
    crate::simd_ops::wu128(rcx.wrapping_add(P5), (xmm0) as u128);
    crate::simd_ops::wu8(rcx.wrapping_add(P6), (0x0u8) as u8);
    crate::simd_ops::wu32(rcx.wrapping_add(P7), ((r8 as u32)) as u32);
    crate::simd_ops::wu64(rcx.wrapping_add(P8), r9);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142527030(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143d0173c; const P1: u64 = 0x143422e38; const P2: u64 = 0x8; const P3: u64 = 0x143422730; const P4: u64 = 0xc; const P5: u64 = 0x20; const P6: u64 = 0x30;
    let (mut rax, mut r10) = (crate::simd_ops::get_rax_in(), 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru32(P0)) as u32 as u64;
    r10 = P1;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    rax = P3;
    crate::simd_ops::wu64(rcx, r10);
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    crate::simd_ops::wu64(rcx, rax);
    rax = rcx;
    crate::simd_ops::wu128(rcx.wrapping_add(P4), (xmm0) as u128);
    xmm0 = (crate::simd_ops::ru128(r8)) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P5), (xmm0) as u128);
    xmm0 = (crate::simd_ops::ru128(r9)) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P6), (xmm0) as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142527080(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143d015e0; const P1: u64 = 0x143422e38; const P2: u64 = 0x8; const P3: u64 = 0x143422d70; const P4: u64 = 0xc; const P5: u64 = 0x20; const P6: u64 = 0x28; const P7: u64 = 0x30;
    let (mut rax, mut r10) = (crate::simd_ops::get_rax_in(), 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru32(P0)) as u32 as u64;
    r10 = P1;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    rax = P3;
    crate::simd_ops::wu64(rcx, r10);
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    crate::simd_ops::wu64(rcx, rax);
    crate::simd_ops::wu128(rcx.wrapping_add(P4), (xmm0) as u128);
    crate::simd_ops::wu8(rcx.wrapping_add(P5), (0x0u8) as u8);
    rax = (crate::simd_ops::ru64(r8)) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P6), rax);
    rax = rcx;
    crate::simd_ops::wu64(rcx.wrapping_add(P7), r9);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425270c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143d0158c; const P1: u64 = 0x143422e38; const P2: u64 = 0x8; const P3: u64 = 0x143422e10; const P4: u64 = 0xc; const P5: u64 = 0x20; const P6: u64 = 0x28;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru32(P0)) as u32 as u64;
    r9 = P1;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    rax = P3;
    crate::simd_ops::wu64(rcx, r9);
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    crate::simd_ops::wu64(rcx, rax);
    rax = rcx;
    crate::simd_ops::wu128(rcx.wrapping_add(P4), (xmm0) as u128);
    crate::simd_ops::wu8(rcx.wrapping_add(P5), (0x0u8) as u8);
    crate::simd_ops::wu64(rcx.wrapping_add(P6), r8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142527100(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143d01564; const P1: u64 = 0x143422e38; const P2: u64 = 0x8; const P3: u64 = 0x1434228c0; const P4: u64 = 0xc; const P5: u64 = 0x20; const P6: u64 = 0x28; const P7: u64 = 0x30; const P8: u64 = 0x40; const P9: u64 = 0x60; const P10: u64 = 0x68; const P11: u64 = 0x50;
    let (mut rax, mut r10) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru32(P0)) as u32 as u64;
    r10 = P1;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    rax = P3;
    crate::simd_ops::wu64(rcx, r10);
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    crate::simd_ops::wu64(rcx, rax);
    crate::simd_ops::wu128(rcx.wrapping_add(P4), (xmm0) as u128);
    crate::simd_ops::wu8(rcx.wrapping_add(P5), (0x0u8) as u8);
    rax = (crate::simd_ops::ru64(r8)) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P6), rax);
    xmm0 = (crate::simd_ops::ru128(r9)) as u128;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    crate::simd_ops::wu128(rcx.wrapping_add(P7), (xmm0) as u128);
    xmm0 = (crate::simd_ops::ru128(rax)) as u128;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu128(rcx.wrapping_add(P8), (xmm0) as u128);
    xmm0 = (crate::simd_ops::ru128(rax)) as u128;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P9), rax);
    rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x40u64))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(P10), ((rax as u8)) as u8);
    rax = rcx;
    crate::simd_ops::wu128(rcx.wrapping_add(P11), (xmm0) as u128);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425271c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143d01560; const P1: u64 = 0x143422e38; const P2: u64 = 0x8; const P3: u64 = 0x143422c80; const P4: u64 = 0x30; const P5: u64 = 0xc; const P6: u64 = 0x20; const P7: u64 = 0x28;
    let (mut rax, mut r10) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru32(P0)) as u32 as u64;
    r10 = P1;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    rax = P3;
    crate::simd_ops::wu64(rcx, r10);
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    crate::simd_ops::wu64(rcx, rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rax);
    rax = rcx;
    crate::simd_ops::wu128(rcx.wrapping_add(P5), (xmm0) as u128);
    crate::simd_ops::wu32(rcx.wrapping_add(P6), ((r8 as u32)) as u32);
    crate::simd_ops::wu64(rcx.wrapping_add(P7), r9);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142527210(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143d015dc; const P1: u64 = 0x143422e38; const P2: u64 = 0x8; const P3: u64 = 0x1434226b8; const P4: u64 = 0x30; const P5: u64 = 0x38; const P6: u64 = 0x40; const P7: u64 = 0xc; const P8: u64 = 0x20; const P9: u64 = 0x28;
    let (mut rax, mut r10) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru32(P0)) as u32 as u64;
    r10 = P1;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    rax = P3;
    crate::simd_ops::wu64(rcx, r10);
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    crate::simd_ops::wu64(rcx, rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rax);
    rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x30u64))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(P5), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P6), rax);
    rax = rcx;
    crate::simd_ops::wu128(rcx.wrapping_add(P7), (xmm0) as u128);
    crate::simd_ops::wu64(rcx.wrapping_add(P8), r8);
    crate::simd_ops::wu8(rcx.wrapping_add(P9), ((r9 as u8)) as u8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425272e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143d015c0; const P1: u64 = 0x143422e38; const P2: u64 = 0x8; const P3: u64 = 0x143422be0; const P4: u64 = 0xc; const P5: u64 = 0x20; const P6: u64 = 0x30;
    let (mut rax, mut r10) = (crate::simd_ops::get_rax_in(), 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru32(P0)) as u32 as u64;
    r10 = P1;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    rax = P3;
    crate::simd_ops::wu64(rcx, r10);
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    crate::simd_ops::wu64(rcx, rax);
    rax = rcx;
    crate::simd_ops::wu128(rcx.wrapping_add(P4), (xmm0) as u128);
    xmm0 = (crate::simd_ops::ru128(r8)) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P5), (xmm0) as u128);
    crate::simd_ops::wu32(rcx.wrapping_add(P6), ((r9 as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142527320(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143d01578; const P1: u64 = 0x143422e38; const P2: u64 = 0x8; const P3: u64 = 0x143422668; const P4: u64 = 0x38; const P5: u64 = 0xc; const P6: u64 = 0x20; const P7: u64 = 0x28; const P8: u64 = 0x30;
    let (mut rax, mut r10) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru32(P0)) as u32 as u64;
    r10 = P1;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    rax = P3;
    crate::simd_ops::wu64(rcx, r10);
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    crate::simd_ops::wu64(rcx, rax);
    rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x28u64))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(P4), ((rax as u8)) as u8);
    rax = rcx;
    crate::simd_ops::wu128(rcx.wrapping_add(P5), (xmm0) as u128);
    crate::simd_ops::wu8(rcx.wrapping_add(P6), (0x0u8) as u8);
    crate::simd_ops::wu64(rcx.wrapping_add(P7), r8);
    crate::simd_ops::wu64(rcx.wrapping_add(P8), r9);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142527370(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143d0174c; const P1: u64 = 0x143422e38; const P2: u64 = 0x8; const P3: u64 = 0x143422690; const P4: u64 = 0xc; const P5: u64 = 0x20; const P6: u64 = 0x28; const P7: u64 = 0x48; const P8: u64 = 0x50; const P9: u64 = 0x38; const P10: u64 = 0x64; const P11: u64 = 0x54;
    let (mut rax, mut r10) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru32(P0)) as u32 as u64;
    r10 = P1;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    rax = P3;
    crate::simd_ops::wu64(rcx, r10);
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    crate::simd_ops::wu64(rcx, rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    crate::simd_ops::wu128(rcx.wrapping_add(P4), (xmm0) as u128);
    crate::simd_ops::wu8(rcx.wrapping_add(P5), (0x0u8) as u8);
    xmm0 = (crate::simd_ops::ru128(r8)) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P6), (xmm0) as u128);
    xmm0 = (crate::simd_ops::ru128(r9)) as u128;
    crate::simd_ops::wu64(rcx.wrapping_add(P7), rax);
    rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x30u64))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(P8), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    crate::simd_ops::wu128(rcx.wrapping_add(P9), (xmm0) as u128);
    xmm0 = (crate::simd_ops::ru128(rax)) as u128;
    rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x40u64))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(P10), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    crate::simd_ops::wu128(rcx.wrapping_add(P11), (xmm0) as u128);
    xmm0 = (crate::simd_ops::ru128(rax)) as u128;
    rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x50u64))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(0x78u64), ((rax as u8)) as u8);
    rax = rcx;
    crate::simd_ops::wu128(rcx.wrapping_add(0x68u64), (xmm0) as u128);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142527440(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143d015e8; const P1: u64 = 0x143422e38; const P2: u64 = 0x8; const P3: u64 = 0x143422ca8; const P4: u64 = 0xc; const P5: u64 = 0x20; const P6: u64 = 0x28; const P7: u64 = 0x48; const P8: u64 = 0x38;
    let (mut rax, mut r10) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru32(P0)) as u32 as u64;
    r10 = P1;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    rax = P3;
    crate::simd_ops::wu64(rcx, r10);
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    crate::simd_ops::wu64(rcx, rax);
    rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64;
    crate::simd_ops::wu128(rcx.wrapping_add(P4), (xmm0) as u128);
    crate::simd_ops::wu8(rcx.wrapping_add(P5), (0x0u8) as u8);
    xmm0 = (crate::simd_ops::ru128(r8)) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P6), (xmm0) as u128);
    xmm0 = (crate::simd_ops::ru128(r9)) as u128;
    crate::simd_ops::wu32(rcx.wrapping_add(P7), ((rax as u32)) as u32);
    rax = rcx;
    crate::simd_ops::wu128(rcx.wrapping_add(P8), (xmm0) as u128);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142527490(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    r9 = 0x811c9dc5;
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax = ((r9 as u32)) as u32 as u64; return rax; }
    loop { r8 = ((crate::simd_ops::ru8(rcx.wrapping_add(rax))) as u32) as u32 as u64; rax = ((rax).wrapping_add(1)) as u64; r8 = ((((r8 as u32)) ^ ((r9 as u32)))) as u32 as u64; r9 = ((((r8 as u32)) as i32).wrapping_mul((0x1000193u32) as i32) as u32) as u32 as u64; fl_l = rax; fl_r = rdx; if !((((fl_l as u64)) < ((fl_r as u64)))) { break; } }
    rax = ((r9 as u32)) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142527500(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1405eb9e0; const P1: u64 = 0x142646840; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x14129dff0; const P5: u64 = 0x1405eb9f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r14 = r8; r15 = rdx; rbx = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); r9 = r15; r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (0x0u64) as u64); rdx = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(rax)) as u64; rcx = (((rcx) | (0x2u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rbp = ((rax as u32)) as u32 as u64; rax = 0xffffffff; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; rbp = (if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { (rax as u32) } else { (rbp as u32) }) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rsi); rsi = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rdx = ((((rbp as u32)) as i32 as i64) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rdi); rcx = ((rdx.wrapping_add(rsi)) as u64) as u64; fl_l = rcx; fl_r = rsi; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 5; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rbx.wrapping_add(P2), rcx); rax = rbx; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_r = (0xfu64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx)) as u64; bb = 4; continue; }
            4 => { crate::simd_ops::wu8(rax.wrapping_add(rcx), (0x0u8) as u8); bb = 6; continue; }
            5 => { r8 = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_r = (0xfu64) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rbx)) as u64; bb = 8; continue; }
            8 => { rax = ((rbp.wrapping_add(0x1u64)) as u32) as u32 as u64; r9 = r14; rdx = ((((rax as u32)) as i32 as i64) as u64) as u64; rcx = ((rbx.wrapping_add(rsi)) as u64) as u64; r8 = r15; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425275d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x18; const P2: u64 = 0x142527500;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1425275d0, funcinfo_rva: 0x37fa57c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x38, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = (rcx) as u64;
    rax = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32);
    xmm0 = 0u128;
    crate::simd_ops::wu128(rcx, (xmm0) as u128);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), (rax) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), (0xfu64) as u64);
    crate::simd_ops::wu8(rcx, ((rax as u8)) as u8);
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x142527601); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    rax = (rbx) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142527620(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x18; const P2: u64 = 0x142527500;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x142527620, funcinfo_rva: 0x37fa5a8, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x38, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (r8) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r9) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = (rcx) as u64;
    rax = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32);
    xmm0 = 0u128;
    crate::simd_ops::wu128(rcx, (xmm0) as u128);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), (rax) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), (0xfu64) as u64);
    crate::simd_ops::wu8(rcx, ((rax as u8)) as u8);
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32);
    r8 = ((rsp.wrapping_add(0x50u64)) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x142527665); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    rax = (rbx) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142527680(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0xc; const P2: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if (((fl_l as u32)) != ((fl_r as u32))) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
    rax = (crate::simd_ops::ru32(rdx)) as u32 as u64;
    fl_l = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if (((fl_l as u32)) != ((fl_r as u32))) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if (((fl_l as u32)) != ((fl_r as u32))) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425276f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x8; const P2: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(rcx, rax);
    rax = (crate::simd_ops::ru32(rdx)) as u32 as u64;
    fl_l = (((r8 as u32)).wrapping_add(crate::simd_ops::ru32(rdx.wrapping_add(P0)))) as u32 as u64; fl_r = fl_l;
    r8 = ((fl_l as u32)) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32);
    rax = rcx;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((r8 as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142527710(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(rcx, rax);
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142527730(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if (((fl_l as u32)) != ((fl_r as u32))) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
    rax = (crate::simd_ops::ru32(rdx)) as u32 as u64;
    fl_l = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142527760(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1; const P1: u64 = 0x2; const P2: u64 = 0x3;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu8(rcx, ((rdx as u8)) as u8); rax = 0x1; return rax; }
            2 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x800; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = ((rdx as u32)) as u32 as u64; rdx = (rdx & !0xffu64) | ((((((rdx as u8)) & (0x3fu8))) as u8 as u64)); rax = (((rax as u32)).wrapping_shr(6u32)) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) | (0xc0u8))) as u8 as u64)); fl_l = ((((rdx as u8)) | (0x80u8))) as u8 as u64; fl_r = fl_l; rdx = (rdx & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rcx, ((rax as u8)) as u8); crate::simd_ops::wu8(rcx.wrapping_add(P0), ((rdx as u8)) as u8); rax = 0x2; return rax; }
            4 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x10000; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = ((rdx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(12u32)) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) | (0xe0u8))) as u8 as u64)); crate::simd_ops::wu8(rcx, ((rax as u8)) as u8); rax = ((rdx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(6u32)) as u32 as u64; rdx = (rdx & !0xffu64) | ((((((rdx as u8)) & (0x3fu8))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x3fu8))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) | (0x80u8))) as u8 as u64)); fl_l = ((((rdx as u8)) | (0x80u8))) as u8 as u64; fl_r = fl_l; rdx = (rdx & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rcx.wrapping_add(P0), ((rax as u8)) as u8); crate::simd_ops::wu8(rcx.wrapping_add(P1), ((rdx as u8)) as u8); rax = 0x3; return rax; }
            6 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x110000; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = ((rdx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(18u32)) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) | (0xf0u8))) as u8 as u64)); crate::simd_ops::wu8(rcx, ((rax as u8)) as u8); rax = ((rdx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(12u32)) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x3fu8))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) | (0x80u8))) as u8 as u64)); crate::simd_ops::wu8(rcx.wrapping_add(P0), ((rax as u8)) as u8); rax = ((rdx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(6u32)) as u32 as u64; rdx = (rdx & !0xffu64) | ((((((rdx as u8)) & (0x3fu8))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x3fu8))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) | (0x80u8))) as u8 as u64)); fl_l = ((((rdx as u8)) | (0x80u8))) as u8 as u64; fl_r = fl_l; rdx = (rdx & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rcx.wrapping_add(P1), ((rax as u8)) as u8); crate::simd_ops::wu8(rcx.wrapping_add(P2), ((rdx as u8)) as u8); rax = 0x4; return rax; }
            8 => { rax = 0; fl_l = 0; fl_r = 0; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142527800(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rbx = rdx; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; r11 = 0x1; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(0x14u64))) as u32 as u64; r14 = ((rdx as u32)) as u32 as u64; r14 = (((r14 as u32)).wrapping_sub(crate::simd_ops::ru32(rcx.wrapping_add(0x18u64)))) as u32 as u64; fl_l = rdx; fl_r = rsi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 13; continue; } bb = 1; continue; }
            1 => { r9 = (crate::simd_ops::ru64(rcx)) as u64; rax = r9; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r9))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 3; continue; } bb = 2; continue; }
            2 => { r8 = ((crate::simd_ops::ru8(r9.wrapping_add(rdx))) as u32) as u32 as u64; bb = 14; continue; }
            3 => { rax = ((crate::simd_ops::ru8(r9.wrapping_add(rdx))) as u32) as u32 as u64; r8 = r9; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xe0u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xc0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 4; continue; }
            4 => { r8 = ((crate::simd_ops::ru8(r9.wrapping_add(rdx))) as u32) as u32 as u64; rdi = 0x2; fl_l = ((((r8 as u32)) & (0x1fu32))) as u32 as u64; fl_r = fl_l; r8 = ((fl_l as u32)) as u32 as u64; r10 = r9; bb = 16; continue; }
            5 => { rax = ((crate::simd_ops::ru8(r9.wrapping_add(rdx))) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xf0u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xe0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 6; continue; }
            6 => { r8 = ((crate::simd_ops::ru8(rdx.wrapping_add(r9))) as u32) as u32 as u64; rdi = 0x3; fl_l = ((((r8 as u32)) & (0xfu32))) as u32 as u64; fl_r = fl_l; r8 = ((fl_l as u32)) as u32 as u64; r10 = r9; bb = 16; continue; }
            7 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(r9))) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xf8u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xf0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 8; continue; }
            8 => { r8 = ((crate::simd_ops::ru8(r9.wrapping_add(rdx))) as u32) as u32 as u64; rdi = 0x4; fl_l = ((((r8 as u32)) & (0x7u32))) as u32 as u64; fl_r = fl_l; r8 = ((fl_l as u32)) as u32 as u64; r10 = r9; bb = 16; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r9))) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 11; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; rax = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x18u64), ((rax as u32)) as u32); bb = 11; continue; }
            11 => { fl_l = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rdx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), ((rdx as u32)) as u32); bb = 12; continue; }
            12 => { rdx = (((rdx as u32)).wrapping_sub(crate::simd_ops::ru32(rcx.wrapping_add(0x18u64)))) as u32 as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x14u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0xcu64), ((rdx as u32)) as u32); crate::simd_ops::wu32(rsp, ((rbp as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x4u64), ((r14 as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rsp)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbx.wrapping_add(0x4u64), (xmm0) as u128); crate::simd_ops::wu32(rbx, (0x11cu32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x14u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rax); bb = 24; continue; }
            13 => { r8 = (r8 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 14; continue; }
            14 => { r10 = (crate::simd_ops::ru64(rcx)) as u64; r8 = ((((r8 as u32)) & (0x7fu32))) as u32 as u64; rdi = ((r11 as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru8(r10.wrapping_add(rdx))) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rax = ((rbp.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x14u64), ((rax as u32)) as u32); rax = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x18u64), ((rax as u32)) as u32); bb = 16; continue; }
            16 => { rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), ((rdx as u32)) as u32); fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 23; continue; } bb = 17; continue; }
            17 => { bb = 18; continue; }
            18 => { r9 = ((rdx as u32)) as u32 as u64; fl_l = r9; fl_r = rsi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 12; continue; } bb = 19; continue; }
            19 => { rax = ((crate::simd_ops::ru8(r10.wrapping_add(rdx))) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xc0u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 12; continue; } bb = 20; continue; }
            20 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(r10))) as u32) as u32 as u64; rax = ((((rax as u32)) & (0x3fu32))) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(6u32)) as u32 as u64; r8 = ((((r8 as u32)) | ((rax as u32)))) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r10))) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 22; continue; } bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; rax = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x18u64), ((rax as u32)) as u32); bb = 22; continue; }
            22 => { rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; r11 = (((r11 as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), ((rdx as u32)) as u32); fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 18; continue; } bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x14u64))) as u32 as u64; rdx = (((rdx as u32)).wrapping_sub(crate::simd_ops::ru32(rcx.wrapping_add(0x18u64)))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xcu64), ((rax as u32)) as u32); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x18u64), ((r8 as u32)) as u32); crate::simd_ops::wu32(rsp, (0x11cu32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x4u64), ((rbp as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x8u64), ((r14 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x10u64), ((rdx as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x14u64), ((rax as u32)) as u32); let y_addr: u64 = rsp; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); bb = 24; continue; }
            24 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425285c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1425292a0; const P1: u64 = 0x141470f70;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; rbx = rdx; r10 = (crate::simd_ops::ru64(rcx)) as u64; r8 = ((rdi as u32)) as u32 as u64; r11 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; r9 = rcx; bb = 1; continue; }
            1 => { rax = ((r8 as u32)) as u32 as u64; rcx = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; rdx = ((r8 as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r10))) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(r9.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(0x18u64), ((rcx as u32)) as u32); bb = 3; continue; }
            3 => { r8 = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(0x10u64), ((r8 as u32)) as u32); fl_l = r8; fl_r = r11; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(r10))) as u32) as u32 as u64; bb = 6; continue; }
            5 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { rax = ((((rax as u8)) as i8 as i32) as u32) as u32 as u64; rax = ((((rax as u32)) | (0x20u32))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(0x61u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1a; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 1; continue; } bb = 7; continue; }
            7 => { fl_l = rcx; fl_r = r11; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(r10))) as u32) as u32 as u64; bb = 10; continue; }
            9 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 10; continue; }
            10 => { rax = ((((rax as u8)) as i8 as i32) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_sub(0x30u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xa; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 1; continue; } bb = 11; continue; }
            11 => { rax = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = rcx; fl_r = r11; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(r10))) as u8 as u64; fl_r = (0x5fu8) as u8 as u64; r8 = ((rax as u32)) as u32 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 1; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(r9.wrapping_add(0x50u64))) as u64; rdx = ((r10.wrapping_add(rdi)) as u64) as u64; rax = (((rax as u32)).wrapping_sub((rdi as u32))) as u32 as u64; fl_l = (crate::simd_ops::ru8(r9.wrapping_add(0x59u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 15; continue; } bb = 14; continue; }
            14 => { r8 = rdx; r9 = ((rax as u32)) as u32 as u64; rdx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 19; continue; }
            15 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); rdi = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), ((rdi as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); bb = 18; continue; }
            17 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x117u32) as u32); bb = 18; continue; }
            18 => { rax = rsp.wrapping_add(0x20); bb = 19; continue; }
            19 => { xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425286d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x10u64)) as u64; rdi = (crate::simd_ops::ru64(rcx)) as u64; r14 = r8; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rsi = rdx; r11 = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; r10 = rcx; rbp = ((r9 as u32)) as u32 as u64; bb = 1; continue; }
            1 => { rdx = ((r11 as u32)) as u32 as u64; rax = ((r11.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(rdi))) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(r10.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(0x18u64), ((rax as u32)) as u32); bb = 3; continue; }
            3 => { rdx = ((rax as u32)) as u32 as u64; r11 = ((rax as u32)) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(0x10u64), ((rax as u32)) as u32); fl_l = rdx; fl_r = rbx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = ((crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u32) as u32 as u64; r8 = ((rax.wrapping_add(rdi)) as u64) as u64; bb = 6; continue; }
            5 => { rcx = (rcx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; r8 = ((rdi.wrapping_add(rax)) as u64) as u64; bb = 6; continue; }
            6 => { rcx = ((((rcx as u8)) as i8 as i32) as u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub(0x30u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0xa; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 1; continue; } bb = 7; continue; }
            7 => { fl_l = rdx; fl_r = rbx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 19; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(r8)) as u8 as u64; fl_r = (0x2eu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 1; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(r8)) as u8 as u64; fl_r = (0x5fu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 1; continue; } bb = 10; continue; }
            10 => { fl_l = rdx; fl_r = rbx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 19; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u8 as u64; fl_r = (0x65u8) as u8 as u64; r8 = rdi; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 13; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u8 as u64; fl_r = (0x45u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 20; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 15; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::lk_add_u32(r10.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(0x18u64), ((rcx as u32)) as u32); bb = 15; continue; }
            15 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(0x10u64), ((rax as u32)) as u32); fl_l = rcx; fl_r = rbx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 20; continue; } bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u8 as u64; fl_r = (0x2bu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 32; continue; } bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u8 as u64; fl_r = (0x2du8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 20; continue; } bb = 18; continue; }
            18 => { fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(0x10u64), ((rax as u32)) as u32); bb = 20; continue; }
            19 => { r8 = rdi; bb = 20; continue; }
            20 => { rdx = ((rax as u32)) as u32 as u64; fl_l = rdx; fl_r = rbx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 22; continue; } bb = 21; continue; }
            21 => { rcx = ((crate::simd_ops::ru8(rdi.wrapping_add(rax))) as u32) as u32 as u64; bb = 23; continue; }
            22 => { rcx = (rcx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 23; continue; }
            23 => { rcx = ((((rcx as u8)) as i8 as i32) as u32) as u32 as u64; rcx = ((((rcx as u32)) | (0x20u32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub(0x61u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1a; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 30; continue; } bb = 24; continue; }
            24 => { fl_l = rdx; fl_r = rbx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 26; continue; } bb = 25; continue; }
            25 => { rcx = ((crate::simd_ops::ru8(rax.wrapping_add(r8))) as u32) as u32 as u64; bb = 27; continue; }
            26 => { rcx = (rcx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 27; continue; }
            27 => { rcx = ((((rcx as u8)) as i8 as i32) as u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub(0x30u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0xa; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 30; continue; } bb = 28; continue; }
            28 => { fl_l = rdx; fl_r = rbx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 33; continue; } bb = 29; continue; }
            29 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r8))) as u8 as u64; fl_r = (0x5fu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 33; continue; } bb = 30; continue; }
            30 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r8))) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 32; continue; } bb = 31; continue; }
            31 => { fl_l = (crate::simd_ops::lk_add_u32(r10.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(0x18u64), ((rcx as u32)) as u32); bb = 32; continue; }
            32 => { fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(0x10u64), ((rax as u32)) as u32); bb = 20; continue; }
            33 => { rcx = (crate::simd_ops::ru64(r14)) as u64; r8 = ((rax as u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_sub(crate::simd_ops::ru32(r10.wrapping_add(0x18u64)))) as u32 as u64; rdx = (crate::simd_ops::ru32(r10.wrapping_add(0x14u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub((rbp as u32))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x14u64), ((rax as u32)) as u32); rax = ((rdi.wrapping_add(rbp)) as u64) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x18u64), rax); rax = rsi; crate::simd_ops::wu64(rsp, rcx); crate::simd_ops::wu32(rsp.wrapping_add(0x8u64), ((rdx as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0xcu64), ((r8 as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rsp)) as u128; crate::simd_ops::wu32(rsi, (0x116u32) as u32); crate::simd_ops::wu128(rsi.wrapping_add(0x4u64), (xmm0) as u128); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x10u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142528860(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142528ca0; const P1: u64 = 0x1403952e0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r9 as u32)) as u32); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rbp = rsp; rsp = ((rsp).wrapping_sub(0x50u64)) as u64; r14 = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; rbx = r8; rsi = rdx; rdi = rcx; r11 = 0x2401; bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; r9 = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = r9; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 39; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdi)) as u64; r8 = rax; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rax))) as u8 as u64; fl_r = (0x60u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 36; continue; } bb = 3; continue; }
            3 => { r10 = (((crate::simd_ops::ru8(rax.wrapping_add(rcx))) as i8 as i32) as u32) as u32 as u64; rdx = ((r10 as u32)) as u32 as u64; fl_l = ((r10 as u8)) as u8 as u64; fl_r = (0xdu8) as u8 as u64; if (((fl_l as u8)) > ((fl_r as u8))) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = ((((r11 as u32)) >> ((((rdx as u32)) as u32 & 31))) & 1) as u64; if ((fl_l as u32) != 0) { bb = 39; continue; } bb = 5; continue; }
            5 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x5cu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 6; continue; }
            6 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x7bu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 22; continue; } bb = 7; continue; }
            7 => { fl_l = ((r10 as u8)) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x18u64), ((rax as u32)) as u32); bb = 9; continue; }
            9 => { rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), ((rax as u32)) as u32); bb = 1; continue; }
            10 => { rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; fl_l = rdx; fl_r = r9; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 21; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru8(r8.wrapping_add(rax))) as u8 as u64; fl_r = (0x75u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 21; continue; } bb = 12; continue; }
            12 => { rdx = ((rcx.wrapping_add(0x2u64)) as u32) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; fl_l = rax; fl_r = r9; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 21; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru8(r8.wrapping_add(rdx))) as u8 as u64; fl_r = (0x7bu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 21; continue; } bb = 14; continue; }
            14 => { fl_l = ((r10 as u8)) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 16; continue; } bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x18u64), ((rax as u32)) as u32); bb = 16; continue; }
            16 => { rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), ((rax as u32)) as u32); fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r8))) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 18; continue; } bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x18u64), ((rdx as u32)) as u32); bb = 18; continue; }
            18 => { rax = ((rdx as u32)) as u32 as u64; rcx = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), ((rax as u32)) as u32); fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 20; continue; } bb = 19; continue; }
            19 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x18u64), ((rcx as u32)) as u32); bb = 20; continue; }
            20 => { crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), ((rcx as u32)) as u32); bb = 1; continue; }
            21 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 1; continue; }
            22 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x68u64))) as u64; rcx = rdi.wrapping_add(0x60); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbp.wrapping_add(0x38u64), ((rax as u32)) as u32); fl_l = rdx; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x70u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 24; continue; } bb = 23; continue; }
            23 => { crate::simd_ops::wu32(rdx, ((rax as u32)) as u32); fl_l = (crate::simd_ops::lk_add_u64(rdi.wrapping_add(0x68u64), (0x4u64) as u64)) as u64; fl_r = fl_l; bb = 25; continue; }
            24 => { r8 = rbp.wrapping_add(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 25; continue; }
            25 => { r9 = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; rdx = ((r9.wrapping_add(0x1u64)) as u32) as u32 as u64; r10 = ((rdx as u32)) as u32 as u64; fl_l = r10; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 32; continue; } bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(rdi)) as u64; r8 = rax; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(rax))) as u8 as u64; fl_r = (0x7bu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 33; continue; } bb = 27; continue; }
            27 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32 as u64; rax = ((r9 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_sub(crate::simd_ops::ru32(rdi.wrapping_add(0x18u64)))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x24u64), ((rax as u32)) as u32); rax = ((r9 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_sub((r14 as u32))) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), rbx); fl_l = (crate::simd_ops::ru8(r9.wrapping_add(r8))) as u8 as u64; fl_r = (0xau8) as u8 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0xcu64), ((rax as u32)) as u32); rax = ((r8.wrapping_add(r14)) as u64) as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x28u64), ((rcx as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x30u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x8u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x20u64), (0x11du32) as u32); crate::simd_ops::wu128(rbp.wrapping_sub(0x1cu64), (xmm0) as u128); if (((fl_l as u8)) != ((fl_r as u8))) { bb = 29; continue; } bb = 28; continue; }
            28 => { fl_l = (((rcx as u32)).wrapping_add(1)) as u32 as u64; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x18u64), ((rdx as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0x14u64), ((rcx as u32)) as u32); bb = 29; continue; }
            29 => { crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), ((rdx as u32)) as u32); rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; fl_l = (crate::simd_ops::ru8(r8.wrapping_add(r10))) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 31; continue; } bb = 30; continue; }
            30 => { rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x18u64), ((rdx as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0x14u64), ((rax as u32)) as u32); bb = 31; continue; }
            31 => { let y_addr: u64 = rbp.wrapping_sub(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsi; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), ((rdx as u32)) as u32); bb = 41; continue; }
            32 => { r8 = (crate::simd_ops::ru64(rdi)) as u64; bb = 33; continue; }
            33 => { fl_l = (crate::simd_ops::ru8(r9.wrapping_add(r8))) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 35; continue; } bb = 34; continue; }
            34 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x18u64), ((rdx as u32)) as u32); bb = 35; continue; }
            35 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32 as u64; rcx = ((rdx as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub(crate::simd_ops::ru32(rdi.wrapping_add(0x18u64)))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), ((rdx as u32)) as u32); rdx = (((rdx as u32)).wrapping_sub((r14 as u32))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x28u64), ((rax as u32)) as u32); fl_l = (((rdx as u32)).wrapping_sub(1)) as u32 as u64; rdx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x24u64), ((rcx as u32)) as u32); rax = ((r8.wrapping_add(r14)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), rbx); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x30u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x1cu64), (xmm0) as u128); crate::simd_ops::wu32(rbp.wrapping_sub(0x20u64), (0x109u32) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0xcu64), ((rdx as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x8u64), rax); let y_addr: u64 = rbp.wrapping_sub(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsi; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); bb = 41; continue; }
            36 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(rcx))) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 38; continue; } bb = 37; continue; }
            37 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x18u64), ((rax as u32)) as u32); bb = 38; continue; }
            38 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32 as u64; rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), ((rdx as u32)) as u32); rcx = (((rcx as u32)).wrapping_sub(crate::simd_ops::ru32(rdi.wrapping_add(0x18u64)))) as u32 as u64; rdx = (((rdx as u32)).wrapping_sub((r14 as u32))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x28u64), ((rax as u32)) as u32); fl_l = (((rdx as u32)).wrapping_sub(1)) as u32 as u64; rdx = ((fl_l as u32)) as u32 as u64; rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x40u64))) as u32 as u64; crate::simd_ops::wu32(rsi, ((rax as u32)) as u32); rax = ((r8.wrapping_add(r14)) as u64) as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x14u64), ((rdx as u32)) as u32); bb = 40; continue; }
            39 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub(crate::simd_ops::ru32(rdi.wrapping_add(0x18u64)))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x28u64), ((rax as u32)) as u32); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsi.wrapping_add(0x14u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsi, (0x11au32) as u32); bb = 40; continue; }
            40 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), rbx); crate::simd_ops::wu32(rbp.wrapping_sub(0x24u64), ((rcx as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x30u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsi.wrapping_add(0x4u64), (xmm0) as u128); crate::simd_ops::wu64(rsi.wrapping_add(0x18u64), rax); bb = 41; continue; }
            41 => { rax = rsi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142528ae0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x14; const P2: u64 = 0x18; const P3: u64 = 0x1; const P4: u64 = 0x142528860;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rbx = rdx;
    r9 = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    rax = ((r8 as u32)) as u32 as u64;
    rax = (((rax as u32)).wrapping_sub(crate::simd_ops::ru32(rcx.wrapping_add(P2)))) as u32 as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rdx = ((r8.wrapping_add(P3)) as u32) as u32 as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((r9 as u32)) as u32);
    fl_l = (crate::simd_ops::ru8(r8.wrapping_add(rax))) as u8 as u64; fl_r = (0xau8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = ((r9.wrapping_add(P3)) as u32) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rdx as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32); }
    r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rdx as u32)) as u32);
    rdx = rbx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x10cu32) as u32);
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142528b40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142528ca0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r8 = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; r11 = rdx; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(0x14u64))) as u32 as u64; rbp = ((r8 as u32)) as u32 as u64; rbp = (((rbp as u32)).wrapping_sub(crate::simd_ops::ru32(rcx.wrapping_add(0x18u64)))) as u32 as u64; fl_l = r8; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rbx = ((crate::simd_ops::ru8(r8.wrapping_add(rax))) as u32) as u32 as u64; bb = 3; continue; }
            2 => { rbx = (rbx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = (crate::simd_ops::ru8(r8.wrapping_add(rax))) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = ((rsi.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x14u64), ((rax as u32)) as u32); rax = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x18u64), ((rax as u32)) as u32); bb = 5; continue; }
            5 => { rdi = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; r14 = 0x2401; crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), ((rdi as u32)) as u32); bb = 6; continue; }
            6 => { r8 = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = r8; fl_r = r9; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((crate::simd_ops::ru8(r8.wrapping_add(rax))) as u32) as u32 as u64; bb = 9; continue; }
            8 => { rdx = (rdx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 21; continue; } bb = 10; continue; }
            10 => { fl_l = r8; fl_r = r9; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((crate::simd_ops::ru8(r8.wrapping_add(rax))) as u32) as u32 as u64; bb = 13; continue; }
            12 => { rdx = (rdx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 13; continue; }
            13 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0xdu8) as u8 as u64; if (((fl_l as u8)) > ((fl_r as u8))) { bb = 15; continue; } bb = 14; continue; }
            14 => { fl_l = ((((r14 as u32)) >> ((((rdx as u32)) as u32 & 31))) & 1) as u64; if ((fl_l as u32) != 0) { bb = 20; continue; } bb = 15; continue; }
            15 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x5cu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 19; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = (crate::simd_ops::ru8(r8.wrapping_add(rax))) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 18; continue; } bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; rax = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x18u64), ((rax as u32)) as u32); bb = 18; continue; }
            18 => { rax = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), ((rax as u32)) as u32); bb = 6; continue; }
            19 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            20 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x14u64))) as u32 as u64; r8 = (((r8 as u32)).wrapping_sub(crate::simd_ops::ru32(rcx.wrapping_add(0x18u64)))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(r11.wrapping_add(0x14u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), ((r8 as u32)) as u32); crate::simd_ops::wu32(r11, (0x11au32) as u32); bb = 24; continue; }
            21 => { r9 = (crate::simd_ops::ru64(rcx)) as u64; fl_l = (crate::simd_ops::ru8(r9.wrapping_add(r8))) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 23; continue; } bb = 22; continue; }
            22 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; rax = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x18u64), ((rax as u32)) as u32); bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x14u64))) as u32 as u64; r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; rdx = ((r8 as u32)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), ((r8 as u32)) as u32); rdx = (((rdx as u32)).wrapping_sub(crate::simd_ops::ru32(rcx.wrapping_add(0x18u64)))) as u32 as u64; r8 = (((r8 as u32)).wrapping_sub((rdi as u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); r8 = (((r8 as u32)).wrapping_sub(1)) as u32 as u64; rax = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu32(r11.wrapping_add(0x14u64), ((r8 as u32)) as u32); fl_l = ((rax).wrapping_add(r9)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), ((rdx as u32)) as u32); crate::simd_ops::wu32(r11, (0x115u32) as u32); bb = 24; continue; }
            24 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rsi as u32)) as u32); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((rbp as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(r11.wrapping_add(0x18u64), rax); rax = r11; crate::simd_ops::wu128(r11.wrapping_add(0x4u64), (xmm0) as u128); rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142528ca0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; r9 = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r9))) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x18u64), ((rdx as u32)) as u32); bb = 2; continue; }
            2 => { r10 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rax = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), ((rdx as u32)) as u32); fl_l = rax; fl_r = r10; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 27; continue; } bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(r9))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 27; continue; } bb = 4; continue; }
            4 => { rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xdu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 22; continue; } bb = 5; continue; }
            5 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x7au8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 9; continue; } bb = 6; continue; }
            6 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x18u64), ((rdx as u32)) as u32); bb = 8; continue; }
            8 => { crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), ((rdx as u32)) as u32); return rax; }
            9 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 11; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x18u64), ((rdx as u32)) as u32); bb = 11; continue; }
            11 => { r8 = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), ((rdx as u32)) as u32); fl_l = r8; fl_r = r10; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 27; continue; } bb = 12; continue; }
            12 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(r9))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x20u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 19; continue; } bb = 13; continue; }
            13 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x9u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 19; continue; } bb = 14; continue; }
            14 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xdu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 19; continue; } bb = 15; continue; }
            15 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 20; continue; } bb = 16; continue; }
            16 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xbu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 21; continue; } bb = 17; continue; }
            17 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xcu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 27; continue; } bb = 18; continue; }
            18 => { fl_l = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rdx = ((fl_l as u32)) as u32 as u64; bb = 11; continue; }
            19 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 21; continue; } bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; rax = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x18u64), ((rax as u32)) as u32); bb = 21; continue; }
            21 => { fl_l = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rdx = ((fl_l as u32)) as u32 as u64; bb = 11; continue; }
            22 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 24; continue; } bb = 23; continue; }
            23 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x18u64), ((rdx as u32)) as u32); bb = 24; continue; }
            24 => { rax = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), ((rdx as u32)) as u32); fl_l = rax; fl_r = r10; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 27; continue; } bb = 25; continue; }
            25 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r9))) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 27; continue; } bb = 26; continue; }
            26 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; rax = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x18u64), ((rax as u32)) as u32); rax = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), ((rax as u32)) as u32); bb = 27; continue; }
            27 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142528d70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x10u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; r15 = r8; r11 = (crate::simd_ops::ru64(rcx)) as u64; rsi = rdx; r10 = rcx; r14 = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r11))) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x18u64), ((r14 as u32)) as u32); bb = 2; continue; }
            2 => { rdi = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rax = ((r14 as u32)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), ((r14 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); bb = 3; continue; }
            3 => { rcx = ((rax as u32)) as u32 as u64; fl_l = rcx; fl_r = rdi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 22; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(r11))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 22; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(r11))) as u8 as u64; fl_r = (0x5du8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 19; continue; } bb = 6; continue; }
            6 => { rbx = ((crate::simd_ops::ru8(rcx.wrapping_add(r11))) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; bb = 7; continue; }
            7 => { rcx = ((rax as u32)) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(0x10u64), ((rax as u32)) as u32); r8 = ((rcx as u32)) as u32 as u64; fl_l = rcx; fl_r = rdi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(r11.wrapping_add(rax))) as u8 as u64; fl_r = (0x3du8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rdx = ((fl_l as u32)) as u32 as u64; bb = 7; continue; }
            10 => { fl_l = r8; fl_r = rdi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { r8 = ((crate::simd_ops::ru8(rax.wrapping_add(r11))) as u32) as u32 as u64; bb = 13; continue; }
            12 => { r8 = (r8 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 13; continue; }
            13 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rdx = (!((rdx as u32))) as u32 as u64; bb = 15; continue; }
            15 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r11))) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 18; continue; } bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::lk_add_u32(r10.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(0x18u64), ((rcx as u32)) as u32); bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru32(r10.wrapping_add(0x14u64))) as u32 as u64; r8 = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(0x10u64), ((r8 as u32)) as u32); rdx = ((r8 as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(r15)) as u64; r8 = (((r8 as u32)).wrapping_sub((r14 as u32))) as u32 as u64; rdx = (((rdx as u32)).wrapping_sub(crate::simd_ops::ru32(r10.wrapping_add(0x18u64)))) as u32 as u64; r8 = (((r8 as u32)).wrapping_sub((r9 as u32))) as u32 as u64; crate::simd_ops::wu64(rsp, rax); r8 = (((r8 as u32)).wrapping_sub(0x2u32)) as u32 as u64; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x50u64))) as u32 as u64; crate::simd_ops::wu32(rsi, ((rax as u32)) as u32); rax = ((r14 as u32)) as u32 as u64; fl_l = ((rax).wrapping_add(r11)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x8u64), ((rcx as u32)) as u32); crate::simd_ops::wu64(rsi.wrapping_add(0x18u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0xcu64), ((rdx as u32)) as u32); crate::simd_ops::wu32(rsi.wrapping_add(0x14u64), ((r8 as u32)) as u32); bb = 23; continue; }
            19 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(r11))) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 21; continue; } bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::lk_add_u32(r10.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(0x18u64), ((rcx as u32)) as u32); bb = 21; continue; }
            21 => { fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(0x10u64), ((rax as u32)) as u32); bb = 3; continue; }
            22 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r10.wrapping_add(0x18u64))) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(r15)) as u64; rdx = (crate::simd_ops::ru32(r10.wrapping_add(0x14u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xcu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x58u64))) as u32 as u64; crate::simd_ops::wu32(rsi, ((rax as u32)) as u32); crate::simd_ops::wu64(rsp, rcx); crate::simd_ops::wu32(rsp.wrapping_add(0x8u64), ((rdx as u32)) as u32); crate::simd_ops::wu32(rsi.wrapping_add(0x14u64), (0x0u32) as u32); crate::simd_ops::wu64(rsi.wrapping_add(0x18u64), (0x0u64) as u64); bb = 23; continue; }
            23 => { xmm0 = (crate::simd_ops::ru128(rsp)) as u128; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = rsi; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu128(rsi.wrapping_add(0x4u64), (xmm0) as u128); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x10u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142528ee0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x14; const P3: u64 = 0x1; const P4: u64 = 0x18;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; r11 = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = r11; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rbx = ((crate::simd_ops::ru8(rdx.wrapping_add(rax))) as u32) as u32 as u64; bb = 3; continue; }
            2 => { rbx = (rbx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 3; continue; }
            3 => { r10 = (crate::simd_ops::ru64(rcx)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r10))) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = ((rdx.wrapping_add(P3)) as u32) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P4), ((rax as u32)) as u32); bb = 5; continue; }
            5 => { rax = ((rdx.wrapping_add(P3)) as u32) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { r8 = ((rax as u32)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rax as u32)) as u32); r9 = ((r8 as u32)) as u32 as u64; fl_l = r8; fl_r = r11; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 9; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r10))) as u8 as u64; fl_r = (0x3du8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rdx = ((fl_l as u32)) as u32 as u64; bb = 6; continue; }
            9 => { fl_l = r9; fl_r = r11; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = ((crate::simd_ops::ru8(rax.wrapping_add(r10))) as u32) as u32 as u64; bb = 12; continue; }
            11 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 12; continue; }
            12 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rdx = (!((rdx as u32))) as u32 as u64; bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rax = ((rdx as u32)) as u32 as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142528f60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142528d70;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r8 = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; r10 = rcx; r9 = (crate::simd_ops::ru64(rcx)) as u64; rax = ((r8 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_sub(crate::simd_ops::ru32(rcx.wrapping_add(0x18u64)))) as u32 as u64; rdi = rdx; r11 = (crate::simd_ops::ru32(rcx.wrapping_add(0x14u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((r11 as u32)) as u32); rcx = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(r8.wrapping_add(r9))) as u8 as u64; fl_r = (0xau8) as u8 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x54u64), ((rax as u32)) as u32); if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (((r11 as u32)).wrapping_add(1)) as u32 as u64; r11 = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(0x18u64), ((rcx as u32)) as u32); crate::simd_ops::wu32(r10.wrapping_add(0x14u64), ((r11 as u32)) as u32); bb = 2; continue; }
            2 => { crate::simd_ops::wu32(r10.wrapping_add(0x10u64), ((rcx as u32)) as u32); fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(r9))) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = ((r11.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(0x14u64), ((rax as u32)) as u32); rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(0x18u64), ((rax as u32)) as u32); bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(r10.wrapping_add(0x8u64))) as u64; rbp = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; rbx = ((rbp as u32)) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(0x10u64), ((rbx as u32)) as u32); r8 = ((rbx as u32)) as u32 as u64; fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 17; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(r9.wrapping_add(rbp))) as u8 as u64; fl_r = (0x5bu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 17; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rsi); r8 = ((rbp.wrapping_add(0x1u64)) as u32) as u32 as u64; rsi = ((crate::simd_ops::ru8(r9.wrapping_add(rbp))) as u32) as u32 as u64; rax = 0; fl_l = 0; fl_r = 0; bb = 7; continue; }
            7 => { rcx = ((r8 as u32)) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(0x10u64), ((r8 as u32)) as u32); r11 = ((rcx as u32)) as u32 as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(r8.wrapping_add(r9))) as u8 as u64; fl_r = (0x3du8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 10; continue; } bb = 9; continue; }
            9 => { r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; bb = 7; continue; }
            10 => { fl_l = r11; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { r11 = ((crate::simd_ops::ru8(r8.wrapping_add(r9))) as u32) as u32 as u64; bb = 13; continue; }
            12 => { r11 = (r11 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 13; continue; }
            13 => { fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((r11 as u8)) as u8 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = (!((rax as u32))) as u32 as u64; bb = 15; continue; }
            15 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x11bu32) as u32); r8 = rsp.wrapping_add(0x50); r9 = ((rax as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x119u32) as u32); rdx = rdi; rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            17 => { rax = ((r8 as u32)) as u32 as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 22; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r9))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 22; continue; } bb = 19; continue; }
            19 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r9))) as u8 as u64; fl_r = (0xdu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 22; continue; } bb = 20; continue; }
            20 => { rax = ((r8 as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r9))) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 22; continue; } bb = 21; continue; }
            21 => { fl_l = (((r8 as u32)).wrapping_add(1)) as u32 as u64; r8 = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(0x10u64), ((r8 as u32)) as u32); bb = 17; continue; }
            22 => { rcx = (crate::simd_ops::ru32(r10.wrapping_add(0x14u64))) as u32 as u64; rdx = ((r8 as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_sub(crate::simd_ops::ru32(r10.wrapping_add(0x18u64)))) as u32 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r8 = (((r8 as u32)).wrapping_sub((rbp as u32))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = ((r9.wrapping_add(rbp)) as u64) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x3cu64), ((rdx as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rcx as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), rax); rax = rdi; crate::simd_ops::wu128(rdi.wrapping_add(0x4u64), (xmm0) as u128); crate::simd_ops::wu32(rdi, (0x118u32) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0x14u64), ((r8 as u32)) as u32); rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425290f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1; const P2: u64 = 0x14; const P3: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    r8 = ((rdx.wrapping_add(P1)) as u32) as u32 as u64;
    fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(rax))) as u8 as u64; fl_r = (0xau8) as u8 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((r8 as u32)) as u32);
    if !((((fl_l as u8)) != ((fl_r as u8)))) { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(P2), (1) as u32)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P3), ((r8 as u32)) as u32); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142529120(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x4;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rdx, ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64;
    rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P3), ((rax as u32)) as u32);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142529140(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140000000;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = ((((rcx as u8)) as i8 as i32) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_add(0xffffff9fu32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x15u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 9; continue; } bb = 1; continue; }
            1 => { r8 = ((P0) as u64) as u64; rax = (rax as u32 as i32 as i64) as u64; rdx = (crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x252917cu64))) as u32 as u64; fl_l = ((rdx).wrapping_add(r8)) as u64 as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; j_tgt = (rdx) as u64; bb = 0xffff_fffe; continue; }
            2 => { rax = (rax & !0xffu64) | (((0x7u8) as u8 as u64)); return rax; }
            3 => { rax = (rax & !0xffu64) | (((0x8u8) as u8 as u64)); return rax; }
            4 => { rax = (rax & !0xffu64) | (((0xcu8) as u8 as u64)); return rax; }
            5 => { rax = (rax & !0xffu64) | (((0xau8) as u8 as u64)); return rax; }
            6 => { rax = (rax & !0xffu64) | (((0xdu8) as u8 as u64)); return rax; }
            7 => { rax = (rax & !0xffu64) | (((0x9u8) as u8 as u64)); return rax; }
            8 => { rax = (rax & !0xffu64) | (((0xbu8) as u8 as u64)); return rax; }
            9 => { rax = (((rcx as u8)) as u32) as u32 as u64; return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x142529140u64) { 0xb => { bb = 1; continue; } 0x21 => { bb = 2; continue; } 0x24 => { bb = 3; continue; } 0x27 => { bb = 4; continue; } 0x2a => { bb = 5; continue; } 0x2d => { bb = 6; continue; } 0x30 => { bb = 7; continue; } 0x33 => { bb = 8; continue; } 0x36 => { bb = 9; continue; } _ => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(j_tgt, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); } } }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425291e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141470f70;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = rdx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r8);
    rax = (0xffffffffffffffffu64) as u64;
    loop { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r8.wrapping_add(rax))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if !((((fl_l as u8)) != ((fl_r as u8)))) { break; } }
    rdx = rsp.wrapping_add(0x20);
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32);
    crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), (0x0u32) as u32);
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rbx, rax); rax = rbx; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    crate::simd_ops::wu64(rbx, (0x0u64) as u64);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142529240(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141470f70; const P1: u64 = 0xc; const P2: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = rdx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r8);
    rdx = rsp.wrapping_add(0x20);
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r9 as u32)) as u32);
    crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), (0x0u32) as u32);
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P2), ((rax as u32)) as u32); rax = rbx; crate::simd_ops::wu64(rbx, rcx); rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    crate::simd_ops::wu64(rbx, (0x0u64) as u64);
    rax = rbx;
    crate::simd_ops::wu32(rbx.wrapping_add(P2), (0x117u32) as u32);
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425292a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0xffffffffffffffe8; const P5: u64 = 0xfffffffffffffff0; const P6: u64 = 0xfffffffffffffff4; const P7: u64 = 0x141470f70; const P8: u64 = 0x141471180; const P9: u64 = 0x141471080; const P10: u64 = 0xc; const P11: u64 = 0x30; const P12: u64 = 0x14252a360; const P13: u64 = 0x14183acbc;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(P0), rbx); crate::simd_ops::wu64(rax.wrapping_add(P1), rbp); crate::simd_ops::wu64(rax.wrapping_add(P2), rsi); crate::simd_ops::wu64(rax.wrapping_add(P3), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbp = r9; crate::simd_ops::wu64(rax.wrapping_add(P4), r8); crate::simd_ops::wu32(rax.wrapping_add(P5), ((rbp as u32)) as u32); r14 = r8; crate::simd_ops::wu32(rax.wrapping_add(P6), (0x0u32) as u32); rdi = rdx; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rcx; r10 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r10 = ((r10).wrapping_shr(2u32)) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_r = r10; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rdx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = rbx; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rdx = rsp.wrapping_add(0x20); rcx = rbx; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rsi = rax; rcx = (crate::simd_ops::ru32(rax.wrapping_add(P10))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rdi, rax); crate::simd_ops::wu32(rdi.wrapping_add(P0), ((rcx as u32)) as u32); bb = 6; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P11))) as u64; rdx = rbp.wrapping_add(0x1); rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); r8 = rbp; rdx = r14; rcx = rax; rbx = rax; rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rbx.wrapping_add(rbp), (0x0u8) as u8); crate::simd_ops::wu64(rsi, rbx); crate::simd_ops::wu32(rsi.wrapping_add(0xcu64), (0x117u32) as u32); crate::simd_ops::wu64(rdi, rbx); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), (0x117u32) as u32); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = rdi; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142529380(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426cdcb0; const P1: u64 = 0x141834d64; const P2: u64 = 0x14306e910; const P3: u64 = 0x14183acda; const P4: u64 = 0x141471180; const P5: u64 = 0x141471080;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x142529380, funcinfo_rva: 0x37fa6e4, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x78, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x38u64)) as u64; rbx = (rdx) as u64; r15 = (rcx) as u64; rax = ((P0) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); rbp = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rbp) as u64); crate::simd_ops::wu64(rcx, (rbp) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), (rbp) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), (rax) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), (rbp) as u64); rcx = (0x800u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1425293c6); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (rax) as u64; crate::simd_ops::wu64(r15, (rax) as u64); crate::simd_ops::wu64(r15.wrapping_add(0x8u64), (0x80u64) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); r8 = ((rax.wrapping_add(0x10u64)) as u64) as u64; r9 = ((rbp.wrapping_add(0x3fu64)) as u32) as u32 as u64; bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r8, (rcx) as u64); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x8u64), (rax) as u64); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x10u64), (rax) as u64); r8 = ((r8.wrapping_add(0x20u64)) as u64) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(r8.wrapping_sub(0x8u64), (rax) as u64); rdx = ((rdx.wrapping_add(0x20u64)) as u64) as u64; fl_l = (r9) as u64 as u64; fl_r = (0x1u64) as u64 as u64; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r8, (rax) as u64); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x8u64), (rax) as u64); crate::simd_ops::wu64(r15.wrapping_add(0x30u64), (rbx) as u64); rax = (0x11fu32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x88u64), ((rax as u32)) as u32); rcx = ((P2) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), (rcx) as u64); bb = 3; continue; }
            3 => { r12 = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r12) as u64); rsi = (0xffffffffffffffffu64) as u64; bb = 4; continue; }
            4 => { rsi = ((rsi).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r12.wrapping_add(rsi))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rsi as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), ((rax as u32)) as u32); rbx = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; rax = ((rbx.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shr(2u32)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 21; continue; } bb = 6; continue; }
            6 => { fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 7; continue; }
            7 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r15.wrapping_add(0x20u64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { r8 = ((rsi as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; rcx = (r12) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1425294a4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 20; continue; } bb = 9; continue; }
            9 => { r9 = ((rbx.wrapping_sub(0x1u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), (r9) as u64); rdx = (0x811c9dc5u32) as u32 as u64; r8 = ((rsi as u32)) as u32 as u64; rcx = (rbp) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { bb = 11; continue; }
            11 => { rax = ((crate::simd_ops::ru8(r12.wrapping_add(rcx))) as u32) as u32 as u64; rax = ((((rax as u32)) ^ ((rdx as u32)))) as u32 as u64; rdx = ((((rax as u32)) as i32).wrapping_mul((0x1000193u32) as i32) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 11; continue; } bb = 12; continue; }
            12 => { rdi = ((rdx as u32)) as u32 as u64; fl_l = (((rdi) & (r9))) as u64 as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; r14 = (rbp) as u64; bb = 13; continue; }
            13 => { rbx = (rdi) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(r15))) as u64; rbp = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; r13 = ((rbp as u32)) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 16; continue; } bb = 14; continue; }
            14 => { r8 = ((rbp as u32)) as u32 as u64; rdx = (r12) as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252950d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 21; continue; } bb = 15; continue; }
            15 => { r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; bb = 16; continue; }
            16 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r15.wrapping_add(0x20u64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 19; continue; } bb = 17; continue; }
            17 => { r8 = (r13) as u64; rdx = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252952e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 20; continue; } bb = 18; continue; }
            18 => { r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; bb = 19; continue; }
            19 => { rdi = ((rdi).wrapping_add(1)) as u64; rdi = ((rdi).wrapping_add(r14)) as u64; rdi = (((rdi) & (r9))) as u64; r14 = ((r14).wrapping_add(1)) as u64; fl_l = (r14) as u64 as u64; fl_r = (r9) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 13; continue; } bb = 20; continue; }
            20 => { rcx = (r15) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142529553); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 21; continue; }
            21 => { rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = (r15) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142529560); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x88u64))) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x88u64), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rcx = ((rcx).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), (rcx) as u64); fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x134u32) as u32 as u64; rbp = (0x0u32) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 3; continue; } bb = 22; continue; }
            22 => { rax = (r15) as u64; rsp = ((rsp).wrapping_add(0x38u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425295c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r9 = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    r8 = 0x811c9dc5;
    fl_l = r9; fl_r = r9;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax = ((r8 as u32)) as u32 as u64; return rax; }
    rdx = (crate::simd_ops::ru64(rdx)) as u64;
    loop { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; rax = ((((rax as u32)) ^ ((r8 as u32)))) as u32 as u64; r8 = ((((rax as u32)) as i32).wrapping_mul((0x1000193u32) as i32) as u32) as u32 as u64; fl_l = rcx; fl_r = r9; if !((((fl_l as u64)) < ((fl_r as u64)))) { break; } }
    rax = ((r8 as u32)) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142529600(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14183acda;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    if (((fl_l as u32)) != ((fl_r as u32))) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rdx = (crate::simd_ops::ru64(rdx)) as u64;
    r8 = ((rax as u32)) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) != 0)) { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142529640(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x14; const P2: u64 = 0x18;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu32(rcx, ((r8 as u32)) as u32);
    rax = rcx;
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P0), (xmm0) as u128);
    crate::simd_ops::wu32(rcx.wrapping_add(P1), (0x0u32) as u32);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), r9);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142529670(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14; const P1: u64 = 0x4; const P2: u64 = 0x18;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64;
    crate::simd_ops::wu32(rcx, ((r8 as u32)) as u32);
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rax as u32)) as u32);
    rax = rcx;
    crate::simd_ops::wu128(rcx.wrapping_add(P1), (xmm0) as u128);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), r9);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425296a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14; const P1: u64 = 0x18; const P2: u64 = 0x4;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (((r8 as u8)) as u32) as u32 as u64;
    crate::simd_ops::wu32(rcx, ((rax as u32)) as u32);
    rax = 0;
    fl_l = 0; fl_r = 0;
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    rax = rcx;
    crate::simd_ops::wu128(rcx.wrapping_add(P2), (xmm0) as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425296d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1418350d0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rcx; fl_r = rcx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx);
    loop { rbx = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; fl_l = rbx; fl_r = rbx; if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142529710(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141834d64; const P1: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = 0x2008;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rbx, rax);
    crate::simd_ops::wu64(rbx.wrapping_add(P1), rcx);
    crate::simd_ops::wu64(rax, rcx);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142529740(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143422e70; const P1: u64 = 0x143422e74; const P2: u64 = 0x143423044; const P3: u64 = 0x142527620; const P4: u64 = 0x140000000; const P5: u64 = 0x143422e98; const P6: u64 = 0x143422e9c; const P7: u64 = 0x143422ea0; const P8: u64 = 0x143422ea4; const P9: u64 = 0x143422ea8; const P10: u64 = 0x1434278e0; const P11: u64 = 0x14336adcc; const P12: u64 = 0x14336add0; const P13: u64 = 0x1426467f0; const P14: u64 = 0x142529b2b; const P15: u64 = 0x141834d30; const P16: u64 = 0x142529b7e; const P17: u64 = 0x143422eb0; const P18: u64 = 0x1401e9110; const P19: u64 = 0x143427970; const P20: u64 = 0x143422eb8; const P21: u64 = 0x143422ed8; const P22: u64 = 0x143422edc; const P23: u64 = 0x143422ede; const P24: u64 = 0x140203bc0; const P25: u64 = 0x143422f0c; const P26: u64 = 0x143427960; const P27: u64 = 0x143422f18; const P28: u64 = 0x143422f38; const P29: u64 = 0x143422f40; const P30: u64 = 0x143427950; const P31: u64 = 0x143422f48; const P32: u64 = 0x143422f68; const P33: u64 = 0x143422ee0; const P34: u64 = 0x143427920; const P35: u64 = 0x143422ee8; const P36: u64 = 0x143422ef8; const P37: u64 = 0x143422efa; const P38: u64 = 0x143422efc; const P39: u64 = 0x14336adb0; const P40: u64 = 0x14336adb4; const P41: u64 = 0x143422f04; const P42: u64 = 0x143427900; const P43: u64 = 0x143423000; const P44: u64 = 0x143423008; const P45: u64 = 0x143423010; const P46: u64 = 0x143423014; const P47: u64 = 0x143423016; const P48: u64 = 0x143423018; const P49: u64 = 0x143423030; const P50: u64 = 0x143423040; const P51: u64 = 0x143422f70; const P52: u64 = 0x143422f90; const P53: u64 = 0x143422f98; const P54: u64 = 0x143422f9a; const P55: u64 = 0x14306e9c0; const P56: u64 = 0x143073d78; const P57: u64 = 0x143422fa0; const P58: u64 = 0x143422fd0; const P59: u64 = 0x143422fe8; const P60: u64 = 0x143422ff8; const P61: u64 = 0x143422ffc; const P62: u64 = 0x1433711c8; const P63: u64 = 0x1433711d0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x142529740, funcinfo_rva: 0x37fa754, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x78, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut fl_k: u32 = 0;
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rbp = (rsp) as u64; rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rbx = (rdx) as u64; r8 = (rcx) as u64; rsi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbp.wrapping_sub(0x50u64), ((rsi as u32)) as u32); rdx = (((crate::simd_ops::ru32(rcx)) as i32 as i64) as u64) as u64; rax = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x101u32) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 7; continue; } bb = 1; continue; }
            1 => { if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 2; continue; }
            2 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rbx, (xmm0) as u128); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (0x5u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (0xfu64) as u64); rax = (crate::simd_ops::ru32(P0)) as u32 as u64; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(P1)) as u32) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x4u64), ((rax as u8)) as u8); crate::simd_ops::wu8(rbx.wrapping_add(0x5u64), ((rsi as u8)) as u8); bb = 104; continue; }
            4 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x100u32) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 103; continue; } bb = 5; continue; }
            5 => { r8 = ((rdx as u32)) as u32 as u64; rdx = ((P2) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1425297c5); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 104; continue; }
            6 => { xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rbx, (xmm0) as u128); rdi = (0x4u32) as u32 as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (rdi) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (0xfu64) as u64); crate::simd_ops::wu32(rbx, (0x273d3d27u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x4u64), ((rsi as u8)) as u8); bb = 104; continue; }
            7 => { rax = (((rax as u32)).wrapping_sub(0x102u32)) as u32 as u64; r9 = ((P4) as u64) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1bu32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 101; continue; } bb = 8; continue; }
            8 => { rax = (rax as u32 as i32 as i64) as u64; rcx = (crate::simd_ops::ru32(r9.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x252a2c8u64))) as u32 as u64; fl_l = ((rcx).wrapping_add(r9)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; j_tgt = (rcx) as u64; bb = 0xffff_fffe; continue; }
            9 => { xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rbx, (xmm0) as u128); rdi = (0x4u32) as u32 as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (rdi) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (0xfu64) as u64); crate::simd_ops::wu32(rbx, (0x273d3c27u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x4u64), ((rsi as u8)) as u8); bb = 104; continue; }
            10 => { xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rbx, (xmm0) as u128); rdi = (0x4u32) as u32 as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (rdi) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (0xfu64) as u64); crate::simd_ops::wu32(rbx, (0x273d3e27u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x4u64), ((rsi as u8)) as u8); bb = 104; continue; }
            11 => { xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rbx, (xmm0) as u128); rdi = (0x4u32) as u32 as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (rdi) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (0xfu64) as u64); crate::simd_ops::wu32(rbx, (0x273d7e27u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x4u64), ((rsi as u8)) as u8); bb = 104; continue; }
            12 => { xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rbx, (xmm0) as u128); rdi = (0x4u32) as u32 as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (rdi) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (0xfu64) as u64); crate::simd_ops::wu32(rbx, (0x272e2e27u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x4u64), ((rsi as u8)) as u8); bb = 104; continue; }
            13 => { xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rbx, (xmm0) as u128); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (0x5u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (0xfu64) as u64); rax = (crate::simd_ops::ru32(P5)) as u32 as u64; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(P6)) as u32) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x4u64), ((rax as u8)) as u8); crate::simd_ops::wu8(rbx.wrapping_add(0x5u64), ((rsi as u8)) as u8); bb = 104; continue; }
            14 => { xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rbx, (xmm0) as u128); rdi = (0x4u32) as u32 as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (rdi) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (0xfu64) as u64); crate::simd_ops::wu32(rbx, (0x273e2d27u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x4u64), ((rsi as u8)) as u8); bb = 104; continue; }
            15 => { xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rbx, (xmm0) as u128); rdi = (0x4u32) as u32 as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (rdi) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (0xfu64) as u64); crate::simd_ops::wu32(rbx, (0x273a3a27u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x4u64), ((rsi as u8)) as u8); bb = 104; continue; }
            16 => { xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rbx, (xmm0) as u128); rdi = (0x4u32) as u32 as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (rdi) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (0xfu64) as u64); crate::simd_ops::wu32(rbx, (0x273d2b27u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x4u64), ((rsi as u8)) as u8); bb = 104; continue; }
            17 => { xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rbx, (xmm0) as u128); rdi = (0x4u32) as u32 as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (rdi) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (0xfu64) as u64); crate::simd_ops::wu32(rbx, (0x273d2d27u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x4u64), ((rsi as u8)) as u8); bb = 104; continue; }
            18 => { xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rbx, (xmm0) as u128); rdi = (0x4u32) as u32 as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (rdi) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (0xfu64) as u64); crate::simd_ops::wu32(rbx, (0x273d2a27u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x4u64), ((rsi as u8)) as u8); bb = 104; continue; }
            19 => { xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rbx, (xmm0) as u128); rdi = (0x4u32) as u32 as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (rdi) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (0xfu64) as u64); crate::simd_ops::wu32(rbx, (0x273d2f27u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x4u64), ((rsi as u8)) as u8); bb = 104; continue; }
            20 => { xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rbx, (xmm0) as u128); rdi = (0x4u32) as u32 as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (rdi) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (0xfu64) as u64); crate::simd_ops::wu32(rbx, (0x273d2527u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x4u64), ((rsi as u8)) as u8); bb = 104; continue; }
            21 => { xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rbx, (xmm0) as u128); rdi = (0x4u32) as u32 as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (rdi) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (0xfu64) as u64); crate::simd_ops::wu32(rbx, (0x273d5e27u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x4u64), ((rsi as u8)) as u8); bb = 104; continue; }
            22 => { xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rbx, (xmm0) as u128); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (0x5u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (0xfu64) as u64); rax = (crate::simd_ops::ru32(P7)) as u32 as u64; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(P8)) as u32) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x4u64), ((rax as u8)) as u8); crate::simd_ops::wu8(rbx.wrapping_add(0x5u64), ((rsi as u8)) as u8); bb = 104; continue; }
            23 => { r9 = (crate::simd_ops::ru64(r8.wrapping_add(0x18u64))) as u64; fl_l = (r9) as u64 as u64; fl_r = (r9) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 24; continue; }
            24 => { r8 = (crate::simd_ops::ru32(r8.wrapping_add(0x14u64))) as u32 as u64; rdx = ((P9) as u64) as u64; rcx = ((rbp.wrapping_sub(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142529a79); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (0x2u32) as u32 as u64; bb = 26; continue; }
            25 => { xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x48u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(P10)) as u128; ymm1_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x38u64), (xmm1) as u128); rax = (crate::simd_ops::ru32(P11)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x48u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru16(P12)) as u32) as u32 as u64; crate::simd_ops::wu16(rbp.wrapping_sub(0x44u64), ((rax as u16)) as u16); crate::simd_ops::wu8(rbp.wrapping_sub(0x42u64), ((rsi as u8)) as u8); rax = ((rbp.wrapping_sub(0x48u64)) as u64) as u64; rdi = (0x4u32) as u32 as u64; bb = 26; continue; }
            26 => { xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rbx, (xmm0) as u128); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (rsi) as u64); let y_addr: u64 = rax; xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); let y_addr: u64 = rbx; core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), (0xfu64) as u64); crate::simd_ops::wu8(rax, (0x0u8) as u8); rdi = ((((rdi as u32)) | (0x1u32))) as u32 as u64; fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 32; continue; } bb = 27; continue; }
            27 => { rdi = ((((rdi as u32)) & (0xfffffffbu32))) as u32 as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0xfu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 32; continue; } bb = 28; continue; }
            28 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x48u64))) as u64; rax = (rcx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 31; continue; } bb = 29; continue; }
            29 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 31; continue; } bb = 30; continue; }
            30 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142529b25); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P13)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P14, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            31 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142529b2f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P15) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 32; continue; }
            32 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 104; continue; } bb = 33; continue; }
            33 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x8u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0xfu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 104; continue; } bb = 34; continue; }
            34 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x20u64))) as u64; rax = (rcx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 37; continue; } bb = 35; continue; }
            35 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 37; continue; } bb = 36; continue; }
            36 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142529b78); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P13)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P16, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            37 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142529b82); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P15) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 104; continue; }
            38 => { r9 = (crate::simd_ops::ru64(r8.wrapping_add(0x18u64))) as u64; fl_l = (r9) as u64 as u64; fl_r = (r9) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 40; continue; } bb = 39; continue; }
            39 => { r8 = (crate::simd_ops::ru32(r8.wrapping_add(0x14u64))) as u32 as u64; rdx = ((P17) as u64) as u64; rcx = ((rbp.wrapping_sub(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142529ba4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (0x8u32) as u32 as u64; bb = 41; continue; }
            40 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), (rsi) as u64); rcx = (0x30u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142529bba); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P18) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (rax) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), (rax) as u64); xmm0 = (crate::simd_ops::ru128(P19)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x38u64), (xmm0) as u128); let y_addr: u64 = P20; xmm1 = core::ptr::read_unaligned(y_addr as *const u128); ymm1_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); let y_addr: u64 = rax; core::ptr::write_unaligned(y_addr as *mut u128, xmm1); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm1_hi); rcx = (crate::simd_ops::ru32(P21)) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(0x20u64), ((rcx as u32)) as u32); rcx = ((crate::simd_ops::ru16(P22)) as u32) as u32 as u64; crate::simd_ops::wu16(rax.wrapping_add(0x24u64), ((rcx as u16)) as u16); rax = ((crate::simd_ops::ru8(P23)) as u32) as u32 as u64; crate::simd_ops::wu8(rdx.wrapping_add(0x26u64), ((rax as u8)) as u8); crate::simd_ops::wu8(rdx.wrapping_add(0x27u64), ((rsi as u8)) as u8); rax = ((rbp.wrapping_sub(0x48u64)) as u64) as u64; rdi = (0x10u32) as u32 as u64; bb = 41; continue; }
            41 => { xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rbx, (xmm0) as u128); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (rsi) as u64); let y_addr: u64 = rax; xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); let y_addr: u64 = rbx; core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), (0xfu64) as u64); crate::simd_ops::wu8(rax, (0x0u8) as u8); rdi = ((((rdi as u32)) | (0x1u32))) as u32 as u64; fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 44; continue; } bb = 42; continue; }
            42 => { rdi = ((((rdi as u32)) & (0xffffffefu32))) as u32 as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64))) as u64; fl_l = (r8) as u64 as u64; fl_r = (0xfu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 44; continue; } bb = 43; continue; }
            43 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x48u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142529c4e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P24) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 44; continue; }
            44 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x8u8) as u8 as u64; fl_k = 4; bb = 81; continue; }
            45 => { r9 = (crate::simd_ops::ru64(r8.wrapping_add(0x18u64))) as u64; fl_l = (r9) as u64 as u64; fl_r = (r9) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 47; continue; } bb = 46; continue; }
            46 => { r8 = (crate::simd_ops::ru32(r8.wrapping_add(0x14u64))) as u32 as u64; rdx = ((P25) as u64) as u64; rcx = ((rbp.wrapping_sub(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142529c75); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (0x20u32) as u32 as u64; bb = 48; continue; }
            47 => { xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x48u64), (xmm0) as u128); rcx = (0x30u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142529c90); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P18) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), (rax) as u64); xmm0 = (crate::simd_ops::ru128(P26)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x38u64), (xmm0) as u128); let y_addr: u64 = P27; xmm1 = core::ptr::read_unaligned(y_addr as *const u128); ymm1_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); let y_addr: u64 = rax; core::ptr::write_unaligned(y_addr as *mut u128, xmm1); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm1_hi); rcx = (crate::simd_ops::ru32(P28)) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(0x20u64), ((rcx as u32)) as u32); crate::simd_ops::wu8(rax.wrapping_add(0x24u64), ((rsi as u8)) as u8); rax = ((rbp.wrapping_sub(0x48u64)) as u64) as u64; rdi = (0x40u32) as u32 as u64; bb = 48; continue; }
            48 => { xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rbx, (xmm0) as u128); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (rsi) as u64); let y_addr: u64 = rax; xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); let y_addr: u64 = rbx; core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), (0xfu64) as u64); crate::simd_ops::wu8(rax, (0x0u8) as u8); rdi = ((((rdi as u32)) | (0x1u32))) as u32 as u64; fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x40u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 51; continue; } bb = 49; continue; }
            49 => { rdi = ((((rdi as u32)) & (0xffffffbfu32))) as u32 as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64))) as u64; fl_l = (r8) as u64 as u64; fl_r = (0xfu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 51; continue; } bb = 50; continue; }
            50 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x48u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142529d0c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P24) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 51; continue; }
            51 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x20u8) as u8 as u64; fl_k = 4; bb = 81; continue; }
            52 => { r9 = (crate::simd_ops::ru64(r8.wrapping_add(0x18u64))) as u64; fl_l = (r9) as u64 as u64; fl_r = (r9) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 54; continue; } bb = 53; continue; }
            53 => { r8 = (crate::simd_ops::ru32(r8.wrapping_add(0x14u64))) as u32 as u64; rdx = ((P29) as u64) as u64; rcx = ((rbp.wrapping_sub(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142529d33); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (0x80u32) as u32 as u64; bb = 55; continue; }
            54 => { xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x48u64), (xmm0) as u128); rcx = (0x30u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142529d4e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P18) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), (rax) as u64); xmm0 = (crate::simd_ops::ru128(P30)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x38u64), (xmm0) as u128); let y_addr: u64 = P31; xmm1 = core::ptr::read_unaligned(y_addr as *const u128); ymm1_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); let y_addr: u64 = rax; core::ptr::write_unaligned(y_addr as *mut u128, xmm1); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm1_hi); rcx = ((crate::simd_ops::ru8(P32)) as u32) as u32 as u64; crate::simd_ops::wu8(rax.wrapping_add(0x20u64), ((rcx as u8)) as u8); crate::simd_ops::wu8(rax.wrapping_add(0x21u64), ((rsi as u8)) as u8); rax = ((rbp.wrapping_sub(0x48u64)) as u64) as u64; rdi = (0x100u32) as u32 as u64; bb = 55; continue; }
            55 => { xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rbx, (xmm0) as u128); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (rsi) as u64); let y_addr: u64 = rax; xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); let y_addr: u64 = rbx; core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), (0xfu64) as u64); crate::simd_ops::wu8(rax, (0x0u8) as u8); rdi = ((((rdi as u32)) | (0x1u32))) as u32 as u64; fl_l = ((((rdi as u32)) >> (8u32)) & 1) as u64; if ((fl_l as u32) == 0) { bb = 58; continue; } bb = 56; continue; }
            56 => { fl_l = ((((rdi as u32)) >> (8u32)) & 1) as u64; rdi = (((rdi as u32)) & !((1u32) << (8u32))) as u32 as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64))) as u64; fl_l = (r8) as u64 as u64; fl_r = (0xfu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 58; continue; } bb = 57; continue; }
            57 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x48u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142529dcc); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P24) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 58; continue; }
            58 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((((fl_l as u8)) & ((fl_r as u8))) as i8) >= 0) { bb = 104; continue; } bb = 59; continue; }
            59 => { fl_k = 4; bb = 82; continue; }
            60 => { r9 = (crate::simd_ops::ru64(r8.wrapping_add(0x18u64))) as u64; fl_l = (r9) as u64 as u64; fl_r = (r9) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 62; continue; } bb = 61; continue; }
            61 => { r8 = (crate::simd_ops::ru32(r8.wrapping_add(0x14u64))) as u32 as u64; rdx = ((P33) as u64) as u64; rcx = ((rbp.wrapping_sub(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142529df8); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (0x200u32) as u32 as u64; bb = 63; continue; }
            62 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), (rsi) as u64); rcx = (0x20u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142529e0e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P18) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), (rax) as u64); xmm0 = (crate::simd_ops::ru128(P34)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x38u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(P35)) as u128; ymm1_hi = 0; crate::simd_ops::wu128(rax, (xmm1) as u128); rcx = ((crate::simd_ops::ru16(P36)) as u32) as u32 as u64; crate::simd_ops::wu16(rax.wrapping_add(0x10u64), ((rcx as u16)) as u16); rcx = ((crate::simd_ops::ru8(P37)) as u32) as u32 as u64; crate::simd_ops::wu8(rax.wrapping_add(0x12u64), ((rcx as u8)) as u8); crate::simd_ops::wu8(rax.wrapping_add(0x13u64), ((rsi as u8)) as u8); rax = ((rbp.wrapping_sub(0x48u64)) as u64) as u64; rdi = (0x400u32) as u32 as u64; bb = 63; continue; }
            63 => { xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rbx, (xmm0) as u128); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (rsi) as u64); let y_addr: u64 = rax; xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); let y_addr: u64 = rbx; core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), (0xfu64) as u64); crate::simd_ops::wu8(rax, (0x0u8) as u8); rdi = ((((rdi as u32)) | (0x1u32))) as u32 as u64; fl_l = ((((rdi as u32)) >> (10u32)) & 1) as u64; if ((fl_l as u32) == 0) { bb = 66; continue; } bb = 64; continue; }
            64 => { fl_l = ((((rdi as u32)) >> (10u32)) & 1) as u64; rdi = (((rdi as u32)) & !((1u32) << (10u32))) as u32 as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64))) as u64; fl_l = (r8) as u64 as u64; fl_r = (0xfu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 66; continue; } bb = 65; continue; }
            65 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x48u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142529e97); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P24) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 66; continue; }
            66 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = (0x200u32) as u32 as u64; fl_k = 2; bb = 81; continue; }
            67 => { r9 = (crate::simd_ops::ru64(r8.wrapping_add(0x18u64))) as u64; fl_l = (r9) as u64 as u64; fl_r = (r9) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 69; continue; } bb = 68; continue; }
            68 => { r8 = (crate::simd_ops::ru32(r8.wrapping_add(0x14u64))) as u32 as u64; rdx = ((P38) as u64) as u64; rcx = ((rbp.wrapping_sub(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142529ec0); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (0x800u32) as u32 as u64; bb = 70; continue; }
            69 => { xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x48u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(P10)) as u128; ymm1_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x38u64), (xmm1) as u128); rax = (crate::simd_ops::ru32(P39)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x48u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru16(P40)) as u32) as u32 as u64; crate::simd_ops::wu16(rbp.wrapping_sub(0x44u64), ((rax as u16)) as u16); crate::simd_ops::wu8(rbp.wrapping_sub(0x42u64), ((rsi as u8)) as u8); rax = ((rbp.wrapping_sub(0x48u64)) as u64) as u64; rdi = (0x1000u32) as u32 as u64; bb = 70; continue; }
            70 => { xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rbx, (xmm0) as u128); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (rsi) as u64); let y_addr: u64 = rax; xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); let y_addr: u64 = rbx; core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), (0xfu64) as u64); crate::simd_ops::wu8(rax, (0x0u8) as u8); rdi = ((((rdi as u32)) | (0x1u32))) as u32 as u64; fl_l = ((((rdi as u32)) >> (12u32)) & 1) as u64; if ((fl_l as u32) == 0) { bb = 73; continue; } bb = 71; continue; }
            71 => { fl_l = ((((rdi as u32)) >> (12u32)) & 1) as u64; rdi = (((rdi as u32)) & !((1u32) << (12u32))) as u32 as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64))) as u64; fl_l = (r8) as u64 as u64; fl_r = (0xfu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 73; continue; } bb = 72; continue; }
            72 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x48u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142529f49); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P24) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 73; continue; }
            73 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = (0x800u32) as u32 as u64; fl_k = 2; bb = 81; continue; }
            74 => { r8 = (crate::simd_ops::ru64(r8.wrapping_add(0x18u64))) as u64; fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 76; continue; } bb = 75; continue; }
            75 => { rdx = ((P41) as u64) as u64; rcx = ((rbp.wrapping_sub(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142529f6e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (0x2000u32) as u32 as u64; bb = 77; continue; }
            76 => { xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x48u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(P42)) as u128; ymm1_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x38u64), (xmm1) as u128); xmm0 = ((crate::simd_ops::ru64(P43)) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), ((xmm0 as u64)) as u64); rax = ((crate::simd_ops::ru16(P44)) as u32) as u32 as u64; crate::simd_ops::wu16(rbp.wrapping_sub(0x40u64), ((rax as u16)) as u16); crate::simd_ops::wu8(rbp.wrapping_sub(0x3eu64), ((rsi as u8)) as u8); rax = ((rbp.wrapping_sub(0x48u64)) as u64) as u64; rdi = (0x4000u32) as u32 as u64; bb = 77; continue; }
            77 => { xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rbx, (xmm0) as u128); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (rsi) as u64); let y_addr: u64 = rax; xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); let y_addr: u64 = rbx; core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), (0xfu64) as u64); crate::simd_ops::wu8(rax, (0x0u8) as u8); rdi = ((((rdi as u32)) | (0x1u32))) as u32 as u64; fl_l = ((((rdi as u32)) >> (14u32)) & 1) as u64; if ((fl_l as u32) == 0) { bb = 80; continue; } bb = 78; continue; }
            78 => { fl_l = ((((rdi as u32)) >> (14u32)) & 1) as u64; rdi = (((rdi as u32)) & !((1u32) << (14u32))) as u32 as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64))) as u64; fl_l = (r8) as u64 as u64; fl_r = (0xfu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 80; continue; } bb = 79; continue; }
            79 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x48u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142529ffb); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P24) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 80; continue; }
            80 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = (0x2000u32) as u32 as u64; fl_k = 2; bb = 81; continue; }
            81 => { if (match fl_k { 2 => ((((fl_l as u32)) & ((fl_r as u32))) == 0), 4 => ((((fl_l as u8)) & ((fl_r as u8))) == 0), _ => false }) { bb = 104; continue; } bb = 82; continue; }
            82 => { r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x8u64))) as u64; fl_l = (r8) as u64 as u64; fl_r = (0xfu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 104; continue; } bb = 83; continue; }
            83 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x20u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252a022); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P24) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 104; continue; }
            84 => { xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rbx, (xmm0) as u128); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (0x7u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (0xfu64) as u64); rax = (crate::simd_ops::ru64(P45)) as u64; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); rax = ((crate::simd_ops::ru16(P46)) as u32) as u32 as u64; crate::simd_ops::wu16(rbx.wrapping_add(0x4u64), ((rax as u16)) as u16); rax = ((crate::simd_ops::ru8(P47)) as u32) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x6u64), ((rax as u8)) as u8); crate::simd_ops::wu8(rbx.wrapping_add(0x7u64), ((rsi as u8)) as u8); bb = 104; continue; }
            85 => { xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rbx, (xmm0) as u128); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (rsi) as u64); rcx = (0x20u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252a080); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P18) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbx, (rax) as u64); rdi = (0x10u32) as u32 as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (rdi) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (0x1fu64) as u64); xmm0 = (crate::simd_ops::ru128(P48)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rax, (xmm0) as u128); crate::simd_ops::wu8(rax.wrapping_add(0x10u64), ((rsi as u8)) as u8); bb = 104; continue; }
            86 => { xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rbx, (xmm0) as u128); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (rsi) as u64); rcx = (0x20u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252a0c3); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P18) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbx, (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (0x12u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (0x1fu64) as u64); xmm0 = (crate::simd_ops::ru128(P49)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rax, (xmm0) as u128); rcx = ((crate::simd_ops::ru16(P50)) as u32) as u32 as u64; crate::simd_ops::wu16(rax.wrapping_add(0x10u64), ((rcx as u16)) as u16); crate::simd_ops::wu8(rax.wrapping_add(0x12u64), ((rsi as u8)) as u8); bb = 104; continue; }
            87 => { xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rbx, (xmm0) as u128); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (rsi) as u64); rcx = (0x30u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252a110); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P18) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (rax) as u64; crate::simd_ops::wu64(rbx, (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (0x2bu64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (0x2fu64) as u64); let y_addr: u64 = P51; xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); let y_addr: u64 = rax; core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); xmm0 = ((crate::simd_ops::ru64(P52)) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rax.wrapping_add(0x20u64), ((xmm0 as u64)) as u64); rcx = ((crate::simd_ops::ru16(P53)) as u32) as u32 as u64; crate::simd_ops::wu16(rax.wrapping_add(0x28u64), ((rcx as u16)) as u16); rax = ((crate::simd_ops::ru8(P54)) as u32) as u32 as u64; crate::simd_ops::wu8(rdx.wrapping_add(0x2au64), ((rax as u8)) as u8); crate::simd_ops::wu8(rdx.wrapping_add(0x2bu64), ((rsi as u8)) as u8); bb = 104; continue; }
            88 => { r11 = (crate::simd_ops::ru32(r8.wrapping_add(0x18u64))) as u32 as u64; fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 100; continue; } bb = 89; continue; }
            89 => { r9 = ((P55) as u64) as u64; rdx = (0x6fau32) as u32 as u64; fl_k = 2; bb = 90; continue; }
            90 => { rcx = (rdx) as u64; rcx = ((rcx).wrapping_shr(1u32)) as u64; rax = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; r10 = ((r9.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru32(r10)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 92; continue; } bb = 91; continue; }
            91 => { r9 = ((r10.wrapping_add(0xcu64)) as u64) as u64; rax = (0xffffffffffffffffu64) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; fl_l = ((rdx).wrapping_add(rax)) as u64 as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; fl_k = 5; bb = 93; continue; }
            92 => { rdx = (rcx) as u64; fl_k = 0; bb = 93; continue; }
            93 => { fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) > 0) { fl_k = 3; bb = 90; continue; } bb = 94; continue; }
            94 => { rax = ((P56) as u64) as u64; fl_l = (r9) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { fl_k = 1; bb = 98; continue; } bb = 95; continue; }
            95 => { rax = (crate::simd_ops::ru32(r9)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { fl_k = 0; bb = 98; continue; } bb = 96; continue; }
            96 => { fl_l = ((r9).wrapping_add(0x4u64)) as u64 as u64; fl_r = fl_l; r9 = ((fl_l as u64)) as u64; r8 = ((r11 as u32)) as u32 as u64; if (((fl_l as u64)) == 0) { fl_k = 5; bb = 99; continue; } bb = 97; continue; }
            97 => { rdx = ((P57) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252a1e2); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 104; continue; }
            98 => { r8 = ((r11 as u32)) as u32 as u64; bb = 99; continue; }
            99 => { rdx = ((P58) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252a1f9); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 104; continue; }
            100 => { xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rbx, (xmm0) as u128); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (rsi) as u64); rcx = (0x20u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252a218); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P18) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbx, (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (0x16u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (0x1fu64) as u64); xmm0 = (crate::simd_ops::ru128(P59)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rax, (xmm0) as u128); rcx = (crate::simd_ops::ru32(P60)) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(0x10u64), ((rcx as u32)) as u32); rcx = ((crate::simd_ops::ru16(P61)) as u32) as u32 as u64; crate::simd_ops::wu16(rax.wrapping_add(0x14u64), ((rcx as u16)) as u16); crate::simd_ops::wu8(rax.wrapping_add(0x16u64), ((rsi as u8)) as u8); bb = 104; continue; }
            101 => { rax = ((rdx.wrapping_sub(0x11fu64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x14u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 103; continue; } bb = 102; continue; }
            102 => { r8 = (crate::simd_ops::ru64(r9.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x306e018u64))) as u64; rdx = ((P41) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252a273); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 104; continue; }
            103 => { xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rbx, (xmm0) as u128); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (0x9u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (0xfu64) as u64); xmm0 = ((crate::simd_ops::ru64(P62)) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbx, ((xmm0 as u64)) as u64); rax = ((crate::simd_ops::ru8(P63)) as u32) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x8u64), ((rax as u8)) as u8); crate::simd_ops::wu8(rbx.wrapping_add(0x9u64), ((rsi as u8)) as u8); bb = 104; continue; }
            104 => { rax = (rbx) as u64; r11 = ((rsp.wrapping_add(0x70u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsp = (r11) as u64; rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x142529740u64) { 0x33 => { bb = 1; continue; } 0x35 => { bb = 2; continue; } 0x39 => { bb = 3; continue; } 0x6c => { bb = 4; continue; } 0x78 => { bb = 5; continue; } 0x8f => { bb = 6; continue; } 0xb7 => { bb = 7; continue; } 0xcc => { bb = 8; continue; } 0xdb => { bb = 9; continue; } 0x103 => { bb = 10; continue; } 0x12b => { bb = 11; continue; } 0x153 => { bb = 12; continue; } 0x17b => { bb = 13; continue; } 0x1ae => { bb = 14; continue; } 0x1d6 => { bb = 15; continue; } 0x1fe => { bb = 16; continue; } 0x226 => { bb = 17; continue; } 0x24e => { bb = 18; continue; } 0x276 => { bb = 19; continue; } 0x29e => { bb = 20; continue; } 0x2c6 => { bb = 21; continue; } 0x2ee => { bb = 22; continue; } 0x321 => { bb = 23; continue; } 0x32a => { bb = 24; continue; } 0x346 => { bb = 25; continue; } 0x37d => { bb = 26; continue; } 0x3ad => { bb = 27; continue; } 0x3ba => { bb = 28; continue; } 0x3cd => { bb = 29; continue; } 0x3e2 => { bb = 30; continue; } 0x3ec => { bb = 31; continue; } 0x3f5 => { bb = 32; continue; } 0x3ff => { bb = 33; continue; } 0x40d => { bb = 34; continue; } 0x420 => { bb = 35; continue; } 0x435 => { bb = 36; continue; } 0x43f => { bb = 37; continue; } 0x44c => { bb = 38; continue; } 0x455 => { bb = 39; continue; } 0x471 => { bb = 40; continue; } 0x4ca => { bb = 41; continue; } 0x4fa => { bb = 42; continue; } 0x507 => { bb = 43; continue; } 0x514 => { bb = 44; continue; } 0x51d => { bb = 45; continue; } 0x526 => { bb = 46; continue; } 0x542 => { bb = 47; continue; } 0x588 => { bb = 48; continue; } 0x5b8 => { bb = 49; continue; } 0x5c5 => { bb = 50; continue; } 0x5d2 => { bb = 51; continue; } 0x5db => { bb = 52; continue; } 0x5e4 => { bb = 53; continue; } 0x600 => { bb = 54; continue; } 0x647 => { bb = 55; continue; } 0x677 => { bb = 56; continue; } 0x685 => { bb = 57; continue; } 0x692 => { bb = 58; continue; } 0x69b => { bb = 59; continue; } 0x6a0 => { bb = 60; continue; } 0x6a9 => { bb = 61; continue; } 0x6c5 => { bb = 62; continue; } 0x712 => { bb = 63; continue; } 0x742 => { bb = 64; continue; } 0x750 => { bb = 65; continue; } 0x75d => { bb = 66; continue; } 0x768 => { bb = 67; continue; } 0x771 => { bb = 68; continue; } 0x78d => { bb = 69; continue; } 0x7c4 => { bb = 70; continue; } 0x7f4 => { bb = 71; continue; } 0x802 => { bb = 72; continue; } 0x80f => { bb = 73; continue; } 0x81a => { bb = 74; continue; } 0x823 => { bb = 75; continue; } 0x83b => { bb = 76; continue; } 0x876 => { bb = 77; continue; } 0x8a6 => { bb = 78; continue; } 0x8b4 => { bb = 79; continue; } 0x8c1 => { bb = 80; continue; } 0x8c7 => { bb = 81; continue; } 0x8cd => { bb = 82; continue; } 0x8db => { bb = 83; continue; } 0x8ec => { bb = 84; continue; } 0x92b => { bb = 85; continue; } 0x96e => { bb = 86; continue; } 0x9bb => { bb = 87; continue; } 0xa22 => { bb = 88; continue; } 0xa2f => { bb = 89; continue; } 0xa40 => { bb = 90; continue; } 0xa5b => { bb = 91; continue; } 0xa6e => { bb = 92; continue; } 0xa71 => { bb = 93; continue; } 0xa76 => { bb = 94; continue; } 0xa82 => { bb = 95; continue; } 0xa8f => { bb = 96; continue; } 0xa98 => { bb = 97; continue; } 0xaac => { bb = 98; continue; } 0xaaf => { bb = 99; continue; } 0xac3 => { bb = 100; continue; } 0xb16 => { bb = 101; continue; } 0xb21 => { bb = 102; continue; } 0xb3a => { bb = 103; continue; } 0xb6c => { bb = 104; continue; } _ => { drop(_eh); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(j_tgt, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; } } }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252a340(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14; const P1: u64 = 0x18; const P2: u64 = 0x4;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rcx, ((r8 as u32)) as u32);
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    rax = rcx;
    crate::simd_ops::wu128(rcx.wrapping_add(P2), (xmm0) as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252a360(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x2000; const P2: u64 = 0x141834d64;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdi = rdx; rsi = rcx; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = ((rax.wrapping_add(P0)) as u64) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = ((rax).wrapping_add(0x7u64)) as u64; rcx = ((r8.wrapping_add(P1)) as u64) as u64; rax = ((rax).wrapping_add(r8)) as u64; rax = (((rax) & (0xfffffffffffffff8u64))) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = rax; rcx = ((rcx).wrapping_sub(r8)) as u64; rcx = ((rcx).wrapping_add(rdi)) as u64; crate::simd_ops::wu64(rsi.wrapping_add(P0), rcx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { rcx = 0x2000; fl_l = rdi; fl_r = rcx; rcx = (if (((fl_l as u64)) > ((fl_r as u64))) { rdi } else { rcx }) as u64; rcx = ((rcx).wrapping_add(0x8u64)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rax, rcx); crate::simd_ops::wu64(rsi, rax); rax = ((rax).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rsi.wrapping_add(P0), rdi); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252a400(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14336f5a4; const P1: u64 = 0x1425292a0;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = rdx;
    r8 = P0;
    rdx = rsp.wrapping_add(0x20);
    r9 = 0x3;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rax)) as u64;
    rax = rbx;
    crate::simd_ops::wu64(rbx, rcx);
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252a440(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x141470f70; const P3: u64 = 0x141471180; const P4: u64 = 0x141471080;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = r8; rsi = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rdi = rcx; rax = (0xffffffffffffffffu64) as u64; bb = 1; continue; }
            1 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r8.wrapping_add(rax))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 1; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), ((r9 as u32)) as u32); rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shr(2u32)) as u64; fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; fl_r = rcx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rdx = rsp.wrapping_add(0x20); rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rdx = rsp.wrapping_add(0x20); rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi, rbx); rax = rsi; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252a4d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x18; const P2: u64 = 0x1; const P3: u64 = 0x2; const P4: u64 = 0x14129dff0;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64; r9 = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 24; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_r = (0xfu64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r10 = rax; rcx = rax; bb = 4; continue; }
            3 => { rax = r9; r10 = r9; bb = 4; continue; }
            4 => { rdx = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0xdu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P2))) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = ((rcx.wrapping_add(P3)) as u64) as u64; bb = 9; continue; }
            7 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rax = ((rcx.wrapping_add(P2)) as u64) as u64; bb = 9; continue; }
            9 => { rdx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; bb = 10; continue; }
            10 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 17; continue; } bb = 11; continue; }
            11 => { bb = 12; continue; }
            12 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0xdu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 15; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P2))) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 15; continue; } bb = 14; continue; }
            14 => { fl_l = ((rax).wrapping_add(0x2u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; rdx = (rdx & !0xffu64) | (((0xau8) as u8 as u64)); bb = 16; continue; }
            15 => { fl_l = ((rax).wrapping_add(1)) as u64; rax = ((fl_l as u64)) as u64; bb = 16; continue; }
            16 => { crate::simd_ops::wu8(r10, ((rdx as u8)) as u8); rcx = ((r10.wrapping_add(P2)) as u64) as u64; rdx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; r10 = rcx; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 12; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(r9.wrapping_add(P1))) as u64; rcx = r9; fl_l = rax; fl_r = (0xfu64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(r9)) as u64; bb = 19; continue; }
            19 => { r10 = ((r10).wrapping_sub(rcx)) as u64; rcx = (crate::simd_ops::ru64(r9.wrapping_add(P0))) as u64; fl_l = r10; fl_r = rcx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 23; continue; } bb = 20; continue; }
            20 => { crate::simd_ops::wu64(r9.wrapping_add(P0), r10); fl_l = rax; fl_r = (0xfu64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 22; continue; } bb = 21; continue; }
            21 => { r9 = (crate::simd_ops::ru64(r9)) as u64; bb = 22; continue; }
            22 => { crate::simd_ops::wu8(r9.wrapping_add(r10), (0x0u8) as u8); return rax; }
            23 => { r10 = ((r10).wrapping_sub(rcx)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = r10; rcx = r9; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            24 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252a590(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14183acd4; const P1: u64 = 0x140000000; const P2: u64 = 0x14129dff0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rbx = (rcx) as u64; fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 110; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64 as u64; fl_r = (0xfu64) as u64 as u64; rdi = (rcx) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdi = (crate::simd_ops::ru64(rcx)) as u64; bb = 3; continue; }
            3 => { fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 110; continue; } bb = 4; continue; }
            4 => { r8 = (rsi) as u64; rdx = (0x5cu32) as u32 as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 110; continue; } bb = 5; continue; }
            5 => { rax = ((rax).wrapping_sub(rdi)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0xffffffffffffffffu64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 110; continue; } bb = 6; continue; }
            6 => { rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rbp = ((P1) as u64) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 7; continue; }
            7 => { r11 = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rax = (rbx) as u64; fl_l = (r11) as u64 as u64; fl_r = (0xfu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rbx)) as u64; bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(rdx))) as u8 as u64; fl_r = (0x5cu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 13; continue; } bb = 10; continue; }
            10 => { fl_l = (r11) as u64 as u64; fl_r = (0xfu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((crate::simd_ops::ru8(rdx.wrapping_add(rax))) as u32) as u32 as u64; crate::simd_ops::wu8(rax.wrapping_add(rdi), ((rcx as u8)) as u8); rdi = ((rdi).wrapping_add(1)) as u64; fl_l = ((rdx).wrapping_add(1)) as u64 as u64; rdx = ((fl_l as u64)) as u64; bb = 105; continue; }
            12 => { rcx = ((crate::simd_ops::ru8(rbx.wrapping_add(rdx))) as u32) as u32 as u64; rax = (rbx) as u64; crate::simd_ops::wu8(rbx.wrapping_add(rdi), ((rcx as u8)) as u8); rdi = ((rdi).wrapping_add(1)) as u64; fl_l = ((rdx).wrapping_add(1)) as u64 as u64; rdx = ((fl_l as u64)) as u64; bb = 105; continue; }
            13 => { rax = ((rdx.wrapping_add(0x1u64)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 113; continue; } bb = 14; continue; }
            14 => { r10 = (rbx) as u64; fl_l = (r11) as u64 as u64; fl_r = (0xfu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { r10 = (crate::simd_ops::ru64(rbx)) as u64; bb = 16; continue; }
            16 => { r10 = (((crate::simd_ops::ru8(r10.wrapping_add(rdx).wrapping_add(0x1u64))) as i8 as i64) as u64) as u64; rdx = ((rdx).wrapping_add(0x2u64)) as u64; r9 = (r10) as u64; fl_l = ((r10 as u32)) as u32 as u64; fl_r = (0x7au32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 79; continue; } bb = 17; continue; }
            17 => { rax = ((crate::simd_ops::ru8(r10.wrapping_add(rbp).wrapping_add(0x252aa60u64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru32(rbp.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x252aa44u64))) as u32 as u64; fl_l = ((rcx).wrapping_add(rbp)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; j_tgt = (rcx) as u64; bb = 0xffff_fffe; continue; }
            18 => { rax = (rbx) as u64; fl_l = (r11) as u64 as u64; fl_r = (0xfu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(rbx)) as u64; bb = 20; continue; }
            20 => { crate::simd_ops::wu8(rax.wrapping_add(rdi), (0xau8) as u8); bb = 104; continue; }
            21 => { rax = (rbx) as u64; fl_l = (r11) as u64 as u64; fl_r = (0xfu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru64(rbx)) as u64; bb = 23; continue; }
            23 => { crate::simd_ops::wu8(rax.wrapping_add(rdi), (0xau8) as u8); rdi = ((rdi).wrapping_add(1)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 106; continue; } bb = 24; continue; }
            24 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64 as u64; fl_r = (0xfu64) as u64 as u64; rax = (rbx) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 26; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(rbx)) as u64; bb = 26; continue; }
            26 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(rdx))) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 27; continue; }
            27 => { fl_l = ((rdx).wrapping_add(1)) as u64 as u64; rdx = ((fl_l as u64)) as u64; bb = 105; continue; }
            28 => { rax = ((rdx.wrapping_add(0x2u64)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 113; continue; } bb = 29; continue; }
            29 => { r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r10 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 30; continue; }
            30 => { rax = (rbx) as u64; fl_l = (r11) as u64 as u64; fl_r = (0xfu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 32; continue; } bb = 31; continue; }
            31 => { rax = (crate::simd_ops::ru64(rbx)) as u64; bb = 32; continue; }
            32 => { rax = ((rax).wrapping_add(r10)) as u64; rcx = (((crate::simd_ops::ru8(rax.wrapping_add(rdx))) as i8 as i32) as u32) as u32 as u64; r8 = ((rcx.wrapping_sub(0x30u64)) as u32) as u32 as u64; rcx = ((((rcx as u32)) | (0x20u32))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = (0xau32) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 36; continue; } bb = 33; continue; }
            33 => { rax = ((rcx.wrapping_sub(0x61u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x6u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 113; continue; } bb = 34; continue; }
            34 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = (0xau32) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 36; continue; } bb = 35; continue; }
            35 => { r8 = ((rcx.wrapping_sub(0x57u64)) as u32) as u32 as u64; bb = 36; continue; }
            36 => { r9 = (((r9 as u32)).wrapping_shl(4u32)) as u32 as u64; r10 = ((r10).wrapping_add(1)) as u64; r9 = (((r9 as u32)).wrapping_add((r8 as u32))) as u32 as u64; fl_l = (r10) as u64 as u64; fl_r = (0x2u64) as u64 as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 30; continue; } bb = 37; continue; }
            37 => { rax = (rbx) as u64; fl_l = (r11) as u64 as u64; fl_r = (0xfu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 39; continue; } bb = 38; continue; }
            38 => { rax = (crate::simd_ops::ru64(rbx)) as u64; bb = 39; continue; }
            39 => { crate::simd_ops::wu8(rax.wrapping_add(rdi), ((r9 as u8)) as u8); rdi = ((rdi).wrapping_add(1)) as u64; fl_l = ((rdx).wrapping_add(0x2u64)) as u64 as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 105; continue; }
            40 => { fl_l = (rdx) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 106; continue; } bb = 41; continue; }
            41 => { bb = 42; continue; }
            42 => { rax = (rbx) as u64; fl_l = (r11) as u64 as u64; fl_r = (0xfu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 44; continue; } bb = 43; continue; }
            43 => { rax = (crate::simd_ops::ru64(rbx)) as u64; bb = 44; continue; }
            44 => { rax = ((crate::simd_ops::ru8(rax.wrapping_add(rdx))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x20u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 46; continue; } bb = 45; continue; }
            45 => { rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_sub(0x9u8)) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if (((fl_l as u8)) > ((fl_r as u8))) { bb = 105; continue; } bb = 46; continue; }
            46 => { rdx = ((rdx).wrapping_add(1)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 42; continue; } bb = 47; continue; }
            47 => { bb = 106; continue; }
            48 => { rax = ((rdx.wrapping_add(0x3u64)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 113; continue; } bb = 49; continue; }
            49 => { rax = (rbx) as u64; fl_l = (r11) as u64 as u64; fl_r = (0xfu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 51; continue; } bb = 50; continue; }
            50 => { rax = (crate::simd_ops::ru64(rbx)) as u64; bb = 51; continue; }
            51 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(rdx))) as u8 as u64; fl_r = (0x7bu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 113; continue; } bb = 52; continue; }
            52 => { rdx = ((rdx).wrapping_add(1)) as u64; rax = (rbx) as u64; fl_l = (r11) as u64 as u64; fl_r = (0xfu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 54; continue; } bb = 53; continue; }
            53 => { rax = (crate::simd_ops::ru64(rbx)) as u64; bb = 54; continue; }
            54 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(rdx))) as u8 as u64; fl_r = (0x7du8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 113; continue; } bb = 55; continue; }
            55 => { r10 = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r11 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 56; continue; }
            56 => { fl_l = (rdx) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 113; continue; } bb = 57; continue; }
            57 => { rcx = (rbx) as u64; rax = (r10) as u64; fl_l = (r10) as u64 as u64; fl_r = (0xfu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 59; continue; } bb = 58; continue; }
            58 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; bb = 59; continue; }
            59 => { r8 = (((crate::simd_ops::ru8(rcx.wrapping_add(rdx))) as i8 as i32) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x7du8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 66; continue; } bb = 60; continue; }
            60 => { rcx = ((r8 as u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_add(0xffffffd0u32)) as u32 as u64; rcx = ((((rcx as u32)) | (0x20u32))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = (0xau32) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 64; continue; } bb = 61; continue; }
            61 => { rax = ((rcx.wrapping_sub(0x61u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x6u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 113; continue; } bb = 62; continue; }
            62 => { rax = (r10) as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = (0xau32) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 64; continue; } bb = 63; continue; }
            63 => { r8 = ((rcx.wrapping_sub(0x57u64)) as u32) as u32 as u64; bb = 64; continue; }
            64 => { r9 = (((r9 as u32)).wrapping_shl(4u32)) as u32 as u64; rdx = ((rdx).wrapping_add(1)) as u64; r9 = (((r9 as u32)).wrapping_add((r8 as u32))) as u32 as u64; r11 = (((r11 as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((r11 as u32)) as u32 as u64; fl_r = (0x10u32) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 56; continue; } bb = 65; continue; }
            65 => { fl_l = (rdx) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 113; continue; } bb = 66; continue; }
            66 => { rcx = (rbx) as u64; fl_l = (rax) as u64 as u64; fl_r = (0xfu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 68; continue; } bb = 67; continue; }
            67 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; bb = 68; continue; }
            68 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rdx))) as u8 as u64; fl_r = (0x7du8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 113; continue; } bb = 69; continue; }
            69 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (rbx) as u64; fl_l = (rax) as u64 as u64; fl_r = (0xfu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 71; continue; } bb = 70; continue; }
            70 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; bb = 71; continue; }
            71 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 73; continue; } bb = 72; continue; }
            72 => { rax = (0x1u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(rdi), ((r9 as u8)) as u8); fl_l = ((rdi).wrapping_add(rax)) as u64 as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 105; continue; }
            73 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = (0x800u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 75; continue; } bb = 74; continue; }
            74 => { rax = ((r9 as u32)) as u32 as u64; r9 = (r9 & !0xffu64) | ((((((r9 as u8)) & (0x3fu8))) as u8 as u64)); rax = (((rax as u32)).wrapping_shr(6u32)) as u32 as u64; r9 = (r9 & !0xffu64) | ((((((r9 as u8)) | (0x80u8))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) | (0xc0u8))) as u8 as u64)); crate::simd_ops::wu8(rcx.wrapping_add(rdi).wrapping_add(0x1u64), ((r9 as u8)) as u8); crate::simd_ops::wu8(rcx.wrapping_add(rdi), ((rax as u8)) as u8); rax = (0x2u32) as u32 as u64; fl_l = ((rdi).wrapping_add(rax)) as u64 as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 105; continue; }
            75 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = (0x10000u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 77; continue; } bb = 76; continue; }
            76 => { rax = ((r9 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(12u32)) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) | (0xe0u8))) as u8 as u64)); crate::simd_ops::wu8(rcx.wrapping_add(rdi), ((rax as u8)) as u8); rax = ((r9 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(6u32)) as u32 as u64; r9 = (r9 & !0xffu64) | ((((((r9 as u8)) & (0x3fu8))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x3fu8))) as u8 as u64)); r9 = (r9 & !0xffu64) | ((((((r9 as u8)) | (0x80u8))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) | (0x80u8))) as u8 as u64)); crate::simd_ops::wu8(rcx.wrapping_add(rdi).wrapping_add(0x2u64), ((r9 as u8)) as u8); crate::simd_ops::wu8(rcx.wrapping_add(rdi).wrapping_add(0x1u64), ((rax as u8)) as u8); rax = (0x3u32) as u32 as u64; fl_l = ((rdi).wrapping_add(rax)) as u64 as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 105; continue; }
            77 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = (0x110000u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 113; continue; } bb = 78; continue; }
            78 => { rax = ((r9 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(18u32)) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) | (0xf0u8))) as u8 as u64)); crate::simd_ops::wu8(rcx.wrapping_add(rdi), ((rax as u8)) as u8); rax = ((r9 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(12u32)) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x3fu8))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) | (0x80u8))) as u8 as u64)); crate::simd_ops::wu8(rcx.wrapping_add(rdi).wrapping_add(0x1u64), ((rax as u8)) as u8); rax = ((r9 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(6u32)) as u32 as u64; r9 = (r9 & !0xffu64) | ((((((r9 as u8)) & (0x3fu8))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x3fu8))) as u8 as u64)); r9 = (r9 & !0xffu64) | ((((((r9 as u8)) | (0x80u8))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) | (0x80u8))) as u8 as u64)); crate::simd_ops::wu8(rcx.wrapping_add(rdi).wrapping_add(0x3u64), ((r9 as u8)) as u8); crate::simd_ops::wu8(rcx.wrapping_add(rdi).wrapping_add(0x2u64), ((rax as u8)) as u8); rax = (0x4u32) as u32 as u64; fl_l = ((rdi).wrapping_add(rax)) as u64 as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 105; continue; }
            79 => { r8 = ((r10.wrapping_sub(0x30u64)) as u32) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = (0xau32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 92; continue; } bb = 80; continue; }
            80 => { r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 81; continue; }
            81 => { fl_l = (rdx) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 88; continue; } bb = 82; continue; }
            82 => { rax = (rbx) as u64; fl_l = (r11) as u64 as u64; fl_r = (0xfu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 84; continue; } bb = 83; continue; }
            83 => { rax = (crate::simd_ops::ru64(rbx)) as u64; bb = 84; continue; }
            84 => { rax = (((crate::simd_ops::ru8(rax.wrapping_add(rdx))) as i8 as i32) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_sub(0x30u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0xau32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 88; continue; } bb = 85; continue; }
            85 => { rax = (rbx) as u64; fl_l = (r11) as u64 as u64; fl_r = (0xfu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 87; continue; } bb = 86; continue; }
            86 => { rax = (crate::simd_ops::ru64(rbx)) as u64; bb = 87; continue; }
            87 => { rcx = (((crate::simd_ops::ru8(rax.wrapping_add(rdx))) as i8 as i32) as u32) as u32 as u64; r8 = ((r8.wrapping_add(r8.wrapping_mul(4))) as u32) as u32 as u64; rdx = ((rdx).wrapping_add(1)) as u64; r8 = ((r8.wrapping_sub(0x18u64)) as u32) as u32 as u64; r9 = (((r9 as u32)).wrapping_add(1)) as u32 as u64; r8 = ((rcx.wrapping_add(r8.wrapping_mul(2))) as u32) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 81; continue; } bb = 88; continue; }
            88 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = (0xffu32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 113; continue; } bb = 89; continue; }
            89 => { rax = (rbx) as u64; fl_l = (r11) as u64 as u64; fl_r = (0xfu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 91; continue; } bb = 90; continue; }
            90 => { rax = (crate::simd_ops::ru64(rbx)) as u64; bb = 91; continue; }
            91 => { crate::simd_ops::wu8(rax.wrapping_add(rdi), ((r8 as u8)) as u8); bb = 104; continue; }
            92 => { r9 = (((r9 as u32)).wrapping_add(0xffffff9fu32)) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = (0x15u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 101; continue; } bb = 93; continue; }
            93 => { rax = ((((r9 as u32)) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru32(rbp.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x252aadcu64))) as u32 as u64; fl_l = ((rcx).wrapping_add(rbp)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; j_tgt = (rcx) as u64; bb = 0xffff_fffe; continue; }
            94 => { r10 = (r10 & !0xffu64) | (((0x7u8) as u8 as u64)); bb = 101; continue; }
            95 => { r10 = (r10 & !0xffu64) | (((0x8u8) as u8 as u64)); bb = 101; continue; }
            96 => { r10 = (r10 & !0xffu64) | (((0xcu8) as u8 as u64)); bb = 101; continue; }
            97 => { r10 = (r10 & !0xffu64) | (((0xau8) as u8 as u64)); bb = 101; continue; }
            98 => { r10 = (r10 & !0xffu64) | (((0xdu8) as u8 as u64)); bb = 101; continue; }
            99 => { r10 = (r10 & !0xffu64) | (((0x9u8) as u8 as u64)); bb = 101; continue; }
            100 => { r10 = (r10 & !0xffu64) | (((0xbu8) as u8 as u64)); bb = 101; continue; }
            101 => { rax = (rbx) as u64; fl_l = (r11) as u64 as u64; fl_r = (0xfu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 103; continue; } bb = 102; continue; }
            102 => { rax = (crate::simd_ops::ru64(rbx)) as u64; bb = 103; continue; }
            103 => { crate::simd_ops::wu8(rax.wrapping_add(rdi), ((r10 as u8)) as u8); bb = 104; continue; }
            104 => { fl_l = ((rdi).wrapping_add(1)) as u64 as u64; rdi = ((fl_l as u64)) as u64; bb = 105; continue; }
            105 => { fl_l = (rdx) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 106; continue; }
            106 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 112; continue; } bb = 107; continue; }
            107 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (rdi) as u64); fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64 as u64; fl_r = (0xfu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 109; continue; } bb = 108; continue; }
            108 => { rbx = (crate::simd_ops::ru64(rbx)) as u64; bb = 109; continue; }
            109 => { crate::simd_ops::wu8(rdi.wrapping_add(rbx), (0x0u8) as u8); bb = 110; continue; }
            110 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 111; continue; }
            111 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            112 => { rdi = ((rdi).wrapping_sub(rax)) as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = (rdi) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 110; continue; }
            113 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 111; continue; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x14252a590u64) { 0x24 => { bb = 1; continue; } 0x2e => { bb = 2; continue; } 0x31 => { bb = 3; continue; } 0x3a => { bb = 4; continue; } 0x53 => { bb = 5; continue; } 0x60 => { bb = 6; continue; } 0x70 => { bb = 7; continue; } 0x7d => { bb = 8; continue; } 0x80 => { bb = 9; continue; } 0x86 => { bb = 10; continue; } 0x8c => { bb = 11; continue; } 0xa1 => { bb = 12; continue; } 0xb6 => { bb = 13; continue; } 0xc3 => { bb = 14; continue; } 0xcc => { bb = 15; continue; } 0xcf => { bb = 16; continue; } 0xe6 => { bb = 17; continue; } 0xfb => { bb = 18; continue; } 0x104 => { bb = 19; continue; } 0x107 => { bb = 20; continue; } 0x110 => { bb = 21; continue; } 0x119 => { bb = 22; continue; } 0x11c => { bb = 23; continue; } 0x12c => { bb = 24; continue; } 0x136 => { bb = 25; continue; } 0x139 => { bb = 26; continue; } 0x143 => { bb = 27; continue; } 0x14b => { bb = 28; continue; } 0x158 => { bb = 29; continue; } 0x160 => { bb = 30; continue; } 0x169 => { bb = 31; continue; } 0x16c => { bb = 32; continue; } 0x180 => { bb = 33; continue; } 0x18c => { bb = 34; continue; } 0x192 => { bb = 35; continue; } 0x196 => { bb = 36; continue; } 0x1a6 => { bb = 37; continue; } 0x1af => { bb = 38; continue; } 0x1b2 => { bb = 39; continue; } 0x1c2 => { bb = 40; continue; } 0x1cb => { bb = 41; continue; } 0x1d0 => { bb = 42; continue; } 0x1d9 => { bb = 43; continue; } 0x1dc => { bb = 44; continue; } 0x1e4 => { bb = 45; continue; } 0x1ee => { bb = 46; continue; } 0x1f6 => { bb = 47; continue; } 0x1fb => { bb = 48; continue; } 0x208 => { bb = 49; continue; } 0x211 => { bb = 50; continue; } 0x214 => { bb = 51; continue; } 0x21e => { bb = 52; continue; } 0x22a => { bb = 53; continue; } 0x22d => { bb = 54; continue; } 0x237 => { bb = 55; continue; } 0x241 => { bb = 56; continue; } 0x24a => { bb = 57; continue; } 0x256 => { bb = 58; continue; } 0x259 => { bb = 59; continue; } 0x264 => { bb = 60; continue; } 0x274 => { bb = 61; continue; } 0x280 => { bb = 62; continue; } 0x289 => { bb = 63; continue; } 0x28d => { bb = 64; continue; } 0x2a0 => { bb = 65; continue; } 0x2a9 => { bb = 66; continue; } 0x2b2 => { bb = 67; continue; } 0x2b5 => { bb = 68; continue; } 0x2bf => { bb = 69; continue; } 0x2cb => { bb = 70; continue; } 0x2ce => { bb = 71; continue; } 0x2d7 => { bb = 72; continue; } 0x2e8 => { bb = 73; continue; } 0x2f1 => { bb = 74; continue; } 0x316 => { bb = 75; continue; } 0x31f => { bb = 76; continue; } 0x352 => { bb = 77; continue; } 0x35f => { bb = 78; continue; } 0x3a0 => { bb = 79; continue; } 0x3aa => { bb = 80; continue; } 0x3b0 => { bb = 81; continue; } 0x3b5 => { bb = 82; continue; } 0x3be => { bb = 83; continue; } 0x3c1 => { bb = 84; continue; } 0x3cd => { bb = 85; continue; } 0x3d6 => { bb = 86; continue; } 0x3d9 => { bb = 87; continue; } 0x3f5 => { bb = 88; continue; } 0x402 => { bb = 89; continue; } 0x40b => { bb = 90; continue; } 0x40e => { bb = 91; continue; } 0x414 => { bb = 92; continue; } 0x41e => { bb = 93; continue; } 0x42d => { bb = 94; continue; } 0x432 => { bb = 95; continue; } 0x437 => { bb = 96; continue; } 0x43c => { bb = 97; continue; } 0x441 => { bb = 98; continue; } 0x446 => { bb = 99; continue; } 0x44b => { bb = 100; continue; } 0x44e => { bb = 101; continue; } 0x457 => { bb = 102; continue; } 0x45a => { bb = 103; continue; } 0x45e => { bb = 104; continue; } 0x461 => { bb = 105; continue; } 0x46a => { bb = 106; continue; } 0x473 => { bb = 107; continue; } 0x47e => { bb = 108; continue; } 0x481 => { bb = 109; continue; } 0x485 => { bb = 110; continue; } 0x487 => { bb = 111; continue; } 0x49c => { bb = 112; continue; } 0x4af => { bb = 113; continue; } _ => { let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(j_tgt, rcx, rdx, r8, r9, s5, s6, s7, s8); } } }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252ab40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x10; const P2: u64 = 0x14; const P3: u64 = 0x18; const P4: u64 = 0x40; const P5: u64 = 0x14252abe0;
    let (mut rax, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; let y_addr: u64 = rcx.wrapping_add(P0); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); rsi = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; r14 = rdx; rdi = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; rbp = rcx; rbx = (crate::simd_ops::ru32(rcx.wrapping_add(P3))) as u32 as u64; let y_addr: u64 = rsp.wrapping_add(0x30u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(P4))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); let y_addr: u64 = rax; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = r14; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rsp.wrapping_add(0x30u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_add(0x40u64), (xmm0) as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((rsi as u32)) as u32); rax = r14; crate::simd_ops::wu32(rbp.wrapping_add(0x14u64), ((rdi as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x18u64), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252abe0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x58; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x24; const P4: u64 = 0x40; const P5: u64 = 0x1425279d0; const P6: u64 = 0x20; const P7: u64 = 0x14; const P8: u64 = 0x1; const P9: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rsi = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64; rbx = rcx; r9 = (r9 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 9; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((crate::simd_ops::ru8(rdx.wrapping_add(rax))) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x20u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 15; continue; } bb = 4; continue; }
            4 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x9u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 15; continue; } bb = 5; continue; }
            5 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xdu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 15; continue; } bb = 6; continue; }
            6 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 16; continue; } bb = 7; continue; }
            7 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xbu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 17; continue; } bb = 8; continue; }
            8 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xcu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 17; continue; } bb = 9; continue; }
            9 => { fl_l = ((r9 as u8)) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(P3))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbx.wrapping_add(P4), (xmm0) as u128); bb = 11; continue; }
            11 => { rdx = rsp.wrapping_add(0x20); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (r9 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; let y_addr: u64 = rax; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbx.wrapping_add(P6); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 14; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x118; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 1; continue; } bb = 13; continue; }
            13 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x119; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 1; continue; } bb = 14; continue; }
            14 => { rax = ((rbx.wrapping_add(P6)) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            15 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 17; continue; } bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(P7), (1) as u32)) as u32 as u64; rax = ((rdx.wrapping_add(P8)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P9), ((rax as u32)) as u32); bb = 17; continue; }
            17 => { rax = ((rdx.wrapping_add(P8)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P2), ((rax as u32)) as u32); bb = 2; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252acb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x24; const P3: u64 = 0x40; const P4: u64 = 0x1425279d0; const P5: u64 = 0x20; const P6: u64 = 0x14; const P7: u64 = 0x1; const P8: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rsi = (((rdx as u8)) as u32) as u32 as u64; rbx = rcx; bb = 1; continue; }
            1 => { r9 = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; fl_l = rdx; fl_r = r9; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 9; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((crate::simd_ops::ru8(rdx.wrapping_add(rax))) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x20u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 15; continue; } bb = 4; continue; }
            4 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x9u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 15; continue; } bb = 5; continue; }
            5 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xdu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 15; continue; } bb = 6; continue; }
            6 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 16; continue; } bb = 7; continue; }
            7 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xbu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 17; continue; } bb = 8; continue; }
            8 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xcu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 17; continue; } bb = 9; continue; }
            9 => { fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(P2))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbx.wrapping_add(P3), (xmm0) as u128); bb = 11; continue; }
            11 => { rdx = rsp.wrapping_add(0x20); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (r8 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; let y_addr: u64 = rax; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbx.wrapping_add(P5); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 14; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x118; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 1; continue; } bb = 13; continue; }
            13 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x119; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 1; continue; } bb = 14; continue; }
            14 => { rax = ((rbx.wrapping_add(P5)) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            15 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 17; continue; } bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(P6), (1) as u32)) as u32 as u64; rax = ((rdx.wrapping_add(P7)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P8), ((rax as u32)) as u32); bb = 17; continue; }
            17 => { rax = ((rdx.wrapping_add(P7)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P1), ((rax as u32)) as u32); bb = 2; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252ad90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x18; const P2: u64 = 0x8; const P3: u64 = 0x20; const P4: u64 = 0x34; const P5: u64 = 0x38; const P6: u64 = 0x24; const P7: u64 = 0x40; const P8: u64 = 0x48; const P9: u64 = 0x60; const P10: u64 = 0x68; const P11: u64 = 0x70;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x18u64)) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx, rdx);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), r8);
    crate::simd_ops::wu32(rcx.wrapping_add(P3), ((rax as u32)) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(P4), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rcx.wrapping_add(P5), rax);
    crate::simd_ops::wu64(rsp, rax);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rax);
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P6), (xmm0) as u128);
    crate::simd_ops::wu64(rcx.wrapping_add(P7), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P8), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P9), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P10), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P11), rax);
    rax = rcx;
    crate::simd_ops::wu64(rcx.wrapping_add(0x50u64), r9);
    crate::simd_ops::wu16(rcx.wrapping_add(0x58u64), (0x100u16) as u16);
    rsp = ((rsp).wrapping_add(0x18u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252adf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rdx)) as u32 as u64;
    rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64;
    rax = (rax & !0xffu64) | (((if (((fl_l as u32)) < ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252ae10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14306e9c0; const P1: u64 = 0xc; const P2: u64 = 0x143073d78; const P3: u64 = 0x4;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = ((rcx as u32)) as u32 as u64; r9 = P0; rax = 0x6fa; bb = 1; continue; }
            1 => { r8 = rax; r8 = ((r8).wrapping_shr(1u32)) as u64; rdx = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; rcx = (crate::simd_ops::ru32(r9.wrapping_add(rdx.wrapping_mul(4)))) as u32 as u64; r10 = ((r9.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; rcx = ((((rcx as u32)) & (0xffffffu32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (0xffffffffffffffffu64) as u64; r9 = ((r10.wrapping_add(P1)) as u64) as u64; rcx = ((rcx).wrapping_sub(r8)) as u64; fl_l = ((rax).wrapping_add(rcx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 4; continue; }
            3 => { rax = r8; bb = 4; continue; }
            4 => { fl_l = rax; fl_r = rax; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) > 0) { bb = 1; continue; } bb = 5; continue; }
            5 => { rax = P2; fl_l = r9; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(r9)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = ((r9.wrapping_add(P3)) as u64) as u64; return rax; }
            8 => { rax = 0; fl_l = 0; fl_r = 0; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252ae80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434278a0; const P1: u64 = 0x1434237b8; const P2: u64 = 0x1425274d0; const P3: u64 = 0x1401e9110; const P4: u64 = 0x14183acc2; const P5: u64 = 0x14252f360; const P6: u64 = 0x14252f430; const P7: u64 = 0x140208c90; const P8: u64 = 0x143423730; const P9: u64 = 0x143423740; const P10: u64 = 0x14342374c; const P11: u64 = 0x143423754; const P12: u64 = 0x14252b3c0; const P13: u64 = 0x1426467f0; const P14: u64 = 0x14252b33b; const P15: u64 = 0x141834d30; const P16: u64 = 0x14252b383; const P17: u64 = 0x1402032e0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x14252ae80, funcinfo_rva: 0x37fa878, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0xc8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (r8) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rsp.wrapping_sub(0x1fu64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x88u64)) as u64; r13 = (rdx) as u64; rdi = (rcx) as u64; rsi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x288u64))) as u32 as u64; xmm0 = 0u128; ymm0_hi = 0; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu128(rdx, (xmm0) as u128); crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu64(rdx.wrapping_add(0x18u64), (0xfu64) as u64); crate::simd_ops::wu8(rdx, ((rsi as u8)) as u8); bb = 67; continue; }
            2 => { crate::simd_ops::wu32(rbp.wrapping_add(0x7fu64), (0xffffffffu32) as u32); r15 = (rsi) as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x9u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(P0)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x7u64), (xmm1) as u128); crate::simd_ops::wu8(rbp.wrapping_sub(0x9u64), ((rsi as u8)) as u8); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x8u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { rbx = (rsi) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(0x1b0u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x1b8u64))) as u64; rax = ((rax).wrapping_sub(r9)) as u64; rax = ((((rax) as i64).wrapping_shr(4u32) as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { bb = 5; continue; }
            5 => { r10 = (crate::simd_ops::ru32(r9.wrapping_add(rsi).wrapping_add(0xcu64))) as u32 as u64; r10 = (((r10 as u32)).wrapping_sub(1)) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x38u64))) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(r10.wrapping_mul(4)))) as u32 as u64; r8 = (crate::simd_ops::ru32(r9.wrapping_add(rsi).wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(r8.wrapping_mul(4)))) as u32 as u64; r9 = ((crate::simd_ops::ru8(r9.wrapping_add(rsi).wrapping_add(0x4u64))) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rdx as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r10 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r8 as u32)) as u32); r8 = ((rbx as u32)) as u32 as u64; rdx = ((P1) as u64) as u64; rcx = ((rbp.wrapping_sub(0x9u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252af5f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = ((rbx).wrapping_add(1)) as u64; rsi = ((rsi.wrapping_add(0x10u64)) as u64) as u64; r9 = (crate::simd_ops::ru64(rdi.wrapping_add(0x1b0u64))) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x1b8u64))) as u64; rax = ((rax).wrapping_sub(r9)) as u64; rax = ((((rax) as i64).wrapping_shr(4u32) as u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { rsi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; rbx = ((rbx).wrapping_sub(crate::simd_ops::ru64(rdi.wrapping_add(0x20u64)))) as u64; rbx = ((((rbx) as i64).wrapping_shr(2u32) as u64)) as u64; xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x21u64), (xmm0) as u128); crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), (rsi) as u64); fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { rax = (0x3fffffffffffffffu64) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 68; continue; } bb = 9; continue; }
            9 => { rsi = ((rbx.wrapping_mul(4)) as u64) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252afc3); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r14 = (rax) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x21u64), (rax) as u64); r12 = ((rsi.wrapping_add(rax)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), (r12) as u64); rdx = (0xffffffffu32) as u32 as u64; rbx = ((rbx).wrapping_shl(2u32)) as u64; r8 = (rbx) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252afe6); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (r12) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), (rbx) as u64); rsi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 11; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x19u64))) as u64; r14 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x21u64))) as u64; bb = 11; continue; }
            11 => { r11 = (rsi) as u64; r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; rax = ((rax).wrapping_sub(r8)) as u64; rax = ((((rax) as i64).wrapping_shr(2u32) as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 12; continue; }
            12 => { bb = 13; continue; }
            13 => { rdx = ((r11 as u32)) as u32 as u64; rcx = (crate::simd_ops::ru32(r8.wrapping_add(r11.wrapping_mul(4)))) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252b027); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = (rax as u32 as i32 as i64) as u64; crate::simd_ops::wu32(r14.wrapping_add(rax.wrapping_mul(4)), ((rsi as u32)) as u32); bb = 15; continue; }
            15 => { r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; rcx = ((crate::simd_ops::ru8(r8.wrapping_add(r11.wrapping_mul(4)))) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252b03f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; r11 = ((r11).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; rax = ((rax).wrapping_sub(r8)) as u64; rax = ((((rax) as i64).wrapping_shr(2u32) as u64)) as u64; fl_l = (r11) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 13; continue; } bb = 16; continue; }
            16 => { rdx = ((rsi as u32)) as u32 as u64; rcx = (rsi) as u64; rbx = ((rbx).wrapping_sub(r14)) as u64; rbx = ((((rbx) as i64).wrapping_shr(2u32) as u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 17; continue; }
            17 => { bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru32(r14.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rax = ((rdx as u32)) as u32 as u64; fl_l = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rdx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(rcx.wrapping_mul(4)), ((rax as u32)) as u32); bb = 20; continue; }
            20 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 18; continue; } bb = 21; continue; }
            21 => { r10 = (0xffffffffu32) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x67u64), ((r10 as u32)) as u32); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; rbx = ((rbx).wrapping_sub(crate::simd_ops::ru64(rdi.wrapping_add(0x20u64)))) as u64; rbx = ((((rbx) as i64).wrapping_shr(2u32) as u64)) as u64; rbx = ((rbx).wrapping_add(1)) as u64; r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x77u64))) as u64; rsi = (crate::simd_ops::ru64(r9.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(r9)) as u64; rcx = (rsi) as u64; rcx = ((rcx).wrapping_sub(rdx)) as u64; rcx = ((((rcx) as i64).wrapping_shr(2u32) as u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rax = ((rdx.wrapping_add(rbx.wrapping_mul(4))) as u64) as u64; crate::simd_ops::wu64(r9.wrapping_add(0x8u64), (rax) as u64); bb = 29; continue; }
            23 => { if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 29; continue; } bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru64(r9.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_sub(rdx)) as u64; rax = ((((rax) as i64).wrapping_shr(2u32) as u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 26; continue; } bb = 25; continue; }
            25 => { r8 = ((rbp.wrapping_add(0x67u64)) as u64) as u64; rdx = (rbx) as u64; rcx = (r9) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252b0e0); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x77u64))) as u64; bb = 29; continue; }
            26 => { fl_l = (rbx) as u64 as u64; fl_r = (rcx) as u64 as u64; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 28; continue; } bb = 27; continue; }
            27 => { rbx = ((rbx.wrapping_mul(4)) as u64) as u64; r8 = (rbx) as u64; r8 = (((r8) & (0xfffffffffffffffcu64))) as u64; rdx = ((r10 as u32)) as u32 as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252b105); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rsi).wrapping_add(rbx)) as u64 as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x77u64))) as u64; bb = 28; continue; }
            28 => { crate::simd_ops::wu64(r9.wrapping_add(0x8u64), (rsi) as u64); bb = 29; continue; }
            29 => { rsi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; rax = (rdx) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((((rax) as i64).wrapping_shr(2u32) as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 57; continue; } bb = 30; continue; }
            30 => { r13 = (crate::simd_ops::ru32(rbp.wrapping_add(0x7fu64))) as u32 as u64; bb = 31; continue; }
            31 => { r12 = ((rcx.wrapping_add(rsi.wrapping_mul(4))) as u64) as u64; rbx = ((crate::simd_ops::ru8(r12)) as u32) as u32 as u64; crate::simd_ops::wu8(rbp.wrapping_add(0x67u64), ((rbx as u8)) as u8); rax = (crate::simd_ops::ru64(r9)) as u64; rcx = ((rax.wrapping_add(rsi.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x7u64))) as u32 as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x41u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 33; continue; } bb = 32; continue; }
            32 => { rax = (0x1u32) as u32 as u64; bb = 55; continue; }
            33 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x288u64))) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 40; continue; } bb = 34; continue; }
            34 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x228u64))) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x230u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((((rax) as i64).wrapping_shr(3u32) as u64)) as u64; fl_l = (r15) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 40; continue; } bb = 35; continue; }
            35 => { bb = 36; continue; }
            36 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(r15.wrapping_mul(8)))) as u32 as u64; fl_l = (rax) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 40; continue; } bb = 37; continue; }
            37 => { rax = ((rdi.wrapping_add(0x240u64)) as u64) as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64 as u64; fl_r = (0xfu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 39; continue; } bb = 38; continue; }
            38 => { rax = (crate::simd_ops::ru64(rax)) as u64; bb = 39; continue; }
            39 => { r8 = (crate::simd_ops::ru32(rcx.wrapping_add(r15.wrapping_mul(8)).wrapping_add(0x4u64))) as u32 as u64; r8 = ((r8).wrapping_add(rax)) as u64; rdx = ((P8) as u64) as u64; rcx = ((rbp.wrapping_sub(0x9u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252b1bd); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r15 = ((r15).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x228u64))) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x230u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((((rax) as i64).wrapping_shr(3u32) as u64)) as u64; fl_l = (r15) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 36; continue; } bb = 40; continue; }
            40 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x288u64))) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 47; continue; } bb = 41; continue; }
            41 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x38u64))) as u64; rbx = (((crate::simd_ops::ru32(rax.wrapping_add(rsi.wrapping_mul(4)))) as i32 as i64) as u64) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 46; continue; } bb = 42; continue; }
            42 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 46; continue; } bb = 43; continue; }
            43 => { rcx = (rbx) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; r9 = (crate::simd_ops::ru64(rdi.wrapping_add(0x290u64))) as u64; r9 = ((r9).wrapping_add(0xffffffffffffffe0u64)) as u64; r9 = ((r9).wrapping_add(rcx)) as u64; fl_l = (crate::simd_ops::ru64(r9.wrapping_add(0x18u64))) as u64 as u64; fl_r = (0xfu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 45; continue; } bb = 44; continue; }
            44 => { r9 = (crate::simd_ops::ru64(r9)) as u64; bb = 45; continue; }
            45 => { r8 = ((rbx as u32)) as u32 as u64; rdx = ((P9) as u64) as u64; rcx = ((rbp.wrapping_sub(0x9u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252b226); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r13 = ((rbx as u32)) as u32 as u64; bb = 46; continue; }
            46 => { rbx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x67u64))) as u32) as u32 as u64; bb = 47; continue; }
            47 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x288u64))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 49; continue; } bb = 48; continue; }
            48 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x38u64))) as u64; r8 = (crate::simd_ops::ru32(rax.wrapping_add(rsi.wrapping_mul(4)))) as u32 as u64; rdx = ((P10) as u64) as u64; rcx = ((rbp.wrapping_sub(0x9u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252b24e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 49; continue; }
            49 => { r8 = (crate::simd_ops::ru32(r14.wrapping_add(rsi.wrapping_mul(4)))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = (0xffffffffu32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 51; continue; } bb = 50; continue; }
            50 => { rdx = ((P11) as u64) as u64; rcx = ((rbp.wrapping_sub(0x9u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252b268); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 51; continue; }
            51 => { rdx = ((rsi as u32)) as u32 as u64; rcx = (crate::simd_ops::ru32(r12)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252b273); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 53; continue; } bb = 52; continue; }
            52 => { rax = (rax as u32 as i32 as i64) as u64; r9 = (crate::simd_ops::ru32(r14.wrapping_add(rax.wrapping_mul(4)))) as u32 as u64; bb = 54; continue; }
            53 => { r9 = (0xffffffffu32) as u32 as u64; bb = 54; continue; }
            54 => { r8 = ((rbp.wrapping_sub(0x9u64)) as u64) as u64; rdx = (r12) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252b294); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (((rbx as u8)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252b29c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (rax as u32 as i32 as i64) as u64; r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x77u64))) as u64; bb = 55; continue; }
            55 => { rsi = ((rsi).wrapping_add(rax)) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; rax = (rdx) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((((rax) as i64).wrapping_shr(2u32) as u64)) as u64; fl_l = (rsi) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 31; continue; } bb = 56; continue; }
            56 => { r13 = (crate::simd_ops::ru64(rbp.wrapping_add(0x6fu64))) as u64; bb = 57; continue; }
            57 => { rdx = ((rdx).wrapping_sub(rcx)) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = (crate::simd_ops::ru64(r9)) as u64; rcx = ((rax.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x7u64))) as u32 as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let y_addr: u64 = rbp.wrapping_sub(0x9u64); xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); let y_addr: u64 = r13; core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); xmm1 = (crate::simd_ops::ru128(P0)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x7u64), (xmm1) as u128); crate::simd_ops::wu8(rbp.wrapping_sub(0x9u64), ((rax as u8)) as u8); fl_l = (r14) as u64 as u64; fl_r = (r14) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 62; continue; } bb = 58; continue; }
            58 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x11u64))) as u64; rdx = ((rdx).wrapping_sub(r14)) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rdx = ((rdx.wrapping_mul(4)) as u64) as u64; rax = (r14) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 61; continue; } bb = 59; continue; }
            59 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; r14 = (crate::simd_ops::ru64(r14.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(r14)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 61; continue; } bb = 60; continue; }
            60 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252b335); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P13)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P14, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            61 => { rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252b342); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P15) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 62; continue; }
            62 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0xfu64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0xfu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 67; continue; } bb = 63; continue; }
            63 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x9u64))) as u64; rax = (rcx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 66; continue; } bb = 64; continue; }
            64 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 66; continue; } bb = 65; continue; }
            65 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252b37d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P13)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P16, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            66 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252b387); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P15) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 67; continue; }
            67 => { rax = (r13) as u64; rsp = ((rsp).wrapping_add(0x88u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            68 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252b3a6); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P17) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252b3c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140000000; const P1: u64 = 0x1434230b8; const P2: u64 = 0x1425274d0; const P3: u64 = 0x1434230c8; const P4: u64 = 0x1434230e0; const P5: u64 = 0x143423078; const P6: u64 = 0x143423088; const P7: u64 = 0x1434230a0; const P8: u64 = 0x143423150; const P9: u64 = 0x143423168; const P10: u64 = 0x143423180; const P11: u64 = 0x143423198; const P12: u64 = 0x1434230f0; const P13: u64 = 0x143423108; const P14: u64 = 0x143423120; const P15: u64 = 0x143423138; const P16: u64 = 0x143423200; const P17: u64 = 0x143423220; const P18: u64 = 0x143423240; const P19: u64 = 0x143423258; const P20: u64 = 0x1434231b0; const P21: u64 = 0x1434231c8; const P22: u64 = 0x1434231e0; const P23: u64 = 0x1434231f0; const P24: u64 = 0x1434232e0; const P25: u64 = 0x1434232f0; const P26: u64 = 0x143423300; const P27: u64 = 0x143423318; const P28: u64 = 0x143423270; const P29: u64 = 0x143423288; const P30: u64 = 0x1434232a0; const P31: u64 = 0x1434232c0; const P32: u64 = 0x143423390; const P33: u64 = 0x1434233b0; const P34: u64 = 0x1434233c8; const P35: u64 = 0x1434233e0; const P36: u64 = 0x143423330; const P37: u64 = 0x143423348; const P38: u64 = 0x143423360; const P39: u64 = 0x143423378; const P40: u64 = 0x143423450; const P41: u64 = 0x143423468; const P42: u64 = 0x143423480; const P43: u64 = 0x143423498; const P44: u64 = 0x1434233f8; const P45: u64 = 0x143423410; const P46: u64 = 0x143423428; const P47: u64 = 0x143423438; const P48: u64 = 0x143423510; const P49: u64 = 0x143423528; const P50: u64 = 0x143423540; const P51: u64 = 0x143423550; const P52: u64 = 0x1434234b0; const P53: u64 = 0x1434234c0; const P54: u64 = 0x1434234d8; const P55: u64 = 0x1434234f0; const P56: u64 = 0x1434235c0; const P57: u64 = 0x1434235d8; const P58: u64 = 0x1434235f0; const P59: u64 = 0x1426cdcb0; const P60: u64 = 0x143423608; const P61: u64 = 0x143423560; const P62: u64 = 0x143423578; const P63: u64 = 0x143423590; const P64: u64 = 0x1434235a8; const P65: u64 = 0x143423678; const P66: u64 = 0x143423690; const P67: u64 = 0x143423698; const P68: u64 = 0x1434236a8; const P69: u64 = 0x143423618; const P70: u64 = 0x143423628; const P71: u64 = 0x143423640; const P72: u64 = 0x143423658; const P73: u64 = 0x1434236f8; const P74: u64 = 0x143423718; const P75: u64 = 0x143423724; const P76: u64 = 0x14342372c; const P77: u64 = 0x1434236bc; const P78: u64 = 0x1434236c0; const P79: u64 = 0x1434236d4; const P80: u64 = 0x1434236e0; const P81: u64 = 0x143423760; const P82: u64 = 0x143423778; const P83: u64 = 0x143423798; const P84: u64 = 0x14252c480; const P85: u64 = 0x143423098; const P86: u64 = 0x140571d70;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r10 = (crate::simd_ops::ru32(rdx)) as u32 as u64; r11 = ((r9 as u32)) as u32 as u64; rax = (((r10 as u8)) as u32) as u32 as u64; rdi = (r8) as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; rbx = (rdx) as u64; rsi = (rcx) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x4fu32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 89; continue; } bb = 1; continue; }
            1 => { rcx = ((P0) as u64) as u64; r9 = (crate::simd_ops::ru32(rcx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x252c334u64))) as u32 as u64; fl_l = ((r9).wrapping_add(rcx)) as u64 as u64; fl_r = fl_l; r9 = ((fl_l as u64)) as u64; j_tgt = (r9) as u64; bb = 0xffff_fffe; continue; }
            2 => { r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; rdx = ((P1) as u64) as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; rcx = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
            3 => { rcx = ((r10 as u32)) as u32 as u64; rax = ((r10 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(16u32)) as u32 as u64; r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(24u32)) as u32 as u64; r9 = (((rax as u8)) as u32) as u32 as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rcx as u32)) as u32); rdx = ((P3) as u64) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rdx = ((P4) as u64) as u64; rcx = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
            6 => { r9 = ((r10 as u32)) as u32 as u64; rdx = ((P5) as u64) as u64; r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; rcx = (rdi) as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; r9 = (((((r9 as u32)) as i32).wrapping_shr(16u32) as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
            7 => { rbx = ((r10 as u32)) as u32 as u64; rdx = ((P6) as u64) as u64; rbx = (((((rbx as u32)) as i32).wrapping_shr(16u32) as u32)) as u32 as u64; rcx = (rdi) as u64; r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; r9 = ((rbx as u32)) as u32 as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = ((rbx as u32)) as u32 as u64; bb = 88; continue; }
            8 => { rax = ((r10 as u32)) as u32 as u64; rdx = ((P7) as u64) as u64; rax = (((rax as u32)).wrapping_shr(16u32)) as u32 as u64; rcx = (rdi) as u64; r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; r9 = (((rax as u8)) as u32) as u32 as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
            9 => { r9 = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rcx = (rdi) as u64; r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; rdx = ((P8) as u64) as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32 as u64; bb = 88; continue; }
            10 => { r9 = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rcx = (rdi) as u64; r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; rdx = ((P9) as u64) as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32 as u64; bb = 88; continue; }
            11 => { rax = ((r10 as u32)) as u32 as u64; rdx = ((P10) as u64) as u64; rax = (((rax as u32)).wrapping_shr(16u32)) as u32 as u64; rcx = (rdi) as u64; r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; r9 = (((rax as u8)) as u32) as u32 as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
            12 => { rax = ((r10 as u32)) as u32 as u64; rdx = ((P11) as u64) as u64; rax = (((rax as u32)).wrapping_shr(16u32)) as u32 as u64; rcx = (rdi) as u64; r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; r9 = (((rax as u8)) as u32) as u32 as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
            13 => { r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; rdx = ((P12) as u64) as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; rcx = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
            14 => { rbx = ((r10 as u32)) as u32 as u64; rdx = ((P13) as u64) as u64; rbx = (((((rbx as u32)) as i32).wrapping_shr(16u32) as u32)) as u32 as u64; rcx = (rdi) as u64; r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; r9 = ((rbx as u32)) as u32 as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = ((rbx as u32)) as u32 as u64; bb = 88; continue; }
            15 => { rcx = ((r10 as u32)) as u32 as u64; rdx = ((P14) as u64) as u64; rax = ((r10 as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(24u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(16u32)) as u32 as u64; r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rcx as u32)) as u32); rcx = (rdi) as u64; r9 = (((rax as u8)) as u32) as u32 as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            16 => { rcx = ((r10 as u32)) as u32 as u64; rdx = ((P15) as u64) as u64; rax = ((r10 as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(24u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(16u32)) as u32 as u64; r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rcx as u32)) as u32); rcx = (rdi) as u64; r9 = (((rax as u8)) as u32) as u32 as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            17 => { rax = ((r10 as u32)) as u32 as u64; rcx = (rdi) as u64; rax = (((rax as u32)).wrapping_shr(16u32)) as u32 as u64; r9 = (((rax as u8)) as u32) as u32 as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((P16) as u64) as u64; r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32 as u64; bb = 88; continue; }
            18 => { rax = ((r10 as u32)) as u32 as u64; rcx = (rdi) as u64; rax = (((rax as u32)).wrapping_shr(16u32)) as u32 as u64; r9 = (((rax as u8)) as u32) as u32 as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((P17) as u64) as u64; r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32 as u64; bb = 88; continue; }
            19 => { rcx = ((r10 as u32)) as u32 as u64; rdx = ((P18) as u64) as u64; rcx = (((rcx as u32)).wrapping_shr(24u32)) as u32 as u64; rax = ((r10 as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(16u32)) as u32 as u64; r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rcx as u32)) as u32); rcx = (rdi) as u64; r9 = (((rax as u8)) as u32) as u32 as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            20 => { rcx = ((r10 as u32)) as u32 as u64; rdx = ((P19) as u64) as u64; rcx = (((rcx as u32)).wrapping_shr(24u32)) as u32 as u64; rax = ((r10 as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(16u32)) as u32 as u64; r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rcx as u32)) as u32); rcx = (rdi) as u64; r9 = (((rax as u8)) as u32) as u32 as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            21 => { r9 = ((r10 as u32)) as u32 as u64; rdx = ((P20) as u64) as u64; r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; rcx = (rdi) as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; r9 = (((((r9 as u32)) as i32).wrapping_shr(16u32) as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
            22 => { rax = ((r10 as u32)) as u32 as u64; rcx = (rdi) as u64; rax = (((rax as u32)).wrapping_shr(16u32)) as u32 as u64; r9 = (((rax as u8)) as u32) as u32 as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((P21) as u64) as u64; r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32 as u64; bb = 88; continue; }
            23 => { rcx = ((r10 as u32)) as u32 as u64; rdx = ((P22) as u64) as u64; rax = ((r10 as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(24u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub(1)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(16u32)) as u32 as u64; r9 = (((rax as u8)) as u32) as u32 as u64; r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; r9 = (((r9 as u32)).wrapping_sub(1)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rcx as u32)) as u32); rcx = (rdi) as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            24 => { rax = ((r10 as u32)) as u32 as u64; rdx = ((P23) as u64) as u64; rax = (((rax as u32)).wrapping_shr(16u32)) as u32 as u64; rcx = (rdi) as u64; r9 = (((rax as u8)) as u32) as u32 as u64; r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; r9 = (((r9 as u32)).wrapping_sub(1)) as u32 as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
            25 => { r8 = ((r11 as u32)) as u32 as u64; rdx = ((P24) as u64) as u64; rcx = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
            26 => { r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; rdx = ((P25) as u64) as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; r9 = ((r11 as u32)) as u32 as u64; rcx = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
            27 => { r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; rdx = ((P26) as u64) as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; r9 = ((r11 as u32)) as u32 as u64; rcx = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
            28 => { r9 = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rcx = (rdi) as u64; r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; rdx = ((P27) as u64) as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r11 as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            29 => { r9 = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rcx = (rdi) as u64; r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; rdx = ((P28) as u64) as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r11 as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            30 => { r9 = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rcx = (rdi) as u64; r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; rdx = ((P29) as u64) as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r11 as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            31 => { r9 = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rcx = (rdi) as u64; r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; rdx = ((P30) as u64) as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r11 as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            32 => { r9 = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rcx = (rdi) as u64; r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; rdx = ((P31) as u64) as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r11 as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            33 => { r9 = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rcx = (rdi) as u64; r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; rdx = ((P32) as u64) as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r11 as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            34 => { rcx = ((r10 as u32)) as u32 as u64; rdx = ((P33) as u64) as u64; rax = ((r10 as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(24u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(16u32)) as u32 as u64; r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rcx as u32)) as u32); rcx = (rdi) as u64; r9 = (((rax as u8)) as u32) as u32 as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            35 => { rcx = ((r10 as u32)) as u32 as u64; rdx = ((P34) as u64) as u64; rax = ((r10 as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(24u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(16u32)) as u32 as u64; r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rcx as u32)) as u32); rcx = (rdi) as u64; r9 = (((rax as u8)) as u32) as u32 as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            36 => { rcx = ((r10 as u32)) as u32 as u64; rdx = ((P35) as u64) as u64; rax = ((r10 as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(24u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(16u32)) as u32 as u64; r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rcx as u32)) as u32); rcx = (rdi) as u64; r9 = (((rax as u8)) as u32) as u32 as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            37 => { rcx = ((r10 as u32)) as u32 as u64; rdx = ((P36) as u64) as u64; rax = ((r10 as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(24u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(16u32)) as u32 as u64; r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rcx as u32)) as u32); rcx = (rdi) as u64; r9 = (((rax as u8)) as u32) as u32 as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            38 => { rcx = ((r10 as u32)) as u32 as u64; rdx = ((P37) as u64) as u64; rax = ((r10 as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(24u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(16u32)) as u32 as u64; r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rcx as u32)) as u32); rcx = (rdi) as u64; r9 = (((rax as u8)) as u32) as u32 as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            39 => { rcx = ((r10 as u32)) as u32 as u64; rdx = ((P38) as u64) as u64; rax = ((r10 as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(24u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(16u32)) as u32 as u64; r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rcx as u32)) as u32); rcx = (rdi) as u64; r9 = (((rax as u8)) as u32) as u32 as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            40 => { rax = ((r10 as u32)) as u32 as u64; rdx = ((P39) as u64) as u64; rbx = ((r10 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(16u32)) as u32 as u64; r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; rcx = (rdi) as u64; rbx = (((rbx as u32)).wrapping_shr(24u32)) as u32 as u64; r9 = (((rax as u8)) as u32) as u32 as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rbx as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = ((rbx as u32)) as u32 as u64; bb = 88; continue; }
            41 => { rax = ((r10 as u32)) as u32 as u64; rdx = ((P40) as u64) as u64; rbx = ((r10 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(16u32)) as u32 as u64; r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; rcx = (rdi) as u64; rbx = (((rbx as u32)).wrapping_shr(24u32)) as u32 as u64; r9 = (((rax as u8)) as u32) as u32 as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rbx as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = ((rbx as u32)) as u32 as u64; bb = 88; continue; }
            42 => { rax = ((r10 as u32)) as u32 as u64; rdx = ((P41) as u64) as u64; rbx = ((r10 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(16u32)) as u32 as u64; r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; rcx = (rdi) as u64; rbx = (((rbx as u32)).wrapping_shr(24u32)) as u32 as u64; r9 = (((rax as u8)) as u32) as u32 as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rbx as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = ((rbx as u32)) as u32 as u64; bb = 88; continue; }
            43 => { rax = ((r10 as u32)) as u32 as u64; rdx = ((P42) as u64) as u64; rbx = ((r10 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(16u32)) as u32 as u64; r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; rcx = (rdi) as u64; rbx = (((rbx as u32)).wrapping_shr(24u32)) as u32 as u64; r9 = (((rax as u8)) as u32) as u32 as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rbx as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = ((rbx as u32)) as u32 as u64; bb = 88; continue; }
            44 => { rax = ((r10 as u32)) as u32 as u64; rdx = ((P43) as u64) as u64; rbx = ((r10 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(16u32)) as u32 as u64; r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; rcx = (rdi) as u64; rbx = (((rbx as u32)).wrapping_shr(24u32)) as u32 as u64; r9 = (((rax as u8)) as u32) as u32 as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rbx as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = ((rbx as u32)) as u32 as u64; bb = 88; continue; }
            45 => { rax = ((r10 as u32)) as u32 as u64; rdx = ((P44) as u64) as u64; rbx = ((r10 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(16u32)) as u32 as u64; r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; rcx = (rdi) as u64; rbx = (((rbx as u32)).wrapping_shr(24u32)) as u32 as u64; r9 = (((rax as u8)) as u32) as u32 as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rbx as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = ((rbx as u32)) as u32 as u64; bb = 88; continue; }
            46 => { rcx = ((r10 as u32)) as u32 as u64; rdx = ((P45) as u64) as u64; rax = ((r10 as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(24u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(16u32)) as u32 as u64; r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rcx as u32)) as u32); rcx = (rdi) as u64; r9 = (((rax as u8)) as u32) as u32 as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            47 => { rcx = ((r10 as u32)) as u32 as u64; rdx = ((P46) as u64) as u64; rax = ((r10 as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(24u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(16u32)) as u32 as u64; r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rcx as u32)) as u32); rcx = (rdi) as u64; r9 = (((rax as u8)) as u32) as u32 as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            48 => { rax = ((r10 as u32)) as u32 as u64; rdx = ((P47) as u64) as u64; rbx = ((r10 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(16u32)) as u32 as u64; r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; rcx = (rdi) as u64; rbx = (((rbx as u32)).wrapping_shr(24u32)) as u32 as u64; r9 = (((rax as u8)) as u32) as u32 as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rbx as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = ((rbx as u32)) as u32 as u64; bb = 88; continue; }
            49 => { rax = ((r10 as u32)) as u32 as u64; rdx = ((P48) as u64) as u64; rbx = ((r10 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(16u32)) as u32 as u64; r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; rcx = (rdi) as u64; rbx = (((rbx as u32)).wrapping_shr(24u32)) as u32 as u64; r9 = (((rax as u8)) as u32) as u32 as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rbx as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = ((rbx as u32)) as u32 as u64; bb = 88; continue; }
            50 => { rcx = ((r10 as u32)) as u32 as u64; rdx = ((P49) as u64) as u64; rax = ((r10 as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(24u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(16u32)) as u32 as u64; r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rcx as u32)) as u32); rcx = (rdi) as u64; r9 = (((rax as u8)) as u32) as u32 as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            51 => { rax = ((r10 as u32)) as u32 as u64; rdx = ((P50) as u64) as u64; rax = (((rax as u32)).wrapping_shr(16u32)) as u32 as u64; rcx = (rdi) as u64; r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; r9 = (((rax as u8)) as u32) as u32 as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
            52 => { rax = ((r10 as u32)) as u32 as u64; rdx = ((P51) as u64) as u64; rax = (((rax as u32)).wrapping_shr(16u32)) as u32 as u64; rcx = (rdi) as u64; r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; r9 = (((rax as u8)) as u32) as u32 as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
            53 => { rax = ((r10 as u32)) as u32 as u64; rdx = ((P52) as u64) as u64; rax = (((rax as u32)).wrapping_shr(16u32)) as u32 as u64; rcx = (rdi) as u64; r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; r9 = (((rax as u8)) as u32) as u32 as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
            54 => { rcx = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rax = ((r10 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(16u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 56; continue; } bb = 55; continue; }
            55 => { r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 57; continue; }
            56 => { fl_l = (((rax as u32)).wrapping_sub(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; rdx = (0x1u32) as u32 as u64; r9 = (((rdx as u32)).wrapping_shl(((((rax as u32)) as u32) & 31))) as u32 as u64; bb = 57; continue; }
            57 => { r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; rdx = ((P53) as u64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rcx as u32)) as u32); rcx = (rdi) as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            58 => { r9 = ((r10 as u32)) as u32 as u64; rdx = ((P54) as u64) as u64; r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; rcx = (rdi) as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; r9 = (((((r9 as u32)) as i32).wrapping_shr(16u32) as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
            59 => { rcx = ((r10 as u32)) as u32 as u64; rax = ((r10 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(16u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(24u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub(1)) as u32 as u64; r9 = (((rax as u8)) as u32) as u32 as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((P55) as u64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rcx as u32)) as u32); rcx = (rdi) as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            60 => { r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; rdx = ((P56) as u64) as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; r9 = ((r11 as u32)) as u32 as u64; rcx = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
            61 => { r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; rdx = ((P57) as u64) as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; r9 = ((r11 as u32)) as u32 as u64; rcx = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
            62 => { r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; rdx = ((P58) as u64) as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; r9 = ((r11 as u32)) as u32 as u64; rcx = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
            63 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((P59) as u64) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdx = ((P60) as u64) as u64; rax = (((rax as u8)) as u32) as u32 as u64; r9 = ((r11 as u32)) as u32 as u64; rcx = (if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { rdx } else { rcx }) as u64; r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rcx) as u64); rdx = ((P61) as u64) as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; rcx = (rdi) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            64 => { r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; rdx = ((P62) as u64) as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; r9 = ((r11 as u32)) as u32 as u64; rcx = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
            65 => { r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; rdx = ((P63) as u64) as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; r9 = ((r11 as u32)) as u32 as u64; rcx = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
            66 => { rax = ((r10 as u32)) as u32 as u64; rdx = ((P64) as u64) as u64; rax = (((rax as u32)).wrapping_shr(16u32)) as u32 as u64; rcx = (rdi) as u64; r9 = (((rax as u8)) as u32) as u32 as u64; r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; r9 = (((r9 as u32)).wrapping_sub(1)) as u32 as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
            67 => { rbx = ((r10 as u32)) as u32 as u64; rdx = ((P65) as u64) as u64; rbx = (((((rbx as u32)) as i32).wrapping_shr(16u32) as u32)) as u32 as u64; rcx = (rdi) as u64; r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; r9 = ((rbx as u32)) as u32 as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = ((rbx as u32)) as u32 as u64; bb = 88; continue; }
            68 => { rdx = ((P66) as u64) as u64; rcx = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
            69 => { r8 = ((r11 as u32)) as u32 as u64; rdx = ((P67) as u64) as u64; rcx = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
            70 => { r9 = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rcx = (rdi) as u64; r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; rdx = ((P68) as u64) as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32 as u64; bb = 88; continue; }
            71 => { r8 = ((r11 as u32)) as u32 as u64; rdx = ((P69) as u64) as u64; rcx = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
            72 => { r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; rdx = ((P70) as u64) as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; r9 = ((r11 as u32)) as u32 as u64; rcx = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
            73 => { rax = ((r10 as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r11 as u32)) as u32); rax = (((rax as u32)).wrapping_shr(16u32)) as u32 as u64; rdx = ((P71) as u64) as u64; r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; rcx = (rdi) as u64; r9 = (((rax as u8)) as u32) as u32 as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            74 => { rax = ((r10 as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r11 as u32)) as u32); rax = (((rax as u32)).wrapping_shr(16u32)) as u32 as u64; rcx = (rdi) as u64; r9 = (((rax as u8)) as u32) as u32 as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((P72) as u64) as u64; r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            75 => { rax = ((r10 as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r11 as u32)) as u32); rax = (((rax as u32)).wrapping_shr(16u32)) as u32 as u64; rcx = (rdi) as u64; r9 = (((rax as u8)) as u32) as u32 as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((P73) as u64) as u64; r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32 as u64; bb = 88; continue; }
            76 => { rdx = ((P74) as u64) as u64; rcx = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
            77 => { rax = ((r10 as u32)) as u32 as u64; r9 = (0x52u32) as u32 as u64; rax = (((rax as u32)).wrapping_shr(16u32)) as u32 as u64; r10 = ((((r10 as u32)) & (0xff00u32))) as u32 as u64; rdx = (((rax as u8)) as u32) as u32 as u64; fl_l = ((r10 as u32)) as u32 as u64; fl_r = (0x200u32) as u32 as u64; rax = (0x55u32) as u32 as u64; r9 = (if (((fl_l as u32)) == ((fl_r as u32))) { (rax as u32) } else { (r9 as u32) }) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 79; continue; } bb = 78; continue; }
            78 => { rcx = ((P75) as u64) as u64; bb = 82; continue; }
            79 => { fl_l = ((r10 as u32)) as u32 as u64; fl_r = (0x100u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 81; continue; } bb = 80; continue; }
            80 => { rcx = ((P76) as u64) as u64; bb = 82; continue; }
            81 => { fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; rax = ((P77) as u64) as u64; rcx = ((P59) as u64) as u64; rcx = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { rax } else { rcx }) as u64; bb = 82; continue; }
            82 => { crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rdx as u32)) as u32); r8 = (rcx) as u64; rdx = ((P78) as u64) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            83 => { fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; rax = ((P79) as u64) as u64; rcx = ((P59) as u64) as u64; r9 = ((r11 as u32)) as u32 as u64; rcx = (if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { rax } else { rcx }) as u64; rdx = ((P80) as u64) as u64; r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rcx) as u64); rcx = (rdi) as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            84 => { r9 = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rax = ((P79) as u64) as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; rcx = ((P59) as u64) as u64; rdx = ((P81) as u64) as u64; rcx = (if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { rax } else { rcx }) as u64; r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rcx) as u64); r9 = ((((r9 as u32)) & (0x1u32))) as u32 as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; rcx = (rdi) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r11 as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            85 => { r9 = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((P82) as u64) as u64; bb = 87; continue; }
            86 => { r9 = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((P83) as u64) as u64; bb = 87; continue; }
            87 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; rax = ((P79) as u64) as u64; rcx = ((P59) as u64) as u64; rcx = (if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { rax } else { rcx }) as u64; r10 = (((r10 as u32)).wrapping_shr(8u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rcx) as u64); r9 = ((((r9 as u32)) & (0xffffffu32))) as u32 as u64; r8 = (((r10 as u8)) as u32) as u32 as u64; rcx = (rdi) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r11 as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32 as u64; fl_l = ((((r8 as u32)) & (0xffffffu32))) as u32 as u64; fl_r = fl_l; r8 = ((fl_l as u32)) as u32 as u64; bb = 88; continue; }
            88 => { rdx = (rdi) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P84) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = (0x2u32) as u32 as u64; rdx = ((P85) as u64) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P86) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 89; continue; }
            89 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x14252b3c0u64) { 0x2d => { bb = 1; continue; } 0x42 => { bb = 2; continue; } 0x68 => { bb = 3; continue; } 0x84 => { bb = 4; continue; } 0xa7 => { bb = 5; continue; } 0xc5 => { bb = 6; continue; } 0xf2 => { bb = 7; continue; } 0x11a => { bb = 8; continue; } 0x14a => { bb = 9; continue; } 0x16e => { bb = 10; continue; } 0x192 => { bb = 11; continue; } 0x1c2 => { bb = 12; continue; } 0x1f2 => { bb = 13; continue; } 0x218 => { bb = 14; continue; } 0x240 => { bb = 15; continue; } 0x27b => { bb = 16; continue; } 0x2b6 => { bb = 17; continue; } 0x2e7 => { bb = 18; continue; } 0x318 => { bb = 19; continue; } 0x355 => { bb = 20; continue; } 0x392 => { bb = 21; continue; } 0x3bf => { bb = 22; continue; } 0x3f0 => { bb = 23; continue; } 0x430 => { bb = 24; continue; } 0x463 => { bb = 25; continue; } 0x484 => { bb = 26; continue; } 0x4ad => { bb = 27; continue; } 0x4d6 => { bb = 28; continue; } 0x506 => { bb = 29; continue; } 0x536 => { bb = 30; continue; } 0x566 => { bb = 31; continue; } 0x596 => { bb = 32; continue; } 0x5c6 => { bb = 33; continue; } 0x5f6 => { bb = 34; continue; } 0x631 => { bb = 35; continue; } 0x66c => { bb = 36; continue; } 0x6a7 => { bb = 37; continue; } 0x6e2 => { bb = 38; continue; } 0x71d => { bb = 39; continue; } 0x758 => { bb = 40; continue; } 0x78b => { bb = 41; continue; } 0x7be => { bb = 42; continue; } 0x7f1 => { bb = 43; continue; } 0x824 => { bb = 44; continue; } 0x857 => { bb = 45; continue; } 0x88a => { bb = 46; continue; } 0x8c5 => { bb = 47; continue; } 0x900 => { bb = 48; continue; } 0x933 => { bb = 49; continue; } 0x966 => { bb = 50; continue; } 0x9a1 => { bb = 51; continue; } 0x9d1 => { bb = 52; continue; } 0xa01 => { bb = 53; continue; } 0xa31 => { bb = 54; continue; } 0xa41 => { bb = 55; continue; } 0xa46 => { bb = 56; continue; } 0xa52 => { bb = 57; continue; } 0xa7d => { bb = 58; continue; } 0xaaa => { bb = 59; continue; } 0xaee => { bb = 60; continue; } 0xb17 => { bb = 61; continue; } 0xb40 => { bb = 62; continue; } 0xb69 => { bb = 63; continue; } 0xbb6 => { bb = 64; continue; } 0xbdf => { bb = 65; continue; } 0xc08 => { bb = 66; continue; } 0xc3b => { bb = 67; continue; } 0xc63 => { bb = 68; continue; } 0xc81 => { bb = 69; continue; } 0xca2 => { bb = 70; continue; } 0xcc6 => { bb = 71; continue; } 0xce7 => { bb = 72; continue; } 0xd10 => { bb = 73; continue; } 0xd46 => { bb = 74; continue; } 0xd83 => { bb = 75; continue; } 0xdb9 => { bb = 76; continue; } 0xdd7 => { bb = 77; continue; } 0xdff => { bb = 78; continue; } 0xe08 => { bb = 79; continue; } 0xe11 => { bb = 80; continue; } 0xe1a => { bb = 81; continue; } 0xe2f => { bb = 82; continue; } 0xe55 => { bb = 83; continue; } 0xe9a => { bb = 84; continue; } 0xee8 => { bb = 85; continue; } 0xef5 => { bb = 86; continue; } 0xf00 => { bb = 87; continue; } 0xf41 => { bb = 88; continue; } 0xf61 => { bb = 89; continue; } _ => { let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(j_tgt, rcx, rdx, r8, r9, s5, s6, s7, s8); } } }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252c480(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140000000; const P1: u64 = 0x14336f5a4; const P2: u64 = 0x1425274d0; const P3: u64 = 0x1430782bc; const P4: u64 = 0x1430782b4; const P5: u64 = 0x1431edc44; const P6: u64 = 0x14342304c; const P7: u64 = 0x143422efc; const P8: u64 = 0x143423058; const P9: u64 = 0x143423064; const P10: u64 = 0x14342306c; const P11: u64 = 0x1434230b0; const P12: u64 = 0x143422f04;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r10 = (crate::simd_ops::ru64(rcx.wrapping_add(0x50u64))) as u64; rbx = (rdx) as u64; r9 = ((((r8 as u32)) as i32 as i64) as u64) as u64; r14 = (rcx) as u64; r9 = ((r9).wrapping_shl(4u32)) as u64; rcx = ((P0) as u64) as u64; r9 = ((r9).wrapping_add(r10)) as u64; rax = (((crate::simd_ops::ru32(r9)) as i32 as i64) as u64) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x252c704u64))) as u32 as u64; fl_l = ((rdx).wrapping_add(rcx)) as u64 as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; j_tgt = (rdx) as u64; bb = 0xffff_fffe; continue; }
            1 => { rdx = ((P1) as u64) as u64; rcx = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; }
            2 => { fl_l = (crate::simd_ops::ru8(r9.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rax = ((P3) as u64) as u64; r8 = ((P4) as u64) as u64; rcx = (rbx) as u64; r8 = (if (((fl_l as u8)) != ((fl_r as u8))) { rax } else { r8 }) as u64; rdx = ((P5) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; }
            3 => { xmm2 = ((crate::simd_ops::ru64(r9.wrapping_add(0x8u64))) as u64) as u128; r8 = ((xmm2 as u64)) as u64; rdx = ((P6) as u64) as u64; rcx = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; }
            4 => { rcx = (crate::simd_ops::ru32(r9.wrapping_add(0x8u64))) as u32 as u64; rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (((rcx as u32)).wrapping_sub(1)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r14.wrapping_add(0x210u64)))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(rax))) as u8 as u64; fl_r = (0x20u8) as u8 as u64; if (((fl_l as u8)) < ((fl_r as u8))) { bb = 23; continue; } bb = 7; continue; }
            7 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (rax) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 8; continue; }
            8 => { r9 = (crate::simd_ops::ru64(rcx)) as u64; rcx = (rbx) as u64; fl_l = (r8) as u64 as u64; fl_r = (0x20u64) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = ((P7) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; }
            10 => { r8 = (0x20u32) as u32 as u64; rdx = ((P8) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; }
            11 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rbp) as u64); rbp = (crate::simd_ops::ru32(r9.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rdi) as u64); rdi = ((rbp as u32)) as u32 as u64; rdi = (((rdi as u32)).wrapping_shr(30u32)) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 15; continue; } bb = 12; continue; }
            12 => { rax = ((rbp as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(20u32)) as u32 as u64; rax = ((((rax as u32)) & (0x3ffu32))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 16; continue; } bb = 13; continue; }
            13 => { rsi = ((rbp as u32)) as u32 as u64; rsi = (((rsi as u32)).wrapping_shr(10u32)) as u32 as u64; rsi = ((((rsi as u32)) & (0x3ffu32))) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 17; continue; } bb = 14; continue; }
            14 => { fl_l = ((((rbp as u32)) & (0x3ffu32))) as u32 as u64; fl_r = fl_l; rbp = ((fl_l as u32)) as u32 as u64; bb = 18; continue; }
            15 => { rcx = (0xffffffffu32) as u32 as u64; bb = 16; continue; }
            16 => { rsi = (0xffffffffu32) as u32 as u64; bb = 17; continue; }
            17 => { rbp = (0xffffffffu32) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 22; continue; } bb = 18; continue; }
            18 => { rax = ((((rcx as u32)) as i32 as i64) as u64) as u64; rdx = ((P9) as u64) as u64; rax = ((rax).wrapping_add(rax)) as u64; rcx = (rbx) as u64; r8 = (crate::simd_ops::ru32(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; r8 = (((r8 as u32)).wrapping_sub(1)) as u32 as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; r8 = ((r8).wrapping_add(crate::simd_ops::ru64(r14.wrapping_add(0x210u64)))) as u64; r9 = (crate::simd_ops::ru64(r8)) as u64; r8 = (crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rdi as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0x50u64))) as u64; rdx = ((P10) as u64) as u64; rcx = ((((rsi as u32)) as i32 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; r8 = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; rcx = (rbx) as u64; r8 = (((r8 as u32)).wrapping_sub(1)) as u32 as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; r8 = ((r8).wrapping_add(crate::simd_ops::ru64(r14.wrapping_add(0x210u64)))) as u64; r9 = (crate::simd_ops::ru64(r8)) as u64; r8 = (crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 20; continue; }
            20 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 22; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0x50u64))) as u64; rdx = ((P10) as u64) as u64; rcx = ((((rbp as u32)) as i32 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; r8 = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; rcx = (rbx) as u64; r8 = (((r8 as u32)).wrapping_sub(1)) as u32 as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; r8 = ((r8).wrapping_add(crate::simd_ops::ru64(r14.wrapping_add(0x210u64)))) as u64; r9 = (crate::simd_ops::ru64(r8)) as u64; r8 = (crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 22; continue; }
            22 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 23; continue; }
            23 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            24 => { rdx = ((P11) as u64) as u64; rcx = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; }
            25 => { rax = (crate::simd_ops::ru32(r9.wrapping_add(0x8u64))) as u32 as u64; r8 = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; r8 = ((r8).wrapping_add(crate::simd_ops::ru64(r14))) as u64; fl_l = (crate::simd_ops::ru64(r8.wrapping_add(0x60u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 23; continue; } bb = 26; continue; }
            26 => { r8 = ((r8).wrapping_add(0x50u64)) as u64; fl_l = (crate::simd_ops::ru64(r8.wrapping_add(0x18u64))) as u64 as u64; fl_r = (0xfu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 28; continue; } bb = 27; continue; }
            27 => { r8 = (crate::simd_ops::ru64(r8)) as u64; bb = 28; continue; }
            28 => { rdx = ((P12) as u64) as u64; rcx = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x14252c480u64) { 0x35 => { bb = 1; continue; } 0x4f => { bb = 2; continue; } 0x80 => { bb = 3; continue; } 0xa5 => { bb = 4; continue; } 0xc1 => { bb = 5; continue; } 0xc4 => { bb = 6; continue; } 0xce => { bb = 7; continue; } 0xd6 => { bb = 8; continue; } 0xe2 => { bb = 9; continue; } 0xf9 => { bb = 10; continue; } 0x116 => { bb = 11; continue; } 0x132 => { bb = 12; continue; } 0x143 => { bb = 13; continue; } 0x153 => { bb = 14; continue; } 0x15b => { bb = 15; continue; } 0x160 => { bb = 16; continue; } 0x165 => { bb = 17; continue; } 0x172 => { bb = 18; continue; } 0x1a6 => { bb = 19; continue; } 0x1d9 => { bb = 20; continue; } 0x1de => { bb = 21; continue; } 0x211 => { bb = 22; continue; } 0x220 => { bb = 23; continue; } 0x22c => { bb = 24; continue; } 0x246 => { bb = 25; continue; } 0x25b => { bb = 26; continue; } 0x266 => { bb = 27; continue; } 0x269 => { bb = 28; continue; } _ => { let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(j_tgt, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; } } }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252c730(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = 0; fl_l = 0; fl_r = 0; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(rcx))) as u8 as u64; fl_r = (0x20u8) as u8 as u64; if (((fl_l as u8)) < ((fl_r as u8))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
            4 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252c760(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (rax & !0xffu64) | (((0x3u8) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252c770(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x10; const P2: u64 = 0x140571d70;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x14252c770, funcinfo_rva: 0x37fa8f0, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x38, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = (rcx) as u64;
    rax = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32);
    xmm0 = 0u128;
    crate::simd_ops::wu128(rcx, (xmm0) as u128);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), (0xfu64) as u64);
    crate::simd_ops::wu8(rcx, ((rax as u8)) as u8);
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), (0x1u64) as u64);
    rax = (rcx) as u64;
    fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64 as u64; fl_r = (0xfu64) as u64 as u64;
    if !((((fl_l as u64)) <= ((fl_r as u64)))) { rax = (crate::simd_ops::ru64(rcx)) as u64; }
    crate::simd_ops::wu16(rax, (0x0u16) as u16);
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64 as u64; fl_r = (0xfu64) as u64 as u64;
    if !((((fl_l as u64)) <= ((fl_r as u64)))) { rdx = (crate::simd_ops::ru64(rdx)) as u64; }
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x14252c7c5); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    rax = (rbx) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252c7e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(r8)) as u32 as u64;
    fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    rax = (rax & !0xffu64) | (((if (((fl_l as u32)) < ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252c7f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141472e40; const P1: u64 = 0x1401e9110; const P2: u64 = 0x14183acc2; const P3: u64 = 0x141472810; const P4: u64 = 0x141473510; const P5: u64 = 0x14252f430; const P6: u64 = 0x1426467f0; const P7: u64 = 0x14252cc85; const P8: u64 = 0x141834d30; const P9: u64 = 0x14252ccc7; const P10: u64 = 0x14252cd09; const P11: u64 = 0x1402032e0; const P12: u64 = 0x14252cd3a;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x14252c7f0, funcinfo_rva: 0x37fa92c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0xc8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rsp.wrapping_sub(0x1fu64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x88u64)) as u64; rsi = (rcx) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xb0u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 71; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x88u64))) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x80u64))) as u64; r8 = (rdx) as u64; r8 = ((r8).wrapping_sub(rcx)) as u64; r8 = ((((r8) as i64).wrapping_shr(3u32) as u64)) as u64; r9 = ((crate::simd_ops::ru8(rbp.wrapping_add(0x67u64))) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252c83a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; rbx = ((rbx).wrapping_sub(crate::simd_ops::ru64(rsi.wrapping_add(0x20u64)))) as u64; rbx = ((((rbx) as i64).wrapping_shr(2u32) as u64)) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x1u64), (xmm0) as u128); rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r14 = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), (rdi) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0xfu64), (rdi) as u64); r15 = (0x3fffffffffffffffu64) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (rbx) as u64 as u64; fl_r = (r15) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 73; continue; } bb = 3; continue; }
            3 => { rbx = ((rbx.wrapping_mul(4)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252c884); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), (rax) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x1u64), (rax) as u64); r14 = ((rbx.wrapping_add(rax)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), (r14) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0xfu64), (r14) as u64); r8 = (rbx) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252c8a5); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbp.wrapping_add(0x7u64), (r14) as u64); bb = 5; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x1u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), (rax) as u64); bb = 5; continue; }
            5 => { xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x49u64), (xmm0) as u128); r12 = (rdi) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x7fu64), (rdi) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), (rdi) as u64); crate::simd_ops::wu128(rbp.wrapping_sub(0x31u64), (xmm0) as u128); r13 = (rdi) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x21u64), (rdi) as u64); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rsi.wrapping_add(0x20u64)))) as u64; rax = ((((rax) as i64).wrapping_shr(2u32) as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x6fu64), (rax) as u64); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = (rax) as u64 as u64; fl_r = (r15) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 74; continue; } bb = 7; continue; }
            7 => { rdx = ((rbp.wrapping_add(0x6fu64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x49u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252c8fe); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r12 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x39u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x7fu64), (r12) as u64); bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rsi.wrapping_add(0x20u64)))) as u64; rax = ((((rax) as i64).wrapping_shr(2u32) as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x6fu64), (rax) as u64); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { fl_l = (rax) as u64 as u64; fl_r = (r15) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 72; continue; } bb = 10; continue; }
            10 => { rdx = ((rbp.wrapping_add(0x6fu64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x31u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252c931); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r13 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x21u64))) as u64; bb = 11; continue; }
            11 => { r8 = (rdi) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), (rdi) as u64); r15 = (rdi) as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((((rax) as i64).wrapping_shr(2u32) as u64)) as u64; rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x41u64))) as u64; r14 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x49u64))) as u64; rdi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x29u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 43; continue; } bb = 12; continue; }
            12 => { bb = 15; continue; }
            13 => { bb = 14; continue; }
            14 => { r12 = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64; bb = 15; continue; }
            15 => { rdx = ((crate::simd_ops::ru8(rcx.wrapping_add(r15.wrapping_mul(4)))) as u32) as u32 as u64; crate::simd_ops::wu8(rbp.wrapping_add(0x6fu64), ((rdx as u8)) as u8); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x80u64))) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x88u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((((rax) as i64).wrapping_shr(3u32) as u64)) as u64; fl_l = (r8) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 32; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(r8.wrapping_mul(8)))) as u32 as u64; fl_l = (rax) as u64 as u64; fl_r = (r15) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 32; continue; } bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru32(rcx.wrapping_add(r8.wrapping_mul(8)).wrapping_add(0x4u64))) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub((rax as u32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub(1)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; fl_l = 0; fl_r = ((rax as u32)) as u32 as u64; rax = (0u32.wrapping_sub(fl_r as u32)) as u32 as u64; rax = (if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) < 0) { (rcx as u32) } else { (rax as u32) }) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x2aaau32) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 31; continue; } bb = 18; continue; }
            18 => { crate::simd_ops::wu32(rbp.wrapping_add(0x77u64), (0x10017u32) as u32); fl_l = (rbx) as u64 as u64; fl_r = (r12) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::wu32(rbx, (0x10017u32) as u32); fl_l = ((rbx).wrapping_add(0x4u64)) as u64 as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), (rbx) as u64); bb = 21; continue; }
            20 => { r8 = ((rbp.wrapping_add(0x77u64)) as u64) as u64; rdx = (rbx) as u64; rcx = ((rbp.wrapping_sub(0x49u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252c9e9); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r12 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x39u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x7fu64), (r12) as u64); rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x41u64))) as u64; r14 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x49u64))) as u64; bb = 21; continue; }
            21 => { crate::simd_ops::wu32(rbp.wrapping_add(0x77u64), (0x43u32) as u32); fl_l = (rbx) as u64 as u64; fl_r = (r12) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 23; continue; } bb = 22; continue; }
            22 => { crate::simd_ops::wu32(rbx, (0x43u32) as u32); fl_l = ((rbx).wrapping_add(0x4u64)) as u64 as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), (rbx) as u64); bb = 24; continue; }
            23 => { r8 = ((rbp.wrapping_add(0x77u64)) as u64) as u64; rdx = (rbx) as u64; rcx = ((rbp.wrapping_sub(0x49u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252ca25); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x39u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x7fu64), (rax) as u64); rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x41u64))) as u64; r14 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x49u64))) as u64; bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x38u64))) as u64; r8 = ((rax.wrapping_add(r15.wrapping_mul(4))) as u64) as u64; fl_l = (rdi) as u64 as u64; fl_r = (r13) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 26; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru32(r8)) as u32 as u64; crate::simd_ops::wu32(rdi, ((rax as u32)) as u32); fl_l = ((rdi).wrapping_add(0x4u64)) as u64 as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), (rdi) as u64); bb = 27; continue; }
            26 => { rdx = (rdi) as u64; rcx = ((rbp.wrapping_sub(0x31u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252ca5d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r13 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x21u64))) as u64; rdi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x29u64))) as u64; bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x38u64))) as u64; r8 = ((rax.wrapping_add(r15.wrapping_mul(4))) as u64) as u64; fl_l = (rdi) as u64 as u64; fl_r = (r13) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 29; continue; } bb = 28; continue; }
            28 => { rax = (crate::simd_ops::ru32(r8)) as u32 as u64; crate::simd_ops::wu32(rdi, ((rax as u32)) as u32); fl_l = ((rdi).wrapping_add(0x4u64)) as u64 as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), (rdi) as u64); bb = 30; continue; }
            29 => { rdx = (rdi) as u64; rcx = ((rbp.wrapping_sub(0x31u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252ca8d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r13 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x21u64))) as u64; rdi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x29u64))) as u64; bb = 30; continue; }
            30 => { r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x19u64))) as u64; rdx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x6fu64))) as u32) as u32 as u64; bb = 31; continue; }
            31 => { fl_l = ((r8).wrapping_add(1)) as u64 as u64; r8 = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), (r8) as u64); bb = 32; continue; }
            32 => { rcx = (((rdx as u8)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252caac); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 42; continue; } bb = 33; continue; }
            33 => { r12 = ((rax as u32)) as u32 as u64; bb = 34; continue; }
            34 => { rax = (rbx) as u64; rax = ((rax).wrapping_sub(r14)) as u64; rax = ((((rax) as i64).wrapping_shr(2u32) as u64)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x11u64))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(r15.wrapping_mul(4)), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; r8 = ((rax.wrapping_add(r15.wrapping_mul(4))) as u64) as u64; fl_l = (rbx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 36; continue; } bb = 35; continue; }
            35 => { rax = (crate::simd_ops::ru32(r8)) as u32 as u64; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); fl_l = ((rbx).wrapping_add(0x4u64)) as u64 as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), (rbx) as u64); bb = 37; continue; }
            36 => { rdx = (rbx) as u64; rcx = ((rbp.wrapping_sub(0x49u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252caf6); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x39u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x7fu64), (rax) as u64); rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x41u64))) as u64; r14 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x49u64))) as u64; bb = 37; continue; }
            37 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x38u64))) as u64; r8 = ((rax.wrapping_add(r15.wrapping_mul(4))) as u64) as u64; fl_l = (rdi) as u64 as u64; fl_r = (r13) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 39; continue; } bb = 38; continue; }
            38 => { rax = (crate::simd_ops::ru32(r8)) as u32 as u64; crate::simd_ops::wu32(rdi, ((rax as u32)) as u32); fl_l = ((rdi).wrapping_add(0x4u64)) as u64 as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), (rdi) as u64); bb = 40; continue; }
            39 => { rdx = (rdi) as u64; rcx = ((rbp.wrapping_sub(0x31u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252cb2e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r13 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x21u64))) as u64; rdi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x29u64))) as u64; bb = 40; continue; }
            40 => { r15 = ((r15).wrapping_add(1)) as u64; fl_l = (r12) as u64 as u64; fl_r = (0x1u64) as u64 as u64; r12 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 34; continue; } bb = 41; continue; }
            41 => { r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x19u64))) as u64; bb = 42; continue; }
            42 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((((rax) as i64).wrapping_shr(2u32) as u64)) as u64; fl_l = (r15) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 14; continue; } bb = 43; continue; }
            43 => { r15 = (crate::simd_ops::ru64(rsi.wrapping_add(0x88u64))) as u64; r9 = (crate::simd_ops::ru64(rsi.wrapping_add(0x80u64))) as u64; r12 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x11u64))) as u64; fl_l = (r9) as u64 as u64; fl_r = (r15) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 50; continue; } bb = 44; continue; }
            44 => { rsi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 45; continue; }
            45 => { rax = (crate::simd_ops::ru32(r9.wrapping_add(0x4u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(r9)) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_sub((rcx as u32))) as u32 as u64; rdx = (((rdx as u32)).wrapping_sub(1)) as u32 as u64; r11 = (crate::simd_ops::ru32(r12.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; r8 = (crate::simd_ops::ru32(r12.wrapping_add(rax.wrapping_mul(4)))) as u32 as u64; r8 = (((r8 as u32)).wrapping_sub((r11 as u32))) as u32 as u64; r8 = (((r8 as u32)).wrapping_sub(1)) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; fl_l = 0; fl_r = ((rax as u32)) as u32 as u64; rax = (0u32.wrapping_sub(fl_r as u32)) as u32 as u64; rax = (if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) < 0) { (rdx as u32) } else { (rax as u32) }) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x2aaau32) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 47; continue; } bb = 46; continue; }
            46 => { rax = ((r11.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::lk_and_u32(r14.wrapping_add(rax.wrapping_mul(4)), (0xffu32) as u32); r8 = (((r8 as u32)).wrapping_shl(8u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_add(0x100u32)) as u32 as u64; crate::simd_ops::lk_or_u32(r14.wrapping_add(rax.wrapping_mul(4)), ((r8 as u32)) as u32); crate::simd_ops::wu16(r14.wrapping_add(r11.wrapping_mul(4)).wrapping_add(0x2u64), ((rsi as u16)) as u16); fl_l = (crate::simd_ops::lk_or_u32(r14.wrapping_add(r11.wrapping_mul(4)), (0xfffe0000u32) as u32)) as u32 as u64; fl_r = fl_l; bb = 48; continue; }
            47 => { crate::simd_ops::wu16(r14.wrapping_add(r11.wrapping_mul(4)).wrapping_add(0x2u64), ((rsi as u16)) as u16); rax = (((r8 as u16)) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_shl(16u32)) as u32 as u64; fl_l = (crate::simd_ops::lk_or_u32(r14.wrapping_add(r11.wrapping_mul(4)), ((rax as u32)) as u32)) as u32 as u64; fl_r = fl_l; bb = 48; continue; }
            48 => { r9 = ((r9).wrapping_add(0x8u64)) as u64; fl_l = (r9) as u64 as u64; fl_r = (r15) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 45; continue; } bb = 49; continue; }
            49 => { rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; bb = 50; continue; }
            50 => { rcx = ((rsi.wrapping_add(0x20u64)) as u64) as u64; rax = ((rbp.wrapping_sub(0x49u64)) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 52; continue; } bb = 51; continue; }
            51 => { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rcx, (r14) as u64); r14 = (rax) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), (rbx) as u64); rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), (rdx) as u64); bb = 53; continue; }
            52 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64; bb = 53; continue; }
            53 => { rsi = ((rsi).wrapping_add(0x38u64)) as u64; rax = ((rbp.wrapping_sub(0x31u64)) as u64) as u64; fl_l = (rsi) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 55; continue; } bb = 54; continue; }
            54 => { rcx = (crate::simd_ops::ru64(rsi)) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x31u64))) as u64; crate::simd_ops::wu64(rsi, (rax) as u64); crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), (rdi) as u64); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), (r13) as u64); r13 = (rax) as u64; bb = 56; continue; }
            55 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x31u64))) as u64; bb = 56; continue; }
            56 => { fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 61; continue; } bb = 57; continue; }
            57 => { r13 = ((r13).wrapping_sub(rcx)) as u64; r13 = ((((r13) as i64).wrapping_shr(2u32) as u64)) as u64; rdx = ((r13.wrapping_mul(4)) as u64) as u64; rax = (rcx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 60; continue; } bb = 58; continue; }
            58 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 60; continue; } bb = 59; continue; }
            59 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252cc7f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P7, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            60 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252cc86); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 61; continue; }
            61 => { fl_l = (r14) as u64 as u64; fl_r = (r14) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 66; continue; } bb = 62; continue; }
            62 => { rbx = ((rbx).wrapping_sub(r14)) as u64; rbx = ((((rbx) as i64).wrapping_shr(2u32) as u64)) as u64; rdx = ((rbx.wrapping_mul(4)) as u64) as u64; rax = (r14) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 65; continue; } bb = 63; continue; }
            63 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; r14 = (crate::simd_ops::ru64(r14.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(r14)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 65; continue; } bb = 64; continue; }
            64 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252ccc1); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P9, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            65 => { rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252cccb); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 66; continue; }
            66 => { fl_l = (r12) as u64 as u64; fl_r = (r12) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 71; continue; } bb = 67; continue; }
            67 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x9u64))) as u64; rcx = ((rcx).wrapping_sub(r12)) as u64; rcx = (((rcx) & (0xfffffffffffffffcu64))) as u64; rax = (r12) as u64; fl_l = (rcx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 70; continue; } bb = 68; continue; }
            68 => { rcx = ((rcx).wrapping_add(0x27u64)) as u64; r12 = (crate::simd_ops::ru64(r12.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(r12)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 70; continue; } bb = 69; continue; }
            69 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252cd03); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P10, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            70 => { rdx = (rcx) as u64; rcx = (r12) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252cd10); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 71; continue; }
            71 => { rsp = ((rsp).wrapping_add(0x88u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            72 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252cd29); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 73; continue; }
            73 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252cd2f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 74; continue; }
            74 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252cd35); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P12, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252cd50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x8u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xb0u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r10 = rcx; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 15; continue; } bb = 1; continue; }
            1 => { r9 = (crate::simd_ops::ru64(rcx.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rbp = (crate::simd_ops::ru64(rcx.wrapping_add(0x88u64))) as u64; fl_l = r9; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdi); crate::simd_ops::wu64(rsp, r14); r14 = 0; fl_l = 0; fl_r = 0; bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru32(r9)) as u32 as u64; rdx = (crate::simd_ops::ru64(r10.wrapping_add(0x20u64))) as u64; r8 = ((rdi.wrapping_add(0x1u64)) as u32) as u32 as u64; r11 = ((rdi.wrapping_mul(4)) as u64) as u64; rsi = (crate::simd_ops::ru32(r11.wrapping_add(rdx))) as u32 as u64; rbx = ((r11.wrapping_add(rdx)) as u64) as u64; rax = ((rsi as u32)) as u32 as u64; rax = (((((rax as u32)) as i32).wrapping_shr(16u32) as u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_add((rax as u32))) as u32 as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(r8.wrapping_mul(4)))) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x17u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 4; continue; }
            4 => { bb = 5; continue; }
            5 => { rax = ((rcx as u32)) as u32 as u64; rax = (((((rax as u32)) as i32).wrapping_shr(16u32) as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; r8 = (((r8 as u32)).wrapping_add((rax as u32))) as u32 as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(r8.wrapping_mul(4)))) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x17u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } bb = 7; continue; }
            7 => { rdx = ((r8 as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_sub((rdi as u32))) as u32 as u64; rdx = (((rdx as u32)).wrapping_sub(1)) as u32 as u64; fl_l = ((rsi as u8)) as u8 as u64; fl_r = (0x17u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x16u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu32(rbx, ((rcx as u32)) as u32); rdx = (crate::simd_ops::ru64(r10.wrapping_add(0x38u64))) as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(r8.wrapping_mul(4)))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(r11), ((rcx as u32)) as u32); bb = 12; continue; }
            10 => { rax = ((((rdx as u16)) as i16 as i32) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu16(rbx.wrapping_add(0x2u64), ((r14 as u16)) as u16); rcx = (crate::simd_ops::ru64(r10.wrapping_add(0x20u64))) as u64; rax = (((rdx as u16)) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_shl(16u32)) as u32 as u64; fl_l = (crate::simd_ops::lk_or_u32(rcx.wrapping_add(r11), ((rax as u32)) as u32)) as u32 as u64; fl_r = fl_l; bb = 12; continue; }
            12 => { crate::simd_ops::wu32(r9.wrapping_add(0x4u64), ((r8 as u32)) as u32); r9 = ((r9).wrapping_add(0x8u64)) as u64; fl_l = r9; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 13; continue; }
            13 => { r14 = (crate::simd_ops::ru64(rsp)) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; bb = 14; continue; }
            14 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; bb = 15; continue; }
            15 => { rsp = ((rsp).wrapping_add(0x8u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252ce60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xffffffffffffffff;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    r9 = (crate::simd_ops::ru64(rcx)) as u64;
    r8 = rax;
    fl_l = rax; fl_r = rax;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    loop { rdx = ((crate::simd_ops::ru8(r9.wrapping_add(r8).wrapping_add(P1))) as u32) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(5u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rcx as u32))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(2u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rcx as u32))) as u32 as u64; rax = ((((rax as u32)) ^ ((rdx as u32)))) as u32 as u64; fl_l = r8; fl_r = (0x1u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252cea0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = ((rcx as u32)) as u32 as u64; r10 = ((rcx as u32)) as u32 as u64; r11 = (((r11 as u32)).wrapping_shr(30u32)) as u32 as u64; fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(20u32)) as u32 as u64; rax = ((((rax as u32)) & (0x3ffu32))) as u32 as u64; crate::simd_ops::wu32(rdx, ((rax as u32)) as u32); fl_l = ((r11 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rcx = (((rcx as u32)).wrapping_shr(10u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0x3ffu32))) as u32 as u64; crate::simd_ops::wu32(r8, ((rcx as u32)) as u32); fl_l = ((r11 as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = ((((r10 as u32)) & (0x3ffu32))) as u32 as u64; fl_r = fl_l; r10 = ((fl_l as u32)) as u32 as u64; rax = ((r11 as u32)) as u32 as u64; crate::simd_ops::wu32(r9, ((r10 as u32)) as u32); return rax; }
            4 => { crate::simd_ops::wu32(rdx, (0xffffffffu32) as u32); bb = 5; continue; }
            5 => { crate::simd_ops::wu32(r8, (0xffffffffu32) as u32); bb = 6; continue; }
            6 => { crate::simd_ops::wu32(r9, (0xffffffffu32) as u32); rax = ((r11 as u32)) as u32 as u64; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252cf00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (((rcx as u32)).wrapping_shl(10u32)) as u32 as u64;
    rcx = ((((rcx as u32)) | ((rdx as u32)))) as u32 as u64;
    rcx = (((rcx as u32)).wrapping_shl(10u32)) as u32 as u64;
    rcx = ((((rcx as u32)) | ((r8 as u32)))) as u32 as u64;
    fl_l = ((((rcx as u32)) | (0xc0000000u32))) as u32 as u64; fl_r = fl_l;
    rcx = ((fl_l as u32)) as u32 as u64;
    rax = ((rcx as u32)) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252cf20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    rcx = (((rcx as u32)).wrapping_shl(10u32)) as u32 as u64;
    rcx = ((((rcx as u32)) | ((rdx as u32)))) as u32 as u64;
    rcx = (((rcx as u32)).wrapping_shl(10u32)) as u32 as u64;
    fl_l = ((((rcx as u32)) >> (31u32)) & 1) as u64;
    rcx = (((rcx as u32)) | ((1u32) << (31u32))) as u32 as u64;
    rax = ((rcx as u32)) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252cf40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    fl_l = ((((rcx as u32)) >> (10u32)) & 1) as u64;
    rcx = (((rcx as u32)) | ((1u32) << (10u32))) as u32 as u64;
    fl_l = (((rcx as u32)).wrapping_shl(20u32)) as u32 as u64;
    rcx = ((fl_l as u32)) as u32 as u64;
    rax = ((rcx as u32)) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252cf50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e9110; const P1: u64 = 0x14183acda; const P2: u64 = 0x140571d70; const P3: u64 = 0x14183acc8; const P4: u64 = 0x14183acbc; const P5: u64 = 0x141834d30; const P6: u64 = 0x1426467f0; const P7: u64 = 0x14252d30a; const P8: u64 = 0x14252d34b; const P9: u64 = 0x140200110; const P10: u64 = 0x1402032e0; const P11: u64 = 0x14252d37a;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x14252cf50, funcinfo_rva: 0x37fa9d8, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x78, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r15 = (rdx) as u64; rdi = (rcx) as u64; rsi = (crate::simd_ops::ru64(rcx.wrapping_add(0x1f0u64))) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x28u64), (xmm0) as u128); rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rax) as u64); fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = (0xfffffffffffffffu64) as u64; fl_l = (rsi) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 64; continue; } bb = 2; continue; }
            2 => { rbx = (rsi) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252cfa9); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r13 = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); fl_l = ((rax).wrapping_add(rbx)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rax) as u64); r14 = (r13) as u64; bb = 3; continue; }
            3 => { crate::simd_ops::wu64(r14, (0x0u64) as u64); crate::simd_ops::wu64(r14.wrapping_add(0x8u64), (0x0u64) as u64); r14 = ((r14).wrapping_add(0x10u64)) as u64; fl_l = (rsi) as u64 as u64; fl_r = (0x1u64) as u64 as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (r14) as u64); bb = 6; continue; }
            5 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), (r13) as u64); bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), (r14) as u64); rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rbp = (crate::simd_ops::ru64(rdi.wrapping_add(0x1e8u64))) as u64; fl_l = (rbp) as u64 as u64; fl_r = (rbp) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 7; continue; }
            7 => { rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x1e0u64))) as u64; bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rsi)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x1f8u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 10; continue; }
            10 => { r8 = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = (r8) as u64 as u64; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x200u64))) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 11; continue; }
            11 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252d041); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = (0x1u32) as u32 as u64; bb = 15; continue; }
            13 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 15; continue; }
            14 => { fl_l = (rcx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x1f8u64))) as u64 as u64; rax = (rax & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 15; continue; }
            15 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rbx = ((rbx).wrapping_add(1)) as u64; rsi = ((rsi).wrapping_add(0x18u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbp) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 17; continue; }
            17 => { r12 = (rbp) as u64; bb = 18; continue; }
            18 => { fl_l = (rbx) as u64 as u64; fl_r = (r12) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 30; continue; } bb = 19; continue; }
            19 => { rax = ((rbx.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; rsi = ((rax.wrapping_mul(8)) as u64) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x1e0u64))) as u64; xmm0 = (crate::simd_ops::ru128(rsi.wrapping_add(rax))) as u128; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(rax).wrapping_add(0x10u64))) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub(1)) as u32 as u64; fl_l = ((rcx).wrapping_add(rcx)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; crate::simd_ops::wu128(r13.wrapping_add(rcx.wrapping_mul(8)), (xmm0) as u128); rbp = (crate::simd_ops::ru64(rdi.wrapping_add(0x1e8u64))) as u64; bb = 20; continue; }
            20 => { rbx = ((rbx).wrapping_add(1)) as u64; rsi = ((rsi.wrapping_add(0x18u64)) as u64) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbp) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 18; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x1e0u64))) as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(rax))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 22; continue; }
            22 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x1f8u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 23; continue; }
            23 => { r8 = (crate::simd_ops::ru64(rsi.wrapping_add(rax).wrapping_add(0x8u64))) as u64; fl_l = (r8) as u64 as u64; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x200u64))) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 26; continue; } bb = 24; continue; }
            24 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252d0e6); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 26; continue; } bb = 25; continue; }
            25 => { rax = (0x1u32) as u32 as u64; bb = 28; continue; }
            26 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 28; continue; }
            27 => { fl_l = (rcx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x1f8u64))) as u64 as u64; rax = (rax & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 28; continue; }
            28 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 20; continue; } bb = 29; continue; }
            29 => { bb = 18; continue; }
            30 => { rbx = (r14) as u64; rbx = ((rbx).wrapping_sub(r13)) as u64; fl_l = ((((rbx) as i64).wrapping_shr(4u32) as u64)) as u64 as u64; rbx = ((fl_l as u64)) as u64; rdi = (0x80u32) as u32 as u64; bb = 31; continue; }
            31 => { rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = ((rbx as u32)) as u32 as u64; fl_r = (0x7fu32) as u32 as u64; rcx = (if (((fl_l as u32)) > ((fl_r as u32))) { (rdi as u32) } else { (rcx as u32) }) as u32 as u64; rax = (((rbx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7fu8))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) | ((rax as u8)))) as u8 as u64)); crate::simd_ops::wu8(rsp.wrapping_add(0x80u64), ((rcx as u8)) as u8); r8 = (0x1u32) as u32 as u64; rdx = ((rsp.wrapping_add(0x80u64)) as u64) as u64; rcx = (r15) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252d147); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (((rbx as u32)).wrapping_shr(7u32)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 31; continue; } bb = 32; continue; }
            32 => { r14 = (r13) as u64; fl_l = (r13) as u64 as u64; fl_r = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 57; continue; } bb = 33; continue; }
            33 => { r12 = (0x7fffffffffffffffu64) as u64; bb = 34; continue; }
            34 => { rbx = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; bb = 35; continue; }
            35 => { rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = ((rbx as u32)) as u32 as u64; fl_r = (0x7fu32) as u32 as u64; rcx = (if (((fl_l as u32)) > ((fl_r as u32))) { (rdi as u32) } else { (rcx as u32) }) as u32 as u64; rax = (((rbx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7fu8))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) | ((rax as u8)))) as u8 as u64)); crate::simd_ops::wu8(rsp.wrapping_add(0x80u64), ((rcx as u8)) as u8); r8 = (0x1u32) as u32 as u64; rdx = ((rsp.wrapping_add(0x80u64)) as u64) as u64; rcx = (r15) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252d1a7); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (((rbx as u32)).wrapping_shr(7u32)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 35; continue; } bb = 36; continue; }
            36 => { rdi = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), (rcx) as u64); rsi = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; rbp = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; rax = (rbp) as u64; rax = ((rax).wrapping_sub(rsi)) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 40; continue; } bb = 37; continue; }
            37 => { rax = ((rsi.wrapping_add(rdi)) as u64) as u64; crate::simd_ops::wu64(r15.wrapping_add(0x10u64), (rax) as u64); rax = (r15) as u64; fl_l = (rbp) as u64 as u64; fl_r = (0xfu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 39; continue; } bb = 38; continue; }
            38 => { rax = (crate::simd_ops::ru64(r15)) as u64; bb = 39; continue; }
            39 => { rbx = ((rax.wrapping_add(rsi)) as u64) as u64; r8 = (rdi) as u64; rdx = (rcx) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252d1f6); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu8(rbx.wrapping_add(rdi), (0x0u8) as u8); bb = 53; continue; }
            40 => { rax = (r12) as u64; rax = ((rax).wrapping_sub(rsi)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 63; continue; } bb = 41; continue; }
            41 => { r13 = ((rsi.wrapping_add(rdi)) as u64) as u64; rbx = (r13) as u64; rbx = (((rbx) | (0xfu64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (r12) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 43; continue; } bb = 42; continue; }
            42 => { rbx = (r12) as u64; bb = 46; continue; }
            43 => { rcx = (rbp) as u64; rcx = ((rcx).wrapping_shr(1u32)) as u64; rax = (r12) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; fl_l = (rbp) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 45; continue; } bb = 44; continue; }
            44 => { rbx = (r12) as u64; bb = 46; continue; }
            45 => { rax = ((rcx.wrapping_add(rbp)) as u64) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rax) as u64 as u64; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rbx }) as u64; bb = 46; continue; }
            46 => { rcx = ((rbx.wrapping_add(0x1u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252d24d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r12 = (rax) as u64; crate::simd_ops::wu64(r15.wrapping_add(0x10u64), (r13) as u64); crate::simd_ops::wu64(r15.wrapping_add(0x18u64), (rbx) as u64); r13 = ((rax.wrapping_add(rsi)) as u64) as u64; r8 = (rsi) as u64; rcx = (rax) as u64; fl_l = (rbp) as u64 as u64; fl_r = (0xfu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 51; continue; } bb = 47; continue; }
            47 => { rbx = (crate::simd_ops::ru64(r15)) as u64; rdx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252d273); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = (rdi) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rcx = (r13) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252d286); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu8(rdi.wrapping_add(r13), (0x0u8) as u8); rdx = ((rbp.wrapping_add(0x1u64)) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 50; continue; } bb = 48; continue; }
            48 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_sub(0x8u64))) as u64; rbx = ((rbx).wrapping_sub(rcx)) as u64; rax = ((rbx.wrapping_sub(0x8u64)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 55; continue; } bb = 49; continue; }
            49 => { rbx = (rcx) as u64; bb = 50; continue; }
            50 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252d2b8); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 52; continue; }
            51 => { rdx = (r15) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252d2c2); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = (rdi) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rcx = (r13) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252d2d5); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu8(rdi.wrapping_add(r13), (0x0u8) as u8); bb = 52; continue; }
            52 => { crate::simd_ops::wu64(r15, (r12) as u64); r12 = (0x7fffffffffffffffu64) as u64; bb = 53; continue; }
            53 => { r14 = ((r14).wrapping_add(0x10u64)) as u64; fl_l = (r14) as u64 as u64; fl_r = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 56; continue; } bb = 54; continue; }
            54 => { rdi = (0x80u32) as u32 as u64; bb = 34; continue; }
            55 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252d304); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P7, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            56 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; bb = 57; continue; }
            57 => { fl_l = (r13) as u64 as u64; fl_r = (r13) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 62; continue; } bb = 58; continue; }
            58 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = ((rcx).wrapping_sub(r13)) as u64; rcx = (((rcx) & (0xfffffffffffffff0u64))) as u64; rax = (r13) as u64; fl_l = (rcx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 61; continue; } bb = 59; continue; }
            59 => { rcx = ((rcx).wrapping_add(0x27u64)) as u64; r13 = (crate::simd_ops::ru64(r13.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(r13)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 61; continue; } bb = 60; continue; }
            60 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252d345); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P8, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            61 => { rdx = (rcx) as u64; rcx = (r13) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252d352); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 62; continue; }
            62 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            63 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252d36f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 64; continue; }
            64 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252d375); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P11, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252d700(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140571d70; const P1: u64 = 0x14252f4c0; const P2: u64 = 0x14252f430; const P3: u64 = 0x140000000; const P4: u64 = 0x14252d390;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp; rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = ((r8 as u32)) as u32 as u64; rdi = rdx; rbx = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; r14 = rcx; rdx = rbp.wrapping_add(0x50); rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(rcx))) as u64; r8 = 0x1; rcx = rdi; crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), rbx); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x20u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rbp.wrapping_add(0x50u64), ((rax as u8)) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x21u64))) as u32) as u32 as u64; rdx = rbp.wrapping_add(0x50); r8 = 0x1; crate::simd_ops::wu8(rbp.wrapping_add(0x50u64), ((rax as u8)) as u8); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x22u64))) as u32) as u32 as u64; rdx = rbp.wrapping_add(0x50); r8 = 0x1; crate::simd_ops::wu8(rbp.wrapping_add(0x50u64), ((rax as u8)) as u8); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x23u64))) as u32) as u32 as u64; rdx = rbp.wrapping_add(0x50); r8 = 0x1; crate::simd_ops::wu8(rbp.wrapping_add(0x50u64), ((rax as u8)) as u8); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x28u64))) as u64; rcx = rdi; rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r14.wrapping_add(0x20u64)))) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x20u64))) as u64; r15 = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(r14.wrapping_add(0x28u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((((rax) as i64).wrapping_shr(2u32) as u64)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { r9 = ((crate::simd_ops::ru8(rcx.wrapping_add(r15.wrapping_mul(4)))) as u32) as u32 as u64; rcx = ((r9 as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x260u64))) as u64; rbx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; r8 = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rdx = (((r9 as u8)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(r8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (((rax as u8)) as u32) as u32 as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0x20u64))) as u64; rdx = rbp.wrapping_add(0x50); r8 = 0x4; rcx = (crate::simd_ops::ru32(rax.wrapping_add(r15.wrapping_mul(4)))) as u32 as u64; rcx = ((((rcx as u32)) & (0xffffff00u32))) as u32 as u64; rax = (((r9 as u8)) as u32) as u32 as u64; rcx = ((((rcx as u32)) | ((rax as u32)))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x50u64), ((rcx as u32)) as u32); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r13 = rbx; fl_l = rbx; fl_r = (0x1u64) as u64; if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { bb = 6; continue; } bb = 4; continue; }
            4 => { rbx = ((r15.wrapping_mul(4).wrapping_add(0x4u64)) as u64) as u64; rsi = r13.wrapping_sub(0x1); bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0x20u64))) as u64; rdx = rbp.wrapping_add(0x50); r8 = 0x4; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(rax))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x50u64), ((rcx as u32)) as u32); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rbx.wrapping_add(0x4); fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x20u64))) as u64; r15 = ((r15).wrapping_add(r13)) as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(0x28u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((((rax) as i64).wrapping_shr(2u32) as u64)) as u64; fl_l = r15; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 1; continue; } bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(r14.wrapping_add(0x58u64))) as u64; rsi = 0x80; rbx = ((rbx).wrapping_sub(crate::simd_ops::ru64(r14.wrapping_add(0x50u64)))) as u64; fl_l = ((((rbx) as i64).wrapping_shr(4u32) as u64)) as u64; rbx = ((fl_l as u64)) as u64; bb = 8; continue; }
            8 => { rcx = 0; fl_l = 0; fl_r = 0; rdx = rbp.wrapping_add(0x50); rax = (((rbx as u8)) as u32) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x7f; r8 = 0x1; rcx = (if (((fl_l as u32)) > ((fl_r as u32))) { (rsi as u32) } else { (rcx as u32) }) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7fu8))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) | ((rax as u8)))) as u8 as u64)); crate::simd_ops::wu8(rbp.wrapping_add(0x50u64), ((rcx as u8)) as u8); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (((rbx as u32)).wrapping_shr(7u32)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 8; continue; } bb = 9; continue; }
            9 => { r13 = (crate::simd_ops::ru64(r14.wrapping_add(0x58u64))) as u64; r12 = (crate::simd_ops::ru64(r14.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x58u64), r13); fl_l = r12; fl_r = r13; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 32; continue; } bb = 10; continue; }
            10 => { rdx = P3; bb = 11; continue; }
            11 => { rax = (((crate::simd_ops::ru32(r12)) as i32 as i64) as u64) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x6; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 31; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru32(rdx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x252de60u64))) as u32 as u64; fl_l = ((rcx).wrapping_add(rdx)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; j_tgt = rcx; bb = 0xffff_fffe; continue; }
            13 => { crate::simd_ops::wu8(rbp.wrapping_add(0x50u64), (0x0u8) as u8); bb = 14; continue; }
            14 => { r8 = 0x1; rdx = rbp.wrapping_add(0x50); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 30; continue; }
            15 => { r8 = 0x1; crate::simd_ops::wu8(rbp.wrapping_add(0x50u64), (0x1u8) as u8); rdx = rbp.wrapping_add(0x50); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru8(r12.wrapping_add(0x8u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rbp.wrapping_add(0x50u64), ((rax as u8)) as u8); bb = 14; continue; }
            16 => { r8 = 0x1; crate::simd_ops::wu8(rbp.wrapping_add(0x50u64), (0x2u8) as u8); rdx = rbp.wrapping_add(0x50); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = ((crate::simd_ops::ru64(r12.wrapping_add(0x8u64))) as u64) as u128; r8 = 0x8; rdx = rbp.wrapping_add(0x40); rcx = rdi; crate::simd_ops::wu64(rbp.wrapping_add(0x40u64), ((xmm0 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 30; continue; }
            17 => { r8 = 0x1; crate::simd_ops::wu8(rbp.wrapping_add(0x50u64), (0x3u8) as u8); rdx = rbp.wrapping_add(0x50); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru32(r12.wrapping_add(0x8u64))) as u32 as u64; bb = 18; continue; }
            18 => { rcx = 0; fl_l = 0; fl_r = 0; rdx = rbp.wrapping_add(0x50); rax = (((rbx as u8)) as u32) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x7f; r8 = 0x1; rcx = (if (((fl_l as u32)) > ((fl_r as u32))) { (rsi as u32) } else { (rcx as u32) }) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7fu8))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) | ((rax as u8)))) as u8 as u64)); crate::simd_ops::wu8(rbp.wrapping_add(0x50u64), ((rcx as u8)) as u8); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (((rbx as u32)).wrapping_shr(7u32)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 18; continue; } bb = 19; continue; }
            19 => { bb = 30; continue; }
            20 => { r8 = 0x1; crate::simd_ops::wu8(rbp.wrapping_add(0x50u64), (0x4u8) as u8); rdx = rbp.wrapping_add(0x50); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(r12.wrapping_add(0x8u64))) as u32 as u64; rdx = rbp.wrapping_add(0x50); r8 = 0x4; crate::simd_ops::wu32(rbp.wrapping_add(0x50u64), ((rax as u32)) as u32); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 30; continue; }
            21 => { rax = (crate::simd_ops::ru32(r12.wrapping_add(0x8u64))) as u32 as u64; rdx = rbp.wrapping_add(0x50); r15 = (((rax) as i64).wrapping_mul((0x84u64) as i64) as u64) as u64; r8 = 0x1; crate::simd_ops::wu8(rbp.wrapping_add(0x50u64), (0x5u8) as u8); r15 = ((r15).wrapping_add(crate::simd_ops::ru64(r14.wrapping_add(0x98u64)))) as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru32(r15.wrapping_add(0x80u64))) as u32 as u64; bb = 22; continue; }
            22 => { rcx = 0; fl_l = 0; fl_r = 0; rdx = rbp.wrapping_add(0x50); rax = (((rbx as u8)) as u32) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x7f; r8 = 0x1; rcx = (if (((fl_l as u32)) > ((fl_r as u32))) { (rsi as u32) } else { (rcx as u32) }) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7fu8))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) | ((rax as u8)))) as u8 as u64)); crate::simd_ops::wu8(rbp.wrapping_add(0x50u64), ((rcx as u8)) as u8); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (((rbx as u32)).wrapping_shr(7u32)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 22; continue; } bb = 23; continue; }
            23 => { rsi = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(r15.wrapping_add(0x80u64))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 29; continue; } bb = 24; continue; }
            24 => { r13 = 0x80; bb = 25; continue; }
            25 => { rbx = (crate::simd_ops::ru32(r15.wrapping_add(rsi.wrapping_mul(4)))) as u32 as u64; bb = 26; continue; }
            26 => { rcx = 0; fl_l = 0; fl_r = 0; rdx = rbp.wrapping_add(0x50); rax = (((rbx as u8)) as u32) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x7f; r8 = 0x1; rcx = (if (((fl_l as u32)) > ((fl_r as u32))) { (r13 as u32) } else { (rcx as u32) }) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7fu8))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) | ((rax as u8)))) as u8 as u64)); crate::simd_ops::wu8(rbp.wrapping_add(0x50u64), ((rcx as u8)) as u8); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (((rbx as u32)).wrapping_shr(7u32)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 26; continue; } bb = 27; continue; }
            27 => { rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r15.wrapping_add(0x80u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 25; continue; } bb = 28; continue; }
            28 => { r13 = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; bb = 29; continue; }
            29 => { rsi = 0x80; bb = 30; continue; }
            30 => { rdx = P3; bb = 31; continue; }
            31 => { r12 = ((r12).wrapping_add(0x10u64)) as u64; fl_l = r12; fl_r = r13; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 32; continue; }
            32 => { rbx = (crate::simd_ops::ru64(r14.wrapping_add(0x70u64))) as u64; r12 = 0x80; rbx = ((rbx).wrapping_sub(crate::simd_ops::ru64(r14.wrapping_add(0x68u64)))) as u64; fl_l = ((((rbx) as i64).wrapping_shr(2u32) as u64)) as u64; rbx = ((fl_l as u64)) as u64; bb = 33; continue; }
            33 => { rcx = 0; fl_l = 0; fl_r = 0; rdx = rbp.wrapping_add(0x50); rax = (((rbx as u8)) as u32) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x7f; r8 = 0x1; rcx = (if (((fl_l as u32)) > ((fl_r as u32))) { (r12 as u32) } else { (rcx as u32) }) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7fu8))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) | ((rax as u8)))) as u8 as u64)); crate::simd_ops::wu8(rbp.wrapping_add(0x50u64), ((rcx as u8)) as u8); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (((rbx as u32)).wrapping_shr(7u32)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 33; continue; } bb = 34; continue; }
            34 => { r15 = (crate::simd_ops::ru64(r14.wrapping_add(0x70u64))) as u64; rsi = (crate::simd_ops::ru64(r14.wrapping_add(0x68u64))) as u64; fl_l = rsi; fl_r = r15; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 39; continue; } bb = 35; continue; }
            35 => { bb = 36; continue; }
            36 => { rbx = (crate::simd_ops::ru32(rsi)) as u32 as u64; bb = 37; continue; }
            37 => { rcx = 0; fl_l = 0; fl_r = 0; rdx = rbp.wrapping_add(0x50); rax = (((rbx as u8)) as u32) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x7f; r8 = 0x1; rcx = (if (((fl_l as u32)) > ((fl_r as u32))) { (r12 as u32) } else { (rcx as u32) }) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7fu8))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) | ((rax as u8)))) as u8 as u64)); crate::simd_ops::wu8(rbp.wrapping_add(0x50u64), ((rcx as u8)) as u8); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (((rbx as u32)).wrapping_shr(7u32)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 37; continue; } bb = 38; continue; }
            38 => { rsi = ((rsi).wrapping_add(0x4u64)) as u64; fl_l = rsi; fl_r = r15; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 36; continue; } bb = 39; continue; }
            39 => { rsi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x10u64))) as u64; rbx = (crate::simd_ops::ru32(rsi.wrapping_add(0x28u64))) as u32 as u64; bb = 40; continue; }
            40 => { rcx = 0; fl_l = 0; fl_r = 0; rdx = rbp.wrapping_add(0x50); rax = (((rbx as u8)) as u32) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x7f; r8 = 0x1; rcx = (if (((fl_l as u32)) > ((fl_r as u32))) { (r12 as u32) } else { (rcx as u32) }) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7fu8))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) | ((rax as u8)))) as u8 as u64)); crate::simd_ops::wu8(rbp.wrapping_add(0x50u64), ((rcx as u8)) as u8); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (((rbx as u32)).wrapping_shr(7u32)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 40; continue; } bb = 41; continue; }
            41 => { rbx = (crate::simd_ops::ru32(rsi.wrapping_add(0x24u64))) as u32 as u64; bb = 42; continue; }
            42 => { rcx = 0; fl_l = 0; fl_r = 0; rdx = rbp.wrapping_add(0x50); rax = (((rbx as u8)) as u32) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x7f; r8 = 0x1; rcx = (if (((fl_l as u32)) > ((fl_r as u32))) { (r12 as u32) } else { (rcx as u32) }) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7fu8))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) | ((rax as u8)))) as u8 as u64)); crate::simd_ops::wu8(rbp.wrapping_add(0x50u64), ((rcx as u8)) as u8); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (((rbx as u32)).wrapping_shr(7u32)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 42; continue; } bb = 43; continue; }
            43 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0x38u64))) as u64; rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x40u64))) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 47; continue; } bb = 44; continue; }
            44 => { bb = 45; continue; }
            45 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 54; continue; } bb = 46; continue; }
            46 => { rax = ((rax).wrapping_add(0x4u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 45; continue; } bb = 47; continue; }
            47 => { r8 = 0x1; crate::simd_ops::wu8(rbp.wrapping_add(0x50u64), (0x1u8) as u8); rdx = rbp.wrapping_add(0x50); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rdi; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 48; continue; }
            48 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0x1b8u64))) as u64; fl_l = (crate::simd_ops::ru64(r14.wrapping_add(0x1b0u64))) as u64; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 55; continue; } bb = 49; continue; }
            49 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0x1d0u64))) as u64; fl_l = (crate::simd_ops::ru64(r14.wrapping_add(0x1c8u64))) as u64; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 55; continue; } bb = 50; continue; }
            50 => { r8 = 0x1; crate::simd_ops::wu8(rbp.wrapping_add(0x50u64), (0x0u8) as u8); rdx = rbp.wrapping_add(0x50); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 72; continue; }
            51 => { r8 = 0x1; crate::simd_ops::wu8(rbp.wrapping_add(0x50u64), (0x6u8) as u8); rdx = rbp.wrapping_add(0x50); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru32(r12.wrapping_add(0x8u64))) as u32 as u64; bb = 52; continue; }
            52 => { rcx = 0; fl_l = 0; fl_r = 0; rdx = rbp.wrapping_add(0x50); rax = (((rbx as u8)) as u32) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x7f; r8 = 0x1; rcx = (if (((fl_l as u32)) > ((fl_r as u32))) { (rsi as u32) } else { (rcx as u32) }) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7fu8))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) | ((rax as u8)))) as u8 as u64)); crate::simd_ops::wu8(rbp.wrapping_add(0x50u64), ((rcx as u8)) as u8); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (((rbx as u32)).wrapping_shr(7u32)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 52; continue; } bb = 53; continue; }
            53 => { bb = 30; continue; }
            54 => { r8 = 0x1; crate::simd_ops::wu8(rbp.wrapping_add(0x50u64), (0x0u8) as u8); rdx = rbp.wrapping_add(0x50); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 48; continue; }
            55 => { r8 = 0x1; crate::simd_ops::wu8(rbp.wrapping_add(0x50u64), (0x1u8) as u8); rdx = rbp.wrapping_add(0x50); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(r14.wrapping_add(0x1b8u64))) as u64; rbx = ((rbx).wrapping_sub(crate::simd_ops::ru64(r14.wrapping_add(0x1b0u64)))) as u64; fl_l = ((((rbx) as i64).wrapping_shr(4u32) as u64)) as u64; rbx = ((fl_l as u64)) as u64; bb = 56; continue; }
            56 => { rcx = 0; fl_l = 0; fl_r = 0; rdx = rbp.wrapping_add(0x50); rax = (((rbx as u8)) as u32) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x7f; r8 = 0x1; rcx = (if (((fl_l as u32)) > ((fl_r as u32))) { (r12 as u32) } else { (rcx as u32) }) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7fu8))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) | ((rax as u8)))) as u8 as u64)); crate::simd_ops::wu8(rbp.wrapping_add(0x50u64), ((rcx as u8)) as u8); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (((rbx as u32)).wrapping_shr(7u32)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 56; continue; } bb = 57; continue; }
            57 => { r15 = (crate::simd_ops::ru64(r14.wrapping_add(0x1b8u64))) as u64; rsi = (crate::simd_ops::ru64(r14.wrapping_add(0x1b0u64))) as u64; fl_l = rsi; fl_r = r15; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 66; continue; } bb = 58; continue; }
            58 => { bb = 59; continue; }
            59 => { rbx = (crate::simd_ops::ru32(rsi)) as u32 as u64; bb = 60; continue; }
            60 => { rcx = 0; fl_l = 0; fl_r = 0; rdx = rbp.wrapping_add(0x50); rax = (((rbx as u8)) as u32) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x7f; r8 = 0x1; rcx = (if (((fl_l as u32)) > ((fl_r as u32))) { (r12 as u32) } else { (rcx as u32) }) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7fu8))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) | ((rax as u8)))) as u8 as u64)); crate::simd_ops::wu8(rbp.wrapping_add(0x50u64), ((rcx as u8)) as u8); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (((rbx as u32)).wrapping_shr(7u32)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 60; continue; } bb = 61; continue; }
            61 => { rbx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; bb = 62; continue; }
            62 => { rcx = 0; fl_l = 0; fl_r = 0; rdx = rbp.wrapping_add(0x50); rax = (((rbx as u8)) as u32) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x7f; r8 = 0x1; rcx = (if (((fl_l as u32)) > ((fl_r as u32))) { (r12 as u32) } else { (rcx as u32) }) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7fu8))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) | ((rax as u8)))) as u8 as u64)); crate::simd_ops::wu8(rbp.wrapping_add(0x50u64), ((rcx as u8)) as u8); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (((rbx as u32)).wrapping_shr(7u32)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 62; continue; } bb = 63; continue; }
            63 => { rbx = (crate::simd_ops::ru32(rsi.wrapping_add(0xcu64))) as u32 as u64; bb = 64; continue; }
            64 => { rcx = 0; fl_l = 0; fl_r = 0; rdx = rbp.wrapping_add(0x50); rax = (((rbx as u8)) as u32) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x7f; r8 = 0x1; rcx = (if (((fl_l as u32)) > ((fl_r as u32))) { (r12 as u32) } else { (rcx as u32) }) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7fu8))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) | ((rax as u8)))) as u8 as u64)); crate::simd_ops::wu8(rbp.wrapping_add(0x50u64), ((rcx as u8)) as u8); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (((rbx as u32)).wrapping_shr(7u32)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 64; continue; } bb = 65; continue; }
            65 => { rax = ((crate::simd_ops::ru8(rsi.wrapping_add(0x4u64))) as u32) as u32 as u64; r8 = ((rbx.wrapping_add(0x1u64)) as u32) as u32 as u64; rdx = rbp.wrapping_add(0x50); crate::simd_ops::wu8(rbp.wrapping_add(0x50u64), ((rax as u8)) as u8); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = ((rsi).wrapping_add(0x10u64)) as u64; fl_l = rsi; fl_r = r15; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 59; continue; } bb = 66; continue; }
            66 => { rbx = (crate::simd_ops::ru64(r14.wrapping_add(0x1d0u64))) as u64; rbx = ((rbx).wrapping_sub(crate::simd_ops::ru64(r14.wrapping_add(0x1c8u64)))) as u64; fl_l = ((((rbx) as i64).wrapping_shr(2u32) as u64)) as u64; rbx = ((fl_l as u64)) as u64; bb = 67; continue; }
            67 => { rcx = 0; fl_l = 0; fl_r = 0; rdx = rbp.wrapping_add(0x50); rax = (((rbx as u8)) as u32) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x7f; r8 = 0x1; rcx = (if (((fl_l as u32)) > ((fl_r as u32))) { (r12 as u32) } else { (rcx as u32) }) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7fu8))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) | ((rax as u8)))) as u8 as u64)); crate::simd_ops::wu8(rbp.wrapping_add(0x50u64), ((rcx as u8)) as u8); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (((rbx as u32)).wrapping_shr(7u32)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 67; continue; } bb = 68; continue; }
            68 => { r15 = (crate::simd_ops::ru64(r14.wrapping_add(0x1d0u64))) as u64; rsi = (crate::simd_ops::ru64(r14.wrapping_add(0x1c8u64))) as u64; fl_l = rsi; fl_r = r15; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 72; continue; } bb = 69; continue; }
            69 => { rbx = (crate::simd_ops::ru32(rsi)) as u32 as u64; bb = 70; continue; }
            70 => { rcx = 0; fl_l = 0; fl_r = 0; rdx = rbp.wrapping_add(0x50); rax = (((rbx as u8)) as u32) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x7f; r8 = 0x1; rcx = (if (((fl_l as u32)) > ((fl_r as u32))) { (r12 as u32) } else { (rcx as u32) }) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7fu8))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) | ((rax as u8)))) as u8 as u64)); crate::simd_ops::wu8(rbp.wrapping_add(0x50u64), ((rcx as u8)) as u8); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (((rbx as u32)).wrapping_shr(7u32)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 70; continue; } bb = 71; continue; }
            71 => { rsi = ((rsi).wrapping_add(0x4u64)) as u64; fl_l = rsi; fl_r = r15; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 69; continue; } bb = 72; continue; }
            72 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x14252d700u64) { 0xc1 => { bb = 1; continue; } 0xdd => { bb = 2; continue; } 0xef => { bb = 3; continue; } 0x121 => { bb = 4; continue; } 0x130 => { bb = 5; continue; } 0x156 => { bb = 6; continue; } 0x171 => { bb = 7; continue; } 0x182 => { bb = 8; continue; } 0x1ad => { bb = 9; continue; } 0x1c2 => { bb = 10; continue; } 0x1d0 => { bb = 11; continue; } 0x1dd => { bb = 12; continue; } 0x1e9 => { bb = 13; continue; } 0x1ed => { bb = 14; continue; } 0x204 => { bb = 15; continue; } 0x225 => { bb = 16; continue; } 0x25e => { bb = 17; continue; } 0x280 => { bb = 18; continue; } 0x2ab => { bb = 19; continue; } 0x2b0 => { bb = 20; continue; } 0x2e5 => { bb = 21; continue; } 0x320 => { bb = 22; continue; } 0x34b => { bb = 23; continue; } 0x356 => { bb = 24; continue; } 0x360 => { bb = 25; continue; } 0x364 => { bb = 26; continue; } 0x390 => { bb = 27; continue; } 0x39b => { bb = 28; continue; } 0x39f => { bb = 29; continue; } 0x3a4 => { bb = 30; continue; } 0x3ab => { bb = 31; continue; } 0x3b8 => { bb = 32; continue; } 0x3d0 => { bb = 33; continue; } 0x3fc => { bb = 34; continue; } 0x409 => { bb = 35; continue; } 0x410 => { bb = 36; continue; } 0x412 => { bb = 37; continue; } 0x43e => { bb = 38; continue; } 0x447 => { bb = 39; continue; } 0x450 => { bb = 40; continue; } 0x47c => { bb = 41; continue; } 0x480 => { bb = 42; continue; } 0x4ac => { bb = 43; continue; } 0x4b9 => { bb = 44; continue; } 0x4c0 => { bb = 45; continue; } 0x4c9 => { bb = 46; continue; } 0x4d2 => { bb = 47; continue; } 0x4f3 => { bb = 48; continue; } 0x507 => { bb = 49; continue; } 0x51b => { bb = 50; continue; } 0x536 => { bb = 51; continue; } 0x551 => { bb = 52; continue; } 0x57c => { bb = 53; continue; } 0x581 => { bb = 54; continue; } 0x59c => { bb = 55; continue; } 0x5c4 => { bb = 56; continue; } 0x5f0 => { bb = 57; continue; } 0x607 => { bb = 58; continue; } 0x610 => { bb = 59; continue; } 0x612 => { bb = 60; continue; } 0x63e => { bb = 61; continue; } 0x641 => { bb = 62; continue; } 0x66d => { bb = 63; continue; } 0x670 => { bb = 64; continue; } 0x69c => { bb = 65; continue; } 0x6c0 => { bb = 66; continue; } 0x6d2 => { bb = 67; continue; } 0x6fe => { bb = 68; continue; } 0x711 => { bb = 69; continue; } 0x713 => { bb = 70; continue; } 0x73f => { bb = 71; continue; } 0x748 => { bb = 72; continue; } _ => { return crate::simd_ops::tail_stk_xmm(j_tgt, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); } } }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252de90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14183acda; const P1: u64 = 0x141470d30; const P2: u64 = 0x14252cf50; const P3: u64 = 0x140571d70;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = (crate::simd_ops::ru64(rcx.wrapping_add(0x1e8u64))) as u64; rdi = 0; fl_l = 0; fl_r = 0; rsi = rcx; r14 = ((rdi.wrapping_add(0x10u64)) as u32) as u32 as u64; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x1e0u64))) as u64; bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x1f8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = r8; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(0x200u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = 0x1; bb = 9; continue; }
            7 => { rax = 0; fl_l = 0; fl_r = 0; bb = 9; continue; }
            8 => { fl_l = rcx; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(0x1f8u64))) as u64; rax = (rax & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 9; continue; }
            9 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { rdi = ((rdi).wrapping_add(1)) as u64; rbx = ((rbx).wrapping_add(0x18u64)) as u64; fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 11; continue; }
            11 => { bb = 12; continue; }
            12 => { fl_l = rdi; fl_r = rbp; bb = 13; continue; }
            13 => { if (((fl_l as u64)) == ((fl_r as u64))) { bb = 25; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x1e0u64))) as u64; rcx = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rbx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; r14 = ((r14).wrapping_add(0x2u64)) as u64; fl_l = ((r14).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x8u64)))) as u64; fl_r = fl_l; r14 = ((fl_l as u64)) as u64; bb = 15; continue; }
            15 => { rdi = ((rdi).wrapping_add(1)) as u64; rbx = rbx.wrapping_add(0x18); fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 13; continue; } bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 17; continue; }
            17 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x1f8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 18; continue; }
            18 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = r8; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(0x200u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 19; continue; }
            19 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { rax = 0x1; bb = 23; continue; }
            21 => { rax = 0; fl_l = 0; fl_r = 0; bb = 23; continue; }
            22 => { fl_l = rcx; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(0x1f8u64))) as u64; rax = (rax & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 23; continue; }
            23 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 15; continue; } bb = 24; continue; }
            24 => { bb = 12; continue; }
            25 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 28; continue; } bb = 26; continue; }
            26 => { bb = 27; continue; }
            27 => { r14 = ((r14).wrapping_add(crate::simd_ops::ru64(rax.wrapping_add(0x10u64)))) as u64; rax = ((rax).wrapping_add(0x88u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 27; continue; } bb = 28; continue; }
            28 => { rdi = rsi.wrapping_add(0x268); rdx = r14; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), (0x1u64) as u64); rax = rdi; fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; fl_r = (0xfu64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 30; continue; } bb = 29; continue; }
            29 => { rax = (crate::simd_ops::ru64(rdi)) as u64; bb = 30; continue; }
            30 => { rdx = rdi; crate::simd_ops::wu16(rax, (0x3u16) as u16); rcx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rax = (0x7878787878787879u64) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rsi))) as u64; r14 = 0x80; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rbx = rdx; rbx = ((((rbx) as i64).wrapping_shr(6u32) as u64)) as u64; rax = rbx; rax = ((rax).wrapping_shr(63u32)) as u64; fl_l = ((rbx).wrapping_add(rax)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 31; continue; }
            31 => { rcx = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x40); rax = (((rbx as u8)) as u32) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x7f; r8 = 0x1; rcx = (if (((fl_l as u32)) > ((fl_r as u32))) { (r14 as u32) } else { (rcx as u32) }) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7fu8))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) | ((rax as u8)))) as u8 as u64)); crate::simd_ops::wu8(rsp.wrapping_add(0x40u64), ((rcx as u8)) as u8); rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = (((rbx as u32)).wrapping_shr(7u32)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 31; continue; } bb = 32; continue; }
            32 => { rdi = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rbx = (crate::simd_ops::ru64(rsi)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 37; continue; } bb = 33; continue; }
            33 => { bb = 34; continue; }
            34 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_r = (0xfu64) as u64; rdx = rbx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 36; continue; } bb = 35; continue; }
            35 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; bb = 36; continue; }
            36 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = rsi.wrapping_add(0x268); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x88u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 34; continue; } bb = 37; continue; }
            37 => { rbx = (crate::simd_ops::ru32(rsi.wrapping_add(0x1cu64))) as u32 as u64; bb = 38; continue; }
            38 => { rcx = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x40); rax = (((rbx as u8)) as u32) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x7f; r8 = 0x1; rcx = (if (((fl_l as u32)) > ((fl_r as u32))) { (r14 as u32) } else { (rcx as u32) }) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7fu8))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) | ((rax as u8)))) as u8 as u64)); crate::simd_ops::wu8(rsp.wrapping_add(0x40u64), ((rcx as u8)) as u8); rcx = rsi.wrapping_add(0x268); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = (((rbx as u32)).wrapping_shr(7u32)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 38; continue; } bb = 39; continue; }
            39 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252e0c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142527500; const P1: u64 = 0x14129dea0; const P2: u64 = 0x141472a90; const P3: u64 = 0x1402050d0; const P4: u64 = 0x141472c00;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x288u64))) as u8 as u64; fl_r = (0x10u8) as u8 as u64; rbx = rcx; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); r8 = rsp.wrapping_add(0x60); rsi = (crate::simd_ops::ru64(rcx.wrapping_add(0x250u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rdi = rcx.wrapping_add(0x240); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; rcx = rbx.wrapping_add(0x228); rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rbx.wrapping_add(0x20u64)))) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x230u64))) as u64; rax = ((((rax) as i64).wrapping_shr(2u32) as u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rsi as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); fl_l = rdx; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x238u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rdx, ((rax as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x4u64), ((rsi as u32)) as u32); fl_l = (crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x230u64), (0x8u64) as u64)) as u64; fl_r = fl_l; bb = 4; continue; }
            3 => { r9 = rsp.wrapping_add(0x20); r8 = rsp.wrapping_add(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; fl_r = (0xfu64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdi = (crate::simd_ops::ru64(rdi)) as u64; bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x2b0u64))) as u64; rdx = ((rdi.wrapping_add(rsi)) as u64) as u64; r8 = rbx.wrapping_add(0x1a8); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdx); fl_l = rcx; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x2b8u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru32(r8)) as u32 as u64; r8 = (0xffffffffffffffffu64) as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rcx = ((rcx).wrapping_add(0x8u64)) as u64; rax = 0; fl_l = 0; fl_r = 0; xmm0 = 0u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), rax); bb = 8; continue; }
            8 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 9; continue; }
            9 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x2b0u64), (0x28u64) as u64); rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            10 => { rdx = rcx; r9 = rsp.wrapping_add(0x28); rcx = rbx.wrapping_add(0x2a8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            11 => { rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252e210(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14252ead0; const P1: u64 = 0x1d0; const P2: u64 = 0x1c8; const P3: u64 = 0x1d8; const P4: u64 = 0x1403952e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    rdx = rsp.wrapping_add(0x20);
    rbx = rcx;
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    rcx = ((rbx.wrapping_add(P2)) as u64) as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rax as u32)) as u32);
    fl_l = rdx; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { crate::simd_ops::wu32(rdx, ((rax as u32)) as u32); crate::simd_ops::lk_add_u64(rbx.wrapping_add(P1), (0x4u64) as u64); rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    r8 = rsp.wrapping_add(0x40);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252e270(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14252ead0; const P1: u64 = 0x1b8; const P2: u64 = 0x1b0; const P3: u64 = 0x1c0; const P4: u64 = 0x141470ab0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; xmm0 = (crate::simd_ops::ru128(rdx)) as u128; rdx = rsp.wrapping_add(0x20); rdi = ((r9 as u32)) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rbx = (((r8 as u8)) as u32) as u32 as u64; rsi = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rsi.wrapping_add(P1))) as u64; rcx = ((rsi.wrapping_add(P2)) as u64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x24u64), ((rbx as u8)) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rdi as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), ((rax as u32)) as u32); fl_l = rdx; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(P3))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); crate::simd_ops::lk_add_u64(rsi.wrapping_add(P1), (0x10u64) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { r8 = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252e320(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x28;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    r8 = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(rax).wrapping_add(P1), ((rdx as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252e340(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14252ead0; const P1: u64 = 0x18; const P2: u64 = 0x24; const P3: u64 = 0x2c0; const P4: u64 = 0x8; const P5: u64 = 0x1402050d0; const P6: u64 = 0x140203140; const P7: u64 = 0xfffffffffffffff8; const P8: u64 = 0x1426467f0; const P9: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; xmm0 = (crate::simd_ops::ru128(rdx)) as u128; rdi = rdx; rbx = rcx; rdx = rsp.wrapping_add(0x20); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; r9 = (((r8) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; r8 = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu32(r9.wrapping_add(r8).wrapping_add(P2), ((rax as u32)) as u32); fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; rcx = rsp.wrapping_add(0x20); rdx = (crate::simd_ops::ru64(rdi)) as u64; xmm0 = 0u128; xmm1 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm1) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; rdx = rsp.wrapping_add(0x20); rcx = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rax = ((rax).wrapping_add(0x50u64)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = rdx; fl_r = (0xfu64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rdx = ((rdx).wrapping_add(1)) as u64; rax = rcx; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P7))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x14252e402, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252e420(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (((r8 as u32)).wrapping_sub((rdx as u32))) as u32 as u64;
    r8 = (((r8 as u32)).wrapping_sub(1)) as u32 as u64;
    rax = (((r8 as u8)) as u32) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64;
    if !((((fl_l as u32)) == ((fl_r as u32)))) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    r8 = (((r8 as u32)).wrapping_shl(24u32)) as u32 as u64;
    fl_l = (crate::simd_ops::lk_or_u32(rax.wrapping_add(rdx.wrapping_mul(4)), ((r8 as u32)) as u32)) as u32 as u64; fl_r = fl_l;
    rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252e450(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0xb0; const P2: u64 = 0x80; const P3: u64 = 0x88; const P4: u64 = 0x90; const P5: u64 = 0x1411d6220;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; r10 = ((r8 as u32)) as u32 as u64; r9 = rcx; r10 = (((r10 as u32)).wrapping_sub((rdx as u32))) as u32 as u64; r10 = (((r10 as u32)).wrapping_sub(1)) as u32 as u64; rax = ((((r10 as u16)) as i16 as i32) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rcx = ((rax.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; rax = (((r10 as u16)) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_shl(16u32)) as u32 as u64; fl_l = (crate::simd_ops::lk_or_u32(rcx, ((rax as u32)) as u32)) as u32 as u64; fl_r = fl_l; bb = 4; continue; }
            2 => { rax = ((r10 as u32)) as u32 as u64; fl_l = 0; fl_r = ((rax as u32)) as u32 as u64; rax = (0u32.wrapping_sub(fl_r as u32)) as u32 as u64; rax = (if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) < 0) { (r10 as u32) } else { (rax as u32) }) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x800000; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 7; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu8(rcx.wrapping_add(P1), (0x1u8) as u8); bb = 4; continue; }
            4 => { crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rdx as u32)) as u32); rcx = ((r9.wrapping_add(P2)) as u64) as u64; rdx = (crate::simd_ops::ru64(r9.wrapping_add(P3))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((r8 as u32)) as u32); fl_l = rdx; fl_r = (crate::simd_ops::ru64(r9.wrapping_add(P4))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rdx, rax); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::lk_add_u64(r9.wrapping_add(P3), (0x8u64) as u64); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            6 => { r8 = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            7 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252e510(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x30; const P2: u64 = 0x141473510; const P3: u64 = 0x40; const P4: u64 = 0x38; const P5: u64 = 0x1a8; const P6: u64 = 0x48;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu32(rsp.wrapping_add(0x10u64), ((rdx as u32)) as u32);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = ((rdx as u32)) as u32 as u64;
    rcx = ((rcx).wrapping_add(0x20u64)) as u64;
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    fl_l = rdx; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { crate::simd_ops::wu32(rdx, ((rax as u32)) as u32); fl_l = (crate::simd_ops::lk_add_u64(rbx.wrapping_add(P0), (0x4u64) as u64)) as u64; fl_r = fl_l; } else { r8 = rsp.wrapping_add(0x38); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64;
    rcx = ((rbx.wrapping_add(P4)) as u64) as u64;
    r8 = ((rbx.wrapping_add(P5)) as u64) as u64;
    fl_l = rdx; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rax = (crate::simd_ops::ru32(r8)) as u32 as u64; crate::simd_ops::wu32(rdx, ((rax as u32)) as u32); crate::simd_ops::lk_add_u64(rbx.wrapping_add(P3), (0x4u64) as u64); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252e580(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x30; const P2: u64 = 0x20; const P3: u64 = 0x141473510; const P4: u64 = 0x40; const P5: u64 = 0x38; const P6: u64 = 0x1a8; const P7: u64 = 0x48;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rax = (((r8 as u8)) as u32) as u32 as u64; rcx = (((r9 as u16)) as u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; rcx = ((((rcx as u32)) | ((rax as u32)))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; rcx = ((((rcx as u32)) | ((rdx as u32)))) as u32 as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rcx as u32)) as u32); fl_l = rdx; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rdx, ((rcx as u32)) as u32); fl_l = (crate::simd_ops::lk_add_u64(rbx.wrapping_add(P0), (0x4u64) as u64)) as u64; fl_r = fl_l; bb = 3; continue; }
            2 => { r8 = rsp.wrapping_add(0x40); rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; rcx = ((rbx.wrapping_add(P5)) as u64) as u64; r8 = ((rbx.wrapping_add(P6)) as u64) as u64; fl_l = rdx; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(P7))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(r8)) as u32 as u64; crate::simd_ops::wu32(rdx, ((rax as u32)) as u32); crate::simd_ops::lk_add_u64(rbx.wrapping_add(P4), (0x4u64) as u64); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252e600(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x30; const P2: u64 = 0x20; const P3: u64 = 0x141473510; const P4: u64 = 0x40; const P5: u64 = 0x38; const P6: u64 = 0x1a8; const P7: u64 = 0x48;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rax = (((r9 as u8)) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x50u64))) as u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; rcx = ((((rcx as u32)) | ((rax as u32)))) as u32 as u64; rax = (((r8 as u8)) as u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; rcx = ((((rcx as u32)) | ((rax as u32)))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; rcx = ((((rcx as u32)) | ((rdx as u32)))) as u32 as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((rcx as u32)) as u32); fl_l = rdx; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rdx, ((rcx as u32)) as u32); fl_l = (crate::simd_ops::lk_add_u64(rbx.wrapping_add(P0), (0x4u64) as u64)) as u64; fl_r = fl_l; bb = 3; continue; }
            2 => { r8 = rsp.wrapping_add(0x50); rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; rcx = ((rbx.wrapping_add(P5)) as u64) as u64; r8 = ((rbx.wrapping_add(P6)) as u64) as u64; fl_l = rdx; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(P7))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(r8)) as u32 as u64; crate::simd_ops::wu32(rdx, ((rax as u32)) as u32); crate::simd_ops::lk_add_u64(rbx.wrapping_add(P4), (0x4u64) as u64); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252e680(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141471560; const P1: u64 = 0x70; const P2: u64 = 0x68; const P3: u64 = 0x190; const P4: u64 = 0x198; const P5: u64 = 0x188; const P6: u64 = 0x141472040; const P7: u64 = 0x141471620; const P8: u64 = 0x4; const P9: u64 = 0x78; const P10: u64 = 0x141473510;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); crate::simd_ops::wu32(rsp.wrapping_add(0x10u64), ((rdx as u32)) as u32); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = ((rdx as u32)) as u32 as u64; rbx = rcx; rdx = rsp.wrapping_add(0x38); rcx = ((rcx).wrapping_add(0x188u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r8 = rax; rcx = 0; fl_l = 0; fl_r = 0; rax = ((rax).wrapping_add(0x4u64)) as u64; fl_l = r8; fl_r = r8; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rcx } else { rax }) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rdi = ((rdi).wrapping_sub(crate::simd_ops::ru64(rbx.wrapping_add(P2)))) as u64; rdi = ((((rdi) as i64).wrapping_shr(2u32) as u64)) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x8000; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = 0xffffffff; bb = 11; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shr(2u32)) as u64; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = ((rbx.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rdx = rsp.wrapping_add(0x38); rcx = ((rbx.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu16(rax.wrapping_add(P8), ((rdi as u16)) as u16); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(P9))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu32(rdx, ((rbp as u32)) as u32); fl_l = (crate::simd_ops::lk_add_u64(rbx.wrapping_add(P1), (0x4u64) as u64)) as u64; fl_r = fl_l; bb = 10; continue; }
            9 => { r8 = rsp.wrapping_add(0x38); rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rax = (((rdi as u16)) as u32) as u32 as u64; bb = 11; continue; }
            11 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252e770(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14252ebc0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x6u32) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    r8 = rsp.wrapping_add(0x20);
    rax = ((rdx as u32)) as u32 as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rdx as u32)) as u32);
    rdx = rsp.wrapping_add(0x30);
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax);
    crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x6u32) as u32);
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252e7b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414716e0; const P1: u64 = 0x141472200; const P2: u64 = 0x141471800; const P3: u64 = 0x1414731f0; const P4: u64 = 0x1414733f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r14 = rcx.wrapping_add(0xe8); rbp = rcx; rcx = r14; rdi = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = 0; fl_l = 0; fl_r = 0; fl_l = rax; fl_r = rax; rdx = rax.wrapping_add(0x84); rdx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rcx } else { rdx }) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rbx = rbp.wrapping_add(0x50); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); rsi = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rsi = ((rsi).wrapping_sub(crate::simd_ops::ru64(rbx))) as u64; rsi = ((((rsi) as i64).wrapping_shr(4u32) as u64)) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x800000; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = 0xffffffff; bb = 14; continue; }
            4 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x5u32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rbp = ((rbp).wrapping_add(0x98u64)) as u64; rax = (0xf83e0f83e0f83e1u64) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rbp))) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rdx as u32)) as u32); rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shr(2u32)) as u64; fl_l = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; fl_r = rcx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rdx = rdi; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            7 => { rdx = rdi; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rax.wrapping_add(0x84u64), ((rsi as u32)) as u32); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; fl_l = rdx; fl_r = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { let y_addr: u64 = rdi; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rdx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rdi.wrapping_add(0x20u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rdx.wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); let y_addr: u64 = rdi.wrapping_add(0x40u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rdx.wrapping_add(0x40u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rdi.wrapping_add(0x60u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rdx.wrapping_add(0x60u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x80u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x80u64), ((rax as u32)) as u32); fl_l = (crate::simd_ops::lk_add_u64(rbp.wrapping_add(0x8u64), (0x84u64) as u64)) as u64; fl_r = fl_l; bb = 10; continue; }
            9 => { r8 = rdi; rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rdx; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rdx, (xmm0) as u128); fl_l = (crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0x10u64) as u64)) as u64; fl_r = fl_l; bb = 13; continue; }
            12 => { r8 = rsp.wrapping_add(0x20); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 13; continue; }
            13 => { rax = ((rsi as u32)) as u32 as u64; bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252e950(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14252ebc0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x4u32) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    r8 = rsp.wrapping_add(0x20);
    rax = ((rdx as u32)) as u32 as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rdx as u32)) as u32);
    rdx = rsp.wrapping_add(0x30);
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax);
    crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x4u32) as u32);
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252e990(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14252ead0; const P1: u64 = 0x14252ebc0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    rdx = rsp.wrapping_add(0x30);
    rbx = rcx;
    crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x3u32) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    r8 = rsp.wrapping_add(0x20);
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32);
    rdx = rsp.wrapping_add(0x30);
    rax = ((rax as u32)) as u32 as u64;
    rcx = rbx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax);
    crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x3u32) as u32);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252e9f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14252ebc0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), ((xmm1 as u64)) as u64);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64;
    r8 = rsp.wrapping_add(0x20);
    rdx = rsp.wrapping_add(0x30);
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), ((xmm1 as u64)) as u64);
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x2u32) as u32);
    crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x2u32) as u32);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252ea40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14252ebc0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    r8 = rsp.wrapping_add(0x20);
    rax = (((rdx as u8)) as u32) as u32 as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((rdx as u8)) as u8);
    rdx = rsp.wrapping_add(0x30);
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax);
    crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x1u32) as u32);
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252ea90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14252ebc0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    r8 = rsp.wrapping_add(0x20);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rdx as u32)) as u32);
    crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rdx as u32)) as u32);
    rdx = rsp.wrapping_add(0x30);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax);
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252ead0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1e8; const P1: u64 = 0x1f0; const P2: u64 = 0x141471ec0; const P3: u64 = 0x1e0; const P4: u64 = 0x141471b80; const P5: u64 = 0x141471420; const P6: u64 = 0x10; const P7: u64 = 0x288; const P8: u64 = 0x218; const P9: u64 = 0x210; const P10: u64 = 0x220; const P11: u64 = 0x141470ab0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rsi = rdx; rdi = rcx; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shr(2u32)) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = ((rcx).wrapping_add(0x1e0u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = ((rdi.wrapping_add(P3)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            3 => { rdx = rsi; rcx = ((rdi.wrapping_add(P3)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P6))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P6), ((rax as u32)) as u32); fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(P7))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P8))) as u64; rcx = ((rdi.wrapping_add(P9)) as u64) as u64; fl_l = rdx; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(P10))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { xmm0 = (crate::simd_ops::ru128(rsi)) as u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); crate::simd_ops::lk_add_u64(rdi.wrapping_add(P8), (0x10u64) as u64); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { r8 = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            8 => { rax = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252ebc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xb8; const P1: u64 = 0x141471940; const P2: u64 = 0x10; const P3: u64 = 0x58; const P4: u64 = 0x50; const P5: u64 = 0x8; const P6: u64 = 0x141472540; const P7: u64 = 0x141471a30; const P8: u64 = 0x60; const P9: u64 = 0x1414733f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = ((rcx.wrapping_add(P0)) as u64) as u64; rbx = rcx; rcx = rsi; rbp = r8; r14 = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = 0; fl_l = 0; fl_r = 0; fl_l = rax; fl_r = rax; r9 = ((rax.wrapping_add(P2)) as u64) as u64; r9 = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rcx } else { r9 }) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(r9)) as u32 as u64; bb = 12; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; rdi = ((rdi).wrapping_sub(crate::simd_ops::ru64(rbx.wrapping_add(P4)))) as u64; rdi = ((((rdi) as i64).wrapping_shr(4u32) as u64)) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x800000; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = 0xffffffff; bb = 11; continue; }
            4 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(P5))) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shr(2u32)) as u64; fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(P2))) as u64; fl_r = rcx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rdx = r14; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            7 => { rdx = r14; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rax.wrapping_add(P2), ((rdi as u32)) as u32); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_l = rdx; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(P8))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { xmm0 = (crate::simd_ops::ru128(rbp)) as u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); fl_l = (crate::simd_ops::lk_add_u64(rbx.wrapping_add(P3), (0x10u64) as u64)) as u64; fl_r = fl_l; bb = 10; continue; }
            9 => { r8 = rbp; rcx = ((rbx.wrapping_add(P4)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            10 => { rax = ((rdi as u32)) as u32 as u64; bb = 11; continue; }
            11 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252ecc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141470d30; const P1: u64 = 0x14252d700; const P2: u64 = 0x140203140; const P3: u64 = 0x1426467f0; const P4: u64 = 0x141834d30; const P5: u64 = 0x1418350d0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x18u64))) as u32 as u64; rbx = rcx; rdi = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(rcx))) as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x20u64), ((rdx as u8)) as u8); crate::simd_ops::wu8(rdi.wrapping_add(0x22u64), ((r8 as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x28u64))) as u64; rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rcx.wrapping_add(0x20u64)))) as u64; rcx = rdi; rax = ((((rax) as i64).wrapping_shr(2u32) as u64)) as u64; rdx = (((rax) as i64).wrapping_mul((0x7u64) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(0x20u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; rdx = rdi; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x2c0u64))) as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x18u64), (0xffffffffu32) as u32); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { r8 = rdi.wrapping_add(0x70); rcx = rbx; rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(rax, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rdi.wrapping_add(0x30); rdx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = rdx; fl_r = (0xfu64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rdx = ((rdx).wrapping_add(1)) as u64; rax = rcx; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x14252ed73, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), rax); bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rbx.wrapping_add(0x40u64), rax); bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x50u64))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rbx.wrapping_add(0x58u64), rax); bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x68u64))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x70u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rbx.wrapping_add(0x70u64), rax); bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x80u64))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x88u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rbx.wrapping_add(0x88u64), rax); bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x98u64))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0xa0u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 17; continue; }
            17 => { crate::simd_ops::wu64(rbx.wrapping_add(0xa0u64), rax); bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x1b0u64))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x1b8u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::wu64(rbx.wrapping_add(0x1b8u64), rax); bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x1c8u64))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x1d0u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 22; continue; } bb = 21; continue; }
            21 => { crate::simd_ops::wu64(rbx.wrapping_add(0x1d0u64), rax); bb = 22; continue; }
            22 => { rdi = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0xc8u64))) as u64; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 29; continue; } bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0xc0u64))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0xb8u64))) as u64; fl_l = rcx; fl_r = (0x20u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 25; continue; } bb = 24; continue; }
            24 => { rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0xb8u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0xc0u64), rdi); bb = 28; continue; }
            25 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 26; continue; }
            26 => { bb = 27; continue; }
            27 => { xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(0xd0u64))) as u128; ymm0_hi = 0; rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu128(rax, (xmm0) as u128); crate::simd_ops::wu32(rax.wrapping_add(0x10u64), ((rdx as u32)) as u32); rax = rax.wrapping_add(0x18); fl_l = rcx; fl_r = (0x1u64) as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 27; continue; } bb = 28; continue; }
            28 => { crate::simd_ops::wu64(rbx.wrapping_add(0xc8u64), rdi); bb = 29; continue; }
            29 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0xf8u64))) as u64; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 36; continue; } bb = 30; continue; }
            30 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0xf0u64))) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0xe8u64))) as u64; fl_l = rdx; fl_r = (0x20u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 32; continue; } bb = 31; continue; }
            31 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0xe8u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0xf0u64), rdi); bb = 35; continue; }
            32 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 35; continue; } bb = 33; continue; }
            33 => { bb = 34; continue; }
            34 => { let y_addr: u64 = rbx.wrapping_add(0x100u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rbx.wrapping_add(0x120u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); let y_addr: u64 = rbx.wrapping_add(0x140u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_add(0x40u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rbx.wrapping_add(0x160u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_add(0x60u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x180u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x80u64), ((rax as u32)) as u32); rcx = rcx.wrapping_add(0x88); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rcx.wrapping_sub(0x4u64), ((rax as u32)) as u32); fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 34; continue; } bb = 35; continue; }
            35 => { crate::simd_ops::wu64(rbx.wrapping_add(0xf8u64), rdi); bb = 36; continue; }
            36 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x198u64))) as u64; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 43; continue; } bb = 37; continue; }
            37 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x190u64))) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x188u64))) as u64; fl_l = r8; fl_r = (0x20u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 39; continue; } bb = 38; continue; }
            38 => { rcx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0x188u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x190u64), rdi); bb = 42; continue; }
            39 => { rcx = rdi; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 42; continue; } bb = 40; continue; }
            40 => { bb = 41; continue; }
            41 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x1a0u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(rcx.wrapping_mul(8)), ((rax as u32)) as u32); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu16(rdx.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x4u64), ((rax as u16)) as u16); rcx = ((rcx).wrapping_add(1)) as u64; fl_l = rcx; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 41; continue; } bb = 42; continue; }
            42 => { crate::simd_ops::wu64(rbx.wrapping_add(0x198u64), rdi); bb = 43; continue; }
            43 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x228u64))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x230u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 45; continue; } bb = 44; continue; }
            44 => { crate::simd_ops::wu64(rbx.wrapping_add(0x230u64), rax); bb = 45; continue; }
            45 => { rbx = ((rbx).wrapping_add(0x240u64)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdi); fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_r = (0xfu64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 47; continue; } bb = 46; continue; }
            46 => { rbx = (crate::simd_ops::ru64(rbx)) as u64; bb = 47; continue; }
            47 => { crate::simd_ops::wu8(rbx, ((rdi as u8)) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252efc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434278a0; const P1: u64 = 0x140203210; const P2: u64 = 0x140208950; const P3: u64 = 0x1414737c0; const P4: u64 = 0x141834d30; const P5: u64 = 0x1426467f0; const P6: u64 = 0x14252f1d3;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x14252efc0, funcinfo_rva: 0x37faba0, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0xc8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rbp = ((rsp.wrapping_sub(0x47u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0xb0u64)) as u64; r10 = (((rdx as u8)) as u32) as u32 as u64; rbx = (rcx) as u64; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; r9 = (rdi) as u64; r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(rcx))) as u64; rax = (0x7878787878787879u64) as u64; let prod = (rax as i64 as i128).wrapping_mul((r9) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rsi = (rdx) as u64; rsi = ((((rsi) as i64).wrapping_shr(6u32) as u64)) as u64; rax = (rsi) as u64; rax = ((rax).wrapping_shr(63u32)) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x49u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(P0)) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x39u64), (xmm1) as u128); crate::simd_ops::wu8(rbp.wrapping_sub(0x49u64), (0x0u8) as u8); crate::simd_ops::wu8(rbp.wrapping_sub(0x29u64), (0x0u8) as u8); crate::simd_ops::wu8(rbp.wrapping_sub(0x27u64), (0x0u8) as u8); r14 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x25u64), (r14) as u64); crate::simd_ops::wu128(rbp.wrapping_sub(0x19u64), (xmm0) as u128); crate::simd_ops::wu128(rbp.wrapping_sub(0x9u64), (xmm1) as u128); crate::simd_ops::wu8(rbp.wrapping_sub(0x19u64), ((r14 as u8)) as u8); crate::simd_ops::wu128(rbp.wrapping_add(0x7u64), (xmm0) as u128); crate::simd_ops::wu128(rbp.wrapping_add(0x17u64), (xmm1) as u128); crate::simd_ops::wu8(rbp.wrapping_add(0x7u64), ((r14 as u8)) as u8); crate::simd_ops::wu128(rbp.wrapping_add(0x27u64), (xmm0) as u128); crate::simd_ops::wu64(rbp.wrapping_add(0x37u64), (r14) as u64); crate::simd_ops::wu8(rbp.wrapping_sub(0x28u64), ((r10 as u8)) as u8); crate::simd_ops::wu8(rbp.wrapping_sub(0x26u64), ((r8 as u8)) as u8); fl_l = (rdi) as u64 as u64; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rbp.wrapping_add(0x67u64), (rdi) as u64); rdx = ((rbp.wrapping_sub(0x49u64)) as u64) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252f070); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((crate::simd_ops::ru8(rbp.wrapping_sub(0x29u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x20u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbp.wrapping_sub(0x28u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x21u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbp.wrapping_sub(0x27u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x22u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbp.wrapping_sub(0x26u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x23u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x25u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x24u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x21u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x28u64), ((rax as u32)) as u32); rcx = ((rdi.wrapping_add(0x30u64)) as u64) as u64; rdx = ((rbp.wrapping_sub(0x19u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252f0a6); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rdi.wrapping_add(0x50u64)) as u64) as u64; rdx = ((rbp.wrapping_add(0x7u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252f0b4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rdi.wrapping_add(0x70u64)) as u64) as u64; rdx = ((rbp.wrapping_add(0x27u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252f0c2); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0x88u64) as u64)) as u64; fl_r = fl_l; bb = 3; continue; }
            2 => { r8 = ((rbp.wrapping_sub(0x49u64)) as u64) as u64; rdx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252f0d9); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 3; continue; }
            3 => { crate::simd_ops::wu32(rbx.wrapping_add(0x18u64), ((rsi as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0xb0u64), (0x0u8) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0x1a8u64), ((r14 as u32)) as u32); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x27u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x37u64))) as u64; rdx = ((rdx).wrapping_sub(rcx)) as u64; rdx = (((rdx) & (0xfffffffffffffffcu64))) as u64; rax = (rcx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 18; continue; } bb = 6; continue; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252f128); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x1fu64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0xfu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 11; continue; } bb = 8; continue; }
            8 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x7u64))) as u64; rax = (rcx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 18; continue; } bb = 10; continue; }
            10 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252f15f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x1u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0xfu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 15; continue; } bb = 12; continue; }
            12 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x19u64))) as u64; rax = (rcx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 18; continue; } bb = 14; continue; }
            14 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252f196); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 15; continue; }
            15 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x31u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0xfu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 20; continue; } bb = 16; continue; }
            16 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x49u64))) as u64; rax = (rcx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 19; continue; } bb = 17; continue; }
            17 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 19; continue; } bb = 18; continue; }
            18 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252f1cd); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P6, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            19 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252f1d4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 20; continue; }
            20 => { rax = ((rsi as u32)) as u32 as u64; r11 = ((rsp.wrapping_add(0xb0u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = (r11) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252f200(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x80;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = 0x811c9dc5;
    fl_l = r8; fl_r = r8;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    loop { rax = ((((rax as u32)) ^ (crate::simd_ops::ru32(rdx.wrapping_add(rcx.wrapping_mul(4)))))) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; rax = ((((rax as u32)) as i32).wrapping_mul((0x1000193u32) as i32) as u32) as u32 as u64; fl_l = rcx; fl_r = r8; if !((((fl_l as u64)) < ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252f240(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rax = (((crate::simd_ops::ru32(rdx)) as i32).wrapping_mul((0x5bd1e995u32) as i32) as u32) as u32 as u64;
    r8 = rcx;
    r8 = ((r8).wrapping_shr(32u32)) as u64;
    r8 = ((((r8 as u32)) ^ ((rax as u32)))) as u32 as u64;
    rax = ((r8 as u32)) as u32 as u64;
    rax = (((rax as u32)).wrapping_shr(18u32)) as u32 as u64;
    rax = ((((rax as u32)) ^ ((rcx as u32)))) as u32 as u64;
    rcx = ((((rax as u32)) as i32).wrapping_mul((0x5bd1e995u32) as i32) as u32) as u32 as u64;
    rax = ((rcx as u32)) as u32 as u64;
    rax = (((rax as u32)).wrapping_shr(22u32)) as u32 as u64;
    rax = ((((rax as u32)) ^ ((r8 as u32)))) as u32 as u64;
    rdx = ((((rax as u32)) as i32).wrapping_mul((0x5bd1e995u32) as i32) as u32) as u32 as u64;
    rax = ((rdx as u32)) as u32 as u64;
    rax = (((rax as u32)).wrapping_shr(17u32)) as u32 as u64;
    rax = ((((rax as u32)) ^ ((rcx as u32)))) as u32 as u64;
    rcx = ((((rax as u32)) as i32).wrapping_mul((0x5bd1e995u32) as i32) as u32) as u32 as u64;
    rcx = (((rcx as u32)).wrapping_shr(19u32)) as u32 as u64;
    rcx = ((((rcx as u32)) ^ ((rdx as u32)))) as u32 as u64;
    rax = ((((rcx as u32)) as i32).wrapping_mul((0x5bd1e995u32) as i32) as u32) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252f290(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r9 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    r8 = 0x811c9dc5;
    fl_l = r9; fl_r = r9;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax = ((r8 as u32)) as u32 as u64; return rax; }
    rdx = (crate::simd_ops::ru64(rdx)) as u64;
    loop { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; rax = ((((rax as u32)) ^ ((r8 as u32)))) as u32 as u64; r8 = ((((rax as u32)) as i32).wrapping_mul((0x1000193u32) as i32) as u32) as u32 as u64; fl_l = rcx; fl_r = r9; if !((((fl_l as u64)) < ((fl_r as u64)))) { break; } }
    rax = ((r8 as u32)) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252f2d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x80; const P1: u64 = 0x14183acda;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    if (((fl_l as u32)) != ((fl_r as u32))) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    r8 = ((rax as u32)) as u32 as u64;
    r8 = ((r8).wrapping_shl(2u32)) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) != 0)) { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252f310(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14183acda;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = rcx;
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rcx; fl_r = rcx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { fl_l = rcx; fl_r = (crate::simd_ops::ru64(rdx)) as u64; rax = (rax & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    r9 = (crate::simd_ops::ru64(rdx)) as u64;
    fl_l = r9; fl_r = r9;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { r8 = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; fl_l = r8; fl_r = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; } rdx = r9; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if !(((((fl_l as u32)) & ((fl_r as u32))) != 0)) { rax = 0x1; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; } rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(rdx)) as u64;
    rax = (rax & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64));
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252f360(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140000000;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut j_tgt) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r9 = (((rcx as u8)) as u32) as u32 as u64; r10 = ((rdx as u32)) as u32 as u64; rax = ((r9.wrapping_sub(0x17u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x39u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rdx = ((P0) as u64) as u64; rax = (rax as u32 as i32 as i64) as u64; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rax).wrapping_add(0x252f3ecu64))) as u32) as u32 as u64; r8 = (crate::simd_ops::ru32(rdx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x252f3e4u64))) as u32 as u64; fl_l = ((r8).wrapping_add(rdx)) as u64 as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; j_tgt = (r8) as u64; bb = 0xffff_fffe; continue; }
            2 => { rcx = (((((rcx as u32)) as i32).wrapping_shr(16u32) as u32)) as u32 as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((r10 as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; return rax; }
            3 => { rdx = ((r9 as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x44u32) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 4; continue; }
            4 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x5u32) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 5; continue; }
            5 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 6; continue; }
            6 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 7; continue; }
            7 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rcx = (((rcx as u32)).wrapping_shr(24u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { rax = ((r10.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((rcx as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; return rax; }
            10 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = (0x43u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = (((((rcx as u32)) as i32).wrapping_shr(8u32) as u32)) as u32 as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((r10 as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; return rax; }
            12 => { rax = (0xffffffffu32) as u32 as u64; return rax; }
            13 => { rcx = (((rcx as u32)).wrapping_shr(24u32)) as u32 as u64; rax = ((rcx.wrapping_add(0x2u64)) as u32) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((r10 as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x14252f360u64) { 0x10 => { bb = 1; continue; } 0x2f => { bb = 2; continue; } 0x39 => { bb = 3; continue; } 0x41 => { bb = 4; continue; } 0x46 => { bb = 5; continue; } 0x4b => { bb = 6; continue; } 0x50 => { bb = 7; continue; } 0x56 => { bb = 8; continue; } 0x5d => { bb = 9; continue; } 0x64 => { bb = 10; continue; } 0x6a => { bb = 11; continue; } 0x74 => { bb = 12; continue; } 0x7a => { bb = 13; continue; } _ => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(j_tgt, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); } } }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252f430(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140000000;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (((rcx as u32)).wrapping_add(0xfffffff9u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x49u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = ((((rcx as u32)) as i32 as i64) as u64) as u64; rdx = ((P0) as u64) as u64; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rax).wrapping_add(0x252f46cu64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x252f464u64))) as u32 as u64; fl_l = ((rcx).wrapping_add(rdx)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; j_tgt = (rcx) as u64; bb = 0xffff_fffe; continue; }
            2 => { rax = (0x2u32) as u32 as u64; return rax; }
            3 => { rax = (0x1u32) as u32 as u64; return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x14252f430u64) { 0x8 => { bb = 1; continue; } 0x26 => { bb = 2; continue; } 0x2c => { bb = 3; continue; } _ => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(j_tgt, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); } } }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252f4c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140571d70;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = ((rdx as u32)) as u32 as u64;
    rdi = rcx;
    rsi = 0x80;
    loop { rdx = 0; fl_l = 0; fl_r = 0; rax = (((rbx as u8)) as u32) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x7f; r8 = 0x1; rcx = rdi; rdx = (if (((fl_l as u32)) > ((fl_r as u32))) { (rsi as u32) } else { (rdx as u32) }) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7fu8))) as u8 as u64)); rdx = (rdx & !0xffu64) | ((((((rdx as u8)) | ((rax as u8)))) as u8 as u64)); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((rdx as u8)) as u8); rdx = rsp.wrapping_add(0x38); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = (((rbx as u32)).wrapping_shr(7u32)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if !(((((fl_l as u32)) & ((fl_r as u32))) != 0)) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252f530(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140571d70;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), ((xmm1 as u64)) as u64);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    r8 = 0x8;
    rdx = rsp.wrapping_add(0x38);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252f560(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140571d70;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu32(rsp.wrapping_add(0x10u64), ((rdx as u32)) as u32);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    r8 = 0x4;
    rdx = rsp.wrapping_add(0x38);
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252f590(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140571d70;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu8(rsp.wrapping_add(0x10u64), ((rdx as u8)) as u8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    r8 = 0x1;
    rdx = rsp.wrapping_add(0x38);
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252f5c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2;
    if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { return rax; }
    r8 = 0x2;
    loop { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; rdx = (((r8 as u32)).wrapping_shl(((((rax as u32)) as u32) & 31))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if !(((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14252f5f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141472810; const P1: u64 = 0x141472790; const P2: u64 = 0x1414728a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x14252f5f0, funcinfo_rva: 0x37fac20, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x48, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = (rcx) as u64; rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx, (rax) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), (rax) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), (rax) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), (0xffffffffffffffffu64) as u64); rcx = ((rcx).wrapping_add(0x20u64)) as u64; crate::simd_ops::wu64(rcx, (rax) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), (rax) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x40u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x48u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x50u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x58u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x60u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x68u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x70u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x78u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x80u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x88u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x90u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x98u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0xa0u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0xa8u64), (rax) as u64); crate::simd_ops::wu8(rbx.wrapping_add(0xb0u64), ((rax as u8)) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0xffffffffffffffffu64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0xb8u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0xc0u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0xc8u64), (rax) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbx.wrapping_add(0xd0u64), (xmm0) as u128); xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu64(rbx.wrapping_add(0xe8u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0xf0u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0xf8u64), (rax) as u64); let y_addr: u64 = rbx.wrapping_add(0x100u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); let y_addr: u64 = rbx.wrapping_add(0x120u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); let y_addr: u64 = rbx.wrapping_add(0x140u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); let y_addr: u64 = rbx.wrapping_add(0x160u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); crate::simd_ops::wu32(rbx.wrapping_add(0x180u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x188u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x190u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x198u64), (rax) as u64); crate::simd_ops::wu32(rbx.wrapping_add(0x1a0u64), (0xffffffffu32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x1a8u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x1b0u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x1b8u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x1c0u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x1c8u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x1d0u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x1d8u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x1e0u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x1e8u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x1f0u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x1f8u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x200u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x210u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x218u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x220u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x228u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x230u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x238u64), (rax) as u64); crate::simd_ops::wu128(rbx.wrapping_add(0x240u64), (xmm0) as u128); crate::simd_ops::wu64(rbx.wrapping_add(0x250u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x258u64), (0xfu64) as u64); crate::simd_ops::wu8(rbx.wrapping_add(0x240u64), ((rax as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x260u64), (rax) as u64); crate::simd_ops::wu128(rbx.wrapping_add(0x268u64), (xmm0) as u128); crate::simd_ops::wu64(rbx.wrapping_add(0x278u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x280u64), (0xfu64) as u64); crate::simd_ops::wu8(rbx.wrapping_add(0x268u64), ((rax as u8)) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0x288u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x290u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x298u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x2a0u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x2a8u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x2b0u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x2b8u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x2c0u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (0x20u64) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rcx))) as u64; rax = ((((rax) as i64).wrapping_shr(2u32) as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x20u64) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = ((rsp.wrapping_add(0x58u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252f82d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (0x20u64) as u64); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x48u64))) as u64; rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rbx.wrapping_add(0x38u64)))) as u64; rax = ((((rax) as i64).wrapping_shr(2u32) as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x20u64) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = ((rsp.wrapping_add(0x58u64)) as u64) as u64; rcx = ((rbx.wrapping_add(0x38u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252f859); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (0x10u64) as u64); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x60u64))) as u64; rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rbx.wrapping_add(0x50u64)))) as u64; rax = ((((rax) as i64).wrapping_shr(4u32) as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x10u64) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = ((rsp.wrapping_add(0x58u64)) as u64) as u64; rcx = ((rbx.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252f885); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (0x10u64) as u64); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x78u64))) as u64; rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rbx.wrapping_add(0x68u64)))) as u64; rax = ((((rax) as i64).wrapping_shr(2u32) as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x10u64) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = ((rsp.wrapping_add(0x58u64)) as u64) as u64; rcx = ((rbx.wrapping_add(0x68u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252f8b1); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (0x8u64) as u64); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rbx))) as u64; rax = (0x7878787878787879u64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(6u32) as u64)) as u64; rax = (rdx) as u64; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x8u64) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = ((rsp.wrapping_add(0x58u64)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14252f8f2); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 10; continue; }
            10 => { rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425300b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434241f0; const P1: u64 = 0x140203690; const P2: u64 = 0x1426680a0; const P3: u64 = 0x8; const P4: u64 = 0x142646330;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = P0;
    rbx = rcx;
    crate::simd_ops::wu64(rcx, rax);
    rcx = ((rcx).wrapping_add(0x28u64)) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = P2;
    rcx = ((rbx.wrapping_add(P3)) as u64) as u64;
    crate::simd_ops::wu64(rbx, rax);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142530150(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141834d64; const P1: u64 = 0x142529380; const P2: u64 = 0x142524b00; const P3: u64 = 0x143423cf0; const P4: u64 = 0x142527620; const P5: u64 = 0x140571d70; const P6: u64 = 0x1426467f0; const P7: u64 = 0x1425302a1; const P8: u64 = 0x141834d30; const P9: u64 = 0x14146ec60; const P10: u64 = 0x14146ef90; const P11: u64 = 0x14146ece0; const P12: u64 = 0x1418350d0; const P13: u64 = 0x14252f5f0; const P14: u64 = 0x14252f910; const P15: u64 = 0x140203210; const P16: u64 = 0x1413b9c60;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 127>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x142530150, funcinfo_rva: 0x37fad9c, handler: 1, shape: 2, frame_reg: 0, establisher_off: 0x3f8, conts: &[0x1425303d6] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x3e0u64)) as u64; rbx = (r9) as u64; r15 = (r8) as u64; r14 = (rdx) as u64; rsi = (rcx) as u64; rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rdi as u32)) as u32); rcx = (0x2008u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142530182); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (rdi) as u64); crate::simd_ops::wu64(rax, (rdi) as u64); rdx = ((rsp.wrapping_add(0x58u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x68u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14253019e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; r8 = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64 as u64; fl_r = (0xfu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { r14 = (crate::simd_ops::ru64(r14)) as u64; bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); rax = ((rsp.wrapping_add(0x58u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); r9 = ((rsp.wrapping_add(0x68u64)) as u64) as u64; rdx = (r14) as u64; rcx = ((rsp.wrapping_add(0xa0u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1425301d2); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1425301f4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; r9 = (rax) as u64; rdx = ((P3) as u64) as u64; rcx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14253020d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = 0u128; crate::simd_ops::wu128(rsi, (xmm0) as u128); crate::simd_ops::wu64(rsi.wrapping_add(0x18u64), (0xfu64) as u64); crate::simd_ops::wu8(rsi, (0x0u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x2u32) as u32); crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), (0x1u64) as u64); rax = (rsi) as u64; fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64 as u64; fl_r = (0xfu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rsi)) as u64; bb = 5; continue; }
            5 => { crate::simd_ops::wu16(rax, (0x0u16) as u16); rdx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64 as u64; fl_r = (0xfu64) as u64 as u64; rdx = (if (((fl_l as u64)) > ((fl_r as u64))) { crate::simd_ops::ru64(rsp.wrapping_add(0x38u64)) } else { rdx }) as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142530261); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0xfu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 10; continue; } bb = 6; continue; }
            6 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = (rcx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14253029b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P7, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            9 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1425302a2); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rdi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (0xfu64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x0u8) as u8); rcx = ((rsp.wrapping_add(0xe0u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1425302c2); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rsp.wrapping_add(0xc8u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1425302cf); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rsp.wrapping_add(0xb0u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1425302dc); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1425302ec); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (rdi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (rdi) as u64); bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 13; continue; }
            13 => { bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142530313); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rbx) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 15; continue; }
            15 => { bb = 26; continue; }
            16 => { rcx = ((rsp.wrapping_add(0x110u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14253032d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x14253032d, (P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x1425303d6 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 21; continue; } _ => l.unknown(0x142530150) } }; rax = c_rax; xmm0 = c_x0; r9 = (r15) as u64; r8 = ((rsp.wrapping_add(0x68u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0xa0u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x110u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14253034b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x14253034b, (P14) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x1425303d6 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 21; continue; } _ => l.unknown(0x142530150) } }; rax = c_rax; xmm0 = c_x0; rdx = ((rsp.wrapping_add(0x378u64)) as u64) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14253035b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x14253035b, (P15) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x1425303d6 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 21; continue; } _ => l.unknown(0x142530150) } }; rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x1u32) as u32); rcx = ((rsp.wrapping_add(0x110u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142530370); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x142530370, (P16) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x1425303d6 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 21; continue; } _ => l.unknown(0x142530150) } }; rax = c_rax; xmm0 = c_x0; rcx = ((rsp.wrapping_add(0xe0u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14253037e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x14253037e, (P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x1425303d6 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 21; continue; } _ => l.unknown(0x142530150) } }; rax = c_rax; xmm0 = c_x0; rcx = ((rsp.wrapping_add(0xc8u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14253038b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x14253038b, (P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x1425303d6 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 21; continue; } _ => l.unknown(0x142530150) } }; rax = c_rax; xmm0 = c_x0; rcx = ((rsp.wrapping_add(0xb0u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142530398); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x142530398, (P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x1425303d6 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 21; continue; } _ => l.unknown(0x142530150) } }; rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1425303a8); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x1425303a8, (P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x1425303d6 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 21; continue; } _ => l.unknown(0x142530150) } }; rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (rdi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (rdi) as u64); bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { rbx = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1425303c4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x1425303c4, (P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x1425303d6 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 21; continue; } _ => l.unknown(0x142530150) } }; rax = c_rax; xmm0 = c_x0; rcx = (rbx) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 19; continue; } bb = 20; continue; }
            20 => { rax = (rsi) as u64; bb = 27; continue; }
            21 => { rcx = ((rsp.wrapping_add(0xe0u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1425303de); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x1425303de, (P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x1425303d6 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 21; continue; } _ => l.unknown(0x142530150) } }; rax = c_rax; xmm0 = c_x0; rcx = ((rsp.wrapping_add(0xc8u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1425303eb); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x1425303eb, (P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x1425303d6 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 21; continue; } _ => l.unknown(0x142530150) } }; rax = c_rax; xmm0 = c_x0; rcx = ((rsp.wrapping_add(0xb0u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1425303f8); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x1425303f8, (P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x1425303d6 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 21; continue; } _ => l.unknown(0x142530150) } }; rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142530408); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x142530408, (P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x1425303d6 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 21; continue; } _ => l.unknown(0x142530150) } }; rax = c_rax; xmm0 = c_x0; rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (rdi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (rdi) as u64); bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 24; continue; }
            24 => { rbx = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142530426); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x142530426, (P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x1425303d6 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 21; continue; } _ => l.unknown(0x142530150) } }; rax = c_rax; xmm0 = c_x0; rcx = (rbx) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 24; continue; } bb = 25; continue; }
            25 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x400u64))) as u64; bb = 26; continue; }
            26 => { rax = (rsi) as u64; bb = 27; continue; }
            27 => { r11 = ((rsp.wrapping_add(0x3e0u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142530460(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1433e1058; const P1: u64 = 0x143106ac0; const P2: u64 = 0x1433e0fb0; const P3: u64 = 0x1433e0ff8; const P4: u64 = 0x1433e1000; const P5: u64 = 0x1433e1008; const P6: u64 = 0x14183ad52; const P7: u64 = 0x1433e0ff0; const P8: u64 = 0x1433dfcb4; const P9: u64 = 0x1433ddfc4; const P10: u64 = 0x1433e1030; const P11: u64 = 0x1433e0fd8; const P12: u64 = 0x1433e0648; const P13: u64 = 0x1433de034; const P14: u64 = 0x1433de010; const P15: u64 = 0x1433ddffc; const P16: u64 = 0x1433e06ac; const P17: u64 = 0x1433de008; const P18: u64 = 0x1433ddff4; const P19: u64 = 0x1433e06a4; const P20: u64 = 0x1433de020; const P21: u64 = 0x1433ddfc0; const P22: u64 = 0x1433de018; const P23: u64 = 0x1433ddfdc; const P24: u64 = 0x1433e069c; const P25: u64 = 0x1433e0694; const P26: u64 = 0x1433e068c; const P27: u64 = 0x1433e0684; const P28: u64 = 0x1433de01c; const P29: u64 = 0x14330f208; const P30: u64 = 0x14330f204; const P31: u64 = 0x1433e067c; const P32: u64 = 0x1433de004; const P33: u64 = 0x1433ddfbc; const P34: u64 = 0x1433e0660; const P35: u64 = 0x1433de024; const P36: u64 = 0x1433ddfe4; const P37: u64 = 0x1433e0658; const P38: u64 = 0x1433de028; const P39: u64 = 0x1433ddfcc; const P40: u64 = 0x1433de02c; const P41: u64 = 0x1433ddfec; const P42: u64 = 0x1433e0734; const P43: u64 = 0x1433e0f10; const P44: u64 = 0x1433e0f04; const P45: u64 = 0x1433e0efc; const P46: u64 = 0x1433e0ef8; const P47: u64 = 0x1433e0ef0; const P48: u64 = 0x1433e0ee8; const P49: u64 = 0x1433e0ee0; const P50: u64 = 0x1433e0ed8; const P51: u64 = 0x1433e0ed0; const P52: u64 = 0x1432a4f48; const P53: u64 = 0x1433e0ec8; const P54: u64 = 0x1433e0ec0; const P55: u64 = 0x1433e0eb8; const P56: u64 = 0x1433e0eb0; const P57: u64 = 0x14336adcc; const P58: u64 = 0x1433dfee0; const P59: u64 = 0x14307dd80; const P60: u64 = 0x1433dfac8; const P61: u64 = 0x1433de038; const P62: u64 = 0x1433dfc5c; const P63: u64 = 0x1433dfccc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; r8 = ((rcx.wrapping_add(0x8u64)) as u64) as u64; rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rbp = (rdx) as u64; rsi = (rcx) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 41; continue; } bb = 1; continue; }
            1 => { r9 = (crate::simd_ops::ru64(r8)) as u64; fl_l = (r9) as u64 as u64; fl_r = (r9) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { rcx = ((rdi as u32)) as u32 as u64; rdx = ((P0) as u64) as u64; bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru8(r9.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rdx.wrapping_add(rcx).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = (rcx) as u64 as u64; fl_r = (0x7u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 5; continue; }
            5 => { rax = ((rcx.wrapping_sub(0x6u64)) as u32) as u32 as u64; bb = 299; continue; }
            6 => { fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 41; continue; } bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(r8)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { rcx = (rdi) as u64; r8 = ((P1) as u64) as u64; bb = 9; continue; }
            9 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r8.wrapping_add(rcx).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 12; continue; } bb = 10; continue; }
            10 => { fl_l = (rcx) as u64 as u64; fl_r = (0x5u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 11; continue; }
            11 => { rax = ((rcx.wrapping_add(0x23u64)) as u32) as u32 as u64; bb = 299; continue; }
            12 => { r8 = ((rsi.wrapping_add(0x8u64)) as u64) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 41; continue; } bb = 13; continue; }
            13 => { rdx = (crate::simd_ops::ru64(r8)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 14; continue; }
            14 => { rcx = (rdi) as u64; r8 = ((P2) as u64) as u64; bb = 15; continue; }
            15 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r8.wrapping_add(rcx).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 18; continue; } bb = 16; continue; }
            16 => { fl_l = (rcx) as u64 as u64; fl_r = (0x7u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 17; continue; }
            17 => { rax = ((rcx.wrapping_add(0x25u64)) as u32) as u32 as u64; bb = 299; continue; }
            18 => { r8 = ((rsi.wrapping_add(0x8u64)) as u64) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 41; continue; } bb = 19; continue; }
            19 => { rdx = (crate::simd_ops::ru64(r8)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 20; continue; }
            20 => { rcx = (rdi) as u64; r8 = ((P3) as u64) as u64; bb = 21; continue; }
            21 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r8.wrapping_add(rcx).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 24; continue; } bb = 22; continue; }
            22 => { fl_l = (rcx) as u64 as u64; fl_r = (0x7u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 23; continue; }
            23 => { rax = ((rcx.wrapping_add(0x2au64)) as u32) as u32 as u64; bb = 299; continue; }
            24 => { r8 = ((rsi.wrapping_add(0x8u64)) as u64) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 41; continue; } bb = 25; continue; }
            25 => { rdx = (crate::simd_ops::ru64(r8)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 26; continue; }
            26 => { rcx = (rdi) as u64; r8 = ((P4) as u64) as u64; bb = 27; continue; }
            27 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r8.wrapping_add(rcx).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 30; continue; } bb = 28; continue; }
            28 => { fl_l = (rcx) as u64 as u64; fl_r = (0x7u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 27; continue; } bb = 29; continue; }
            29 => { rax = ((rcx.wrapping_add(0x2bu64)) as u32) as u32 as u64; bb = 299; continue; }
            30 => { r8 = ((rsi.wrapping_add(0x8u64)) as u64) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 41; continue; } bb = 31; continue; }
            31 => { rcx = (crate::simd_ops::ru64(r8)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 34; continue; } bb = 32; continue; }
            32 => { rdx = ((P5) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 34; continue; } bb = 33; continue; }
            33 => { rax = ((rbx.wrapping_add(0x33u64)) as u32) as u32 as u64; bb = 299; continue; }
            34 => { r8 = ((rsi.wrapping_add(0x8u64)) as u64) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 41; continue; } bb = 35; continue; }
            35 => { rdx = (crate::simd_ops::ru64(r8)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 40; continue; } bb = 36; continue; }
            36 => { rcx = (rdi) as u64; r8 = ((P7) as u64) as u64; bb = 37; continue; }
            37 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r8.wrapping_add(rcx).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 40; continue; } bb = 38; continue; }
            38 => { fl_l = (rcx) as u64 as u64; fl_r = (0x7u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 37; continue; } bb = 39; continue; }
            39 => { rax = ((rcx.wrapping_add(0x33u64)) as u32) as u32 as u64; bb = 299; continue; }
            40 => { r8 = ((rsi.wrapping_add(0x8u64)) as u64) as u64; bb = 41; continue; }
            41 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (r14) as u64); r14 = ((P8) as u64) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 61; continue; } bb = 42; continue; }
            42 => { rdx = (crate::simd_ops::ru64(r8)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 47; continue; } bb = 43; continue; }
            43 => { rcx = (rdi) as u64; bb = 44; continue; }
            44 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r14.wrapping_add(rcx).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 47; continue; } bb = 45; continue; }
            45 => { fl_l = (rcx) as u64 as u64; fl_r = (0x7u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 44; continue; } bb = 46; continue; }
            46 => { rax = ((rcx.wrapping_add(0x2eu64)) as u32) as u32 as u64; bb = 298; continue; }
            47 => { r8 = ((rsi.wrapping_add(0x8u64)) as u64) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 61; continue; } bb = 48; continue; }
            48 => { rdx = (crate::simd_ops::ru64(r8)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 53; continue; } bb = 49; continue; }
            49 => { rcx = (rdi) as u64; r8 = ((P9) as u64) as u64; bb = 50; continue; }
            50 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r8.wrapping_add(rcx).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 53; continue; } bb = 51; continue; }
            51 => { fl_l = (rcx) as u64 as u64; fl_r = (0x7u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 50; continue; } bb = 52; continue; }
            52 => { rax = ((rcx.wrapping_add(0x32u64)) as u32) as u32 as u64; bb = 298; continue; }
            53 => { r8 = ((rsi.wrapping_add(0x8u64)) as u64) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 61; continue; } bb = 54; continue; }
            54 => { rcx = (crate::simd_ops::ru64(r8)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 57; continue; } bb = 55; continue; }
            55 => { rdx = ((P10) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 57; continue; } bb = 56; continue; }
            56 => { rax = ((rbx.wrapping_add(0x3cu64)) as u32) as u32 as u64; bb = 298; continue; }
            57 => { r8 = ((rsi.wrapping_add(0x8u64)) as u64) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 61; continue; } bb = 58; continue; }
            58 => { rcx = (crate::simd_ops::ru64(r8)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 182; continue; } bb = 59; continue; }
            59 => { rdx = ((P11) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 182; continue; } bb = 60; continue; }
            60 => { rax = ((rbx.wrapping_add(0x3du64)) as u32) as u32 as u64; bb = 298; continue; }
            61 => { rcx = (rdi) as u64; rdx = ((P12) as u64) as u64; bb = 62; continue; }
            62 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rdx.wrapping_add(rcx).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 182; continue; } bb = 63; continue; }
            63 => { fl_l = (rcx) as u64 as u64; fl_r = (0x5u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 62; continue; } bb = 64; continue; }
            64 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 181; continue; } bb = 65; continue; }
            65 => { rcx = (rdi) as u64; r8 = ((P13) as u64) as u64; bb = 66; continue; }
            66 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r8.wrapping_add(rcx).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 69; continue; } bb = 67; continue; }
            67 => { fl_l = (rcx) as u64 as u64; fl_r = (0x4u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 66; continue; } bb = 68; continue; }
            68 => { rax = ((rcx.wrapping_sub(0x2u64)) as u32) as u32 as u64; bb = 298; continue; }
            69 => { rcx = (rdi) as u64; r8 = ((P14) as u64) as u64; bb = 70; continue; }
            70 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r8.wrapping_add(rcx).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 73; continue; } bb = 71; continue; }
            71 => { fl_l = (rcx) as u64 as u64; fl_r = (0x5u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 70; continue; } bb = 72; continue; }
            72 => { rax = ((rcx.wrapping_sub(0x2u64)) as u32) as u32 as u64; bb = 298; continue; }
            73 => { r8 = (rdi) as u64; r9 = ((P15) as u64) as u64; bb = 74; continue; }
            74 => { rcx = ((crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u32) as u32 as u64; r8 = ((r8).wrapping_add(1)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r9.wrapping_add(r8).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 77; continue; } bb = 75; continue; }
            75 => { fl_l = (r8) as u64 as u64; fl_r = (0x5u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 74; continue; } bb = 76; continue; }
            76 => { rax = ((r8.wrapping_sub(0x1u64)) as u32) as u32 as u64; bb = 298; continue; }
            77 => { r8 = (rdi) as u64; r9 = ((P16) as u64) as u64; bb = 78; continue; }
            78 => { rcx = ((crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u32) as u32 as u64; r8 = ((r8).wrapping_add(1)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r9.wrapping_add(r8).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 81; continue; } bb = 79; continue; }
            79 => { fl_l = (r8) as u64 as u64; fl_r = (0x6u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 78; continue; } bb = 80; continue; }
            80 => { rax = ((r8.wrapping_sub(0x1u64)) as u32) as u32 as u64; bb = 298; continue; }
            81 => { r8 = (rdi) as u64; r9 = ((P17) as u64) as u64; bb = 82; continue; }
            82 => { rcx = ((crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u32) as u32 as u64; r8 = ((r8).wrapping_add(1)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r9.wrapping_add(r8).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 85; continue; } bb = 83; continue; }
            83 => { fl_l = (r8) as u64 as u64; fl_r = (0x5u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 82; continue; } bb = 84; continue; }
            84 => { rax = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; bb = 298; continue; }
            85 => { r8 = (rdi) as u64; r9 = ((P18) as u64) as u64; bb = 86; continue; }
            86 => { rcx = ((crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u32) as u32 as u64; r8 = ((r8).wrapping_add(1)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r9.wrapping_add(r8).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 89; continue; } bb = 87; continue; }
            87 => { fl_l = (r8) as u64 as u64; fl_r = (0x5u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 86; continue; } bb = 88; continue; }
            88 => { rax = ((r8.wrapping_add(0x2u64)) as u32) as u32 as u64; bb = 298; continue; }
            89 => { rcx = (rdi) as u64; r8 = ((P19) as u64) as u64; bb = 90; continue; }
            90 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r8.wrapping_add(rcx).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 93; continue; } bb = 91; continue; }
            91 => { fl_l = (rcx) as u64 as u64; fl_r = (0x5u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 90; continue; } bb = 92; continue; }
            92 => { rax = ((rcx.wrapping_add(0x3u64)) as u32) as u32 as u64; bb = 298; continue; }
            93 => { rcx = (rdi) as u64; r8 = ((P20) as u64) as u64; bb = 94; continue; }
            94 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r8.wrapping_add(rcx).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 97; continue; } bb = 95; continue; }
            95 => { fl_l = (rcx) as u64 as u64; fl_r = (0x4u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 94; continue; } bb = 96; continue; }
            96 => { rax = ((rcx.wrapping_add(0x5u64)) as u32) as u32 as u64; bb = 298; continue; }
            97 => { rcx = (rdi) as u64; r8 = ((P21) as u64) as u64; bb = 98; continue; }
            98 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r8.wrapping_add(rcx).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 101; continue; } bb = 99; continue; }
            99 => { fl_l = (rcx) as u64 as u64; fl_r = (0x4u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 98; continue; } bb = 100; continue; }
            100 => { rax = ((rcx.wrapping_add(0x6u64)) as u32) as u32 as u64; bb = 298; continue; }
            101 => { rcx = (rdi) as u64; r8 = ((P22) as u64) as u64; bb = 102; continue; }
            102 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r8.wrapping_add(rcx).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 105; continue; } bb = 103; continue; }
            103 => { fl_l = (rcx) as u64 as u64; fl_r = (0x4u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 102; continue; } bb = 104; continue; }
            104 => { rax = ((rcx.wrapping_add(0x7u64)) as u32) as u32 as u64; bb = 298; continue; }
            105 => { rcx = (rdi) as u64; r8 = ((P23) as u64) as u64; bb = 106; continue; }
            106 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r8.wrapping_add(rcx).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 109; continue; } bb = 107; continue; }
            107 => { fl_l = (rcx) as u64 as u64; fl_r = (0x6u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 106; continue; } bb = 108; continue; }
            108 => { rax = ((rcx.wrapping_add(0x6u64)) as u32) as u32 as u64; bb = 298; continue; }
            109 => { rcx = (rdi) as u64; r8 = ((P24) as u64) as u64; bb = 110; continue; }
            110 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r8.wrapping_add(rcx).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 113; continue; } bb = 111; continue; }
            111 => { fl_l = (rcx) as u64 as u64; fl_r = (0x5u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 110; continue; } bb = 112; continue; }
            112 => { rax = ((rcx.wrapping_add(0x8u64)) as u32) as u32 as u64; bb = 298; continue; }
            113 => { rcx = (rdi) as u64; r8 = ((P25) as u64) as u64; bb = 114; continue; }
            114 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r8.wrapping_add(rcx).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 117; continue; } bb = 115; continue; }
            115 => { fl_l = (rcx) as u64 as u64; fl_r = (0x6u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 114; continue; } bb = 116; continue; }
            116 => { rax = ((rcx.wrapping_add(0x8u64)) as u32) as u32 as u64; bb = 298; continue; }
            117 => { rcx = (rdi) as u64; r8 = ((P26) as u64) as u64; bb = 118; continue; }
            118 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r8.wrapping_add(rcx).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 121; continue; } bb = 119; continue; }
            119 => { fl_l = (rcx) as u64 as u64; fl_r = (0x6u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 118; continue; } bb = 120; continue; }
            120 => { rax = ((rcx.wrapping_add(0x9u64)) as u32) as u32 as u64; bb = 298; continue; }
            121 => { rcx = (rdi) as u64; r8 = ((P27) as u64) as u64; bb = 122; continue; }
            122 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r8.wrapping_add(rcx).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 125; continue; } bb = 123; continue; }
            123 => { fl_l = (rcx) as u64 as u64; fl_r = (0x6u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 122; continue; } bb = 124; continue; }
            124 => { rax = ((rcx.wrapping_add(0xau64)) as u32) as u32 as u64; bb = 298; continue; }
            125 => { rcx = (rdi) as u64; r8 = ((P28) as u64) as u64; bb = 126; continue; }
            126 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r8.wrapping_add(rcx).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 129; continue; } bb = 127; continue; }
            127 => { fl_l = (rcx) as u64 as u64; fl_r = (0x4u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 126; continue; } bb = 128; continue; }
            128 => { rax = ((rcx.wrapping_add(0xdu64)) as u32) as u32 as u64; bb = 298; continue; }
            129 => { rcx = (rdi) as u64; r8 = ((P29) as u64) as u64; bb = 130; continue; }
            130 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r8.wrapping_add(rcx).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 133; continue; } bb = 131; continue; }
            131 => { fl_l = (rcx) as u64 as u64; fl_r = (0x4u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 130; continue; } bb = 132; continue; }
            132 => { rax = ((rcx.wrapping_add(0xeu64)) as u32) as u32 as u64; bb = 298; continue; }
            133 => { rcx = (rdi) as u64; r8 = ((P30) as u64) as u64; bb = 134; continue; }
            134 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r8.wrapping_add(rcx).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 137; continue; } bb = 135; continue; }
            135 => { fl_l = (rcx) as u64 as u64; fl_r = (0x4u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 134; continue; } bb = 136; continue; }
            136 => { rax = ((rcx.wrapping_add(0xfu64)) as u32) as u32 as u64; bb = 298; continue; }
            137 => { rcx = (rdi) as u64; r8 = ((P31) as u64) as u64; bb = 138; continue; }
            138 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r8.wrapping_add(rcx).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 141; continue; } bb = 139; continue; }
            139 => { fl_l = (rcx) as u64 as u64; fl_r = (0x5u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 138; continue; } bb = 140; continue; }
            140 => { rax = ((rcx.wrapping_add(0xfu64)) as u32) as u32 as u64; bb = 298; continue; }
            141 => { rcx = (rdi) as u64; r8 = ((P32) as u64) as u64; bb = 142; continue; }
            142 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r8.wrapping_add(rcx).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 145; continue; } bb = 143; continue; }
            143 => { fl_l = (rcx) as u64 as u64; fl_r = (0x4u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 142; continue; } bb = 144; continue; }
            144 => { rax = ((rcx.wrapping_add(0x11u64)) as u32) as u32 as u64; bb = 298; continue; }
            145 => { rcx = (rdi) as u64; r8 = ((P33) as u64) as u64; bb = 146; continue; }
            146 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r8.wrapping_add(rcx).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 149; continue; } bb = 147; continue; }
            147 => { fl_l = (rcx) as u64 as u64; fl_r = (0x4u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 146; continue; } bb = 148; continue; }
            148 => { rax = ((rcx.wrapping_add(0x12u64)) as u32) as u32 as u64; bb = 298; continue; }
            149 => { rcx = (rdi) as u64; r8 = ((P34) as u64) as u64; bb = 150; continue; }
            150 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r8.wrapping_add(rcx).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 153; continue; } bb = 151; continue; }
            151 => { fl_l = (rcx) as u64 as u64; fl_r = (0x5u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 150; continue; } bb = 152; continue; }
            152 => { rax = ((rcx.wrapping_add(0x12u64)) as u32) as u32 as u64; bb = 298; continue; }
            153 => { rcx = (rdi) as u64; r8 = ((P35) as u64) as u64; bb = 154; continue; }
            154 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r8.wrapping_add(rcx).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 157; continue; } bb = 155; continue; }
            155 => { fl_l = (rcx) as u64 as u64; fl_r = (0x4u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 154; continue; } bb = 156; continue; }
            156 => { rax = ((rcx.wrapping_add(0x14u64)) as u32) as u32 as u64; bb = 298; continue; }
            157 => { rcx = (rdi) as u64; r8 = ((P36) as u64) as u64; bb = 158; continue; }
            158 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r8.wrapping_add(rcx).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 161; continue; } bb = 159; continue; }
            159 => { fl_l = (rcx) as u64 as u64; fl_r = (0x5u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 158; continue; } bb = 160; continue; }
            160 => { rax = ((rcx.wrapping_add(0x14u64)) as u32) as u32 as u64; bb = 298; continue; }
            161 => { rcx = (rdi) as u64; r8 = ((P37) as u64) as u64; bb = 162; continue; }
            162 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r8.wrapping_add(rcx).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 165; continue; } bb = 163; continue; }
            163 => { fl_l = (rcx) as u64 as u64; fl_r = (0x5u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 162; continue; } bb = 164; continue; }
            164 => { rax = ((rcx.wrapping_add(0x15u64)) as u32) as u32 as u64; bb = 298; continue; }
            165 => { rcx = (rdi) as u64; r8 = ((P38) as u64) as u64; bb = 166; continue; }
            166 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r8.wrapping_add(rcx).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 169; continue; } bb = 167; continue; }
            167 => { fl_l = (rcx) as u64 as u64; fl_r = (0x4u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 166; continue; } bb = 168; continue; }
            168 => { rax = ((rcx.wrapping_add(0x17u64)) as u32) as u32 as u64; bb = 298; continue; }
            169 => { rcx = (rdi) as u64; r8 = ((P39) as u64) as u64; bb = 170; continue; }
            170 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r8.wrapping_add(rcx).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 173; continue; } bb = 171; continue; }
            171 => { fl_l = (rcx) as u64 as u64; fl_r = (0x6u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 170; continue; } bb = 172; continue; }
            172 => { rax = ((rcx.wrapping_add(0x28u64)) as u32) as u32 as u64; bb = 298; continue; }
            173 => { rcx = (rdi) as u64; r8 = ((P40) as u64) as u64; bb = 174; continue; }
            174 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r8.wrapping_add(rcx).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 177; continue; } bb = 175; continue; }
            175 => { fl_l = (rcx) as u64 as u64; fl_r = (0x5u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 174; continue; } bb = 176; continue; }
            176 => { rax = ((rcx.wrapping_add(0x2au64)) as u32) as u32 as u64; bb = 298; continue; }
            177 => { rcx = (rdi) as u64; r8 = ((P41) as u64) as u64; bb = 178; continue; }
            178 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r8.wrapping_add(rcx).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 181; continue; } bb = 179; continue; }
            179 => { fl_l = (rcx) as u64 as u64; fl_r = (0x6u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 178; continue; } bb = 180; continue; }
            180 => { rax = ((rcx.wrapping_add(0x2au64)) as u32) as u32 as u64; bb = 298; continue; }
            181 => { rdx = (rbx) as u64; bb = 183; continue; }
            182 => { rdx = (rbx) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 244; continue; } bb = 183; continue; }
            183 => { rcx = (rdi) as u64; r8 = ((P42) as u64) as u64; bb = 184; continue; }
            184 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r8.wrapping_add(rcx).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 244; continue; } bb = 185; continue; }
            185 => { fl_l = (rcx) as u64 as u64; fl_r = (0x6u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 184; continue; } bb = 186; continue; }
            186 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 243; continue; } bb = 187; continue; }
            187 => { rcx = (rdi) as u64; r8 = ((P43) as u64) as u64; bb = 188; continue; }
            188 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r8.wrapping_add(rcx).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 191; continue; } bb = 189; continue; }
            189 => { fl_l = (rcx) as u64 as u64; fl_r = (0x8u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 188; continue; } bb = 190; continue; }
            190 => { rax = ((rcx.wrapping_add(0x14u64)) as u32) as u32 as u64; bb = 298; continue; }
            191 => { rcx = (rdi) as u64; r8 = ((P44) as u64) as u64; bb = 192; continue; }
            192 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r8.wrapping_add(rcx).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 195; continue; } bb = 193; continue; }
            193 => { fl_l = (rcx) as u64 as u64; fl_r = (0x5u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 192; continue; } bb = 194; continue; }
            194 => { rax = ((rcx.wrapping_add(0x18u64)) as u32) as u32 as u64; bb = 298; continue; }
            195 => { rcx = (rdi) as u64; r8 = ((P45) as u64) as u64; bb = 196; continue; }
            196 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r8.wrapping_add(rcx).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 199; continue; } bb = 197; continue; }
            197 => { fl_l = (rcx) as u64 as u64; fl_r = (0x5u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 196; continue; } bb = 198; continue; }
            198 => { rax = ((rcx.wrapping_add(0x19u64)) as u32) as u32 as u64; bb = 298; continue; }
            199 => { rcx = (rdi) as u64; r8 = ((P46) as u64) as u64; bb = 200; continue; }
            200 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r8.wrapping_add(rcx).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 203; continue; } bb = 201; continue; }
            201 => { fl_l = (rcx) as u64 as u64; fl_r = (0x4u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 200; continue; } bb = 202; continue; }
            202 => { rax = ((rcx.wrapping_add(0x1bu64)) as u32) as u32 as u64; bb = 298; continue; }
            203 => { rcx = (rdi) as u64; r8 = ((P47) as u64) as u64; bb = 204; continue; }
            204 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r8.wrapping_add(rcx).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 207; continue; } bb = 205; continue; }
            205 => { fl_l = (rcx) as u64 as u64; fl_r = (0x5u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 204; continue; } bb = 206; continue; }
            206 => { rax = ((rcx.wrapping_add(0x1bu64)) as u32) as u32 as u64; bb = 298; continue; }
            207 => { rcx = (rdi) as u64; r8 = ((P48) as u64) as u64; bb = 208; continue; }
            208 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r8.wrapping_add(rcx).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 211; continue; } bb = 209; continue; }
            209 => { fl_l = (rcx) as u64 as u64; fl_r = (0x6u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 208; continue; } bb = 210; continue; }
            210 => { rax = ((rcx.wrapping_add(0x1bu64)) as u32) as u32 as u64; bb = 298; continue; }
            211 => { rcx = (rdi) as u64; r8 = ((P49) as u64) as u64; bb = 212; continue; }
            212 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r8.wrapping_add(rcx).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 215; continue; } bb = 213; continue; }
            213 => { fl_l = (rcx) as u64 as u64; fl_r = (0x8u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 212; continue; } bb = 214; continue; }
            214 => { rax = ((rcx.wrapping_add(0x1au64)) as u32) as u32 as u64; bb = 298; continue; }
            215 => { rcx = (rdi) as u64; r8 = ((P50) as u64) as u64; bb = 216; continue; }
            216 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r8.wrapping_add(rcx).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 219; continue; } bb = 217; continue; }
            217 => { fl_l = (rcx) as u64 as u64; fl_r = (0x8u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 216; continue; } bb = 218; continue; }
            218 => { rax = ((rcx.wrapping_add(0x1bu64)) as u32) as u32 as u64; bb = 298; continue; }
            219 => { rcx = (rdi) as u64; r8 = ((P51) as u64) as u64; bb = 220; continue; }
            220 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r8.wrapping_add(rcx).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 223; continue; } bb = 221; continue; }
            221 => { fl_l = (rcx) as u64 as u64; fl_r = (0x7u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 220; continue; } bb = 222; continue; }
            222 => { rax = ((rcx.wrapping_add(0x1du64)) as u32) as u32 as u64; bb = 298; continue; }
            223 => { rcx = (rdi) as u64; r8 = ((P52) as u64) as u64; bb = 224; continue; }
            224 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r8.wrapping_add(rcx).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 227; continue; } bb = 225; continue; }
            225 => { fl_l = (rcx) as u64 as u64; fl_r = (0x8u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 224; continue; } bb = 226; continue; }
            226 => { rax = ((rcx.wrapping_add(0x1du64)) as u32) as u32 as u64; bb = 298; continue; }
            227 => { rcx = (rdi) as u64; r8 = ((P53) as u64) as u64; bb = 228; continue; }
            228 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r8.wrapping_add(rcx).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 231; continue; } bb = 229; continue; }
            229 => { fl_l = (rcx) as u64 as u64; fl_r = (0x8u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 228; continue; } bb = 230; continue; }
            230 => { rax = ((rcx.wrapping_add(0x1eu64)) as u32) as u32 as u64; bb = 298; continue; }
            231 => { rcx = (rdi) as u64; r8 = ((P54) as u64) as u64; bb = 232; continue; }
            232 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r8.wrapping_add(rcx).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 235; continue; } bb = 233; continue; }
            233 => { fl_l = (rcx) as u64 as u64; fl_r = (0x7u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 232; continue; } bb = 234; continue; }
            234 => { rax = ((rcx.wrapping_add(0x20u64)) as u32) as u32 as u64; bb = 298; continue; }
            235 => { rcx = (rdi) as u64; r8 = ((P55) as u64) as u64; bb = 236; continue; }
            236 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r8.wrapping_add(rcx).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 239; continue; } bb = 237; continue; }
            237 => { fl_l = (rcx) as u64 as u64; fl_r = (0x8u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 236; continue; } bb = 238; continue; }
            238 => { rax = ((rcx.wrapping_add(0x2fu64)) as u32) as u32 as u64; bb = 298; continue; }
            239 => { rcx = (rdi) as u64; r8 = ((P56) as u64) as u64; bb = 240; continue; }
            240 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r8.wrapping_add(rcx).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 243; continue; } bb = 241; continue; }
            241 => { fl_l = (rcx) as u64 as u64; fl_r = (0x8u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 240; continue; } bb = 242; continue; }
            242 => { rax = ((rcx.wrapping_add(0x30u64)) as u32) as u32 as u64; bb = 298; continue; }
            243 => { rdx = (rbx) as u64; bb = 245; continue; }
            244 => { rdx = (rbx) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 266; continue; } bb = 245; continue; }
            245 => { rcx = (rdi) as u64; r8 = ((P57) as u64) as u64; bb = 246; continue; }
            246 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r8.wrapping_add(rcx).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 266; continue; } bb = 247; continue; }
            247 => { fl_l = (rcx) as u64 as u64; fl_r = (0x7u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 246; continue; } bb = 248; continue; }
            248 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 265; continue; } bb = 249; continue; }
            249 => { rcx = (rdi) as u64; r8 = ((P58) as u64) as u64; bb = 250; continue; }
            250 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r8.wrapping_add(rcx).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 253; continue; } bb = 251; continue; }
            251 => { fl_l = (rcx) as u64 as u64; fl_r = (0x5u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 250; continue; } bb = 252; continue; }
            252 => { rax = ((rcx.wrapping_add(0x24u64)) as u32) as u32 as u64; bb = 298; continue; }
            253 => { rcx = (rdi) as u64; r8 = ((P59) as u64) as u64; bb = 254; continue; }
            254 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r8.wrapping_add(rcx).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 257; continue; } bb = 255; continue; }
            255 => { fl_l = (rcx) as u64 as u64; fl_r = (0x5u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 254; continue; } bb = 256; continue; }
            256 => { rax = ((rcx.wrapping_add(0x25u64)) as u32) as u32 as u64; bb = 298; continue; }
            257 => { rcx = (rdi) as u64; r8 = ((P60) as u64) as u64; bb = 258; continue; }
            258 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r8.wrapping_add(rcx).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 261; continue; } bb = 259; continue; }
            259 => { fl_l = (rcx) as u64 as u64; fl_r = (0x4u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 258; continue; } bb = 260; continue; }
            260 => { rax = ((rcx.wrapping_add(0x27u64)) as u32) as u32 as u64; bb = 298; continue; }
            261 => { rcx = (rdi) as u64; r8 = ((P61) as u64) as u64; bb = 262; continue; }
            262 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r8.wrapping_add(rcx).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 265; continue; } bb = 263; continue; }
            263 => { fl_l = (rcx) as u64 as u64; fl_r = (0x4u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 262; continue; } bb = 264; continue; }
            264 => { rax = ((rcx.wrapping_add(0x29u64)) as u32) as u32 as u64; bb = 298; continue; }
            265 => { rdx = (rbx) as u64; bb = 267; continue; }
            266 => { rdx = (rbx) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 279; continue; } bb = 267; continue; }
            267 => { rcx = (rdi) as u64; r8 = ((P62) as u64) as u64; bb = 268; continue; }
            268 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r8.wrapping_add(rcx).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 279; continue; } bb = 269; continue; }
            269 => { fl_l = (rcx) as u64 as u64; fl_r = (0x6u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 268; continue; } bb = 270; continue; }
            270 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 279; continue; } bb = 271; continue; }
            271 => { rcx = (rdi) as u64; r8 = ((P63) as u64) as u64; bb = 272; continue; }
            272 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r8.wrapping_add(rcx).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 276; continue; } bb = 273; continue; }
            273 => { fl_l = (rcx) as u64 as u64; fl_r = (0x7u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 272; continue; } bb = 274; continue; }
            274 => { rax = ((rcx.wrapping_add(0x2du64)) as u32) as u32 as u64; bb = 298; continue; }
            275 => { bb = 276; continue; }
            276 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rdi))) as u32) as u32 as u64; rdi = ((rdi).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r14.wrapping_add(rdi).wrapping_sub(0x1u64))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 279; continue; } bb = 277; continue; }
            277 => { fl_l = (rdi) as u64 as u64; fl_r = (0x7u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 276; continue; } bb = 278; continue; }
            278 => { rax = ((rdi.wrapping_add(0x2eu64)) as u32) as u32 as u64; bb = 298; continue; }
            279 => { r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 297; continue; } bb = 280; continue; }
            280 => { r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; rax = (rbx) as u64; fl_l = (r9) as u64 as u64; fl_r = (r9) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 291; continue; } bb = 281; continue; }
            281 => { fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 297; continue; } bb = 282; continue; }
            282 => { fl_l = (r9) as u64 as u64; fl_r = (rbx) as u64 as u64; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 283; continue; }
            283 => { rdx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru8(rax.wrapping_add(r9))) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 285; continue; } bb = 284; continue; }
            284 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 283; continue; } bb = 285; continue; }
            285 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 297; continue; } bb = 286; continue; }
            286 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 297; continue; } bb = 287; continue; }
            287 => { fl_l = (r8) as u64 as u64; fl_r = (rax) as u64 as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 288; continue; }
            288 => { rdx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru8(rax.wrapping_add(r8))) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 296; continue; } bb = 289; continue; }
            289 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 288; continue; } bb = 290; continue; }
            290 => { bb = 296; continue; }
            291 => { fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 297; continue; } bb = 292; continue; }
            292 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 297; continue; } bb = 293; continue; }
            293 => { fl_l = (r8) as u64 as u64; fl_r = (rax) as u64 as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 294; continue; }
            294 => { rdx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru8(rax.wrapping_add(r8))) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 296; continue; } bb = 295; continue; }
            295 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 294; continue; } bb = 296; continue; }
            296 => { rax = (0x36u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 298; continue; } bb = 297; continue; }
            297 => { rax = (0xffffffffu32) as u32 as u64; bb = 298; continue; }
            298 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 299; continue; }
            299 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425311c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143d01600; const P1: u64 = 0x141480c60; const P2: u64 = 0x1425311c0; const P3: u64 = 0x143d01590; const P4: u64 = 0x143d01610; const P5: u64 = 0x141473df0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; r14 = 0; fl_l = 0; fl_r = 0; fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P0)) as u32 as u64; rbx = rcx; rcx = ((r14 as u32)) as u32 as u64; rdi = r9; rcx = (if (((fl_l as u32)) == ((fl_r as u32))) { rdx } else { rcx }) as u64; rbp = r8; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rdx = rcx.wrapping_add(0x20); rcx = r9; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; rcx = rax.wrapping_add(0x8); rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r14 } else { rcx }) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x8u64))) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { r9 = rdi; rcx = rsp.wrapping_add(0x20); r8 = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rax)) as u128; rax = rsp.wrapping_add(0x20); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); bb = 13; continue; }
            5 => { xmm0 = 0u128; rax = rsp.wrapping_add(0x20); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); bb = 13; continue; }
            6 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P3)) as u32 as u64; rdi = r14; rdi = (if (((fl_l as u32)) == ((fl_r as u32))) { rdx } else { rdi }) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru32(P4)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); rsi = r14; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rsi = (if (((fl_l as u32)) == ((fl_r as u32))) { rcx } else { rsi }) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); bb = 11; continue; }
            10 => { xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); bb = 11; continue; }
            11 => { rax = rsp.wrapping_add(0x20); xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); bb = 12; continue; }
            12 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 13; continue; }
            13 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            14 => { crate::simd_ops::wu64(rbx, r14); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), r14); bb = 12; continue; }
            15 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P4)) as u32 as u64; rdi = r14; rdi = (if (((fl_l as u32)) == ((fl_r as u32))) { rdx } else { rdi }) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 16; continue; }
            16 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rax = rsp.wrapping_add(0x20); if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdx); xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); bb = 13; continue; }
            18 => { xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); bb = 13; continue; }
            19 => { crate::simd_ops::wu64(rbx, r14); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), r14); bb = 13; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142531360(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143424b18;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x58u64)) as u64;
    rax = P0;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r9);
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rdx = rsp.wrapping_add(0x20);
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x58u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425313a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    r8 = rcx;
    r9 = rcx;
    r10 = (0x7f007f007f007fu64) as u64;
    rax = 0x7f;
    fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    rcx = (0x7f807f807f807f80u64) as u64;
    rax = (if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { (rdx as u32) } else { (rax as u32) }) as u32 as u64;
    r9 = ((r9).wrapping_shr(8u32)) as u64;
    rdx = ((((rax as u32)) as i32 as i64) as u64) as u64;
    r9 = (((r9) & (r10))) as u64;
    r9 = (((r9) as i64).wrapping_mul((rdx) as i64) as u64) as u64;
    r8 = (((r8) & (r10))) as u64;
    rax = (0x80008000800080u64) as u64;
    r8 = (((r8) as i64).wrapping_mul((rdx) as i64) as u64) as u64;
    rdx = ((r8.wrapping_add(rcx)) as u64) as u64;
    r8 = (((r8) & (r10))) as u64;
    rdx = ((rdx).wrapping_shr(8u32)) as u64;
    rdx = (((rdx) & (rax))) as u64;
    rax = ((r9.wrapping_add(rcx)) as u64) as u64;
    rcx = (0x8000800080008000u64) as u64;
    r9 = (((r9) & (r10))) as u64;
    rax = (((rax) & (rcx))) as u64;
    r9 = ((r9).wrapping_shl(8u32)) as u64;
    rdx = (((rdx) | (rax))) as u64;
    rcx = rdx;
    rcx = ((rcx).wrapping_shr(7u32)) as u64;
    rdx = ((rdx).wrapping_sub(rcx)) as u64;
    rdx = (((rdx) | (r9))) as u64;
    fl_l = (((rdx) | (r8))) as u64; fl_r = fl_l;
    rdx = ((fl_l as u64)) as u64;
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142531430(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143427108; const P1: u64 = 0x143426d98;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { xmm5 = ((crate::simd_ops::ru64(P0)) as u64) as u128; fl_l = ((xmm0 as u64)) as u64; fl_r = ((xmm5 as u64)) as u64; xmm4 = ((crate::simd_ops::ru64(P1)) as u64) as u128; xmm3 = (xmm2) as u128; if !(f64::from_bits(fl_l) >= f64::from_bits(fl_r)) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = ((xmm4 as u64)) as u64; fl_r = ((xmm0 as u64)) as u64; if !(f64::from_bits(fl_l) >= f64::from_bits(fl_r)) { bb = 4; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::cvt_f64_i32((f64::from_bits(xmm0 as u64)))) as u32 as u64; xmm2 = 0u128; xmm2 = (xmm2 & !0xffff_ffff_ffff_ffffu128) | (((((rdx as u32)) as i32) as f64).to_bits() as u128); fl_l = ((xmm2 as u64)) as u64; fl_r = ((xmm0 as u64)) as u64; if (f64::from_bits(fl_l).is_nan() || f64::from_bits(fl_r).is_nan()) { bb = 4; continue; } bb = 3; continue; }
            3 => { if (f64::from_bits(fl_l) == f64::from_bits(fl_r) || f64::from_bits(fl_l).is_nan() || f64::from_bits(fl_r).is_nan()) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = 0x80000000; bb = 5; continue; }
            5 => { fl_l = ((xmm1 as u64)) as u64; fl_r = ((xmm5 as u64)) as u64; if !(f64::from_bits(fl_l) >= f64::from_bits(fl_r)) { bb = 9; continue; } bb = 6; continue; }
            6 => { fl_l = ((xmm4 as u64)) as u64; fl_r = ((xmm1 as u64)) as u64; if !(f64::from_bits(fl_l) >= f64::from_bits(fl_r)) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::cvt_f64_i32((f64::from_bits(xmm1 as u64)))) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | (((((rax as u32)) as i32) as f64).to_bits() as u128); fl_l = ((xmm0 as u64)) as u64; fl_r = ((xmm1 as u64)) as u64; if (f64::from_bits(fl_l).is_nan() || f64::from_bits(fl_r).is_nan()) { bb = 9; continue; } bb = 8; continue; }
            8 => { if (f64::from_bits(fl_l) == f64::from_bits(fl_r) || f64::from_bits(fl_l).is_nan() || f64::from_bits(fl_r).is_nan()) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = 0x80000000; bb = 10; continue; }
            10 => { fl_l = ((xmm3 as u64)) as u64; fl_r = ((xmm5 as u64)) as u64; if !(f64::from_bits(fl_l) >= f64::from_bits(fl_r)) { bb = 14; continue; } bb = 11; continue; }
            11 => { fl_l = ((xmm4 as u64)) as u64; fl_r = ((xmm3 as u64)) as u64; if !(f64::from_bits(fl_l) >= f64::from_bits(fl_r)) { bb = 14; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::cvt_f64_i32((f64::from_bits(xmm3 as u64)))) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | (((((rcx as u32)) as i32) as f64).to_bits() as u128); fl_l = ((xmm0 as u64)) as u64; fl_r = ((xmm3 as u64)) as u64; if (f64::from_bits(fl_l).is_nan() || f64::from_bits(fl_r).is_nan()) { bb = 14; continue; } bb = 13; continue; }
            13 => { if (f64::from_bits(fl_l) == f64::from_bits(fl_r) || f64::from_bits(fl_l).is_nan() || f64::from_bits(fl_r).is_nan()) { bb = 15; continue; } bb = 14; continue; }
            14 => { rcx = 0x80000000; bb = 15; continue; }
            15 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x80000000; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 25; continue; } bb = 16; continue; }
            16 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x80000000; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 25; continue; } bb = 17; continue; }
            17 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7fffffff; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 25; continue; } bb = 18; continue; }
            18 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 21; continue; } bb = 19; continue; }
            19 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 23; continue; } bb = 20; continue; }
            20 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; bb = 21; continue; }
            21 => { if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 24; continue; } bb = 22; continue; }
            22 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 24; continue; } bb = 23; continue; }
            23 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            24 => { rax = (((rax as u32)).wrapping_sub((rdx as u32))) as u32 as u64; rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64; let d = ((rcx as u32)) as i32 as i64; let n = (((rdx as u32 as u64) << 32) | (rax as u32 as u64)) as i64; if d != 0 { rax = n.wrapping_div(d) as i32 as u32 as u64; rdx = n.wrapping_rem(d) as i32 as u32 as u64; } fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            25 => { rax = 0xffffffff; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142531500(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); r9 = ((rcx as u32)) as u32 as u64; r10 = r8; r9 = ((((r9 as u32)) & (0x7fu32))) as u32 as u64; rbx = rdx; r11 = rcx; fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x7f; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 1; continue; }
            1 => { r8 = 0; fl_l = 0; fl_r = 0; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { rdx = 0x8; bb = 3; continue; }
            3 => { fl_l = rdx; fl_r = (0x40u64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = ((crate::simd_ops::ru8(r8.wrapping_add(rbx))) as u32) as u32 as u64; r8 = ((r8).wrapping_add(1)) as u64; rcx = ((r11).wrapping_shr((((rdx) as u32) & 63))) as u64; rcx = ((((rcx as u32)) & (0x7fu32))) as u32 as u64; rdx = ((rdx).wrapping_add(0x8u64)) as u64; rcx = ((((rcx as u32)) as i32).wrapping_mul(((rax as u32)) as i32) as u32) as u32 as u64; r9 = (((r9 as u32)).wrapping_sub((rcx as u32))) as u32 as u64; fl_l = r8; fl_r = r10; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rax = ((r9 as u32)) as u32 as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142531560(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143424ce8; const P1: u64 = 0x141482e20; const P2: u64 = 0x1418350d0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x142531560, funcinfo_rva: 0x37faf28, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x88, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = (rsp) as u64; crate::simd_ops::wu64(r11.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(r11.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(r11.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(r11.wrapping_add(0x20u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; r15 = (r8) as u64; rsi = (rdx) as u64; r12 = (rcx) as u64; rax = ((P0) as u64) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x68u64), (rax) as u64); crate::simd_ops::wu64(r11.wrapping_sub(0x60u64), (r9) as u64); rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r11.wrapping_sub(0x58u64), (rbx) as u64); rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r11.wrapping_sub(0x50u64), (rdi) as u64); r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r11.wrapping_sub(0x48u64), (r9) as u64); r10 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r11.wrapping_sub(0x40u64), (r10) as u64); crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), (rbx) as u64); crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), (rbx) as u64); rbp = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 1; continue; }
            1 => { r14 = (0x8u32) as u32 as u64; fl_l = (r14) as u64 as u64; fl_r = (rdx) as u64 as u64; r14 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 2; continue; }
            2 => { fl_l = (rbp) as u64 as u64; fl_r = (0x7u64) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 19; continue; } bb = 3; continue; }
            3 => { rax = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shr(2u32)) as u64; fl_l = (r9) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 11; continue; } bb = 4; continue; }
            4 => { fl_l = (r9) as u64 as u64; fl_r = (r9) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 5; continue; }
            5 => { r11 = (crate::simd_ops::ru64(rsi)) as u64; fl_l = (r11) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 6; continue; }
            6 => { r9 = ((rdi.wrapping_sub(0x1u64)) as u64) as u64; rcx = (r11) as u64; rcx = ((rcx).wrapping_shr(5u32)) as u64; rcx = (((rcx) ^ (r11))) as u64; rcx = ((rcx).wrapping_shr(4u32)) as u64; rcx = (((rcx) & (r9))) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 7; continue; }
            7 => { rax = (rcx) as u64; rax = ((rax).wrapping_add(rax)) as u64; r8 = (crate::simd_ops::ru64(rbx.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_l = (r8) as u64 as u64; fl_r = (r11) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 8; continue; }
            8 => { fl_l = (r8) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = ((rcx).wrapping_add(1)) as u64; rcx = ((rcx).wrapping_add(rdx)) as u64; rcx = (((rcx) & (r9))) as u64; rdx = ((rdx).wrapping_add(1)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (r9) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } bb = 10; continue; }
            10 => { rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14253163a); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 11; continue; }
            11 => { r9 = ((rdi.wrapping_sub(0x1u64)) as u64) as u64; r11 = (crate::simd_ops::ru64(rsi)) as u64; rcx = (r11) as u64; rcx = ((rcx).wrapping_shr(5u32)) as u64; rcx = (((rcx) ^ (r11))) as u64; rcx = ((rcx).wrapping_shr(4u32)) as u64; rcx = (((rcx) & (r9))) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 12; continue; }
            12 => { rax = (rcx) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rbx)) as u64; r8 = (crate::simd_ops::ru64(rax)) as u64; fl_l = (r8) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 13; continue; }
            13 => { fl_l = (r8) as u64 as u64; fl_r = (r11) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 14; continue; }
            14 => { rcx = ((rcx).wrapping_add(1)) as u64; rcx = ((rcx).wrapping_add(rdx)) as u64; rcx = (((rcx) & (r9))) as u64; rdx = ((rdx).wrapping_add(1)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (r9) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 12; continue; } bb = 15; continue; }
            15 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 17; continue; }
            16 => { crate::simd_ops::wu64(rax, (r11) as u64); fl_l = (crate::simd_ops::lk_add_u64(rsp.wrapping_add(0x40u64), (1) as u64)) as u64; bb = 17; continue; }
            17 => { rcx = ((r14.wrapping_add(rsi)) as u64) as u64; rdx = (0xffu32) as u32 as u64; rcx = ((rdx).wrapping_shl((((rcx) as u32) & 63))) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rcx) as u64); rbp = ((rbp).wrapping_add(1)) as u64; rsi = ((rsi).wrapping_add(0x8u64)) as u64; fl_l = (rbp) as u64 as u64; fl_r = (r15) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 19; continue; } bb = 18; continue; }
            18 => { r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 2; continue; }
            19 => { rax = (crate::simd_ops::ru64(r12)) as u64; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = (r12) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1425316e7); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1425316f8); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 21; continue; }
            21 => { rax = (rbx) as u64; r11 = ((rsp.wrapping_add(0x70u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}
