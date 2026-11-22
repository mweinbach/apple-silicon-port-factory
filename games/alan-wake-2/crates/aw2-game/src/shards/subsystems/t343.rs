#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn fn_1420ca8f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x2; const P1: u64 = 0x143428760; const P2: u64 = 0x143428240;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    xmm1 = (crate::simd_ops::ru128(P1)) as u128;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm2 = (xmm1) & (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8))));
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    xmm3 = (xmm2) | (crate::simd_ops::ru128(P2));
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm3) as u128);
    rax = 0x1;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420ca930(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x2; const P1: u64 = 0x143428720;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    xmm1 = (crate::simd_ops::ru128(P1)) as u128;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm2 = (xmm1) & (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8))));
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm2) as u128);
    rax = 0x1;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420ca960(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x2;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm3 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm4 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    xmm0 = crate::simd_ops::vdivps(xmm4, xmm3);
    xmm1 = crate::simd_ops::vcvttps2dq(xmm0);
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    xmm2 = crate::simd_ops::vcvtdq2ps(xmm1);
    xmm3 = crate::simd_ops::vmulps(xmm2, xmm3);
    xmm0 = crate::simd_ops::vsubps(xmm4, xmm3);
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm0) as u128);
    rax = 0x1;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420ca9b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x2;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm3 = crate::simd_ops::vrcpps(crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8))));
    xmm0 = crate::simd_ops::vmulps(xmm3, xmm3);
    xmm2 = crate::simd_ops::vmulps(xmm0, crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8))));
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm1 = crate::simd_ops::vaddps(xmm3, xmm3);
    xmm3 = crate::simd_ops::vsubps(xmm1, xmm2);
    xmm0 = crate::simd_ops::vmulps(xmm3, crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8))));
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm0) as u128);
    rax = 0x1;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420caa00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x2;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(rax.wrapping_mul(4)))) as u128;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm1 = crate::simd_ops::vmulps(xmm0, crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8))));
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm1) as u128);
    rax = 0x1;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420caa30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x2;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm1 = crate::simd_ops::vmulps(xmm0, crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8))));
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm1) as u128);
    rax = 0x1;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420caa70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x2;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(rax.wrapping_mul(4)))) as u128;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    xmm2 = crate::simd_ops::vsubps(xmm0, xmm1);
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm2) as u128);
    rax = 0x1;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420caab0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
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
    xmm2 = crate::simd_ops::vsubps(xmm0, xmm1);
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm2) as u128);
    rax = 0x1;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420caaf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x2;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(rax.wrapping_mul(4)))) as u128;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm1 = crate::simd_ops::vaddps(xmm0, crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8))));
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm1) as u128);
    rax = 0x1;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cab20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x2;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm1 = crate::simd_ops::vaddps(xmm0, crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8))));
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm1) as u128);
    rax = 0x1;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cab90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x2;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm0 = crate::simd_ops::vcvtdq2ps(crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8))));
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm0) as u128);
    rax = 0x1;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cabc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x6; const P1: u64 = 0x4; const P2: u64 = 0x2;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rdx = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    r9 = rcx;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm1 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P2))) as u32) as u32 as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm2 = crate::simd_ops::vpcmpeqd(xmm1, crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8))));
    xmm1 = crate::simd_ops::vpcmpgtd(xmm1, crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8))));
    xmm2 = crate::simd_ops::vpblendvb(xmm1, xmm2, xmm2);
    rax = (crate::simd_ops::vmovmskps(xmm2) as u32) as u32 as u64;
    rax = ((((rax as u32)) & ((rdx as u32)))) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64;
    rax = ((crate::simd_ops::ru16(r9)) as u32) as u32 as u64;
    rcx = (rcx & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64));
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(rax.wrapping_mul(8)), ((rcx as u32)) as u32);
    rax = 0x1;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cac20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x6; const P1: u64 = 0x4; const P2: u64 = 0x2;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    r9 = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P2))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm0 = crate::simd_ops::vpcmpgtd(xmm0, crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8))));
    rax = (crate::simd_ops::vmovmskps(xmm0) as u32) as u32 as u64;
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
pub unsafe extern "win64" fn fn_1420cac70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x2; const P2: u64 = 0x6; const P3: u64 = 0x143428a00;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    r9 = rcx;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm1 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm2 = crate::simd_ops::vpcmpeqd(xmm1, crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8))));
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P2))) as u32) as u32 as u64;
    rcx = 0x0;
    xmm3 = (xmm2) ^ (crate::simd_ops::ru128(P3));
    rdx = (crate::simd_ops::vmovmskps(xmm3) as u32) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64;
    rax = ((crate::simd_ops::ru16(r9)) as u32) as u32 as u64;
    rcx = (rcx & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { 1u8 } else { 0u8 }) as u8 as u64));
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(rax.wrapping_mul(8)), ((rcx as u32)) as u32);
    rax = 0x1;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cacc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x6; const P1: u64 = 0x4; const P2: u64 = 0x2;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rdx = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    r9 = rcx;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm1 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P2))) as u32) as u32 as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm1 = crate::simd_ops::vpcmpeqd(xmm1, crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8))));
    rax = (crate::simd_ops::vmovmskps(xmm1) as u32) as u32 as u64;
    rax = ((((rax as u32)) & ((rdx as u32)))) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64;
    rax = ((crate::simd_ops::ru16(r9)) as u32) as u32 as u64;
    rcx = (rcx & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64));
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(rax.wrapping_mul(8)), ((rcx as u32)) as u32);
    rax = 0x1;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cad30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x2;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm1 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm2 = crate::simd_ops::vpmulld(xmm1, crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8))));
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm2) as u128);
    rax = 0x1;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cad70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
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
    xmm2 = crate::simd_ops::vpsubd(xmm0, xmm1);
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm2) as u128);
    rax = 0x1;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cadb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x2;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm1 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm2 = crate::simd_ops::vpaddd(xmm1, crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8))));
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm2) as u128);
    rax = 0x1;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cadf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x2;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm0 = 0u128;
    xmm1 = crate::simd_ops::vpsubd(xmm0, crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8))));
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm1) as u128);
    rax = 0x1;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cae20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x2;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm1 = crate::simd_ops::vroundps(crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8))), 0xcu8);
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    xmm2 = crate::simd_ops::vcvtps2dq(xmm1);
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm2) as u128);
    rax = 0x1;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cae50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x2;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rdx = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    rdx = ((rdx).wrapping_add(rdx)) as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    rdx = (crate::simd_ops::ru32(r8.wrapping_add(rdx.wrapping_mul(8)))) as u32 as u64;
    rdx = ((((rdx as u32)) | (crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(8)))))) as u32 as u64;
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(rax.wrapping_mul(8)), ((rdx as u32)) as u32);
    rax = 0x1;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cae80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x2;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rdx = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    rdx = ((rdx).wrapping_add(rdx)) as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    r9 = (crate::simd_ops::ru32(r8.wrapping_add(rdx.wrapping_mul(8)))) as u32 as u64;
    r9 = ((((r9 as u32)) & (crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(8)))))) as u32 as u64;
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(rax.wrapping_mul(8)), ((r9 as u32)) as u32);
    rax = 0x1;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420caeb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x2;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rdx = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    rdx = ((rdx).wrapping_add(rdx)) as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    rdx = (crate::simd_ops::ru32(r8.wrapping_add(rdx.wrapping_mul(8)))) as u32 as u64;
    rdx = ((((rdx as u32)) ^ (crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(8)))))) as u32 as u64;
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(rax.wrapping_mul(8)), ((rdx as u32)) as u32);
    rax = 0x1;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420caee0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x2;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rdx = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    rdx = ((rdx).wrapping_add(rdx)) as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    r9 = (crate::simd_ops::ru32(r8.wrapping_add(rdx.wrapping_mul(8)))) as u32 as u64;
    r9 = ((((r9 as u32)) ^ (crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(8)))))) as u32 as u64;
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    r9 = ((((r9 as u32)) ^ (0x1u32))) as u32 as u64;
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(rax.wrapping_mul(8)), ((r9 as u32)) as u32);
    rax = 0x1;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420caf10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x2;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    rdx = (crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(8)))) as u32 as u64;
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    rdx = ((((rdx as u32)) ^ (0x1u32))) as u32 as u64;
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(rax.wrapping_mul(8)), ((rdx as u32)) as u32);
    rax = 0x1;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420caf40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x2; const P1: u64 = 0x4; const P2: u64 = 0x6;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rdx = rcx;
    rax = ((rax).wrapping_add(rax)) as u64;
    fl_l = (crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(8)))) as u32 as u64; fl_r = 0x0;
    if !((((fl_l as u32)) == ((fl_r as u32)))) { rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u32) as u32 as u64; } else { rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P2))) as u32) as u32 as u64; }
    rcx = (((rax as u16)) as u32) as u32 as u64;
    rax = ((crate::simd_ops::ru16(rdx)) as u32) as u32 as u64;
    rcx = ((rcx).wrapping_add(rcx)) as u64;
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rcx.wrapping_mul(8)))) as u128;
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm0) as u128);
    rax = 0x1;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420caf80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x2; const P2: u64 = 0x6; const P3: u64 = 0x8; const P4: u64 = 0xc;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    r11 = rcx;
    r10 = ((crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    r9 = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    r10 = ((r10).wrapping_shl(4u32)) as u64;
    r10 = ((r10).wrapping_add(r8)) as u64;
    r9 = ((r9).wrapping_shl(4u32)) as u64;
    fl_l = ((r9).wrapping_add(r8)) as u64; fl_r = fl_l;
    r9 = ((fl_l as u64)) as u64;
    rdx = (crate::simd_ops::ru32(r10.wrapping_add(rax.wrapping_mul(4)))) as u32 as u64;
    crate::simd_ops::wu32(r9, ((rdx as u32)) as u32);
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P2))) as u32) as u32 as u64;
    rcx = (crate::simd_ops::ru32(r10.wrapping_add(rax.wrapping_mul(4)))) as u32 as u64;
    crate::simd_ops::wu32(r9.wrapping_add(P0), ((rcx as u32)) as u32);
    rax = ((crate::simd_ops::ru16(r11.wrapping_add(0x8u64))) as u32) as u32 as u64;
    rcx = (crate::simd_ops::ru32(r10.wrapping_add(rax.wrapping_mul(4)))) as u32 as u64;
    crate::simd_ops::wu32(r9.wrapping_add(P3), ((rcx as u32)) as u32);
    rax = ((crate::simd_ops::ru16(r11.wrapping_add(0xau64))) as u32) as u32 as u64;
    rcx = (crate::simd_ops::ru32(r10.wrapping_add(rax.wrapping_mul(4)))) as u32 as u64;
    rax = 0x1;
    crate::simd_ops::wu32(r9.wrapping_add(P4), ((rcx as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cb070(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x2;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm0 = crate::simd_ops::vbroadcastss(crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(8))));
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm0) as u128);
    rax = 0x1;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cb0a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x2; const P1: u64 = 0x4; const P2: u64 = 0x6; const P3: u64 = 0x8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    r11 = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    r10 = rcx;
    r9 = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rdx = ((crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    r9 = ((r9).wrapping_add(r9)) as u64;
    r11 = ((r11).wrapping_shl(4u32)) as u64;
    fl_l = ((r11).wrapping_add(r8)) as u64; fl_r = fl_l;
    r11 = ((fl_l as u64)) as u64;
    rax = (crate::simd_ops::ru32(r8.wrapping_add(r9.wrapping_mul(8)))) as u32 as u64;
    crate::simd_ops::wu32(r11.wrapping_add(rdx.wrapping_mul(4)), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(r9.wrapping_mul(8)).wrapping_add(P1))) as u32 as u64;
    rcx = ((crate::simd_ops::ru16(rcx.wrapping_add(P2))) as u32) as u32 as u64;
    crate::simd_ops::wu32(r11.wrapping_add(rcx.wrapping_mul(4)), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(r9.wrapping_mul(8)).wrapping_add(P3))) as u32 as u64;
    rcx = ((crate::simd_ops::ru16(r10.wrapping_add(P3))) as u32) as u32 as u64;
    crate::simd_ops::wu32(r11.wrapping_add(rcx.wrapping_mul(4)), ((rax as u32)) as u32);
    rax = 0x1;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cb0f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x2; const P1: u64 = 0x4; const P2: u64 = 0x6;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    r10 = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    r9 = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rdx = ((crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    r9 = ((r9).wrapping_add(r9)) as u64;
    r10 = ((r10).wrapping_shl(4u32)) as u64;
    fl_l = ((r10).wrapping_add(r8)) as u64; fl_r = fl_l;
    r10 = ((fl_l as u64)) as u64;
    rax = (crate::simd_ops::ru32(r8.wrapping_add(r9.wrapping_mul(8)))) as u32 as u64;
    crate::simd_ops::wu32(r10.wrapping_add(rdx.wrapping_mul(4)), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(r9.wrapping_mul(8)).wrapping_add(P1))) as u32 as u64;
    rcx = ((crate::simd_ops::ru16(rcx.wrapping_add(P2))) as u32) as u32 as u64;
    crate::simd_ops::wu32(r10.wrapping_add(rcx.wrapping_mul(4)), ((rax as u32)) as u32);
    rax = 0x1;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cb1c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x2;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rcx = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    fl_l = ((rcx).wrapping_add(rcx)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    rax = (crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(8)))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(rcx.wrapping_mul(8)), ((rax as u32)) as u32);
    rax = 0x1;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cb1f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x2; const P1: u64 = 0x4; const P2: u64 = 0x6; const P3: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rdx = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    rdx = ((rdx).wrapping_add(rdx)) as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rdx.wrapping_mul(8)))) as u128;
    xmm2 = crate::simd_ops::vshufps(xmm0, crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8))), 0x0u8);
    rdx = ((crate::simd_ops::ru16(rcx.wrapping_add(P2))) as u32) as u32 as u64;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P3))) as u32) as u32 as u64;
    rdx = ((rdx).wrapping_add(rdx)) as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rdx.wrapping_mul(8)))) as u128;
    xmm1 = crate::simd_ops::vshufps(xmm0, crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8))), 0x0u8);
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    xmm2 = crate::simd_ops::vshufps(xmm2, xmm1, 0x88u8);
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm2) as u128);
    rax = 0x1;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cb250(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x2; const P1: u64 = 0x4; const P2: u64 = 0x6;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rdx = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    rdx = ((rdx).wrapping_add(rdx)) as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rdx.wrapping_mul(8)))) as u128;
    xmm1 = crate::simd_ops::vshufps(xmm0, crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8))), 0x0u8);
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P2))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm0 = crate::simd_ops::vpermilps(crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8))), 0x0u8);
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    xmm2 = crate::simd_ops::vshufps(xmm1, xmm0, 0x88u8);
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm2) as u128);
    rax = 0x1;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cb2a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x2; const P1: u64 = 0x4;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rdx = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    rdx = ((rdx).wrapping_add(rdx)) as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rdx.wrapping_mul(8)))) as u128;
    xmm1 = crate::simd_ops::vshufps(xmm0, crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8))), 0x0u8);
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    xmm2 = crate::simd_ops::vpermilps(xmm1, 0x88u8);
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm2) as u128);
    rax = 0x1;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cb2e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x2;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    r9 = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    fl_l = ((r9).wrapping_add(r9)) as u64; fl_r = fl_l;
    r9 = ((fl_l as u64)) as u64;
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(rax.wrapping_mul(4)))) as u128;
    rax = 0x1;
    crate::simd_ops::wu128(r8.wrapping_add(r9.wrapping_mul(8)), (xmm0) as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cb310(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x2;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    xmm0 = crate::simd_ops::vpbroadcastd(crate::simd_ops::ru32(rdx.wrapping_add(rax.wrapping_mul(4))));
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm0) as u128);
    rax = 0x1;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cb3a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x2; const P1: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    rdx = (crate::simd_ops::ru64(r9.wrapping_add(rax.wrapping_mul(8)))) as u64;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    crate::simd_ops::wu64(rdx, ((xmm0 as u64)) as u64);
    xmm0 = crate::simd_ops::vpermilps(xmm0, 0xaau8);
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((xmm0 as u32)) as u32);
    rax = 0x1;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cb450(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x2; const P1: u64 = 0x4; const P2: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    rdx = (crate::simd_ops::ru64(r9.wrapping_add(rax.wrapping_mul(8)))) as u64;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    crate::simd_ops::wu32(rdx, ((xmm0 as u32)) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P1), (((xmm0 >> 32) as u32) as u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P2), (((xmm0 >> 64) as u32) as u32) as u32);
    rax = 0x1;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cb4e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x2;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    fl_l = (crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(8)))) as u32 as u64; fl_r = 0x0;
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    rdx = (rdx & !0xffu64) | (((if (((fl_l as u32)) != ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64));
    rcx = (crate::simd_ops::ru64(r9.wrapping_add(rax.wrapping_mul(8)))) as u64;
    rax = 0x1;
    crate::simd_ops::wu8(rcx, ((rdx as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cb540(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x2; const P1: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rdx = (crate::simd_ops::ru64(r9.wrapping_add(rax.wrapping_mul(8)))) as u64;
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    xmm0 = ((crate::simd_ops::ru64(rdx)) as u64) as u128;
    xmm0 = crate::simd_ops::vinsertps(xmm0, ((crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u128), 0x20u8);
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm0) as u128);
    rax = 0x1;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cb5a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x2;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rdx = (crate::simd_ops::ru64(r9.wrapping_add(rax.wrapping_mul(8)))) as u64;
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    xmm0 = crate::simd_ops::vbroadcastss(crate::simd_ops::ru32(rdx));
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm0) as u128);
    rax = 0x1;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cb600(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x2; const P1: u64 = 0x4; const P2: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    xmm0 = 0u128;
    rdx = (crate::simd_ops::ru64(r9.wrapping_add(rax.wrapping_mul(8)))) as u64;
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    xmm0 = (xmm0 & !(0xffff_ffffu128 << 0)) | ((((crate::simd_ops::ru32(rdx)) as u32) as u128) << 0);
    xmm0 = (xmm0 & !(0xffff_ffffu128 << 32)) | ((((crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32) as u128) << 32);
    xmm0 = (xmm0 & !(0xffff_ffffu128 << 64)) | ((((crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32) as u128) << 64);
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm0) as u128);
    rax = 0x1;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cb670(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x2;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rdx = (crate::simd_ops::ru64(r9.wrapping_add(rax.wrapping_mul(8)))) as u64;
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    xmm0 = crate::simd_ops::vpbroadcastd(crate::simd_ops::ru32(rdx));
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm0) as u128);
    rax = 0x1;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cb6a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x2;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rdx = (crate::simd_ops::ru64(r9.wrapping_add(rax.wrapping_mul(8)))) as u64;
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    r9 = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(rax.wrapping_mul(8)), ((r9 as u32)) as u32);
    rax = 0x1;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cb720(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x2; const P1: u64 = 0x4;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    fl_l = (crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(8)))) as u32 as u64; fl_r = 0x0;
    rax = 0x1;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u32) as u32 as u64; }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cb740(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x2; const P1: u64 = 0x4;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    fl_l = (crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(8)))) as u32 as u64; fl_r = 0x0;
    if !((((fl_l as u32)) == ((fl_r as u32)))) { rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u32) as u32 as u64; return rax; }
    rax = 0x1;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cb8c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x18; const P2: u64 = 0x20; const P3: u64 = 0x1405eb9e0; const P4: u64 = 0x142646840; const P5: u64 = 0x38; const P6: u64 = 0x140571d70;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 137>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(P0), rdx); crate::simd_ops::wu64(rax.wrapping_add(P1), r8); crate::simd_ops::wu64(rax.wrapping_add(P2), r9); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x430u64)) as u64; rbx = rdx; rsi = ((rax.wrapping_add(P1)) as u64) as u64; rdi = rcx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); rdx = rsp.wrapping_add(0x30); r9 = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (0x0u64) as u64); r8 = 0x400; rcx = (crate::simd_ops::ru64(rax)) as u64; rcx = (((rcx) | (0x2u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rax = rsp.wrapping_add(0x30); r8 = (0xffffffffffffffffu64) as u64; bb = 1; continue; }
            1 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r8))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rcx = ((rdi.wrapping_add(P5)) as u64) as u64; rdx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x430u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cb950(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646870; const P1: u64 = 0x1405eb9e0; const P2: u64 = 0x142646868; const P3: u64 = 0x142646898;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rcx = 0x1;
    rdi = rsp.wrapping_add(0x60);
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    rbx = rax;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64;
    r9 = 0;
    fl_l = 0; fl_r = 0;
    rdx = rbx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi);
    rcx = (crate::simd_ops::ru64(rax)) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp);
    rcx = 0x1;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    rcx = rax;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cb9c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1433ddef8; const P1: u64 = 0x140203690; const P2: u64 = 0x38; const P3: u64 = 0x10; const P4: u64 = 0x142645738; const P5: u64 = 0x1433ddf30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = P0;
    rbx = rcx;
    crate::simd_ops::wu64(rcx, rax);
    rcx = ((rcx).wrapping_add(0x58u64)) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    r8 = 0x8000;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = P5;
    crate::simd_ops::wu64(rbx, rax);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cbad0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645738;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    r8 = 0x8000;
    return crate::simd_ops::call_fwd(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cbbe0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1429822c0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x100;
    if (((fl_l as u32)) >= ((fl_r as u32))) { rax = 0; fl_l = 0; fl_r = 0; return rax; }
    rax = ((rcx as u32)) as u32 as u64;
    rcx = P0;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)))) as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xfffffffe;
    if !((((fl_l as u32)) == ((fl_r as u32)))) { fl_l = ((rax).wrapping_shr(32u32)) as u64; rax = ((fl_l as u64)) as u64; return rax; }
    rax = 0;
    fl_l = 0; fl_r = 0;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cbc10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645790; const P1: u64 = 0x1426455a8; const P2: u64 = 0x140000000; const P3: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (((r8 as u16)) as u32) as u32 as u64; rsi = rdx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x40u64))) as u32 as u64; rbp = ((rax as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = rsi.wrapping_add(0x38); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rsi.wrapping_add(0x40u64), ((rbp as u32)) as u32); bb = 2; continue; }
            2 => { crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x44u64), (1) as u32); rax = ((rdi.wrapping_sub(0x83u64)) as u32) as u32 as u64; rdx = ((rdi as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x10; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 21; continue; } bb = 3; continue; }
            3 => { r8 = P2; rax = (rax as u32 as i32 as i64) as u64; rcx = (crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x20cbe70u64))) as u32 as u64; fl_l = ((rcx).wrapping_add(r8)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; j_tgt = rcx; bb = 0xffff_fffe; continue; }
            4 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x138u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rcx = ((rsi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x89; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 5; continue; }
            5 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 6; continue; }
            6 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 7; continue; }
            7 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = 0x24; crate::simd_ops::wu8(rcx.wrapping_add(rax).wrapping_add(0x48u64), (0x0u8) as u8); bb = 21; continue; }
            9 => { rax = 0x23; crate::simd_ops::wu8(rcx.wrapping_add(rax).wrapping_add(0x48u64), (0x0u8) as u8); bb = 21; continue; }
            10 => { rax = 0x22; crate::simd_ops::wu8(rcx.wrapping_add(rax).wrapping_add(0x48u64), (0x0u8) as u8); bb = 21; continue; }
            11 => { rax = 0x21; crate::simd_ops::wu8(rcx.wrapping_add(rax).wrapping_add(0x48u64), (0x0u8) as u8); bb = 21; continue; }
            12 => { rax = 0x20; crate::simd_ops::wu8(rcx.wrapping_add(rax).wrapping_add(0x48u64), (0x0u8) as u8); bb = 21; continue; }
            13 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x138u64))) as u64; rax = rcx; rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = rcx.wrapping_add(0x1); rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x48u64), rax); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x138u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; crate::simd_ops::wu64(rsi.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x50u64), rcx); bb = 21; continue; }
            14 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x138u64))) as u64; rax = rcx; rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = rcx.wrapping_add(0x1); rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x48u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x48u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x138u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; crate::simd_ops::wu64(rsi.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x50u64), rcx); bb = 21; continue; }
            15 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x138u64))) as u64; rax = rcx; rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = rcx.wrapping_add(0x1); rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x4cu64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x4cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x138u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; crate::simd_ops::wu64(rsi.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x50u64), rcx); bb = 21; continue; }
            16 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x138u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; crate::simd_ops::wu64(rsi.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x58u64), rcx); bb = 21; continue; }
            17 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x138u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; crate::simd_ops::wu32(rsi.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x58u64), ((rcx as u32)) as u32); bb = 21; continue; }
            18 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x138u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; crate::simd_ops::wu32(rsi.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x5cu64), ((rcx as u32)) as u32); bb = 21; continue; }
            19 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x138u64))) as u64; rax = rcx; rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = rcx.wrapping_add(0x1); fl_l = ((((rax as u32)) & (0x3u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x60u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x60u64), ((rax as u32)) as u32); bb = 21; continue; }
            20 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x138u64))) as u64; rax = rcx; rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = rcx.wrapping_add(0x1); fl_l = ((((rax as u32)) & (0x3u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x64u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x64u64), ((rax as u32)) as u32); bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x44u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { rax = 0xffffffff; rcx = rsi.wrapping_add(0x38); let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rsi.wrapping_add(0x40u64), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 23; continue; }
            23 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x1420cbc10u64) { 0x2a => { bb = 1; continue; } 0x37 => { bb = 2; continue; } 0x4b => { bb = 3; continue; } 0x61 => { bb = 4; continue; } 0x7e => { bb = 5; continue; } 0x83 => { bb = 6; continue; } 0x88 => { bb = 7; continue; } 0x8d => { bb = 8; continue; } 0x9c => { bb = 9; continue; } 0xab => { bb = 10; continue; } 0xba => { bb = 11; continue; } 0xc9 => { bb = 12; continue; } 0xd8 => { bb = 13; continue; } 0x11b => { bb = 14; continue; } 0x15c => { bb = 15; continue; } 0x19d => { bb = 16; continue; } 0x1b7 => { bb = 17; continue; } 0x1d0 => { bb = 18; continue; } 0x1e9 => { bb = 19; continue; } 0x20f => { bb = 20; continue; } 0x233 => { bb = 21; continue; } 0x239 => { bb = 22; continue; } 0x24b => { bb = 23; continue; } _ => { return crate::simd_ops::tail_stk(j_tgt, rax, rcx, rdx, r8, r9, rsp); } } }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cbec0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434268d0; const P1: u64 = 0x142646120; const P2: u64 = 0x142646080; const P3: u64 = 0x142646208; const P4: u64 = 0x1420d3ab0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rax = r9; r11 = rdx; r9 = (((r8 as u16)) as u32) as u32 as u64; rbx = rcx; fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x83; r9 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 1; continue; }
            1 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x138u64))) as u64; xmm2 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x8cu64))) as u32) as u128; rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm6) as u128); xmm4 = 0u128; xmm1 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x88u64))) as u32) as u128; rax = ((((rax as u32)) & ((r9 as u32)))) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm7) as u128); xmm6 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(xmm4 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm4 as u32) }).to_bits() as u128); xmm6 = (xmm6 & !0xffff_ffffu128) | ((if f32::from_bits(xmm6 as u32) < f32::from_bits(crate::simd_ops::ru32(P0)) { f32::from_bits(xmm6 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P0)) }).to_bits() as u128); xmm7 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(xmm4 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm4 as u32) }).to_bits() as u128); xmm7 = (xmm7 & !0xffff_ffffu128) | ((if f32::from_bits(xmm7 as u32) < f32::from_bits(crate::simd_ops::ru32(P0)) { f32::from_bits(xmm7 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P0)) }).to_bits() as u128); rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm7 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((xmm6 as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x58u64), rax); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = rsp.wrapping_add(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rdx = rsp.wrapping_add(0x20); rax = (((rax as u32)).wrapping_sub(crate::simd_ops::ru32(rsp.wrapping_add(0x28u64)))) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128); xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm7 as u32)).to_bits() as u128); rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm1 as u32)))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(crate::simd_ops::ru32(rsp.wrapping_add(0x2cu64)))) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128); xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm1 as u32)))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x24u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            4 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; bb = 5; continue; }
            5 => { rsp = ((rsp).wrapping_add(0x60u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { bb = 7; continue; }
            7 => { r8 = rcx.wrapping_add(0x50); r9 = (r9 & !0xffu64) | (((0x1u8) as u8 as u64)); rcx = rsp.wrapping_add(0x20); rdx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(r11.wrapping_add(0x138u64))) as u64; rcx = ((rcx).wrapping_add(1)) as u64; rcx = ((((rcx as u32)) & (0x3u32))) as u32 as u64; xmm0 = ((crate::simd_ops::ru32(rax)) as u32) as u128; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(r11.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x48u64)))).to_bits() as u128); crate::simd_ops::wu32(r11.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x48u64), ((xmm1 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as u32) as u128; xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(r11.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x4cu64)))).to_bits() as u128); crate::simd_ops::wu32(r11.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x4cu64), ((xmm1 as u32)) as u32); rax = (crate::simd_ops::ru64(r11.wrapping_add(0x138u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; crate::simd_ops::wu64(r11.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x50u64), (0x0u64) as u64); rsp = ((rsp).wrapping_add(0x60u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cc040(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645790; const P1: u64 = 0x1426455a8; const P2: u64 = 0x140000000; const P3: u64 = 0x1420d3ab0; const P4: u64 = 0x1420d3b80; const P5: u64 = 0x1434268d0; const P6: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm6) as u128); rsi = (rdx) as u64; xmm6 = (xmm3) as u128; rdi = (((r8 as u16)) as u32) as u32 as u64; r14 = (rcx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = (crate::simd_ops::ru32(rsi.wrapping_add(0x40u64))) as u32 as u64; rbp = ((rax as u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rsi.wrapping_add(0x38u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rsi.wrapping_add(0x40u64), ((rbp as u32)) as u32); bb = 2; continue; }
            2 => { crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x44u64), (1) as u32); rax = ((rdi.wrapping_sub(0x83u64)) as u32) as u32 as u64; rdx = ((rdi as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x10u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 25; continue; } bb = 3; continue; }
            3 => { r8 = ((P2) as u64) as u64; rax = (rax as u32 as i32 as i64) as u64; rcx = (crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x20cc374u64))) as u32 as u64; fl_l = ((rcx).wrapping_add(r8)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; j_tgt = (rcx) as u64; bb = 0xffff_fffe; continue; }
            4 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x138u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rcx = ((rsi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x89u32) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 5; continue; }
            5 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 6; continue; }
            6 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 7; continue; }
            7 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = (0x24u32) as u32 as u64; crate::simd_ops::wu8(rax.wrapping_add(rcx).wrapping_add(0x48u64), (0x1u8) as u8); bb = 25; continue; }
            9 => { rax = (0x23u32) as u32 as u64; crate::simd_ops::wu8(rax.wrapping_add(rcx).wrapping_add(0x48u64), (0x1u8) as u8); bb = 25; continue; }
            10 => { rax = (0x22u32) as u32 as u64; crate::simd_ops::wu8(rax.wrapping_add(rcx).wrapping_add(0x48u64), (0x1u8) as u8); bb = 25; continue; }
            11 => { rax = (0x21u32) as u32 as u64; crate::simd_ops::wu8(rax.wrapping_add(rcx).wrapping_add(0x48u64), (0x1u8) as u8); bb = 25; continue; }
            12 => { rax = (0x20u32) as u32 as u64; crate::simd_ops::wu8(rax.wrapping_add(rcx).wrapping_add(0x48u64), (0x1u8) as u8); bb = 25; continue; }
            13 => { bb = 14; continue; }
            14 => { crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), ((xmm6 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x6cu64), ((xmm6 as u32)) as u32); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; r8 = ((r14.wrapping_add(0x50u64)) as u64) as u64; r9 = (r9 & !0xffu64) | (((0x1u8) as u8 as u64)); rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x138u64))) as u64; rcx = ((rcx).wrapping_add(1)) as u64; rcx = ((((rcx as u32)) & (0x3u32))) as u32 as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; xmm0 = ((crate::simd_ops::ru32(rsi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x48u64))) as u32) as u128; xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(core::ptr::read_unaligned((rax) as *const u32))).to_bits() as u128); crate::simd_ops::wu32(rsi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x48u64), ((xmm1 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as u32) as u128; xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(core::ptr::read_unaligned((rsi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x4cu64)) as *const u32))).to_bits() as u128); crate::simd_ops::wu32(rsi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x4cu64), ((xmm1 as u32)) as u32); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x138u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = ((((rax as u32)) & (0x3u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; crate::simd_ops::wu64(rsi.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x50u64), (0x0u64) as u64); bb = 25; continue; }
            15 => { bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x138u64))) as u64; rdx = ((r14.wrapping_add(0x50u64)) as u64) as u64; rax = ((rax).wrapping_add(1)) as u64; r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; xmm0 = (xmm6) as u128; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(core::ptr::read_unaligned((rsi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x48u64)) as *const u32))).to_bits() as u128); crate::simd_ops::wu32(rsi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x48u64), ((xmm1 as u32)) as u32); bb = 25; continue; }
            17 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x138u64))) as u64; rdx = ((r14.wrapping_add(0x5cu64)) as u64) as u64; rax = ((rax).wrapping_add(1)) as u64; r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; xmm0 = (xmm6) as u128; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(core::ptr::read_unaligned((rsi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x4cu64)) as *const u32))).to_bits() as u128); crate::simd_ops::wu32(rsi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x4cu64), ((xmm1 as u32)) as u32); bb = 25; continue; }
            18 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x138u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = ((((rax as u32)) & (0x3u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; xmm0 = 0u128; xmm3 = (xmm0 & !0xffff_ffffu128) | (xmm6 & 0xffff_ffffu128); xmm2 = 0u128; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; xmm3 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) > f32::from_bits(xmm2 as u32) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(xmm2 as u32) }).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) < f32::from_bits(core::ptr::read_unaligned((P5) as *const u32)) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(core::ptr::read_unaligned((P5) as *const u32)) }).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), ((xmm3 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x6cu64), ((xmm3 as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x58u64), (rax) as u64); bb = 25; continue; }
            19 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x138u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; xmm0 = 0u128; fl_l = ((((rax as u32)) & (0x3u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; xmm3 = (xmm0 & !0xffff_ffffu128) | (xmm6 & 0xffff_ffffu128); xmm2 = 0u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) > f32::from_bits(xmm2 as u32) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(xmm2 as u32) }).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) < f32::from_bits(core::ptr::read_unaligned((P5) as *const u32)) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(core::ptr::read_unaligned((P5) as *const u32)) }).to_bits() as u128); rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; crate::simd_ops::wu32(rsi.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x58u64), ((xmm3 as u32)) as u32); bb = 25; continue; }
            20 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x138u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; xmm0 = 0u128; fl_l = ((((rax as u32)) & (0x3u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; xmm3 = (xmm0 & !0xffff_ffffu128) | (xmm6 & 0xffff_ffffu128); xmm2 = 0u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) > f32::from_bits(xmm2 as u32) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(xmm2 as u32) }).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) < f32::from_bits(core::ptr::read_unaligned((P5) as *const u32)) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(core::ptr::read_unaligned((P5) as *const u32)) }).to_bits() as u128); rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; crate::simd_ops::wu32(rsi.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x5cu64), ((xmm3 as u32)) as u32); bb = 25; continue; }
            21 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x138u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = ((((rax as u32)) & (0x3u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; xmm0 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(core::ptr::read_unaligned((rsi.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x60u64)) as *const u32))).to_bits() as u128); crate::simd_ops::wu32(rsi.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x60u64), ((xmm0 as u32)) as u32); bb = 25; continue; }
            22 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x138u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = ((((rax as u32)) & (0x3u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; xmm0 = ((crate::simd_ops::ru32(rsi.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x60u64))) as u32) as u128; xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm6 as u32)).to_bits() as u128); crate::simd_ops::wu32(rsi.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x60u64), ((xmm1 as u32)) as u32); bb = 25; continue; }
            23 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x138u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = ((((rax as u32)) & (0x3u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; xmm0 = ((crate::simd_ops::ru32(rsi.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x64u64))) as u32) as u128; xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm6 as u32)).to_bits() as u128); crate::simd_ops::wu32(rsi.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x64u64), ((xmm1 as u32)) as u32); bb = 25; continue; }
            24 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x138u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = ((((rax as u32)) & (0x3u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; xmm0 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(core::ptr::read_unaligned((rsi.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x64u64)) as *const u32))).to_bits() as u128); crate::simd_ops::wu32(rsi.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x64u64), ((xmm0 as u32)) as u32); bb = 25; continue; }
            25 => { fl_l = (crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x44u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 27; continue; } bb = 26; continue; }
            26 => { rax = (0xffffffffu32) as u32 as u64; rcx = ((rsi.wrapping_add(0x38u64)) as u64) as u64; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rsi.wrapping_add(0x40u64), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 27; continue; }
            27 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x1420cc040u64) { 0x37 => { bb = 1; continue; } 0x44 => { bb = 2; continue; } 0x58 => { bb = 3; continue; } 0x6e => { bb = 4; continue; } 0x8b => { bb = 5; continue; } 0x90 => { bb = 6; continue; } 0x95 => { bb = 7; continue; } 0x9a => { bb = 8; continue; } 0xa9 => { bb = 9; continue; } 0xb8 => { bb = 10; continue; } 0xc7 => { bb = 11; continue; } 0xd6 => { bb = 12; continue; } 0xe5 => { bb = 13; continue; } 0xf0 => { bb = 14; continue; } 0x163 => { bb = 15; continue; } 0x170 => { bb = 16; continue; } 0x1a2 => { bb = 17; continue; } 0x1d4 => { bb = 18; continue; } 0x218 => { bb = 19; continue; } 0x24c => { bb = 20; continue; } 0x280 => { bb = 21; continue; } 0x29f => { bb = 22; continue; } 0x2c2 => { bb = 23; continue; } 0x2e5 => { bb = 24; continue; } 0x302 => { bb = 25; continue; } 0x308 => { bb = 26; continue; } 0x31a => { bb = 27; continue; } _ => { let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(j_tgt, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; } } }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cc3d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x142645790; const P2: u64 = 0x8; const P3: u64 = 0x1426455a8; const P4: u64 = 0xc; const P5: u64 = 0x1; const P6: u64 = 0x54; const P7: u64 = 0x60; const P8: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = 0x83; rsi = rcx; fl_l = ((r8 as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 6; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rbx = ((rdx.wrapping_add(P0)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; rdi = ((rax as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(P2), ((rdi as u32)) as u32); bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = ((rcx.wrapping_add(P5)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P4), ((rax as u32)) as u32); xmm2 = ((crate::simd_ops::ru32(rsi.wrapping_add(P6))) as u32) as u128; xmm6 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(crate::simd_ops::ru32(rsi.wrapping_add(P7))) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(crate::simd_ops::ru32(rsi.wrapping_add(P7))) }).to_bits() as u128); crate::simd_ops::wu32(rbx.wrapping_add(P4), ((rcx as u32)) as u32); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = 0xffffffff; rcx = rbx; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rbx.wrapping_add(P2), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; xmm0 = (xmm6) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { xmm0 = 0u128; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cc490(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143427080; const P1: u64 = 0x143879688; const P2: u64 = 0x143879698; const P3: u64 = 0x143879670; const P4: u64 = 0x142645790; const P5: u64 = 0x1426455a8; const P6: u64 = 0x142406920; const P7: u64 = 0x1420d3d40; const P8: u64 = 0x142645598;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0; let mut ymm2_hi: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(r11.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; rax = 0x83; rsi = r8; rbp = rdx; r14 = rcx; fl_l = ((r9 as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 25; continue; } bb = 1; continue; }
            1 => { rdi = ((crate::simd_ops::ru8(rsp.wrapping_add(0xe0u64))) as u32) as u32 as u64; crate::simd_ops::wu64(r11.wrapping_add(0x8u64), r15); crate::simd_ops::wu128(r11.wrapping_sub(0x28u64), (xmm6) as u128); crate::simd_ops::wu128(r11.wrapping_sub(0x38u64), (xmm7) as u128); xmm7 = ((crate::simd_ops::ru32(P0)) as u32) as u128; xmm6 = 0u128; fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x40u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { xmm0 = ((crate::simd_ops::ru32(P1)) as u32) as u128; ymm0_hi = 0; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 4; continue; } bb = 3; continue; }
            3 => { if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x4fd8u64))) as u32) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; xmm2 = ((crate::simd_ops::ru32(P2)) as u32) as u128; ymm2_hi = 0; xmm1 = (((rcx as u32)) as u32) as u128; ymm1_hi = 0; xmm0 = (((rax as u32)) as u32) as u128; ymm0_hi = 0; xmm3 = crate::simd_ops::vpcmpeqd(xmm0, xmm1); xmm1 = ((crate::simd_ops::ru32(P3)) as u32) as u128; ymm1_hi = 0; xmm0 = crate::simd_ops::vblendvps(xmm1, xmm2, xmm3); ymm0_hi = 0; xmm7 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(crate::simd_ops::ru32(P1))).to_bits() as u128); bb = 5; continue; }
            5 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x40u64))) as u32 as u64; r15 = ((rax as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = rsi.wrapping_add(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsi.wrapping_add(0x40u64), ((r15 as u32)) as u32); bb = 7; continue; }
            7 => { crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x44u64), (1) as u32); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((((fl_l as u8)) & ((fl_r as u8))) as i8) >= 0) { bb = 14; continue; } bb = 8; continue; }
            8 => { rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x4fd8u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x140u64))) as u64; rax = ((rax).wrapping_sub(1)) as u64; rax = ((((rax as u32)) & (0x1u32))) as u32 as u64; fl_l = ((rax).wrapping_add(0x4u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 11; continue; }
            10 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x138u64))) as u64; rax = ((rax).wrapping_sub(1)) as u64; fl_l = ((((rax as u32)) & (0x3u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; bb = 11; continue; }
            11 => { rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x4fd8u64))) as u32) as u32 as u64; rcx = ((rcx).wrapping_shl(3u32)) as u64; let y_addr: u64 = rsi.wrapping_add(rcx).wrapping_add(0x48u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp.wrapping_add(0x28u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x140u64))) as u32 as u64; rax = ((((rax as u32)) & (0x1u32))) as u32 as u64; fl_l = ((rax).wrapping_add(0x4u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 18; continue; }
            13 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x138u64))) as u64; fl_l = rax; fl_r = (0x2u64) as u64; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 17; continue; }
            14 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x138u64))) as u32 as u64; rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x4fd8u64))) as u32) as u32 as u64; let y_addr: u64 = rsi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x48u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp.wrapping_add(0x28u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x140u64))) as u64; rax = ((rax).wrapping_sub(1)) as u64; rax = ((((rax as u32)) & (0x1u32))) as u32 as u64; fl_l = ((rax).wrapping_add(0x4u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 18; continue; }
            16 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x138u64))) as u64; fl_l = ((rax).wrapping_sub(1)) as u64; rax = ((fl_l as u64)) as u64; bb = 17; continue; }
            17 => { fl_l = ((((rax as u32)) & (0x3u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; bb = 18; continue; }
            18 => { rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rcx = ((rcx).wrapping_shl(3u32)) as u64; rax = rsi.wrapping_add(0x48); rax = ((rax).wrapping_add(rcx)) as u64; fl_l = ((xmm7 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; let y_addr: u64 = rax; xmm2 = crate::simd_ops::ru128(y_addr); ymm2_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); xmm0 = ((crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64) as u128; ymm0_hi = 0; let y_addr: u64 = rsp.wrapping_add(0x50u64); crate::simd_ops::wu128(y_addr, xmm2); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm2_hi); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), ((xmm0 as u64)) as u64); if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 22; continue; } bb = 19; continue; }
            19 => { rcx = (((xmm2 >> 64) as u64)) as u64; fl_l = rcx; fl_r = rcx; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 22; continue; } bb = 20; continue; }
            20 => { rdx = (((xmm1 >> 64) as u64)) as u64; fl_l = rdx; fl_r = rcx; if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { bb = 22; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rdx = ((rdx).wrapping_sub(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32) as u128; ymm0_hi = 0; xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm2 as u32)).to_bits() as u128); ymm1_hi = 0; xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x24u64))) as u32) as u128; ymm0_hi = 0; xmm2 = crate::simd_ops::vshufps(xmm2, xmm2, 0x55u8); ymm2_hi = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm1 as u32)) as u32); xmm6 = 0u128; xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm2 as u32)).to_bits() as u128); ymm1_hi = 0; xmm6 = (xmm6 & !0xffff_ffff_ffff_ffffu128) | ((((rdx) as i64) as f64).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((xmm1 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = 0u128; ymm0_hi = 0; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); ymm0_hi = 0; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); xmm1 = (xmm6 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm6 as u64) / f64::from_bits(xmm0 as u64)).to_bits() as u128); ymm1_hi = 0; xmm2 = (xmm1 & !0xffff_ffffu128) | (((f64::from_bits(xmm1 as u64)) as f32).to_bits() as u128); ymm2_hi = 0; xmm3 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) / f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm1 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x20u64)))).to_bits() as u128); ymm1_hi = 0; xmm0 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x24u64)))).to_bits() as u128); ymm0_hi = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((xmm0 as u32)) as u32); bb = 23; continue; }
            22 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32) as u128; ymm0_hi = 0; xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm2 as u32)).to_bits() as u128); ymm1_hi = 0; xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x24u64))) as u32) as u128; ymm0_hi = 0; xmm2 = crate::simd_ops::vshufps(xmm2, xmm2, 0x55u8); ymm2_hi = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm1 as u32)) as u32); xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm2 as u32)).to_bits() as u128); ymm1_hi = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((xmm1 as u32)) as u32); bb = 23; continue; }
            23 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r8 = rbp.wrapping_add(0x50); r9 = (((rdi as u8)) as u32) as u32 as u64; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x44u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x80u64))) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x90u64))) as u128; if (((fl_l as u32)) != 0) { bb = 36; continue; } bb = 24; continue; }
            24 => { rax = 0xffffffff; rcx = rsi.wrapping_add(0x38); let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rsi.wrapping_add(0x40u64), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 36; continue; }
            25 => { rax = 0x86; fl_l = ((r9 as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 35; continue; } bb = 26; continue; }
            26 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x40u64))) as u32 as u64; rdi = ((rax as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 28; continue; } bb = 27; continue; }
            27 => { rcx = rsi.wrapping_add(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsi.wrapping_add(0x40u64), ((rdi as u32)) as u32); bb = 28; continue; }
            28 => { crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x44u64), (1) as u32); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0xe0u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if ((((fl_l as u8)) as i8) >= (((fl_r as u8)) as i8)) { bb = 32; continue; } bb = 29; continue; }
            29 => { rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x4fd8u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 31; continue; } bb = 30; continue; }
            30 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x140u64))) as u64; rax = ((rax).wrapping_sub(1)) as u64; rax = ((((rax as u32)) & (0x1u32))) as u32 as u64; rax = ((rax).wrapping_add(0x4u64)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rcx = ((rcx).wrapping_shl(3u32)) as u64; rax = rsi.wrapping_add(0x48); fl_l = ((rax).wrapping_add(rcx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 33; continue; }
            31 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x138u64))) as u64; rax = ((rax).wrapping_sub(1)) as u64; rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rcx = ((rcx).wrapping_shl(3u32)) as u64; rax = rsi.wrapping_add(0x48); fl_l = ((rax).wrapping_add(rcx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 33; continue; }
            32 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x138u64))) as u32 as u64; fl_l = ((((rax as u32)) & (0x3u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = rax.wrapping_add(0x9); rax = ((rsi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; bb = 33; continue; }
            33 => { let y_addr: u64 = rax; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp.wrapping_add(0x28u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(r14, rax); fl_l = (crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x44u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 36; continue; } bb = 34; continue; }
            34 => { rax = 0xffffffff; rcx = rsi.wrapping_add(0x38); let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rsi.wrapping_add(0x40u64), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 36; continue; }
            35 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r14, rcx); bb = 36; continue; }
            36 => { rax = r14; r11 = rsp.wrapping_add(0xa0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cc840(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645790; const P1: u64 = 0x1426455a8; const P2: u64 = 0x143427080; const P3: u64 = 0x143879688; const P4: u64 = 0x143879698; const P5: u64 = 0x143879670; const P6: u64 = 0x140000000; const P7: u64 = 0x142406920; const P8: u64 = 0x1420d3d40; const P9: u64 = 0x1420d38f0; const P10: u64 = 0x142645598; const P11: u64 = 0x1420cd680; const P12: u64 = 0x1434268d0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x28u64), (xmm6) as u128); rdi = (((r9 as u8)) as u32) as u32 as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x38u64), (xmm7) as u128); r14 = rdx; rsi = (((r8 as u16)) as u32) as u32 as u64; r15 = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru32(r14.wrapping_add(0x40u64))) as u32 as u64; r12 = ((rax as u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = r14.wrapping_add(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(r14.wrapping_add(0x40u64), ((r12 as u32)) as u32); bb = 2; continue; }
            2 => { crate::simd_ops::lk_add_u32(r14.wrapping_add(0x44u64), (1) as u32); fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((((fl_l as u8)) & ((fl_r as u8))) as i8) >= 0) { bb = 9; continue; } bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru8(r15.wrapping_add(0x4fd8u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0x140u64))) as u64; rax = ((rax).wrapping_sub(1)) as u64; rax = ((((rax as u32)) & (0x1u32))) as u32 as u64; fl_l = ((rax).wrapping_add(0x4u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 6; continue; }
            5 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0x138u64))) as u64; rax = ((rax).wrapping_sub(1)) as u64; fl_l = ((((rax as u32)) & (0x3u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; bb = 6; continue; }
            6 => { rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((crate::simd_ops::ru8(r15.wrapping_add(0x4fd8u64))) as u32) as u32 as u64; rcx = ((rcx).wrapping_shl(3u32)) as u64; let y_addr: u64 = r14.wrapping_add(rcx).wrapping_add(0x48u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); xmm1 = ((crate::simd_ops::ru64(r14.wrapping_add(rcx).wrapping_add(0x68u64))) as u64) as u128; let y_addr: u64 = rsp.wrapping_add(0x28u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), ((xmm1 as u64)) as u64); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0x140u64))) as u32 as u64; rax = ((((rax as u32)) & (0x1u32))) as u32 as u64; fl_l = ((rax).wrapping_add(0x4u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 13; continue; }
            8 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0x138u64))) as u64; fl_l = rax; fl_r = (0x2u64) as u64; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 12; continue; }
            9 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0x138u64))) as u32 as u64; rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((crate::simd_ops::ru8(r15.wrapping_add(0x4fd8u64))) as u32) as u32 as u64; let y_addr: u64 = r14.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x48u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); xmm1 = ((crate::simd_ops::ru64(r14.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x68u64))) as u64) as u128; let y_addr: u64 = rsp.wrapping_add(0x28u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), ((xmm1 as u64)) as u64); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0x140u64))) as u64; rax = ((rax).wrapping_sub(1)) as u64; rax = ((((rax as u32)) & (0x1u32))) as u32 as u64; fl_l = ((rax).wrapping_add(0x4u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 13; continue; }
            11 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0x138u64))) as u64; fl_l = ((rax).wrapping_sub(1)) as u64; rax = ((fl_l as u64)) as u64; bb = 12; continue; }
            12 => { fl_l = ((((rax as u32)) & (0x3u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; bb = 13; continue; }
            13 => { xmm7 = ((crate::simd_ops::ru32(P2)) as u32) as u128; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; r12 = 0; fl_l = 0; fl_r = 0; rcx = ((rcx).wrapping_shl(3u32)) as u64; rax = r14.wrapping_add(0x48); rax = ((rax).wrapping_add(rcx)) as u64; xmm6 = 0u128; let y_addr: u64 = rax; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64) as u128; let y_addr: u64 = rsp.wrapping_add(0x50u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), ((xmm1 as u64)) as u64); fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x40u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 17; continue; } bb = 14; continue; }
            14 => { xmm0 = ((crate::simd_ops::ru32(P3)) as u32) as u128; ymm0_hi = 0; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 16; continue; } bb = 15; continue; }
            15 => { if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = ((crate::simd_ops::ru8(r15.wrapping_add(0x4fd8u64))) as u32) as u32 as u64; xmm2 = ((crate::simd_ops::ru32(P4)) as u32) as u128; xmm0 = (((rax as u32)) as u32) as u128; ymm0_hi = 0; xmm1 = (((r12 as u32)) as u32) as u128; xmm3 = crate::simd_ops::vpcmpeqd(xmm0, xmm1); xmm1 = ((crate::simd_ops::ru32(P5)) as u32) as u128; xmm0 = crate::simd_ops::vblendvps(xmm1, xmm2, xmm3); ymm0_hi = 0; xmm7 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(crate::simd_ops::ru32(P3))).to_bits() as u128); bb = 17; continue; }
            17 => { rax = ((rsi.wrapping_sub(0x83u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xe; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 45; continue; } bb = 18; continue; }
            18 => { rdx = P6; rax = (rax as u32 as i32 as i64) as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x20ccd14u64))) as u32 as u64; fl_l = ((rcx).wrapping_add(rdx)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; j_tgt = rcx; bb = 0xffff_fffe; continue; }
            19 => { fl_l = ((xmm7 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 23; continue; } bb = 20; continue; }
            20 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; fl_l = rcx; fl_r = rcx; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 23; continue; } bb = 21; continue; }
            21 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = rdx; fl_r = rcx; if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { bb = 23; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rdx = ((rdx).wrapping_sub(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), rax); xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0xc8u64))) as u32) as u128; ymm0_hi = 0; xmm2 = ((crate::simd_ops::ru32(rsp.wrapping_add(0xccu64))) as u32) as u128; xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x50u64)))).to_bits() as u128); xmm0 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x54u64)))).to_bits() as u128); ymm0_hi = 0; xmm6 = 0u128; xmm6 = (xmm6 & !0xffff_ffff_ffff_ffffu128) | ((((rdx) as i64) as f64).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0xccu64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0xc8u64), ((xmm1 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = 0u128; ymm0_hi = 0; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); ymm0_hi = 0; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; xmm1 = (xmm6 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm6 as u64) / f64::from_bits(xmm0 as u64)).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | (((f64::from_bits(xmm1 as u64)) as f32).to_bits() as u128); xmm3 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) / f32::from_bits(xmm2 as u32)).to_bits() as u128); crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), rax); xmm1 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0xc8u64)))).to_bits() as u128); xmm0 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0xccu64)))).to_bits() as u128); ymm0_hi = 0; bb = 24; continue; }
            23 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), rax); xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0xc8u64))) as u32) as u128; ymm0_hi = 0; xmm2 = ((crate::simd_ops::ru32(rsp.wrapping_add(0xccu64))) as u32) as u128; xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x50u64)))).to_bits() as u128); xmm0 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x54u64)))).to_bits() as u128); ymm0_hi = 0; bb = 24; continue; }
            24 => { crate::simd_ops::wu32(rsp.wrapping_add(0xccu64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0xc8u64), ((xmm1 as u32)) as u32); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; r8 = r15.wrapping_add(0x50); r9 = (((rdi as u8)) as u32) as u32 as u64; rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm1 = ((crate::simd_ops::ru32(rax)) as u32) as u128; xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as u32) as u128; ymm0_hi = 0; xmm2 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm3 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm6 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32)).sqrt().to_bits() as u128); bb = 42; continue; }
            25 => { fl_l = ((xmm7 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 29; continue; } bb = 26; continue; }
            26 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; fl_l = rcx; fl_r = rcx; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 29; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = rax; fl_r = rcx; if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { bb = 29; continue; } bb = 28; continue; }
            28 => { xmm6 = 0u128; rax = ((rax).wrapping_sub(rcx)) as u64; xmm6 = (xmm6 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = 0u128; ymm0_hi = 0; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); ymm0_hi = 0; xmm1 = (xmm6 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm6 as u64) / f64::from_bits(xmm0 as u64)).to_bits() as u128); xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32) as u128; ymm0_hi = 0; xmm2 = (xmm1 & !0xffff_ffffu128) | (((f64::from_bits(xmm1 as u64)) as f32).to_bits() as u128); xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x50u64)))).to_bits() as u128); xmm3 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) / f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm0 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); ymm0_hi = 0; bb = 30; continue; }
            29 => { xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32) as u128; ymm0_hi = 0; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x50u64)))).to_bits() as u128); ymm0_hi = 0; bb = 30; continue; }
            30 => { rdx = r15.wrapping_add(0x50); r8 = (((rdi as u8)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm6 = (xmm0) as u128; bb = 42; continue; }
            31 => { fl_l = ((xmm7 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 35; continue; } bb = 32; continue; }
            32 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; fl_l = rcx; fl_r = rcx; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 35; continue; } bb = 33; continue; }
            33 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = rax; fl_r = rcx; if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { bb = 35; continue; } bb = 34; continue; }
            34 => { xmm6 = 0u128; rax = ((rax).wrapping_sub(rcx)) as u64; xmm6 = (xmm6 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = 0u128; ymm0_hi = 0; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); ymm0_hi = 0; xmm1 = (xmm6 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm6 as u64) / f64::from_bits(xmm0 as u64)).to_bits() as u128); xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x2cu64))) as u32) as u128; ymm0_hi = 0; xmm2 = (xmm1 & !0xffff_ffffu128) | (((f64::from_bits(xmm1 as u64)) as f32).to_bits() as u128); xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x54u64)))).to_bits() as u128); xmm3 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) / f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm0 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); ymm0_hi = 0; bb = 36; continue; }
            35 => { xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x2cu64))) as u32) as u128; ymm0_hi = 0; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x54u64)))).to_bits() as u128); ymm0_hi = 0; bb = 36; continue; }
            36 => { rdx = r15.wrapping_add(0x5c); r8 = (((rdi as u8)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm6 = (xmm0) as u128; bb = 42; continue; }
            37 => { xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x3cu64))) as u32) as u128; ymm0_hi = 0; xmm1 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x38u64))) as u32) as u128; xmm3 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm6 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32)).sqrt().to_bits() as u128); xmm6 = (xmm6) as u128; bb = 42; continue; }
            38 => { xmm6 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x38u64))) as u32) as u128; bb = 42; continue; }
            39 => { xmm6 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x3cu64))) as u32) as u128; bb = 42; continue; }
            40 => { xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32) as u128; ymm0_hi = 0; xmm6 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x68u64)))).to_bits() as u128); bb = 42; continue; }
            41 => { xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x44u64))) as u32) as u128; ymm0_hi = 0; xmm6 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x6cu64)))).to_bits() as u128); bb = 42; continue; }
            42 => { fl_l = (crate::simd_ops::lk_add_u32(r14.wrapping_add(0x44u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 44; continue; } bb = 43; continue; }
            43 => { rax = 0xffffffff; rcx = r14.wrapping_add(0x38); let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(r14.wrapping_add(0x40u64), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 44; continue; }
            44 => { xmm0 = (xmm6) as u128; ymm0_hi = 0; bb = 48; continue; }
            45 => { fl_l = (crate::simd_ops::lk_add_u32(r14.wrapping_add(0x44u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 47; continue; } bb = 46; continue; }
            46 => { rax = 0xffffffff; rcx = r14.wrapping_add(0x38); let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(r14.wrapping_add(0x40u64), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 47; continue; }
            47 => { r9 = (((rdi as u8)) as u32) as u32 as u64; r8 = (((rsi as u16)) as u32) as u32 as u64; rdx = r14; rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm2 = ((crate::simd_ops::ru32(P12)) as u32) as u128; rax = (((rax as u8)) as u32) as u32 as u64; xmm0 = (((rax as u32)) as u32) as u128; ymm0_hi = 0; xmm1 = (((r12 as u32)) as u32) as u128; xmm3 = crate::simd_ops::vpcmpeqd(xmm0, xmm1); xmm0 = crate::simd_ops::vblendvps(xmm2, xmm6, xmm3); ymm0_hi = 0; bb = 48; continue; }
            48 => { r11 = rsp.wrapping_add(0xa0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; xmm6 = (crate::simd_ops::ru128(r11.wrapping_sub(0x10u64))) as u128; xmm7 = (crate::simd_ops::ru128(r11.wrapping_sub(0x20u64))) as u128; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x1420cc840u64) { 0x46 => { bb = 1; continue; } 0x54 => { bb = 2; continue; } 0x5d => { bb = 3; continue; } 0x6a => { bb = 4; continue; } 0x7d => { bb = 5; continue; } 0x8a => { bb = 6; continue; } 0xb9 => { bb = 7; continue; } 0xc9 => { bb = 8; continue; } 0xd6 => { bb = 9; continue; } 0x10b => { bb = 10; continue; } 0x11e => { bb = 11; continue; } 0x128 => { bb = 12; continue; } 0x12b => { bb = 13; continue; } 0x164 => { bb = 14; continue; } 0x172 => { bb = 15; continue; } 0x174 => { bb = 16; continue; } 0x1a8 => { bb = 17; continue; } 0x1b7 => { bb = 18; continue; } 0x1cc => { bb = 19; continue; } 0x1d6 => { bb = 20; continue; } 0x1e4 => { bb = 21; continue; } 0x1f2 => { bb = 22; continue; } 0x27c => { bb = 23; continue; } 0x2a7 => { bb = 24; continue; } 0x2f4 => { bb = 25; continue; } 0x2fa => { bb = 26; continue; } 0x304 => { bb = 27; continue; } 0x30e => { bb = 28; continue; } 0x349 => { bb = 29; continue; } 0x355 => { bb = 30; continue; } 0x36e => { bb = 31; continue; } 0x374 => { bb = 32; continue; } 0x37e => { bb = 33; continue; } 0x388 => { bb = 34; continue; } 0x3c3 => { bb = 35; continue; } 0x3cf => { bb = 36; continue; } 0x3e5 => { bb = 37; continue; } 0x407 => { bb = 38; continue; } 0x40f => { bb = 39; continue; } 0x417 => { bb = 40; continue; } 0x425 => { bb = 41; continue; } 0x431 => { bb = 42; continue; } 0x438 => { bb = 43; continue; } 0x44e => { bb = 44; continue; } 0x454 => { bb = 45; continue; } 0x45b => { bb = 46; continue; } 0x471 => { bb = 47; continue; } 0x4a5 => { bb = 48; continue; } _ => { return crate::simd_ops::tail_stk_xmm(j_tgt, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); } } }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cd050(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64;
    fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x89;
    rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64;
    if !((((fl_l as u32)) == ((fl_r as u32)))) { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if !((((fl_l as u32)) == ((fl_r as u32)))) { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if !((((fl_l as u32)) == ((fl_r as u32)))) { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if !((((fl_l as u32)) == ((fl_r as u32)))) { rax = 0x24; rax = ((crate::simd_ops::ru8(rax.wrapping_add(rdx))) as u32) as u32 as u64; return rax; } rax = 0x23; rax = ((crate::simd_ops::ru8(rax.wrapping_add(rdx))) as u32) as u32 as u64; return rax; } rax = 0x22; rax = ((crate::simd_ops::ru8(rax.wrapping_add(rdx))) as u32) as u32 as u64; return rax; } rax = 0x21; rax = ((crate::simd_ops::ru8(rax.wrapping_add(rdx))) as u32) as u32 as u64; return rax; }
    rax = 0x20;
    rax = ((crate::simd_ops::ru8(rax.wrapping_add(rdx))) as u32) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cd0b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143426ff0; const P1: u64 = 0x143426460;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm2 = ((crate::simd_ops::ru32(P0)) as u32) as u128;
    xmm3 = (!(xmm2)) & (xmm1);
    xmm0 = (xmm1) as u128;
    xmm1 = ((crate::simd_ops::ru32(P1)) as u32) as u128;
    fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm3 as u32)) as u32 as u64;
    rax = (rax & !0xffu64) | (((if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64));
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cdca0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x71; const P1: u64 = 0xf; const P2: u64 = 0x142646288; const P3: u64 = 0x4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    rbx = rcx;
    if (((fl_l as u8)) == ((fl_r as u8))) { crate::simd_ops::wu8(rbx.wrapping_add(P3), (0x1u8) as u8); rax = rbx; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rdx = 0x1;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x20001u32) as u32);
    rcx = rsp.wrapping_add(0x20);
    crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((rdx as u32)) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x0u64) as u64);
    r8 = ((rdx.wrapping_add(P1)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) != 0)) { crate::simd_ops::wu8(rbx.wrapping_add(P3), ((rax as u8)) as u8); rax = rbx; crate::simd_ops::wu32(rbx, (0xau32) as u32); rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    crate::simd_ops::wu8(rbx.wrapping_add(P3), (0x1u8) as u8);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cdd10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646288; const P1: u64 = 0x142645790; const P2: u64 = 0x1426455a8; const P3: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x71u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rsi = rdx; rdi = rcx; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 14; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rcx = rsp.wrapping_add(0x20); rdx = 0x1; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (0x20001u64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); r8 = ((rdx.wrapping_add(0xfu64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu8(rdi.wrapping_add(0x4u64), ((rax as u8)) as u8); rax = rdi; crate::simd_ops::wu32(rdi, (0xau32) as u32); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rbx = rsi.wrapping_add(0x418); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbp); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rbp = ((rax as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rbp as u32)) as u32); bb = 5; continue; }
            5 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(0xcu64), (1) as u32); rcx = rsi.wrapping_add(0x428); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rdx = rcx.wrapping_add(0xf0); rax = (crate::simd_ops::ru64(rdx)) as u64; rax = ((rax).wrapping_add(1)) as u64; rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; let y_addr: u64 = rcx.wrapping_add(rax.wrapping_mul(8)); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); xmm2 = ((crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x20u64))) as u64) as u128; rax = rcx.wrapping_add(0xa0); fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { bb = 7; continue; }
            7 => { let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), ((xmm2 as u64)) as u64); rcx = ((rcx).wrapping_add(0x28u64)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 8; continue; }
            8 => { fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 9; continue; }
            9 => { bb = 10; continue; }
            10 => { let y_addr: u64 = rax; crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), ((xmm2 as u64)) as u64); rax = ((rax).wrapping_add(0x28u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(0xcu64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = 0xffffffff; rcx = rbx; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 14; continue; }
            14 => { crate::simd_ops::wu8(rdi.wrapping_add(0x4u64), (0x1u8) as u8); rax = rdi; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cde50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645790; const P1: u64 = 0x1426455a8; const P2: u64 = 0x142645598; const P3: u64 = 0x1434268d0; const P4: u64 = 0x1420ce4c0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm5: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rsi = rdx; rcx = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 29; continue; } bb = 1; continue; }
            1 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x8; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 18; continue; } bb = 2; continue; }
            2 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 17; continue; } bb = 3; continue; }
            3 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 4; continue; }
            4 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x5; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 34; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 34; continue; } bb = 6; continue; }
            6 => { rbx = rdi.wrapping_add(0x418); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = ((rax as u32)) as u32 as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rbp as u32)) as u32); bb = 8; continue; }
            8 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(0xcu64), (1) as u32); fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x1cu64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu8(rdi.wrapping_add(0x530u64), (0x1u8) as u8); bb = 11; continue; }
            10 => { crate::simd_ops::wu8(rdi.wrapping_add(0x530u64), (0x0u8) as u8); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x518u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((crate::simd_ops::ru32(rsi.wrapping_add(0x28u64))) as i32) as f32).to_bits() as u128); rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rdi.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x428u64)))).to_bits() as u128); crate::simd_ops::wu32(rdi.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x428u64), ((xmm1 as u32)) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x518u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((crate::simd_ops::ru32(rsi.wrapping_add(0x2cu64))) as i32) as f32).to_bits() as u128); rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rdi.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x42cu64)))).to_bits() as u128); crate::simd_ops::wu32(rdi.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x42cu64), ((xmm1 as u32)) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x518u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = ((((rax as u32)) & (0x3u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x430u64), rax); bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(0xcu64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = 0xffffffff; rcx = rbx; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 13; continue; }
            13 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 35; continue; }
            14 => { rbx = rdi.wrapping_add(0x418); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rbp = ((rax as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rbp as u32)) as u32); bb = 16; continue; }
            16 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(0xcu64), (1) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x518u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; xmm5 = 0u128; xmm0 = 0u128; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x438u64), rax); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x518u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; xmm2 = ((crate::simd_ops::ru32(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x43cu64))) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x438u64))) as u32) as u128; xmm3 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(xmm5 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm5 as u32) }).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) < f32::from_bits(crate::simd_ops::ru32(P3)) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P3)) }).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(xmm5 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm5 as u32) }).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) < f32::from_bits(crate::simd_ops::ru32(P3)) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P3)) }).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((xmm2 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x3cu64), ((xmm3 as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x438u64), rax); xmm0 = (xmm0 & !0xffff_ffffu128) | ((((crate::simd_ops::ru32(rsi.wrapping_add(0x30u64))) as i32) as f32).to_bits() as u128); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x518u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rdi.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x440u64)))).to_bits() as u128); crate::simd_ops::wu32(rdi.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x440u64), ((xmm1 as u32)) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x518u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = ((((rax as u32)) & (0x3u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((crate::simd_ops::ru32(rsi.wrapping_add(0x34u64))) as i32) as f32).to_bits() as u128); rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rdi.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x444u64)))).to_bits() as u128); crate::simd_ops::wu32(rdi.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x444u64), ((xmm1 as u32)) as u32); bb = 11; continue; }
            17 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 35; continue; }
            18 => { rbx = rdi.wrapping_add(0x418); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rbp = ((rax as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rbp as u32)) as u32); bb = 20; continue; }
            20 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(0xcu64), (1) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x518u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; xmm5 = 0u128; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x438u64), rax); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x518u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; xmm2 = ((crate::simd_ops::ru32(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x43cu64))) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x438u64))) as u32) as u128; xmm3 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(xmm5 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm5 as u32) }).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) < f32::from_bits(crate::simd_ops::ru32(P3)) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P3)) }).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(xmm5 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm5 as u32) }).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) < f32::from_bits(crate::simd_ops::ru32(P3)) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P3)) }).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((xmm2 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x3cu64), ((xmm3 as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x438u64), rax); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x518u64))) as u64; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x30u64))) as u32 as u64; rax = ((rax).wrapping_add(1)) as u64; rdx = ((crate::simd_ops::ru8(rsi.wrapping_add(0x34u64))) as u32) as u32 as u64; rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; r8 = ((rdi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 28; continue; } bb = 21; continue; }
            21 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 27; continue; } bb = 22; continue; }
            22 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 26; continue; } bb = 23; continue; }
            23 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 25; continue; } bb = 24; continue; }
            24 => { rax = 0x24; crate::simd_ops::wu8(r8.wrapping_add(rax).wrapping_add(0x428u64), ((rdx as u8)) as u8); bb = 11; continue; }
            25 => { rax = 0x23; crate::simd_ops::wu8(r8.wrapping_add(rax).wrapping_add(0x428u64), ((rdx as u8)) as u8); bb = 11; continue; }
            26 => { rax = 0x21; crate::simd_ops::wu8(r8.wrapping_add(rax).wrapping_add(0x428u64), ((rdx as u8)) as u8); bb = 11; continue; }
            27 => { rax = 0x22; crate::simd_ops::wu8(r8.wrapping_add(rax).wrapping_add(0x428u64), ((rdx as u8)) as u8); bb = 11; continue; }
            28 => { rax = 0x20; crate::simd_ops::wu8(r8.wrapping_add(rax).wrapping_add(0x428u64), ((rdx as u8)) as u8); bb = 11; continue; }
            29 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 34; continue; } bb = 30; continue; }
            30 => { rbx = rdi.wrapping_add(0x418); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rsi = ((rax as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 32; continue; } bb = 31; continue; }
            31 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rsi as u32)) as u32); bb = 32; continue; }
            32 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(0xcu64), (1) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x518u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; crate::simd_ops::wu32(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x448u64), (0x0u32) as u32); crate::simd_ops::wu8(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x44cu64), (0x0u8) as u8); fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(0xcu64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 34; continue; } bb = 33; continue; }
            33 => { rax = 0xffffffff; rcx = rbx; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 34; continue; }
            34 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 35; continue; }
            35 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420ce220(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645790; const P1: u64 = 0x1426455a8; const P2: u64 = 0x1434268d0; const P3: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm5: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = rdx; rbx = rcx.wrapping_add(0x418); rsi = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rdi = ((rax as u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rdi as u32)) as u32); bb = 2; continue; }
            2 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(0xcu64), (1) as u32); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x518u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; xmm5 = 0u128; xmm0 = 0u128; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x438u64), rax); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x518u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; xmm2 = ((crate::simd_ops::ru32(rsi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x43cu64))) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(rsi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x438u64))) as u32) as u128; xmm3 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(xmm5 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm5 as u32) }).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) < f32::from_bits(crate::simd_ops::ru32(P2)) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P2)) }).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(xmm5 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm5 as u32) }).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) < f32::from_bits(crate::simd_ops::ru32(P2)) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P2)) }).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((xmm2 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((xmm3 as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x438u64), rax); xmm0 = (xmm0 & !0xffff_ffffu128) | ((((crate::simd_ops::ru32(rbp.wrapping_add(0x30u64))) as i32) as f32).to_bits() as u128); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x518u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rsi.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x440u64)))).to_bits() as u128); crate::simd_ops::wu32(rsi.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x440u64), ((xmm1 as u32)) as u32); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x518u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((crate::simd_ops::ru32(rbp.wrapping_add(0x34u64))) as i32) as f32).to_bits() as u128); rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rsi.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x444u64)))).to_bits() as u128); crate::simd_ops::wu32(rsi.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x444u64), ((xmm1 as u32)) as u32); fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(0xcu64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = 0xffffffff; rcx = rbx; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420ce370(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645790; const P1: u64 = 0x1426455a8; const P2: u64 = 0x1434268d0; const P3: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm5: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = rdx; rbx = rcx.wrapping_add(0x418); rsi = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rdi = ((rax as u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rdi as u32)) as u32); bb = 2; continue; }
            2 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(0xcu64), (1) as u32); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x518u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; xmm5 = 0u128; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x438u64), rax); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x518u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; xmm2 = ((crate::simd_ops::ru32(rsi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x43cu64))) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(rsi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x438u64))) as u32) as u128; xmm3 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(xmm5 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm5 as u32) }).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) < f32::from_bits(crate::simd_ops::ru32(P2)) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P2)) }).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(xmm5 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm5 as u32) }).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) < f32::from_bits(crate::simd_ops::ru32(P2)) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P2)) }).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((xmm2 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((xmm3 as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x438u64), rax); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x518u64))) as u64; rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x30u64))) as u32 as u64; rax = ((rax).wrapping_add(1)) as u64; rdx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x34u64))) as u32) as u32 as u64; rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; r8 = ((rsi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 10; continue; } bb = 3; continue; }
            3 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 4; continue; }
            4 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 5; continue; }
            5 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = 0x24; bb = 11; continue; }
            7 => { rax = 0x23; bb = 11; continue; }
            8 => { rax = 0x21; bb = 11; continue; }
            9 => { rax = 0x22; bb = 11; continue; }
            10 => { rax = 0x20; bb = 11; continue; }
            11 => { crate::simd_ops::wu8(r8.wrapping_add(rax).wrapping_add(0x428u64), ((rdx as u8)) as u8); fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(0xcu64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = 0xffffffff; rcx = rbx; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420ce4c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645790; const P1: u64 = 0x1426455a8; const P2: u64 = 0x1434268d0; const P3: u64 = 0x142645598;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm5: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rdx; rbx = rcx.wrapping_add(0x418); rdi = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rbp = ((rax as u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rbp as u32)) as u32); bb = 2; continue; }
            2 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(0xcu64), (1) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x518u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; xmm5 = 0u128; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x438u64), rax); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x518u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; xmm2 = ((crate::simd_ops::ru32(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x43cu64))) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x438u64))) as u32) as u128; xmm3 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(xmm5 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm5 as u32) }).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) < f32::from_bits(crate::simd_ops::ru32(P2)) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P2)) }).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(xmm5 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm5 as u32) }).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) < f32::from_bits(crate::simd_ops::ru32(P2)) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P2)) }).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((xmm2 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((xmm3 as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x438u64), rax); fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x530u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x53cu64))) as u32) as u32 as u64; r11 = rdi.wrapping_add(0x528); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { r10 = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; r9 = (crate::simd_ops::ru64(rdi.wrapping_add(0x534u64))) as u64; rax = r10; r10 = (((r10 as u32)).wrapping_sub(crate::simd_ops::ru32(r11))) as u32 as u64; r8 = r9; r9 = (((r9 as u32)).wrapping_sub(crate::simd_ops::ru32(r11))) as u32 as u64; r8 = ((r8).wrapping_shr(32u32)) as u64; r8 = (((r8 as u32)).wrapping_sub(crate::simd_ops::ru32(r11.wrapping_add(0x4u64)))) as u32 as u64; rax = ((rax).wrapping_shr(32u32)) as u64; rax = (((rax as u32)).wrapping_sub(crate::simd_ops::ru32(r11.wrapping_add(0x4u64)))) as u32 as u64; r8 = ((((r8 as u32)) as i32).wrapping_mul(((rax as u32)) as i32) as u32) as u32 as u64; r9 = ((((r9 as u32)) as i32).wrapping_mul(((r10 as u32)) as i32) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_add((r9 as u32))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) > 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { r8 = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x518u64))) as u64; rdx = (crate::simd_ops::ru32(r11.wrapping_add(0x4u64))) as u32 as u64; rax = ((rax).wrapping_add(1)) as u64; rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; xmm0 = 0u128; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((r8 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_sub(crate::simd_ops::ru32(r11))) as u32 as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128); xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x428u64)))).to_bits() as u128); crate::simd_ops::wu32(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x428u64), ((xmm1 as u32)) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x518u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; r8 = ((r8).wrapping_shr(32u32)) as u64; rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; r8 = (((r8 as u32)).wrapping_sub((rdx as u32))) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | (((((r8 as u32)) as i32) as f32).to_bits() as u128); rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x42cu64)))).to_bits() as u128); crate::simd_ops::wu32(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x42cu64), ((xmm1 as u32)) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x518u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = ((((rax as u32)) & (0x3u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x430u64), rax); bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x528u64), rax); fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(0xcu64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = 0xffffffff; rcx = rbx; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420ce6a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x418; const P2: u64 = 0x142645790; const P3: u64 = 0x8; const P4: u64 = 0x1426455a8; const P5: u64 = 0xc; const P6: u64 = 0x518; const P7: u64 = 0x448; const P8: u64 = 0x44c; const P9: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rsi = rcx; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = ((rcx.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; rdi = ((rax as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbx.wrapping_add(P3), ((rdi as u32)) as u32); bb = 3; continue; }
            3 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(P5), (1) as u32); rax = (crate::simd_ops::ru64(rsi.wrapping_add(P6))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; crate::simd_ops::wu32(rsi.wrapping_add(rax.wrapping_mul(8)).wrapping_add(P7), (0x0u32) as u32); crate::simd_ops::wu8(rsi.wrapping_add(rax.wrapping_mul(8)).wrapping_add(P8), (0x0u8) as u8); fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(P5), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = 0xffffffff; rcx = rbx; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rbx.wrapping_add(P3), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420ce740(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x418; const P2: u64 = 0x142645790; const P3: u64 = 0x8; const P4: u64 = 0x1426455a8; const P5: u64 = 0xc; const P6: u64 = 0x1c; const P7: u64 = 0x530; const P8: u64 = 0x518; const P9: u64 = 0x28; const P10: u64 = 0x428; const P11: u64 = 0x2c; const P12: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = 0x0; rsi = rdx; rdi = rcx; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = ((rcx.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbp); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; rbp = ((rax as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(P3), ((rbp as u32)) as u32); bb = 4; continue; }
            4 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(P5), (1) as u32); fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(P6))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu8(rdi.wrapping_add(P7), (0x1u8) as u8); bb = 7; continue; }
            6 => { crate::simd_ops::wu8(rdi.wrapping_add(P7), (0x0u8) as u8); rax = (crate::simd_ops::ru64(rdi.wrapping_add(P8))) as u64; rax = ((rax).wrapping_add(1)) as u64; rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((crate::simd_ops::ru32(rsi.wrapping_add(P9))) as i32) as f32).to_bits() as u128); rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rdi.wrapping_add(rax.wrapping_mul(8)).wrapping_add(P10)))).to_bits() as u128); crate::simd_ops::wu32(rdi.wrapping_add(rax.wrapping_mul(8)).wrapping_add(P10), ((xmm1 as u32)) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(P8))) as u64; rax = ((rax).wrapping_add(1)) as u64; rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((crate::simd_ops::ru32(rsi.wrapping_add(P11))) as i32) as f32).to_bits() as u128); rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rdi.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x42cu64)))).to_bits() as u128); crate::simd_ops::wu32(rdi.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x42cu64), ((xmm1 as u32)) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x518u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = ((((rax as u32)) & (0x3u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x430u64), rax); bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(0xcu64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = 0xffffffff; rcx = rbx; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420ce9e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x142406920; const P2: u64 = 0x4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; xmm0 = 0u128; fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm6) as u128); rdi = r8; xmm6 = (xmm3) as u128; rsi = rdx; rbx = rcx; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; fl_l = rax; fl_r = rax; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_r = rax; if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { bb = 4; continue; } bb = 3; continue; }
            3 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32) as u128; xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(crate::simd_ops::ru32(rdi))).to_bits() as u128); xmm2 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x24u64))) as u32) as u128; xmm0 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(crate::simd_ops::ru32(rdi.wrapping_add(P2)))).to_bits() as u128); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm1 as u32)) as u32); xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rbx, rax); xmm1 = 0u128; xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((((rcx) as i64) as f64).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) / f64::from_bits(xmm0 as u64)).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | (((f64::from_bits(xmm1 as u64)) as f32).to_bits() as u128); xmm3 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) / f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm0 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(crate::simd_ops::ru32(rbx))).to_bits() as u128); crate::simd_ops::wu32(rbx, ((xmm0 as u32)) as u32); xmm0 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_add(P2)))).to_bits() as u128); crate::simd_ops::wu32(rbx.wrapping_add(P2), ((xmm0 as u32)) as u32); bb = 5; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rcx, rax); xmm0 = ((crate::simd_ops::ru32(rcx)) as u32) as u128; xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(crate::simd_ops::ru32(r8))).to_bits() as u128); xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32) as u128; crate::simd_ops::wu32(rcx, ((xmm1 as u32)) as u32); xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(P2)))).to_bits() as u128); crate::simd_ops::wu32(rcx.wrapping_add(P2), ((xmm1 as u32)) as u32); bb = 5; continue; }
            5 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cead0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if !((((fl_l as u32)) == ((fl_r as u32)))) { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if !((((fl_l as u32)) == ((fl_r as u32)))) { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if !((((fl_l as u32)) == ((fl_r as u32)))) { rax = 0x4; return rax; } rax = 0x3; return rax; } rax = 0x1; return rax; } rax = 0x2; return rax; }
    rax = 0;
    fl_l = 0; fl_r = 0;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420ceb10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rdx = (((rcx as u16)) as u32) as u32 as u64;
    fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x89;
    rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64;
    if !((((fl_l as u32)) == ((fl_r as u32)))) { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if !((((fl_l as u32)) == ((fl_r as u32)))) { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if !((((fl_l as u32)) == ((fl_r as u32)))) { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if !((((fl_l as u32)) == ((fl_r as u32)))) { rax = 0x4; return rax; } rax = 0x3; return rax; } rax = 0x2; return rax; } rax = 0x1; return rax; }
    rax = 0;
    fl_l = 0; fl_r = 0;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420ceb50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646120; const P1: u64 = 0x142646298; const P2: u64 = 0x142646290; const P3: u64 = 0x142646080; const P4: u64 = 0x142646108; const P5: u64 = 0x142646208;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x90u64)) as u64; rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x48u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x84u64), ((rax as u32)) as u32); rbx = r8; rdi = rdx; rsi = rcx; xmm1 = 0u128; ymm1_hi = 0; xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x28u64), (xmm0) as u128); let y_addr: u64 = rsp.wrapping_add(0x44u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); let y_addr: u64 = rsp.wrapping_add(0x64u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { rdx = rsp.wrapping_add(0x28); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 2; continue; }
            2 => { rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((rax as u32)) as u32 as u64; rdx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x44u64))) as u8 as u64; fl_r = (0x1eu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 11; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rcx = rdi; rax = (((rax as u32)).wrapping_sub(crate::simd_ops::ru32(rsp.wrapping_add(0x28u64)))) as u32 as u64; rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rbp); rax = (((rax as u32)).wrapping_sub((rdx as u32))) as u32 as u64; rax = (((((rax as u32)) as i32).wrapping_shr(1u32) as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xb8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(crate::simd_ops::ru32(rsp.wrapping_add(0x2cu64)))) as u32 as u64; rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64; rax = (((rax as u32)).wrapping_sub((rdx as u32))) as u32 as u64; rdx = rsp.wrapping_add(0xb8); rax = (((((rax as u32)) as i32).wrapping_shr(1u32) as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xbcu64), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru32(rsp.wrapping_add(0xb8u64))) as u32 as u64; rbp = (crate::simd_ops::ru32(rsp.wrapping_add(0xbcu64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xc8u64), ((rdi as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0xccu64), ((rbp as u32)) as u32); fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0xc4u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 6; continue; }
            6 => { rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x24u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = (((rdi as u32)).wrapping_add(1)) as u32 as u64; rdi = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xc8u64), ((rdi as u32)) as u32); bb = 10; continue; }
            10 => { rdx = ((rbp as u32)) as u32 as u64; rcx = ((rdi as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; crate::simd_ops::wu64(rsi, rax); rax = rsi; rsp = ((rsp).wrapping_add(0x90u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            11 => { crate::simd_ops::wu64(rsi, rbx); rax = rsi; rsp = ((rsp).wrapping_add(0x90u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cecc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645790; const P1: u64 = 0x1426455a8; const P2: u64 = 0x37; const P3: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (((r8 as u16)) as u32) as u32 as u64; rbp = rdx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x10u64))) as u32 as u64; rsi = ((rax as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = rbp.wrapping_add(0x8); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((rsi as u32)) as u32); bb = 2; continue; }
            2 => { crate::simd_ops::lk_add_u32(rbp.wrapping_add(0x14u64), (1) as u32); rax = (0xaaaaaaaaaaaaaaabu64) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x240u64))) as u64; rcx = ((rcx).wrapping_add(1)) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; rax = (((rcx) as i64).wrapping_mul((0x81u64) as i64) as u64) as u64; rax = ((rax).wrapping_add(rbp)) as u64; crate::simd_ops::lk_and_u8(rdi.wrapping_add(rax).wrapping_add(P2), (0xfeu8) as u8); fl_l = (crate::simd_ops::lk_add_u32(rbp.wrapping_add(0x14u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = 0xffffffff; rcx = rbp.wrapping_add(0x8); let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rbp.wrapping_add(0x10u64), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420ced70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645790; const P1: u64 = 0x1426455a8; const P2: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (((r8 as u16)) as u32) as u32 as u64; rbp = rdx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x10u64))) as u32 as u64; rsi = ((rax as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = rbp.wrapping_add(0x8); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((rsi as u32)) as u32); bb = 2; continue; }
            2 => { crate::simd_ops::lk_add_u32(rbp.wrapping_add(0x14u64), (1) as u32); r9 = (0xaaaaaaaaaaaaaaabu64) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x240u64))) as u64; rax = r9; rcx = ((rcx).wrapping_add(1)) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; rcx = (((rcx) as i64).wrapping_mul((0x81u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rdi)) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(rbp).wrapping_add(0x37u64))) as u32) as u32 as u64; rdx = (((rax as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) | (0x1u8))) as u8 as u64)); crate::simd_ops::wu8(rcx.wrapping_add(rbp).wrapping_add(0x37u64), ((rax as u8)) as u8); rdx = (rdx & !0xffu64) | ((((((rdx as u8)) & (0x1u8))) as u8 as u64)); rcx = ((rdi as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x5; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x65; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 5; continue; }
            5 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x240u64))) as u64; rax = r9; rcx = ((rcx).wrapping_add(1)) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; rcx = (((rcx) as i64).wrapping_mul((0x81u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rdi)) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(rbp).wrapping_add(0x37u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = ((((rax as u8)) | (0x2u8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); bb = 9; continue; }
            8 => { fl_l = ((((rax as u8)) & (0xfdu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); bb = 9; continue; }
            9 => { crate::simd_ops::wu8(rcx.wrapping_add(rbp).wrapping_add(0x37u64), ((rax as u8)) as u8); bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::lk_add_u32(rbp.wrapping_add(0x14u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = 0xffffffff; rcx = rbp.wrapping_add(0x8); let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rbp.wrapping_add(0x10u64), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420ceeb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1420ceff0; const P1: u64 = 0x1434268d0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = (((rax as u8)) as u32) as u32 as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    xmm0 = (((rax as u32)) as u32) as u128;
    xmm1 = (((rcx as u32)) as u32) as u128;
    xmm3 = crate::simd_ops::vpcmpeqd(xmm0, xmm1);
    xmm0 = ((crate::simd_ops::ru32(P1)) as u32) as u128;
    xmm2 = 0u128;
    xmm0 = crate::simd_ops::vblendvps(xmm0, xmm2, xmm3);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cf2c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70; const P1: u64 = 0x3a8; const P2: u64 = 0x3ac; const P3: u64 = 0x8; const P4: u64 = 0x33; const P5: u64 = 0x1426462a0; const P6: u64 = 0x3b4; const P7: u64 = 0x2d; const P8: u64 = 0x3bc; const P9: u64 = 0x18; const P10: u64 = 0x1b; const P11: u64 = 0xf; const P12: u64 = 0x142646288;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rdi = rdx; rbx = rcx; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { r9 = 0; fl_l = 0; fl_r = 0; r8 = ((rdx.wrapping_add(P2)) as u64) as u64; rdx = ((r9.wrapping_add(P3)) as u32) as u32 as u64; rcx = ((rdx.wrapping_add(P4)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); r9 = 0; fl_l = 0; fl_r = 0; r8 = ((rdi.wrapping_add(P6)) as u64) as u64; rdx = ((r9.wrapping_add(P3)) as u32) as u32 as u64; rcx = ((rdx.wrapping_add(P7)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); r9 = 0; fl_l = 0; fl_r = 0; r8 = ((rdi.wrapping_add(P8)) as u64) as u64; rdx = ((r9.wrapping_add(P9)) as u32) as u32 as u64; rcx = ((rdx.wrapping_add(P10)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rdx = 0x1; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x60001u32) as u32); rcx = rsp.wrapping_add(0x20); crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((rdx as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x0u64) as u64); r8 = ((rdx.wrapping_add(P11)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu8(rbx.wrapping_add(0x4u64), ((rax as u8)) as u8); rax = rbx; crate::simd_ops::wu32(rbx, (0xau32) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { crate::simd_ops::wu8(rbx.wrapping_add(0x4u64), (0x1u8) as u8); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cf390(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646288; const P1: u64 = 0x142645790; const P2: u64 = 0x1426455a8; const P3: u64 = 0x1426462a8; const P4: u64 = 0x1426462a0; const P5: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0; let mut ymm2_hi: u128 = 0; let mut ymm3_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x70u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r14 = rdx; r15 = rcx; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 15; continue; } bb = 1; continue; }
            1 => { rdx = 0x1; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (0x60001u64) as u64); r12 = 0; fl_l = 0; fl_r = 0; rcx = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r12); r8 = ((rdx.wrapping_add(0xfu64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu32(r15, (0xau32) as u32); rax = r15; crate::simd_ops::wu8(r15.wrapping_add(0x4u64), ((r12 as u8)) as u8); rsp = ((rsp).wrapping_add(0x60u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rbp); rbp = r14.wrapping_add(0x88); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x8u64))) as u32 as u64; rbx = ((rax as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbp.wrapping_add(0x8u64), ((rbx as u32)) as u32); bb = 5; continue; }
            5 => { crate::simd_ops::lk_add_u32(rbp.wrapping_add(0xcu64), (1) as u32); rdi = r14.wrapping_add(0xb8); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x208u64))) as u64; rsi = (0xaaaaaaaaaaaaaaabu64) as u64; rcx = ((rcx).wrapping_add(1)) as u64; rax = rsi; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; rbx = (((rcx) as i64).wrapping_mul((0x81u64) as i64) as u64) as u64; rcx = 0x14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = ((crate::simd_ops::ru8(rbx.wrapping_add(rdi).wrapping_add(0x4u64))) as u32) as u32 as u64; rcx = (((rdx as u8)) as u32) as u32 as u64; rdx = (rdx & !0xffu64) | ((((((rdx as u8)) | (0x2u8))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0xfdu8))) as u8 as u64)); rdx = (((rdx as u8)) as u32) as u32 as u64; rcx = (((rcx as u8)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; rax = rsi; rdx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (rcx as u32) } else { (rdx as u32) }) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(rdi).wrapping_add(0x4u64), ((rdx as u8)) as u8); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x208u64))) as u64; rcx = ((rcx).wrapping_add(1)) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; rbx = (((rcx) as i64).wrapping_mul((0x81u64) as i64) as u64) as u64; rcx = 0x90; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = ((crate::simd_ops::ru8(rbx.wrapping_add(r14).wrapping_add(0x121u64))) as u32) as u32 as u64; rcx = (((rdx as u8)) as u32) as u32 as u64; rdx = (rdx & !0xffu64) | ((((((rdx as u8)) | (0x2u8))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0xfdu8))) as u8 as u64)); rdx = (((rdx as u8)) as u32) as u32 as u64; rcx = (((rcx as u8)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; rax = rsi; rdx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (rcx as u32) } else { (rdx as u32) }) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(r14).wrapping_add(0x121u64), ((rdx as u8)) as u8); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x208u64))) as u64; rcx = ((rcx).wrapping_add(1)) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; rbx = (((rcx) as i64).wrapping_mul((0x81u64) as i64) as u64) as u64; rcx = 0x91; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = ((crate::simd_ops::ru8(rbx.wrapping_add(r14).wrapping_add(0x122u64))) as u32) as u32 as u64; rcx = (((rdx as u8)) as u32) as u32 as u64; rdx = (rdx & !0xffu64) | ((((((rdx as u8)) | (0x2u8))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0xfdu8))) as u8 as u64)); rdx = (((rdx as u8)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; rcx = (((rcx as u8)) as u32) as u32 as u64; rax = rsi; rdx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (rcx as u32) } else { (rdx as u32) }) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(r14).wrapping_add(0x122u64), ((rdx as u8)) as u8); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x208u64))) as u64; rcx = ((rcx).wrapping_add(1)) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; rax = (((rcx) as i64).wrapping_mul((0x81u64) as i64) as u64) as u64; rcx = rdi; let y_addr: u64 = rax.wrapping_add(rdi); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rax.wrapping_add(rdi).wrapping_add(0x20u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rax.wrapping_add(rdi).wrapping_add(0x40u64); xmm2 = crate::simd_ops::ru128(y_addr); ymm2_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rax.wrapping_add(rdi).wrapping_add(0x60u64); xmm3 = crate::simd_ops::ru128(y_addr); ymm3_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); rdx = ((crate::simd_ops::ru8(rax.wrapping_add(rdi).wrapping_add(0x80u64))) as u32) as u32 as u64; rax = rdi.wrapping_add(0x183); fl_l = rdi; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { bb = 7; continue; }
            7 => { let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rcx.wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); let y_addr: u64 = rcx.wrapping_add(0x40u64); crate::simd_ops::wu128(y_addr, xmm2); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm2_hi); let y_addr: u64 = rcx.wrapping_add(0x60u64); crate::simd_ops::wu128(y_addr, xmm3); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm3_hi); crate::simd_ops::wu8(rcx.wrapping_add(0x80u64), ((rdx as u8)) as u8); rcx = ((rcx).wrapping_add(0x81u64)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 8; continue; }
            8 => { rcx = rdi.wrapping_add(0x204); fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { let y_addr: u64 = rax; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rax.wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); let y_addr: u64 = rax.wrapping_add(0x40u64); crate::simd_ops::wu128(y_addr, xmm2); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm2_hi); let y_addr: u64 = rax.wrapping_add(0x60u64); crate::simd_ops::wu128(y_addr, xmm3); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm3_hi); crate::simd_ops::wu8(rax.wrapping_add(0x80u64), ((rdx as u8)) as u8); rax = ((rax).wrapping_add(0x81u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x70u64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 12; continue; } bb = 11; continue; }
            11 => { r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r14.wrapping_add(0x3acu64), (0x8u64) as u64); crate::simd_ops::wu64(r14.wrapping_add(0x3b4u64), (0x8u64) as u64); r8 = r14.wrapping_add(0x3ac); crate::simd_ops::wu64(r14.wrapping_add(0x3bcu64), (0x18u64) as u64); rdx = ((r9.wrapping_add(0x8u64)) as u32) as u32 as u64; rcx = ((rdx.wrapping_add(0x32u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = 0; fl_l = 0; fl_r = 0; r8 = r14.wrapping_add(0x3b4); rdx = ((r9.wrapping_add(0x8u64)) as u32) as u32 as u64; rcx = ((rdx.wrapping_add(0x2cu64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = 0; fl_l = 0; fl_r = 0; r8 = r14.wrapping_add(0x3bc); rdx = ((r9.wrapping_add(0x18u64)) as u32) as u32 as u64; rcx = ((rdx.wrapping_add(0x1au64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x3acu64))) as u64; r8 = rsp.wrapping_add(0x88); crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rax); r9 = 0; fl_l = 0; fl_r = 0; rax = ((rax).wrapping_shr(32u32)) as u64; rax = ((((rax as u32)) & (0xfffffff3u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x8cu64), ((rax as u32)) as u32); rdx = ((r9.wrapping_add(0x8u64)) as u32) as u32 as u64; rcx = ((rdx.wrapping_add(0x33u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x3b4u64))) as u64; r8 = rsp.wrapping_add(0x90); crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rax); r9 = 0; fl_l = 0; fl_r = 0; rax = ((rax).wrapping_shr(32u32)) as u64; rax = ((((rax as u32)) & (0xfffffff3u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x94u64), ((rax as u32)) as u32); rdx = ((r9.wrapping_add(0x8u64)) as u32) as u32 as u64; rcx = ((rdx.wrapping_add(0x2du64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm1 = (crate::simd_ops::ru128(r14.wrapping_add(0x3bcu64))) as u128; ymm1_hi = 0; xmm0 = ((crate::simd_ops::ru64(r14.wrapping_add(0x3ccu64))) as u64) as u128; ymm0_hi = 0; r9 = 0; fl_l = 0; fl_r = 0; r8 = rsp.wrapping_add(0x30); rax = ((xmm1 as u64)) as u64; rax = ((rax).wrapping_shr(32u32)) as u64; rax = ((((rax as u32)) & (0xfffffff3u32))) as u32 as u64; rdx = ((r9.wrapping_add(0x18u64)) as u32) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm1) as u128); rcx = ((rdx.wrapping_add(0x1bu64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), ((xmm0 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(r14.wrapping_add(0x3a8u64), (0x1u8) as u8); bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::lk_add_u32(rbp.wrapping_add(0xcu64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; if (((fl_l as u32)) != 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = 0xffffffff; rcx = rbp; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rbp.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 14; continue; }
            14 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; bb = 15; continue; }
            15 => { crate::simd_ops::wu8(r15.wrapping_add(0x4u64), (0x1u8) as u8); rax = r15; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cf750(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1420cfae0; const P1: u64 = 0x1420cf930; const P2: u64 = 0x10; const P3: u64 = 0x88; const P4: u64 = 0x142645790; const P5: u64 = 0x8; const P6: u64 = 0x1426455a8; const P7: u64 = 0xc; const P8: u64 = 0x2c0; const P9: u64 = 0xb7; const P10: u64 = 0x142645598;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdi = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 11; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rbx = ((rdi.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64; rsi = ((rax as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbx.wrapping_add(P5), ((rsi as u32)) as u32); bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(P7), (1) as u32)) as u32 as u64; r8 = 0x1; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r9 = 0x81; r10 = (0xaaaaaaaaaaaaaaabu64) as u64; bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P8))) as u64; rax = r10; rcx = ((rcx).wrapping_add(1)) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; rax = (((rcx) as i64).wrapping_mul((0x81u64) as i64) as u64) as u64; rax = ((rax).wrapping_add(r8)) as u64; r8 = ((r8).wrapping_add(1)) as u64; crate::simd_ops::lk_and_u8(rax.wrapping_add(rdi).wrapping_add(P9), (0xfeu8) as u8); fl_l = r9; fl_r = (0x1u64) as u64; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(P7), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = 0xffffffff; rcx = rbx; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rbx.wrapping_add(P5), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cf850(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x88; const P2: u64 = 0x142645790; const P3: u64 = 0x8; const P4: u64 = 0x1426455a8; const P5: u64 = 0xc; const P6: u64 = 0x2c0; const P7: u64 = 0xb7; const P8: u64 = 0x142645598;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = 0x1; rsi = rcx; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 9; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = ((rcx.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; rdi = ((rax as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbx.wrapping_add(P3), ((rdi as u32)) as u32); bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(P5), (1) as u32)) as u32 as u64; r8 = 0x1; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r9 = 0x81; r10 = (0xaaaaaaaaaaaaaaabu64) as u64; bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P6))) as u64; rax = r10; rcx = ((rcx).wrapping_add(1)) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; rax = (((rcx) as i64).wrapping_mul((0x81u64) as i64) as u64) as u64; rax = ((rax).wrapping_add(r8)) as u64; r8 = ((r8).wrapping_add(1)) as u64; crate::simd_ops::lk_and_u8(rax.wrapping_add(rsi).wrapping_add(P7), (0xfeu8) as u8); fl_l = r9; fl_r = (0x1u64) as u64; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(P5), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = 0xffffffff; rcx = rbx; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rbx.wrapping_add(P3), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            9 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cf930(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645790; const P1: u64 = 0x1426455a8; const P2: u64 = 0x1429822c0; const P3: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = 0x8; rsi = rdx; rdi = rcx; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 23; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x1cu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 23; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = rcx.wrapping_add(0x88); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rbp = ((rax as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rbp as u32)) as u32); bb = 4; continue; }
            4 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(0xcu64), (1) as u32); rdx = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x100; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rcx = P2; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0xfffffffe; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { fl_l = ((rcx).wrapping_shr(32u32)) as u64; rcx = ((fl_l as u64)) as u64; bb = 8; continue; }
            7 => { rcx = (((rdx as u16)) as u32) as u32 as u64; bb = 8; continue; }
            8 => { rax = ((crate::simd_ops::ru16(rsi.wrapping_add(0x14u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x38u64), ((rcx as u16)) as u16); rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; fl_l = ((rcx as u16)) as u16 as u64; fl_r = (0x7eu16) as u16 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x2c0u64))) as u64; rdx = (if (((fl_l as u16)) <= ((fl_r as u16))) { (rax as u32) } else { (rdx as u32) }) as u32 as u64; rax = (0xaaaaaaaaaaaaaaabu64) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x3au64), ((rdx as u8)) as u8); let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; rdx = (((rcx) as i64).wrapping_mul((0x81u64) as i64) as u64) as u64; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(rdx).wrapping_add(0x125u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(rdx).wrapping_add(0x126u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 12; continue; }
            11 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(rdx).wrapping_add(0x127u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 15; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(rdx).wrapping_add(0x128u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); bb = 16; continue; }
            15 => { fl_l = ((((rax as u8)) | (0x4u8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(rdx).wrapping_add(0x123u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 19; continue; } bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(rdx).wrapping_add(0x124u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { fl_l = ((((rax as u8)) & (0xfdu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); bb = 20; continue; }
            19 => { fl_l = ((((rax as u8)) | (0x2u8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); bb = 20; continue; }
            20 => { r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x2d0u64))) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x3bu64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x2d8u64))) as u64; rcx = r8.wrapping_sub(0x31); fl_l = rax; fl_r = rcx; rcx = (if (((fl_l as u64)) > ((fl_r as u64))) { rax } else { rcx }) as u64; rax = r8.wrapping_add(0x1); crate::simd_ops::wu64(rdi.wrapping_add(0x2d0u64), rax); rax = (0x47ae147ae147ae15u64) as u64; let prod = (rax as u128).wrapping_mul((r8) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rax = r8; crate::simd_ops::wu64(rdi.wrapping_add(0x2d8u64), rcx); rax = ((rax).wrapping_sub(rdx)) as u64; rax = ((rax).wrapping_shr(1u32)) as u64; rax = ((rax).wrapping_add(rdx)) as u64; rax = ((rax).wrapping_shr(5u32)) as u64; rax = (((rax) as i64).wrapping_mul((0x32u64) as i64) as u64) as u64; r8 = ((r8).wrapping_sub(rax)) as u64; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x38u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(r8.wrapping_mul(4)).wrapping_add(0x2e0u64), ((rax as u32)) as u32); fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(0xcu64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { rax = 0xffffffff; rcx = rbx; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 22; continue; }
            22 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            23 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cfae0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645790; const P1: u64 = 0x1426455a8; const P2: u64 = 0x1420cff40; const P3: u64 = 0x142645598;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = 0x10; rdi = rdx; rbp = rcx; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = rcx.wrapping_add(0x88); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rsi = ((rax as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rsi as u32)) as u32); bb = 4; continue; }
            4 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(0xcu64), (1) as u32); rdx = ((crate::simd_ops::ru16(rdi.wrapping_add(0x1eu64))) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru16(rdi.wrapping_add(0x22u64))) as u32) as u32 as u64; r8 = ((rdx as u32)) as u32 as u64; r9 = ((crate::simd_ops::ru16(rdi.wrapping_add(0x1cu64))) as u32) as u32 as u64; rdi = (((rdx as u8)) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_shr(2u32)) as u32 as u64; rdi = (rdi & !0xffu64) | (((!((rdi as u8))) as u8 as u64)); rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; r8 = (r8 & !0xffu64) | ((((((r8 as u8)) & (0x1u8))) as u8 as u64)); rdx = (rdx & !0xffu64) | ((((((rdx as u8)) & (0x1u8))) as u8 as u64)); rdi = (rdi & !0xffu64) | ((((((rdi as u8)) & (0x1u8))) as u8 as u64)); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r11 = (0xaaaaaaaaaaaaaaabu64) as u64; r10 = (((rax as u16)) as u32) as u32 as u64; fl_l = ((r10 as u16)) as u16 as u64; fl_r = ((r10 as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) == 0) { bb = 11; continue; } bb = 5; continue; }
            5 => { r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x2c0u64))) as u64; rax = r11; r8 = ((r8).wrapping_add(1)) as u64; let prod = (rax as u128).wrapping_mul((r8) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_sub(rcx)) as u64; rcx = ((r10 as u32)) as u32 as u64; rax = (((r8) as i64).wrapping_mul((0x81u64) as i64) as u64) as u64; rax = ((rax).wrapping_add(rbp)) as u64; rdx = ((crate::simd_ops::ru8(r10.wrapping_add(rax).wrapping_add(0xb7u64))) as u32) as u32 as u64; r9 = (((rdx as u8)) as u32) as u32 as u64; r9 = (r9 & !0xffu64) | (((((r9 as u8)).wrapping_shr(1u32)) as u8 as u64)); r9 = (r9 & !0xffu64) | ((((((r9 as u8)) & (0x1u8))) as u8 as u64)); fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x5; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x65; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 8; continue; }
            8 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { fl_l = ((((r9 as u8)) ^ (0x1u8))) as u8 as u64; fl_r = fl_l; r9 = (r9 & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); bb = 12; continue; }
            11 => { r9 = ((crate::simd_ops::ru8(rsp.wrapping_add(0x38u64))) as u32) as u32 as u64; bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x2c0u64))) as u64; rax = r11; rcx = ((rcx).wrapping_add(1)) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; rax = (((rcx) as i64).wrapping_mul((0x81u64) as i64) as u64) as u64; rax = ((rax).wrapping_add(r10)) as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(rbp).wrapping_add(0xb7u64))) as u32) as u32 as u64; rcx = (((rdx as u8)) as u32) as u32 as u64; rdx = (rdx & !0xffu64) | ((((((rdx as u8)) | (0x1u8))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0xfeu8))) as u8 as u64)); rdx = (((rdx as u8)) as u32) as u32 as u64; rcx = (((rcx as u8)) as u32) as u32 as u64; fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; rdx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (rcx as u32) } else { (rdx as u32) }) as u32 as u64; crate::simd_ops::wu8(rax.wrapping_add(rbp).wrapping_add(0xb7u64), ((rdx as u8)) as u8); rax = r11; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x2c0u64))) as u64; rcx = ((rcx).wrapping_add(1)) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; r8 = (((rcx) as i64).wrapping_mul((0x81u64) as i64) as u64) as u64; r8 = ((r8).wrapping_add(r10)) as u64; rdx = ((crate::simd_ops::ru8(r8.wrapping_add(rbp).wrapping_add(0xb7u64))) as u32) as u32 as u64; rax = (((rdx as u8)) as u32) as u32 as u64; rdx = (rdx & !0xffu64) | ((((((rdx as u8)) | (0x2u8))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xfdu8))) as u8 as u64)); rdx = (((rdx as u8)) as u32) as u32 as u64; fl_l = ((r9 as u8)) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rdx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (rax as u32) } else { (rdx as u32) }) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(rbp).wrapping_add(0xb7u64), ((rdx as u8)) as u8); fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(0xcu64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = 0xffffffff; rcx = rbx; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            15 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cfcb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x328; const P1: u64 = 0x32c; const P2: u64 = 0x8; const P3: u64 = 0x33; const P4: u64 = 0x1426462a0; const P5: u64 = 0x334; const P6: u64 = 0x2d; const P7: u64 = 0x33c; const P8: u64 = 0x18; const P9: u64 = 0x1b;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    rbx = rcx;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { r9 = 0; fl_l = 0; fl_r = 0; r8 = ((rcx.wrapping_add(P1)) as u64) as u64; rdx = ((r9.wrapping_add(P2)) as u32) as u32 as u64; rcx = ((rdx.wrapping_add(P3)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); r9 = 0; fl_l = 0; fl_r = 0; r8 = ((rbx.wrapping_add(P5)) as u64) as u64; rdx = ((r9.wrapping_add(P2)) as u32) as u32 as u64; rcx = ((rdx.wrapping_add(P6)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); r9 = 0; fl_l = 0; fl_r = 0; r8 = ((rbx.wrapping_add(P7)) as u64) as u64; rdx = ((r9.wrapping_add(P8)) as u32) as u32 as u64; rcx = ((rdx.wrapping_add(P9)) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cfd20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x32c; const P1: u64 = 0x334; const P2: u64 = 0x33c; const P3: u64 = 0x8; const P4: u64 = 0x32; const P5: u64 = 0x1426462a0; const P6: u64 = 0x2c; const P7: u64 = 0x18; const P8: u64 = 0x1a; const P9: u64 = 0x33; const P10: u64 = 0x2d; const P11: u64 = 0x1b;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx.wrapping_add(P0), (0x8u64) as u64); rbp = rcx; crate::simd_ops::wu64(rcx.wrapping_add(P1), (0x8u64) as u64); crate::simd_ops::wu64(rcx.wrapping_add(P2), (0x18u64) as u64); r8 = ((rcx.wrapping_add(P0)) as u64) as u64; rdx = ((r9.wrapping_add(P3)) as u32) as u32 as u64; rcx = ((rdx.wrapping_add(P4)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = 0; fl_l = 0; fl_r = 0; r8 = rbp.wrapping_add(0x334); rdx = ((r9.wrapping_add(P3)) as u32) as u32 as u64; rcx = ((rdx.wrapping_add(P6)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = 0; fl_l = 0; fl_r = 0; r8 = rbp.wrapping_add(0x33c); rdx = ((r9.wrapping_add(P7)) as u32) as u32 as u64; rcx = ((rdx.wrapping_add(P8)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x32cu64))) as u64; r8 = rsp.wrapping_add(0x60); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); r9 = 0; fl_l = 0; fl_r = 0; rax = ((rax).wrapping_shr(32u32)) as u64; rax = ((((rax as u32)) & (0xfffffff3u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x64u64), ((rax as u32)) as u32); rdx = ((r9.wrapping_add(P3)) as u32) as u32 as u64; rcx = ((rdx.wrapping_add(P9)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x334u64))) as u64; r8 = rsp.wrapping_add(0x68); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rax); r9 = 0; fl_l = 0; fl_r = 0; rax = ((rax).wrapping_shr(32u32)) as u64; rax = ((((rax as u32)) & (0xfffffff3u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x6cu64), ((rax as u32)) as u32); rdx = ((r9.wrapping_add(P3)) as u32) as u32 as u64; rcx = ((rdx.wrapping_add(P10)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm1 = (crate::simd_ops::ru128(rbp.wrapping_add(0x33cu64))) as u128; xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_add(0x34cu64))) as u64) as u128; r9 = 0; fl_l = 0; fl_r = 0; r8 = rsp.wrapping_add(0x20); rax = ((xmm1 as u64)) as u64; rax = ((rax).wrapping_shr(32u32)) as u64; rax = ((((rax as u32)) & (0xfffffff3u32))) as u32 as u64; rdx = ((r9.wrapping_add(P7)) as u32) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm1) as u128); rcx = ((rdx.wrapping_add(P11)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm0 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu8(rbp.wrapping_add(0x328u64), (0x1u8) as u8); rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cfe50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1a; const P1: u64 = 0x18; const P2: u64 = 0x1e; const P3: u64 = 0x1420cff40; const P4: u64 = 0x240; const P5: u64 = 0x37;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r10 = rdx; rbx = r8; rdx = ((crate::simd_ops::ru16(rdx.wrapping_add(P0))) as u32) as u32 as u64; rsi = r9; rax = (((rdx as u8)) as u32) as u32 as u64; r8 = ((rdx as u32)) as u32 as u64; rax = (rax & !0xffu64) | (((!((rax as u8))) as u8 as u64)); r8 = (((r8 as u32)).wrapping_shr(2u32)) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x1u8))) as u8 as u64)); rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; crate::simd_ops::wu8(r9, ((rax as u8)) as u8); rdi = rcx; r9 = ((crate::simd_ops::ru16(r10.wrapping_add(P1))) as u32) as u32 as u64; r8 = (r8 & !0xffu64) | ((((((r8 as u8)) & (0x1u8))) as u8 as u64)); rcx = ((crate::simd_ops::ru16(r10.wrapping_add(P2))) as u32) as u32 as u64; rdx = (rdx & !0xffu64) | ((((((rdx as u8)) & (0x1u8))) as u8 as u64)); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r10 = (((rax as u16)) as u32) as u32 as u64; crate::simd_ops::wu16(rbx, ((r10 as u16)) as u16); fl_l = ((r10 as u16)) as u16 as u64; fl_r = ((r10 as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; rax = (0xaaaaaaaaaaaaaaabu64) as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rcx = ((rcx).wrapping_add(1)) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; r9 = (((rcx) as i64).wrapping_mul((0x81u64) as i64) as u64) as u64; rcx = ((r10 as u32)) as u32 as u64; r9 = ((r9).wrapping_add(r10)) as u64; rdx = ((crate::simd_ops::ru8(r9.wrapping_add(rdi).wrapping_add(P5))) as u32) as u32 as u64; rdx = (rdx & !0xffu64) | (((((rdx as u8)).wrapping_shr(1u32)) as u8 as u64)); rdx = (rdx & !0xffu64) | ((((((rdx as u8)) & (0x1u8))) as u8 as u64)); crate::simd_ops::wu8(r8, ((rdx as u8)) as u8); fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x5; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x65; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); fl_l = (crate::simd_ops::ru8(r9.wrapping_add(rdi).wrapping_add(P5))) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { rdx = (rdx & !0xffu64) | ((((((rdx as u8)) ^ ((rax as u8)))) as u8 as u64)); crate::simd_ops::wu8(r8, ((rdx as u8)) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420cff40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646140; const P1: u64 = 0x140000000;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (((rdx as u8)) as u32) as u32 as u64; rsi = (((r8 as u8)) as u32) as u32 as u64; rdi = ((rcx as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0xff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdx = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rax = (((rdx as u16)) as u32) as u32 as u64; bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x10; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = ((rcx.wrapping_sub(0xdu64)) as u32) as u32 as u64; rcx = ((r9 as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rdi = ((rax as u32)) as u32 as u64; bb = 8; continue; }
            6 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x90; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x13; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdx = 0; fl_l = 0; fl_r = 0; rcx = ((rdi as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rax = ((rdi.wrapping_sub(0xcu64)) as u32) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; r8 = P1; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x22; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 28; continue; } bb = 12; continue; }
            12 => { rax = ((crate::simd_ops::ru8(r8.wrapping_add(rax).wrapping_add(0x20d00d0u64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x20d0094u64))) as u32 as u64; fl_l = ((rcx).wrapping_add(r8)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; j_tgt = rcx; bb = 0xffff_fffe; continue; }
            13 => { rax = ((rbx.wrapping_add(0x6eu64)) as u32) as u32 as u64; bb = 27; continue; }
            14 => { rax = ((rbx.wrapping_add(0x70u64)) as u32) as u32 as u64; bb = 27; continue; }
            15 => { fl_l = 0; fl_r = ((rbx as u8)) as u8 as u64; rbx = (rbx & !0xffu64) | (((0u8.wrapping_sub(fl_r as u8)) as u8 as u64)); rax = (rax & !0xffffu64) | (((if (((fl_l as u8)) < ((fl_r as u8))) { 0xffffu16 } else { 0u16 }) as u16 as u64)); fl_l = ((((rax as u16)) & (0x42u16))) as u16 as u64; fl_r = fl_l; rax = (rax & !0xffffu64) | ((((fl_l as u16)) as u16 as u64)); bb = 27; continue; }
            16 => { rax = 0x38; bb = 26; continue; }
            17 => { rax = 0x47; bb = 26; continue; }
            18 => { rax = 0x3f; bb = 26; continue; }
            19 => { rax = 0x39; bb = 26; continue; }
            20 => { rax = 0x41; bb = 26; continue; }
            21 => { rax = 0x3b; bb = 26; continue; }
            22 => { rax = 0x3c; bb = 26; continue; }
            23 => { rax = 0x3e; bb = 26; continue; }
            24 => { rbx = (rbx & !0xffu64) | ((((((rbx as u8)) ^ (0x1u8))) as u8 as u64)); rax = (((rbx as u8)) as u32) as u32 as u64; fl_l = (((rax as u16)).wrapping_shl(6u32)) as u16 as u64; rax = (rax & !0xffffu64) | ((((fl_l as u16)) as u16 as u64)); bb = 27; continue; }
            25 => { rax = 0x3a; bb = 26; continue; }
            26 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; rax = (rax & !0xffffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (rdx as u16) } else { (rax as u16) }) as u16 as u64)); bb = 27; continue; }
            27 => { fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) != 0) { bb = 3; continue; } bb = 28; continue; }
            28 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x100; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } bb = 29; continue; }
            29 => { rax = ((rdi as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x29822c0u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xfffffffe; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 30; continue; }
            30 => { rax = ((rax).wrapping_shr(32u32)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            31 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 29; continue; } bb = 32; continue; }
            32 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0x3d; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x1420cff40u64) { 0x20 => { bb = 1; continue; } 0x22 => { bb = 2; continue; } 0x25 => { bb = 3; continue; } 0x35 => { bb = 4; continue; } 0x3a => { bb = 5; continue; } 0x4a => { bb = 6; continue; } 0x52 => { bb = 7; continue; } 0x5a => { bb = 8; continue; } 0x5f => { bb = 9; continue; } 0x64 => { bb = 10; continue; } 0x6e => { bb = 11; continue; } 0x83 => { bb = 12; continue; } 0x99 => { bb = 13; continue; } 0x9e => { bb = 14; continue; } 0xa3 => { bb = 15; continue; } 0xae => { bb = 16; continue; } 0xb5 => { bb = 17; continue; } 0xbc => { bb = 18; continue; } 0xc3 => { bb = 19; continue; } 0xca => { bb = 20; continue; } 0xd1 => { bb = 21; continue; } 0xd8 => { bb = 22; continue; } 0xdf => { bb = 23; continue; } 0xe6 => { bb = 24; continue; } 0xf2 => { bb = 25; continue; } 0xf7 => { bb = 26; continue; } 0xfd => { bb = 27; continue; } 0x106 => { bb = 28; continue; } 0x112 => { bb = 29; continue; } 0x125 => { bb = 30; continue; } 0x139 => { bb = 31; continue; } 0x13d => { bb = 32; continue; } _ => { return crate::simd_ops::tail_stk(j_tgt, rax, rcx, rdx, r8, r9, rsp); } } }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d0100(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645790; const P1: u64 = 0x1426455a8; const P2: u64 = 0x140000000; const P3: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (((r8 as u16)) as u32) as u32 as u64; rbp = rdx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x10u64))) as u32 as u64; rsi = ((rax as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = rbp.wrapping_add(0x8); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((rsi as u32)) as u32); bb = 2; continue; }
            2 => { crate::simd_ops::lk_add_u32(rbp.wrapping_add(0x14u64), (1) as u32); rax = (0xaaaaaaaaaaaaaaabu64) as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(0xc0u64))) as u64; rcx = ((rcx).wrapping_add(1)) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; rax = ((rdi.wrapping_sub(0x95u64)) as u32) as u32 as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1b; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 13; continue; } bb = 3; continue; }
            3 => { r8 = P2; rax = (rax as u32 as i32 as i64) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(rax).wrapping_add(0x20d022cu64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x20d0204u64))) as u32 as u64; fl_l = ((rcx).wrapping_add(r8)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; j_tgt = rcx; bb = 0xffff_fffe; continue; }
            4 => { crate::simd_ops::wu8(rdi.wrapping_add(rdx).wrapping_sub(0x95u64), (0x0u8) as u8); bb = 13; continue; }
            5 => { crate::simd_ops::wu8(rdx.wrapping_add(0x25u64), (0x0u8) as u8); bb = 13; continue; }
            6 => { crate::simd_ops::wu8(rdx.wrapping_add(0x26u64), (0x0u8) as u8); bb = 13; continue; }
            7 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdx.wrapping_add(0x28u64), ((rax as u32)) as u32); bb = 13; continue; }
            8 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu16(rdx.wrapping_add(0x28u64), ((rax as u16)) as u16); bb = 13; continue; }
            9 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu16(rdx.wrapping_add(0x2au64), ((rax as u16)) as u16); bb = 13; continue; }
            10 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdx.wrapping_add(0x2cu64), ((rax as u32)) as u32); bb = 13; continue; }
            11 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu16(rdx.wrapping_add(0x2cu64), ((rax as u16)) as u16); bb = 13; continue; }
            12 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu16(rdx.wrapping_add(0x2eu64), ((rax as u16)) as u16); bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::lk_add_u32(rbp.wrapping_add(0x14u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = 0xffffffff; rcx = rbp.wrapping_add(0x8); let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rbp.wrapping_add(0x10u64), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x1420d0100u64) { 0x2a => { bb = 1; continue; } 0x37 => { bb = 2; continue; } 0x75 => { bb = 3; continue; } 0x94 => { bb = 4; continue; } 0x9e => { bb = 5; continue; } 0xa4 => { bb = 6; continue; } 0xaa => { bb = 7; continue; } 0xb1 => { bb = 8; continue; } 0xb9 => { bb = 9; continue; } 0xc1 => { bb = 10; continue; } 0xc8 => { bb = 11; continue; } 0xd0 => { bb = 12; continue; } 0xd6 => { bb = 13; continue; } 0xdc => { bb = 14; continue; } 0xee => { bb = 15; continue; } _ => { return crate::simd_ops::tail_stk(j_tgt, rax, rcx, rdx, r8, r9, rsp); } } }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d0250(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645790; const P1: u64 = 0x1426455a8; const P2: u64 = 0x1434268d0; const P3: u64 = 0x143427080; const P4: u64 = 0x1434264e4; const P5: u64 = 0x143426ea4; const P6: u64 = 0x143426f84; const P7: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbp); rbx = rdx.wrapping_add(0x8); crate::simd_ops::wu64(rax.wrapping_sub(0x10u64), rsi); rsi = r9; crate::simd_ops::wu64(rax.wrapping_sub(0x18u64), rdi); crate::simd_ops::wu64(rax.wrapping_sub(0x20u64), r14); r14 = rdx; crate::simd_ops::wu64(rax.wrapping_sub(0x28u64), r15); r15 = rcx; rdi = (((r8 as u16)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rbp = ((rax as u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rbp as u32)) as u32); bb = 2; continue; }
            2 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(0xcu64), (1) as u32); rax = (0xaaaaaaaaaaaaaaabu64) as u64; r8 = (crate::simd_ops::ru64(r14.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm7) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm8) as u128); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(0xc0u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rcx = ((rcx).wrapping_add(1)) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = ((rdi as u32)) as u32 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdx = ((rdx).wrapping_add(rdx)) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x95; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 4; continue; }
            4 => { xmm8 = ((crate::simd_ops::ru32(P2)) as u32) as u128; rax = ((crate::simd_ops::ru8(r15.wrapping_add(0x40u64))) as u32) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; xmm1 = ((crate::simd_ops::ru32(r15.wrapping_add(0x38u64))) as u32) as u128; xmm7 = ((crate::simd_ops::ru32(P3)) as u32) as u128; xmm2 = ((crate::simd_ops::ru32(P4)) as u32) as u128; xmm3 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(xmm2 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm2 as u32) }).to_bits() as u128); xmm2 = ((crate::simd_ops::ru32(P5)) as u32) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) < f32::from_bits(xmm2 as u32) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(xmm2 as u32) }).to_bits() as u128); xmm0 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) / f32::from_bits(xmm3 as u32)).to_bits() as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rsi); xmm3 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x88u64)))).to_bits() as u128); xmm0 = (((rax as u32)) as u32) as u128; rax = ((crate::simd_ops::ru8(r15.wrapping_add(0x4cu64))) as u32) as u32 as u64; xmm1 = (((rcx as u32)) as u32) as u128; xmm2 = crate::simd_ops::vpcmpeqd(xmm0, xmm1); xmm0 = crate::simd_ops::vblendvps(xmm7, xmm8, xmm2); xmm4 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm3 = ((crate::simd_ops::ru32(r15.wrapping_add(0x44u64))) as u32) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) > f32::from_bits(crate::simd_ops::ru32(P4)) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P4)) }).to_bits() as u128); xmm1 = 0u128; xmm1 = ((crate::simd_ops::ru32(P5)) as u32) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) < f32::from_bits(xmm1 as u32) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(xmm1 as u32) }).to_bits() as u128); xmm2 = (xmm1) as u128; xmm0 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) / f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm3 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x8cu64)))).to_bits() as u128); xmm0 = (((rax as u32)) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm4 as u32)) as u32); xmm1 = (((rcx as u32)) as u32) as u128; xmm2 = crate::simd_ops::vpcmpeqd(xmm0, xmm1); xmm0 = crate::simd_ops::vblendvps(xmm7, xmm8, xmm2); xmm1 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((xmm1 as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rax); xmm1 = (xmm7) as u128; xmm5 = (xmm1) as u128; xmm1 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x8cu64))) as u32) as u128; xmm4 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(xmm5 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm5 as u32) }).to_bits() as u128); xmm1 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x88u64))) as u32) as u128; xmm2 = (xmm8) as u128; xmm3 = (xmm2) as u128; xmm2 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(xmm5 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm5 as u32) }).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) < f32::from_bits(xmm3 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm3 as u32) }).to_bits() as u128); xmm0 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(crate::simd_ops::ru32(P6))).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x88u64), ((xmm0 as u32)) as u32); xmm4 = (xmm4 & !0xffff_ffffu128) | ((if f32::from_bits(xmm4 as u32) < f32::from_bits(xmm3 as u32) { f32::from_bits(xmm4 as u32) } else { f32::from_bits(xmm3 as u32) }).to_bits() as u128); xmm1 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(crate::simd_ops::ru32(P6))).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x8cu64), ((xmm1 as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rax); rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x88u64)))))) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x90u64), ((rax as u16)) as u16); rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x8cu64)))))) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x92u64), ((rax as u16)) as u16); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x90u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x2cu64), ((rax as u32)) as u32); bb = 7; continue; }
            5 => { bb = 6; continue; }
            6 => { xmm8 = ((crate::simd_ops::ru32(P2)) as u32) as u128; rax = ((crate::simd_ops::ru8(r15.wrapping_add(0x28u64))) as u32) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; xmm1 = ((crate::simd_ops::ru32(r15.wrapping_add(0x20u64))) as u32) as u128; xmm7 = ((crate::simd_ops::ru32(P3)) as u32) as u128; xmm2 = ((crate::simd_ops::ru32(P4)) as u32) as u128; xmm3 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(xmm2 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm2 as u32) }).to_bits() as u128); xmm2 = ((crate::simd_ops::ru32(P5)) as u32) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) < f32::from_bits(xmm2 as u32) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(xmm2 as u32) }).to_bits() as u128); xmm0 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) / f32::from_bits(xmm3 as u32)).to_bits() as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rsi); xmm3 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x88u64)))).to_bits() as u128); xmm0 = (((rax as u32)) as u32) as u128; rax = ((crate::simd_ops::ru8(r15.wrapping_add(0x34u64))) as u32) as u32 as u64; xmm1 = (((rcx as u32)) as u32) as u128; xmm2 = crate::simd_ops::vpcmpeqd(xmm0, xmm1); xmm0 = crate::simd_ops::vblendvps(xmm7, xmm8, xmm2); xmm4 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm3 = ((crate::simd_ops::ru32(r15.wrapping_add(0x2cu64))) as u32) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) > f32::from_bits(crate::simd_ops::ru32(P4)) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P4)) }).to_bits() as u128); xmm1 = 0u128; xmm1 = ((crate::simd_ops::ru32(P5)) as u32) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) < f32::from_bits(xmm1 as u32) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(xmm1 as u32) }).to_bits() as u128); xmm2 = (xmm1) as u128; xmm0 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) / f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm3 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x8cu64)))).to_bits() as u128); xmm0 = (((rax as u32)) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((xmm4 as u32)) as u32); xmm1 = (((rcx as u32)) as u32) as u128; xmm2 = crate::simd_ops::vpcmpeqd(xmm0, xmm1); xmm0 = crate::simd_ops::vblendvps(xmm7, xmm8, xmm2); xmm1 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), ((xmm1 as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rax); xmm1 = (xmm7) as u128; xmm5 = (xmm1) as u128; xmm1 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x8cu64))) as u32) as u128; xmm4 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(xmm5 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm5 as u32) }).to_bits() as u128); xmm1 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x88u64))) as u32) as u128; xmm2 = (xmm8) as u128; xmm3 = (xmm2) as u128; xmm2 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(xmm5 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm5 as u32) }).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) < f32::from_bits(xmm3 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm3 as u32) }).to_bits() as u128); xmm0 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(crate::simd_ops::ru32(P6))).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x88u64), ((xmm0 as u32)) as u32); xmm4 = (xmm4 & !0xffff_ffffu128) | ((if f32::from_bits(xmm4 as u32) < f32::from_bits(xmm3 as u32) { f32::from_bits(xmm4 as u32) } else { f32::from_bits(xmm3 as u32) }).to_bits() as u128); xmm1 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(crate::simd_ops::ru32(P6))).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x8cu64), ((xmm1 as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rax); rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x88u64)))))) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x90u64), ((rax as u16)) as u16); rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x8cu64)))))) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x92u64), ((rax as u16)) as u16); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x90u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x28u64), ((rax as u32)) as u32); bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(0xcu64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; if (((fl_l as u32)) != 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = 0xffffffff; rcx = rbx; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; rsp = ((rsp).wrapping_add(0x70u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            9 => { rsp = ((rsp).wrapping_add(0x70u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d0600(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645790; const P1: u64 = 0x1426455a8; const P2: u64 = 0x140000000; const P3: u64 = 0x1434268d0; const P4: u64 = 0x143426eec; const P5: u64 = 0x1420d3b80; const P6: u64 = 0x143427080; const P7: u64 = 0x143426f84; const P8: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); rbp = (rdx) as u64; xmm6 = (xmm3) as u128; rdi = (((r8 as u16)) as u32) as u32 as u64; r14 = (rcx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = (crate::simd_ops::ru32(rbp.wrapping_add(0x10u64))) as u32 as u64; rsi = ((rax as u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rbp.wrapping_add(0x8u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((rsi as u32)) as u32); bb = 2; continue; }
            2 => { crate::simd_ops::lk_add_u32(rbp.wrapping_add(0x14u64), (1) as u32); rax = (0xaaaaaaaaaaaaaaabu64) as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(0xc0u64))) as u64; rcx = ((rcx).wrapping_add(1)) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; rax = ((rdi.wrapping_sub(0x95u64)) as u32) as u32 as u64; r9 = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; r9 = ((r9).wrapping_shl(4u32)) as u64; r9 = ((r9).wrapping_add(r8)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1bu32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 15; continue; } bb = 3; continue; }
            3 => { rdx = ((P2) as u64) as u64; rax = (rax as u32 as i32 as i64) as u64; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rax).wrapping_add(0x20d0874u64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x20d0854u64))) as u32 as u64; fl_l = ((rcx).wrapping_add(rdx)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; j_tgt = (rcx) as u64; bb = 0xffff_fffe; continue; }
            4 => { crate::simd_ops::wu8(rdi.wrapping_add(r9).wrapping_sub(0x95u64), (0x1u8) as u8); bb = 15; continue; }
            5 => { xmm0 = 0u128; xmm3 = (xmm0 & !0xffff_ffffu128) | (xmm6 & 0xffff_ffffu128); xmm2 = 0u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) > f32::from_bits(xmm2 as u32) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(xmm2 as u32) }).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) < f32::from_bits(core::ptr::read_unaligned((P3) as *const u32)) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(core::ptr::read_unaligned((P3) as *const u32)) }).to_bits() as u128); xmm0 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(core::ptr::read_unaligned((P4) as *const u32))).to_bits() as u128); rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm0 as u32)))) as u32 as u64; crate::simd_ops::wu8(r9.wrapping_add(0x25u64), ((rax as u8)) as u8); bb = 15; continue; }
            6 => { xmm0 = 0u128; xmm3 = (xmm0 & !0xffff_ffffu128) | (xmm6 & 0xffff_ffffu128); xmm2 = 0u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) > f32::from_bits(xmm2 as u32) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(xmm2 as u32) }).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) < f32::from_bits(core::ptr::read_unaligned((P3) as *const u32)) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(core::ptr::read_unaligned((P3) as *const u32)) }).to_bits() as u128); xmm0 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(core::ptr::read_unaligned((P4) as *const u32))).to_bits() as u128); rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm0 as u32)))) as u32 as u64; crate::simd_ops::wu8(r9.wrapping_add(0x26u64), ((rax as u8)) as u8); bb = 15; continue; }
            7 => { bb = 8; continue; }
            8 => { rdx = ((r14.wrapping_add(0x20u64)) as u64) as u64; r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); xmm0 = (xmm6) as u128; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm1 = 0u128; xmm3 = (xmm1 & !0xffff_ffffu128) | (xmm0 & 0xffff_ffffu128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) > f32::from_bits(core::ptr::read_unaligned((P6) as *const u32)) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(core::ptr::read_unaligned((P6) as *const u32)) }).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) < f32::from_bits(core::ptr::read_unaligned((P3) as *const u32)) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(core::ptr::read_unaligned((P3) as *const u32)) }).to_bits() as u128); xmm0 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(core::ptr::read_unaligned((P7) as *const u32))).to_bits() as u128); rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm0 as u32)))) as u32 as u64; crate::simd_ops::wu16(r9.wrapping_add(0x28u64), ((rax as u16)) as u16); bb = 15; continue; }
            9 => { bb = 10; continue; }
            10 => { rdx = ((r14.wrapping_add(0x2cu64)) as u64) as u64; r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); xmm0 = (xmm6) as u128; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm1 = 0u128; xmm3 = (xmm1 & !0xffff_ffffu128) | (xmm0 & 0xffff_ffffu128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) > f32::from_bits(core::ptr::read_unaligned((P6) as *const u32)) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(core::ptr::read_unaligned((P6) as *const u32)) }).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) < f32::from_bits(core::ptr::read_unaligned((P3) as *const u32)) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(core::ptr::read_unaligned((P3) as *const u32)) }).to_bits() as u128); xmm0 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(core::ptr::read_unaligned((P7) as *const u32))).to_bits() as u128); rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm0 as u32)))) as u32 as u64; crate::simd_ops::wu16(r9.wrapping_add(0x2au64), ((rax as u16)) as u16); bb = 15; continue; }
            11 => { bb = 12; continue; }
            12 => { rdx = ((r14.wrapping_add(0x38u64)) as u64) as u64; r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); xmm0 = (xmm6) as u128; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm1 = 0u128; xmm3 = (xmm1 & !0xffff_ffffu128) | (xmm0 & 0xffff_ffffu128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) > f32::from_bits(core::ptr::read_unaligned((P6) as *const u32)) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(core::ptr::read_unaligned((P6) as *const u32)) }).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) < f32::from_bits(core::ptr::read_unaligned((P3) as *const u32)) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(core::ptr::read_unaligned((P3) as *const u32)) }).to_bits() as u128); xmm0 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(core::ptr::read_unaligned((P7) as *const u32))).to_bits() as u128); rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm0 as u32)))) as u32 as u64; crate::simd_ops::wu16(r9.wrapping_add(0x2cu64), ((rax as u16)) as u16); bb = 15; continue; }
            13 => { bb = 14; continue; }
            14 => { rdx = ((r14.wrapping_add(0x44u64)) as u64) as u64; r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); xmm0 = (xmm6) as u128; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm1 = 0u128; xmm3 = (xmm1 & !0xffff_ffffu128) | (xmm0 & 0xffff_ffffu128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) > f32::from_bits(core::ptr::read_unaligned((P6) as *const u32)) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(core::ptr::read_unaligned((P6) as *const u32)) }).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) < f32::from_bits(core::ptr::read_unaligned((P3) as *const u32)) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(core::ptr::read_unaligned((P3) as *const u32)) }).to_bits() as u128); xmm0 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(core::ptr::read_unaligned((P7) as *const u32))).to_bits() as u128); rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm0 as u32)))) as u32 as u64; crate::simd_ops::wu16(r9.wrapping_add(0x2eu64), ((rax as u16)) as u16); bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::lk_add_u32(rbp.wrapping_add(0x14u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = (0xffffffffu32) as u32 as u64; rcx = ((rbp.wrapping_add(0x8u64)) as u64) as u64; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rbp.wrapping_add(0x10u64), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P8)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 17; continue; }
            17 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x1420d0600u64) { 0x3f => { bb = 1; continue; } 0x4c => { bb = 2; continue; } 0x8e => { bb = 3; continue; } 0xab => { bb = 4; continue; } 0xb9 => { bb = 5; continue; } 0xe6 => { bb = 6; continue; } 0x113 => { bb = 7; continue; } 0x120 => { bb = 8; continue; } 0x15e => { bb = 9; continue; } 0x160 => { bb = 10; continue; } 0x19b => { bb = 11; continue; } 0x1a0 => { bb = 12; continue; } 0x1db => { bb = 13; continue; } 0x1e0 => { bb = 14; continue; } 0x219 => { bb = 15; continue; } 0x21f => { bb = 16; continue; } 0x231 => { bb = 17; continue; } _ => { let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(j_tgt, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; } } }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d08a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645790; const P1: u64 = 0x1426455a8; const P2: u64 = 0x140000000; const P3: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = (((r8 as u16)) as u32) as u32 as u64; rsi = (((r9 as u8)) as u32) as u32 as u64; rbp = (rcx) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 1; continue; }
            1 => { fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rbx) as u64); rbx = ((rdx.wrapping_add(0x8u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (r14) as u64); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; r14 = ((rax as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((r14 as u32)) as u32); bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rdx as u32)) as u32); fl_l = ((rsi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 14; continue; } bb = 6; continue; }
            6 => { rax = ((rdi.wrapping_sub(0x95u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x5u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 14; continue; } bb = 7; continue; }
            7 => { r8 = ((P2) as u64) as u64; rax = (rax as u32 as i32 as i64) as u64; rcx = (crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x20d09c8u64))) as u32 as u64; fl_l = ((rcx).wrapping_add(r8)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; j_tgt = (rcx) as u64; bb = 0xffff_fffe; continue; }
            8 => { xmm6 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x24u64))) as u32) as u128; bb = 15; continue; }
            9 => { xmm6 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x30u64))) as u32) as u128; bb = 15; continue; }
            10 => { xmm6 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x3cu64))) as u32) as u128; bb = 15; continue; }
            11 => { xmm6 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x48u64))) as u32) as u128; bb = 15; continue; }
            12 => { xmm2 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x24u64))) as u32) as u128; xmm6 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(core::ptr::read_unaligned((rbp.wrapping_add(0x30u64)) as *const u32)) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(core::ptr::read_unaligned((rbp.wrapping_add(0x30u64)) as *const u32)) }).to_bits() as u128); bb = 15; continue; }
            13 => { xmm2 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x3cu64))) as u32) as u128; xmm6 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(core::ptr::read_unaligned((rbp.wrapping_add(0x48u64)) as *const u32)) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(core::ptr::read_unaligned((rbp.wrapping_add(0x48u64)) as *const u32)) }).to_bits() as u128); xmm6 = (xmm6) as u128; bb = 15; continue; }
            14 => { xmm6 = 0u128; bb = 15; continue; }
            15 => { rax = ((rdx.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = (0xffffffffu32) as u32 as u64; rcx = (rbx) as u64; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 17; continue; }
            17 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; xmm0 = (xmm6) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            18 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; xmm0 = 0u128; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x1420d08a0u64) { 0x23 => { bb = 1; continue; } 0x2c => { bb = 2; continue; } 0x37 => { bb = 3; continue; } 0x5b => { bb = 4; continue; } 0x68 => { bb = 5; continue; } 0x7b => { bb = 6; continue; } 0x86 => { bb = 7; continue; } 0x9c => { bb = 8; continue; } 0xa3 => { bb = 9; continue; } 0xaa => { bb = 10; continue; } 0xb1 => { bb = 11; continue; } 0xb8 => { bb = 12; continue; } 0xc4 => { bb = 13; continue; } 0xd4 => { bb = 14; continue; } 0xd8 => { bb = 15; continue; } 0xe2 => { bb = 16; continue; } 0xf3 => { bb = 17; continue; } 0x112 => { bb = 18; continue; } _ => { let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(j_tgt, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; } } }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d09f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx, rax);
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rax as u32)) as u32);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d0a10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645790; const P1: u64 = 0x1426455a8; const P2: u64 = 0x1411f3ec0; const P3: u64 = 0x1420d1d10; const P4: u64 = 0x1420d3d40; const P5: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = (((r9 as u16)) as u32) as u32 as u64; rbp = r8; r14 = rdx; rsi = rcx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 1; continue; }
            1 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(r8.wrapping_add(0x38u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r15); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x10u64))) as u32 as u64; r15 = ((rax as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = rbp.wrapping_add(0x8); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((r15 as u32)) as u32); bb = 5; continue; }
            5 => { crate::simd_ops::lk_add_u32(rbp.wrapping_add(0x14u64), (1) as u32); r9 = ((crate::simd_ops::ru8(rsp.wrapping_add(0x70u64))) as u32) as u32 as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_l = ((r9 as u8)) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; if (((((fl_l as u8)) & ((fl_r as u8))) as i8) >= 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = ((crate::simd_ops::ru8(r14.wrapping_add(0x4fd8u64))) as u32) as u32 as u64; rcx = r8; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = rax; bb = 8; continue; }
            7 => { rcx = (crate::simd_ops::ru64(r8.wrapping_add(0xc0u64))) as u64; rax = (0xaaaaaaaaaaaaaaabu64) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = ((rdx).wrapping_add(r8)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 8; continue; }
            8 => { rcx = ((rdi as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x95; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 9; continue; }
            9 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsi, rax); bb = 14; continue; }
            11 => { rdx = ((rdx).wrapping_add(0x2cu64)) as u64; rcx = rsp.wrapping_add(0x58); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r8 = r14.wrapping_add(0x38); bb = 13; continue; }
            12 => { rdx = ((rdx).wrapping_add(0x28u64)) as u64; rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r8 = r14.wrapping_add(0x20); bb = 13; continue; }
            13 => { rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x14u64))) as u32 as u64; fl_r = 0x1; crate::simd_ops::wu32(rbp.wrapping_add(0x14u64), ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rax = 0xffffffff; rcx = rbp.wrapping_add(0x8); let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rbp.wrapping_add(0x10u64), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = rsi; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            17 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx, rax); rax = rsi; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d0b60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645790; const P1: u64 = 0x1426455a8; const P2: u64 = 0x1411f3ec0; const P3: u64 = 0x140000000; const P4: u64 = 0x1434268d0; const P5: u64 = 0x143426eec; const P6: u64 = 0x1420d1d10; const P7: u64 = 0x1420d38f0; const P8: u64 = 0x1420d3d40; const P9: u64 = 0x143428760; const P10: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rsi = (((r8 as u16)) as u32) as u32 as u64; rdi = (((r9 as u8)) as u32) as u32 as u64; rbp = rdx; r14 = rcx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 1; continue; }
            1 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 30; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), r15); crate::simd_ops::wu128(rax.wrapping_sub(0x28u64), (xmm6) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x10u64))) as u32 as u64; r15 = ((rax as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = rbp.wrapping_add(0x8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((r15 as u32)) as u32); bb = 5; continue; }
            5 => { crate::simd_ops::lk_add_u32(rbp.wrapping_add(0x14u64), (1) as u32); r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((((fl_l as u8)) & ((fl_r as u8))) as i8) >= 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = ((crate::simd_ops::ru8(r14.wrapping_add(0x4fd8u64))) as u32) as u32 as u64; rcx = r8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; bb = 8; continue; }
            7 => { rcx = (crate::simd_ops::ru64(r8.wrapping_add(0xc0u64))) as u64; rax = (0xaaaaaaaaaaaaaaabu64) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = ((rdx).wrapping_add(r8)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 8; continue; }
            8 => { rax = ((rsi.wrapping_sub(0x95u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1b; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 26; continue; } bb = 9; continue; }
            9 => { r8 = P3; rax = (rax as u32 as i32 as i64) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(rax).wrapping_add(0x20d0e64u64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x20d0e3cu64))) as u32 as u64; fl_l = ((rcx).wrapping_add(r8)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; j_tgt = rcx; bb = 0xffff_fffe; continue; }
            10 => { rax = ((crate::simd_ops::ru8(rsi.wrapping_add(rdx).wrapping_sub(0x95u64))) as u32) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; xmm0 = (((rax as u32)) as u32) as u128; xmm1 = (((rcx as u32)) as u32) as u128; xmm3 = crate::simd_ops::vpcmpeqd(xmm0, xmm1); xmm0 = ((crate::simd_ops::ru32(P4)) as u32) as u128; xmm2 = 0u128; xmm6 = crate::simd_ops::vblendvps(xmm0, xmm2, xmm3); bb = 27; continue; }
            11 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x25u64))) as u32) as u32 as u64; bb = 13; continue; }
            12 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x26u64))) as u32) as u32 as u64; bb = 13; continue; }
            13 => { xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128); xmm6 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(crate::simd_ops::ru32(P5))).to_bits() as u128); bb = 27; continue; }
            14 => { rdx = ((rdx).wrapping_add(0x28u64)) as u64; rcx = rsp.wrapping_add(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = r14.wrapping_add(0x20); r8 = (((rdi as u8)) as u32) as u32 as u64; xmm0 = ((crate::simd_ops::ru32(rax)) as u32) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm6 = (xmm0) as u128; bb = 27; continue; }
            15 => { rdx = ((rdx).wrapping_add(0x28u64)) as u64; rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = r14.wrapping_add(0x2c); bb = 18; continue; }
            16 => { rdx = ((rdx).wrapping_add(0x2cu64)) as u64; rcx = rsp.wrapping_add(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = r14.wrapping_add(0x38); r8 = (((rdi as u8)) as u32) as u32 as u64; xmm0 = ((crate::simd_ops::ru32(rax)) as u32) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm6 = (xmm0) as u128; bb = 27; continue; }
            17 => { rdx = ((rdx).wrapping_add(0x2cu64)) as u64; rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = r14.wrapping_add(0x44); bb = 18; continue; }
            18 => { xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as u32) as u128; r8 = (((rdi as u8)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm6 = (xmm0) as u128; bb = 27; continue; }
            19 => { rdx = ((rdx).wrapping_add(0x28u64)) as u64; rcx = rsp.wrapping_add(0x48); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = r14.wrapping_add(0x20); r9 = (((rdi as u8)) as u32) as u32 as u64; rcx = rsp.wrapping_add(0x90); rdx = (crate::simd_ops::ru64(rax)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm2 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x90u64))) as u32) as u128; xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x94u64))) as u32) as u128; xmm1 = 0u128; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 25; continue; } bb = 20; continue; }
            20 => { fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 25; continue; } bb = 21; continue; }
            21 => { xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm6 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32)).sqrt().to_bits() as u128); bb = 27; continue; }
            22 => { rdx = ((rdx).wrapping_add(0x2cu64)) as u64; rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = r14.wrapping_add(0x38); r9 = (((rdi as u8)) as u32) as u32 as u64; rcx = rsp.wrapping_add(0x20); rdx = (crate::simd_ops::ru64(rax)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm2 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32) as u128; xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x24u64))) as u32) as u128; xmm1 = 0u128; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 25; continue; } bb = 23; continue; }
            23 => { fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 25; continue; } bb = 24; continue; }
            24 => { xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm6 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32)).sqrt().to_bits() as u128); xmm6 = (xmm6) as u128; bb = 27; continue; }
            25 => { xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm3 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32)).sqrt().to_bits() as u128); xmm6 = (xmm3) ^ (crate::simd_ops::ru128(P9)); bb = 27; continue; }
            26 => { xmm6 = 0u128; bb = 27; continue; }
            27 => { fl_l = (crate::simd_ops::lk_add_u32(rbp.wrapping_add(0x14u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 29; continue; } bb = 28; continue; }
            28 => { rax = 0xffffffff; rcx = rbp.wrapping_add(0x8); let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rbp.wrapping_add(0x10u64), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 29; continue; }
            29 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; xmm0 = (xmm6) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; rsp = ((rsp).wrapping_add(0x70u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            30 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; xmm0 = 0u128; rsp = ((rsp).wrapping_add(0x70u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x1420d0b60u64) { 0x26 => { bb = 1; continue; } 0x2f => { bb = 2; continue; } 0x3a => { bb = 3; continue; } 0x57 => { bb = 4; continue; } 0x65 => { bb = 5; continue; } 0x79 => { bb = 6; continue; } 0x8f => { bb = 7; continue; } 0xb8 => { bb = 8; continue; } 0xc7 => { bb = 9; continue; } 0xe6 => { bb = 10; continue; } 0x113 => { bb = 11; continue; } 0x119 => { bb = 12; continue; } 0x11d => { bb = 13; continue; } 0x132 => { bb = 14; continue; } 0x15a => { bb = 15; continue; } 0x16e => { bb = 16; continue; } 0x196 => { bb = 17; continue; } 0x1a8 => { bb = 18; continue; } 0x1bf => { bb = 19; continue; } 0x201 => { bb = 20; continue; } 0x207 => { bb = 21; continue; } 0x219 => { bb = 22; continue; } 0x252 => { bb = 23; continue; } 0x258 => { bb = 24; continue; } 0x26e => { bb = 25; continue; } 0x288 => { bb = 26; continue; } 0x28c => { bb = 27; continue; } 0x292 => { bb = 28; continue; } 0x2a4 => { bb = 29; continue; } 0x2c7 => { bb = 30; continue; } _ => { return crate::simd_ops::tail_stk_xmm(j_tgt, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); } } }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d0e90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645790; const P1: u64 = 0x1426455a8; const P2: u64 = 0x1411f3ec0; const P3: u64 = 0x140000000; const P4: u64 = 0x1420d1d10; const P5: u64 = 0x1420d3d40; const P6: u64 = 0x1420d38f0; const P7: u64 = 0x142645598;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0xb0u64)) as u64; rsi = (((r8 as u16)) as u32) as u32 as u64; rdi = (((r9 as u8)) as u32) as u32 as u64; rbp = rdx; r14 = rcx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 1; continue; }
            1 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 29; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbx); rbx = rdx.wrapping_add(0x8); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), r15); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; r15 = ((rax as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((r15 as u32)) as u32); bb = 5; continue; }
            5 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(0xcu64), (1) as u32); rax = (0xaaaaaaaaaaaaaaabu64) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0xa0u64), (xmm6) as u128); r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0xc0u64))) as u64; let prod = (rax as u128).wrapping_mul((r8) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rdx = ((crate::simd_ops::ru8(r14.wrapping_add(0x4fd8u64))) as u32) as u32 as u64; r8 = ((r8).wrapping_sub(rax)) as u64; rbp = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; rbp = ((rbp).wrapping_shl(4u32)) as u64; rbp = ((rbp).wrapping_add(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; rcx = ((rsi.wrapping_sub(0x95u64)) as u32) as u32 as u64; r10 = rax; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1b; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 25; continue; } bb = 6; continue; }
            6 => { rax = P3; rcx = ((((rcx as u32)) as i32 as i64) as u64) as u64; rcx = ((crate::simd_ops::ru8(rax.wrapping_add(rcx).wrapping_add(0x20d121cu64))) as u32) as u32 as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x20d11f4u64))) as u32 as u64; fl_l = ((rdx).wrapping_add(rax)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; j_tgt = rdx; bb = 0xffff_fffe; continue; }
            7 => { rax = ((crate::simd_ops::ru8(rsi.wrapping_add(r10).wrapping_sub(0x95u64))) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(rbp).wrapping_sub(0x95u64))) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rdi = (rdi & !0xffu64) | (((if (((fl_l as u8)) != ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 26; continue; }
            8 => { rax = ((crate::simd_ops::ru8(r10.wrapping_add(0x25u64))) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x25u64))) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rdi = (rdi & !0xffu64) | (((if (((fl_l as u8)) != ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 26; continue; }
            9 => { rax = ((crate::simd_ops::ru8(r10.wrapping_add(0x26u64))) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x26u64))) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rdi = (rdi & !0xffu64) | (((if (((fl_l as u8)) != ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 26; continue; }
            10 => { rdx = r10.wrapping_add(0x28); rcx = rsp.wrapping_add(0xd0); rsi = r14.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (((rdi as u8)) as u32) as u32 as u64; rcx = rsp.wrapping_add(0x20); r8 = rsi; rdx = (crate::simd_ops::ru64(rax)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbp.wrapping_add(0x28); r14 = rax; rcx = rsp.wrapping_add(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (((rdi as u8)) as u32) as u32 as u64; rcx = rsp.wrapping_add(0x30); r8 = rsi; rdx = (crate::simd_ops::ru64(rax)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = ((crate::simd_ops::ru32(rax)) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r14)) as u32 as u64; if (f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 24; continue; } bb = 11; continue; }
            11 => { if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 24; continue; } bb = 12; continue; }
            12 => { xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r14.wrapping_add(0x4u64))) as u32 as u64; bb = 22; continue; }
            13 => { rdx = r10.wrapping_add(0x2c); rcx = rsp.wrapping_add(0x38); rsi = r14.wrapping_add(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (((rdi as u8)) as u32) as u32 as u64; rcx = rsp.wrapping_add(0x40); r8 = rsi; rdx = (crate::simd_ops::ru64(rax)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbp.wrapping_add(0x2c); r14 = rax; rcx = rsp.wrapping_add(0x48); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (((rdi as u8)) as u32) as u32 as u64; rcx = rsp.wrapping_add(0x50); r8 = rsi; rdx = (crate::simd_ops::ru64(rax)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = ((crate::simd_ops::ru32(rax)) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r14)) as u32 as u64; if (f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 24; continue; } bb = 14; continue; }
            14 => { if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 24; continue; } bb = 15; continue; }
            15 => { xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r14.wrapping_add(0x4u64))) as u32 as u64; bb = 22; continue; }
            16 => { rdx = rbp.wrapping_add(0x28); rcx = rsp.wrapping_add(0x58); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = r10.wrapping_add(0x28); rcx = rsp.wrapping_add(0x60); xmm6 = ((crate::simd_ops::ru32(rax)) as u32) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (xmm6) as u128; r8 = (((rdi as u8)) as u32) as u32 as u64; rdx = r14.wrapping_add(0x20); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm6 = (xmm0) as u128; xmm0 = ((crate::simd_ops::ru32(rcx)) as u32) as u128; rdx = r14.wrapping_add(0x20); bb = 21; continue; }
            17 => { rdx = rbp.wrapping_add(0x28); rcx = rsp.wrapping_add(0x68); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (((rdi as u8)) as u32) as u32 as u64; rdx = r14.wrapping_add(0x2c); xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as u32) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = r10.wrapping_add(0x28); rcx = rsp.wrapping_add(0x70); xmm6 = (xmm0) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = r14.wrapping_add(0x2c); bb = 20; continue; }
            18 => { rdx = rbp.wrapping_add(0x2c); rcx = rsp.wrapping_add(0x78); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = r10.wrapping_add(0x2c); rcx = rsp.wrapping_add(0x80); xmm6 = ((crate::simd_ops::ru32(rax)) as u32) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (xmm6) as u128; r8 = (((rdi as u8)) as u32) as u32 as u64; rdx = r14.wrapping_add(0x38); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm6 = (xmm0) as u128; xmm0 = ((crate::simd_ops::ru32(rcx)) as u32) as u128; rdx = r14.wrapping_add(0x38); bb = 21; continue; }
            19 => { rdx = rbp.wrapping_add(0x2c); rcx = rsp.wrapping_add(0x88); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (((rdi as u8)) as u32) as u32 as u64; rdx = r14.wrapping_add(0x44); xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as u32) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = r10.wrapping_add(0x2c); rcx = rsp.wrapping_add(0x90); xmm6 = (xmm0) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = r14.wrapping_add(0x44); bb = 20; continue; }
            20 => { xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as u32) as u128; bb = 21; continue; }
            21 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; bb = 22; continue; }
            22 => { if (f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 24; continue; } bb = 23; continue; }
            23 => { if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 25; continue; } bb = 24; continue; }
            24 => { rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 26; continue; }
            25 => { rdi = (rdi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 26; continue; }
            26 => { fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(0xcu64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0xa0u64))) as u128; if (((fl_l as u32)) != 0) { bb = 28; continue; } bb = 27; continue; }
            27 => { rcx = 0xffffffff; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rbx.wrapping_add(0x8u64), ((rcx as u32)) as u32); rcx = (tmp_xchg) as u32 as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 28; continue; }
            28 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; rax = (((rdi as u8)) as u32) as u32 as u64; bb = 30; continue; }
            29 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 30; continue; }
            30 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xe8u64))) as u64; rsp = ((rsp).wrapping_add(0xb0u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x1420d0e90u64) { 0x29 => { bb = 1; continue; } 0x32 => { bb = 2; continue; } 0x3d => { bb = 3; continue; } 0x59 => { bb = 4; continue; } 0x66 => { bb = 5; continue; } 0xc7 => { bb = 6; continue; } 0xe5 => { bb = 7; continue; } 0xfe => { bb = 8; continue; } 0x10f => { bb = 9; continue; } 0x120 => { bb = 10; continue; } 0x17d => { bb = 11; continue; } 0x183 => { bb = 12; continue; } 0x193 => { bb = 13; continue; } 0x1ed => { bb = 14; continue; } 0x1f3 => { bb = 15; continue; } 0x203 => { bb = 16; continue; } 0x248 => { bb = 17; continue; } 0x283 => { bb = 18; continue; } 0x2c8 => { bb = 19; continue; } 0x304 => { bb = 20; continue; } 0x309 => { bb = 21; continue; } 0x312 => { bb = 22; continue; } 0x314 => { bb = 23; continue; } 0x316 => { bb = 24; continue; } 0x31b => { bb = 25; continue; } 0x31e => { bb = 26; continue; } 0x32d => { bb = 27; continue; } 0x33e => { bb = 28; continue; } 0x34c => { bb = 29; continue; } 0x34e => { bb = 30; continue; } _ => { return crate::simd_ops::tail_stk_xmm(j_tgt, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); } } }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d1240(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411f3ec0; const P1: u64 = 0x142645790; const P2: u64 = 0x1426455a8; const P3: u64 = 0x140000000; const P4: u64 = 0x1420d1d10; const P5: u64 = 0x1420d3d40; const P6: u64 = 0x143426f84; const P7: u64 = 0x143427080; const P8: u64 = 0x1434268d0; const P9: u64 = 0x143426460; const P10: u64 = 0x143426bf8; const P11: u64 = 0x143426780; const P12: u64 = 0x1420d38f0; const P13: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rsi = (((r8 as u16)) as u32) as u32 as u64; rdi = (((r9 as u8)) as u32) as u32 as u64; rbx = rdx; r14 = rcx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 34; continue; } bb = 1; continue; }
            1 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 34; continue; } bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 34; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rax.wrapping_add(0x18u64), r15); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbp); fl_l = ((r9 as u8)) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; if (((((fl_l as u8)) & ((fl_r as u8))) as i8) >= 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = ((crate::simd_ops::ru8(rcx.wrapping_add(0x4fd8u64))) as u32) as u32 as u64; rcx = r8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = rax; bb = 6; continue; }
            5 => { rcx = (crate::simd_ops::ru64(r8.wrapping_add(0xc0u64))) as u64; rax = (0xaaaaaaaaaaaaaaabu64) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; rbp = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rbp = ((rbp).wrapping_shl(4u32)) as u64; fl_l = ((rbp).wrapping_add(r8)) as u64; fl_r = fl_l; rbp = ((fl_l as u64)) as u64; bb = 6; continue; }
            6 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; r15 = ((rax as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = rbx.wrapping_add(0x8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), ((r15 as u32)) as u32); bb = 8; continue; }
            8 => { r9 = (crate::simd_ops::ru32(rbx.wrapping_add(0x14u64))) as u32 as u64; rax = ((rsi.wrapping_sub(0x95u64)) as u32) as u32 as u64; r9 = (((r9 as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm6) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm7) as u128); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x14u64), ((r9 as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1b; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 30; continue; } bb = 9; continue; }
            9 => { rdx = P3; rax = (rax as u32 as i32 as i64) as u64; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rax).wrapping_add(0x20d1664u64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x20d163cu64))) as u32 as u64; fl_l = ((rcx).wrapping_add(rdx)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; j_tgt = rcx; bb = 0xffff_fffe; continue; }
            10 => { rdi = ((crate::simd_ops::ru8(rsi.wrapping_add(rbp).wrapping_sub(0x95u64))) as u32) as u32 as u64; r9 = (crate::simd_ops::ru32(rbx.wrapping_add(0x14u64))) as u32 as u64; bb = 31; continue; }
            11 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x25u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r9 = (crate::simd_ops::ru32(rbx.wrapping_add(0x14u64))) as u32 as u64; rdi = (rdi & !0xffu64) | (((if (((fl_l as u8)) != ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 31; continue; }
            12 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x26u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r9 = (crate::simd_ops::ru32(rbx.wrapping_add(0x14u64))) as u32 as u64; rdi = (rdi & !0xffu64) | (((if (((fl_l as u8)) != ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 31; continue; }
            13 => { rdx = rbp.wrapping_add(0x28); rcx = rsp.wrapping_add(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = r14.wrapping_add(0x20); r9 = (((rdi as u8)) as u32) as u32 as u64; rcx = rsp.wrapping_add(0x30); rdx = (crate::simd_ops::ru64(rax)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru32(rbx.wrapping_add(0x14u64))) as u32 as u64; xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(rax)) as u32) as u128; xmm2 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm3 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm0 = 0u128; fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; rdi = (rdi & !0xffu64) | (((if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 31; continue; }
            14 => { rax = (((crate::simd_ops::ru16(rbp.wrapping_add(0x2cu64))) as i16 as i32) as u32) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128); xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(crate::simd_ops::ru32(P6))).to_bits() as u128); xmm5 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(crate::simd_ops::ru32(P7)) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P7)) }).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((if f32::from_bits(xmm5 as u32) < f32::from_bits(crate::simd_ops::ru32(P8)) { f32::from_bits(xmm5 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P8)) }).to_bits() as u128); rax = (((crate::simd_ops::ru16(rbp.wrapping_add(0x2eu64))) as i16 as i32) as u32) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128); xmm2 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(crate::simd_ops::ru32(P6))).to_bits() as u128); xmm7 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(crate::simd_ops::ru32(P7)) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P7)) }).to_bits() as u128); xmm7 = (xmm7 & !0xffff_ffffu128) | ((if f32::from_bits(xmm7 as u32) < f32::from_bits(crate::simd_ops::ru32(P8)) { f32::from_bits(xmm7 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P8)) }).to_bits() as u128); xmm2 = ((crate::simd_ops::ru32(P9)) as u32) as u128; xmm1 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) * f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm0 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm3 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm3 as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xa0u64), ((xmm5 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0xa4u64), ((xmm7 as u32)) as u32); xmm6 = 0u128; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 16; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm6 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((xmm6 as u32)) as u32); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 19; continue; }
            16 => { fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P8)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 18; continue; } bb = 17; continue; }
            17 => { xmm1 = (xmm3) as u128; xmm4 = crate::simd_ops::vrsqrtss(xmm1, (xmm3 as u32)); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm1 = ((crate::simd_ops::ru32(P10)) as u32) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm3 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm1 = ((crate::simd_ops::ru32(P11)) as u32) as u128; xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm0 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm1 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm7 as u32)).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0xa0u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0xa4u64), ((xmm1 as u32)) as u32); bb = 18; continue; }
            18 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; bb = 19; continue; }
            19 => { r8 = r14.wrapping_add(0x38); r9 = (((rdi as u8)) as u32) as u32 as u64; rcx = rsp.wrapping_add(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru32(rbx.wrapping_add(0x14u64))) as u32 as u64; xmm0 = ((crate::simd_ops::ru32(rax)) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as u32) as u128; xmm2 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm3 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; rdi = (rdi & !0xffu64) | (((if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 31; continue; }
            20 => { rax = (((crate::simd_ops::ru16(rbp.wrapping_add(0x28u64))) as i16 as i32) as u32) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128); xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(crate::simd_ops::ru32(P6))).to_bits() as u128); xmm6 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(crate::simd_ops::ru32(P7)) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P7)) }).to_bits() as u128); rax = (((crate::simd_ops::ru16(rbp.wrapping_add(0x2au64))) as i16 as i32) as u32) as u32 as u64; xmm6 = (xmm6 & !0xffff_ffffu128) | ((if f32::from_bits(xmm6 as u32) < f32::from_bits(crate::simd_ops::ru32(P8)) { f32::from_bits(xmm6 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P8)) }).to_bits() as u128); xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128); xmm2 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(crate::simd_ops::ru32(P6))).to_bits() as u128); xmm3 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(crate::simd_ops::ru32(P7)) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P7)) }).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) < f32::from_bits(crate::simd_ops::ru32(P8)) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P8)) }).to_bits() as u128); xmm2 = ((crate::simd_ops::ru32(P9)) as u32) as u128; xmm1 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm0 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm4 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm4 as u32)) as u32 as u64; xmm7 = 0u128; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 22; continue; } bb = 21; continue; }
            21 => { xmm6 = 0u128; bb = 24; continue; }
            22 => { fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P8)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 24; continue; } bb = 23; continue; }
            23 => { xmm1 = (xmm4) as u128; xmm5 = crate::simd_ops::vrsqrtss(xmm1, (xmm4 as u32)); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm1 = ((crate::simd_ops::ru32(P10)) as u32) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm4 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm2 = ((crate::simd_ops::ru32(P11)) as u32) as u128; xmm3 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); bb = 24; continue; }
            24 => { rdx = r14.wrapping_add(0x20); r8 = (((rdi as u8)) as u32) as u32 as u64; xmm0 = (xmm6) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm7 as u32)) as u32 as u64; rdi = (rdi & !0xffu64) | (((if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 31; continue; }
            25 => { rdx = rbp.wrapping_add(0x28); rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = r14.wrapping_add(0x2c); bb = 28; continue; }
            26 => { rdx = rbp.wrapping_add(0x2c); rcx = rsp.wrapping_add(0x48); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = r14.wrapping_add(0x38); xmm0 = ((crate::simd_ops::ru32(rax)) as u32) as u128; bb = 29; continue; }
            27 => { rdx = rbp.wrapping_add(0x2c); rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = r14.wrapping_add(0x44); bb = 28; continue; }
            28 => { xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as u32) as u128; bb = 29; continue; }
            29 => { r8 = (((rdi as u8)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm1 = 0u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; rdi = (rdi & !0xffu64) | (((if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 31; continue; }
            30 => { rdi = (rdi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 31; continue; }
            31 => { xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; rcx = ((r9.wrapping_sub(0x1u64)) as u32) as u32 as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x14u64), ((rcx as u32)) as u32); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 33; continue; } bb = 32; continue; }
            32 => { rcx = 0xffffffff; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rbx.wrapping_add(0x10u64), ((rcx as u32)) as u32); rcx = (tmp_xchg) as u32 as u64; rcx = rbx.wrapping_add(0x8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P13), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 33; continue; }
            33 => { rax = (((rdi as u8)) as u32) as u32 as u64; bb = 35; continue; }
            34 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 35; continue; }
            35 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; rsp = ((rsp).wrapping_add(0x80u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x1420d1240u64) { 0x29 => { bb = 1; continue; } 0x32 => { bb = 2; continue; } 0x3f => { bb = 3; continue; } 0x4c => { bb = 4; continue; } 0x61 => { bb = 5; continue; } 0x8a => { bb = 6; continue; } 0x9a => { bb = 7; continue; } 0xa8 => { bb = 8; continue; } 0xd6 => { bb = 9; continue; } 0xf3 => { bb = 10; continue; } 0x104 => { bb = 11; continue; } 0x115 => { bb = 12; continue; } 0x126 => { bb = 13; continue; } 0x173 => { bb = 14; continue; } 0x1eb => { bb = 15; continue; } 0x1fe => { bb = 16; continue; } 0x208 => { bb = 17; continue; } 0x24e => { bb = 18; continue; } 0x256 => { bb = 19; continue; } 0x28e => { bb = 20; continue; } 0x2f4 => { bb = 21; continue; } 0x2fa => { bb = 22; continue; } 0x304 => { bb = 23; continue; } 0x334 => { bb = 24; continue; } 0x34f => { bb = 25; continue; } 0x363 => { bb = 26; continue; } 0x37b => { bb = 27; continue; } 0x38d => { bb = 28; continue; } 0x392 => { bb = 29; continue; } 0x3a9 => { bb = 30; continue; } 0x3ac => { bb = 31; continue; } 0x3cb => { bb = 32; continue; } 0x3dd => { bb = 33; continue; } 0x3e3 => { bb = 34; continue; } 0x3e5 => { bb = 35; continue; } _ => { return crate::simd_ops::tail_stk_xmm(j_tgt, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); } } }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d19a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x55c; const P1: u64 = 0x142645790; const P2: u64 = 0xfffffffffffffffc; const P3: u64 = 0xfffffffffffffff4; const P4: u64 = 0x1426455a8; const P5: u64 = 0x24; const P6: u64 = 0x4; const P7: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = rcx; rbx = ((rdx.wrapping_add(P0)) as u64) as u64; rsi = 0x10; bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; rbp = ((rax as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = ((rbx.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbx.wrapping_add(P2), ((rbp as u32)) as u32); bb = 3; continue; }
            3 => { crate::simd_ops::lk_add_u32(rbx, (1) as u32); fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = 0; fl_l = 0; fl_r = 0; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(rbx.wrapping_add(P6), ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64)); bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::lk_add_u32(rbx, (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = 0xffffffff; rcx = ((rbx.wrapping_add(P3)) as u64) as u64; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rbx.wrapping_add(P2), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rbx = ((rbx).wrapping_add(0x330u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = r14; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu8(r14.wrapping_add(P6), (0x1u8) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d1a50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1420e41f0; const P1: u64 = 0x142645790; const P2: u64 = 0x1426455a8; const P3: u64 = 0x142646478; const P4: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rsi = 0; fl_l = 0; fl_r = 0; r12 = rdx.wrapping_add(0x540); r15 = ((rsi as u32)) as u32 as u64; rdi = rdx.wrapping_add(0x38d8); r13 = (0xaaaaaaaaaaaaaaabu64) as u64; bb = 1; continue; }
            1 => { rcx = r12; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x10u64))) as u32 as u64; rbp = ((rax as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = r14.wrapping_add(0x8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(r14.wrapping_add(0x10u64), ((rbp as u32)) as u32); bb = 3; continue; }
            3 => { crate::simd_ops::lk_add_u32(r14.wrapping_add(0x14u64), (1) as u32); rdx = rsp.wrapping_add(0x20); rcx = (0x100000000u64) as u64; rax = r15; rax = (((rax) | (rcx))) as u64; rcx = ((rsi as u32)) as u32 as u64; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(r14, rax); rax = (tmp_xchg) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; r8 = rdi.wrapping_sub(0x90); r9 = rdi.wrapping_add(0x30); rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(r14.wrapping_add(0x18u64), ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64)); crate::simd_ops::wu64(r14.wrapping_add(0x38u64), r8); rax = r13; rcx = (crate::simd_ops::ru64(r9)) as u64; rcx = ((rcx).wrapping_add(1)) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; rax = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_add(rax)) as u64; let y_addr: u64 = r8.wrapping_add(rax.wrapping_mul(8)); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); xmm1 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x20u64))) as u128; fl_l = r8; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { bb = 5; continue; }
            5 => { let y_addr: u64 = r8; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(r8.wrapping_add(0x20u64), (xmm1) as u128); r8 = ((r8).wrapping_add(0x30u64)) as u64; fl_l = r8; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { rax = rdi; fl_l = rdi; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { let y_addr: u64 = rax; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rax.wrapping_add(0x20u64), (xmm1) as u128); rax = ((rax).wrapping_add(0x30u64)) as u64; fl_l = rax; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(0xd0u64), ((rsi as u32)) as u32); fl_l = (crate::simd_ops::lk_add_u32(r14.wrapping_add(0x14u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = 0xffffffff; rcx = r14.wrapping_add(0x8); let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(r14.wrapping_add(0x10u64), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            11 => { rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; r15 = ((r15).wrapping_add(1)) as u64; rdi = ((rdi).wrapping_add(0xd8u64)) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 1; continue; } bb = 12; continue; }
            12 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = r13; crate::simd_ops::wu8(r13.wrapping_add(0x4u64), (0x1u8) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d1be0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x24; const P2: u64 = 0x28; const P3: u64 = 0x26;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = 0;
    fl_l = 0; fl_r = 0;
    xmm0 = 0u128; ymm0_hi = 0;
    let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rax as u32)) as u32);
    crate::simd_ops::wu16(rcx.wrapping_add(P1), ((rax as u16)) as u16);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    crate::simd_ops::wu8(rcx.wrapping_add(P3), ((rax as u8)) as u8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d1c10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1434268d0; const P1: u64 = 0x143426eec;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm1 = (xmm0) as u128;
    xmm0 = 0u128;
    xmm3 = (xmm0 & !0xffff_ffffu128) | (xmm1 & 0xffff_ffffu128);
    xmm2 = 0u128;
    xmm3 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) > f32::from_bits(xmm2 as u32) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(xmm2 as u32) }).to_bits() as u128);
    xmm3 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) < f32::from_bits(crate::simd_ops::ru32(P0)) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P0)) }).to_bits() as u128);
    xmm0 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(crate::simd_ops::ru32(P1))).to_bits() as u128);
    rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm0 as u32)))) as u32 as u64;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d1c40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143426eec;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = 0u128;
    rax = (((rcx as u8)) as u32) as u32 as u64;
    xmm0 = (xmm0 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128);
    xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(crate::simd_ops::ru32(P0))).to_bits() as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d1c60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143427080; const P1: u64 = 0x1434268d0; const P2: u64 = 0x143426f84;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    xmm2 = ((crate::simd_ops::ru32(rsp.wrapping_add(0xcu64))) as u32) as u128;
    xmm1 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x8u64))) as u32) as u128;
    xmm4 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(crate::simd_ops::ru32(P0)) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P0)) }).to_bits() as u128);
    xmm2 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(crate::simd_ops::ru32(P0)) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P0)) }).to_bits() as u128);
    xmm2 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) < f32::from_bits(crate::simd_ops::ru32(P1)) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P1)) }).to_bits() as u128);
    xmm4 = (xmm4 & !0xffff_ffffu128) | ((if f32::from_bits(xmm4 as u32) < f32::from_bits(crate::simd_ops::ru32(P1)) { f32::from_bits(xmm4 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P1)) }).to_bits() as u128);
    xmm0 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(crate::simd_ops::ru32(P2))).to_bits() as u128);
    xmm1 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(crate::simd_ops::ru32(P2))).to_bits() as u128);
    crate::simd_ops::wu32(rsp.wrapping_add(0x8u64), ((xmm0 as u32)) as u32);
    crate::simd_ops::wu32(rsp.wrapping_add(0xcu64), ((xmm1 as u32)) as u32);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rax);
    rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0xcu64)))))) as u32 as u64;
    rcx = (crate::simd_ops::cvt_f32_i32((f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x8u64)))))) as u32 as u64;
    rax = (((rax as u16)) as u32) as u32 as u64;
    rax = (((rax as u32)).wrapping_shl(16u32)) as u32 as u64;
    rdx = (((rcx as u16)) as u32) as u32 as u64;
    fl_l = ((((rax as u32)) | ((rdx as u32)))) as u32 as u64; fl_r = fl_l;
    rax = ((fl_l as u32)) as u32 as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d1ce0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143427080; const P1: u64 = 0x1434268d0; const P2: u64 = 0x143426f84;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm1 = (xmm0) as u128;
    xmm0 = 0u128;
    xmm3 = (xmm0 & !0xffff_ffffu128) | (xmm1 & 0xffff_ffffu128);
    xmm3 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) > f32::from_bits(crate::simd_ops::ru32(P0)) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P0)) }).to_bits() as u128);
    xmm3 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) < f32::from_bits(crate::simd_ops::ru32(P1)) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P1)) }).to_bits() as u128);
    xmm0 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(crate::simd_ops::ru32(P2))).to_bits() as u128);
    rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm0 as u32)))) as u32 as u64;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d1d10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426f84; const P1: u64 = 0x143427080; const P2: u64 = 0x1434268d0; const P3: u64 = 0x2; const P4: u64 = 0x143426460; const P5: u64 = 0x143426bf8; const P6: u64 = 0x143426780;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x18u64)) as u64; rax = (((crate::simd_ops::ru16(rdx)) as i16 as i32) as u32) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128); xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(crate::simd_ops::ru32(P0))).to_bits() as u128); xmm5 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(crate::simd_ops::ru32(P1)) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P1)) }).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((if f32::from_bits(xmm5 as u32) < f32::from_bits(crate::simd_ops::ru32(P2)) { f32::from_bits(xmm5 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P2)) }).to_bits() as u128); rax = (((crate::simd_ops::ru16(rdx.wrapping_add(P3))) as i16 as i32) as u32) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128); xmm2 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(crate::simd_ops::ru32(P0))).to_bits() as u128); crate::simd_ops::wu128(rsp, (xmm6) as u128); xmm6 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(crate::simd_ops::ru32(P1)) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P1)) }).to_bits() as u128); xmm6 = (xmm6 & !0xffff_ffffu128) | ((if f32::from_bits(xmm6 as u32) < f32::from_bits(crate::simd_ops::ru32(P2)) { f32::from_bits(xmm6 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P2)) }).to_bits() as u128); xmm2 = ((crate::simd_ops::ru32(P4)) as u32) as u128; xmm1 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm0 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm3 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm3 as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((xmm5 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), ((xmm6 as u32)) as u32); if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx, rax); rax = rcx; xmm6 = (crate::simd_ops::ru128(rsp)) as u128; rsp = ((rsp).wrapping_add(0x18u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P2)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { xmm1 = (xmm3) as u128; xmm4 = crate::simd_ops::vrsqrtss(xmm1, (xmm3 as u32)); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm1 = ((crate::simd_ops::ru32(P5)) as u32) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm3 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm1 = ((crate::simd_ops::ru32(P6)) as u32) as u128; xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm0 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm1 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), ((xmm1 as u32)) as u32); bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp)) as u128; crate::simd_ops::wu64(rcx, rax); rax = rcx; rsp = ((rsp).wrapping_add(0x18u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d1e00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143426f84; const P1: u64 = 0x143427080; const P2: u64 = 0x1434268d0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = 0u128;
    rax = ((((rcx as u16)) as i16 as i32) as u32) as u32 as u64;
    xmm0 = (xmm0 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128);
    xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(crate::simd_ops::ru32(P0))).to_bits() as u128);
    xmm0 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(crate::simd_ops::ru32(P1)) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P1)) }).to_bits() as u128);
    xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) < f32::from_bits(crate::simd_ops::ru32(P2)) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P2)) }).to_bits() as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d1e30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143426f84; const P1: u64 = 0x143427080; const P2: u64 = 0x1434268d0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(crate::simd_ops::ru32(P0))).to_bits() as u128);
    xmm0 = 0u128;
    xmm0 = (xmm0 & !0xffff_ffffu128) | (xmm1 & 0xffff_ffffu128);
    xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) > f32::from_bits(crate::simd_ops::ru32(P1)) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P1)) }).to_bits() as u128);
    xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) < f32::from_bits(crate::simd_ops::ru32(P2)) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P2)) }).to_bits() as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d1e60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645790; const P1: u64 = 0x1426455a8; const P2: u64 = 0x140000000; const P3: u64 = 0x142645598; const P4: u64 = 0x1420d6580;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut fl_k: u32 = 0;
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; rsi = (rdx) as u64; rdi = (((r8 as u16)) as u32) as u32 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rbp) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; rbp = ((rax as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = ((rsi.wrapping_add(0x8u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu32(rsi.wrapping_add(0x10u64), ((rbp as u32)) as u32); bb = 3; continue; }
            3 => { r9 = (crate::simd_ops::ru32(rsi.wrapping_add(0x14u64))) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = ((r9.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x14u64), ((rax as u32)) as u32); rax = (0xaaaaaaaaaaaaaaabu64) as u64; r8 = (crate::simd_ops::ru64(rsi.wrapping_add(0x38u64))) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(0xc0u64))) as u64; rcx = ((rcx).wrapping_add(1)) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; rax = ((rdi.wrapping_sub(0x95u64)) as u32) as u32 as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1bu32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 14; continue; } bb = 4; continue; }
            4 => { r8 = ((P2) as u64) as u64; rax = (rax as u32 as i32 as i64) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(rax).wrapping_add(0x20d1fe8u64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x20d1fc0u64))) as u32 as u64; fl_l = ((rcx).wrapping_add(r8)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; j_tgt = (rcx) as u64; bb = 0xffff_fffe; continue; }
            5 => { crate::simd_ops::wu8(rdi.wrapping_add(rdx).wrapping_sub(0x95u64), (0x0u8) as u8); fl_l = (crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x14u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; fl_k = 1; bb = 15; continue; }
            6 => { crate::simd_ops::wu8(rdx.wrapping_add(0x25u64), (0x0u8) as u8); fl_l = (crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x14u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; fl_k = 1; bb = 15; continue; }
            7 => { crate::simd_ops::wu8(rdx.wrapping_add(0x26u64), (0x0u8) as u8); fl_l = (crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x14u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; fl_k = 1; bb = 15; continue; }
            8 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdx.wrapping_add(0x28u64), ((rax as u32)) as u32); fl_l = (crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x14u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; fl_k = 1; bb = 15; continue; }
            9 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu16(rdx.wrapping_add(0x28u64), ((rax as u16)) as u16); fl_l = (crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x14u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; fl_k = 1; bb = 15; continue; }
            10 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu16(rdx.wrapping_add(0x2au64), ((rax as u16)) as u16); fl_l = (crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x14u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; fl_k = 1; bb = 15; continue; }
            11 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdx.wrapping_add(0x2cu64), ((rax as u32)) as u32); fl_l = (crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x14u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; fl_k = 1; bb = 15; continue; }
            12 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu16(rdx.wrapping_add(0x2cu64), ((rax as u16)) as u16); fl_l = (crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x14u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; fl_k = 1; bb = 15; continue; }
            13 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu16(rdx.wrapping_add(0x2eu64), ((rax as u16)) as u16); fl_l = (crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x14u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; fl_k = 1; bb = 15; continue; }
            14 => { crate::simd_ops::wu32(rsi.wrapping_add(0x14u64), ((r9 as u32)) as u32); fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; fl_k = 0; bb = 15; continue; }
            15 => { if (match fl_k { 0 => ((((fl_l as u32)) & ((fl_r as u32))) != 0), 1 => (((fl_l as u32)) != 0), _ => false }) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = (0xffffffffu32) as u32 as u64; rcx = ((rsi.wrapping_add(0x8u64)) as u64) as u64; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rsi.wrapping_add(0x10u64), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 17; continue; }
            17 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 18; continue; }
            18 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            19 => { fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x40u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 20; continue; }
            20 => { r8 = (((rdi as u16)) as u32) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x1420d1e60u64) { 0x1c => { bb = 1; continue; } 0x35 => { bb = 2; continue; } 0x42 => { bb = 3; continue; } 0x91 => { bb = 4; continue; } 0xb0 => { bb = 5; continue; } 0xbe => { bb = 6; continue; } 0xc8 => { bb = 7; continue; } 0xd2 => { bb = 8; continue; } 0xdd => { bb = 9; continue; } 0xe9 => { bb = 10; continue; } 0xf5 => { bb = 11; continue; } 0x100 => { bb = 12; continue; } 0x10c => { bb = 13; continue; } 0x118 => { bb = 14; continue; } 0x11f => { bb = 15; continue; } 0x121 => { bb = 16; continue; } 0x133 => { bb = 17; continue; } 0x138 => { bb = 18; continue; } 0x143 => { bb = 19; continue; } 0x14a => { bb = 20; continue; } _ => { let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(j_tgt, rcx, rdx, r8, r9, s5, s6, s7, s8); } } }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d2050(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645790; const P1: u64 = 0x1426455a8; const P2: u64 = 0x140000000; const P3: u64 = 0x142645598; const P4: u64 = 0x1420d71c0;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; rsi = (((r9 as u8)) as u32) as u32 as u64; rdi = (((r8 as u16)) as u32) as u32 as u64; rbp = (rcx) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { xmm6 = 0u128; xmm0 = (xmm6) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rbx) as u64); rbx = ((rdx.wrapping_add(0x8u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (r14) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; r14 = ((rax as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((r14 as u32)) as u32); bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rax as u32)) as u32); fl_l = ((rsi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 16; continue; } bb = 6; continue; }
            6 => { rax = ((rdi.wrapping_sub(0x95u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x5u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 16; continue; } bb = 7; continue; }
            7 => { r8 = ((P2) as u64) as u64; rax = (rax as u32 as i32 as i64) as u64; rcx = (crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x20d21dcu64))) as u32 as u64; fl_l = ((rcx).wrapping_add(r8)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; j_tgt = (rcx) as u64; bb = 0xffff_fffe; continue; }
            8 => { xmm6 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x24u64))) as u32) as u128; bb = 14; continue; }
            9 => { xmm6 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x30u64))) as u32) as u128; bb = 14; continue; }
            10 => { xmm6 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x3cu64))) as u32) as u128; bb = 14; continue; }
            11 => { xmm6 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x48u64))) as u32) as u128; bb = 14; continue; }
            12 => { xmm2 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x24u64))) as u32) as u128; xmm6 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(core::ptr::read_unaligned((rbp.wrapping_add(0x30u64)) as *const u32)) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(core::ptr::read_unaligned((rbp.wrapping_add(0x30u64)) as *const u32)) }).to_bits() as u128); bb = 14; continue; }
            13 => { xmm2 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x3cu64))) as u32) as u128; xmm6 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(core::ptr::read_unaligned((rbp.wrapping_add(0x48u64)) as *const u32)) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(core::ptr::read_unaligned((rbp.wrapping_add(0x48u64)) as *const u32)) }).to_bits() as u128); bb = 14; continue; }
            14 => { crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rdx as u32)) as u32); fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 19; continue; } bb = 15; continue; }
            15 => { rax = (0xffffffffu32) as u32 as u64; rcx = (rbx) as u64; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; xmm0 = (xmm6) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            16 => { crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rdx as u32)) as u32); fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { rax = (0xffffffffu32) as u32 as u64; rcx = (rbx) as u64; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 18; continue; }
            18 => { xmm6 = 0u128; bb = 19; continue; }
            19 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; xmm0 = (xmm6) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            20 => { fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x40u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 22; continue; } bb = 21; continue; }
            21 => { r8 = (((rdi as u16)) as u32) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; }
            22 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; xmm0 = 0u128; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x1420d2050u64) { 0x25 => { bb = 1; continue; } 0x30 => { bb = 2; continue; } 0x4e => { bb = 3; continue; } 0x6c => { bb = 4; continue; } 0x79 => { bb = 5; continue; } 0x91 => { bb = 6; continue; } 0x9c => { bb = 7; continue; } 0xb2 => { bb = 8; continue; } 0xb9 => { bb = 9; continue; } 0xc0 => { bb = 10; continue; } 0xc7 => { bb = 11; continue; } 0xce => { bb = 12; continue; } 0xda => { bb = 13; continue; } 0xe4 => { bb = 14; continue; } 0xeb => { bb = 15; continue; } 0x11b => { bb = 16; continue; } 0x122 => { bb = 17; continue; } 0x133 => { bb = 18; continue; } 0x137 => { bb = 19; continue; } 0x156 => { bb = 20; continue; } 0x15d => { bb = 21; continue; } 0x175 => { bb = 22; continue; } _ => { let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(j_tgt, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; } } }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d2200(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645790; const P1: u64 = 0x1426455a8; const P2: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru64(r8.wrapping_add(0x38u64))) as u64; fl_r = (0x0u64) as u64; rbp = r8; rdi = (((r9 as u16)) as u32) as u32 as u64; r15 = rdx; rsi = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 18; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(r8.wrapping_add(0x40u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r14); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x10u64))) as u32 as u64; r14 = ((rax as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rbp.wrapping_add(0x8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((r14 as u32)) as u32); bb = 4; continue; }
            4 => { crate::simd_ops::lk_add_u32(rbp.wrapping_add(0x14u64), (1) as u32); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x60u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x40u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; if ((((fl_l as u8)) as i8) >= (((fl_r as u8)) as i8)) { bb = 8; continue; } bb = 5; continue; }
            5 => { rax = ((crate::simd_ops::ru8(r15.wrapping_add(0x4fd8u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = 0x174; rdx = ((rax.wrapping_add(r8)) as u64) as u64; bb = 9; continue; }
            7 => { rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x1f0u64))) as u64; rax = (0xaaaaaaaaaaaaaaabu64) as u64; rcx = ((rcx).wrapping_add(0x2u64)) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; rax = (((rcx) as i64).wrapping_mul((0x7cu64) as i64) as u64) as u64; rdx = ((rax.wrapping_add(r8)) as u64) as u64; bb = 9; continue; }
            8 => { rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x1f0u64))) as u64; rax = (0xaaaaaaaaaaaaaaabu64) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; rdx = (((rcx) as i64).wrapping_mul((0x7cu64) as i64) as u64) as u64; fl_l = ((rdx).wrapping_add(r8)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 9; continue; }
            9 => { rcx = ((rdi as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0xb3; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 10; continue; }
            10 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 11; continue; }
            11 => { rax = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x14u64))) as u32 as u64; fl_r = 0x1; crate::simd_ops::wu32(rbp.wrapping_add(0x14u64), ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); crate::simd_ops::wu64(rsi, rax); crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rax as u32)) as u32); if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = rsi; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            13 => { xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(0x48u64))) as u64) as u128; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x50u64))) as u32 as u64; bb = 15; continue; }
            14 => { xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(0x3cu64))) as u64) as u128; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x44u64))) as u32 as u64; bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x14u64))) as u32 as u64; fl_r = 0x1; crate::simd_ops::wu32(rbp.wrapping_add(0x14u64), ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); crate::simd_ops::wu64(rsi, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rax as u32)) as u32); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = 0xffffffff; rcx = rbp.wrapping_add(0x8); let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rbp.wrapping_add(0x10u64), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 17; continue; }
            17 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = rsi; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            18 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = rsi; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d2370(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x1420d0a10; const P2: u64 = 0x40; const P3: u64 = 0x1420d7530;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    fl_l = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64;
    rbx = rcx;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x60u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((rax as u8)) as u8); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = rbx; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    fl_l = (crate::simd_ops::ru64(r8.wrapping_add(P2))) as u64; fl_r = (0x0u64) as u64;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x60u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((rax as u8)) as u8); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = rbx; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx, rax);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d23d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x1420d0b60; const P2: u64 = 0x40; const P3: u64 = 0x1420d5bd0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; } fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; } xmm0 = 0u128; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d2670(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x72; const P1: u64 = 0x1417a23c0; const P2: u64 = 0x4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rbx = rdx;
    fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = rbx; crate::simd_ops::wu8(rbx.wrapping_add(P2), (0x1u8) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    crate::simd_ops::wu8(rdx.wrapping_add(P2), (0x1u8) as u8);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d26b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1420d2b20; const P1: u64 = 0x540; const P2: u64 = 0xae0; const P3: u64 = 0x1401dffd0; const P4: u64 = 0x55c; const P5: u64 = 0x142645790; const P6: u64 = 0xfffffffffffffffc; const P7: u64 = 0xfffffffffffffff4; const P8: u64 = 0x1426455a8; const P9: u64 = 0x24; const P10: u64 = 0x4; const P11: u64 = 0x142645598; const P12: u64 = 0x1417a23c0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r8 = (crate::simd_ops::ru64(rdx)) as u64; rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); r14 = rcx; rax = ((rdx.wrapping_add(P1)) as u64) as u64; fl_l = r8; fl_r = r8; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r15 = rdx; rax = ((r8.wrapping_add(P2)) as u64) as u64; rcx = 0x60; rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { rcx }) as u64; rdx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = ((r15.wrapping_add(P4)) as u64) as u64; rbp = 0x10; bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64; rsi = ((rax as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = ((rbx.wrapping_add(P7)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbx.wrapping_add(P6), ((rsi as u32)) as u32); bb = 3; continue; }
            3 => { crate::simd_ops::lk_add_u32(rbx, (1) as u32); fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P9))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = 0; fl_l = 0; fl_r = 0; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(rbx.wrapping_add(P10), ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64)); bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::lk_add_u32(rbx, (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = 0xffffffff; rcx = ((rbx.wrapping_add(P7)) as u64) as u64; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rbx.wrapping_add(P6), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P11), rax), rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rbx = ((rbx).wrapping_add(0x330u64)) as u64; fl_l = rbp; fl_r = (0x1u64) as u64; rbp = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(r15.wrapping_add(0x72u64))) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { crate::simd_ops::wu8(r14.wrapping_add(0x4u64), (0x1u8) as u8); rax = r14; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d27b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x72; const P1: u64 = 0x1420d40a0; const P2: u64 = 0x1420d4010; const P3: u64 = 0x1417a1dd0; const P4: u64 = 0x4; const P5: u64 = 0x3f08; const P6: u64 = 0x1420d4ae0; const P7: u64 = 0x4118; const P8: u64 = 0x4328; const P9: u64 = 0x4538;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rdi = (crate::simd_ops::ru64(rcx)) as u64; rbx = rdx; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = P1; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x1u32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rcx = rsp.wrapping_add(0x20); rax = P2; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = 0; fl_l = 0; fl_r = 0; xmm0 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), ((rax as u32)) as u32); crate::simd_ops::wu128(rsp.wrapping_add(0x34u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rbx, (0xbu32) as u32); rax = rbx; crate::simd_ops::wu8(rbx.wrapping_add(P4), (0x0u8) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { r8 = ((rdi.wrapping_add(P5)) as u64) as u64; rdx = 0x1; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = ((rdi.wrapping_add(P7)) as u64) as u64; rdx = 0x2; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = ((rdi.wrapping_add(P8)) as u64) as u64; rdx = 0x3; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = ((rdi.wrapping_add(P9)) as u64) as u64; rdx = 0x4; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rbx; crate::simd_ops::wu8(rbx.wrapping_add(P4), (0x1u8) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d2890(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1420d2b20; const P1: u64 = 0x1401e2660; const P2: u64 = 0x1420e41f0; const P3: u64 = 0x142645790; const P4: u64 = 0x1426455a8; const P5: u64 = 0x142646478; const P6: u64 = 0x142645598; const P7: u64 = 0x1420d40a0; const P8: u64 = 0x1420d4010; const P9: u64 = 0x1417a1dd0; const P10: u64 = 0x1420d4ae0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; r13 = rdx.wrapping_add(0x540); rax = 0x1; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(rdx.wrapping_add(0x4f89u64), ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64)); r8 = (crate::simd_ops::ru64(rdx)) as u64; rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rdi = rdx.wrapping_add(0x38d8); fl_l = r8; fl_r = r8; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r13); rcx = 0x60; rdx = rsp.wrapping_add(0x20); rax = r8.wrapping_add(0xae0); rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { rcx }) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = 0; fl_l = 0; fl_r = 0; r15 = (0xaaaaaaaaaaaaaaabu64) as u64; r12 = ((rsi as u32)) as u32 as u64; bb = 1; continue; }
            1 => { rcx = r13; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x10u64))) as u32 as u64; rbp = ((rax as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = r14.wrapping_add(0x8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(r14.wrapping_add(0x10u64), ((rbp as u32)) as u32); bb = 3; continue; }
            3 => { crate::simd_ops::lk_add_u32(r14.wrapping_add(0x14u64), (1) as u32); rdx = rsp.wrapping_add(0x30); rcx = (0x100000000u64) as u64; rax = r12; rax = (((rax) | (rcx))) as u64; rcx = ((rsi as u32)) as u32 as u64; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(r14, rax); rax = (tmp_xchg) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; r8 = rdi.wrapping_sub(0x90); r9 = rdi.wrapping_add(0x30); rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(r14.wrapping_add(0x18u64), ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64)); crate::simd_ops::wu64(r14.wrapping_add(0x38u64), r8); rax = r15; rcx = (crate::simd_ops::ru64(r9)) as u64; rcx = ((rcx).wrapping_add(1)) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; rax = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_add(rax)) as u64; let y_addr: u64 = r8.wrapping_add(rax.wrapping_mul(8)); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); xmm1 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x20u64))) as u128; fl_l = r8; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { bb = 5; continue; }
            5 => { let y_addr: u64 = r8; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(r8.wrapping_add(0x20u64), (xmm1) as u128); r8 = ((r8).wrapping_add(0x30u64)) as u64; fl_l = r8; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { rax = rdi; fl_l = rdi; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { let y_addr: u64 = rax; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rax.wrapping_add(0x20u64), (xmm1) as u128); rax = ((rax).wrapping_add(0x30u64)) as u64; fl_l = rax; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(0xd0u64), ((rsi as u32)) as u32); fl_l = (crate::simd_ops::lk_add_u32(r14.wrapping_add(0x14u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = 0xffffffff; rcx = r14.wrapping_add(0x8); let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(r14.wrapping_add(0x10u64), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            11 => { rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; r12 = ((r12).wrapping_add(1)) as u64; rdi = ((rdi).wrapping_add(0xd8u64)) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 1; continue; } bb = 12; continue; }
            12 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; fl_l = (crate::simd_ops::ru8(r15.wrapping_add(0x72u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 15; continue; } bb = 13; continue; }
            13 => { rax = P7; crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), (0x1u32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rcx = rsp.wrapping_add(0x40); rax = P8; xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x64u64), ((rax as u32)) as u32); crate::simd_ops::wu128(rsp.wrapping_add(0x54u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu32(r13, (0xbu32) as u32); crate::simd_ops::wu8(r13.wrapping_add(0x4u64), (0x0u8) as u8); bb = 16; continue; }
            15 => { r8 = r15.wrapping_add(0x3f08); rdx = 0x1; rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = r15.wrapping_add(0x4118); rdx = 0x2; rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = r15.wrapping_add(0x4328); rdx = 0x3; rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = r15.wrapping_add(0x4538); rdx = 0x4; rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(r13.wrapping_add(0x4u64), (0x1u8) as u8); bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rax = r13; rsp = ((rsp).wrapping_add(0x70u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d2b00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4f88;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = 0x11;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rax = 0x1; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(rcx.wrapping_add(P0), ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64)); }
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d2b40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x40; const P2: u64 = 0x18; const P3: u64 = 0x310; const P4: u64 = 0x2a0; const P5: u64 = 0x6c; const P6: u64 = 0x6a; const P7: u64 = 0x69; const P8: u64 = 0x68; const P9: u64 = 0x208; const P10: u64 = 0x1417a2060; const P11: u64 = 0x1417a1ff0; const P12: u64 = 0x1420d8180;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64; rbx = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 2; continue; }
            2 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P2))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 11; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P3))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); rdi = ((rdx.wrapping_add(P4)) as u64) as u64; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(P5))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 9; continue; } bb = 5; continue; }
            5 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(P6))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xdu8) as u8 as u64; if (((fl_l as u8)) >= ((fl_r as u8))) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(P7))) as u8 as u64; fl_r = (0xdu8) as u8 as u64; if (((fl_l as u8)) >= ((fl_r as u8))) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(P8))) as u8 as u64; fl_r = (0xdu8) as u8 as u64; if (((fl_l as u8)) < ((fl_r as u8))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; rdx = rsp.wrapping_add(0x30); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x0u32) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdi.wrapping_add(P7))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x31u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdi.wrapping_add(P8))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x32u64), ((rax as u8)) as u8); rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P9))) as u32 as u64; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P9))) as u32 as u64; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rdx = rdi; rcx = rbx; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 11; continue; }
            11 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d2cc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426ff0; const P1: u64 = 0x143426460; const P2: u64 = 0x1420d31b0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    xmm4 = ((crate::simd_ops::ru32(P0)) as u32) as u128;
    xmm5 = ((crate::simd_ops::ru32(P1)) as u32) as u128;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdx);
    xmm2 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x38u64))) as u32) as u128;
    xmm3 = (!(xmm4)) & (xmm2);
    fl_l = ((xmm5 as u32)) as u32 as u64; fl_r = ((xmm3 as u32)) as u32 as u64;
    if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rcx; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    xmm1 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x3cu64))) as u32) as u128;
    xmm2 = (!(xmm4)) & (xmm1);
    fl_l = ((xmm5 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64;
    if !(!(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32)))) { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx, rax); rax = rcx; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = rcx;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d2d20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426ff0; const P1: u64 = 0x10; const P2: u64 = 0x4; const P3: u64 = 0x143426890; const P4: u64 = 0x8; const P5: u64 = 0x1434264e4; const P6: u64 = 0x143426ea4; const P7: u64 = 0x1434268d0; const P8: u64 = 0x143427080; const P9: u64 = 0x14; const P10: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; xmm1 = ((crate::simd_ops::ru32(P0)) as u32) as u128; xmm2 = ((crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32) as u128; xmm4 = (!(xmm1)) & (xmm2); xmm2 = ((crate::simd_ops::ru32(r8.wrapping_add(P2))) as u32) as u128; xmm3 = (!(xmm1)) & (xmm2); xmm5 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) > f32::from_bits(xmm4 as u32) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(xmm4 as u32) }).to_bits() as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdx); xmm1 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x38u64))) as u32) as u128; xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x3cu64))) as u32) as u128; xmm3 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm2 = (xmm4) as u128; xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm4 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32)).sqrt().to_bits() as u128); xmm2 = 0u128; xmm1 = (xmm5) as u128; xmm3 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(xmm2 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm2 as u32) }).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) < f32::from_bits(crate::simd_ops::ru32(P3)) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P3)) }).to_bits() as u128); fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm4 as u32)) as u32 as u64; rax = (rax & !0xffu64) | (((if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = 0; fl_l = 0; fl_r = 0; rax = rcx; crate::simd_ops::wu64(rcx, rdx); rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { rax = ((crate::simd_ops::ru8(r8.wrapping_add(P4))) as u32) as u32 as u64; xmm1 = ((crate::simd_ops::ru32(r8)) as u32) as u128; xmm2 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(crate::simd_ops::ru32(P5)) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P5)) }).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) < f32::from_bits(crate::simd_ops::ru32(P6)) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P6)) }).to_bits() as u128); xmm5 = ((crate::simd_ops::ru32(P7)) as u32) as u128; xmm4 = ((crate::simd_ops::ru32(P8)) as u32) as u128; xmm0 = (((rax as u32)) as u32) as u128; rax = ((crate::simd_ops::ru8(r8.wrapping_add(P9))) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdx); rdx = 0; fl_l = 0; fl_r = 0; xmm3 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x38u64)))).to_bits() as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x10u64), (xmm6) as u128); xmm1 = (((rdx as u32)) as u32) as u128; xmm2 = crate::simd_ops::vpcmpeqd(xmm0, xmm1); xmm0 = crate::simd_ops::vblendvps(xmm4, xmm5, xmm2); xmm2 = ((crate::simd_ops::ru32(r8.wrapping_add(P10))) as u32) as u128; xmm1 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); crate::simd_ops::wu32(rcx, ((xmm1 as u32)) as u32); xmm1 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(crate::simd_ops::ru32(P5)) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P5)) }).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) < f32::from_bits(crate::simd_ops::ru32(P6)) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P6)) }).to_bits() as u128); xmm3 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x3cu64)))).to_bits() as u128); xmm0 = (((rax as u32)) as u32) as u128; xmm1 = (((rdx as u32)) as u32) as u128; xmm2 = crate::simd_ops::vpcmpeqd(xmm0, xmm1); crate::simd_ops::wu128(rsp, (xmm7) as u128); rax = rcx; xmm7 = (crate::simd_ops::ru128(rsp)) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; xmm0 = crate::simd_ops::vblendvps(xmm4, xmm5, xmm2); xmm1 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); crate::simd_ops::wu32(rcx.wrapping_add(P2), ((xmm1 as u32)) as u32); rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d2e50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426ff0; const P1: u64 = 0x143426460; const P2: u64 = 0x1434268d0; const P3: u64 = 0x143427080; const P4: u64 = 0x143426890; const P5: u64 = 0x1434264e4; const P6: u64 = 0x143426ea4;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x58u64)) as u64; xmm5 = ((crate::simd_ops::ru32(P0)) as u32) as u128; xmm4 = ((crate::simd_ops::ru32(P1)) as u32) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rdx); xmm2 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x68u64))) as u32) as u128; xmm3 = (!(xmm5)) & (xmm2); fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = ((xmm3 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { xmm1 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x6cu64))) as u32) as u128; xmm2 = (!(xmm5)) & (xmm1); fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = 0; fl_l = 0; fl_r = 0; rax = rcx; crate::simd_ops::wu64(rcx, rdx); rsp = ((rsp).wrapping_add(0x58u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x8u64))) as u32) as u32 as u64; xmm1 = ((crate::simd_ops::ru32(r8.wrapping_add(0x10u64))) as u32) as u128; xmm2 = ((crate::simd_ops::ru32(r8.wrapping_add(0x4u64))) as u32) as u128; xmm3 = (!(xmm5)) & (xmm2); xmm4 = (!(xmm5)) & (xmm1); xmm1 = 0u128; xmm1 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) > f32::from_bits(xmm4 as u32) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(xmm4 as u32) }).to_bits() as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm6) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm7) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm8) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x10u64), (xmm9) as u128); crate::simd_ops::wu128(rsp, (xmm10) as u128); xmm10 = ((crate::simd_ops::ru32(P2)) as u32) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rdx); rdx = 0; fl_l = 0; fl_r = 0; xmm2 = (xmm4) as u128; xmm4 = ((crate::simd_ops::ru32(P3)) as u32) as u128; xmm2 = 0u128; xmm8 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(xmm2 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm2 as u32) }).to_bits() as u128); xmm8 = (xmm8 & !0xffff_ffffu128) | ((if f32::from_bits(xmm8 as u32) < f32::from_bits(crate::simd_ops::ru32(P4)) { f32::from_bits(xmm8 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P4)) }).to_bits() as u128); xmm1 = ((crate::simd_ops::ru32(r8)) as u32) as u128; xmm2 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(crate::simd_ops::ru32(P5)) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P5)) }).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) < f32::from_bits(crate::simd_ops::ru32(P6)) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P6)) }).to_bits() as u128); xmm0 = (xmm10 & !0xffff_ffffu128) | ((f32::from_bits(xmm10 as u32) / f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm3 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x68u64)))).to_bits() as u128); xmm0 = (((rax as u32)) as u32) as u128; rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x14u64))) as u32) as u32 as u64; xmm1 = (((rdx as u32)) as u32) as u128; xmm2 = crate::simd_ops::vpcmpeqd(xmm0, xmm1); xmm1 = ((crate::simd_ops::ru32(r8.wrapping_add(0xcu64))) as u32) as u128; xmm0 = crate::simd_ops::vblendvps(xmm4, xmm10, xmm2); xmm2 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(crate::simd_ops::ru32(P5)) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P5)) }).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) < f32::from_bits(crate::simd_ops::ru32(P6)) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P6)) }).to_bits() as u128); xmm5 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm10 & !0xffff_ffffu128) | ((f32::from_bits(xmm10 as u32) / f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm3 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x6cu64)))).to_bits() as u128); xmm0 = (((rax as u32)) as u32) as u128; xmm1 = (((rdx as u32)) as u32) as u128; xmm2 = crate::simd_ops::vpcmpeqd(xmm0, xmm1); xmm0 = crate::simd_ops::vblendvps(xmm4, xmm10, xmm2); xmm1 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm0 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm3 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm4 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32)).sqrt().to_bits() as u128); xmm0 = (xmm10 & !0xffff_ffffu128) | ((f32::from_bits(xmm10 as u32) - f32::from_bits(xmm8 as u32)).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x7cu64), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x78u64), ((xmm5 as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm9 = 0u128; fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = ((xmm9 as u32)) as u32 as u64; xmm3 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rax); if (f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 5; continue; } bb = 4; continue; }
            4 => { if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 6; continue; } bb = 5; continue; }
            5 => { xmm9 = (xmm10 & !0xffff_ffffu128) | ((f32::from_bits(xmm10 as u32) / f32::from_bits(xmm4 as u32)).to_bits() as u128); bb = 6; continue; }
            6 => { xmm0 = (xmm9 & !0xffff_ffffu128) | ((f32::from_bits(xmm9 as u32) * f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x6cu64)))).to_bits() as u128); xmm1 = (xmm9 & !0xffff_ffffu128) | ((f32::from_bits(xmm9 as u32) * f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x68u64)))).to_bits() as u128); xmm10 = (crate::simd_ops::ru128(rsp)) as u128; xmm9 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x6cu64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), ((xmm1 as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu64(rcx, rax); rax = rcx; xmm0 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx))).to_bits() as u128); crate::simd_ops::wu32(rcx, ((xmm0 as u32)) as u32); xmm0 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0x4u64)))).to_bits() as u128); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((xmm0 as u32)) as u32); rsp = ((rsp).wrapping_add(0x58u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d3020(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426ff0; const P1: u64 = 0x143426890; const P2: u64 = 0x1434268d0; const P3: u64 = 0x1434264e4; const P4: u64 = 0x143426ea4; const P5: u64 = 0x143427080;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = ((rsp).wrapping_sub(0x38u64)) as u64; xmm1 = ((crate::simd_ops::ru32(P0)) as u32) as u128; xmm2 = ((crate::simd_ops::ru32(r8.wrapping_add(0x10u64))) as u32) as u128; xmm4 = (!(xmm1)) & (xmm2); xmm2 = ((crate::simd_ops::ru32(r8.wrapping_add(0x4u64))) as u32) as u128; xmm3 = (!(xmm1)) & (xmm2); xmm1 = 0u128; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rdx); r9 = rcx; xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x5cu64))) as u32) as u128; xmm2 = (xmm4) as u128; xmm2 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) > f32::from_bits(xmm4 as u32) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(xmm4 as u32) }).to_bits() as u128); xmm5 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(xmm1 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm1 as u32) }).to_bits() as u128); xmm1 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x58u64))) as u32) as u128; xmm5 = (xmm5 & !0xffff_ffffu128) | ((if f32::from_bits(xmm5 as u32) < f32::from_bits(crate::simd_ops::ru32(P1)) { f32::from_bits(xmm5 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P1)) }).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm3 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm1 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32)).sqrt().to_bits() as u128); fl_l = ((xmm5 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); rax = (rax & !0xffu64) | (((if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); xmm6 = 0u128; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = 0; fl_l = 0; fl_r = 0; rax = r9; crate::simd_ops::wu64(r9, rcx); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x38u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x4cu64))) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32) as u128; xmm3 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm4 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32)).sqrt().to_bits() as u128); fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x10u64), (xmm7) as u128); crate::simd_ops::wu128(rsp, (xmm8) as u128); xmm8 = ((crate::simd_ops::ru32(P2)) as u32) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rdx); if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { xmm1 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) - f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm0 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) - f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm3 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm4 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x58u64)))).to_bits() as u128); xmm7 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x5cu64)))).to_bits() as u128); bb = 5; continue; }
            4 => { xmm7 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x5cu64))) as u32) as u128; xmm4 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x58u64))) as u32) as u128; bb = 5; continue; }
            5 => { rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x8u64))) as u32) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; xmm1 = ((crate::simd_ops::ru32(r8)) as u32) as u128; xmm2 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(crate::simd_ops::ru32(P3)) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P3)) }).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) < f32::from_bits(crate::simd_ops::ru32(P4)) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P4)) }).to_bits() as u128); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm3 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm4 = ((crate::simd_ops::ru32(P5)) as u32) as u128; xmm0 = (((rax as u32)) as u32) as u128; rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x14u64))) as u32) as u32 as u64; xmm1 = (((rcx as u32)) as u32) as u128; xmm2 = crate::simd_ops::vpcmpeqd(xmm0, xmm1); xmm0 = crate::simd_ops::vblendvps(xmm4, xmm8, xmm2); xmm2 = ((crate::simd_ops::ru32(r8.wrapping_add(0xcu64))) as u32) as u128; xmm1 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); crate::simd_ops::wu32(r9, ((xmm1 as u32)) as u32); xmm1 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(crate::simd_ops::ru32(P3)) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P3)) }).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) < f32::from_bits(crate::simd_ops::ru32(P4)) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P4)) }).to_bits() as u128); xmm3 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; xmm0 = (((rax as u32)) as u32) as u128; xmm1 = (((rcx as u32)) as u32) as u128; xmm2 = crate::simd_ops::vpcmpeqd(xmm0, xmm1); xmm0 = crate::simd_ops::vblendvps(xmm4, xmm8, xmm2); xmm8 = (crate::simd_ops::ru128(rsp)) as u128; rax = r9; xmm1 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); crate::simd_ops::wu32(r9.wrapping_add(0x4u64), ((xmm1 as u32)) as u32); rsp = ((rsp).wrapping_add(0x38u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d31b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1434264e4; const P2: u64 = 0x143426ea4; const P3: u64 = 0x143427080; const P4: u64 = 0x1434268d0; const P5: u64 = 0x14; const P6: u64 = 0xc; const P7: u64 = 0x4;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(P0))) as u32) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; xmm1 = ((crate::simd_ops::ru32(r8)) as u32) as u128; xmm2 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(crate::simd_ops::ru32(P1)) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P1)) }).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) < f32::from_bits(crate::simd_ops::ru32(P2)) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P2)) }).to_bits() as u128); xmm4 = ((crate::simd_ops::ru32(P3)) as u32) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x10u64), (xmm6) as u128); xmm6 = ((crate::simd_ops::ru32(P4)) as u32) as u128; xmm0 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) / f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm3 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x38u64)))).to_bits() as u128); xmm0 = (((rax as u32)) as u32) as u128; rax = ((crate::simd_ops::ru8(r8.wrapping_add(P5))) as u32) as u32 as u64; xmm1 = (((rdx as u32)) as u32) as u128; xmm2 = crate::simd_ops::vpcmpeqd(xmm0, xmm1); xmm0 = crate::simd_ops::vblendvps(xmm4, xmm6, xmm2); xmm2 = ((crate::simd_ops::ru32(r8.wrapping_add(P6))) as u32) as u128; xmm1 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); crate::simd_ops::wu32(rcx, ((xmm1 as u32)) as u32); xmm1 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(crate::simd_ops::ru32(P1)) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P1)) }).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) < f32::from_bits(crate::simd_ops::ru32(P2)) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P2)) }).to_bits() as u128); xmm0 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) / f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm3 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x3cu64)))).to_bits() as u128); crate::simd_ops::wu128(rsp, (xmm7) as u128); xmm7 = (crate::simd_ops::ru128(rsp)) as u128; xmm0 = (((rax as u32)) as u32) as u128; xmm1 = (((rdx as u32)) as u32) as u128; xmm2 = crate::simd_ops::vpcmpeqd(xmm0, xmm1); xmm0 = crate::simd_ops::vblendvps(xmm4, xmm6, xmm2); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; rax = rcx; xmm1 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); crate::simd_ops::wu32(rcx.wrapping_add(P7), ((xmm1 as u32)) as u32); rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d3270(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1434264e4; const P2: u64 = 0x143426ea4; const P3: u64 = 0x1434268d0; const P4: u64 = 0x143427080; const P5: u64 = 0x14; const P6: u64 = 0xc; const P7: u64 = 0x4;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(P0))) as u32) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; xmm1 = ((crate::simd_ops::ru32(r8)) as u32) as u128; xmm2 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(crate::simd_ops::ru32(P1)) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P1)) }).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) < f32::from_bits(crate::simd_ops::ru32(P2)) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P2)) }).to_bits() as u128); xmm3 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x38u64)))).to_bits() as u128); xmm5 = ((crate::simd_ops::ru32(P3)) as u32) as u128; xmm4 = ((crate::simd_ops::ru32(P4)) as u32) as u128; xmm0 = (((rax as u32)) as u32) as u128; rax = ((crate::simd_ops::ru8(r8.wrapping_add(P5))) as u32) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x10u64), (xmm6) as u128); xmm1 = (((rdx as u32)) as u32) as u128; xmm2 = crate::simd_ops::vpcmpeqd(xmm0, xmm1); xmm0 = crate::simd_ops::vblendvps(xmm4, xmm5, xmm2); xmm2 = ((crate::simd_ops::ru32(r8.wrapping_add(P6))) as u32) as u128; xmm1 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); crate::simd_ops::wu32(rcx, ((xmm1 as u32)) as u32); xmm1 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(crate::simd_ops::ru32(P1)) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P1)) }).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) < f32::from_bits(crate::simd_ops::ru32(P2)) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P2)) }).to_bits() as u128); xmm3 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x3cu64)))).to_bits() as u128); xmm0 = (((rax as u32)) as u32) as u128; xmm1 = (((rdx as u32)) as u32) as u128; crate::simd_ops::wu128(rsp, (xmm7) as u128); rax = rcx; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp)) as u128; xmm2 = crate::simd_ops::vpcmpeqd(xmm0, xmm1); xmm0 = crate::simd_ops::vblendvps(xmm4, xmm5, xmm2); xmm1 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); crate::simd_ops::wu32(rcx.wrapping_add(P7), ((xmm1 as u32)) as u32); rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d3330(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426ff0; const P1: u64 = 0x143426460;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm4 = ((crate::simd_ops::ru32(P0)) as u32) as u128;
    xmm5 = ((crate::simd_ops::ru32(P1)) as u32) as u128;
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    xmm2 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x8u64))) as u32) as u128;
    xmm3 = (!(xmm4)) & (xmm2);
    fl_l = ((xmm5 as u32)) as u32 as u64; fl_r = ((xmm3 as u32)) as u32 as u64;
    if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
    xmm1 = ((crate::simd_ops::ru32(rsp.wrapping_add(0xcu64))) as u32) as u128;
    xmm2 = (!(xmm4)) & (xmm1);
    fl_l = ((xmm5 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64;
    if !(!(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32)))) { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d3380(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x143426ff0; const P2: u64 = 0x10; const P3: u64 = 0x143426890;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); xmm1 = ((crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32) as u128; xmm4 = ((crate::simd_ops::ru32(P1)) as u32) as u128; xmm3 = ((crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32) as u128; xmm2 = (xmm1) as u128; xmm2 = (!(xmm4)) & (xmm2); xmm1 = (!(xmm4)) & (xmm3); fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { xmm3 = (xmm1) as u128; xmm1 = 0u128; xmm1 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(xmm3 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm3 as u32) }).to_bits() as u128); xmm2 = 0u128; xmm5 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(xmm2 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm2 as u32) }).to_bits() as u128); xmm2 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x8u64))) as u32) as u128; xmm5 = (xmm5 & !0xffff_ffffu128) | ((if f32::from_bits(xmm5 as u32) < f32::from_bits(crate::simd_ops::ru32(P3)) { f32::from_bits(xmm5 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P3)) }).to_bits() as u128); xmm3 = (!(xmm4)) & (xmm2); fl_l = ((xmm5 as u32)) as u32 as u64; fl_r = ((xmm3 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { xmm1 = ((crate::simd_ops::ru32(rsp.wrapping_add(0xcu64))) as u32) as u128; xmm2 = (!(xmm4)) & (xmm1); fl_l = ((xmm5 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            4 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0xcu64))) as u32) as u128; xmm4 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(xmm1 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm1 as u32) }).to_bits() as u128); xmm1 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x8u64))) as u32) as u128; xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm3 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm5 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32)).sqrt().to_bits() as u128); xmm2 = 0u128; xmm1 = (xmm4) as u128; xmm3 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(xmm2 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm2 as u32) }).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) < f32::from_bits(crate::simd_ops::ru32(P3)) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P3)) }).to_bits() as u128); fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm5 as u32)) as u32 as u64; rax = (rax & !0xffu64) | (((if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d3430(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143426ff0; const P1: u64 = 0x10; const P2: u64 = 0x4; const P3: u64 = 0x143426890;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut xmm4: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm1 = ((crate::simd_ops::ru32(P0)) as u32) as u128;
    xmm2 = ((crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32) as u128;
    xmm4 = (!(xmm1)) & (xmm2);
    xmm2 = ((crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32) as u128;
    xmm3 = (!(xmm1)) & (xmm2);
    xmm2 = (xmm4) as u128;
    xmm1 = 0u128;
    xmm1 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) > f32::from_bits(xmm4 as u32) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(xmm4 as u32) }).to_bits() as u128);
    xmm2 = 0u128;
    xmm0 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(xmm2 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm2 as u32) }).to_bits() as u128);
    xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) < f32::from_bits(crate::simd_ops::ru32(P3)) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P3)) }).to_bits() as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d3470(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426ff0; const P1: u64 = 0x143426460; const P2: u64 = 0x1434264e4; const P3: u64 = 0x143426ea4; const P4: u64 = 0x1434268d0; const P5: u64 = 0x8; const P6: u64 = 0x143427080;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x18u64)) as u64;
    xmm4 = ((crate::simd_ops::ru32(P0)) as u32) as u128;
    xmm2 = 0u128;
    xmm1 = (xmm2 & !0xffff_ffffu128) | (xmm0 & 0xffff_ffffu128);
    xmm2 = ((crate::simd_ops::ru32(P1)) as u32) as u128;
    xmm5 = (!(xmm4)) & (xmm1);
    fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm5 as u32)) as u32 as u64;
    crate::simd_ops::wu128(rsp, (xmm6) as u128);
    xmm6 = (xmm0) as u128;
    if !(!(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32)))) { xmm0 = 0u128; xmm6 = (crate::simd_ops::ru128(rsp)) as u128; rsp = ((rsp).wrapping_add(0x18u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    xmm1 = ((crate::simd_ops::ru32(rdx)) as u32) as u128;
    xmm3 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(crate::simd_ops::ru32(P2)) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P2)) }).to_bits() as u128);
    xmm3 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) < f32::from_bits(crate::simd_ops::ru32(P3)) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P3)) }).to_bits() as u128);
    xmm4 = ((crate::simd_ops::ru32(P4)) as u32) as u128;
    rax = 0;
    fl_l = 0; fl_r = 0;
    xmm1 = (((rax as u32)) as u32) as u128;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P5))) as u32) as u32 as u64;
    xmm0 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) / f32::from_bits(xmm3 as u32)).to_bits() as u128);
    xmm3 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128);
    xmm6 = (crate::simd_ops::ru128(rsp)) as u128;
    xmm0 = (((rax as u32)) as u32) as u128;
    xmm2 = crate::simd_ops::vpcmpeqd(xmm0, xmm1);
    xmm1 = ((crate::simd_ops::ru32(P6)) as u32) as u128;
    xmm0 = crate::simd_ops::vblendvps(xmm1, xmm4, xmm2);
    xmm0 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128);
    rsp = ((rsp).wrapping_add(0x18u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d3510(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426ff0; const P1: u64 = 0x4; const P2: u64 = 0x143426890; const P3: u64 = 0x1434264e4; const P4: u64 = 0x143426ea4; const P5: u64 = 0x1434268d0; const P6: u64 = 0x8; const P7: u64 = 0x143427080;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; xmm5 = ((crate::simd_ops::ru32(P0)) as u32) as u128; xmm3 = ((crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32) as u128; xmm4 = (!(xmm5)) & (xmm3); xmm1 = 0u128; xmm2 = (xmm1 & !0xffff_ffffu128) | (xmm0 & 0xffff_ffffu128); crate::simd_ops::wu128(rsp.wrapping_add(0x10u64), (xmm6) as u128); xmm6 = (!(xmm5)) & (xmm2); xmm2 = 0u128; xmm1 = (xmm4) as u128; xmm5 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(xmm2 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm2 as u32) }).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((if f32::from_bits(xmm5 as u32) < f32::from_bits(crate::simd_ops::ru32(P2)) { f32::from_bits(xmm5 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P2)) }).to_bits() as u128); fl_l = ((xmm5 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; crate::simd_ops::wu128(rsp, (xmm7) as u128); xmm7 = (xmm0) as u128; xmm0 = 0u128; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm1 = ((crate::simd_ops::ru32(rdx)) as u32) as u128; xmm3 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(crate::simd_ops::ru32(P3)) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P3)) }).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) < f32::from_bits(crate::simd_ops::ru32(P4)) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P4)) }).to_bits() as u128); xmm2 = ((crate::simd_ops::ru32(P5)) as u32) as u128; rax = 0; fl_l = 0; fl_r = 0; xmm1 = (((rax as u32)) as u32) as u128; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P6))) as u32) as u32 as u64; xmm0 = (((rax as u32)) as u32) as u128; xmm4 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm3 = crate::simd_ops::vpcmpeqd(xmm0, xmm1); xmm0 = ((crate::simd_ops::ru32(P7)) as u32) as u128; xmm0 = crate::simd_ops::vblendvps(xmm0, xmm2, xmm3); xmm0 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 2; continue; }
            2 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp)) as u128; rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d35c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426460; const P1: u64 = 0x143426ff0; const P2: u64 = 0x4; const P3: u64 = 0x1434268d0; const P4: u64 = 0x143426890; const P5: u64 = 0x1434264e4; const P6: u64 = 0x143426ea4; const P7: u64 = 0x8; const P8: u64 = 0x143427080;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x38u64)) as u64; xmm1 = 0u128; xmm2 = (xmm1 & !0xffff_ffffu128) | (xmm0 & 0xffff_ffffu128); xmm1 = ((crate::simd_ops::ru32(P0)) as u32) as u128; crate::simd_ops::wu128(rsp, (xmm8) as u128); xmm8 = ((crate::simd_ops::ru32(P1)) as u32) as u128; xmm4 = (!(xmm8)) & (xmm2); fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm4 as u32)) as u32 as u64; xmm5 = (xmm0) as u128; xmm3 = (xmm0) as u128; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm0 = 0u128; xmm8 = (crate::simd_ops::ru128(rsp)) as u128; rsp = ((rsp).wrapping_add(0x38u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { xmm1 = ((crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32) as u128; xmm3 = (!(xmm8)) & (xmm1); xmm1 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); rax = 0; fl_l = 0; fl_r = 0; xmm6 = ((crate::simd_ops::ru32(P3)) as u32) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x10u64), (xmm7) as u128); xmm7 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) > f32::from_bits(xmm1 as u32) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(xmm1 as u32) }).to_bits() as u128); xmm7 = (xmm7 & !0xffff_ffffu128) | ((if f32::from_bits(xmm7 as u32) < f32::from_bits(crate::simd_ops::ru32(P4)) { f32::from_bits(xmm7 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P4)) }).to_bits() as u128); xmm1 = ((crate::simd_ops::ru32(rdx)) as u32) as u128; xmm3 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(crate::simd_ops::ru32(P5)) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P5)) }).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) < f32::from_bits(crate::simd_ops::ru32(P6)) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P6)) }).to_bits() as u128); xmm0 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) / f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm3 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm1 = (((rax as u32)) as u32) as u128; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P7))) as u32) as u32 as u64; xmm0 = (((rax as u32)) as u32) as u128; xmm2 = crate::simd_ops::vpcmpeqd(xmm0, xmm1); xmm1 = ((crate::simd_ops::ru32(P8)) as u32) as u128; xmm0 = crate::simd_ops::vblendvps(xmm1, xmm6, xmm2); xmm2 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm3 = (xmm2) & (xmm8); xmm0 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) - f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm2 = (!(xmm8)) & (xmm2); xmm8 = (crate::simd_ops::ru128(rsp)) as u128; xmm1 = 0u128; xmm1 = (xmm6) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm5 = (xmm3) | (xmm1); xmm1 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; xmm0 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); rsp = ((rsp).wrapping_add(0x38u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d36b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426ff0; const P1: u64 = 0x4; const P2: u64 = 0x143426890; const P3: u64 = 0x143426ea4; const P4: u64 = 0x1434264e4; const P5: u64 = 0x1434268d0; const P6: u64 = 0x8; const P7: u64 = 0x143427080;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; xmm4 = ((crate::simd_ops::ru32(P0)) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32) as u128; xmm3 = (xmm0) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x10u64), (xmm6) as u128); xmm0 = 0u128; xmm6 = (xmm0 & !0xffff_ffffu128) | (xmm3 & 0xffff_ffffu128); xmm3 = (!(xmm4)) & (xmm1); xmm1 = 0u128; xmm5 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) > f32::from_bits(xmm1 as u32) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(xmm1 as u32) }).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((if f32::from_bits(xmm5 as u32) < f32::from_bits(crate::simd_ops::ru32(P2)) { f32::from_bits(xmm5 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P2)) }).to_bits() as u128); crate::simd_ops::wu128(rsp, (xmm7) as u128); xmm7 = (!(xmm4)) & (xmm6); fl_l = ((xmm5 as u32)) as u32 as u64; fl_r = ((xmm7 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm2 = ((crate::simd_ops::ru32(P3)) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(rdx)) as u32) as u128; xmm3 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(crate::simd_ops::ru32(P4)) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P4)) }).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) < f32::from_bits(xmm2 as u32) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(xmm2 as u32) }).to_bits() as u128); xmm1 = (xmm2) as u128; xmm2 = (xmm6) & (xmm4); xmm4 = ((crate::simd_ops::ru32(P5)) as u32) as u128; xmm1 = (xmm4) as u128; xmm2 = (xmm2) | (xmm1); xmm0 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) - f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm1 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) - f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm0 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) / f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); rax = 0; fl_l = 0; fl_r = 0; xmm1 = (((rax as u32)) as u32) as u128; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P6))) as u32) as u32 as u64; xmm0 = (((rax as u32)) as u32) as u128; xmm2 = crate::simd_ops::vpcmpeqd(xmm0, xmm1); xmm1 = ((crate::simd_ops::ru32(P7)) as u32) as u128; xmm0 = crate::simd_ops::vblendvps(xmm1, xmm4, xmm2); xmm0 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 2; continue; }
            2 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp)) as u128; rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d3780(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434264e4; const P1: u64 = 0x1434268d0; const P2: u64 = 0x143426ea4; const P3: u64 = 0x8; const P4: u64 = 0x143427080;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x18u64)) as u64;
    xmm2 = ((crate::simd_ops::ru32(rdx)) as u32) as u128;
    xmm5 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(crate::simd_ops::ru32(P0)) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P0)) }).to_bits() as u128);
    xmm4 = ((crate::simd_ops::ru32(P1)) as u32) as u128;
    xmm5 = (xmm5 & !0xffff_ffffu128) | ((if f32::from_bits(xmm5 as u32) < f32::from_bits(crate::simd_ops::ru32(P2)) { f32::from_bits(xmm5 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P2)) }).to_bits() as u128);
    crate::simd_ops::wu128(rsp, (xmm6) as u128);
    rax = 0;
    fl_l = 0; fl_r = 0;
    xmm6 = (xmm0) as u128;
    xmm1 = (((rax as u32)) as u32) as u128;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P3))) as u32) as u32 as u64;
    xmm0 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) / f32::from_bits(xmm5 as u32)).to_bits() as u128);
    xmm3 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128);
    xmm6 = (crate::simd_ops::ru128(rsp)) as u128;
    xmm0 = (((rax as u32)) as u32) as u128;
    xmm2 = crate::simd_ops::vpcmpeqd(xmm0, xmm1);
    xmm1 = ((crate::simd_ops::ru32(P4)) as u32) as u128;
    xmm0 = crate::simd_ops::vblendvps(xmm1, xmm4, xmm2);
    xmm0 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128);
    rsp = ((rsp).wrapping_add(0x18u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d37f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1434264e4; const P1: u64 = 0x143426ea4; const P2: u64 = 0x1434268d0; const P3: u64 = 0x8; const P4: u64 = 0x143427080;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm2 = ((crate::simd_ops::ru32(rdx)) as u32) as u128;
    xmm4 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(crate::simd_ops::ru32(P0)) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P0)) }).to_bits() as u128);
    xmm4 = (xmm4 & !0xffff_ffffu128) | ((if f32::from_bits(xmm4 as u32) < f32::from_bits(crate::simd_ops::ru32(P1)) { f32::from_bits(xmm4 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P1)) }).to_bits() as u128);
    xmm2 = ((crate::simd_ops::ru32(P2)) as u32) as u128;
    xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128);
    rax = 0;
    fl_l = 0; fl_r = 0;
    xmm1 = (((rax as u32)) as u32) as u128;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P3))) as u32) as u32 as u64;
    xmm0 = (((rax as u32)) as u32) as u128;
    xmm3 = crate::simd_ops::vpcmpeqd(xmm0, xmm1);
    xmm0 = ((crate::simd_ops::ru32(P4)) as u32) as u128;
    xmm0 = crate::simd_ops::vblendvps(xmm0, xmm2, xmm3);
    xmm0 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d3840(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143426ff0; const P1: u64 = 0x143426460;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm1 = ((crate::simd_ops::ru32(P0)) as u32) as u128;
    xmm2 = ((crate::simd_ops::ru32(P1)) as u32) as u128;
    xmm3 = (!(xmm1)) & (xmm0);
    fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm3 as u32)) as u32 as u64;
    rax = (rax & !0xffu64) | (((if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64));
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d3870(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x143426ff0; const P2: u64 = 0x143426890;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm5: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm3 = ((crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32) as u128;
    xmm5 = ((crate::simd_ops::ru32(P1)) as u32) as u128;
    xmm2 = (xmm3) as u128;
    xmm2 = (!(xmm5)) & (xmm3);
    xmm1 = (xmm0) as u128;
    xmm0 = 0u128;
    xmm1 = (xmm0 & !0xffff_ffffu128) | (xmm1 & 0xffff_ffffu128);
    xmm5 = (!(xmm5)) & (xmm1);
    xmm1 = 0u128;
    xmm3 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(xmm1 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm1 as u32) }).to_bits() as u128);
    xmm3 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) < f32::from_bits(crate::simd_ops::ru32(P2)) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P2)) }).to_bits() as u128);
    fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm5 as u32)) as u32 as u64;
    rax = (rax & !0xffu64) | (((if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64));
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d38c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x143426ff0; const P2: u64 = 0x143426890;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm2 = ((crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32) as u128;
    xmm3 = ((crate::simd_ops::ru32(P1)) as u32) as u128;
    xmm1 = 0u128;
    xmm1 = (!(xmm3)) & (xmm2);
    xmm2 = 0u128;
    xmm0 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(xmm2 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm2 as u32) }).to_bits() as u128);
    xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) < f32::from_bits(crate::simd_ops::ru32(P2)) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P2)) }).to_bits() as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d38f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434264e4; const P1: u64 = 0x143426ea4; const P2: u64 = 0x1434268d0; const P3: u64 = 0x143427080; const P4: u64 = 0x143426ff0; const P5: u64 = 0x143426890;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x58u64)) as u64; xmm5 = (xmm0) as u128; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm2 = ((crate::simd_ops::ru32(rdx)) as u32) as u128; xmm4 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(crate::simd_ops::ru32(P0)) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P0)) }).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((if f32::from_bits(xmm4 as u32) < f32::from_bits(crate::simd_ops::ru32(P1)) { f32::from_bits(xmm4 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P1)) }).to_bits() as u128); xmm2 = ((crate::simd_ops::ru32(P2)) as u32) as u128; xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); rax = 0; fl_l = 0; fl_r = 0; xmm1 = (((rax as u32)) as u32) as u128; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x8u64))) as u32) as u32 as u64; xmm0 = (((rax as u32)) as u32) as u128; xmm3 = crate::simd_ops::vpcmpeqd(xmm0, xmm1); xmm0 = ((crate::simd_ops::ru32(P3)) as u32) as u128; xmm0 = crate::simd_ops::vblendvps(xmm0, xmm2, xmm3); xmm0 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); rsp = ((rsp).wrapping_add(0x58u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm6) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm7) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm8) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x10u64), (xmm9) as u128); xmm2 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32) as u128; xmm6 = ((crate::simd_ops::ru32(P4)) as u32) as u128; xmm9 = ((crate::simd_ops::ru32(P5)) as u32) as u128; xmm8 = 0u128; xmm1 = (xmm2) as u128; xmm0 = 0u128; xmm1 = (!(xmm6)) & (xmm2); xmm7 = (xmm0 & !0xffff_ffffu128) | (xmm5 & 0xffff_ffffu128); xmm4 = 0u128; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { xmm2 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(xmm8 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm8 as u32) }).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) < f32::from_bits(xmm9 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm9 as u32) }).to_bits() as u128); xmm3 = (!(xmm6)) & (xmm7); fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm3 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 8; continue; } bb = 4; continue; }
            4 => { xmm1 = ((crate::simd_ops::ru32(rdx)) as u32) as u128; xmm3 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(crate::simd_ops::ru32(P0)) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P0)) }).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) < f32::from_bits(crate::simd_ops::ru32(P1)) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P1)) }).to_bits() as u128); xmm2 = ((crate::simd_ops::ru32(P2)) as u32) as u128; rax = 0; fl_l = 0; fl_r = 0; xmm1 = (((rax as u32)) as u32) as u128; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x8u64))) as u32) as u32 as u64; xmm0 = (((rax as u32)) as u32) as u128; xmm4 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm3 = crate::simd_ops::vpcmpeqd(xmm0, xmm1); xmm0 = ((crate::simd_ops::ru32(P3)) as u32) as u128; xmm0 = crate::simd_ops::vblendvps(xmm0, xmm2, xmm3); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 8; continue; }
            5 => { xmm5 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(xmm8 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm8 as u32) }).to_bits() as u128); crate::simd_ops::wu128(rsp, (xmm10) as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((if f32::from_bits(xmm5 as u32) < f32::from_bits(xmm9 as u32) { f32::from_bits(xmm5 as u32) } else { f32::from_bits(xmm9 as u32) }).to_bits() as u128); xmm10 = (!(xmm6)) & (xmm7); fl_l = ((xmm5 as u32)) as u32 as u64; fl_r = ((xmm10 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { xmm4 = ((crate::simd_ops::ru32(P2)) as u32) as u128; xmm2 = ((crate::simd_ops::ru32(P1)) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(rdx)) as u32) as u128; xmm3 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(crate::simd_ops::ru32(P0)) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P0)) }).to_bits() as u128); xmm1 = (xmm2) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) < f32::from_bits(xmm2 as u32) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(xmm2 as u32) }).to_bits() as u128); xmm1 = (xmm4) as u128; xmm0 = (xmm10 & !0xffff_ffffu128) | ((f32::from_bits(xmm10 as u32) - f32::from_bits(xmm5 as u32)).to_bits() as u128); rax = 0; fl_l = 0; fl_r = 0; xmm2 = (xmm7) & (xmm6); xmm2 = (xmm2) | (xmm1); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm1 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) - f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm0 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) / f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm1 = (((rax as u32)) as u32) as u128; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x8u64))) as u32) as u32 as u64; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (((rax as u32)) as u32) as u128; xmm2 = crate::simd_ops::vpcmpeqd(xmm0, xmm1); xmm1 = ((crate::simd_ops::ru32(P3)) as u32) as u128; xmm0 = crate::simd_ops::vblendvps(xmm1, xmm4, xmm2); xmm4 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 7; continue; }
            7 => { xmm10 = (crate::simd_ops::ru128(rsp)) as u128; bb = 8; continue; }
            8 => { xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; xmm9 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; xmm0 = (xmm4) as u128; rsp = ((rsp).wrapping_add(0x58u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d3ab0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426ff0; const P1: u64 = 0x143426460; const P2: u64 = 0x1420d31b0; const P3: u64 = 0x1420d2e50; const P4: u64 = 0xc; const P5: u64 = 0x1420d3b80; const P6: u64 = 0x4;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = ((rsp).wrapping_sub(0x48u64)) as u64; r10 = r8; fl_l = ((r9 as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { fl_l = ((r9 as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 8; continue; } bb = 2; continue; }
            2 => { fl_l = ((r9 as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { xmm4 = ((crate::simd_ops::ru32(P0)) as u32) as u128; xmm5 = ((crate::simd_ops::ru32(P1)) as u32) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); xmm2 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32) as u128; xmm3 = (!(xmm4)) & (xmm2); fl_l = ((xmm5 as u32)) as u32 as u64; fl_r = ((xmm3 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { xmm1 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x24u64))) as u32) as u128; xmm2 = (!(xmm4)) & (xmm1); fl_l = ((xmm5 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx, rax); rax = rcx; rsp = ((rsp).wrapping_add(0x48u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rcx; rsp = ((rsp).wrapping_add(0x48u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rcx; rsp = ((rsp).wrapping_add(0x48u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            8 => { xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x5cu64))) as u32) as u128; rdx = ((r8.wrapping_add(P4)) as u64) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm6) as u128); r8 = (((r9 as u8)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm6 = (xmm0) as u128; xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x58u64))) as u32) as u128; rdx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rcx.wrapping_add(P6), ((xmm6 as u32)) as u32); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; rax = rcx; crate::simd_ops::wu32(rcx, ((xmm0 as u32)) as u32); rsp = ((rsp).wrapping_add(0x48u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d3b80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434264e4; const P1: u64 = 0x1434268d0; const P2: u64 = 0x143426ea4; const P3: u64 = 0x143427080; const P4: u64 = 0x143426ff0; const P5: u64 = 0x143426460; const P6: u64 = 0x143426890;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x48u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x10u64), (xmm8) as u128); xmm8 = (xmm0) as u128; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm2 = ((crate::simd_ops::ru32(rdx)) as u32) as u128; xmm5 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(crate::simd_ops::ru32(P0)) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P0)) }).to_bits() as u128); xmm4 = ((crate::simd_ops::ru32(P1)) as u32) as u128; xmm5 = (xmm5 & !0xffff_ffffu128) | ((if f32::from_bits(xmm5 as u32) < f32::from_bits(crate::simd_ops::ru32(P2)) { f32::from_bits(xmm5 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P2)) }).to_bits() as u128); rax = 0; fl_l = 0; fl_r = 0; xmm1 = (((rax as u32)) as u32) as u128; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x8u64))) as u32) as u32 as u64; xmm0 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) / f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm3 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm0 = (((rax as u32)) as u32) as u128; xmm2 = crate::simd_ops::vpcmpeqd(xmm0, xmm1); xmm1 = ((crate::simd_ops::ru32(P3)) as u32) as u128; xmm0 = crate::simd_ops::vblendvps(xmm1, xmm4, xmm2); xmm0 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; rsp = ((rsp).wrapping_add(0x48u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { crate::simd_ops::wu128(rsp, (xmm9) as u128); xmm9 = ((crate::simd_ops::ru32(P4)) as u32) as u128; xmm0 = 0u128; xmm1 = (xmm0 & !0xffff_ffffu128) | (xmm8 & 0xffff_ffffu128); xmm0 = ((crate::simd_ops::ru32(P5)) as u32) as u128; xmm1 = (!(xmm9)) & (xmm1); fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { xmm1 = ((crate::simd_ops::ru32(rdx)) as u32) as u128; xmm3 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(crate::simd_ops::ru32(P0)) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P0)) }).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) < f32::from_bits(crate::simd_ops::ru32(P2)) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P2)) }).to_bits() as u128); xmm4 = ((crate::simd_ops::ru32(P1)) as u32) as u128; xmm9 = (crate::simd_ops::ru128(rsp)) as u128; rax = 0; fl_l = 0; fl_r = 0; xmm1 = (((rax as u32)) as u32) as u128; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x8u64))) as u32) as u32 as u64; xmm0 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) / f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm3 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm0 = (((rax as u32)) as u32) as u128; xmm2 = crate::simd_ops::vpcmpeqd(xmm0, xmm1); xmm1 = ((crate::simd_ops::ru32(P3)) as u32) as u128; xmm0 = crate::simd_ops::vblendvps(xmm1, xmm4, xmm2); xmm0 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; rsp = ((rsp).wrapping_add(0x48u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { xmm9 = (crate::simd_ops::ru128(rsp)) as u128; xmm0 = 0u128; xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; rsp = ((rsp).wrapping_add(0x48u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { xmm1 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32) as u128; xmm3 = (!(xmm9)) & (xmm1); xmm1 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm6) as u128); rax = 0; fl_l = 0; fl_r = 0; xmm6 = ((crate::simd_ops::ru32(P1)) as u32) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm7) as u128); xmm7 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) > f32::from_bits(xmm1 as u32) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(xmm1 as u32) }).to_bits() as u128); xmm7 = (xmm7 & !0xffff_ffffu128) | ((if f32::from_bits(xmm7 as u32) < f32::from_bits(crate::simd_ops::ru32(P6)) { f32::from_bits(xmm7 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P6)) }).to_bits() as u128); xmm1 = ((crate::simd_ops::ru32(rdx)) as u32) as u128; xmm3 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(crate::simd_ops::ru32(P0)) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P0)) }).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) < f32::from_bits(crate::simd_ops::ru32(P2)) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P2)) }).to_bits() as u128); xmm0 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) / f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm3 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; xmm1 = (((rax as u32)) as u32) as u128; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x8u64))) as u32) as u32 as u64; xmm0 = (((rax as u32)) as u32) as u128; xmm2 = crate::simd_ops::vpcmpeqd(xmm0, xmm1); xmm1 = ((crate::simd_ops::ru32(P3)) as u32) as u128; xmm0 = crate::simd_ops::vblendvps(xmm1, xmm6, xmm2); xmm2 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm3 = (xmm2) & (xmm9); xmm0 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) - f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm2 = (!(xmm9)) & (xmm2); xmm9 = (crate::simd_ops::ru128(rsp)) as u128; xmm1 = 0u128; xmm1 = (xmm6) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; xmm5 = (xmm3) | (xmm1); xmm1 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm0 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); rsp = ((rsp).wrapping_add(0x48u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d3d40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426ff0; const P1: u64 = 0x143426890; const P2: u64 = 0x1434264e4; const P3: u64 = 0x143426ea4; const P4: u64 = 0x1434268d0; const P5: u64 = 0x143427080; const P6: u64 = 0x1420d3020; const P7: u64 = 0x1420d38f0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm6) as u128); r11 = r8; r10 = rcx; fl_l = ((r9 as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { fl_l = ((r9 as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 7; continue; } bb = 2; continue; }
            2 => { fl_l = ((r9 as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { xmm1 = ((crate::simd_ops::ru32(P0)) as u32) as u128; xmm2 = ((crate::simd_ops::ru32(r8.wrapping_add(0x10u64))) as u32) as u128; xmm4 = (!(xmm1)) & (xmm2); xmm2 = ((crate::simd_ops::ru32(r8.wrapping_add(0x4u64))) as u32) as u128; xmm3 = (!(xmm1)) & (xmm2); xmm5 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) > f32::from_bits(xmm4 as u32) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(xmm4 as u32) }).to_bits() as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); xmm1 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32) as u128; xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x24u64))) as u32) as u128; xmm3 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm2 = (xmm4) as u128; xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm4 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32)).sqrt().to_bits() as u128); xmm2 = 0u128; xmm1 = (xmm5) as u128; xmm3 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(xmm2 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm2 as u32) }).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) < f32::from_bits(crate::simd_ops::ru32(P1)) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P1)) }).to_bits() as u128); fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm4 as u32)) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm8) as u128); rax = (rax & !0xffu64) | (((if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); xmm0 = 0u128; xmm8 = 0u128; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { xmm1 = ((crate::simd_ops::ru32(r8.wrapping_add(0xcu64))) as u32) as u128; xmm2 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(crate::simd_ops::ru32(P2)) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P2)) }).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) < f32::from_bits(crate::simd_ops::ru32(P3)) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P3)) }).to_bits() as u128); rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x14u64))) as u32) as u32 as u64; xmm5 = ((crate::simd_ops::ru32(P4)) as u32) as u128; xmm4 = ((crate::simd_ops::ru32(P5)) as u32) as u128; xmm0 = (((rax as u32)) as u32) as u128; rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x8u64))) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); rdx = 0; fl_l = 0; fl_r = 0; xmm3 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x24u64)))).to_bits() as u128); xmm1 = (((rdx as u32)) as u32) as u128; xmm2 = crate::simd_ops::vpcmpeqd(xmm0, xmm1); xmm0 = crate::simd_ops::vblendvps(xmm4, xmm5, xmm2); xmm2 = ((crate::simd_ops::ru32(r8)) as u32) as u128; xmm8 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(crate::simd_ops::ru32(P2)) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P2)) }).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) < f32::from_bits(crate::simd_ops::ru32(P3)) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P3)) }).to_bits() as u128); xmm3 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x20u64)))).to_bits() as u128); xmm0 = (((rax as u32)) as u32) as u128; xmm1 = (((rdx as u32)) as u32) as u128; xmm2 = crate::simd_ops::vpcmpeqd(xmm0, xmm1); xmm0 = crate::simd_ops::vblendvps(xmm4, xmm5, xmm2); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm7) as u128); xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; xmm0 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 5; continue; }
            5 => { crate::simd_ops::wu32(rcx, ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((xmm8 as u32)) as u32); xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; rax = r10; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; rsp = ((rsp).wrapping_add(0x68u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = r10; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; rsp = ((rsp).wrapping_add(0x68u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x7cu64))) as u32) as u128; rdx = r8.wrapping_add(0xc); r8 = (((r9 as u8)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm6 = (xmm0) as u128; xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x78u64))) as u32) as u128; rdx = r11; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((xmm6 as u32)) as u32); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; rax = r10; crate::simd_ops::wu32(rcx, ((xmm0 as u32)) as u32); rsp = ((rsp).wrapping_add(0x68u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d3f70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1417a1a00; const P1: u64 = 0x10; const P2: u64 = 0x1423ef2b0; const P3: u64 = 0xc; const P4: u64 = 0x1426469c8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedStack::<1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rax = 0x2030; rsp = ((rsp).wrapping_sub(rax)) as u64; rsi = rdx; rdx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x2040u64), rbx); rcx = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru16(rsp.wrapping_add(rcx.wrapping_mul(2)).wrapping_add(0x24u64))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; rdx = rcx; rdx = (if (((fl_l as u16)) != ((fl_r as u16))) { rbx } else { rdx }) as u64; rcx = ((rcx).wrapping_add(1)) as u64; rbx = rdx; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru32(rsi.wrapping_add(P1))) as u32 as u64; rcx = rsi; fl_l = rbx; fl_r = rdx; rdx = (if (((fl_l as u64)) >= ((fl_r as u64))) { rbx } else { rdx }) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(rsi.wrapping_add(P3))) as u32 as u64; r8 = rsp.wrapping_add(0x24); rcx = (crate::simd_ops::ru64(rsi)) as u64; r9 = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rsi.wrapping_add(P1), ((rbx as u32)) as u32); rax = ((rdi as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x2040u64))) as u64; bb = 5; continue; }
            5 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x2048u64))) as u64; rsp = ((rsp).wrapping_add(0x2030u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d4ae0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1420e41f0; const P1: u64 = 0x40; const P2: u64 = 0x72; const P3: u64 = 0x1417a1e80; const P4: u64 = 0x1417a1d50; const P5: u64 = 0x1420d4b90;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rbp = r8; rcx = ((rcx).wrapping_add(0x540u64)) as u64; rsi = ((rdx as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdi = rax; crate::simd_ops::wu64(rax.wrapping_add(P1), rbp); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 1; continue; }
            1 => { r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; rcx = ((rsi as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x80920004; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 2; continue; }
            2 => { r8 = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; rcx = ((rsi as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { rax = ((rcx as u32)) as u32 as u64; rdx = (0x200000000u64) as u64; rax = (((rax) | (rdx))) as u64; rdx = rdi; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rdi, rax); rax = (tmp_xchg) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x208u64), ((rcx as u32)) as u32); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rax = rdi; bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 3; continue; } bb = 7; continue; }
            7 => { rax = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d4b90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1417a1ae0; const P1: u64 = 0x1417a1c60;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x40u64))) as u64; rbx = rdx; rdx = rsp.wrapping_add(0x20); rdi = (crate::simd_ops::ru32(rax.wrapping_add(0x208u64))) as u32 as u64; rcx = ((rdi as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rax = (0xaaaaaaaaaaaaaaabu64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), (0x0u32) as u32); rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x1f0u64))) as u64; rcx = ((rcx).wrapping_add(1)) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x28u64))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(0x204u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x29u64))) as u32) as u32 as u64; rdx = (((rcx) as i64).wrapping_mul((0x7cu64) as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(0x205u64), ((rax as u8)) as u8); rcx = ((rdi as u32)) as u32 as u64; rax = ((crate::simd_ops::ru16(rsp.wrapping_add(0x24u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rdx.wrapping_add(0x5au64), ((rax as u16)) as u16); rax = ((crate::simd_ops::ru16(rsp.wrapping_add(0x26u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rdx.wrapping_add(0x5cu64), ((rax as u16)) as u16); rax = ((crate::simd_ops::ru16(rsp.wrapping_add(0x24u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rdx.wrapping_add(0x64u64), ((rax as u16)) as u16); rax = ((crate::simd_ops::ru16(rsp.wrapping_add(0x26u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rdx.wrapping_add(0x66u64), ((rax as u16)) as u16); rax = ((crate::simd_ops::ru16(rsp.wrapping_add(0x24u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rdx.wrapping_add(0x6eu64), ((rax as u16)) as u16); rax = ((crate::simd_ops::ru16(rsp.wrapping_add(0x26u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rdx.wrapping_add(0x70u64), ((rax as u16)) as u16); rax = ((crate::simd_ops::ru16(rsp.wrapping_add(0x24u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rdx.wrapping_add(0x78u64), ((rax as u16)) as u16); rax = ((crate::simd_ops::ru16(rsp.wrapping_add(0x26u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rdx.wrapping_add(0x7au64), ((rax as u16)) as u16); rdx = rsp.wrapping_add(0x50); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x50u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 7; continue; } bb = 2; continue; }
            2 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            5 => { rcx = (rcx & !0xffu64) | (((0x2u8) as u8 as u64)); bb = 8; continue; }
            6 => { rcx = (rcx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 8; continue; }
            7 => { rcx = (rcx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x207u64), ((rcx as u8)) as u8); bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d50d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x8; const P2: u64 = 0x1f0; const P3: u64 = 0xc; const P4: u64 = 0x54;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); r10 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rax = (0xaaaaaaaaaaaaaaabu64) as u64; rsi = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rdi = rcx; crate::simd_ops::wu64(rcx, (0xffffffffffffffffu64) as u64); crate::simd_ops::wu64(rcx.wrapping_add(P1), (0xffffffffffffffffu64) as u64); r9 = (crate::simd_ops::ru64(r10.wrapping_add(P2))) as u64; r9 = ((r9).wrapping_add(1)) as u64; let prod = (rax as u128).wrapping_mul((r9) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; r9 = ((r9).wrapping_sub(rax)) as u64; rax = (((r9) as i64).wrapping_mul((0x7cu64) as i64) as u64) as u64; rax = ((rax).wrapping_add(r10)) as u64; r10 = 0; fl_l = 0; fl_r = 0; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rbx = ((r8.wrapping_add(P3)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rbp = ((rax.wrapping_add(P4)) as u64) as u64; bb = 2; continue; }
            2 => { r11 = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; r9 = 0; fl_l = 0; fl_r = 0; rcx = 0; fl_l = 0; fl_r = 0; rax = (0xffffffffffffffffu64) as u64; r8 = rbp; bb = 3; continue; }
            3 => { rdx = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; fl_l = ((r11 as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 4; continue; }
            4 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = rax; fl_r = (0xffffffffffffffffu64) as u64; rax = (if (((fl_l as u64)) == ((fl_r as u64))) { rcx } else { rax }) as u64; bb = 6; continue; }
            6 => { r9 = (((r9 as u32)).wrapping_add(1)) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; r8 = ((r8).wrapping_add(0xau64)) as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 3; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu32(rdi.wrapping_add(rax.wrapping_mul(4)), ((r10 as u32)) as u32); bb = 9; continue; }
            8 => { crate::simd_ops::wu32(rdi.wrapping_add(rcx.wrapping_mul(4)), ((r10 as u32)) as u32); bb = 9; continue; }
            9 => { r10 = (((r10 as u32)).wrapping_add(1)) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 10; continue; }
            10 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; bb = 11; continue; }
            11 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d51b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1434264b4;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = 0u128;
    rax = (((rcx as u8)) as u32) as u32 as u64;
    xmm0 = (xmm0 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128);
    xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P0))).to_bits() as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d51d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143426eec;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P0))).to_bits() as u128);
    rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm0 as u32)))) as u32 as u64;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d51f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xffffffffffffff01; const P1: u64 = 0x1434264b4; const P2: u64 = 0x1; const P3: u64 = 0x1434268d0; const P4: u64 = 0x4; const P5: u64 = 0x143426bf8; const P6: u64 = 0x143426780;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x18u64)) as u64; rax = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; xmm0 = 0u128; crate::simd_ops::wu128(rsp, (xmm6) as u128); rax = ((rax.wrapping_mul(2).wrapping_add(P0)) as u64) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm5 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P1))).to_bits() as u128); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P2))) as u32) as u32 as u64; xmm0 = 0u128; xmm1 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); crate::simd_ops::wu32(rcx, ((xmm5 as u32)) as u32); rax = ((rax.wrapping_mul(2).wrapping_add(P0)) as u64) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm6 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P1))).to_bits() as u128); xmm0 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P3)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P4), ((xmm6 as u32)) as u32); rax = rcx; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm1 = (xmm2) as u128; xmm4 = crate::simd_ops::vrsqrtss(xmm1, (xmm2 as u32)); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm1 = ((crate::simd_ops::ru32(P5)) as u32) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm3 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm1 = ((crate::simd_ops::ru32(P6)) as u32) as u128; xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm0 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm1 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); crate::simd_ops::wu32(rcx, ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P4), ((xmm1 as u32)) as u32); bb = 2; continue; }
            2 => { xmm6 = (crate::simd_ops::ru128(rsp)) as u128; rsp = ((rsp).wrapping_add(0x18u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d52b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xffffffffffffff01; const P1: u64 = 0x1434264b4;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = 0u128;
    rax = (((rcx as u8)) as u32) as u32 as u64;
    rax = ((rax.wrapping_mul(2).wrapping_add(P0)) as u64) as u64;
    xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128);
    xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P1))).to_bits() as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d52e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x10; const P2: u64 = 0x143426eb8; const P3: u64 = 0x143426890;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm2 = ((crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32) as u128;
    xmm1 = ((crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32) as u128;
    xmm0 = 0u128;
    rax = (((r8 as u8)) as u32) as u32 as u64;
    xmm0 = (xmm0 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128);
    xmm5 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(crate::simd_ops::ru32(P2))).to_bits() as u128);
    xmm4 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(xmm5 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm5 as u32) }).to_bits() as u128);
    xmm4 = (xmm4 & !0xffff_ffffu128) | ((if f32::from_bits(xmm4 as u32) < f32::from_bits(crate::simd_ops::ru32(P3)) { f32::from_bits(xmm4 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P3)) }).to_bits() as u128);
    xmm2 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(xmm5 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm5 as u32) }).to_bits() as u128);
    xmm2 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) < f32::from_bits(crate::simd_ops::ru32(P3)) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P3)) }).to_bits() as u128);
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((xmm2 as u32)) as u32);
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    xmm0 = crate::simd_ops::vinsertps(xmm0, xmm4, 0x10u8);
    crate::simd_ops::wu128(rcx, (xmm0) as u128);
    crate::simd_ops::wu128(rdx, (xmm0) as u128);
    xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64) as u128;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), ((xmm0 as u64)) as u64);
    rax = rcx;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d5350(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426eec; const P1: u64 = 0x143426780;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm4 = ((crate::simd_ops::ru32(P0)) as u32) as u128;
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    xmm1 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x8u64)))).to_bits() as u128);
    xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm4 as u32)).to_bits() as u128);
    xmm5 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(crate::simd_ops::ru32(P1))).to_bits() as u128);
    xmm1 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0xcu64)))).to_bits() as u128);
    xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm4 as u32)).to_bits() as u128);
    xmm3 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(crate::simd_ops::ru32(P1))).to_bits() as u128);
    rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm3 as u32)))) as u32 as u64;
    rax = (((rax as u8)) as u32) as u32 as u64;
    rcx = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm5 as u32)))) as u32 as u64;
    rax = (rax & !0xffffu64) | (((((rax as u16)).wrapping_shl(8u32)) as u16 as u64));
    rdx = (((rcx as u8)) as u32) as u32 as u64;
    fl_l = ((((rax as u16)) | ((rdx as u16)))) as u16 as u64; fl_r = fl_l;
    rax = (rax & !0xffffu64) | ((((fl_l as u16)) as u16 as u64));
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d53a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143426eec; const P1: u64 = 0x143426780;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P0))).to_bits() as u128);
    xmm2 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(P0))).to_bits() as u128);
    xmm1 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(crate::simd_ops::ru32(P1))).to_bits() as u128);
    rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm1 as u32)))) as u32 as u64;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d53d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x2; const P1: u64 = 0x6; const P2: u64 = 0x4; const P3: u64 = 0x1434268d0; const P4: u64 = 0x8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut xmm4: u128 = 0; let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x18u64)) as u64;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    xmm0 = 0u128;
    crate::simd_ops::wu128(rsp, (xmm6) as u128);
    xmm6 = 0u128;
    xmm1 = 0u128;
    xmm1 = (xmm1 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128);
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    xmm0 = (xmm0 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128);
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P2))) as u32) as u32 as u64;
    xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128);
    xmm4 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(xmm6 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm6 as u32) }).to_bits() as u128);
    xmm4 = (xmm4 & !0xffff_ffffu128) | ((if f32::from_bits(xmm4 as u32) < f32::from_bits(crate::simd_ops::ru32(P3)) { f32::from_bits(xmm4 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P3)) }).to_bits() as u128);
    xmm0 = 0u128;
    xmm1 = 0u128;
    xmm1 = (xmm1 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128);
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P4))) as u32) as u32 as u64;
    xmm0 = (xmm0 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128);
    xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128);
    xmm3 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(xmm6 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm6 as u32) }).to_bits() as u128);
    xmm3 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) < f32::from_bits(crate::simd_ops::ru32(P3)) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P3)) }).to_bits() as u128);
    xmm6 = (crate::simd_ops::ru128(rsp)) as u128;
    xmm0 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128);
    xmm1 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128);
    xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128);
    xmm0 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32)).sqrt().to_bits() as u128);
    rsp = ((rsp).wrapping_add(0x18u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d5450(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x2; const P1: u64 = 0x6; const P2: u64 = 0x4; const P3: u64 = 0x1434268d0; const P4: u64 = 0x8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut xmm4: u128 = 0; let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x18u64)) as u64;
    rax = ((crate::simd_ops::ru16(rdx.wrapping_add(P0))) as u32) as u32 as u64;
    xmm0 = 0u128;
    crate::simd_ops::wu128(rsp, (xmm6) as u128);
    xmm6 = 0u128;
    xmm1 = 0u128;
    xmm1 = (xmm1 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128);
    rax = ((crate::simd_ops::ru16(rdx.wrapping_add(P1))) as u32) as u32 as u64;
    xmm0 = (xmm0 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128);
    rax = ((crate::simd_ops::ru16(rdx.wrapping_add(P2))) as u32) as u32 as u64;
    xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128);
    xmm4 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(xmm6 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm6 as u32) }).to_bits() as u128);
    xmm4 = (xmm4 & !0xffff_ffffu128) | ((if f32::from_bits(xmm4 as u32) < f32::from_bits(crate::simd_ops::ru32(P3)) { f32::from_bits(xmm4 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P3)) }).to_bits() as u128);
    xmm1 = 0u128;
    xmm1 = (xmm1 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128);
    rax = ((crate::simd_ops::ru16(rdx.wrapping_add(P4))) as u32) as u32 as u64;
    xmm0 = 0u128;
    xmm0 = (xmm0 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128);
    xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128);
    xmm3 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(xmm6 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm6 as u32) }).to_bits() as u128);
    xmm3 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) < f32::from_bits(crate::simd_ops::ru32(P3)) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P3)) }).to_bits() as u128);
    xmm6 = (crate::simd_ops::ru128(rsp)) as u128;
    rax = rcx;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((xmm3 as u32)) as u32);
    crate::simd_ops::wu32(rcx, ((xmm4 as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x18u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d5590(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x72; const P1: u64 = 0x1417a23c0; const P2: u64 = 0x4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    rbx = rcx;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = rbx; crate::simd_ops::wu8(rbx.wrapping_add(P2), (0x1u8) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    crate::simd_ops::wu8(rcx.wrapping_add(P2), (0x1u8) as u8);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d55d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645790; const P1: u64 = 0x1426455a8; const P2: u64 = 0x143426890; const P3: u64 = 0x143426eb8; const P4: u64 = 0x140000000; const P5: u64 = 0x1434264b4; const P6: u64 = 0x1434268d0; const P7: u64 = 0x143426bf8; const P8: u64 = 0x143426780; const P9: u64 = 0x1420d3d40; const P10: u64 = 0x1420d38f0; const P11: u64 = 0x142645598;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 29>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0xc0u64)) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x38u64), (xmm6) as u128); rdi = (((r9 as u8)) as u32) as u32 as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x48u64), (xmm7) as u128); r14 = rdx; rsi = (((r8 as u16)) as u32) as u32 as u64; r15 = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru32(r14.wrapping_add(0x10u64))) as u32 as u64; rbp = ((rax as u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = r14.wrapping_add(0x8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(r14.wrapping_add(0x10u64), ((rbp as u32)) as u32); bb = 2; continue; }
            2 => { crate::simd_ops::lk_add_u32(r14.wrapping_add(0x14u64), (1) as u32); r8 = (crate::simd_ops::ru64(r14.wrapping_add(0x40u64))) as u64; fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((((fl_l as u8)) & ((fl_r as u8))) as i8) >= 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru8(r15.wrapping_add(0x4fd8u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = 0x174; rdx = ((rax.wrapping_add(r8)) as u64) as u64; bb = 7; continue; }
            5 => { rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x1f0u64))) as u64; rax = (0xaaaaaaaaaaaaaaabu64) as u64; rcx = ((rcx).wrapping_add(0x2u64)) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; rax = (((rcx) as i64).wrapping_mul((0x7cu64) as i64) as u64) as u64; rdx = ((rax.wrapping_add(r8)) as u64) as u64; bb = 7; continue; }
            6 => { rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x1f0u64))) as u64; rax = (0xaaaaaaaaaaaaaaabu64) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; rdx = (((rcx) as i64).wrapping_mul((0x7cu64) as i64) as u64) as u64; fl_l = ((rdx).wrapping_add(r8)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 7; continue; }
            7 => { xmm7 = ((crate::simd_ops::ru32(P2)) as u32) as u128; rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x40u64))) as u64; xmm2 = (crate::simd_ops::ru128(r15.wrapping_add(0x20u64))) as u128; xmm4 = ((crate::simd_ops::ru64(r15.wrapping_add(0x30u64))) as u64) as u128; xmm5 = ((crate::simd_ops::ru64(r15.wrapping_add(0x48u64))) as u64) as u128; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x204u64))) as u32) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128); xmm3 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(crate::simd_ops::ru32(P3))).to_bits() as u128); rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x205u64))) as u32) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm2) as u128); xmm0 = 0u128; xmm1 = (xmm0 & !0xffff_ffffu128) | (xmm4 & 0xffff_ffffu128); xmm2 = crate::simd_ops::vshufps(xmm2, xmm2, 0x55u8); xmm2 = (xmm0 & !0xffff_ffffu128) | (xmm2 & 0xffff_ffffu128); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(xmm3 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm3 as u32) }).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(xmm3 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm3 as u32) }).to_bits() as u128); xmm3 = (crate::simd_ops::ru128(r15.wrapping_add(0x38u64))) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) < f32::from_bits(xmm7 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm7 as u32) }).to_bits() as u128); xmm0 = crate::simd_ops::vinsertps(xmm0, xmm2, 0x10u8); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm0) as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) < f32::from_bits(xmm7 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm7 as u32) }).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | (xmm1 & 0xffff_ffffu128); xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm3) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm4 as u64)) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), ((xmm4 as u64)) as u64); xmm4 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(crate::simd_ops::ru32(P3))).to_bits() as u128); xmm0 = 0u128; xmm1 = (xmm0 & !0xffff_ffffu128) | (xmm5 & 0xffff_ffffu128); xmm3 = crate::simd_ops::vshufps(xmm3, xmm3, 0x55u8); xmm2 = (xmm0 & !0xffff_ffffu128) | (xmm3 & 0xffff_ffffu128); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(xmm4 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm4 as u32) }).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(xmm4 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm4 as u32) }).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) < f32::from_bits(xmm7 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm7 as u32) }).to_bits() as u128); xmm0 = crate::simd_ops::vinsertps(xmm0, xmm2, 0x10u8); xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) < f32::from_bits(xmm7 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm7 as u32) }).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | (xmm1 & 0xffff_ffffu128); rax = ((rsi.wrapping_sub(0x95u64)) as u32) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x78u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), ((xmm5 as u64)) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), ((xmm5 as u64)) as u64); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x24; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 27; continue; } bb = 8; continue; }
            8 => { r8 = P4; rax = (rax as u32 as i32 as i64) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(rax).wrapping_add(0x20d5a70u64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x20d5a3cu64))) as u32 as u64; fl_l = ((rcx).wrapping_add(r8)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; j_tgt = rcx; bb = 0xffff_fffe; continue; }
            9 => { rdi = ((crate::simd_ops::ru8(rsi.wrapping_add(rdx).wrapping_sub(0x95u64))) as u32) as u32 as u64; fl_l = ((((rdi as u8)) & (0x1u8))) as u8 as u64; fl_r = fl_l; rdi = (rdi & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); bb = 28; continue; }
            10 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x27u64))) as u32) as u32 as u64; xmm0 = 0u128; rax = ((rax.wrapping_mul(2).wrapping_sub(0xffu64)) as u64) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm5 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P5))).to_bits() as u128); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x28u64))) as u32) as u32 as u64; xmm0 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0xf0u64), ((xmm5 as u32)) as u32); rax = ((rax.wrapping_mul(2).wrapping_sub(0xffu64)) as u64) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm6 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P5))).to_bits() as u128); xmm1 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm0 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P6)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xf4u64), ((xmm6 as u32)) as u32); if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { xmm1 = (xmm2) as u128; xmm4 = crate::simd_ops::vrsqrtss(xmm1, (xmm2 as u32)); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm1 = ((crate::simd_ops::ru32(P7)) as u32) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm3 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm1 = ((crate::simd_ops::ru32(P8)) as u32) as u128; xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm0 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm1 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0xf0u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0xf4u64), ((xmm1 as u32)) as u32); bb = 12; continue; }
            12 => { r8 = rsp.wrapping_add(0x60); rcx = rsp.wrapping_add(0xf8); bb = 18; continue; }
            13 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x27u64))) as u32) as u32 as u64; rdx = rsp.wrapping_add(0x60); bb = 21; continue; }
            14 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x28u64))) as u32) as u32 as u64; rdx = rsp.wrapping_add(0x6c); bb = 21; continue; }
            15 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x29u64))) as u32) as u32 as u64; xmm0 = 0u128; rax = ((rax.wrapping_mul(2).wrapping_sub(0xffu64)) as u64) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm5 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P5))).to_bits() as u128); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x2au64))) as u32) as u32 as u64; xmm0 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0xf0u64), ((xmm5 as u32)) as u32); rax = ((rax.wrapping_mul(2).wrapping_sub(0xffu64)) as u64) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm6 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P5))).to_bits() as u128); xmm1 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm0 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P6)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xf4u64), ((xmm6 as u32)) as u32); if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { xmm1 = (xmm2) as u128; xmm4 = crate::simd_ops::vrsqrtss(xmm1, (xmm2 as u32)); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm1 = ((crate::simd_ops::ru32(P7)) as u32) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm3 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm1 = ((crate::simd_ops::ru32(P8)) as u32) as u128; xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm0 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm1 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0xf0u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0xf4u64), ((xmm1 as u32)) as u32); bb = 17; continue; }
            17 => { r8 = rsp.wrapping_add(0x78); rcx = rsp.wrapping_add(0x90); bb = 18; continue; }
            18 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; r9 = (((rdi as u8)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(rax)) as u32) as u128; xmm2 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm3 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm0 = 0u128; fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; rdi = (rdi & !0xffu64) | (((if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 28; continue; }
            19 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x29u64))) as u32) as u32 as u64; rdx = rsp.wrapping_add(0x78); bb = 21; continue; }
            20 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x2au64))) as u32) as u32 as u64; rdx = rsp.wrapping_add(0x84); bb = 21; continue; }
            21 => { xmm0 = 0u128; rax = ((rax.wrapping_mul(2).wrapping_sub(0xffu64)) as u64) as u64; r8 = (((rdi as u8)) as u32) as u32 as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P5))).to_bits() as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm1 = 0u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; rdi = (rdi & !0xffu64) | (((if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 28; continue; }
            22 => { rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 28; continue; }
            23 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x54u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rdi = (rdi & !0xffu64) | (((if (((fl_l as u8)) != ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 28; continue; }
            24 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x5eu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rdi = (rdi & !0xffu64) | (((if (((fl_l as u8)) != ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 28; continue; }
            25 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x68u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rdi = (rdi & !0xffu64) | (((if (((fl_l as u8)) != ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 28; continue; }
            26 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x72u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rdi = (rdi & !0xffu64) | (((if (((fl_l as u8)) != ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 28; continue; }
            27 => { rdi = (rdi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 28; continue; }
            28 => { fl_l = (crate::simd_ops::lk_add_u32(r14.wrapping_add(0x14u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 30; continue; } bb = 29; continue; }
            29 => { rcx = 0xffffffff; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(r14.wrapping_add(0x10u64), ((rcx as u32)) as u32); rcx = (tmp_xchg) as u32 as u64; rcx = r14.wrapping_add(0x8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P11), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 30; continue; }
            30 => { r11 = rsp.wrapping_add(0xc0); rax = (((rdi as u8)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; xmm6 = (crate::simd_ops::ru128(r11.wrapping_sub(0x10u64))) as u128; xmm7 = (crate::simd_ops::ru128(r11.wrapping_sub(0x20u64))) as u128; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x1420d55d0u64) { 0x3e => { bb = 1; continue; } 0x4c => { bb = 2; continue; } 0x59 => { bb = 3; continue; } 0x66 => { bb = 4; continue; } 0x71 => { bb = 5; continue; } 0x9d => { bb = 6; continue; } 0xc2 => { bb = 7; continue; } 0x1c4 => { bb = 8; continue; } 0x1e3 => { bb = 9; continue; } 0x1f4 => { bb = 10; continue; } 0x256 => { bb = 11; continue; } 0x29c => { bb = 12; continue; } 0x2ae => { bb = 13; continue; } 0x2bc => { bb = 14; continue; } 0x2ca => { bb = 15; continue; } 0x32c => { bb = 16; continue; } 0x372 => { bb = 17; continue; } 0x37f => { bb = 18; continue; } 0x3b3 => { bb = 19; continue; } 0x3be => { bb = 20; continue; } 0x3ca => { bb = 21; continue; } 0x3fa => { bb = 22; continue; } 0x3ff => { bb = 23; continue; } 0x409 => { bb = 24; continue; } 0x413 => { bb = 25; continue; } 0x41d => { bb = 26; continue; } 0x427 => { bb = 27; continue; } 0x42a => { bb = 28; continue; } 0x431 => { bb = 29; continue; } 0x444 => { bb = 30; continue; } _ => { return crate::simd_ops::tail_stk_xmm(j_tgt, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); } } }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d5aa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645790; const P1: u64 = 0x1426455a8; const P2: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (((r9 as u16)) as u32) as u32 as u64; r14 = r8; r15 = rdx; rsi = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru32(r14.wrapping_add(0x10u64))) as u32 as u64; rbp = ((rax as u32)) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = r14.wrapping_add(0x8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(r14.wrapping_add(0x10u64), ((rbp as u32)) as u32); bb = 2; continue; }
            2 => { crate::simd_ops::lk_add_u32(r14.wrapping_add(0x14u64), (1) as u32); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x60u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r8 = (crate::simd_ops::ru64(r14.wrapping_add(0x40u64))) as u64; if ((((fl_l as u8)) as i8) >= (((fl_r as u8)) as i8)) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru8(r15.wrapping_add(0x4fd8u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = 0x174; rdx = ((rax.wrapping_add(r8)) as u64) as u64; bb = 7; continue; }
            5 => { rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x1f0u64))) as u64; rax = (0xaaaaaaaaaaaaaaabu64) as u64; rcx = ((rcx).wrapping_add(0x2u64)) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; rax = (((rcx) as i64).wrapping_mul((0x7cu64) as i64) as u64) as u64; rdx = ((rax.wrapping_add(r8)) as u64) as u64; bb = 7; continue; }
            6 => { rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x1f0u64))) as u64; rax = (0xaaaaaaaaaaaaaaabu64) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; rdx = (((rcx) as i64).wrapping_mul((0x7cu64) as i64) as u64) as u64; fl_l = ((rdx).wrapping_add(r8)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 7; continue; }
            7 => { rcx = ((rdi as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0xb3; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 8; continue; }
            8 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsi, rax); bb = 13; continue; }
            10 => { xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(0x48u64))) as u64) as u128; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x50u64))) as u32 as u64; bb = 12; continue; }
            11 => { xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(0x3cu64))) as u64) as u128; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x44u64))) as u32 as u64; bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rsi, ((xmm0 as u64)) as u64); bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru32(r14.wrapping_add(0x14u64))) as u32 as u64; fl_r = 0x1; crate::simd_ops::wu32(r14.wrapping_add(0x14u64), ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rax as u32)) as u32); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = 0xffffffff; rcx = r14.wrapping_add(0x8); let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(r14.wrapping_add(0x10u64), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = rsi; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d5bd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645790; const P1: u64 = 0x1426455a8; const P2: u64 = 0x143426890; const P3: u64 = 0x143426eb8; const P4: u64 = 0x140000000; const P5: u64 = 0x1434268d0; const P6: u64 = 0x1434264b4; const P7: u64 = 0x143426bf8; const P8: u64 = 0x143426780; const P9: u64 = 0x1420d3d40; const P10: u64 = 0x143428760; const P11: u64 = 0x142645598;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rbp, mut rsi, mut rdi, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 35>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), (rbx) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rax.wrapping_sub(0x5fu64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0xf0u64)) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x38u64), (xmm6) as u128); rdi = (((r9 as u8)) as u32) as u32 as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x48u64), (xmm7) as u128); r15 = (rdx) as u64; rsi = (((r8 as u16)) as u32) as u32 as u64; r13 = (rcx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = (crate::simd_ops::ru32(r15.wrapping_add(0x10u64))) as u32 as u64; r14 = ((rax as u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((r15.wrapping_add(0x8u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(r15.wrapping_add(0x10u64), ((r14 as u32)) as u32); bb = 2; continue; }
            2 => { crate::simd_ops::lk_add_u32(r15.wrapping_add(0x14u64), (1) as u32); r8 = (crate::simd_ops::ru64(r15.wrapping_add(0x40u64))) as u64; fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((((fl_l as u8)) & ((fl_r as u8))) as i8) >= 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru8(r13.wrapping_add(0x4fd8u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (0x174u32) as u32 as u64; rdx = ((rax.wrapping_add(r8)) as u64) as u64; bb = 7; continue; }
            5 => { rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x1f0u64))) as u64; rax = (0xaaaaaaaaaaaaaaabu64) as u64; rcx = ((rcx).wrapping_add(0x2u64)) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; rax = (((rcx) as i64).wrapping_mul((0x7cu64) as i64) as u64) as u64; rdx = ((rax.wrapping_add(r8)) as u64) as u64; bb = 7; continue; }
            6 => { rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x1f0u64))) as u64; rax = (0xaaaaaaaaaaaaaaabu64) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; rdx = (((rcx) as i64).wrapping_mul((0x7cu64) as i64) as u64) as u64; fl_l = ((rdx).wrapping_add(r8)) as u64 as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 7; continue; }
            7 => { xmm7 = ((crate::simd_ops::ru32(P2)) as u32) as u128; rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x40u64))) as u64; xmm2 = (crate::simd_ops::ru128(r13.wrapping_add(0x20u64))) as u128; xmm4 = ((crate::simd_ops::ru64(r13.wrapping_add(0x30u64))) as u64) as u128; xmm5 = ((crate::simd_ops::ru64(r13.wrapping_add(0x48u64))) as u64) as u128; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x204u64))) as u32) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128); xmm3 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(core::ptr::read_unaligned((P3) as *const u32))).to_bits() as u128); rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x205u64))) as u32) as u32 as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x79u64), (xmm2) as u128); xmm0 = 0u128; xmm1 = (xmm0 & !0xffff_ffffu128) | (xmm4 & 0xffff_ffffu128); xmm2 = crate::simd_ops::vshufps(xmm2, xmm2, 0x55u8); xmm2 = (xmm0 & !0xffff_ffffu128) | (xmm2 & 0xffff_ffffu128); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x79u64))) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(xmm3 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm3 as u32) }).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(xmm3 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm3 as u32) }).to_bits() as u128); xmm3 = (crate::simd_ops::ru128(r13.wrapping_add(0x38u64))) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) < f32::from_bits(xmm7 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm7 as u32) }).to_bits() as u128); xmm0 = crate::simd_ops::vinsertps(xmm0, xmm2, 0x10u8); crate::simd_ops::wu128(rbp.wrapping_sub(0x79u64), (xmm0) as u128); crate::simd_ops::wu128(rbp.wrapping_sub(0x39u64), (xmm0) as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) < f32::from_bits(xmm7 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm7 as u32) }).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | (xmm1 & 0xffff_ffffu128); xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128); crate::simd_ops::wu128(rbp.wrapping_sub(0x59u64), (xmm3) as u128); crate::simd_ops::wu64(rbp.wrapping_sub(0x69u64), ((xmm4 as u64)) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), ((xmm4 as u64)) as u64); xmm4 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(core::ptr::read_unaligned((P3) as *const u32))).to_bits() as u128); xmm0 = 0u128; xmm1 = (xmm0 & !0xffff_ffffu128) | (xmm5 & 0xffff_ffffu128); xmm3 = crate::simd_ops::vshufps(xmm3, xmm3, 0x55u8); xmm2 = (xmm0 & !0xffff_ffffu128) | (xmm3 & 0xffff_ffffu128); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x59u64))) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(xmm4 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm4 as u32) }).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(xmm4 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm4 as u32) }).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) < f32::from_bits(xmm7 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm7 as u32) }).to_bits() as u128); xmm0 = crate::simd_ops::vinsertps(xmm0, xmm2, 0x10u8); xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) < f32::from_bits(xmm7 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm7 as u32) }).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | (xmm1 & 0xffff_ffffu128); rax = ((rsi.wrapping_sub(0x95u64)) as u32) as u32 as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x59u64), (xmm0) as u128); crate::simd_ops::wu128(rbp.wrapping_sub(0x21u64), (xmm0) as u128); crate::simd_ops::wu64(rbp.wrapping_sub(0x49u64), ((xmm5 as u64)) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), ((xmm5 as u64)) as u64); fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x24u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 44; continue; } bb = 8; continue; }
            8 => { r8 = ((P4) as u64) as u64; rax = (rax as u32 as i32 as i64) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(rax).wrapping_add(0x20d6554u64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x20d6510u64))) as u32 as u64; fl_l = ((rcx).wrapping_add(r8)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; j_tgt = (rcx) as u64; bb = 0xffff_fffe; continue; }
            9 => { rcx = ((crate::simd_ops::ru8(rsi.wrapping_add(rdx).wrapping_sub(0x95u64))) as u32) as u32 as u64; rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0x1u8))) as u8 as u64)); rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; xmm1 = (((rax as u32)) as u32) as u128; rax = (((rcx as u8)) as u32) as u32 as u64; xmm0 = (((rax as u32)) as u32) as u128; xmm3 = crate::simd_ops::vpcmpeqd(xmm0, xmm1); xmm0 = ((crate::simd_ops::ru32(P5)) as u32) as u128; xmm2 = 0u128; xmm6 = crate::simd_ops::vblendvps(xmm0, xmm2, xmm3); bb = 45; continue; }
            10 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x25u64))) as u32) as u32 as u64; bb = 12; continue; }
            11 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x26u64))) as u32) as u32 as u64; bb = 12; continue; }
            12 => { xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128); xmm6 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(core::ptr::read_unaligned((P6) as *const u32))).to_bits() as u128); bb = 45; continue; }
            13 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x27u64))) as u32) as u32 as u64; xmm0 = 0u128; rax = ((rax.wrapping_mul(2).wrapping_sub(0xffu64)) as u64) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm5 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(core::ptr::read_unaligned((P6) as *const u32))).to_bits() as u128); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x28u64))) as u32) as u32 as u64; xmm0 = 0u128; crate::simd_ops::wu32(rbp.wrapping_add(0x67u64), ((xmm5 as u32)) as u32); rax = ((rax.wrapping_mul(2).wrapping_sub(0xffu64)) as u64) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm6 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(core::ptr::read_unaligned((P6) as *const u32))).to_bits() as u128); xmm1 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm0 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P5)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x6bu64), ((xmm6 as u32)) as u32); if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 15; continue; } bb = 14; continue; }
            14 => { xmm1 = (xmm2) as u128; xmm4 = crate::simd_ops::vrsqrtss(xmm1, (xmm2 as u32)); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm1 = ((crate::simd_ops::ru32(P7)) as u32) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm3 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm1 = ((crate::simd_ops::ru32(P8)) as u32) as u128; xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm0 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm1 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x67u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x6bu64), ((xmm1 as u32)) as u32); bb = 15; continue; }
            15 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; r8 = ((rbp.wrapping_sub(0x39u64)) as u64) as u64; r9 = (((rdi as u8)) as u32) as u32 as u64; rcx = ((rbp.wrapping_sub(0x9u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm6 = ((crate::simd_ops::ru32(rax)) as u32) as u128; bb = 45; continue; }
            16 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x27u64))) as u32) as u32 as u64; xmm0 = 0u128; rax = ((rax.wrapping_mul(2).wrapping_sub(0xffu64)) as u64) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm5 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(core::ptr::read_unaligned((P6) as *const u32))).to_bits() as u128); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x28u64))) as u32) as u32 as u64; xmm0 = 0u128; crate::simd_ops::wu32(rbp.wrapping_add(0x67u64), ((xmm5 as u32)) as u32); rax = ((rax.wrapping_mul(2).wrapping_sub(0xffu64)) as u64) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm6 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(core::ptr::read_unaligned((P6) as *const u32))).to_bits() as u128); xmm1 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm0 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P5)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x6bu64), ((xmm6 as u32)) as u32); if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 18; continue; } bb = 17; continue; }
            17 => { xmm1 = (xmm2) as u128; xmm4 = crate::simd_ops::vrsqrtss(xmm1, (xmm2 as u32)); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm1 = ((crate::simd_ops::ru32(P7)) as u32) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm3 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm1 = ((crate::simd_ops::ru32(P8)) as u32) as u128; xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm0 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm1 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x67u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x6bu64), ((xmm1 as u32)) as u32); bb = 18; continue; }
            18 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; r8 = ((rbp.wrapping_sub(0x39u64)) as u64) as u64; r9 = (((rdi as u8)) as u32) as u32 as u64; rcx = ((rbp.wrapping_sub(0x1u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm6 = ((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as u32) as u128; bb = 45; continue; }
            19 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x29u64))) as u32) as u32 as u64; xmm0 = 0u128; rax = ((rax.wrapping_mul(2).wrapping_sub(0xffu64)) as u64) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm5 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(core::ptr::read_unaligned((P6) as *const u32))).to_bits() as u128); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x2au64))) as u32) as u32 as u64; xmm0 = 0u128; crate::simd_ops::wu32(rbp.wrapping_add(0x67u64), ((xmm5 as u32)) as u32); rax = ((rax.wrapping_mul(2).wrapping_sub(0xffu64)) as u64) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm6 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(core::ptr::read_unaligned((P6) as *const u32))).to_bits() as u128); xmm1 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm0 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P5)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x6bu64), ((xmm6 as u32)) as u32); if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 21; continue; } bb = 20; continue; }
            20 => { xmm1 = (xmm2) as u128; xmm4 = crate::simd_ops::vrsqrtss(xmm1, (xmm2 as u32)); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm1 = ((crate::simd_ops::ru32(P7)) as u32) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm3 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm1 = ((crate::simd_ops::ru32(P8)) as u32) as u128; xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm0 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm1 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x67u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x6bu64), ((xmm1 as u32)) as u32); bb = 21; continue; }
            21 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; r8 = ((rbp.wrapping_sub(0x21u64)) as u64) as u64; r9 = (((rdi as u8)) as u32) as u32 as u64; rcx = ((rbp.wrapping_add(0x7u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm6 = ((crate::simd_ops::ru32(rax)) as u32) as u128; bb = 45; continue; }
            22 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x29u64))) as u32) as u32 as u64; xmm0 = 0u128; rax = ((rax.wrapping_mul(2).wrapping_sub(0xffu64)) as u64) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm5 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(core::ptr::read_unaligned((P6) as *const u32))).to_bits() as u128); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x2au64))) as u32) as u32 as u64; xmm0 = 0u128; crate::simd_ops::wu32(rbp.wrapping_add(0x67u64), ((xmm5 as u32)) as u32); rax = ((rax.wrapping_mul(2).wrapping_sub(0xffu64)) as u64) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm6 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(core::ptr::read_unaligned((P6) as *const u32))).to_bits() as u128); xmm1 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm0 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P5)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x6bu64), ((xmm6 as u32)) as u32); if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 24; continue; } bb = 23; continue; }
            23 => { xmm1 = (xmm2) as u128; xmm4 = crate::simd_ops::vrsqrtss(xmm1, (xmm2 as u32)); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm1 = ((crate::simd_ops::ru32(P7)) as u32) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm3 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm1 = ((crate::simd_ops::ru32(P8)) as u32) as u128; xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm0 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm1 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x67u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x6bu64), ((xmm1 as u32)) as u32); bb = 24; continue; }
            24 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; r8 = ((rbp.wrapping_sub(0x21u64)) as u64) as u64; r9 = (((rdi as u8)) as u32) as u32 as u64; rcx = ((rbp.wrapping_add(0xfu64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm6 = ((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as u32) as u128; bb = 45; continue; }
            25 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x27u64))) as u32) as u32 as u64; xmm0 = 0u128; rax = ((rax.wrapping_mul(2).wrapping_sub(0xffu64)) as u64) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm5 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(core::ptr::read_unaligned((P6) as *const u32))).to_bits() as u128); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x28u64))) as u32) as u32 as u64; xmm0 = 0u128; crate::simd_ops::wu32(rbp.wrapping_add(0x67u64), ((xmm5 as u32)) as u32); rax = ((rax.wrapping_mul(2).wrapping_sub(0xffu64)) as u64) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm6 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(core::ptr::read_unaligned((P6) as *const u32))).to_bits() as u128); xmm1 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm0 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P5)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x6bu64), ((xmm6 as u32)) as u32); if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 27; continue; } bb = 26; continue; }
            26 => { xmm1 = (xmm2) as u128; xmm4 = crate::simd_ops::vrsqrtss(xmm1, (xmm2 as u32)); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm1 = ((crate::simd_ops::ru32(P7)) as u32) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm3 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm1 = ((crate::simd_ops::ru32(P8)) as u32) as u128; xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm0 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm1 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x67u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x6bu64), ((xmm1 as u32)) as u32); bb = 27; continue; }
            27 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; r8 = ((rbp.wrapping_sub(0x39u64)) as u64) as u64; r9 = (((rdi as u8)) as u32) as u32 as u64; rcx = ((rbp.wrapping_add(0x6fu64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm2 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x6fu64))) as u32) as u128; xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x73u64))) as u32) as u128; xmm1 = 0u128; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 35; continue; } bb = 28; continue; }
            28 => { fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 35; continue; } bb = 29; continue; }
            29 => { xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm6 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32)).sqrt().to_bits() as u128); bb = 45; continue; }
            30 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x29u64))) as u32) as u32 as u64; xmm0 = 0u128; rax = ((rax.wrapping_mul(2).wrapping_sub(0xffu64)) as u64) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm5 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(core::ptr::read_unaligned((P6) as *const u32))).to_bits() as u128); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x2au64))) as u32) as u32 as u64; xmm0 = 0u128; crate::simd_ops::wu32(rbp.wrapping_add(0x67u64), ((xmm5 as u32)) as u32); rax = ((rax.wrapping_mul(2).wrapping_sub(0xffu64)) as u64) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm6 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(core::ptr::read_unaligned((P6) as *const u32))).to_bits() as u128); xmm1 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm0 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P5)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x6bu64), ((xmm6 as u32)) as u32); if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 32; continue; } bb = 31; continue; }
            31 => { xmm1 = (xmm2) as u128; xmm4 = crate::simd_ops::vrsqrtss(xmm1, (xmm2 as u32)); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm1 = ((crate::simd_ops::ru32(P7)) as u32) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm3 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm1 = ((crate::simd_ops::ru32(P8)) as u32) as u128; xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm0 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm1 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x67u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x6bu64), ((xmm1 as u32)) as u32); bb = 32; continue; }
            32 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; r8 = ((rbp.wrapping_sub(0x21u64)) as u64) as u64; r9 = (((rdi as u8)) as u32) as u32 as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm2 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32) as u128; xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32) as u128; xmm1 = 0u128; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 35; continue; } bb = 33; continue; }
            33 => { fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 35; continue; } bb = 34; continue; }
            34 => { xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm6 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32)).sqrt().to_bits() as u128); xmm6 = (xmm6) as u128; bb = 45; continue; }
            35 => { xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm3 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32)).sqrt().to_bits() as u128); xmm6 = (xmm3) ^ (crate::simd_ops::ru128(P10)); bb = 45; continue; }
            36 => { xmm0 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x30u64))) as u32) as u128; xmm2 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x34u64))) as u32) as u128; xmm3 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x38u64))) as u32) as u128; xmm4 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x2cu64))) as u32) as u128; xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm1 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm3 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm0 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm6 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 45; continue; }
            37 => { xmm0 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x40u64))) as u32) as u128; xmm2 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x3cu64))) as u32) as u128; xmm3 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x44u64))) as u32) as u128; xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm1 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm3 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm6 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32)).sqrt().to_bits() as u128); xmm6 = (xmm6) as u128; bb = 45; continue; }
            38 => { xmm0 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x4cu64))) as u32) as u128; xmm2 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x48u64))) as u32) as u128; xmm3 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x50u64))) as u32) as u128; xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm1 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm3 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm6 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32)).sqrt().to_bits() as u128); xmm6 = (xmm6) as u128; bb = 45; continue; }
            39 => { xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(0x54u64))) as u64) as u128; xmm6 = 0u128; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; xmm1 = 0u128; rax = (rcx) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = (((rax as u16)) as u32) as u32 as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128); rax = (rcx) as u64; rax = ((rax).wrapping_shr(48u32)) as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((rcx).wrapping_shr(32u32)) as u64 as u64; rcx = ((fl_l as u64)) as u64; rax = (((rcx as u16)) as u32) as u32 as u64; xmm1 = 0u128; xmm1 = (xmm1 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128); rax = ((crate::simd_ops::ru16(rdx.wrapping_add(0x5cu64))) as u32) as u32 as u64; bb = 43; continue; }
            40 => { xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(0x5eu64))) as u64) as u128; xmm6 = 0u128; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; xmm1 = 0u128; rax = (rcx) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = (((rax as u16)) as u32) as u32 as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128); rax = (rcx) as u64; rax = ((rax).wrapping_shr(48u32)) as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((rcx).wrapping_shr(32u32)) as u64 as u64; rcx = ((fl_l as u64)) as u64; rax = (((rcx as u16)) as u32) as u32 as u64; xmm1 = 0u128; xmm1 = (xmm1 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128); rax = ((crate::simd_ops::ru16(rdx.wrapping_add(0x66u64))) as u32) as u32 as u64; bb = 43; continue; }
            41 => { xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(0x68u64))) as u64) as u128; xmm6 = 0u128; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; xmm1 = 0u128; rax = (rcx) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = (((rax as u16)) as u32) as u32 as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128); rax = (rcx) as u64; rax = ((rax).wrapping_shr(48u32)) as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((rcx).wrapping_shr(32u32)) as u64 as u64; rcx = ((fl_l as u64)) as u64; rax = (((rcx as u16)) as u32) as u32 as u64; xmm1 = 0u128; xmm1 = (xmm1 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128); rax = ((crate::simd_ops::ru16(rdx.wrapping_add(0x70u64))) as u32) as u32 as u64; bb = 43; continue; }
            42 => { xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(0x72u64))) as u64) as u128; xmm6 = 0u128; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; xmm1 = 0u128; rax = (rcx) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = (((rax as u16)) as u32) as u32 as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128); rax = (rcx) as u64; rax = ((rax).wrapping_shr(48u32)) as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((rcx).wrapping_shr(32u32)) as u64 as u64; rcx = ((fl_l as u64)) as u64; rax = (((rcx as u16)) as u32) as u32 as u64; xmm1 = 0u128; xmm1 = (xmm1 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128); rax = ((crate::simd_ops::ru16(rdx.wrapping_add(0x7au64))) as u32) as u32 as u64; bb = 43; continue; }
            43 => { xmm4 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(xmm6 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm6 as u32) }).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((if f32::from_bits(xmm4 as u32) < f32::from_bits(core::ptr::read_unaligned((P5) as *const u32)) { f32::from_bits(xmm4 as u32) } else { f32::from_bits(core::ptr::read_unaligned((P5) as *const u32)) }).to_bits() as u128); xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm3 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(xmm6 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm6 as u32) }).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) < f32::from_bits(core::ptr::read_unaligned((P5) as *const u32)) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(core::ptr::read_unaligned((P5) as *const u32)) }).to_bits() as u128); xmm1 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm0 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm6 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32)).sqrt().to_bits() as u128); bb = 45; continue; }
            44 => { xmm6 = 0u128; bb = 45; continue; }
            45 => { fl_l = (crate::simd_ops::lk_add_u32(r15.wrapping_add(0x14u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 47; continue; } bb = 46; continue; }
            46 => { rax = (0xffffffffu32) as u32 as u64; rcx = ((r15.wrapping_add(0x8u64)) as u64) as u64; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(r15.wrapping_add(0x10u64), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P11)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 47; continue; }
            47 => { r11 = ((rsp.wrapping_add(0xf0u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x48u64))) as u64; xmm7 = (crate::simd_ops::ru128(r11.wrapping_sub(0x20u64))) as u128; xmm0 = (xmm6) as u128; xmm6 = (crate::simd_ops::ru128(r11.wrapping_sub(0x10u64))) as u128; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x1420d5bd0u64) { 0x48 => { bb = 1; continue; } 0x56 => { bb = 2; continue; } 0x63 => { bb = 3; continue; } 0x70 => { bb = 4; continue; } 0x7b => { bb = 5; continue; } 0xa7 => { bb = 6; continue; } 0xcc => { bb = 7; continue; } 0x1bf => { bb = 8; continue; } 0x1de => { bb = 9; continue; } 0x211 => { bb = 10; continue; } 0x217 => { bb = 11; continue; } 0x21b => { bb = 12; continue; } 0x230 => { bb = 13; continue; } 0x28a => { bb = 14; continue; } 0x2c8 => { bb = 15; continue; } 0x2e6 => { bb = 16; continue; } 0x340 => { bb = 17; continue; } 0x37e => { bb = 18; continue; } 0x39d => { bb = 19; continue; } 0x3f7 => { bb = 20; continue; } 0x435 => { bb = 21; continue; } 0x453 => { bb = 22; continue; } 0x4ad => { bb = 23; continue; } 0x4eb => { bb = 24; continue; } 0x50a => { bb = 25; continue; } 0x564 => { bb = 26; continue; } 0x5a2 => { bb = 27; continue; } 0x5cf => { bb = 28; continue; } 0x5d9 => { bb = 29; continue; } 0x5ee => { bb = 30; continue; } 0x648 => { bb = 31; continue; } 0x686 => { bb = 32; continue; } 0x6b2 => { bb = 33; continue; } 0x6b8 => { bb = 34; continue; } 0x6d1 => { bb = 35; continue; } 0x6ee => { bb = 36; continue; } 0x723 => { bb = 37; continue; } 0x753 => { bb = 38; continue; } 0x783 => { bb = 39; continue; } 0x7d4 => { bb = 40; continue; } 0x825 => { bb = 41; continue; } 0x873 => { bb = 42; continue; } 0x8bf => { bb = 43; continue; } 0x8f5 => { bb = 44; continue; } 0x8f9 => { bb = 45; continue; } 0x900 => { bb = 46; continue; } 0x913 => { bb = 47; continue; } _ => { let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(j_tgt, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; } } }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d6580(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645790; const P1: u64 = 0x1426455a8; const P2: u64 = 0x140000000; const P3: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = (((r8 as u16)) as u32) as u32 as u64; rbp = (rdx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x10u64))) as u32 as u64; rsi = ((rax as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rbp.wrapping_add(0x8u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((rsi as u32)) as u32); bb = 2; continue; }
            2 => { crate::simd_ops::lk_add_u32(rbp.wrapping_add(0x14u64), (1) as u32); rax = (0xaaaaaaaaaaaaaaabu64) as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x40u64))) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x1f0u64))) as u64; rcx = ((rcx).wrapping_add(1)) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; rax = ((rdi.wrapping_sub(0x95u64)) as u32) as u32 as u64; rdx = (((rcx) as i64).wrapping_mul((0x7cu64) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x24u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 20; continue; } bb = 3; continue; }
            3 => { r8 = ((P2) as u64) as u64; rax = (rax as u32 as i32 as i64) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(rax).wrapping_add(0x20d6738u64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x20d66f4u64))) as u32 as u64; fl_l = ((rcx).wrapping_add(r8)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; j_tgt = (rcx) as u64; bb = 0xffff_fffe; continue; }
            4 => { crate::simd_ops::wu8(rdx.wrapping_add(0x25u64), (0x0u8) as u8); fl_l = (crate::simd_ops::lk_and_u8(rdi.wrapping_add(rdx).wrapping_sub(0x95u64), (0xfeu8) as u8)) as u8 as u64; fl_r = fl_l; bb = 20; continue; }
            5 => { crate::simd_ops::wu8(rdx.wrapping_add(0x26u64), (0x0u8) as u8); bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::lk_and_u8(rdi.wrapping_add(rdx).wrapping_sub(0x95u64), (0xfeu8) as u8)) as u8 as u64; fl_r = fl_l; bb = 20; continue; }
            7 => { crate::simd_ops::wu16(rdx.wrapping_add(0x27u64), (0x8080u16) as u16); bb = 20; continue; }
            8 => { crate::simd_ops::wu8(rdx.wrapping_add(0x27u64), (0x80u8) as u8); bb = 20; continue; }
            9 => { crate::simd_ops::wu8(rdx.wrapping_add(0x28u64), (0x80u8) as u8); bb = 20; continue; }
            10 => { crate::simd_ops::wu16(rdx.wrapping_add(0x29u64), (0x8080u16) as u16); bb = 20; continue; }
            11 => { crate::simd_ops::wu8(rdx.wrapping_add(0x29u64), (0x80u8) as u8); bb = 20; continue; }
            12 => { crate::simd_ops::wu8(rdx.wrapping_add(0x2au64), (0x80u8) as u8); bb = 20; continue; }
            13 => { xmm2 = 0u128; xmm0 = crate::simd_ops::vunpcklps(xmm2, xmm2); rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdx.wrapping_add(0x30u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rdx.wrapping_add(0x38u64), ((xmm2 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), ((xmm2 as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x2cu64), ((rax as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x24u64), ((xmm0 as u64)) as u64); bb = 20; continue; }
            14 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdx.wrapping_add(0x3cu64), (rax) as u64); crate::simd_ops::wu32(rdx.wrapping_add(0x44u64), ((rax as u32)) as u32); bb = 20; continue; }
            15 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdx.wrapping_add(0x48u64), (rax) as u64); crate::simd_ops::wu32(rdx.wrapping_add(0x50u64), ((rax as u32)) as u32); bb = 20; continue; }
            16 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rdx.wrapping_add(0x54u64), (0x0u8) as u8); crate::simd_ops::wu32(rdx.wrapping_add(0x56u64), ((rax as u32)) as u32); bb = 20; continue; }
            17 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rdx.wrapping_add(0x5eu64), (0x0u8) as u8); crate::simd_ops::wu32(rdx.wrapping_add(0x60u64), ((rax as u32)) as u32); bb = 20; continue; }
            18 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rdx.wrapping_add(0x68u64), (0x0u8) as u8); crate::simd_ops::wu32(rdx.wrapping_add(0x6au64), ((rax as u32)) as u32); bb = 20; continue; }
            19 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rdx.wrapping_add(0x72u64), (0x0u8) as u8); crate::simd_ops::wu32(rdx.wrapping_add(0x74u64), ((rax as u32)) as u32); bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::lk_add_u32(rbp.wrapping_add(0x14u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { rax = (0xffffffffu32) as u32 as u64; rcx = ((rbp.wrapping_add(0x8u64)) as u64) as u64; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rbp.wrapping_add(0x10u64), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 22; continue; }
            22 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x1420d6580u64) { 0x2a => { bb = 1; continue; } 0x37 => { bb = 2; continue; } 0x75 => { bb = 3; continue; } 0x94 => { bb = 4; continue; } 0xa5 => { bb = 5; continue; } 0xa9 => { bb = 6; continue; } 0xb6 => { bb = 7; continue; } 0xc1 => { bb = 8; continue; } 0xc7 => { bb = 9; continue; } 0xcd => { bb = 10; continue; } 0xd5 => { bb = 11; continue; } 0xdb => { bb = 12; continue; } 0xe1 => { bb = 13; continue; } 0x106 => { bb = 14; continue; } 0x111 => { bb = 15; continue; } 0x11c => { bb = 16; continue; } 0x127 => { bb = 17; continue; } 0x132 => { bb = 18; continue; } 0x13d => { bb = 19; continue; } 0x146 => { bb = 20; continue; } 0x14c => { bb = 21; continue; } 0x15e => { bb = 22; continue; } _ => { let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(j_tgt, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; } } }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d6770(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645790; const P1: u64 = 0x1426455a8; const P2: u64 = 0x143426890; const P3: u64 = 0x143426eb8; const P4: u64 = 0x140000000; const P5: u64 = 0x143426eec; const P6: u64 = 0x1420d3ab0; const P7: u64 = 0x143426780; const P8: u64 = 0x1420d3b80; const P9: u64 = 0x14183ad82; const P10: u64 = 0x14183ad88; const P11: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0; let mut xmm11: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 39>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rbp = ((rax.wrapping_sub(0x38u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x128u64)) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x20u64), (rsi) as u64); rbx = ((rdx.wrapping_add(0x8u64)) as u64) as u64; crate::simd_ops::wu64(rax.wrapping_sub(0x18u64), (rdi) as u64); crate::simd_ops::wu64(rax.wrapping_sub(0x20u64), (r14) as u64); r14 = (rdx) as u64; crate::simd_ops::wu64(rax.wrapping_sub(0x28u64), (r15) as u64); r15 = (rcx) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x38u64), (xmm6) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x48u64), (xmm7) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x58u64), (xmm8) as u128); xmm8 = (xmm3) as u128; rdi = (((r8 as u16)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rsi = ((rax as u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rsi as u32)) as u32); bb = 2; continue; }
            2 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(0xcu64), (1) as u32); rax = (0xaaaaaaaaaaaaaaabu64) as u64; r8 = (crate::simd_ops::ru64(r14.wrapping_add(0x40u64))) as u64; xmm7 = ((crate::simd_ops::ru32(P2)) as u32) as u128; xmm2 = (crate::simd_ops::ru128(r15.wrapping_add(0x20u64))) as u128; xmm4 = ((crate::simd_ops::ru64(r15.wrapping_add(0x30u64))) as u64) as u128; xmm5 = ((crate::simd_ops::ru64(r15.wrapping_add(0x48u64))) as u64) as u128; rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x1f0u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x118u64))) as u64; rcx = ((rcx).wrapping_add(1)) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm2) as u128); let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; xmm2 = crate::simd_ops::vshufps(xmm2, xmm2, 0x55u8); rdx = ((rdx).wrapping_shr(1u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x204u64))) as u32) as u32 as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128); xmm3 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(core::ptr::read_unaligned((P3) as *const u32))).to_bits() as u128); rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x205u64))) as u32) as u32 as u64; xmm0 = 0u128; xmm2 = (xmm0 & !0xffff_ffffu128) | (xmm2 & 0xffff_ffffu128); xmm1 = (xmm0 & !0xffff_ffffu128) | (xmm4 & 0xffff_ffffu128); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(xmm3 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm3 as u32) }).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(xmm3 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm3 as u32) }).to_bits() as u128); xmm3 = (crate::simd_ops::ru128(r15.wrapping_add(0x38u64))) as u128; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x110u64))) as u64; xmm2 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) < f32::from_bits(xmm7 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm7 as u32) }).to_bits() as u128); xmm0 = crate::simd_ops::vinsertps(xmm0, xmm2, 0x10u8); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm0) as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) < f32::from_bits(xmm7 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm7 as u32) }).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | (xmm1 & 0xffff_ffffu128); xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm3) as u128); rsi = (((rcx) as i64).wrapping_mul((0x7cu64) as i64) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm4 as u64)) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), ((xmm4 as u64)) as u64); xmm4 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(core::ptr::read_unaligned((P3) as *const u32))).to_bits() as u128); xmm0 = 0u128; xmm1 = (xmm0 & !0xffff_ffffu128) | (xmm5 & 0xffff_ffffu128); xmm3 = crate::simd_ops::vshufps(xmm3, xmm3, 0x55u8); xmm2 = (xmm0 & !0xffff_ffffu128) | (xmm3 & 0xffff_ffffu128); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(xmm4 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm4 as u32) }).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(xmm4 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm4 as u32) }).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) < f32::from_bits(xmm7 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm7 as u32) }).to_bits() as u128); xmm0 = crate::simd_ops::vinsertps(xmm0, xmm2, 0x10u8); xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) < f32::from_bits(xmm7 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm7 as u32) }).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | (xmm1 & 0xffff_ffffu128); rsi = ((rsi).wrapping_add(r8)) as u64; rax = ((rdi.wrapping_sub(0x95u64)) as u32) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x78u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), ((xmm5 as u64)) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x78u64), ((xmm5 as u64)) as u64); fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x24u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 26; continue; } bb = 3; continue; }
            3 => { rdx = ((P4) as u64) as u64; rax = (rax as u32 as i32 as i64) as u64; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rax).wrapping_add(0x20d6d44u64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x20d6d00u64))) as u32 as u64; fl_l = ((rcx).wrapping_add(rdx)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; j_tgt = (rcx) as u64; bb = 0xffff_fffe; continue; }
            4 => { xmm0 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) * f32::from_bits(core::ptr::read_unaligned((P5) as *const u32))).to_bits() as u128); rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm0 as u32)))) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x25u64), ((rax as u8)) as u8); fl_l = (crate::simd_ops::lk_or_u8(rdi.wrapping_add(rsi).wrapping_sub(0x95u64), (0x1u8) as u8)) as u8 as u64; fl_r = fl_l; bb = 26; continue; }
            5 => { xmm0 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) * f32::from_bits(core::ptr::read_unaligned((P5) as *const u32))).to_bits() as u128); rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm0 as u32)))) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x26u64), ((rax as u8)) as u8); bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::lk_or_u8(rdi.wrapping_add(rsi).wrapping_sub(0x95u64), (0x1u8) as u8)) as u8 as u64; fl_r = fl_l; bb = 26; continue; }
            7 => { bb = 8; continue; }
            8 => { crate::simd_ops::wu32(rbp.wrapping_add(0x40u64), ((xmm8 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x44u64), ((xmm8 as u32)) as u32); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x40u64))) as u64; r8 = ((rsp.wrapping_add(0x60u64)) as u64) as u64; r9 = (r9 & !0xffu64) | (((0x1u8) as u8 as u64)); rcx = ((rbp.wrapping_add(0x48u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm4 = ((crate::simd_ops::ru32(P5)) as u32) as u128; rcx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x40u64), (rcx) as u64); xmm1 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(core::ptr::read_unaligned((rbp.wrapping_add(0x40u64)) as *const u32))).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm0 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(core::ptr::read_unaligned((P7) as *const u32))).to_bits() as u128); rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm0 as u32)))) as u32 as u64; xmm0 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(core::ptr::read_unaligned((rbp.wrapping_add(0x44u64)) as *const u32))).to_bits() as u128); crate::simd_ops::wu8(rbp.wrapping_add(0x50u64), ((rax as u8)) as u8); xmm2 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm3 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(core::ptr::read_unaligned((P7) as *const u32))).to_bits() as u128); rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm3 as u32)))) as u32 as u64; crate::simd_ops::wu8(rbp.wrapping_add(0x51u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru16(rbp.wrapping_add(0x50u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rsi.wrapping_add(0x27u64), ((rax as u16)) as u16); bb = 26; continue; }
            9 => { bb = 10; continue; }
            10 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rdx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; xmm0 = (xmm8) as u128; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(core::ptr::read_unaligned((P5) as *const u32))).to_bits() as u128); xmm2 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(core::ptr::read_unaligned((P5) as *const u32))).to_bits() as u128); xmm1 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(core::ptr::read_unaligned((P7) as *const u32))).to_bits() as u128); rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm1 as u32)))) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x27u64), ((rax as u8)) as u8); bb = 26; continue; }
            11 => { bb = 12; continue; }
            12 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rdx = ((rsp.wrapping_add(0x6cu64)) as u64) as u64; xmm0 = (xmm8) as u128; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(core::ptr::read_unaligned((P5) as *const u32))).to_bits() as u128); xmm2 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(core::ptr::read_unaligned((P5) as *const u32))).to_bits() as u128); xmm1 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(core::ptr::read_unaligned((P7) as *const u32))).to_bits() as u128); rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm1 as u32)))) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x28u64), ((rax as u8)) as u8); bb = 26; continue; }
            13 => { bb = 14; continue; }
            14 => { crate::simd_ops::wu32(rbp.wrapping_add(0x40u64), ((xmm8 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x44u64), ((xmm8 as u32)) as u32); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x40u64))) as u64; r8 = ((rsp.wrapping_add(0x78u64)) as u64) as u64; r9 = (r9 & !0xffu64) | (((0x1u8) as u8 as u64)); rcx = ((rbp.wrapping_sub(0x70u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm4 = ((crate::simd_ops::ru32(P5)) as u32) as u128; rcx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x40u64), (rcx) as u64); xmm1 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(core::ptr::read_unaligned((rbp.wrapping_add(0x40u64)) as *const u32))).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm0 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(core::ptr::read_unaligned((P7) as *const u32))).to_bits() as u128); rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm0 as u32)))) as u32 as u64; xmm0 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(core::ptr::read_unaligned((rbp.wrapping_add(0x44u64)) as *const u32))).to_bits() as u128); crate::simd_ops::wu8(rbp.wrapping_add(0x50u64), ((rax as u8)) as u8); xmm2 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm3 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(core::ptr::read_unaligned((P7) as *const u32))).to_bits() as u128); rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm3 as u32)))) as u32 as u64; crate::simd_ops::wu8(rbp.wrapping_add(0x51u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru16(rbp.wrapping_add(0x50u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rsi.wrapping_add(0x29u64), ((rax as u16)) as u16); bb = 26; continue; }
            15 => { bb = 16; continue; }
            16 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rdx = ((rsp.wrapping_add(0x78u64)) as u64) as u64; xmm0 = (xmm8) as u128; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(core::ptr::read_unaligned((P5) as *const u32))).to_bits() as u128); xmm2 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(core::ptr::read_unaligned((P5) as *const u32))).to_bits() as u128); xmm1 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(core::ptr::read_unaligned((P7) as *const u32))).to_bits() as u128); rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm1 as u32)))) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x29u64), ((rax as u8)) as u8); bb = 26; continue; }
            17 => { bb = 18; continue; }
            18 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rdx = ((rbp.wrapping_sub(0x7cu64)) as u64) as u64; xmm0 = (xmm8) as u128; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(core::ptr::read_unaligned((P5) as *const u32))).to_bits() as u128); xmm2 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(core::ptr::read_unaligned((P5) as *const u32))).to_bits() as u128); xmm1 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(core::ptr::read_unaligned((P7) as *const u32))).to_bits() as u128); rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm1 as u32)))) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x2au64), ((rax as u8)) as u8); bb = 26; continue; }
            19 => { xmm7 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) * f32::from_bits(core::ptr::read_unaligned((P7) as *const u32))).to_bits() as u128); crate::simd_ops::wu128(rsp.wrapping_add(0xd0u64), (xmm9) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0xc0u64), (xmm10) as u128); xmm0 = (xmm7) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0xb0u64), (xmm11) as u128); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm10 = (xmm0) as u128; xmm0 = (xmm7) as u128; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm11 = (xmm0) as u128; xmm0 = (xmm7) as u128; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm6 = (xmm0) as u128; xmm0 = (xmm7) as u128; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm9 = (xmm0) as u128; xmm0 = (xmm7) as u128; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm8 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm0 = (xmm7) as u128; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm5 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm10 as u32)).to_bits() as u128); xmm3 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm10 as u32)).to_bits() as u128); xmm10 = (crate::simd_ops::ru128(rsp.wrapping_add(0xc0u64))) as u128; xmm4 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm11 as u32)).to_bits() as u128); xmm2 = (xmm11 & !0xffff_ffffu128) | ((f32::from_bits(xmm11 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm1 = (xmm9 & !0xffff_ffffu128) | ((f32::from_bits(xmm9 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm7 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm1 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm9 as u32)).to_bits() as u128); xmm0 = (xmm11 & !0xffff_ffffu128) | ((f32::from_bits(xmm11 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm11 = (crate::simd_ops::ru128(rsp.wrapping_add(0xb0u64))) as u128; xmm6 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) - f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm1 = (xmm9 & !0xffff_ffffu128) | ((f32::from_bits(xmm9 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm9 = (crate::simd_ops::ru128(rsp.wrapping_add(0xd0u64))) as u128; xmm3 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = crate::simd_ops::vunpcklps(xmm6, xmm2); crate::simd_ops::wu64(rsi.wrapping_add(0x30u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rsi.wrapping_add(0x38u64), ((xmm3 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x5cu64), ((xmm3 as u32)) as u32); crate::simd_ops::wu32(rsi.wrapping_add(0x2cu64), ((xmm7 as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x64u64), ((xmm0 as u64)) as u64); bb = 26; continue; }
            20 => { crate::simd_ops::wu32(rsi.wrapping_add(0x3cu64), ((xmm8 as u32)) as u32); crate::simd_ops::wu32(rsi.wrapping_add(0x40u64), ((xmm8 as u32)) as u32); crate::simd_ops::wu32(rsi.wrapping_add(0x44u64), ((xmm8 as u32)) as u32); bb = 26; continue; }
            21 => { crate::simd_ops::wu32(rsi.wrapping_add(0x48u64), ((xmm8 as u32)) as u32); crate::simd_ops::wu32(rsi.wrapping_add(0x4cu64), ((xmm8 as u32)) as u32); crate::simd_ops::wu32(rsi.wrapping_add(0x50u64), ((xmm8 as u32)) as u32); bb = 26; continue; }
            22 => { rax = ((crate::simd_ops::ru16(rsi.wrapping_add(0x5au64))) as u32) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128); xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm1 as u32)))) as u32 as u64; crate::simd_ops::wu16(rsi.wrapping_add(0x56u64), ((rax as u16)) as u16); crate::simd_ops::wu8(rsi.wrapping_add(0x54u64), (0x1u8) as u8); bb = 26; continue; }
            23 => { rax = ((crate::simd_ops::ru16(rsi.wrapping_add(0x64u64))) as u32) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128); xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm1 as u32)))) as u32 as u64; crate::simd_ops::wu16(rsi.wrapping_add(0x60u64), ((rax as u16)) as u16); crate::simd_ops::wu8(rsi.wrapping_add(0x5eu64), (0x1u8) as u8); bb = 26; continue; }
            24 => { rax = ((crate::simd_ops::ru16(rsi.wrapping_add(0x6eu64))) as u32) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128); xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm1 as u32)))) as u32 as u64; crate::simd_ops::wu16(rsi.wrapping_add(0x6au64), ((rax as u16)) as u16); crate::simd_ops::wu8(rsi.wrapping_add(0x68u64), (0x1u8) as u8); bb = 26; continue; }
            25 => { rax = ((crate::simd_ops::ru16(rsi.wrapping_add(0x78u64))) as u32) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128); xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm1 as u32)))) as u32 as u64; crate::simd_ops::wu16(rsi.wrapping_add(0x74u64), ((rax as u16)) as u16); crate::simd_ops::wu8(rsi.wrapping_add(0x72u64), (0x1u8) as u8); bb = 26; continue; }
            26 => { fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(0xcu64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0xe0u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0xf0u64))) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x100u64))) as u128; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x120u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x158u64))) as u64; if (((fl_l as u32)) != 0) { bb = 28; continue; } bb = 27; continue; }
            27 => { rax = (0xffffffffu32) as u32 as u64; rcx = (rbx) as u64; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P11)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 28; continue; }
            28 => { rsp = ((rsp).wrapping_add(0x128u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x1420d6770u64) { 0x52 => { bb = 1; continue; } 0x5e => { bb = 2; continue; } 0x199 => { bb = 3; continue; } 0x1b6 => { bb = 4; continue; } 0x1d2 => { bb = 5; continue; } 0x1e1 => { bb = 6; continue; } 0x1ee => { bb = 7; continue; } 0x1f0 => { bb = 8; continue; } 0x25b => { bb = 9; continue; } 0x260 => { bb = 10; continue; } 0x296 => { bb = 11; continue; } 0x2a0 => { bb = 12; continue; } 0x2d6 => { bb = 13; continue; } 0x2e0 => { bb = 14; continue; } 0x34b => { bb = 15; continue; } 0x350 => { bb = 16; continue; } 0x386 => { bb = 17; continue; } 0x390 => { bb = 18; continue; } 0x3c5 => { bb = 19; continue; } 0x4a5 => { bb = 20; continue; } 0x4b9 => { bb = 21; continue; } 0x4ca => { bb = 22; continue; } 0x4e9 => { bb = 23; continue; } 0x508 => { bb = 24; continue; } 0x527 => { bb = 25; continue; } 0x544 => { bb = 26; continue; } 0x575 => { bb = 27; continue; } 0x586 => { bb = 28; continue; } _ => { let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(j_tgt, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; } } }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d6d70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645790; const P1: u64 = 0x1426455a8; const P2: u64 = 0x143426890; const P3: u64 = 0x143426eb8; const P4: u64 = 0x140000000; const P5: u64 = 0x1420d3ab0; const P6: u64 = 0x143426eec; const P7: u64 = 0x143426780; const P8: u64 = 0x142645598;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 29>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0xb8u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0xa0u64), (xmm7) as u128); rsi = r9; rdi = (((r8 as u16)) as u32) as u32 as u64; r14 = rdx; r15 = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru32(r14.wrapping_add(0x10u64))) as u32 as u64; rbp = ((rax as u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = r14.wrapping_add(0x8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(r14.wrapping_add(0x10u64), ((rbp as u32)) as u32); bb = 2; continue; }
            2 => { crate::simd_ops::lk_add_u32(r14.wrapping_add(0x14u64), (1) as u32); rax = (0xaaaaaaaaaaaaaaabu64) as u64; r8 = (crate::simd_ops::ru64(r14.wrapping_add(0x40u64))) as u64; xmm7 = ((crate::simd_ops::ru32(P2)) as u32) as u128; xmm2 = (crate::simd_ops::ru128(r15.wrapping_add(0x20u64))) as u128; xmm4 = ((crate::simd_ops::ru64(r15.wrapping_add(0x30u64))) as u64) as u128; rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x1f0u64))) as u64; xmm5 = ((crate::simd_ops::ru64(r15.wrapping_add(0x48u64))) as u64) as u128; xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm2) as u128); xmm2 = crate::simd_ops::vshufps(xmm2, xmm2, 0x55u8); rcx = ((rcx).wrapping_add(1)) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x204u64))) as u32) as u32 as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128); xmm3 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(crate::simd_ops::ru32(P3))).to_bits() as u128); rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x205u64))) as u32) as u32 as u64; xmm0 = 0u128; xmm2 = (xmm0 & !0xffff_ffffu128) | (xmm2 & 0xffff_ffffu128); xmm1 = (xmm0 & !0xffff_ffffu128) | (xmm4 & 0xffff_ffffu128); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(xmm3 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm3 as u32) }).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(xmm3 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm3 as u32) }).to_bits() as u128); xmm3 = (crate::simd_ops::ru128(r15.wrapping_add(0x38u64))) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) < f32::from_bits(xmm7 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm7 as u32) }).to_bits() as u128); xmm0 = crate::simd_ops::vinsertps(xmm0, xmm2, 0x10u8); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x68u64), (xmm0) as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) < f32::from_bits(xmm7 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm7 as u32) }).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | (xmm1 & 0xffff_ffffu128); xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm3) as u128); r11 = (((rcx) as i64).wrapping_mul((0x7cu64) as i64) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm4 as u64)) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), ((xmm4 as u64)) as u64); xmm4 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(crate::simd_ops::ru32(P3))).to_bits() as u128); xmm0 = 0u128; xmm1 = (xmm0 & !0xffff_ffffu128) | (xmm5 & 0xffff_ffffu128); xmm3 = crate::simd_ops::vshufps(xmm3, xmm3, 0x55u8); xmm2 = (xmm0 & !0xffff_ffffu128) | (xmm3 & 0xffff_ffffu128); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(xmm4 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm4 as u32) }).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(xmm4 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm4 as u32) }).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) < f32::from_bits(xmm7 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm7 as u32) }).to_bits() as u128); xmm0 = crate::simd_ops::vinsertps(xmm0, xmm2, 0x10u8); xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) < f32::from_bits(xmm7 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm7 as u32) }).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | (xmm1 & 0xffff_ffffu128); r11 = ((r11).wrapping_add(r8)) as u64; rax = ((rdi.wrapping_sub(0x95u64)) as u32) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x80u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), ((xmm5 as u64)) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), ((xmm5 as u64)) as u64); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x24; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 9; continue; } bb = 3; continue; }
            3 => { rdx = P4; rax = (rax as u32 as i32 as i64) as u64; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rax).wrapping_add(0x20d70a8u64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x20d7098u64))) as u32 as u64; fl_l = ((rcx).wrapping_add(rdx)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; j_tgt = rcx; bb = 0xffff_fffe; continue; }
            4 => { bb = 5; continue; }
            5 => { r9 = (r9 & !0xffu64) | (((0x1u8) as u8 as u64)); r8 = rsp.wrapping_add(0x68); rdx = rsi; rcx = rsp.wrapping_add(0xf8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm4 = ((crate::simd_ops::ru32(P6)) as u32) as u128; rcx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xf0u64), rcx); xmm1 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0xf0u64)))).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm0 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(crate::simd_ops::ru32(P7))).to_bits() as u128); rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm0 as u32)))) as u32 as u64; xmm0 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0xf4u64)))).to_bits() as u128); crate::simd_ops::wu8(rsp.wrapping_add(0x100u64), ((rax as u8)) as u8); xmm2 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm3 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(crate::simd_ops::ru32(P7))).to_bits() as u128); rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm3 as u32)))) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x101u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru16(rsp.wrapping_add(0x100u64))) as u32) as u32 as u64; crate::simd_ops::wu16(r11.wrapping_add(0x27u64), ((rax as u16)) as u16); bb = 9; continue; }
            6 => { bb = 7; continue; }
            7 => { r9 = (r9 & !0xffu64) | (((0x1u8) as u8 as u64)); r8 = rsp.wrapping_add(0x80); rdx = rsi; rcx = rsp.wrapping_add(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm4 = ((crate::simd_ops::ru32(P6)) as u32) as u128; rcx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xf0u64), rcx); xmm1 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0xf0u64)))).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm0 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(crate::simd_ops::ru32(P7))).to_bits() as u128); rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm0 as u32)))) as u32 as u64; xmm0 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0xf4u64)))).to_bits() as u128); crate::simd_ops::wu8(rsp.wrapping_add(0x100u64), ((rax as u8)) as u8); xmm2 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm3 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(crate::simd_ops::ru32(P7))).to_bits() as u128); rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm3 as u32)))) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x101u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru16(rsp.wrapping_add(0x100u64))) as u32) as u32 as u64; crate::simd_ops::wu16(r11.wrapping_add(0x29u64), ((rax as u16)) as u16); bb = 9; continue; }
            8 => { xmm0 = 0u128; rax = 0xb6; fl_l = ((rdi as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; rdi = (rdi & !0xffffu64) | ((((fl_l as u16).wrapping_sub(fl_r as u16)) as u16 as u64)); rax = (((rdi as u16)) as u32) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; crate::simd_ops::wu8(r11.wrapping_add(rdx.wrapping_mul(2)).wrapping_add(0x54u64), (0x1u8) as u8); rcx = ((crate::simd_ops::ru16(r11.wrapping_add(rax.wrapping_mul(2)).wrapping_add(0x5au64))) as u32) as u32 as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | (((((rcx as u32)) as i32) as f32).to_bits() as u128); xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x108u64)))).to_bits() as u128); rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm1 as u32)))) as u32 as u64; crate::simd_ops::wu16(r11.wrapping_add(rdx.wrapping_mul(2)).wrapping_add(0x56u64), ((rax as u16)) as u16); rax = ((crate::simd_ops::ru16(r11.wrapping_add(rdx.wrapping_mul(2)).wrapping_add(0x5cu64))) as u32) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128); xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x10cu64)))).to_bits() as u128); rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm1 as u32)))) as u32 as u64; crate::simd_ops::wu16(r11.wrapping_add(rdx.wrapping_mul(2)).wrapping_add(0x58u64), ((rax as u16)) as u16); bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::lk_add_u32(r14.wrapping_add(0x14u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = 0xffffffff; rcx = r14.wrapping_add(0x8); let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(r14.wrapping_add(0x10u64), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            11 => { xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0xa0u64))) as u128; rsp = ((rsp).wrapping_add(0xb8u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x1420d6d70u64) { 0x3b => { bb = 1; continue; } 0x49 => { bb = 2; continue; } 0x17c => { bb = 3; continue; } 0x199 => { bb = 4; continue; } 0x1a0 => { bb = 5; continue; } 0x21d => { bb = 6; continue; } 0x220 => { bb = 7; continue; } 0x29a => { bb = 8; continue; } 0x2f5 => { bb = 9; continue; } 0x2fc => { bb = 10; continue; } 0x30f => { bb = 11; continue; } _ => { return crate::simd_ops::tail_stk_xmm(j_tgt, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); } } }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d70e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x72; const P1: u64 = 0x1420d40a0; const P2: u64 = 0x1420d4010; const P3: u64 = 0x1417a1dd0; const P4: u64 = 0x4; const P5: u64 = 0x3f08; const P6: u64 = 0x1420d4ae0; const P7: u64 = 0x4118; const P8: u64 = 0x4328; const P9: u64 = 0x4538;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rdi = rdx; rbx = rcx; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = P1; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x1u32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rcx = rsp.wrapping_add(0x20); rax = P2; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = 0; fl_l = 0; fl_r = 0; xmm0 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), ((rax as u32)) as u32); crate::simd_ops::wu128(rsp.wrapping_add(0x34u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rbx, (0xbu32) as u32); rax = rbx; crate::simd_ops::wu8(rbx.wrapping_add(P4), (0x0u8) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { r8 = ((rdi.wrapping_add(P5)) as u64) as u64; rdx = 0x1; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = ((rdi.wrapping_add(P7)) as u64) as u64; rdx = 0x2; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = ((rdi.wrapping_add(P8)) as u64) as u64; rdx = 0x3; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = ((rdi.wrapping_add(P9)) as u64) as u64; rdx = 0x4; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rbx; crate::simd_ops::wu8(rbx.wrapping_add(P4), (0x1u8) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d71c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645790; const P1: u64 = 0x1426455a8; const P2: u64 = 0x143426eb8; const P3: u64 = 0x143426890; const P4: u64 = 0x140000000; const P5: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rbp) as u64); rbx = ((rdx.wrapping_add(0x8u64)) as u64) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), (rdi) as u64); rdi = (((r9 as u8)) as u32) as u32 as u64; crate::simd_ops::wu64(rax.wrapping_sub(0x10u64), (r14) as u64); r14 = (rdx) as u64; crate::simd_ops::wu64(rax.wrapping_sub(0x18u64), (r15) as u64); r15 = (rcx) as u64; rsi = (((r8 as u16)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rbp = ((rax as u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rbp as u32)) as u32); bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm8) as u128); rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm9) as u128); fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rdx as u32)) as u32); if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 10; continue; } bb = 3; continue; }
            3 => { xmm4 = ((crate::simd_ops::ru64(r15.wrapping_add(0x30u64))) as u64) as u128; rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x40u64))) as u64; xmm2 = (crate::simd_ops::ru128(r15.wrapping_add(0x20u64))) as u128; xmm3 = (crate::simd_ops::ru128(r15.wrapping_add(0x38u64))) as u128; xmm5 = ((crate::simd_ops::ru64(r15.wrapping_add(0x48u64))) as u64) as u128; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x204u64))) as u32) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128); xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(core::ptr::read_unaligned((P2) as *const u32))).to_bits() as u128); rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x205u64))) as u32) as u32 as u64; xmm0 = 0u128; xmm9 = (xmm0 & !0xffff_ffffu128) | (xmm4 & 0xffff_ffffu128); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm2) as u128); xmm2 = crate::simd_ops::vshufps(xmm2, xmm2, 0x55u8); xmm8 = (xmm0 & !0xffff_ffffu128) | (xmm2 & 0xffff_ffffu128); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), ((xmm5 as u64)) as u64); xmm8 = (xmm8 & !0xffff_ffffu128) | ((if f32::from_bits(xmm8 as u32) > f32::from_bits(xmm1 as u32) { f32::from_bits(xmm8 as u32) } else { f32::from_bits(xmm1 as u32) }).to_bits() as u128); xmm9 = (xmm9 & !0xffff_ffffu128) | ((if f32::from_bits(xmm9 as u32) > f32::from_bits(xmm1 as u32) { f32::from_bits(xmm9 as u32) } else { f32::from_bits(xmm1 as u32) }).to_bits() as u128); xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x80u64), (xmm7) as u128); rax = ((rsi.wrapping_sub(0x95u64)) as u32) as u32 as u64; xmm7 = ((crate::simd_ops::ru32(P3)) as u32) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm4 as u64)) as u64); xmm4 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(core::ptr::read_unaligned((P2) as *const u32))).to_bits() as u128); xmm0 = 0u128; xmm5 = (xmm0 & !0xffff_ffffu128) | (xmm5 & 0xffff_ffffu128); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm3) as u128); xmm3 = crate::simd_ops::vshufps(xmm3, xmm3, 0x55u8); xmm2 = (xmm0 & !0xffff_ffffu128) | (xmm3 & 0xffff_ffffu128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(xmm4 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm4 as u32) }).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((if f32::from_bits(xmm5 as u32) > f32::from_bits(xmm4 as u32) { f32::from_bits(xmm5 as u32) } else { f32::from_bits(xmm4 as u32) }).to_bits() as u128); xmm1 = 0u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) < f32::from_bits(xmm7 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm7 as u32) }).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((if f32::from_bits(xmm5 as u32) < f32::from_bits(xmm7 as u32) { f32::from_bits(xmm5 as u32) } else { f32::from_bits(xmm7 as u32) }).to_bits() as u128); xmm8 = (xmm8 & !0xffff_ffffu128) | ((if f32::from_bits(xmm8 as u32) < f32::from_bits(xmm7 as u32) { f32::from_bits(xmm8 as u32) } else { f32::from_bits(xmm7 as u32) }).to_bits() as u128); xmm9 = (xmm9 & !0xffff_ffffu128) | ((if f32::from_bits(xmm9 as u32) < f32::from_bits(xmm7 as u32) { f32::from_bits(xmm9 as u32) } else { f32::from_bits(xmm7 as u32) }).to_bits() as u128); xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x80u64))) as u128; xmm1 = (xmm3) as u128; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x5u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 10; continue; } bb = 4; continue; }
            4 => { r8 = ((P4) as u64) as u64; rax = (rax as u32 as i32 as i64) as u64; rcx = (crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x20d73b0u64))) as u32 as u64; fl_l = ((rcx).wrapping_add(r8)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; j_tgt = (rcx) as u64; bb = 0xffff_fffe; continue; }
            5 => { xmm8 = (xmm9) as u128; bb = 11; continue; }
            6 => { xmm8 = (xmm2) as u128; bb = 11; continue; }
            7 => { xmm8 = (xmm5) as u128; bb = 11; continue; }
            8 => { xmm1 = (xmm8) as u128; xmm2 = (xmm9) as u128; xmm8 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(xmm2 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm2 as u32) }).to_bits() as u128); xmm8 = (xmm8) as u128; bb = 11; continue; }
            9 => { xmm1 = (xmm2) as u128; xmm8 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(xmm5 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm5 as u32) }).to_bits() as u128); xmm8 = (xmm8) as u128; xmm2 = (xmm5) as u128; bb = 11; continue; }
            10 => { xmm8 = 0u128; bb = 11; continue; }
            11 => { xmm9 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; rax = ((rdx.wrapping_sub(0x1u64)) as u32) as u32 as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = (0xffffffffu32) as u32 as u64; rcx = (rbx) as u64; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 13; continue; }
            13 => { xmm0 = (xmm8) as u128; xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; rsp = ((rsp).wrapping_add(0xa0u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x1420d71c0u64) { 0x42 => { bb = 1; continue; } 0x4e => { bb = 2; continue; } 0x7c => { bb = 3; continue; } 0x14d => { bb = 4; continue; } 0x163 => { bb = 5; continue; } 0x16a => { bb = 6; continue; } 0x170 => { bb = 7; continue; } 0x176 => { bb = 8; continue; } 0x18b => { bb = 9; continue; } 0x19e => { bb = 10; continue; } 0x1a3 => { bb = 11; continue; } 0x1cb => { bb = 12; continue; } 0x1dc => { bb = 13; continue; } _ => { let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(j_tgt, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; } } }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d7530(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645790; const P1: u64 = 0x1426455a8; const P2: u64 = 0x143426890; const P3: u64 = 0x143426eb8; const P4: u64 = 0x140000000; const P5: u64 = 0x1434264b4; const P6: u64 = 0x1434268d0; const P7: u64 = 0x143426bf8; const P8: u64 = 0x143426780; const P9: u64 = 0x1420d3d40; const P10: u64 = 0x1420d5450; const P11: u64 = 0x142645598;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0xb0u64)) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x28u64), (xmm6) as u128); r14 = r8; crate::simd_ops::wu128(rax.wrapping_sub(0x38u64), (xmm7) as u128); r15 = rdx; rdi = (((r9 as u16)) as u32) as u32 as u64; rsi = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru32(r14.wrapping_add(0x10u64))) as u32 as u64; rbp = ((rax as u32)) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = r14.wrapping_add(0x8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(r14.wrapping_add(0x10u64), ((rbp as u32)) as u32); bb = 2; continue; }
            2 => { crate::simd_ops::lk_add_u32(r14.wrapping_add(0x14u64), (1) as u32); r9 = ((crate::simd_ops::ru8(rsp.wrapping_add(0xf0u64))) as u32) as u32 as u64; r8 = (crate::simd_ops::ru64(r14.wrapping_add(0x40u64))) as u64; fl_l = ((r9 as u8)) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; if (((((fl_l as u8)) & ((fl_r as u8))) as i8) >= 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru8(r15.wrapping_add(0x4fd8u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = 0x174; rdx = ((rax.wrapping_add(r8)) as u64) as u64; bb = 7; continue; }
            5 => { rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x1f0u64))) as u64; rax = (0xaaaaaaaaaaaaaaabu64) as u64; rcx = ((rcx).wrapping_add(0x2u64)) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; rax = (((rcx) as i64).wrapping_mul((0x7cu64) as i64) as u64) as u64; rdx = ((rax.wrapping_add(r8)) as u64) as u64; bb = 7; continue; }
            6 => { rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x1f0u64))) as u64; rax = (0xaaaaaaaaaaaaaaabu64) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; rdx = (((rcx) as i64).wrapping_mul((0x7cu64) as i64) as u64) as u64; fl_l = ((rdx).wrapping_add(r8)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 7; continue; }
            7 => { xmm7 = ((crate::simd_ops::ru32(P2)) as u32) as u128; rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x40u64))) as u64; xmm2 = (crate::simd_ops::ru128(r15.wrapping_add(0x20u64))) as u128; xmm4 = ((crate::simd_ops::ru64(r15.wrapping_add(0x30u64))) as u64) as u128; xmm5 = ((crate::simd_ops::ru64(r15.wrapping_add(0x48u64))) as u64) as u128; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x204u64))) as u32) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128); xmm3 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(crate::simd_ops::ru32(P3))).to_bits() as u128); rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x205u64))) as u32) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm2) as u128); xmm0 = 0u128; xmm1 = (xmm0 & !0xffff_ffffu128) | (xmm4 & 0xffff_ffffu128); xmm2 = crate::simd_ops::vshufps(xmm2, xmm2, 0x55u8); xmm2 = (xmm0 & !0xffff_ffffu128) | (xmm2 & 0xffff_ffffu128); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(xmm3 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm3 as u32) }).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(xmm3 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm3 as u32) }).to_bits() as u128); xmm3 = (crate::simd_ops::ru128(r15.wrapping_add(0x38u64))) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) < f32::from_bits(xmm7 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm7 as u32) }).to_bits() as u128); xmm0 = crate::simd_ops::vinsertps(xmm0, xmm2, 0x10u8); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm0) as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) < f32::from_bits(xmm7 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm7 as u32) }).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | (xmm1 & 0xffff_ffffu128); xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm3) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm4 as u64)) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), ((xmm4 as u64)) as u64); xmm4 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(crate::simd_ops::ru32(P3))).to_bits() as u128); xmm0 = 0u128; xmm1 = (xmm0 & !0xffff_ffffu128) | (xmm5 & 0xffff_ffffu128); xmm3 = crate::simd_ops::vshufps(xmm3, xmm3, 0x55u8); xmm2 = (xmm0 & !0xffff_ffffu128) | (xmm3 & 0xffff_ffffu128); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(xmm4 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm4 as u32) }).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(xmm4 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm4 as u32) }).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) < f32::from_bits(xmm7 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm7 as u32) }).to_bits() as u128); xmm0 = crate::simd_ops::vinsertps(xmm0, xmm2, 0x10u8); xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) < f32::from_bits(xmm7 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm7 as u32) }).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | (xmm1 & 0xffff_ffffu128); rax = ((rdi.wrapping_sub(0x95u64)) as u32) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x78u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), ((xmm5 as u64)) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), ((xmm5 as u64)) as u64); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x24; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 19; continue; } bb = 8; continue; }
            8 => { r8 = P4; rax = (rax as u32 as i32 as i64) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(rax).wrapping_add(0x20d7944u64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x20d7928u64))) as u32 as u64; fl_l = ((rcx).wrapping_add(r8)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; j_tgt = rcx; bb = 0xffff_fffe; continue; }
            9 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x27u64))) as u32) as u32 as u64; xmm0 = 0u128; rax = ((rax.wrapping_mul(2).wrapping_sub(0xffu64)) as u64) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm5 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P5))).to_bits() as u128); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x28u64))) as u32) as u32 as u64; xmm0 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0xd8u64), ((xmm5 as u32)) as u32); rax = ((rax.wrapping_mul(2).wrapping_sub(0xffu64)) as u64) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm6 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P5))).to_bits() as u128); xmm1 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm0 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P6)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xdcu64), ((xmm6 as u32)) as u32); if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { xmm1 = (xmm2) as u128; xmm4 = crate::simd_ops::vrsqrtss(xmm1, (xmm2 as u32)); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm1 = ((crate::simd_ops::ru32(P7)) as u32) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm3 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm1 = ((crate::simd_ops::ru32(P8)) as u32) as u128; xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm0 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm1 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0xd8u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0xdcu64), ((xmm1 as u32)) as u32); bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; r8 = rsp.wrapping_add(0x60); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 20; continue; }
            12 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x29u64))) as u32) as u32 as u64; xmm0 = 0u128; rax = ((rax.wrapping_mul(2).wrapping_sub(0xffu64)) as u64) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm5 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P5))).to_bits() as u128); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x2au64))) as u32) as u32 as u64; xmm0 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0xd8u64), ((xmm5 as u32)) as u32); rax = ((rax.wrapping_mul(2).wrapping_sub(0xffu64)) as u64) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm6 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P5))).to_bits() as u128); xmm1 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm0 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P6)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xdcu64), ((xmm6 as u32)) as u32); if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { xmm1 = (xmm2) as u128; xmm4 = crate::simd_ops::vrsqrtss(xmm1, (xmm2 as u32)); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm1 = ((crate::simd_ops::ru32(P7)) as u32) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm3 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm1 = ((crate::simd_ops::ru32(P8)) as u32) as u128; xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm0 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm1 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0xd8u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0xdcu64), ((xmm1 as u32)) as u32); bb = 14; continue; }
            14 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; r8 = rsp.wrapping_add(0x78); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 20; continue; }
            15 => { rdx = ((rdx).wrapping_add(0x54u64)) as u64; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 20; continue; }
            16 => { rdx = ((rdx).wrapping_add(0x5eu64)) as u64; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 20; continue; }
            17 => { rdx = ((rdx).wrapping_add(0x68u64)) as u64; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 20; continue; }
            18 => { rdx = ((rdx).wrapping_add(0x72u64)) as u64; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 20; continue; }
            19 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsi, rax); bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::lk_add_u32(r14.wrapping_add(0x14u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { rax = 0xffffffff; rcx = r14.wrapping_add(0x8); let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(r14.wrapping_add(0x10u64), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P11), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 22; continue; }
            22 => { r11 = rsp.wrapping_add(0xb0); rax = rsi; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; xmm6 = (crate::simd_ops::ru128(r11.wrapping_sub(0x10u64))) as u128; xmm7 = (crate::simd_ops::ru128(r11.wrapping_sub(0x20u64))) as u128; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x1420d7530u64) { 0x43 => { bb = 1; continue; } 0x51 => { bb = 2; continue; } 0x67 => { bb = 3; continue; } 0x74 => { bb = 4; continue; } 0x7f => { bb = 5; continue; } 0xab => { bb = 6; continue; } 0xd0 => { bb = 7; continue; } 0x1d2 => { bb = 8; continue; } 0x1f1 => { bb = 9; continue; } 0x253 => { bb = 10; continue; } 0x299 => { bb = 11; continue; } 0x2b3 => { bb = 12; continue; } 0x315 => { bb = 13; continue; } 0x35b => { bb = 14; continue; } 0x372 => { bb = 15; continue; } 0x380 => { bb = 16; continue; } 0x38e => { bb = 17; continue; } 0x39c => { bb = 18; continue; } 0x3aa => { bb = 19; continue; } 0x3af => { bb = 20; continue; } 0x3b6 => { bb = 21; continue; } 0x3c9 => { bb = 22; continue; } _ => { return crate::simd_ops::tail_stk_xmm(j_tgt, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); } } }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d7970(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645790; const P1: u64 = 0x1426455a8; const P2: u64 = 0x143426890; const P3: u64 = 0x143426eb8; const P4: u64 = 0x140000000; const P5: u64 = 0x1434264b4; const P6: u64 = 0x1434268d0; const P7: u64 = 0x143426bf8; const P8: u64 = 0x143426780; const P9: u64 = 0x1420d3d40; const P10: u64 = 0x1420d38f0; const P11: u64 = 0x142645598;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rbp, mut rsi, mut rdi, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 31>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut fl_k: u32 = 0; let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rax.wrapping_sub(0x5f); rsp = ((rsp).wrapping_sub(0xd0u64)) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x38u64), (xmm6) as u128); rbx = (((r9 as u8)) as u32) as u32 as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x48u64), (xmm7) as u128); r15 = rdx; rsi = (((r8 as u16)) as u32) as u32 as u64; r13 = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru32(r15.wrapping_add(0x10u64))) as u32 as u64; r14 = ((rax as u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = r15.wrapping_add(0x8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(r15.wrapping_add(0x10u64), ((r14 as u32)) as u32); bb = 2; continue; }
            2 => { crate::simd_ops::lk_add_u32(r15.wrapping_add(0x14u64), (1) as u32); r9 = (0xaaaaaaaaaaaaaaabu64) as u64; r8 = (crate::simd_ops::ru64(r15.wrapping_add(0x40u64))) as u64; rax = r9; rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x1f0u64))) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; rax = ((crate::simd_ops::ru8(r13.wrapping_add(0x4fd8u64))) as u32) as u32 as u64; r14 = (((rcx) as i64).wrapping_mul((0x7cu64) as i64) as u64) as u64; r14 = ((r14).wrapping_add(r8)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = 0x174; bb = 5; continue; }
            4 => { rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x1f0u64))) as u64; rax = r9; rcx = ((rcx).wrapping_add(0x2u64)) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; rax = (((rcx) as i64).wrapping_mul((0x7cu64) as i64) as u64) as u64; bb = 5; continue; }
            5 => { xmm7 = ((crate::simd_ops::ru32(P2)) as u32) as u128; rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x40u64))) as u64; r9 = ((rax.wrapping_add(r8)) as u64) as u64; xmm2 = (crate::simd_ops::ru128(r13.wrapping_add(0x20u64))) as u128; xmm4 = ((crate::simd_ops::ru64(r13.wrapping_add(0x30u64))) as u64) as u128; xmm5 = ((crate::simd_ops::ru64(r13.wrapping_add(0x48u64))) as u64) as u128; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x204u64))) as u32) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128); xmm3 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(crate::simd_ops::ru32(P3))).to_bits() as u128); rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x205u64))) as u32) as u32 as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x49u64), (xmm2) as u128); xmm0 = 0u128; xmm1 = (xmm0 & !0xffff_ffffu128) | (xmm4 & 0xffff_ffffu128); xmm2 = crate::simd_ops::vshufps(xmm2, xmm2, 0x55u8); xmm2 = (xmm0 & !0xffff_ffffu128) | (xmm2 & 0xffff_ffffu128); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x49u64))) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(xmm3 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm3 as u32) }).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(xmm3 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm3 as u32) }).to_bits() as u128); xmm3 = (crate::simd_ops::ru128(r13.wrapping_add(0x38u64))) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) < f32::from_bits(xmm7 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm7 as u32) }).to_bits() as u128); xmm0 = crate::simd_ops::vinsertps(xmm0, xmm2, 0x10u8); crate::simd_ops::wu128(rbp.wrapping_sub(0x49u64), (xmm0) as u128); crate::simd_ops::wu128(rbp.wrapping_sub(0x79u64), (xmm0) as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) < f32::from_bits(xmm7 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm7 as u32) }).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | (xmm1 & 0xffff_ffffu128); xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128); crate::simd_ops::wu128(rbp.wrapping_sub(0x29u64), (xmm3) as u128); crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), ((xmm4 as u64)) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x69u64), ((xmm4 as u64)) as u64); xmm4 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(crate::simd_ops::ru32(P3))).to_bits() as u128); xmm0 = 0u128; xmm1 = (xmm0 & !0xffff_ffffu128) | (xmm5 & 0xffff_ffffu128); xmm3 = crate::simd_ops::vshufps(xmm3, xmm3, 0x55u8); xmm2 = (xmm0 & !0xffff_ffffu128) | (xmm3 & 0xffff_ffffu128); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x29u64))) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(xmm4 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm4 as u32) }).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(xmm4 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm4 as u32) }).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) < f32::from_bits(xmm7 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm7 as u32) }).to_bits() as u128); xmm0 = crate::simd_ops::vinsertps(xmm0, xmm2, 0x10u8); xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) < f32::from_bits(xmm7 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm7 as u32) }).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | (xmm1 & 0xffff_ffffu128); rax = ((rsi.wrapping_sub(0x95u64)) as u32) as u32 as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x29u64), (xmm0) as u128); crate::simd_ops::wu128(rbp.wrapping_sub(0x61u64), (xmm0) as u128); crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), ((xmm5 as u64)) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x51u64), ((xmm5 as u64)) as u64); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x24; if (((fl_l as u32)) > ((fl_r as u32))) { fl_k = 1; bb = 49; continue; } bb = 6; continue; }
            6 => { rdx = P4; rax = (rax as u32 as i32 as i64) as u64; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rax).wrapping_add(0x20d80acu64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x20d8070u64))) as u32 as u64; fl_l = ((rcx).wrapping_add(rdx)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; j_tgt = rcx; bb = 0xffff_fffe; continue; }
            7 => { rbx = ((crate::simd_ops::ru8(rsi.wrapping_add(r9).wrapping_sub(0x95u64))) as u32) as u32 as u64; rbx = (rbx & !0xffu64) | ((((((rbx as u8)) ^ (crate::simd_ops::ru8(rsi.wrapping_add(r14).wrapping_sub(0x95u64))))) as u8 as u64)); fl_l = ((((rbx as u8)) & (0x1u8))) as u8 as u64; fl_r = fl_l; rbx = (rbx & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); fl_k = 4; bb = 50; continue; }
            8 => { rcx = ((crate::simd_ops::ru8(rsi.wrapping_add(r9).wrapping_sub(0x95u64))) as u32) as u32 as u64; rcx = (rcx & !0xffu64) | ((((((rcx as u8)) ^ (crate::simd_ops::ru8(rsi.wrapping_add(r14).wrapping_sub(0x95u64))))) as u8 as u64)); fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { fl_k = 4; bb = 48; continue; } bb = 9; continue; }
            9 => { rax = ((crate::simd_ops::ru8(r9.wrapping_add(0x25u64))) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x25u64))) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; fl_k = 2; bb = 31; continue; }
            10 => { rcx = ((crate::simd_ops::ru8(rsi.wrapping_add(r9).wrapping_sub(0x95u64))) as u32) as u32 as u64; rcx = (rcx & !0xffu64) | ((((((rcx as u8)) ^ (crate::simd_ops::ru8(rsi.wrapping_add(r14).wrapping_sub(0x95u64))))) as u8 as u64)); fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { fl_k = 4; bb = 48; continue; } bb = 11; continue; }
            11 => { rax = ((crate::simd_ops::ru8(r9.wrapping_add(0x26u64))) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x26u64))) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; fl_k = 2; bb = 31; continue; }
            12 => { xmm7 = ((crate::simd_ops::ru32(P5)) as u32) as u128; rax = ((crate::simd_ops::ru8(r9.wrapping_add(0x27u64))) as u32) as u32 as u64; xmm0 = 0u128; rax = ((rax.wrapping_mul(2).wrapping_sub(0xffu64)) as u64) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); rax = ((crate::simd_ops::ru8(r9.wrapping_add(0x28u64))) as u32) as u32 as u64; xmm5 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm0 = 0u128; crate::simd_ops::wu32(rbp.wrapping_add(0x67u64), ((xmm5 as u32)) as u32); rax = ((rax.wrapping_mul(2).wrapping_sub(0xffu64)) as u64) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm6 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm1 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm0 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P6)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x6bu64), ((xmm6 as u32)) as u32); if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { xmm1 = (xmm2) as u128; xmm4 = crate::simd_ops::vrsqrtss(xmm1, (xmm2 as u32)); xmm3 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm1 = ((crate::simd_ops::ru32(P7)) as u32) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm1 = ((crate::simd_ops::ru32(P8)) as u32) as u128; xmm3 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm0 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm1 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x67u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x6bu64), ((xmm1 as u32)) as u32); bb = 14; continue; }
            14 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; r8 = rbp.wrapping_sub(0x79); r9 = (((rbx as u8)) as u32) as u32 as u64; rcx = rbp.wrapping_add(0x6f); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru8(r14.wrapping_add(0x27u64))) as u32) as u32 as u64; rsi = rax; xmm0 = 0u128; rcx = ((rcx.wrapping_mul(2).wrapping_sub(0xffu64)) as u64) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rcx) as i64) as f32).to_bits() as u128); rcx = ((crate::simd_ops::ru8(r14.wrapping_add(0x28u64))) as u32) as u32 as u64; xmm5 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm0 = 0u128; crate::simd_ops::wu32(rbp.wrapping_add(0x67u64), ((xmm5 as u32)) as u32); rcx = ((rcx.wrapping_mul(2).wrapping_sub(0xffu64)) as u64) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rcx) as i64) as f32).to_bits() as u128); xmm6 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm1 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm0 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P6)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x6bu64), ((xmm6 as u32)) as u32); if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 16; continue; } bb = 15; continue; }
            15 => { xmm1 = (xmm2) as u128; xmm4 = crate::simd_ops::vrsqrtss(xmm1, (xmm2 as u32)); xmm3 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm1 = ((crate::simd_ops::ru32(P7)) as u32) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm1 = ((crate::simd_ops::ru32(P8)) as u32) as u128; xmm3 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm0 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm1 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x67u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x6bu64), ((xmm1 as u32)) as u32); bb = 16; continue; }
            16 => { r8 = rbp.wrapping_sub(0x79); rcx = rbp.wrapping_sub(0x9); bb = 22; continue; }
            17 => { xmm7 = ((crate::simd_ops::ru32(P5)) as u32) as u128; rax = ((crate::simd_ops::ru8(r9.wrapping_add(0x29u64))) as u32) as u32 as u64; xmm0 = 0u128; rax = ((rax.wrapping_mul(2).wrapping_sub(0xffu64)) as u64) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); rax = ((crate::simd_ops::ru8(r9.wrapping_add(0x2au64))) as u32) as u32 as u64; xmm5 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm0 = 0u128; crate::simd_ops::wu32(rbp.wrapping_add(0x67u64), ((xmm5 as u32)) as u32); rax = ((rax.wrapping_mul(2).wrapping_sub(0xffu64)) as u64) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm6 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm1 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm0 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P6)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x6bu64), ((xmm6 as u32)) as u32); if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 19; continue; } bb = 18; continue; }
            18 => { xmm1 = (xmm2) as u128; xmm4 = crate::simd_ops::vrsqrtss(xmm1, (xmm2 as u32)); xmm3 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm1 = ((crate::simd_ops::ru32(P7)) as u32) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm1 = ((crate::simd_ops::ru32(P8)) as u32) as u128; xmm3 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm0 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm1 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x67u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x6bu64), ((xmm1 as u32)) as u32); bb = 19; continue; }
            19 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; r8 = rbp.wrapping_sub(0x61); r9 = (((rbx as u8)) as u32) as u32 as u64; rcx = rbp.wrapping_sub(0x1); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru8(r14.wrapping_add(0x29u64))) as u32) as u32 as u64; rsi = rax; xmm0 = 0u128; rcx = ((rcx.wrapping_mul(2).wrapping_sub(0xffu64)) as u64) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rcx) as i64) as f32).to_bits() as u128); rcx = ((crate::simd_ops::ru8(r14.wrapping_add(0x2au64))) as u32) as u32 as u64; xmm5 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm0 = 0u128; crate::simd_ops::wu32(rbp.wrapping_add(0x67u64), ((xmm5 as u32)) as u32); rcx = ((rcx.wrapping_mul(2).wrapping_sub(0xffu64)) as u64) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rcx) as i64) as f32).to_bits() as u128); xmm6 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm1 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm0 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P6)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x6bu64), ((xmm6 as u32)) as u32); if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 21; continue; } bb = 20; continue; }
            20 => { xmm1 = (xmm2) as u128; xmm4 = crate::simd_ops::vrsqrtss(xmm1, (xmm2 as u32)); xmm3 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm1 = ((crate::simd_ops::ru32(P7)) as u32) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm1 = ((crate::simd_ops::ru32(P8)) as u32) as u128; xmm3 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm0 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm1 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x67u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x6bu64), ((xmm1 as u32)) as u32); bb = 21; continue; }
            21 => { r8 = rbp.wrapping_sub(0x61); rcx = rbp.wrapping_add(0x7); bb = 22; continue; }
            22 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; r9 = (((rbx as u8)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = ((crate::simd_ops::ru32(rax)) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsi)) as u32 as u64; if (f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { fl_k = 3; bb = 48; continue; } bb = 23; continue; }
            23 => { if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { fl_k = 3; bb = 48; continue; } bb = 24; continue; }
            24 => { xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsi.wrapping_add(0x4u64))) as u32 as u64; bb = 30; continue; }
            25 => { rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x27u64))) as u32) as u32 as u64; rdx = rbp.wrapping_sub(0x79); xmm0 = 0u128; r8 = (((rbx as u8)) as u32) as u32 as u64; rax = ((rax.wrapping_mul(2).wrapping_sub(0xffu64)) as u64) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P5))).to_bits() as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru8(r9.wrapping_add(0x27u64))) as u32) as u32 as u64; rdx = rbp.wrapping_sub(0x79); bb = 29; continue; }
            26 => { rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x28u64))) as u32) as u32 as u64; rdx = rbp.wrapping_sub(0x6d); xmm0 = 0u128; r8 = (((rbx as u8)) as u32) as u32 as u64; rax = ((rax.wrapping_mul(2).wrapping_sub(0xffu64)) as u64) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P5))).to_bits() as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru8(r9.wrapping_add(0x28u64))) as u32) as u32 as u64; rdx = rbp.wrapping_sub(0x6d); bb = 29; continue; }
            27 => { rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x29u64))) as u32) as u32 as u64; rdx = rbp.wrapping_sub(0x61); xmm0 = 0u128; r8 = (((rbx as u8)) as u32) as u32 as u64; rax = ((rax.wrapping_mul(2).wrapping_sub(0xffu64)) as u64) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P5))).to_bits() as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru8(r9.wrapping_add(0x29u64))) as u32) as u32 as u64; rdx = rbp.wrapping_sub(0x61); bb = 29; continue; }
            28 => { rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x2au64))) as u32) as u32 as u64; rdx = rbp.wrapping_sub(0x55); xmm0 = 0u128; r8 = (((rbx as u8)) as u32) as u32 as u64; rax = ((rax.wrapping_mul(2).wrapping_sub(0xffu64)) as u64) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P5))).to_bits() as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru8(r9.wrapping_add(0x2au64))) as u32) as u32 as u64; rdx = rbp.wrapping_sub(0x55); bb = 29; continue; }
            29 => { rax = ((rax.wrapping_mul(2).wrapping_sub(0xffu64)) as u64) as u64; xmm1 = 0u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm7 = (xmm0) as u128; xmm0 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(P5))).to_bits() as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((xmm7 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; bb = 30; continue; }
            30 => { if (f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { fl_k = 3; bb = 48; continue; } fl_k = 3; bb = 31; continue; }
            31 => { if (match fl_k { 2 => (((fl_l as u8)) == ((fl_r as u8))), 3 => (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()), _ => false }) { bb = 49; continue; } bb = 32; continue; }
            32 => { rbx = (rbx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 50; continue; }
            33 => { rax = ((crate::simd_ops::ru8(r9.wrapping_add(0x54u64))) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x54u64))) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 48; continue; } bb = 34; continue; }
            34 => { rax = ((crate::simd_ops::ru16(r9.wrapping_add(0x56u64))) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru16(r14.wrapping_add(0x56u64))) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { fl_k = 0; bb = 48; continue; } bb = 35; continue; }
            35 => { rax = ((crate::simd_ops::ru16(r9.wrapping_add(0x58u64))) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru16(r14.wrapping_add(0x58u64))) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { fl_k = 0; bb = 49; continue; } bb = 36; continue; }
            36 => { rbx = (rbx & !0xffu64) | (((0x1u8) as u8 as u64)); fl_k = 0; bb = 50; continue; }
            37 => { rax = ((crate::simd_ops::ru8(r9.wrapping_add(0x5eu64))) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x5eu64))) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 48; continue; } bb = 38; continue; }
            38 => { rax = ((crate::simd_ops::ru16(r9.wrapping_add(0x60u64))) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru16(r14.wrapping_add(0x60u64))) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { fl_k = 0; bb = 48; continue; } bb = 39; continue; }
            39 => { rax = ((crate::simd_ops::ru16(r9.wrapping_add(0x62u64))) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru16(r14.wrapping_add(0x62u64))) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { fl_k = 0; bb = 49; continue; } bb = 40; continue; }
            40 => { rbx = (rbx & !0xffu64) | (((0x1u8) as u8 as u64)); fl_k = 0; bb = 50; continue; }
            41 => { rax = ((crate::simd_ops::ru8(r9.wrapping_add(0x68u64))) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x68u64))) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 48; continue; } bb = 42; continue; }
            42 => { rax = ((crate::simd_ops::ru16(r9.wrapping_add(0x6au64))) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru16(r14.wrapping_add(0x6au64))) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { fl_k = 0; bb = 48; continue; } bb = 43; continue; }
            43 => { rax = ((crate::simd_ops::ru16(r9.wrapping_add(0x6cu64))) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru16(r14.wrapping_add(0x6cu64))) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { fl_k = 0; bb = 49; continue; } bb = 44; continue; }
            44 => { rbx = (rbx & !0xffu64) | (((0x1u8) as u8 as u64)); fl_k = 0; bb = 50; continue; }
            45 => { rax = ((crate::simd_ops::ru8(r9.wrapping_add(0x72u64))) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x72u64))) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 48; continue; } bb = 46; continue; }
            46 => { rax = ((crate::simd_ops::ru16(r9.wrapping_add(0x74u64))) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru16(r14.wrapping_add(0x74u64))) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { fl_k = 0; bb = 48; continue; } bb = 47; continue; }
            47 => { rax = ((crate::simd_ops::ru16(r9.wrapping_add(0x76u64))) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru16(r14.wrapping_add(0x76u64))) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { fl_k = 0; bb = 49; continue; } fl_k = 0; bb = 48; continue; }
            48 => { rbx = (rbx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 50; continue; }
            49 => { rbx = (rbx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; fl_k = 4; bb = 50; continue; }
            50 => { fl_l = (crate::simd_ops::lk_add_u32(r15.wrapping_add(0x14u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 52; continue; } bb = 51; continue; }
            51 => { rcx = 0xffffffff; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(r15.wrapping_add(0x10u64), ((rcx as u32)) as u32); rcx = (tmp_xchg) as u32 as u64; rcx = r15.wrapping_add(0x8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P11), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 52; continue; }
            52 => { r11 = rsp.wrapping_add(0xd0); rax = (((rbx as u8)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x48u64))) as u64; xmm6 = (crate::simd_ops::ru128(r11.wrapping_sub(0x10u64))) as u128; xmm7 = (crate::simd_ops::ru128(r11.wrapping_sub(0x20u64))) as u128; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x1420d7970u64) { 0x48 => { bb = 1; continue; } 0x56 => { bb = 2; continue; } 0x93 => { bb = 3; continue; } 0x9a => { bb = 4; continue; } 0xb9 => { bb = 5; continue; } 0x1b0 => { bb = 6; continue; } 0x1cd => { bb = 7; continue; } 0x1e6 => { bb = 8; continue; } 0x200 => { bb = 9; continue; } 0x20e => { bb = 10; continue; } 0x228 => { bb = 11; continue; } 0x236 => { bb = 12; continue; } 0x292 => { bb = 13; continue; } 0x2d0 => { bb = 14; continue; } 0x33c => { bb = 15; continue; } 0x37a => { bb = 16; continue; } 0x387 => { bb = 17; continue; } 0x3e3 => { bb = 18; continue; } 0x421 => { bb = 19; continue; } 0x48d => { bb = 20; continue; } 0x4cb => { bb = 21; continue; } 0x4d3 => { bb = 22; continue; } 0x4ee => { bb = 23; continue; } 0x4f4 => { bb = 24; continue; } 0x503 => { bb = 25; continue; } 0x53c => { bb = 26; continue; } 0x572 => { bb = 27; continue; } 0x5a8 => { bb = 28; continue; } 0x5dc => { bb = 29; continue; } 0x602 => { bb = 30; continue; } 0x608 => { bb = 31; continue; } 0x60e => { bb = 32; continue; } 0x615 => { bb = 33; continue; } 0x624 => { bb = 34; continue; } 0x634 => { bb = 35; continue; } 0x640 => { bb = 36; continue; } 0x644 => { bb = 37; continue; } 0x64f => { bb = 38; continue; } 0x65b => { bb = 39; continue; } 0x667 => { bb = 40; continue; } 0x66b => { bb = 41; continue; } 0x676 => { bb = 42; continue; } 0x682 => { bb = 43; continue; } 0x68e => { bb = 44; continue; } 0x692 => { bb = 45; continue; } 0x69d => { bb = 46; continue; } 0x6a9 => { bb = 47; continue; } 0x6b5 => { bb = 48; continue; } 0x6b9 => { bb = 49; continue; } 0x6bb => { bb = 50; continue; } 0x6c2 => { bb = 51; continue; } 0x6d5 => { bb = 52; continue; } _ => { return crate::simd_ops::tail_stk_xmm(j_tgt, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); } } }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d80e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x208; const P2: u64 = 0x1417a1ff0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d8100(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x6a; const P1: u64 = 0x40; const P2: u64 = 0x69; const P3: u64 = 0x68; const P4: u64 = 0x208; const P5: u64 = 0x1417a2060;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x0u32) as u32);
    crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P2))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x31u64), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P3))) as u32) as u32 as u64;
    rdx = rsp.wrapping_add(0x30);
    crate::simd_ops::wu8(rsp.wrapping_add(0x32u64), ((rax as u8)) as u8);
    rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P4))) as u32 as u64;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d8150(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x6c; const P1: u64 = 0x6a; const P2: u64 = 0x69; const P3: u64 = 0x68;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u8 as u64; fl_r = (0xdu8) as u8 as u64; if (((fl_l as u8)) >= ((fl_r as u8))) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P2))) as u8 as u64; fl_r = (0xdu8) as u8 as u64; if (((fl_l as u8)) >= ((fl_r as u8))) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P3))) as u8 as u64; fl_r = (0xdu8) as u8 as u64; if (((fl_l as u8)) >= ((fl_r as u8))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
            5 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d8180(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411f4000; const P1: u64 = 0x1417a21b0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 43>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rbp = rsp.wrapping_sub(0x50); rsp = ((rsp).wrapping_sub(0x150u64)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x30u64))) as u32 as u64; rbx = rdx; crate::simd_ops::wu32(rbp.wrapping_sub(0x40u64), ((rax as u32)) as u32); rdi = rcx; let y_addr: u64 = rdx; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(0x20u64))) as u128; ymm1_hi = 0; let y_addr: u64 = rbp.wrapping_sub(0x70u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rbp.wrapping_sub(0x50u64), (xmm1) as u128); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x44u64), (xmm0) as u128); let y_addr: u64 = rsp.wrapping_add(0x24u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rsp.wrapping_add(0x28u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rsp.wrapping_add(0x48u64), (xmm0) as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x0u32) as u32); bb = 3; continue; }
            2 => { rax = 0; fl_l = 0; fl_r = 0; r8 = rbp.wrapping_sub(0x70); crate::simd_ops::wu32(rbp.wrapping_add(0x60u64), ((rax as u32)) as u32); rcx = rsp.wrapping_add(0x20); crate::simd_ops::wu8(rbp.wrapping_add(0x64u64), ((rax as u8)) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x64u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x40u64), ((rax as u32)) as u32); let y_addr: u64 = rbx.wrapping_add(0x34u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); xmm1 = (crate::simd_ops::ru128(rbx.wrapping_add(0x54u64))) as u128; ymm1_hi = 0; let y_addr: u64 = rbp.wrapping_sub(0x70u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rbp.wrapping_sub(0x50u64), (xmm1) as u128); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x7cu64), (xmm0) as u128); let y_addr: u64 = rsp.wrapping_add(0x5cu64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rsp.wrapping_add(0x60u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rbp.wrapping_sub(0x80u64), (xmm0) as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), (0x0u32) as u32); bb = 6; continue; }
            5 => { rax = 0; fl_l = 0; fl_r = 0; r8 = rbp.wrapping_sub(0x70); crate::simd_ops::wu32(rbp.wrapping_add(0x60u64), ((rax as u32)) as u32); rcx = rsp.wrapping_add(0x58); crate::simd_ops::wu8(rbp.wrapping_add(0x64u64), ((rax as u8)) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { let y_addr: u64 = rsp.wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; ymm1_hi = 0; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; rdx = rbp.wrapping_sub(0x30); let y_addr: u64 = rbp.wrapping_sub(0x28u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x8u64), (xmm1) as u128); let y_addr: u64 = rsp.wrapping_add(0x58u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), ((xmm0 as u64)) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x78u64))) as u128; ymm0_hi = 0; let y_addr: u64 = rbp.wrapping_add(0x10u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); xmm1 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x78u64))) as u64) as u128; ymm1_hi = 0; rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rbp.wrapping_sub(0x30u64), (0x3u8) as u8); crate::simd_ops::wu128(rbp.wrapping_add(0x30u64), (xmm0) as u128); crate::simd_ops::wu64(rbp.wrapping_add(0x40u64), ((xmm1 as u64)) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x2fu64), ((rax as u32)) as u32); crate::simd_ops::wu16(rbp.wrapping_sub(0x2bu64), ((rax as u16)) as u16); crate::simd_ops::wu8(rbp.wrapping_sub(0x29u64), ((rax as u8)) as u8); rcx = (crate::simd_ops::ru32(rcx.wrapping_add(0x208u64))) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r11 = rsp.wrapping_add(0x150); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsp = r11; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d82e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x24; const P2: u64 = 0x34; const P3: u64 = 0x8; const P4: u64 = 0xc; const P5: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    crate::simd_ops::wu32(rdx, (0x5u32) as u32);
    rax = 0;
    fl_l = 0; fl_r = 0;
    xmm0 = 0u128; ymm0_hi = 0;
    let y_addr: u64 = rdx.wrapping_add(P0); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    crate::simd_ops::wu128(rdx.wrapping_add(P1), (xmm0) as u128);
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64;
    crate::simd_ops::wu8(rsp, ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(r8.wrapping_add(P0))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x1u64), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(r8.wrapping_add(P3))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x2u64), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(r8.wrapping_add(P4))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x3u64), ((rax as u8)) as u8);
    rax = 0;
    fl_l = 0; fl_r = 0;
    let y_addr: u64 = rsp.wrapping_add(0x4u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    let y_addr: u64 = rsp; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    crate::simd_ops::wu64(rsp.wrapping_add(0x24u64), rax);
    crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), ((rax as u32)) as u32);
    rax = rdx;
    xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128;
    let y_addr: u64 = rdx.wrapping_add(P3); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    crate::simd_ops::wu128(rdx.wrapping_add(P5), (xmm1) as u128);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d8370(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x2c; const P1: u64 = 0x4; const P2: u64 = 0x24; const P3: u64 = 0x34; const P4: u64 = 0x8; const P5: u64 = 0x28; const P6: u64 = 0x1;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x38u64)) as u64; crate::simd_ops::wu32(rdx, (0x6u32) as u32); r9 = ((r8.wrapping_add(P0)) as u64) as u64; rax = 0; fl_l = 0; fl_r = 0; r10 = rdx; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rdx.wrapping_add(P1); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rdx.wrapping_add(P2), (xmm0) as u128); crate::simd_ops::wu32(rdx.wrapping_add(P3), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; crate::simd_ops::wu8(rsp, ((rax as u8)) as u8); rax = 0; fl_l = 0; fl_r = 0; let y_addr: u64 = rsp.wrapping_add(0x1u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rsp; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu64(rsp.wrapping_add(0x21u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x29u64), ((rax as u32)) as u32); crate::simd_ops::wu16(rsp.wrapping_add(0x2du64), ((rax as u16)) as u16); crate::simd_ops::wu8(rsp.wrapping_add(0x2fu64), ((rax as u8)) as u8); rax = ((r8.wrapping_add(P1)) as u64) as u64; xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; let y_addr: u64 = rdx.wrapping_add(P4); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rdx.wrapping_add(P5), (xmm1) as u128); rdx = ((rdx).wrapping_add(0x9u64)) as u64; fl_l = rax; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rcx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rax = ((rax).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu8(rdx, ((rcx as u8)) as u8); rdx = ((rdx.wrapping_add(P6)) as u64) as u64; fl_l = rax; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rax = r10; rsp = ((rsp).wrapping_add(0x38u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d8420(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x28; const P2: u64 = 0x20; const P3: u64 = 0x1;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rcx = ((rdx.wrapping_add(P0)) as u64) as u64;
    crate::simd_ops::wu64(rdx, (0x4u64) as u64);
    xmm0 = 0u128; ymm0_hi = 0;
    r9 = rdx;
    rdx = ((r8.wrapping_add(P1)) as u64) as u64;
    let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    crate::simd_ops::wu128(rcx.wrapping_add(P2), (xmm0) as u128);
    fl_l = r8; fl_r = rdx;
    if (((fl_l as u64)) == ((fl_r as u64))) { rax = r9; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    loop { rax = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; r8 = ((r8).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu8(rcx, ((rax as u8)) as u8); rcx = ((rcx.wrapping_add(P3)) as u64) as u64; fl_l = r8; fl_r = rdx; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rax = r9;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d8470(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x24; const P2: u64 = 0x34; const P3: u64 = 0x8; const P4: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    crate::simd_ops::wu32(rdx, (0x3u32) as u32);
    rax = 0;
    fl_l = 0; fl_r = 0;
    xmm0 = 0u128; ymm0_hi = 0;
    let y_addr: u64 = rdx.wrapping_add(P0); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    crate::simd_ops::wu128(rdx.wrapping_add(P1), (xmm0) as u128);
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64;
    crate::simd_ops::wu8(rsp, ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(r8.wrapping_add(P0))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x1u64), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(r8.wrapping_add(P3))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x2u64), ((rax as u8)) as u8);
    rax = 0;
    fl_l = 0; fl_r = 0;
    let y_addr: u64 = rsp.wrapping_add(0x3u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    let y_addr: u64 = rsp; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    crate::simd_ops::wu64(rsp.wrapping_add(0x23u64), rax);
    crate::simd_ops::wu32(rsp.wrapping_add(0x2bu64), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rsp.wrapping_add(0x2fu64), ((rax as u8)) as u8);
    rax = rdx;
    xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128;
    let y_addr: u64 = rdx.wrapping_add(P3); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    crate::simd_ops::wu128(rdx.wrapping_add(P4), (xmm1) as u128);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d84f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x24; const P2: u64 = 0x34; const P3: u64 = 0x8; const P4: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    crate::simd_ops::wu32(rdx, (0x2u32) as u32);
    rax = 0;
    fl_l = 0; fl_r = 0;
    xmm0 = 0u128; ymm0_hi = 0;
    let y_addr: u64 = rdx.wrapping_add(P0); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    crate::simd_ops::wu128(rdx.wrapping_add(P1), (xmm0) as u128);
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64;
    crate::simd_ops::wu8(rsp, ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(r8.wrapping_add(P0))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x1u64), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(r8.wrapping_add(P3))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x2u64), ((rax as u8)) as u8);
    rax = 0;
    fl_l = 0; fl_r = 0;
    let y_addr: u64 = rsp.wrapping_add(0x3u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    let y_addr: u64 = rsp; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    crate::simd_ops::wu64(rsp.wrapping_add(0x23u64), rax);
    crate::simd_ops::wu32(rsp.wrapping_add(0x2bu64), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rsp.wrapping_add(0x2fu64), ((rax as u8)) as u8);
    rax = rdx;
    xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128;
    let y_addr: u64 = rdx.wrapping_add(P3); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    crate::simd_ops::wu128(rdx.wrapping_add(P4), (xmm1) as u128);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d8570(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x24; const P2: u64 = 0x34; const P3: u64 = 0x8; const P4: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    xmm0 = 0u128; ymm0_hi = 0;
    crate::simd_ops::wu32(rdx, (0x1u32) as u32);
    rax = 0;
    fl_l = 0; fl_r = 0;
    let y_addr: u64 = rdx.wrapping_add(P0); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    crate::simd_ops::wu128(rdx.wrapping_add(P1), (xmm0) as u128);
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64;
    crate::simd_ops::wu8(rsp, ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(r8.wrapping_add(P0))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x1u64), ((rax as u8)) as u8);
    rax = 0;
    fl_l = 0; fl_r = 0;
    let y_addr: u64 = rsp.wrapping_add(0x2u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    let y_addr: u64 = rsp; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    crate::simd_ops::wu64(rsp.wrapping_add(0x22u64), rax);
    crate::simd_ops::wu32(rsp.wrapping_add(0x2au64), ((rax as u32)) as u32);
    crate::simd_ops::wu16(rsp.wrapping_add(0x2eu64), ((rax as u16)) as u16);
    rax = rdx;
    xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128;
    let y_addr: u64 = rdx.wrapping_add(P3); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    crate::simd_ops::wu128(rdx.wrapping_add(P4), (xmm1) as u128);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d85e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x8; const P2: u64 = 0x28; const P3: u64 = 0x1411f4000;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    rbx = rcx;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { xmm0 = 0u128; ymm0_hi = 0; rax = 0; fl_l = 0; fl_r = 0; let y_addr: u64 = rcx.wrapping_add(P1); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rcx, rax); rax = rcx; crate::simd_ops::wu128(rcx.wrapping_add(P2), (xmm0) as u128); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = 0;
    fl_l = 0; fl_r = 0;
    r8 = rdx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rsp.wrapping_add(0x34u64), ((rax as u8)) as u8);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d8660(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x310; const P2: u64 = 0x2a0; const P3: u64 = 0x6c; const P4: u64 = 0x6a; const P5: u64 = 0x69; const P6: u64 = 0x68; const P7: u64 = 0x208; const P8: u64 = 0x1417a1ff0; const P9: u64 = 0x40; const P10: u64 = 0x1417a2060; const P11: u64 = 0x1420d8180;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64; rdi = rdx; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 9; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbx); rbx = ((rdx.wrapping_add(P2)) as u64) as u64; fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P3))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P4))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xdu8) as u8 as u64; if (((fl_l as u8)) >= ((fl_r as u8))) { bb = 7; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P5))) as u8 as u64; fl_r = (0xdu8) as u8 as u64; if (((fl_l as u8)) >= ((fl_r as u8))) { bb = 7; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P6))) as u8 as u64; fl_r = (0xdu8) as u8 as u64; if (((fl_l as u8)) >= ((fl_r as u8))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = rdi; rcx = 0x40; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rcx))) as u64; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P7))) as u32 as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P9))) as u64; rdx = rsp.wrapping_add(0x30); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x0u32) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P5))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x31u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P6))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x32u64), ((rax as u8)) as u8); rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P7))) as u32 as u64; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 9; continue; }
            9 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d8710(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14397d820; const P2: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rdi = rcx;
    rcx = (0x32dfe6f72bu64) as u64;
    rbx = (((rdx as u8)) as u32) as u32 as u64;
    fl_l = rax; fl_r = rcx;
    if (((fl_l as u64)) != ((fl_r as u64))) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(P0))) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
    crate::simd_ops::wu8(rdi.wrapping_add(P0), ((rbx as u8)) as u8);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420d8760(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rdx = (0x32dfe6f72bu64) as u64;
    rax = rdx;
    let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rcx, rax);
    rax = (tmp_xchg) as u64;
    fl_l = rax; fl_r = rdx;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x1u8) as u8); }
    return rax;
}
