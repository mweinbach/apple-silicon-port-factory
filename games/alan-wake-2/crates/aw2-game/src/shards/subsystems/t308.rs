#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn fn_141c95570(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x24; const P5: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    r9 = rdx;
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu64(r9.wrapping_add(rcx), rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(rdx).wrapping_add(P0), rax);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P1), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(rdx).wrapping_add(P2), rax);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P3), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P4), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P5), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c955e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x24; const P5: u64 = 0x28;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(rdx)) as u64; r10 = rdx; rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; r9 = rdx; r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; crate::simd_ops::wu64(r9.wrapping_add(rcx), rax); r9 = ((rdx.wrapping_add(P0)) as u64) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rdx = ((rdx).wrapping_add(0x10u64)) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; crate::simd_ops::wu64(r9.wrapping_add(rcx), rax); rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru32(r10.wrapping_add(P1))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32); rdx = ((r10.wrapping_add(P2)) as u64) as u64; rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru64(r10.wrapping_add(P2))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(rcx), rax); rdx = ((r10.wrapping_add(P3)) as u64) as u64; rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru32(r10.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32); rdx = ((r10.wrapping_add(P4)) as u64) as u64; rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru32(r10.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32); rdx = ((r10.wrapping_add(P5)) as u64) as u64; fl_l = rdx; fl_r = (crate::simd_ops::ru64(r8)) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru32(r10.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c958a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx, rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P4), ((rax as u32)) as u32);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c95fb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143299cb8; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0xb; const P5: u64 = 0x20; const P6: u64 = 0x28; const P7: u64 = 0x30;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P1), (0xff000002u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P1), (0x2u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P3), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), (0x2u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P5), (0xffffffffu32) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c96720(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x14;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu8(rdx, (0x1u8) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    rax = rdx;
    crate::simd_ops::wu8(rdx.wrapping_add(P2), (0x1u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c974d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14328c370; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0xb; const P5: u64 = 0x20; const P6: u64 = 0x24; const P7: u64 = 0x28; const P8: u64 = 0x23;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P1), (0xff000002u32) as u32);
    crate::simd_ops::lk_or_u32(rdx.wrapping_add(P1), (0x2u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P3), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), (0x2u8) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P5), (0xff000002u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P5), (0x2u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P6), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P7), ((rax as u8)) as u8);
    rax = rdx;
    crate::simd_ops::wu8(rdx.wrapping_add(P8), (0x2u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c97e40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0xd; const P3: u64 = 0xe; const P4: u64 = 0xf; const P5: u64 = 0x10; const P6: u64 = 0x11;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; r9 = ((rcx.wrapping_add(P1)) as u64) as u64; r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64; r11 = rdx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; r10 = rcx; crate::simd_ops::wu32(r8.wrapping_add(rcx).wrapping_add(P0), ((rax as u32)) as u32); r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64; r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rdx = ((rcx.wrapping_add(P2)) as u64) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u32) as u32 as u64; crate::simd_ops::wu8(r9.wrapping_add(r8), ((rax as u8)) as u8); rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r11))) as u64; rcx = (crate::simd_ops::ru64(r11.wrapping_add(0x8u64))) as u64; rax = ((crate::simd_ops::ru8(r10.wrapping_add(P2))) as u32) as u32 as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8); rdx = ((r10.wrapping_add(P3)) as u64) as u64; rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r11))) as u64; rcx = (crate::simd_ops::ru64(r11.wrapping_add(0x8u64))) as u64; rax = ((crate::simd_ops::ru8(r10.wrapping_add(P3))) as u32) as u32 as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8); rdx = ((r10.wrapping_add(P4)) as u64) as u64; rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r11))) as u64; rcx = (crate::simd_ops::ru64(r11.wrapping_add(0x8u64))) as u64; rax = ((crate::simd_ops::ru8(r10.wrapping_add(P4))) as u32) as u32 as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8); rdx = ((r10.wrapping_add(P5)) as u64) as u64; rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r11))) as u64; rcx = (crate::simd_ops::ru64(r11.wrapping_add(0x8u64))) as u64; rax = ((crate::simd_ops::ru8(r10.wrapping_add(P5))) as u32) as u32 as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8); rdx = ((r10.wrapping_add(P6)) as u64) as u64; fl_l = rdx; fl_r = (crate::simd_ops::ru64(r11)) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rcx = (crate::simd_ops::ru64(r11.wrapping_add(0x8u64))) as u64; rax = ((crate::simd_ops::ru8(r10.wrapping_add(P6))) as u32) as u32 as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c981b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0xd; const P3: u64 = 0xe; const P4: u64 = 0xf; const P5: u64 = 0x10; const P6: u64 = 0x11;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; r9 = ((rdx.wrapping_add(P1)) as u64) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; r10 = rdx; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P0), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64; rdx = ((rdx).wrapping_add(0xdu64)) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; crate::simd_ops::wu8(r9.wrapping_add(rcx), ((rax as u8)) as u8); rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rax = ((crate::simd_ops::ru8(r10.wrapping_add(P2))) as u32) as u32 as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8); rdx = ((r10.wrapping_add(P3)) as u64) as u64; rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rax = ((crate::simd_ops::ru8(r10.wrapping_add(P3))) as u32) as u32 as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8); rdx = ((r10.wrapping_add(P4)) as u64) as u64; rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rax = ((crate::simd_ops::ru8(r10.wrapping_add(P4))) as u32) as u32 as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8); rdx = ((r10.wrapping_add(P5)) as u64) as u64; rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rax = ((crate::simd_ops::ru8(r10.wrapping_add(P5))) as u32) as u32 as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8); rdx = ((r10.wrapping_add(P6)) as u64) as u64; fl_l = rdx; fl_r = (crate::simd_ops::ru64(r8)) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rax = ((crate::simd_ops::ru8(r10.wrapping_add(P6))) as u32) as u32 as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c983f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0xd; const P3: u64 = 0xe; const P4: u64 = 0xf; const P5: u64 = 0x10; const P6: u64 = 0x11;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P1), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P2))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P2), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P3))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P3), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P4))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P4), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P5))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P5), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P6))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P6), ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c986a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x30; const P3: u64 = 0x1404b9990;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let y_addr: u64 = rcx.wrapping_add(P0); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    r8 = rdx;
    rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    rdx = ((rcx.wrapping_add(P2)) as u64) as u64;
    let y_addr: u64 = rax.wrapping_add(rcx).wrapping_add(P0); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    fl_l = ((rcx).wrapping_add(rdx)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    return crate::simd_ops::tail_fwd_xmm(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c989f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x1404b9990;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let y_addr: u64 = rdx.wrapping_add(P0); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64;
    rax = ((rax).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    let y_addr: u64 = rax.wrapping_add(rdx).wrapping_add(P0); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64;
    rdx = ((rdx).wrapping_add(0x30u64)) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    fl_l = ((rcx).wrapping_add(rdx)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    return crate::simd_ops::tail_fwd_xmm(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c98bd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x30; const P2: u64 = 0x1404b9990;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let y_addr: u64 = rdx.wrapping_add(P0); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    let y_addr: u64 = r8.wrapping_add(P0); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    fl_l = ((rdx).wrapping_add(0x30u64)) as u64; fl_r = fl_l;
    rdx = ((fl_l as u64)) as u64;
    rcx = ((r8.wrapping_add(P1)) as u64) as u64;
    return crate::simd_ops::tail_fwd_xmm(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c98c30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1434281d0; const P1: u64 = 0x143288680; const P2: u64 = 0x10; const P3: u64 = 0x20; const P4: u64 = 0x30; const P5: u64 = 0x38; const P6: u64 = 0x40; const P7: u64 = 0x48;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = (crate::simd_ops::ru128(P0)) as u128;
    rax = P1;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu128(rdx.wrapping_add(P2), (xmm0) as u128);
    xmm1 = 0u128;
    crate::simd_ops::wu128(rdx.wrapping_add(P3), (xmm1) as u128);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P5), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rax);
    crate::simd_ops::wu8(rdx.wrapping_add(P7), ((rax as u8)) as u8);
    rax = rdx;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c99210(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x140a35240; const P4: u64 = 0x28; const P5: u64 = 0x29; const P6: u64 = 0x2a;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = r8;
    rdi = rdx;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P1), rax);
    rax = ((rdx.wrapping_add(P2)) as u64) as u64;
    rcx = ((r8.wrapping_add(P2)) as u64) as u64;
    fl_l = rcx; fl_r = rax;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rax = ((crate::simd_ops::ru8(rdi.wrapping_add(P4))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rbx.wrapping_add(P4), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdi.wrapping_add(P5))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rbx.wrapping_add(P5), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdi.wrapping_add(P6))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rbx.wrapping_add(P6), ((rax as u8)) as u8);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c992b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1430b5488; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x2a; const P6: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rax);
    crate::simd_ops::wu8(rdx.wrapping_add(P5), ((rax as u8)) as u8);
    rax = rdx;
    crate::simd_ops::wu16(rdx.wrapping_add(P6), (0x100u16) as u16);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c99e00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1402d7860; const P2: u64 = 0x4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rdi = ((rcx.wrapping_add(P0)) as u64) as u64;
    rbx = ((rbx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    rdx = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64;
    rbx = ((rbx).wrapping_add(rdi)) as u64;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64;
    r8 = (crate::simd_ops::ru64(rbx)) as u64;
    rax = (crate::simd_ops::ru64(rdi)) as u64;
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    fl_l = r8; fl_r = rax;
    r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    loop { rcx = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(r8), ((rcx as u32)) as u32); rax = ((rax.wrapping_add(P2)) as u64) as u64; fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c9a150(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1402d7860; const P2: u64 = 0x4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rdi = ((rdx.wrapping_add(P0)) as u64) as u64;
    rbx = ((rbx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rdx = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64;
    rbx = ((rbx).wrapping_add(rdi)) as u64;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64;
    r8 = (crate::simd_ops::ru64(rbx)) as u64;
    rax = (crate::simd_ops::ru64(rdi)) as u64;
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    fl_l = r8; fl_r = rax;
    r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    loop { rcx = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(r8), ((rcx as u32)) as u32); rax = ((rax.wrapping_add(P2)) as u64) as u64; fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c9b1b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x140a34f60; const P2: u64 = 0x18; const P3: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = r8; rdi = rdx; rax = ((rdx.wrapping_add(P0)) as u64) as u64; rcx = ((r8.wrapping_add(P0)) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = (((rax) as i64).wrapping_mul((0xb8u64) as i64) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rax = ((rdi.wrapping_add(P2)) as u64) as u64; rcx = ((rbx.wrapping_add(P2)) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = (((rax) as i64).wrapping_mul((0xb8u64) as i64) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rdi = ((rdi).wrapping_add(0x28u64)) as u64; rcx = ((rbx.wrapping_add(P3)) as u64) as u64; fl_l = rcx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; r8 = (((rax) as i64).wrapping_mul((0xb8u64) as i64) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c9bfb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1432673f8; const P1: u64 = 0x2f; const P2: u64 = 0x8; const P3: u64 = 0x10; const P4: u64 = 0x18; const P5: u64 = 0x20; const P6: u64 = 0x28; const P7: u64 = 0x2d; const P8: u64 = 0x2c; const P9: u64 = 0x80; const P10: u64 = 0x98;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu8(rdx.wrapping_add(P1), (0xffu8) as u8);
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P6), ((rax as u32)) as u32);
    crate::simd_ops::wu16(rdx.wrapping_add(P7), ((rax as u16)) as u16);
    crate::simd_ops::wu8(rdx.wrapping_add(P8), ((rax as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P9), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P10), rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c9dfb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x18; const P5: u64 = 0x1c; const P6: u64 = 0x20; const P7: u64 = 0x24; const P8: u64 = 0x28; const P9: u64 = 0x2c; const P10: u64 = 0x30; const P11: u64 = 0x34;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P3), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P4), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P5), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P6), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P7))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P7), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P8))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P8), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P9))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P9), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P10))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P10), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P11))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x34u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x38u64))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(0x38u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x3cu64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x3cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x40u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x40u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x44u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x44u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x48u64))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(0x48u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x49u64))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(0x49u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4cu64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x4cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x50u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x50u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(0x54u64))) as u64) as u128; crate::simd_ops::wu64(r8.wrapping_add(0x54u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x5cu64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x5cu64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(0x60u64))) as u64) as u128; crate::simd_ops::wu64(r8.wrapping_add(0x60u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x68u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x68u64), ((rax as u32)) as u32); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c9e7b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x20; const P3: u64 = 0x14; const P4: u64 = 0x15; const P5: u64 = 0x18; const P6: u64 = 0x1c; const P7: u64 = 0x14240aa00; const P8: u64 = 0xa8; const P9: u64 = 0x130; const P10: u64 = 0x1403d79a0; const P11: u64 = 0x140;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P0), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    rcx = ((r8.wrapping_add(P2)) as u64) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    rbx = rdx;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P3))) as u32) as u32 as u64;
    rdi = r8;
    crate::simd_ops::wu8(r8.wrapping_add(P3), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P4))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P4), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P5), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64;
    rdx = ((rdx).wrapping_add(0x20u64)) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P6), ((rax as u32)) as u32);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rdx = ((rbx.wrapping_add(P8)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P8)) as u64) as u64;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rdx = ((rbx.wrapping_add(P9)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P9)) as u64) as u64;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P11))) as u32) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(0x140u64), ((rax as u8)) as u8);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c9e880(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14328a978; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x20; const P4: u64 = 0x24; const P5: u64 = 0x28; const P6: u64 = 0x23; const P7: u64 = 0xa8; const P8: u64 = 0xac; const P9: u64 = 0xb0; const P10: u64 = 0xab; const P11: u64 = 0x130;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P3), (0xff000010u32) as u32);
    crate::simd_ops::lk_or_u32(rdx.wrapping_add(P3), (0x10u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P4), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P5), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P6), (0x10u8) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P7), (0xff000010u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P7), (0x10u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P8), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P9), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P10), (0x10u8) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P11), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(0x138u64), rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ca0390(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x14;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    r9 = rcx;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(rcx).wrapping_add(P0), rax);
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    crate::simd_ops::wu8(r8.wrapping_add(rcx).wrapping_add(P1), ((rax as u8)) as u8);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(rdx)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = (crate::simd_ops::ru32(r9.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(P2), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ca0750(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x14;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(rdx).wrapping_add(P0), rax);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(rdx).wrapping_add(P1), ((rax as u8)) as u8);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P2), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ca0940(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x14;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rax);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P1), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ca1080(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x140a34aa0; const P3: u64 = 0x20; const P4: u64 = 0x140265ba0; const P5: u64 = 0x30; const P6: u64 = 0x1402bf220; const P7: u64 = 0x40; const P8: u64 = 0x1403d79a0; const P9: u64 = 0x50; const P10: u64 = 0x60; const P11: u64 = 0x70; const P12: u64 = 0x1409f8d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = r8; rdi = rdx; rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; crate::simd_ops::wu64(r8.wrapping_add(P0), rax); rax = ((rdx.wrapping_add(P1)) as u64) as u64; rcx = ((r8.wrapping_add(P1)) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rdx = ((rdi.wrapping_add(P3)) as u64) as u64; rcx = ((rbx.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = ((rdi.wrapping_add(P5)) as u64) as u64; rcx = ((rbx.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdx = ((rdi.wrapping_add(P7)) as u64) as u64; rcx = ((rbx.wrapping_add(P7)) as u64) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rdx = ((rdi.wrapping_add(P9)) as u64) as u64; rcx = ((rbx.wrapping_add(P9)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = ((rdi.wrapping_add(P10)) as u64) as u64; rcx = ((rbx.wrapping_add(P10)) as u64) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rdx = ((rdi.wrapping_add(P11)) as u64) as u64; rcx = rbx.wrapping_add(0x70); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = rdi.wrapping_add(0x80); rcx = rbx.wrapping_add(0x80); rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rdi = ((rdi).wrapping_add(0x90u64)) as u64; rcx = rbx.wrapping_add(0x90); fl_l = rcx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ca1190(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143296fa8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x28; const P5: u64 = 0x30; const P6: u64 = 0x38; const P7: u64 = 0x40; const P8: u64 = 0x48; const P9: u64 = 0x50; const P10: u64 = 0x58; const P11: u64 = 0x60;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P8), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P9), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P10), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P11), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(0x68u64), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(0x70u64), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(0x78u64), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(0x80u64), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(0x88u64), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(0x90u64), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(0x98u64), rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ca18e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x50; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0x18; const P5: u64 = 0x1c; const P6: u64 = 0x20; const P7: u64 = 0x24; const P8: u64 = 0x2c; const P9: u64 = 0x30; const P10: u64 = 0x38; const P11: u64 = 0x3c; const P12: u64 = 0x1409f8bb0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; rcx = ((r8.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; crate::simd_ops::wu64(r8.wrapping_add(P3), rax); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P4), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P5), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P6))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(P6), ((rax as u8)) as u8); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P7))) as u64) as u128; crate::simd_ops::wu64(r8.wrapping_add(P7), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P8))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P8), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P9))) as u64) as u128; crate::simd_ops::wu64(r8.wrapping_add(P9), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P10))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P10), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P11))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x3cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x40u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x40u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x44u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x44u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x48u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x48u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x4cu64))) as u32) as u32 as u64; fl_l = ((rdx).wrapping_add(0x50u64)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; crate::simd_ops::wu8(r8.wrapping_add(0x4cu64), ((rax as u8)) as u8); return crate::simd_ops::tail_fwd_xmm(P12, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ca2c90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x18; const P5: u64 = 0x1c; const P6: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    r9 = rcx;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(rcx).wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(rcx).wrapping_add(P1), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    rax = (crate::simd_ops::ru32(r9.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(P2), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    rax = (crate::simd_ops::ru32(r9.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(P3), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    rax = (crate::simd_ops::ru32(r9.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(P4), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    rax = (crate::simd_ops::ru32(r9.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(P5), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(rdx)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = (crate::simd_ops::ru32(r9.wrapping_add(P6))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(P6), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ca3000(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x18; const P5: u64 = 0x1c; const P6: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P0), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P1), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P2), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P3), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P4), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P5), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P6), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ca3220(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x18; const P5: u64 = 0x1c; const P6: u64 = 0x20;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P3), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P4), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P5), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P6), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ca49f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1432a3930; const P1: u64 = 0x1c; const P2: u64 = 0x3; const P3: u64 = 0x1401de410;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rax = rdx;
    r9 = P0;
    rdx = ((rcx.wrapping_add(P1)) as u64) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx);
    crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x1u32) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    r8 = ((rcx.wrapping_add(P2)) as u32) as u32 as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rcx = rax;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ca4ad0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1c; const P2: u64 = 0x10; const P3: u64 = 0x18;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r9 = ((rcx.wrapping_add(P1)) as u64) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    r11 = rdx;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    r10 = rcx;
    crate::simd_ops::wu64(r8.wrapping_add(rcx).wrapping_add(P0), rax);
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rdx = ((rcx.wrapping_add(P2)) as u64) as u64;
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r9.wrapping_add(r8), ((rax as u32)) as u32);
    rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r11))) as u64;
    rcx = (crate::simd_ops::ru64(r11.wrapping_add(0x8u64))) as u64;
    rax = (crate::simd_ops::ru64(r10.wrapping_add(P2))) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(rcx), rax);
    rdx = ((r10.wrapping_add(P3)) as u64) as u64;
    fl_l = rdx; fl_r = (crate::simd_ops::ru64(r11)) as u64;
    rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rcx = (crate::simd_ops::ru64(r11.wrapping_add(0x8u64))) as u64;
    rax = (crate::simd_ops::ru32(r10.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ca4bc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1432a3930; const P1: u64 = 0x3; const P2: u64 = 0x1401de410;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    r9 = P0;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx);
    rax = r8;
    crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x1u32) as u32);
    rdx = ((rdx).wrapping_add(0x1cu64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    r8 = ((rcx.wrapping_add(P1)) as u32) as u32 as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rcx = rax;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ca4e70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1c; const P2: u64 = 0x10; const P3: u64 = 0x18;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r9 = ((rdx.wrapping_add(P1)) as u64) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    r10 = rdx;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(rdx).wrapping_add(P0), rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    rdx = ((rdx).wrapping_add(0x10u64)) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu32(r9.wrapping_add(rcx), ((rax as u32)) as u32);
    rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru64(r10.wrapping_add(P2))) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(rcx), rax);
    rdx = ((r10.wrapping_add(P3)) as u64) as u64;
    fl_l = rdx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru32(r10.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ca5080(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x1c; const P4: u64 = 0x20;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P1), rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P3), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P4), rax);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ca50e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14329bc30; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x1c; const P4: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P3), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ca5300(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x20; const P3: u64 = 0x24; const P4: u64 = 0x14;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r10 = rdx;
    rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    r9 = rcx;
    xmm0 = ((crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64) as u128;
    r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    rdx = ((rcx.wrapping_add(P2)) as u64) as u64;
    crate::simd_ops::wu64(rax.wrapping_add(rcx).wrapping_add(P0), ((xmm0 as u64)) as u64);
    crate::simd_ops::wu32(rax.wrapping_add(rcx).wrapping_add(P1), ((r8 as u32)) as u32);
    rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r10))) as u64;
    rax = (crate::simd_ops::ru32(r9.wrapping_add(P2))) as u32 as u64;
    rcx = (crate::simd_ops::ru64(r10.wrapping_add(P0))) as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32);
    rdx = ((r9.wrapping_add(P3)) as u64) as u64;
    rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r10))) as u64;
    rcx = (crate::simd_ops::ru64(r10.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru32(r9.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32);
    rdx = ((r9.wrapping_add(P4)) as u64) as u64;
    xmm0 = ((crate::simd_ops::ru64(rdx)) as u64) as u128;
    rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    fl_l = rdx; fl_r = (crate::simd_ops::ru64(r10)) as u64;
    rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = (crate::simd_ops::ru64(r10.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(rax), ((xmm0 as u64)) as u64);
    crate::simd_ops::wu32(rdx.wrapping_add(rax).wrapping_add(P0), ((rcx as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ca5660(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x20; const P3: u64 = 0x24; const P4: u64 = 0x14;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64) as u128;
    rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    r9 = rdx;
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rax = ((rax).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu64(rax.wrapping_add(rdx).wrapping_add(P0), ((xmm0 as u64)) as u64);
    crate::simd_ops::wu32(rax.wrapping_add(rdx).wrapping_add(P1), ((rcx as u32)) as u32);
    rdx = ((rdx).wrapping_add(0x20u64)) as u64;
    rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru32(r9.wrapping_add(P2))) as u32 as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32);
    rdx = ((r9.wrapping_add(P3)) as u64) as u64;
    rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru32(r9.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32);
    rdx = ((r9.wrapping_add(P4)) as u64) as u64;
    xmm0 = ((crate::simd_ops::ru64(rdx)) as u64) as u128;
    rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    fl_l = rdx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(rax), ((xmm0 as u64)) as u64);
    crate::simd_ops::wu32(rdx.wrapping_add(rax).wrapping_add(P0), ((rcx as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ca5880(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x14; const P3: u64 = 0x1c; const P4: u64 = 0x20; const P5: u64 = 0x24;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P0), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P2), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P3), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P4), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P5), ((rax as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ca6000(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x40; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0x14; const P5: u64 = 0x18; const P6: u64 = 0x1c; const P7: u64 = 0x20; const P8: u64 = 0x24; const P9: u64 = 0x28; const P10: u64 = 0x2c; const P11: u64 = 0x30; const P12: u64 = 0x140265ba0;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; rcx = ((r8.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32); rbx = rdx; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; rdi = r8; crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P3), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P4), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P5), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P6), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P7))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P7), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P8))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P8), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P9))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P9), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P10))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P10), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P11))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x30u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x34u64))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(0x34u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x35u64))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(0x35u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x36u64))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(0x36u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x37u64))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(0x37u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x38u64))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(0x38u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x39u64))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(0x39u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x3au64))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(0x3au64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x3bu64))) as u32) as u32 as u64; rdx = ((rdx).wrapping_add(0x40u64)) as u64; crate::simd_ops::wu8(r8.wrapping_add(0x3bu64), ((rax as u8)) as u8); rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x50u64), rax); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x58u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x58u64), ((rax as u32)) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ca6100(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x9; const P1: u64 = 0x1432856e0; const P2: u64 = 0x30; const P3: u64 = 0x8; const P4: u64 = 0xb; const P5: u64 = 0x40; const P6: u64 = 0x48; const P7: u64 = 0x50;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::wu16(rdx.wrapping_add(P0), (0x0u16) as u16);
    rax = P1;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P3), (0x0u8) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), (0xffu8) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ca6ec0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P0))) as u128;
    crate::simd_ops::wu128(r8.wrapping_add(P0), (xmm0) as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ca7120(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0x14;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r9 = ((rcx.wrapping_add(P1)) as u64) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    r11 = rdx;
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    r10 = rcx;
    crate::simd_ops::wu32(r8.wrapping_add(rcx).wrapping_add(P0), ((rax as u32)) as u32);
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rdx = ((rcx.wrapping_add(P2)) as u64) as u64;
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r9.wrapping_add(r8), ((rax as u32)) as u32);
    rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r11))) as u64;
    rcx = (crate::simd_ops::ru64(r11.wrapping_add(0x8u64))) as u64;
    rax = (crate::simd_ops::ru32(r10.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32);
    rdx = ((r10.wrapping_add(P3)) as u64) as u64;
    fl_l = rdx; fl_r = (crate::simd_ops::ru64(r11)) as u64;
    rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rcx = (crate::simd_ops::ru64(r11.wrapping_add(0x8u64))) as u64;
    rax = (crate::simd_ops::ru64(r10.wrapping_add(P3))) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(rcx), rax);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ca7470(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0x14;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    r9 = ((rdx.wrapping_add(P1)) as u64) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    r10 = rdx;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    rdx = ((rdx).wrapping_add(0x10u64)) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu32(r9.wrapping_add(rcx), ((rax as u32)) as u32);
    rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru32(r10.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32);
    rdx = ((r10.wrapping_add(P3)) as u64) as u64;
    fl_l = rdx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru64(r10.wrapping_add(P3))) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(rcx), rax);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ca7670(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0x14;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P3), rax);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ca7910(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    r9 = rcx;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(rcx).wrapping_add(P0), rax);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(rdx)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = ((crate::simd_ops::ru8(r9.wrapping_add(P1))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(r9).wrapping_add(P1), ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ca7c60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(rdx).wrapping_add(P0), rax);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(rdx).wrapping_add(P1), ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141caa690(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x11; const P3: u64 = 0x12; const P4: u64 = 0x13; const P5: u64 = 0x14; const P6: u64 = 0x18; const P7: u64 = 0x1c; const P8: u64 = 0x20; const P9: u64 = 0x28; const P10: u64 = 0x2c;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rax);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P1), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P2))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P2), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P3))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P3), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P4))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P4), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P5), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P6), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P7))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P7), ((rax as u32)) as u32);
    xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P8))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P8), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P9))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P9), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P10))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P10), ((rax as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141caa940(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rdx = ((rcx.wrapping_add(P0)) as u64) as u64;
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rbx))) as u64;
    rcx = ((rcx).wrapping_add(rdx)) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = ((crate::exe::tls_pointer() as u64)) as u64;
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    r8 = 0xa81a4;
    r8 = ((r8).wrapping_add(crate::simd_ops::ru64(rax))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rbx))) as u64;
    rax = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(r8), ((rax as u8)) as u8);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141caac70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rbx = r8;
    rcx = ((rcx).wrapping_add(rdx)) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = ((crate::exe::tls_pointer() as u64)) as u64;
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    rdx = 0xa81a4;
    rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rax))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rbx))) as u64;
    rax = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(rdx), ((rax as u8)) as u8);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cac4e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x140a34580; const P3: u64 = 0x20; const P4: u64 = 0x1403d79a0; const P5: u64 = 0x30; const P6: u64 = 0x40; const P7: u64 = 0x50; const P8: u64 = 0x140a34820; const P9: u64 = 0x60; const P10: u64 = 0x70; const P11: u64 = 0x14240aa00;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = r8; rdi = rdx; rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; crate::simd_ops::wu64(r8.wrapping_add(P0), rax); rax = ((rdx.wrapping_add(P1)) as u64) as u64; rcx = ((r8.wrapping_add(P1)) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rdx = ((rdi.wrapping_add(P3)) as u64) as u64; rcx = ((rbx.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = ((rdi.wrapping_add(P5)) as u64) as u64; rcx = ((rbx.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = ((rdi.wrapping_add(P6)) as u64) as u64; rcx = ((rbx.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = ((rdi.wrapping_add(P7)) as u64) as u64; rcx = ((rbx.wrapping_add(P7)) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = (((rax) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rax = ((rdi.wrapping_add(P9)) as u64) as u64; rcx = ((rbx.wrapping_add(P9)) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = (((rax) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rdx = ((rdi.wrapping_add(P10)) as u64) as u64; rcx = ((rbx.wrapping_add(P10)) as u64) as u64; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x80u64), rax); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cac5e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143295d18; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x28; const P6: u64 = 0x30; const P7: u64 = 0x38; const P8: u64 = 0x40; const P9: u64 = 0x48; const P10: u64 = 0x50; const P11: u64 = 0x58;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P8), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P9), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P10), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P11), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(0x60u64), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(0x68u64), rax);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(0x70u64), (0xff000001u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(0x70u64), (0x1u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(0x74u64), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(0x78u64), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(0x73u64), (0x1u8) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(0x80u64), rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cad1c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x14; const P3: u64 = 0x18; const P4: u64 = 0x1c; const P5: u64 = 0x1d;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    r9 = rcx;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(rcx).wrapping_add(P0), rax);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(rcx).wrapping_add(P1), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    rax = (crate::simd_ops::ru32(r9.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(P2), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    rax = (crate::simd_ops::ru32(r9.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(P3), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    rax = ((crate::simd_ops::ru8(r9.wrapping_add(P4))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(r9).wrapping_add(P4), ((rax as u8)) as u8);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(rdx)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = ((crate::simd_ops::ru8(r9.wrapping_add(P5))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(r9).wrapping_add(P5), ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cad570(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x14; const P3: u64 = 0x18; const P4: u64 = 0x1c; const P5: u64 = 0x1d;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(rdx).wrapping_add(P0), rax);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P1), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P2), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P3), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P4))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(rdx).wrapping_add(P4), ((rax as u8)) as u8);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P5))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(rdx).wrapping_add(P5), ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cad780(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x14; const P3: u64 = 0x18; const P4: u64 = 0x1c; const P5: u64 = 0x1d;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P3), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P4))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P4), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P5))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P5), ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141caf620(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x20; const P3: u64 = 0x28; const P4: u64 = 0x2c; const P5: u64 = 0x34; const P6: u64 = 0x38; const P7: u64 = 0x40; const P8: u64 = 0x44; const P9: u64 = 0x4c; const P10: u64 = 0x50; const P11: u64 = 0x54;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P1), rax);
    xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P2), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P3), ((rax as u32)) as u32);
    xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P4), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P5), ((rax as u32)) as u32);
    xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P6))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P6), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P7))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P7), ((rax as u32)) as u32);
    xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P8))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P8), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P9))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P9), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P10))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P10), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P11))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(0x54u64), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x58u64))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(0x58u64), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x5cu64))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(0x5cu64), ((rax as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141caf6d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143427660; const P1: u64 = 0x143292bb8; const P2: u64 = 0x8; const P3: u64 = 0x10; const P4: u64 = 0x20; const P5: u64 = 0x28; const P6: u64 = 0x2c; const P7: u64 = 0x44; const P8: u64 = 0x38; const P9: u64 = 0x4c; const P10: u64 = 0x34; const P11: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    xmm1 = (crate::simd_ops::ru128(P0)) as u128;
    xmm0 = 0u128;
    crate::simd_ops::wu32(rsp.wrapping_add(0xcu64), ((xmm0 as u32)) as u32);
    crate::simd_ops::wu128(rsp.wrapping_add(0x10u64), (xmm1) as u128);
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm1) as u128);
    rax = P1;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), (0x3f800000u64) as u64);
    crate::simd_ops::wu32(rdx.wrapping_add(P5), ((rax as u32)) as u32);
    rax = rdx;
    xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0xcu64))) as u64) as u128;
    crate::simd_ops::wu64(rdx.wrapping_add(P6), ((xmm0 as u64)) as u64);
    xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x24u64))) as u64) as u128;
    xmm2 = (xmm1) as u128;
    crate::simd_ops::wu64(rdx.wrapping_add(P7), ((xmm0 as u64)) as u64);
    crate::simd_ops::wu64(rdx.wrapping_add(P8), (((xmm1 >> 64) as u64)) as u64);
    crate::simd_ops::wu32(rdx.wrapping_add(P9), (((xmm1 >> 96) as u32) as u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P10), (((xmm1 >> 32) as u32) as u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P11), ((xmm2 as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cafe30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0xc; const P3: u64 = 0x1401e84a0; const P4: u64 = 0x28; const P5: u64 = 0x40; const P6: u64 = 0x50; const P7: u64 = 0x60; const P8: u64 = 0x68; const P9: u64 = 0x6c; const P10: u64 = 0x74; const P11: u64 = 0x78;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; rcx = ((r8.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32); rbx = rdx; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P2))) as u32) as u32 as u64; rdi = r8; rdx = ((rdx).wrapping_add(0x10u64)) as u64; crate::simd_ops::wu8(r8.wrapping_add(P2), ((rax as u8)) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = ((rbx.wrapping_add(P4)) as u64) as u64; rcx = ((rdi.wrapping_add(P4)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(P5))) as u128; crate::simd_ops::wu128(rdi.wrapping_add(P5), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64) as u128; crate::simd_ops::wu64(rdi.wrapping_add(P6), ((xmm1 as u64)) as u64); xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(P7))) as u64) as u128; crate::simd_ops::wu64(rdi.wrapping_add(P7), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P8))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P8), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(P9))) as u64) as u128; crate::simd_ops::wu64(rdi.wrapping_add(P9), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P10))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P10), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(P11))) as u64) as u128; crate::simd_ops::wu64(rdi.wrapping_add(0x78u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x80u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x80u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x84u64))) as u64) as u128; crate::simd_ops::wu64(rdi.wrapping_add(0x84u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8cu64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x8cu64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x90u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x90u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x91u64))) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x91u64), ((rax as u8)) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cb0640(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143283068; const P1: u64 = 0x8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = P0;
    crate::simd_ops::wu8(rdx.wrapping_add(P1), (0x0u8) as u8);
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cb08a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0xd; const P3: u64 = 0xe; const P4: u64 = 0xf;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    r9 = rcx;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(rcx).wrapping_add(P0), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    crate::simd_ops::wu8(r8.wrapping_add(rcx).wrapping_add(P1), ((rax as u8)) as u8);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    rax = ((crate::simd_ops::ru8(r9.wrapping_add(P2))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(r9).wrapping_add(P2), ((rax as u8)) as u8);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    rax = ((crate::simd_ops::ru8(r9.wrapping_add(P3))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(r9).wrapping_add(P3), ((rax as u8)) as u8);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(rdx)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = ((crate::simd_ops::ru8(r9.wrapping_add(P4))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(r9).wrapping_add(P4), ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cb0c30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0xd; const P3: u64 = 0xe; const P4: u64 = 0xf;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P0), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(rdx).wrapping_add(P1), ((rax as u8)) as u8);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P2))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(rdx).wrapping_add(P2), ((rax as u8)) as u8);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P3))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(rdx).wrapping_add(P3), ((rax as u8)) as u8);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P4))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(rdx).wrapping_add(P4), ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cb0e40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0xd; const P3: u64 = 0xe; const P4: u64 = 0xf;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P1), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P2))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P2), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P3))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P3), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P4))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P4), ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cb15a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x14240aa00; const P3: u64 = 0x3c; const P4: u64 = 0x40; const P5: u64 = 0x44; const P6: u64 = 0x4c; const P7: u64 = 0x50; const P8: u64 = 0x58; const P9: u64 = 0x60; const P10: u64 = 0x68; const P11: u64 = 0x70;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    rcx = ((r8.wrapping_add(P1)) as u64) as u64;
    rbx = rdx;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rdx = ((rdx).wrapping_add(0xcu64)) as u64;
    rdi = r8;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P3), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P4), ((rax as u32)) as u32);
    xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64) as u128;
    crate::simd_ops::wu64(rdi.wrapping_add(P5), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P7))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P7), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P8))) as u64;
    crate::simd_ops::wu64(rdi.wrapping_add(P8), rax);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P9))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P9), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P10))) as u64;
    crate::simd_ops::wu64(rdi.wrapping_add(P10), rax);
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P11))) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu64(rdi.wrapping_add(0x70u64), rax);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cb1650(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14328d2e8; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0x14; const P5: u64 = 0xf; const P6: u64 = 0x44; const P7: u64 = 0x4c; const P8: u64 = 0x58; const P9: u64 = 0x60; const P10: u64 = 0x68; const P11: u64 = 0x70;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P2), (0xff000005u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P2), (0x5u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P3), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P5), (0x5u8) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P7), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P8), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P9), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P10), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P11), rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cb1db0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143283380; const P1: u64 = 0x8; const P2: u64 = 0xb; const P3: u64 = 0xc; const P4: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P1), (0xff000005u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P1), (0x5u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu8(rdx.wrapping_add(P2), (0x5u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P3), (0x0u32) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), (0x0u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cb2df0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x140a34330;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = ((rdx.wrapping_add(P0)) as u64) as u64;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    fl_l = rcx; fl_r = rax;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(5u32)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cb3550(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x140a340f0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rax);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P1), ((rax as u8)) as u8);
    rax = ((rdx.wrapping_add(P2)) as u64) as u64;
    rcx = ((r8.wrapping_add(P2)) as u64) as u64;
    fl_l = rcx; fl_r = rax;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = (((rax) as i64).wrapping_mul((0x98u64) as i64) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cb58f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143297570; const P1: u64 = 0x143159a38; const P2: u64 = 0x8; const P3: u64 = 0x143159a40; const P4: u64 = 0x143427178; const P5: u64 = 0x10; const P6: u64 = 0x14; const P7: u64 = 0x18; const P8: u64 = 0x1c;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    xmm0 = ((crate::simd_ops::ru64(P1)) as u64) as u128;
    crate::simd_ops::wu64(rdx.wrapping_add(P2), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(P3)) as u32 as u64;
    xmm0 = ((crate::simd_ops::ru32(P4)) as u32) as u128;
    crate::simd_ops::wu32(rdx.wrapping_add(P5), ((rax as u32)) as u32);
    rax = rdx;
    crate::simd_ops::wu32(rdx.wrapping_add(P6), ((xmm0 as u32)) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P7), ((xmm0 as u32)) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P8), ((xmm0 as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cb6140(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x24; const P5: u64 = 0x2c; const P6: u64 = 0x30; const P7: u64 = 0x34; const P8: u64 = 0x38; const P9: u64 = 0x40;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32);
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P3))) as u128;
    crate::simd_ops::wu128(r8.wrapping_add(P3), (xmm0) as u128);
    xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P4), ((xmm1 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P5), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P6), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P7))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P7), ((rax as u32)) as u32);
    xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P8))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P8), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P9))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P9), ((rax as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cb61d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xd; const P1: u64 = 0x14328df48; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0xf; const P5: u64 = 0x143159a38; const P6: u64 = 0x14; const P7: u64 = 0x143159a40; const P8: u64 = 0x143427178; const P9: u64 = 0x1c; const P10: u64 = 0x20; const P11: u64 = 0x24;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu16(rdx.wrapping_add(P0), (0x0u16) as u16);
    rax = P1;
    crate::simd_ops::wu8(rdx.wrapping_add(P2), (0x0u8) as u8);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rdx.wrapping_add(P3), ((rcx as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), (0xffu8) as u8);
    crate::simd_ops::wu64(rdx, rax);
    xmm0 = ((crate::simd_ops::ru64(P5)) as u64) as u128;
    crate::simd_ops::wu64(rdx.wrapping_add(P6), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(P7)) as u32 as u64;
    xmm0 = ((crate::simd_ops::ru32(P8)) as u32) as u128;
    crate::simd_ops::wu32(rdx.wrapping_add(P9), ((rax as u32)) as u32);
    rax = rdx;
    crate::simd_ops::wu32(rdx.wrapping_add(P10), ((xmm0 as u32)) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P11), ((xmm0 as u32)) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(0x28u64), ((xmm0 as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(0x38u64), rcx);
    crate::simd_ops::wu32(rdx.wrapping_add(0x40u64), ((rcx as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cb6c10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x24; const P4: u64 = 0x1c; const P5: u64 = 0x28; const P6: u64 = 0x2c;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    r10 = rcx;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r9 = rdx;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(rcx).wrapping_add(P0), rax);
    xmm0 = ((crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64) as u128;
    rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64;
    rax = (crate::simd_ops::ru64(r9.wrapping_add(P0))) as u64;
    rax = ((rax).wrapping_sub(crate::simd_ops::ru64(r9))) as u64;
    crate::simd_ops::wu64(rax.wrapping_add(rcx).wrapping_add(P1), ((xmm0 as u64)) as u64);
    crate::simd_ops::wu32(rax.wrapping_add(rcx).wrapping_add(P2), ((rdx as u32)) as u32);
    rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P3))) as u32 as u64;
    rax = (crate::simd_ops::ru64(r9.wrapping_add(P0))) as u64;
    rax = ((rax).wrapping_sub(crate::simd_ops::ru64(r9))) as u64;
    xmm0 = ((crate::simd_ops::ru64(r10.wrapping_add(P4))) as u64) as u128;
    crate::simd_ops::wu64(rax.wrapping_add(r10).wrapping_add(P4), ((xmm0 as u64)) as u64);
    crate::simd_ops::wu32(rax.wrapping_add(r10).wrapping_add(P3), ((rcx as u32)) as u32);
    rcx = (crate::simd_ops::ru64(r9.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r9))) as u64;
    rax = ((crate::simd_ops::ru8(r10.wrapping_add(P5))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(r10).wrapping_add(P5), ((rax as u8)) as u8);
    rcx = (crate::simd_ops::ru64(r9.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(r9)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = (crate::simd_ops::ru32(r10.wrapping_add(P6))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(r10).wrapping_add(P6), ((rax as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cb6ce0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14329f718; const P1: u64 = 0x14329f730; const P2: u64 = 0x1409fc9b0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = r8;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x0u64) as u64);
    rcx = P0;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    r9 = P1;
    rcx = rax;
    r8 = 0x2;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cb6fc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18; const P2: u64 = 0x10; const P3: u64 = 0x24; const P4: u64 = 0x1c; const P5: u64 = 0x28; const P6: u64 = 0x2c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(rdx).wrapping_add(P0), rax);
    rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rax = ((rax).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64) as u128;
    crate::simd_ops::wu64(rax.wrapping_add(rdx).wrapping_add(P2), ((xmm0 as u64)) as u64);
    crate::simd_ops::wu32(rax.wrapping_add(rdx).wrapping_add(P1), ((rcx as u32)) as u32);
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rax = ((rax).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64) as u128;
    crate::simd_ops::wu64(rax.wrapping_add(rdx).wrapping_add(P4), ((xmm0 as u64)) as u64);
    crate::simd_ops::wu32(rax.wrapping_add(rdx).wrapping_add(P3), ((rcx as u32)) as u32);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P5))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(rdx).wrapping_add(P5), ((rax as u8)) as u8);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P6), ((rax as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cb71e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x1c; const P4: u64 = 0x24; const P5: u64 = 0x28; const P6: u64 = 0x2c;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rax);
    xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P1), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32);
    xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P3), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P4), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P5))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P5), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P6), ((rax as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cbb100(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x18; const P5: u64 = 0x28; const P6: u64 = 0x1c; const P7: u64 = 0x20; const P8: u64 = 0x30;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; r9 = rcx; r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64; crate::simd_ops::wu32(r8.wrapping_add(rcx).wrapping_add(P0), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64; crate::simd_ops::wu32(r8.wrapping_add(rcx).wrapping_add(P1), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r9.wrapping_add(P2))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(P2), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r9.wrapping_add(P3))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(P3), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64; rax = (crate::simd_ops::ru32(r9.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(P4), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(r9.wrapping_add(P5))) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(r9).wrapping_add(P5), rax); rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64; rax = (crate::simd_ops::ru32(r9.wrapping_add(P6))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(P6), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64; rax = (crate::simd_ops::ru64(r9.wrapping_add(P7))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(r9).wrapping_add(P7), rax); rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = (crate::simd_ops::ru64(rdx)) as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rax = (crate::simd_ops::ru32(r9.wrapping_add(P8))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(P8), ((rax as u32)) as u32); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cbb4b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x18; const P5: u64 = 0x28; const P6: u64 = 0x1c; const P7: u64 = 0x20; const P8: u64 = 0x30;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P0), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P1), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P2), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P3), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P4), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P5))) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(rdx).wrapping_add(P5), rax); rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P6), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(P7))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(rdx).wrapping_add(P7), rax); rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = (crate::simd_ops::ru64(r8)) as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P8))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P8), ((rax as u32)) as u32); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cbb6f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x18; const P5: u64 = 0x1c; const P6: u64 = 0x20; const P7: u64 = 0x28; const P8: u64 = 0x30;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P3), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P4), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P5), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P6))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P6), rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P7))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P7), rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P8))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P8), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cbb770(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143293568; const P1: u64 = 0x9; const P2: u64 = 0x20; const P3: u64 = 0x28; const P4: u64 = 0x30; const P5: u64 = 0x8; const P6: u64 = 0xb;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu16(rdx.wrapping_add(P1), (0x0u16) as u16);
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P4), ((rax as u32)) as u32);
    rax = rdx;
    crate::simd_ops::wu8(rdx.wrapping_add(P5), (0x0u8) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P6), (0xffu8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cbbd40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1401e84a0; const P2: u64 = 0x20; const P3: u64 = 0x1403d79a0; const P4: u64 = 0x30; const P5: u64 = 0x40; const P6: u64 = 0x50; const P7: u64 = 0x14240aa00;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rdi = r8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P4)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P4)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P5)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P5)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P6)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P6)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P7, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cbbde0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1431959b8; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x1c; const P5: u64 = 0x20; const P6: u64 = 0x28; const P7: u64 = 0x30; const P8: u64 = 0x38; const P9: u64 = 0x40; const P10: u64 = 0x48; const P11: u64 = 0x50;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P3), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), ((rax as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P8), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P9), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P10), rax);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P11), (0xff000008u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(0x50u64), (0x8u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(0x54u64), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(0x58u64), ((rax as u8)) as u8);
    rax = rdx;
    crate::simd_ops::wu8(rdx.wrapping_add(0x53u64), (0x8u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cbdc50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x140a3e250;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_add(0x10u64)) as u64;
    rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    rdx = rsp.wrapping_add(0x20);
    rax = ((rax).wrapping_add(rcx)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cbdfd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x140a3e250;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rdx.wrapping_add(P1)) as u64) as u64;
    rax = ((rax).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rdx = rsp.wrapping_add(0x20);
    rax = ((rax).wrapping_add(rcx)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cbe1c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1409f87c0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = ((rdx).wrapping_add(0x10u64)) as u64; fl_r = fl_l;
    rdx = ((fl_l as u64)) as u64;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cbe210(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143292ed0; const P1: u64 = 0x10; const P2: u64 = 0x14094f1f0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = P0;
    r8 = rdx;
    rcx = ((rdx.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rax = r8;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cbe900(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x90; const P3: u64 = 0x94; const P4: u64 = 0x98; const P5: u64 = 0x9c; const P6: u64 = 0xa4; const P7: u64 = 0xa8; const P8: u64 = 0xac; const P9: u64 = 0xb0;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rdi = r8;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P2), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P3), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P4), ((rax as u32)) as u32);
    xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64) as u128;
    crate::simd_ops::wu64(rdi.wrapping_add(P5), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P7))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P7), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P8))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P8), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P9))) as u32) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P9), ((rax as u8)) as u8);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cbe9d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14328b9c0; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0xb; const P5: u64 = 0x95; const P6: u64 = 0x94; const P7: u64 = 0x9c; const P8: u64 = 0xa4; const P9: u64 = 0x97;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P1), (0xff000010u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P1), (0x10u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P3), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), (0x10u8) as u8);
    crate::simd_ops::wu16(rdx.wrapping_add(P5), ((rax as u16)) as u16);
    crate::simd_ops::wu8(rdx.wrapping_add(P6), ((rax as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P8), ((rax as u32)) as u32);
    rax = rdx;
    crate::simd_ops::wu8(rdx.wrapping_add(P9), (0xffu8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cc2330(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x90; const P3: u64 = 0x98; const P4: u64 = 0x1409f86d0; const P5: u64 = 0xa8;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rdi = r8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64;
    rdx = ((rbx.wrapping_add(P3)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P3)) as u64) as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P2), ((rax as u32)) as u32);
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P5))) as u32) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P5), ((rax as u8)) as u8);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cc23c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14328a570; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0xb; const P5: u64 = 0x98; const P6: u64 = 0xa0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P1), (0xff000010u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P1), (0x10u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P3), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), (0x10u8) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cc2ad0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x140a33e80; const P2: u64 = 0x18; const P3: u64 = 0x19;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = r8;
    rdi = rdx;
    rax = ((rdx.wrapping_add(P0)) as u64) as u64;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    fl_l = rcx; fl_r = rax;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(5u32)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rax = ((crate::simd_ops::ru8(rdi.wrapping_add(P2))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rbx.wrapping_add(P2), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdi.wrapping_add(P3))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rbx.wrapping_add(P3), ((rax as u8)) as u8);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cc2f60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x15;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P0), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P1), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P2), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P3))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(rdx).wrapping_add(P3), ((rax as u8)) as u8);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P4))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(rdx).wrapping_add(P4), ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cc3140(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x15;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P3))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P3), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P4))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P4), ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cc3ee0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(rdx).wrapping_add(P0), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P1), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P2))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(rdx).wrapping_add(P2), ((rax as u8)) as u8);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P3), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P4), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cc40c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x18;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P0), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P2))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P2), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P3), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P4), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cc5020(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x48; const P1: u64 = 0x14329e8c8; const P2: u64 = 0x14329e8e0; const P3: u64 = 0x14329e898; const P4: u64 = 0x1409fb590; const P5: u64 = 0x14240ab10;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x68u64)) as u64;
    rax = rdx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x4000004u64) as u64);
    crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x0u8) as u8);
    rdx = ((rcx.wrapping_add(P0)) as u64) as u64;
    rcx = P1;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    rcx = P2;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    r9 = P3;
    r8 = 0x3;
    rcx = rax;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rcx = rsp.wrapping_add(0x30);
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x68u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cc51a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14329e8c8; const P1: u64 = 0x14329e8e0; const P2: u64 = 0x14329e898; const P3: u64 = 0x1409fb590; const P4: u64 = 0x14240ab10;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x68u64)) as u64;
    rax = r8;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x4000004u64) as u64);
    crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x0u8) as u8);
    rdx = ((rdx).wrapping_add(0x48u64)) as u64;
    rcx = P0;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    rcx = P1;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    r9 = P2;
    r8 = 0x3;
    rcx = rax;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rcx = rsp.wrapping_add(0x30);
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x68u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cc5680(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x140a335c0; const P2: u64 = 0x18; const P3: u64 = 0x28; const P4: u64 = 0x140a33860; const P5: u64 = 0x38; const P6: u64 = 0x140a33b60; const P7: u64 = 0x48; const P8: u64 = 0x50;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = r8; rdi = rdx; rax = ((rdx.wrapping_add(P0)) as u64) as u64; rcx = ((r8.wrapping_add(P0)) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; r8 = ((r8).wrapping_shl(5u32)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rax = ((rdi.wrapping_add(P2)) as u64) as u64; rcx = ((rbx.wrapping_add(P2)) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; r8 = ((r8).wrapping_shl(5u32)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rax = ((rdi.wrapping_add(P3)) as u64) as u64; rcx = ((rbx.wrapping_add(P3)) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = (((rax) as i64).wrapping_mul((0xc8u64) as i64) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rax = ((rdi.wrapping_add(P5)) as u64) as u64; rcx = ((rbx.wrapping_add(P5)) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = (((rax) as i64).wrapping_mul((0xd0u64) as i64) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P7))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P7), rax); rax = ((crate::simd_ops::ru8(rdi.wrapping_add(P8))) as u32) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(P8), ((rax as u8)) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cc5770(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143294b78; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x28; const P6: u64 = 0x30; const P7: u64 = 0x38; const P8: u64 = 0x40; const P9: u64 = 0x48; const P10: u64 = 0x50;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P8), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P9), rax);
    rax = rdx;
    crate::simd_ops::wu8(rdx.wrapping_add(P10), (0x1u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cc5f70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143282d50; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0xb; const P5: u64 = 0x30; const P6: u64 = 0x34; const P7: u64 = 0x38; const P8: u64 = 0x33; const P9: u64 = 0x58; const P10: u64 = 0x5c; const P11: u64 = 0x60;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P1), (0xff000004u32) as u32);
    crate::simd_ops::lk_or_u32(rdx.wrapping_add(P1), (0x4u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P3), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), (0x4u8) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P5), (0xff000004u32) as u32);
    crate::simd_ops::lk_or_u32(rdx.wrapping_add(P5), (0x4u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P6), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P7), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P8), (0x4u8) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P9), (0xff000004u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P9), (0x4u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P10), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P11), ((rax as u8)) as u8);
    rax = rdx;
    crate::simd_ops::wu8(rdx.wrapping_add(0x5bu64), (0x4u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cc6680(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x140a33390;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = ((rdx.wrapping_add(P0)) as u64) as u64;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    fl_l = rcx; fl_r = rax;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cc71a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14329ecb0; const P2: u64 = 0x14329ecc8; const P3: u64 = 0x1402c4e80;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = rdx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x0u64) as u64);
    rdx = ((rcx.wrapping_add(P0)) as u64) as u64;
    r8 = 0x1;
    rcx = P1;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    r9 = P2;
    rcx = rax;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cc7320(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14329ecb0; const P1: u64 = 0x14329ecc8; const P2: u64 = 0x1402c4e80;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = r8;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x0u64) as u64);
    rcx = P0;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    r9 = P1;
    rcx = rax;
    r8 = 0x1;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cc8370(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x9; const P2: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    r9 = rcx;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    crate::simd_ops::wu8(r8.wrapping_add(rcx).wrapping_add(P0), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    crate::simd_ops::wu8(r8.wrapping_add(rcx).wrapping_add(P1), ((rax as u8)) as u8);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(rdx)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = (crate::simd_ops::ru32(r9.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(P2), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cc8430(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434269c8; const P1: u64 = 0x14329ec48; const P2: u64 = 0x14329ecf8; const P3: u64 = 0x14329ecd8; const P4: u64 = 0x14020f270;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    xmm1 = ((crate::simd_ops::ru64(P0)) as u64) as u128;
    rax = r8;
    rcx = P1;
    xmm0 = 0u128;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), ((xmm0 as u64)) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm1 as u64)) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    r9 = P2;
    rcx = P3;
    rdx = ((rdx).wrapping_add(0xcu64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    r8 = 0x1;
    rcx = rax;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cc86e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x9; const P2: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(rdx).wrapping_add(P0), ((rax as u8)) as u8);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(rdx).wrapping_add(P1), ((rax as u8)) as u8);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P2), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cca0f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143290d50; const P1: u64 = 0x8; const P2: u64 = 0x10;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = P0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), (0x0u64) as u64);
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::wu8(rdx.wrapping_add(P2), (0x0u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ccb2c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    r9 = rcx;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(rcx).wrapping_add(P0), rax);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(rcx).wrapping_add(P1), rax);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    rax = (crate::simd_ops::ru64(r9.wrapping_add(P2))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(r9).wrapping_add(P2), rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    let y_addr: u64 = r9.wrapping_add(P3); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    let y_addr: u64 = rax.wrapping_add(r9).wrapping_add(P3); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(rdx)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = ((crate::simd_ops::ru8(r9.wrapping_add(P4))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(r9).wrapping_add(P4), ((rax as u8)) as u8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ccb380(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14329eda8; const P1: u64 = 0x9; const P2: u64 = 0x14028a830;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    r9 = P0;
    rax = r8;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rdx = ((rdx).wrapping_add(0x18u64)) as u64;
    r8 = ((rcx.wrapping_add(P1)) as u32) as u32 as u64;
    rcx = rax;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ccb650(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(rdx).wrapping_add(P0), rax);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(rdx).wrapping_add(P1), rax);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(rdx).wrapping_add(P2), rax);
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rax = ((rax).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    let y_addr: u64 = rdx.wrapping_add(P3); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    let y_addr: u64 = rax.wrapping_add(rdx).wrapping_add(P3); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P4))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(rdx).wrapping_add(P4), ((rax as u8)) as u8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ccb860(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x40;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut ymm0_hi: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P1), rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P2), rax);
    let y_addr: u64 = rdx.wrapping_add(P3); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    let y_addr: u64 = r8.wrapping_add(P3); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P4))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P4), ((rax as u8)) as u8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ccbaf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xc; const P1: u64 = 0x8; const P2: u64 = 0xd; const P3: u64 = 0xe;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    r9 = rcx;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    crate::simd_ops::wu8(r8.wrapping_add(rcx).wrapping_add(P0), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P2))) as u32) as u32 as u64;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    crate::simd_ops::wu8(r8.wrapping_add(rcx).wrapping_add(P2), ((rax as u8)) as u8);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    rax = ((crate::simd_ops::ru8(r9.wrapping_add(P3))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(r9).wrapping_add(P3), ((rax as u8)) as u8);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(rdx)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = (crate::simd_ops::ru32(r9.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(P1), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ccbe20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xc; const P1: u64 = 0x8; const P2: u64 = 0xd; const P3: u64 = 0xe;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(rdx).wrapping_add(P0), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P2))) as u32) as u32 as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(rdx).wrapping_add(P2), ((rax as u8)) as u8);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P3))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(rdx).wrapping_add(P3), ((rax as u8)) as u8);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P1), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ccc020(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0xd; const P3: u64 = 0xe;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P1), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P2))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P2), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P3))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P3), ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cce8b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426c20; const P1: u64 = 0x14329f150; const P2: u64 = 0x8; const P3: u64 = 0x14329f128; const P4: u64 = 0x14020f270;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    xmm1 = ((crate::simd_ops::ru64(P0)) as u64) as u128;
    rax = rdx;
    r9 = P1;
    rdx = ((rcx.wrapping_add(P2)) as u64) as u64;
    r8 = 0;
    fl_l = 0; fl_r = 0;
    xmm0 = 0u128;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), ((xmm0 as u64)) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm1 as u64)) as u64);
    rcx = P3;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x0u64) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rcx = rax;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ccea40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426c20; const P1: u64 = 0x14329f128; const P2: u64 = 0x14329f150; const P3: u64 = 0x14020f270;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    xmm1 = ((crate::simd_ops::ru64(P0)) as u64) as u128;
    rax = r8;
    rcx = P1;
    xmm0 = 0u128;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), ((xmm0 as u64)) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm1 as u64)) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x0u64) as u64);
    r9 = P2;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rcx = rax;
    r8 = 0;
    fl_l = 0; fl_r = 0;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ccfdf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x15;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    r9 = rcx;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(rcx).wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(rcx).wrapping_add(P1), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    rax = (crate::simd_ops::ru32(r9.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(P2), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    rax = ((crate::simd_ops::ru8(r9.wrapping_add(P3))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(r9).wrapping_add(P3), ((rax as u8)) as u8);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(rdx)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = ((crate::simd_ops::ru8(r9.wrapping_add(P4))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(r9).wrapping_add(P4), ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cd0330(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    r9 = rcx;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    crate::simd_ops::wu8(r8.wrapping_add(rcx).wrapping_add(P0), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(rcx).wrapping_add(P1), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    rax = ((crate::simd_ops::ru8(r9.wrapping_add(P2))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(r9).wrapping_add(P2), ((rax as u8)) as u8);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    rax = (crate::simd_ops::ru32(r9.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(P3), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(rdx)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = (crate::simd_ops::ru32(r9.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(P4), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cd09c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140265ba0; const P1: u64 = 0x10; const P2: u64 = 0x140aa1f80; const P3: u64 = 0x20; const P4: u64 = 0x8; const P5: u64 = 0x140a359f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = r8;
    rbx = rdx;
    rcx = r8;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rbx = ((rbx).wrapping_add(0x20u64)) as u64;
    rcx = ((rdi.wrapping_add(P3)) as u64) as u64;
    fl_l = rcx; fl_r = rbx;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rdx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((rdx.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cd0a30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x1401fb9f0; const P2: u64 = 0x10; const P3: u64 = 0x1401de830;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rcx = ((rdx.wrapping_add(P0)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cd0aa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xfffffffffffffff8; const P1: u64 = 0xfffffffffffffff0; const P2: u64 = 0x20; const P3: u64 = 0x1401fb9f0; const P4: u64 = 0x10; const P5: u64 = 0x1401de830; const P6: u64 = 0x142442c80;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rsi = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rbx = rsi; rbx = (((rbx) as i64).wrapping_mul((crate::simd_ops::ru64(rdi.wrapping_add(P1))) as i64) as u64) as u64; rbx = ((rbx).wrapping_add(rdi)) as u64; rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rsi = ((rsi).wrapping_add(1)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rcx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 5; continue; }
            5 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cd1910(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x140265ba0; const P2: u64 = 0x18; const P3: u64 = 0x38; const P4: u64 = 0x19; const P5: u64 = 0x20; const P6: u64 = 0x30; const P7: u64 = 0x14240aa00; const P8: u64 = 0x60;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rdi = r8;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u32) as u32 as u64;
    rdx = ((rbx.wrapping_add(P3)) as u64) as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P2), ((rax as u8)) as u8);
    rcx = ((rdi.wrapping_add(P3)) as u64) as u64;
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P4))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P4), ((rax as u8)) as u8);
    xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(P5))) as u128;
    crate::simd_ops::wu128(rdi.wrapping_add(P5), (xmm0) as u128);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rax as u32)) as u32);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P8))) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P8), ((rax as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cd19a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1432a8458; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x20; const P4: u64 = 0x28; const P5: u64 = 0x30; const P6: u64 = 0x38; const P7: u64 = 0x3b; const P8: u64 = 0x3c; const P9: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = P0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rcx);
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rcx);
    crate::simd_ops::wu32(rdx.wrapping_add(P5), ((rcx as u32)) as u32);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P6), (0xff000004u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P6), (0x4u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu8(rdx.wrapping_add(P7), (0x4u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P8), ((rcx as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P9), ((rcx as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cd2210(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x140265ba0; const P2: u64 = 0x18; const P3: u64 = 0x19; const P4: u64 = 0x20; const P5: u64 = 0x30; const P6: u64 = 0x38;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rdi = r8;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P2), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P3))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P3), ((rax as u8)) as u8);
    xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(P4))) as u128;
    crate::simd_ops::wu128(rdi.wrapping_add(P4), (xmm0) as u128);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P6))) as u32) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P6), ((rax as u8)) as u8);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cd2a90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x140265ba0; const P2: u64 = 0x18; const P3: u64 = 0x19; const P4: u64 = 0x20; const P5: u64 = 0x30; const P6: u64 = 0x38;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rdi = r8;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P2), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P3))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P3), ((rax as u8)) as u8);
    xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(P4))) as u128;
    crate::simd_ops::wu128(rdi.wrapping_add(P4), (xmm0) as u128);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rax as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cd3310(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x140265ba0; const P2: u64 = 0x18; const P3: u64 = 0x19; const P4: u64 = 0x20; const P5: u64 = 0x30; const P6: u64 = 0x38; const P7: u64 = 0x40; const P8: u64 = 0x44; const P9: u64 = 0x48;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rdi = r8;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P2), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P3))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P3), ((rax as u8)) as u8);
    xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(P4))) as u128;
    crate::simd_ops::wu128(rdi.wrapping_add(P4), (xmm0) as u128);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32);
    xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64) as u128;
    crate::simd_ops::wu64(rdi.wrapping_add(P6), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P7))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P7), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P8))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P8), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P9))) as u32) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P9), ((rax as u8)) as u8);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cd33b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1432a8840; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x20; const P4: u64 = 0x28; const P5: u64 = 0x30; const P6: u64 = 0x38; const P7: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = P0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rcx);
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rcx);
    crate::simd_ops::wu32(rdx.wrapping_add(P5), ((rcx as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rcx);
    crate::simd_ops::wu32(rdx.wrapping_add(P7), ((rcx as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cd3ba0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x140265ba0; const P2: u64 = 0x18; const P3: u64 = 0x40; const P4: u64 = 0x19; const P5: u64 = 0x20; const P6: u64 = 0x30; const P7: u64 = 0x38; const P8: u64 = 0x1408265b0; const P9: u64 = 0x70;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rdi = r8;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u32) as u32 as u64;
    rdx = ((rbx.wrapping_add(P3)) as u64) as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P2), ((rax as u8)) as u8);
    rcx = ((rdi.wrapping_add(P3)) as u64) as u64;
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P4))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P4), ((rax as u8)) as u8);
    xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(P5))) as u128;
    crate::simd_ops::wu128(rdi.wrapping_add(P5), (xmm0) as u128);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P7))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P7), ((rax as u32)) as u32);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rdx = ((rbx.wrapping_add(P9)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P9)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cd3c40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1432a8128; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x20; const P4: u64 = 0x28; const P5: u64 = 0x30; const P6: u64 = 0x40; const P7: u64 = 0x48; const P8: u64 = 0x50; const P9: u64 = 0x58; const P10: u64 = 0x60; const P11: u64 = 0x68;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = P0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rcx);
    crate::simd_ops::wu32(rdx.wrapping_add(P5), ((rcx as u32)) as u32);
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P8), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P9), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P10), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P11), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x70u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x78u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x80u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x88u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x90u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x98u64), rcx);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cd4470(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x140265ba0; const P2: u64 = 0x18; const P3: u64 = 0x48; const P4: u64 = 0x19; const P5: u64 = 0x20; const P6: u64 = 0x30; const P7: u64 = 0x38; const P8: u64 = 0x3c; const P9: u64 = 0x1408265b0; const P10: u64 = 0x78;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rdi = r8;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u32) as u32 as u64;
    rdx = ((rbx.wrapping_add(P3)) as u64) as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P2), ((rax as u8)) as u8);
    rcx = ((rdi.wrapping_add(P3)) as u64) as u64;
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P4))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P4), ((rax as u8)) as u8);
    xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(P5))) as u128;
    crate::simd_ops::wu128(rdi.wrapping_add(P5), (xmm0) as u128);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P7))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P7), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P8))) as u64;
    crate::simd_ops::wu64(rdi.wrapping_add(P8), rax);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rdx = ((rbx.wrapping_add(P10)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P10)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cd4520(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1432a8510; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x20; const P4: u64 = 0x28; const P5: u64 = 0x30; const P6: u64 = 0x3c; const P7: u64 = 0x48; const P8: u64 = 0x50; const P9: u64 = 0x58; const P10: u64 = 0x60; const P11: u64 = 0x68;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = P0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rcx);
    crate::simd_ops::wu32(rdx.wrapping_add(P5), ((rcx as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rcx);
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P8), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P9), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P10), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P11), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x70u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x78u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x80u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x88u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x90u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x98u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0xa0u64), rcx);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cd4d50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x140265ba0; const P2: u64 = 0x18; const P3: u64 = 0x19; const P4: u64 = 0x20; const P5: u64 = 0x30; const P6: u64 = 0x38; const P7: u64 = 0x3c; const P8: u64 = 0x40; const P9: u64 = 0x44; const P10: u64 = 0x48; const P11: u64 = 0x4c;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rdi = r8;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P2), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P3))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P3), ((rax as u8)) as u8);
    xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(P4))) as u128;
    crate::simd_ops::wu128(rdi.wrapping_add(P4), (xmm0) as u128);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P7))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P7), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P8))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P8), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P9))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P9), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P10))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P10), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P11))) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu64(rdi.wrapping_add(0x4cu64), rax);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cd4e00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1432a85c8; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x20; const P4: u64 = 0x28; const P5: u64 = 0x30; const P6: u64 = 0x4c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = P0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rcx);
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rcx);
    crate::simd_ops::wu32(rdx.wrapping_add(P5), ((rcx as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rcx);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cd5660(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x140265ba0; const P2: u64 = 0x18; const P3: u64 = 0x19; const P4: u64 = 0x20; const P5: u64 = 0x30;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rdi = r8;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P2), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P3))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P3), ((rax as u8)) as u8);
    xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(P4))) as u128;
    crate::simd_ops::wu128(rdi.wrapping_add(P4), (xmm0) as u128);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cd7500(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1432a9588; const P1: u64 = 0x9; const P2: u64 = 0x8; const P3: u64 = 0xb;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = P0;
    crate::simd_ops::wu16(rdx.wrapping_add(P1), (0x0u16) as u16);
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::wu8(rdx.wrapping_add(P2), (0x0u8) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P3), (0xffu8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cdacd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14240aa00; const P1: u64 = 0x88; const P2: u64 = 0x90; const P3: u64 = 0x140a6e6c0;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rcx = r8;
    rdi = r8;
    rbx = rdx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64;
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P1), ((rax as u32)) as u32);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cdad30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x90; const P1: u64 = 0x142442de0; const P2: u64 = 0x14240ab10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rdi = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = rdi; fl_r = rdi;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    crate::simd_ops::wu64(rbx.wrapping_add(P0), (0x0u64) as u64);
    rcx = rbx;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cdad80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x8; const P2: u64 = 0x3; const P3: u64 = 0x88; const P4: u64 = 0x90;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::lk_and_u32(rdx, (0xff000010u32) as u32);
    rax = 0;
    fl_l = 0; fl_r = 0;
    fl_l = (crate::simd_ops::lk_or_u32(rdx, (0x10u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P1), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P2), (0x10u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P3), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cdadb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xfffffffffffffff8; const P1: u64 = 0xfffffffffffffff0; const P2: u64 = 0x90; const P3: u64 = 0x142442de0; const P4: u64 = 0x14240ab10; const P5: u64 = 0x142442c80;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rbp = 0; fl_l = 0; fl_r = 0; rsi = ((rbp as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 6; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rbx = rsi; rbx = (((rbx) as i64).wrapping_mul((crate::simd_ops::ru64(rdi.wrapping_add(P1))) as i64) as u64) as u64; rbx = ((rbx).wrapping_add(rdi)) as u64; r14 = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(r14)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = r14; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = r14; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbx.wrapping_add(P2), rbp); rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rsi = ((rsi).wrapping_add(1)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 6; continue; }
            6 => { rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 7; continue; }
            7 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cdae50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x90; const P1: u64 = 0x142442de0; const P2: u64 = 0x14240ab10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rdi = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = rdi; fl_r = rdi;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    crate::simd_ops::wu64(rbx.wrapping_add(P0), (0x0u64) as u64);
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cdb760(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x140ab2170;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_add(0xb8u64)) as u64;
    rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    rdx = rsp.wrapping_add(0x20);
    rax = ((rax).wrapping_add(rcx)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cdbc80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x118; const P1: u64 = 0x140ab7fe0; const P2: u64 = 0xe8; const P3: u64 = 0x14240ab10; const P4: u64 = 0xb8; const P5: u64 = 0x142428ce0;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rdx;
    rcx = ((rdx.wrapping_add(P0)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P4)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rcx = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P5, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cdbce0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xfffffffffffffff8; const P1: u64 = 0xfffffffffffffff0; const P2: u64 = 0x118; const P3: u64 = 0x140ab7fe0; const P4: u64 = 0xe8; const P5: u64 = 0x14240ab10; const P6: u64 = 0xb8; const P7: u64 = 0x142428ce0; const P8: u64 = 0x142442c80;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rbp = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rsi = rbp; rsi = (((rsi) as i64).wrapping_mul((crate::simd_ops::ru64(rbx.wrapping_add(P1))) as i64) as u64) as u64; rsi = ((rsi).wrapping_add(rbx)) as u64; rcx = ((rsi.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = ((rsi.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = ((rsi.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rbp = ((rbp).wrapping_add(1)) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; fl_l = rbp; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rcx = rbx; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 4; continue; }
            4 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cdbd70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x118; const P1: u64 = 0x140ab7fe0; const P2: u64 = 0xe8; const P3: u64 = 0x14240ab10; const P4: u64 = 0xb8; const P5: u64 = 0x142428ce0; const P6: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rcx = ((rdx.wrapping_add(P0)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); }
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cdcf80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xfffffffffffffff8; const P1: u64 = 0xfffffffffffffff0; const P2: u64 = 0xd8; const P3: u64 = 0x8; const P4: u64 = 0x140267a30; const P5: u64 = 0x1401e66f0; const P6: u64 = 0xc8; const P7: u64 = 0x1401de830; const P8: u64 = 0xb8; const P9: u64 = 0x140abc130; const P10: u64 = 0x142428ce0; const P11: u64 = 0x142442c80;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rbp = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rsi = rbp; rsi = (((rsi) as i64).wrapping_mul((crate::simd_ops::ru64(rbx.wrapping_add(P1))) as i64) as u64) as u64; rsi = ((rsi).wrapping_add(rbx)) as u64; r8 = (crate::simd_ops::ru64(rsi.wrapping_add(P2))) as u64; r8 = (crate::simd_ops::ru64(r8.wrapping_add(P3))) as u64; rdx = ((rsi.wrapping_add(P2)) as u64) as u64; rcx = ((rsi.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rsi.wrapping_add(P2))) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = ((rsi.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(rsi.wrapping_add(P8))) as u64; r8 = (crate::simd_ops::ru64(r8.wrapping_add(P3))) as u64; rdx = ((rsi.wrapping_add(P8)) as u64) as u64; rcx = ((rsi.wrapping_add(P8)) as u64) as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rsi.wrapping_add(P8))) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rbp = ((rbp).wrapping_add(1)) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; fl_l = rbp; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rcx = rbx; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 5; continue; }
            5 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cdea00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x3; const P1: u64 = 0x4; const P2: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::lk_and_u32(rdx, (0xff000005u32) as u32);
    rax = rdx;
    fl_l = (crate::simd_ops::lk_or_u32(rdx, (0x5u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu8(rdx.wrapping_add(P0), (0x5u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P1), (0x0u32) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P2), (0x0u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cdef90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14240aa00; const P1: u64 = 0x30; const P2: u64 = 0x60; const P3: u64 = 0x8; const P4: u64 = 0x140ac8f50;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = r8;
    rbx = rdx;
    rcx = r8;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rbx = ((rbx).wrapping_add(0x60u64)) as u64;
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    fl_l = rcx; fl_r = rbx;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rdx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; r8 = (((rax) as i64).wrapping_mul((0x34u64) as i64) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cdf030(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x8; const P2: u64 = 0x3; const P3: u64 = 0x30; const P4: u64 = 0x34; const P5: u64 = 0x38; const P6: u64 = 0x33; const P7: u64 = 0x60; const P8: u64 = 0x68;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::lk_and_u32(rdx, (0xff000005u32) as u32);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::lk_or_u32(rdx, (0x5u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P1), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P2), (0x5u8) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P3), (0xff000005u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P3), (0x5u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P4), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P5), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P6), (0x5u8) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P8), rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cdf5f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14240aa00; const P1: u64 = 0x140abc1d0; const P2: u64 = 0x140ac8600; const P3: u64 = 0x140ac8d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = r8; rbp = rdx; rcx = r8; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r14 = rbp.wrapping_add(0x30); rdi = rsi.wrapping_add(0x30); fl_l = rdi; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rdi)) as u64; r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = rdi; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rbx, rbx); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), (0x0u64) as u64); rdx = (crate::simd_ops::ru64(r14)) as u64; r8 = (crate::simd_ops::ru64(rdi)) as u64; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rax); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rax); r8 = (crate::simd_ops::ru64(rdi)) as u64; rdx = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { rdx = rcx; rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = rax; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(r8, rdx); rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = rax; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), rcx); bb = 9; continue; }
            8 => { crate::simd_ops::wu64(r8, r8); rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rax); bb = 9; continue; }
            9 => { rax = rbp.wrapping_add(0x40); rcx = rsi.wrapping_add(0x40); fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cdf730(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x140ab7f10; const P2: u64 = 0x30; const P3: u64 = 0x8; const P4: u64 = 0x140abc1d0; const P5: u64 = 0x1401e66f0; const P6: u64 = 0x14240ab10;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rdx;
    rcx = ((rdx.wrapping_add(P0)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    r8 = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64;
    r8 = (crate::simd_ops::ru64(r8.wrapping_add(P3))) as u64;
    rdx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rcx = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P6, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cdf790(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xfffffffffffffff8; const P1: u64 = 0xfffffffffffffff0; const P2: u64 = 0x40; const P3: u64 = 0x140ab7f10; const P4: u64 = 0x30; const P5: u64 = 0x8; const P6: u64 = 0x140abc1d0; const P7: u64 = 0x1401e66f0; const P8: u64 = 0x14240ab10; const P9: u64 = 0x142442c80;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rbp = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rsi = rbp; rsi = (((rsi) as i64).wrapping_mul((crate::simd_ops::ru64(rbx.wrapping_add(P1))) as i64) as u64) as u64; rsi = ((rsi).wrapping_add(rbx)) as u64; rcx = ((rsi.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(rsi.wrapping_add(P4))) as u64; r8 = (crate::simd_ops::ru64(r8.wrapping_add(P5))) as u64; rdx = ((rsi.wrapping_add(P4)) as u64) as u64; rcx = ((rsi.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rsi.wrapping_add(P4))) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rbp = ((rbp).wrapping_add(1)) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; fl_l = rbp; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rcx = rbx; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 5; continue; }
            5 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cdf830(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x140ab7f10; const P2: u64 = 0x30; const P3: u64 = 0x8; const P4: u64 = 0x140abc1d0; const P5: u64 = 0x1401e66f0; const P6: u64 = 0x14240ab10; const P7: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rcx = ((rdx.wrapping_add(P0)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; r8 = (crate::simd_ops::ru64(r8.wrapping_add(P3))) as u64; rdx = ((rdi.wrapping_add(P2)) as u64) as u64; rcx = ((rdi.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); }
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cdfe80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x142442de0; const P2: u64 = 0x140246920;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rdi = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = rdi; fl_r = rdi;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    crate::simd_ops::wu64(rbx.wrapping_add(P0), (0x0u64) as u64);
    rdx = rbx;
    rcx = rbx;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cdff20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xfffffffffffffff8; const P1: u64 = 0xfffffffffffffff0; const P2: u64 = 0x10; const P3: u64 = 0x142442de0; const P4: u64 = 0x140246920; const P5: u64 = 0x142442c80;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rbp = 0; fl_l = 0; fl_r = 0; rsi = ((rbp as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 6; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rbx = rsi; rbx = (((rbx) as i64).wrapping_mul((crate::simd_ops::ru64(rdi.wrapping_add(P1))) as i64) as u64) as u64; rbx = ((rbx).wrapping_add(rdi)) as u64; r14 = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(r14)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = r14; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = r14; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbx.wrapping_add(P2), rbp); rdx = rbx; rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rsi = ((rsi).wrapping_add(1)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 6; continue; }
            6 => { rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 7; continue; }
            7 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cdffc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x142442de0; const P2: u64 = 0x140246920;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rdi = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = rdi; fl_r = rdi;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    crate::simd_ops::wu64(rbx.wrapping_add(P0), (0x0u64) as u64);
    rdx = rbx;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ce0670(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xfffffffffffffff8; const P1: u64 = 0xfffffffffffffff0; const P2: u64 = 0x10; const P3: u64 = 0x142442de0; const P4: u64 = 0x8; const P5: u64 = 0x14021cb50; const P6: u64 = 0x1401e66f0; const P7: u64 = 0x142442c80;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; r14 = 0; fl_l = 0; fl_r = 0; rbp = ((r14 as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rbx = rbp; rbx = (((rbx) as i64).wrapping_mul((crate::simd_ops::ru64(rdi.wrapping_add(P1))) as i64) as u64) as u64; rbx = ((rbx).wrapping_add(rdi)) as u64; rsi = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rbx.wrapping_add(P2), r14); r8 = (crate::simd_ops::ru64(rbx)) as u64; r8 = (crate::simd_ops::ru64(r8.wrapping_add(P4))) as u64; rdx = rbx; rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rbp = ((rbp).wrapping_add(1)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; fl_l = rbp; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { rcx = rdi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 6; continue; }
            6 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ce2770(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1406d6620; const P1: u64 = 0x10; const P2: u64 = 0x8; const P3: u64 = 0x140b06760; const P4: u64 = 0x20; const P5: u64 = 0x14240aa00;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = r8;
    rdi = rdx;
    rcx = r8;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = ((rdi.wrapping_add(P1)) as u64) as u64;
    rcx = ((rbx.wrapping_add(P1)) as u64) as u64;
    fl_l = rcx; fl_r = rax;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rdx = (crate::simd_ops::ru64(rax)) as u64; r8 = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; r8 = ((r8).wrapping_shl(5u32)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rdx = ((rdi.wrapping_add(P4)) as u64) as u64;
    rcx = ((rbx.wrapping_add(P4)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P5, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ce27e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x14240ab10; const P2: u64 = 0x10; const P3: u64 = 0x140afc400; const P4: u64 = 0x1401de830;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rcx = ((rdx.wrapping_add(P0)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ce2820(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x24; const P5: u64 = 0x28; const P6: u64 = 0x23;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx, rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P3), (0xff000004u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P3), (0x4u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P4), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P5), ((rax as u8)) as u8);
    rax = rdx;
    crate::simd_ops::wu8(rdx.wrapping_add(P6), (0x4u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ce3560(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x140b06e20; const P2: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = r8;
    rbx = rdx;
    fl_l = r8; fl_r = rdx;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rdx = (crate::simd_ops::ru64(rdx)) as u64; r8 = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; r8 = ((r8).wrapping_shl(5u32)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rbx = ((rbx).wrapping_add(0x10u64)) as u64;
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    fl_l = rcx; fl_r = rbx;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rdx = (crate::simd_ops::ru64(rbx)) as u64; r8 = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; r8 = ((r8).wrapping_shl(5u32)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ce3c40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x70; const P2: u64 = 0xc; const P3: u64 = 0x14; const P4: u64 = 0x18; const P5: u64 = 0x20; const P6: u64 = 0x24; const P7: u64 = 0x2c; const P8: u64 = 0x30; const P9: u64 = 0x40; const P10: u64 = 0x48; const P11: u64 = 0x50; const P12: u64 = 0x1404b9990;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; xmm0 = ((crate::simd_ops::ru64(rdx)) as u64) as u128; crate::simd_ops::wu64(r8, ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; rcx = ((r8.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32); rbx = rdx; xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64) as u128; crate::simd_ops::wu64(r8.wrapping_add(P2), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64; rdi = r8; crate::simd_ops::wu32(r8.wrapping_add(P3), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64) as u128; crate::simd_ops::wu64(r8.wrapping_add(P4), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P5), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P6))) as u64) as u128; crate::simd_ops::wu64(r8.wrapping_add(P6), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P7))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P7), ((rax as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P8))) as u128; crate::simd_ops::wu128(r8.wrapping_add(P8), (xmm0) as u128); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P9))) as u64; crate::simd_ops::wu64(r8.wrapping_add(P9), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P10))) as u64; crate::simd_ops::wu64(r8.wrapping_add(P10), rax); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P11))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x50u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x54u64))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(0x54u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x55u64))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(0x55u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x58u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x58u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x5cu64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x5cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x60u64), rax); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x68u64))) as u32 as u64; rdx = ((rdx).wrapping_add(0x70u64)) as u64; crate::simd_ops::wu32(r8.wrapping_add(0x68u64), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x90u64))) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x90u64), ((rax as u8)) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ce3d30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143427660; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x24; const P4: u64 = 0x143428240; const P5: u64 = 0x18; const P6: u64 = 0x14; const P7: u64 = 0x2c; const P8: u64 = 0x20; const P9: u64 = 0x30; const P10: u64 = 0x40; const P11: u64 = 0x48;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    xmm1 = (crate::simd_ops::ru128(P0)) as u128;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = rdx;
    xmm0 = 0u128;
    crate::simd_ops::wu32(rsp.wrapping_add(0xcu64), ((xmm0 as u32)) as u32);
    crate::simd_ops::wu128(rsp.wrapping_add(0x10u64), (xmm1) as u128);
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm1) as u128);
    crate::simd_ops::wu64(rdx, (0x3f800000u64) as u64);
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rcx as u32)) as u32);
    xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0xcu64))) as u64) as u128;
    crate::simd_ops::wu64(rdx.wrapping_add(P2), ((xmm0 as u64)) as u64);
    xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x24u64))) as u64) as u128;
    crate::simd_ops::wu64(rdx.wrapping_add(P3), ((xmm0 as u64)) as u64);
    xmm0 = (crate::simd_ops::ru128(P4)) as u128;
    crate::simd_ops::wu64(rdx.wrapping_add(P5), (((xmm1 >> 64) as u64)) as u64);
    crate::simd_ops::wu32(rdx.wrapping_add(P6), (((xmm1 >> 32) as u32) as u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P7), (((xmm1 >> 96) as u32) as u32) as u32);
    xmm2 = (xmm1) as u128;
    crate::simd_ops::wu32(rdx.wrapping_add(P8), ((xmm2 as u32)) as u32);
    crate::simd_ops::wu128(rdx.wrapping_add(P9), (xmm0) as u128);
    crate::simd_ops::wu64(rdx.wrapping_add(P10), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P11), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x60u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x70u64), rcx);
    crate::simd_ops::wu32(rdx.wrapping_add(0x78u64), ((rcx as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(0x80u64), rcx);
    crate::simd_ops::wu8(rdx.wrapping_add(0x88u64), ((rcx as u8)) as u8);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ce5000(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x14240aa00; const P3: u64 = 0x58;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    crate::simd_ops::wu64(r8, rax);
    rbx = rdx;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    rdi = r8;
    rdx = ((rdx).wrapping_add(0x10u64)) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P1), rax);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P3))) as u32) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P3), ((rax as u8)) as u8);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ce5070(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x14; const P3: u64 = 0x18; const P4: u64 = 0x13;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx, rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rax);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P1), (0xff000008u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P1), (0x8u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P3), ((rax as u8)) as u8);
    rax = rdx;
    crate::simd_ops::wu8(rdx.wrapping_add(P4), (0x8u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ce5660(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x18; const P2: u64 = 0x1c; const P3: u64 = 0x24; const P4: u64 = 0x28; const P5: u64 = 0x30; const P6: u64 = 0x38; const P7: u64 = 0x40; const P8: u64 = 0x48; const P9: u64 = 0x8; const P10: u64 = 0x140a34f60;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    crate::simd_ops::wu128(r8, (xmm0) as u128);
    xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P0), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P2), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P3), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P4))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P4), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P5))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P5), rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P6))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P6), rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P7))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P7), rax);
    rax = ((rdx.wrapping_add(P8)) as u64) as u64;
    rcx = ((r8.wrapping_add(P8)) as u64) as u64;
    fl_l = rcx; fl_r = rax;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P9))) as u32 as u64; r8 = (((rax) as i64).wrapping_mul((0xb8u64) as i64) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ce5710(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x30; const P5: u64 = 0x38; const P6: u64 = 0x40; const P7: u64 = 0x48; const P8: u64 = 0x50;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx, rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P8), rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ce5750(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xfffffffffffffff8; const P1: u64 = 0xfffffffffffffff0; const P2: u64 = 0x140a02a70; const P3: u64 = 0x142442c80;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rdi = 0;
    fl_l = 0; fl_r = 0;
    fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64;
    if (((fl_l as u32)) <= ((fl_r as u32))) { rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    loop { rcx = rdi; rcx = (((rcx) as i64).wrapping_mul((crate::simd_ops::ru64(rbx.wrapping_add(P1))) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(0x48u64)) as u64; rcx = ((rcx).wrapping_add(rbx)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdi = ((rdi).wrapping_add(1)) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; fl_l = rdi; fl_r = rax; if !((((fl_l as u64)) < ((fl_r as u64)))) { break; } }
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ce5b20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    r10 = rdx;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    r9 = rdx;
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu64(r9.wrapping_add(rcx), rax);
    r9 = ((rdx.wrapping_add(P0)) as u64) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rdx = ((rdx).wrapping_add(0x10u64)) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu64(r9.wrapping_add(rcx), rax);
    rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru64(r10.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(rcx), rax);
    rdx = ((r10.wrapping_add(P2)) as u64) as u64;
    fl_l = rdx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru64(r10.wrapping_add(P2))) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(rcx), rax);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ce5b80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    r9 = rdx;
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu64(r9.wrapping_add(rcx), rax);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(rdx).wrapping_add(P0), rax);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(rdx).wrapping_add(P1), rax);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(rdx).wrapping_add(P2), rax);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ce8690(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14240aa00; const P1: u64 = 0x90; const P2: u64 = 0x140941970; const P3: u64 = 0x150; const P4: u64 = 0x8; const P5: u64 = 0x140b064c0; const P6: u64 = 0x160; const P7: u64 = 0x1403d79a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = r8;
    rdi = rdx;
    rcx = r8;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rdi.wrapping_add(P1)) as u64) as u64;
    rcx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rax = ((rdi.wrapping_add(P3)) as u64) as u64;
    rcx = ((rbx.wrapping_add(P3)) as u64) as u64;
    fl_l = rcx; fl_r = rax;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P4))) as u32 as u64; r8 = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); }
    rdx = ((rdi.wrapping_add(P6)) as u64) as u64;
    rcx = ((rbx.wrapping_add(P6)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P7, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ce8730(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x90; const P1: u64 = 0x3; const P2: u64 = 0x4; const P3: u64 = 0x8; const P4: u64 = 0x14093bce0; const P5: u64 = 0x150; const P6: u64 = 0x158; const P7: u64 = 0x160; const P8: u64 = 0x168;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    crate::simd_ops::lk_and_u32(rdx, (0xff000010u32) as u32);
    rcx = ((rdx.wrapping_add(P0)) as u64) as u64;
    crate::simd_ops::lk_or_u32(rdx, (0x10u32) as u32);
    r9 = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu8(rdx.wrapping_add(P1), (0x10u8) as u8);
    r8 = rdx;
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((r9 as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P3), ((r9 as u8)) as u8);
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(r8.wrapping_add(P5), r9);
    rax = r8;
    crate::simd_ops::wu64(r8.wrapping_add(P6), r9);
    crate::simd_ops::wu64(r8.wrapping_add(P7), r9);
    crate::simd_ops::wu64(r8.wrapping_add(P8), r9);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ce8790(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xfffffffffffffff8; const P1: u64 = 0x14327d778; const P2: u64 = 0x14327f1f8; const P3: u64 = 0xfffffffffffffff0; const P4: u64 = 0x160; const P5: u64 = 0x1401de830; const P6: u64 = 0x150; const P7: u64 = 0x1401ffd50; const P8: u64 = 0x90; const P9: u64 = 0x138; const P10: u64 = 0x140236fa0; const P11: u64 = 0x100; const P12: u64 = 0x14240ab10; const P13: u64 = 0x140260f30; const P14: u64 = 0x142442c80;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rsi = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rbp = P1; r14 = P2; bb = 3; continue; }
            3 => { rbx = rsi; rbx = (((rbx) as i64).wrapping_mul((crate::simd_ops::ru64(rdi.wrapping_add(P3))) as i64) as u64) as u64; rbx = ((rbx).wrapping_add(rdi)) as u64; rcx = ((rbx.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P8), rbp); rcx = ((rbx.wrapping_add(P9)) as u64) as u64; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P11)) as u64) as u64; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x90u64), r14); rcx = rbx.wrapping_add(0xb8); rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rsi = ((rsi).wrapping_add(1)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_sub(0x8u64))) as u32 as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rcx = rdi; rax = crate::simd_ops::call_stk(P14, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 5; continue; }
            5 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ce8d90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1406f27c0; const P2: u64 = 0x10; const P3: u64 = 0x20; const P4: u64 = 0x1406d6620;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = r8;
    rbx = rdx;
    fl_l = r8; fl_r = rdx;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rdx = (crate::simd_ops::ru64(rdx)) as u64; r8 = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; r8 = ((r8).wrapping_shl(5u32)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rax = ((rbx.wrapping_add(P2)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    fl_l = rcx; fl_r = rax;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rdx = (crate::simd_ops::ru64(rax)) as u64; r8 = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = ((r8).wrapping_shl(5u32)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rdx = ((rbx.wrapping_add(P3)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P3)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P4, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ce8e10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x1401de830; const P2: u64 = 0x10;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rcx = ((rdx.wrapping_add(P0)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ce8e80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xfffffffffffffff8; const P1: u64 = 0xfffffffffffffff0; const P2: u64 = 0x20; const P3: u64 = 0x1401de830; const P4: u64 = 0x10; const P5: u64 = 0x142442c80;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rsi = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rbx = rsi; rbx = (((rbx) as i64).wrapping_mul((crate::simd_ops::ru64(rdi.wrapping_add(P1))) as i64) as u64) as u64; rbx = ((rbx).wrapping_add(rdi)) as u64; rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rsi = ((rsi).wrapping_add(1)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 5; continue; }
            5 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ce8f00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x1401de830; const P2: u64 = 0x10; const P3: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rcx = ((rdx.wrapping_add(P0)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); }
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ce9fe0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x140b06960;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    crate::simd_ops::wu128(r8, (xmm0) as u128);
    rax = ((rdx.wrapping_add(P0)) as u64) as u64;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    fl_l = rcx; fl_r = rax;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(6u32)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cea6a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x8; const P2: u64 = 0x3; const P3: u64 = 0x90; const P4: u64 = 0x98; const P5: u64 = 0xa1; const P6: u64 = 0xa0; const P7: u64 = 0xb8; const P8: u64 = 0xa3;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::lk_and_u32(rdx, (0xff000010u32) as u32);
    rax = 0;
    fl_l = 0; fl_r = 0;
    fl_l = (crate::simd_ops::lk_or_u32(rdx, (0x10u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P1), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P2), (0x10u8) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rax);
    crate::simd_ops::wu16(rdx.wrapping_add(P5), ((rax as u16)) as u16);
    crate::simd_ops::wu8(rdx.wrapping_add(P6), ((rax as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rax);
    rax = rdx;
    crate::simd_ops::wu8(rdx.wrapping_add(P8), (0xffu8) as u8);
    return rax;
}
