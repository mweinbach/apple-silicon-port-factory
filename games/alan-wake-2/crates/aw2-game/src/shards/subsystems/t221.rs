#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn fn_141596980(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415c0bf0; const P1: u64 = 0x1415984e0; const P2: u64 = 0x1439315c4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x306u64))) as u32) as u32 as u64; rbx = rcx; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 13; continue; } bb = 1; continue; }
            1 => { rax = (rax & !0xffu64) | ((((((rax as u8)) | (0x4u8))) as u8 as u64)); crate::simd_ops::wu8(rcx.wrapping_add(0x306u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x368u64))) as u64; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(0x370u64))) as u32 as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rax)) as u64; rax = ((rax).wrapping_add(0x8u64)) as u64; crate::simd_ops::lk_add_u16(rcx.wrapping_add(0x2au64), (1) as u16); rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x370u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x368u64))) as u64; rdx = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x303u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((((fl_l as u8)) & ((fl_r as u8))) as i8) < 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x1d8u64))) as u64; rdx = rsp.wrapping_add(0x30); rax = (rax & !0xffu64) | ((((((rax as u8)) | (0x80u8))) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x303u64), ((rax as u8)) as u8); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x1d8u64))) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x288u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x240u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = 0; fl_l = 0; fl_r = 0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rdx = (rdx & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rdx = ((fl_l as u32)) as u32 as u64; bb = 7; continue; }
            7 => { r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rcx = (((rdx as u8)) as u32) as u32 as u64; crate::simd_ops::lk_and_u8(rbx.wrapping_add(0x303u64), (0x80u8) as u8); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0xfu8))) as u8 as u64)); rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_shl(3u32)) as u8 as u64)); rax = (((r8 as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) | ((rax as u8)))) as u8 as u64)); fl_l = (crate::simd_ops::lk_or_u8(rbx.wrapping_add(0x303u64), ((rcx as u8)) as u8)) as u8 as u64; fl_r = fl_l; bb = 9; continue; }
            8 => { rdx = ((rax as u32)) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; r8 = ((((r8 as u32)) & (0x7u32))) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(3u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r8 as u32)) as u32); fl_l = ((((rdx as u32)) & (0xfu32))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; bb = 9; continue; }
            9 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::lk_add_u32(P2, (1) as u32)) as u32 as u64; bb = 13; continue; }
            13 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141596a90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1d8; const P1: u64 = 0x1415ac5b0; const P2: u64 = 0x300; const P3: u64 = 0x304; const P4: u64 = 0x18; const P5: u64 = 0x141592aa0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu16(rbx.wrapping_add(P2), ((rax as u16)) as u16);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P3))) as u32) as u32 as u64;
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64;
    if !((((((fl_l as u8)) & ((fl_r as u8))) as i8) < 0)) { rax = (rax & !0xffu64) | ((((((rax as u8)) | (0x80u8))) as u8 as u64)); rcx = ((rbx.wrapping_add(P4)) as u64) as u64; crate::simd_ops::wu8(rbx.wrapping_add(P3), ((rax as u8)) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P5, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141596ae0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x304; const P1: u64 = 0x278; const P2: u64 = 0x1415ad830; const P3: u64 = 0x1c; const P4: u64 = 0x2b4; const P5: u64 = 0x118; const P6: u64 = 0x2; const P7: u64 = 0x14160c240; const P8: u64 = 0x143930f80; const P9: u64 = 0x1415add10; const P10: u64 = 0x18; const P11: u64 = 0x141592080; const P12: u64 = 0x1415c28a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64; rsi = (((rdx as u8)) as u32) as u32 as u64; rbx = rcx; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x40u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 12; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::lk_or_u8(rbx.wrapping_add(P0), (0x40u8) as u8); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(P4), ((rax as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(P5), ((rax as u32)) as u32); fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = rbx; rcx = ((rax.wrapping_add(P6)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(P8)) as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rcx = ((rbx.wrapping_add(P10)) as u64) as u64; rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x90u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141596ba0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141591570; const P1: u64 = 0x143930f80; const P2: u64 = 0x1415ad8b0; const P3: u64 = 0x1415ada30;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rsi = rcx; rdi = rdx; rcx = ((rcx).wrapping_add(0x18u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x324u64), (1) as u32); fl_l = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x280u64))) as u64; rcx = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rbx); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { xmm0 = ((crate::simd_ops::ru32(rsi.wrapping_add(0x2b4u64))) as u32) as u128; rax = rsi.wrapping_add(0x10); crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((xmm0 as u32)) as u32); rdx = rsp.wrapping_add(0x40); xmm0 = 0u128; crate::simd_ops::wu8(rsp.wrapping_add(0x66u64), ((rcx as u8)) as u8); fl_l = rsi; fl_r = rsi; crate::simd_ops::wu32(rsp.wrapping_add(0x54u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), r14); rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rcx } else { rax }) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (0x4000000u64) as u64); rcx = (crate::simd_ops::ru64(P1)) as u64; r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rax = rbx; rax = ((rax).wrapping_shr(32u32)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x5cu64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), ((rbx as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), (0x1du32) as u32); crate::simd_ops::wu16(rsp.wrapping_add(0x64u64), (0x100u16) as u16); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(rsi)) as u64; r8 = ((rbx as u32)) as u32 as u64; rdx = 0xe; crate::simd_ops::wu64(rsi.wrapping_add(0x280u64), rax); rcx = rsi; rdi = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(r9.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; rcx = rsi.wrapping_add(0x10); xmm2 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x54u64))) as u32) as u128; r9 = (r9 & !0xffu64) | (((0x1u8) as u8 as u64)); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            4 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; r11 = rsp.wrapping_add(0x70); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x84u64))) as u32 as u64; xmm3 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rcx as u32)) as u32); r8 = 0x4000000; rcx = (crate::simd_ops::ru64(P1)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rbx as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r11 = rsp.wrapping_add(0x70); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x302u64))) as u8 as u64; fl_r = (0x78u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = rsi.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rsi; r11 = rsp.wrapping_add(0x70); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            8 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x280u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = 0; fl_l = 0; fl_r = 0; xmm3 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rcx as u32)) as u32); r8 = 0x4000000; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x4u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru64(P1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r11 = rsp.wrapping_add(0x70); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            10 => { rcx = rsi.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); rcx = rsi; r11 = rsp.wrapping_add(0x70); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141596d80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141593d70; const P1: u64 = 0x14160c240; const P2: u64 = 0x143930f80; const P3: u64 = 0x1415add10; const P4: u64 = 0x141592080; const P5: u64 = 0x1415c28a0; const P6: u64 = 0x143931d40;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = 0x0; rsi = (((r9 as u8)) as u32) as u32 as u64; rdi = (((r8 as u8)) as u32) as u32 as u64; rbx = rcx; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rcx.wrapping_add(0x2b0u64), (0x0u32) as u32); r8 = 0x1000000; r9 = (crate::simd_ops::ru64(rdx)) as u64; rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x302u64))) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = rbx; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdi = ((rax as u32)) as u32 as u64; bb = 9; continue; }
            5 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x324u64))) as u32 as u64; rcx = 0x1; crate::simd_ops::lk_or_u8(rbx.wrapping_add(0x304u64), (0x40u8) as u8); rdx = rbx; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rax = (if (((fl_l as u32)) < ((fl_r as u32))) { (rcx as u32) } else { (rax as u32) }) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x324u64), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x278u64))) as u64; rdi = ((rax as u32)) as u32 as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(P2)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rcx = rbx.wrapping_add(0x18); rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x90u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 10; continue; }
            10 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x302u64))) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rax = ((rdi as u32)) as u32 as u64; fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 15; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::lk_add_u32(P6, (1) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            14 => { rax = ((rdi as u32)) as u32 as u64; bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141596e90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x304; const P1: u64 = 0x141592080; const P2: u64 = 0x90; const P3: u64 = 0x1415c2990; const P4: u64 = 0x14160c240; const P5: u64 = 0x278; const P6: u64 = 0x143930f80; const P7: u64 = 0x1415adf30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rbx = rcx;
    fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x10u8) as u8 as u64;
    if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x40u8) as u8 as u64;
    if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = ((rcx).wrapping_add(0x18u64)) as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    crate::simd_ops::lk_and_u8(rbx.wrapping_add(P0), (0xbfu8) as u8);
    rdx = rbx;
    rcx = 0x3;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = (crate::simd_ops::ru64(P6)) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P7, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141596f00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141591570; const P1: u64 = 0x143930f80; const P2: u64 = 0x1434268d0; const P3: u64 = 0x1415ad8b0; const P4: u64 = 0x1415ada30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rsi = rcx; rbx = (((r8 as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(0x18u64)) as u64; r14 = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rbx as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x324u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x324u64), ((rax as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rsp = ((rsp).wrapping_add(0x70u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rbp); rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsi.wrapping_add(0x324u64), ((rbp as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x280u64))) as u64; fl_l = (crate::simd_ops::ru32(r14)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(r14)) as u64; rcx = (crate::simd_ops::ru64(P1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rbx); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { xmm0 = ((crate::simd_ops::ru32(rsi.wrapping_add(0x2b4u64))) as u32) as u128; rax = rsi.wrapping_add(0x10); crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((xmm0 as u32)) as u32); rdx = rsp.wrapping_add(0x40); xmm0 = ((crate::simd_ops::ru32(P2)) as u32) as u128; fl_l = rsi; fl_r = rsi; crate::simd_ops::wu32(rsp.wrapping_add(0x54u64), ((xmm0 as u32)) as u32); rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbp } else { rax }) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (0x8000000u64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); r8 = 0; fl_l = 0; fl_r = 0; rax = rbx; crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), ((rbx as u32)) as u32); rax = ((rax).wrapping_shr(32u32)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x5cu64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), (0x1du32) as u32); crate::simd_ops::wu16(rsp.wrapping_add(0x64u64), (0x100u16) as u16); crate::simd_ops::wu8(rsp.wrapping_add(0x66u64), ((rbp as u8)) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(rsi)) as u64; rdx = ((rbp.wrapping_add(0xeu64)) as u32) as u32 as u64; r8 = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x280u64), rax); rcx = rsi; rdi = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(r9.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; rcx = rsi.wrapping_add(0x10); xmm2 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x54u64))) as u32) as u128; r9 = (r9 & !0xffu64) | (((0x1u8) as u8 as u64)); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            7 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x94u64))) as u32 as u64; r8 = 0x8000000; xmm3 = ((crate::simd_ops::ru32(P2)) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rbp as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rbx as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            8 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { xmm3 = ((crate::simd_ops::ru32(P2)) as u32) as u128; r8 = 0x8000000; rcx = (crate::simd_ops::ru64(P1)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rbp as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x4u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rbp as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            10 => { rcx = rsi.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; rcx = rsi.wrapping_add(0x18); crate::simd_ops::wu32(rsi.wrapping_add(0x2b4u64), (0x3f800000u32) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x78u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            11 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rsp = ((rsp).wrapping_add(0x70u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415970c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415ad830; const P1: u64 = 0x141593d70; const P2: u64 = 0x143930f80; const P3: u64 = 0x1415ada30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x304u64))) as u8 as u64; fl_r = (0x40u8) as u8 as u64; rsi = (((r8 as u8)) as u32) as u32 as u64; rdi = rdx; rbx = rcx; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x280u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 12; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::lk_or_u8(rbx.wrapping_add(0x304u64), (0x20u8) as u8); fl_l = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { r9 = (crate::simd_ops::ru64(rdi)) as u64; r8 = 0x2000000; rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); rcx = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            5 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x302u64))) as u8 as u64; fl_r = (0x78u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 13; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x278u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(P2)) as u64; rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); xmm3 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x4u32) as u32); r8 = 0x2000000; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            9 => { fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 14; continue; } bb = 10; continue; }
            10 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(0x2b0u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x118u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x305u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xf1u8))) as u8 as u64)); fl_l = ((((rax as u8)) | (0x1u8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x305u64), ((rax as u8)) as u8); bb = 12; continue; }
            12 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415971d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f80; const P1: u64 = 0x1415adef0; const P2: u64 = 0x14159f2a0; const P3: u64 = 0x1415242e0; const P4: u64 = 0x143930fa8; const P5: u64 = 0x1415f81d0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x304u64))) as u32) as u32 as u64; rbx = rcx; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 20; continue; } bb = 1; continue; }
            1 => { rax = (rax & !0xffu64) | ((((((rax as u8)) | (0x10u8))) as u8 as u64)); crate::simd_ops::wu8(rcx.wrapping_add(0x304u64), ((rax as u8)) as u8); fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xfffffffd; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 20; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x278u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); rdi = rcx.wrapping_add(0x10); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = rcx; fl_r = rcx; r8 = rdi; rcx = (crate::simd_ops::ru64(P0)) as u64; r8 = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbp } else { r8 }) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x278u64), rbp); bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x280u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(P0)) as u64; fl_l = rbx; fl_r = rbx; rdi = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbp } else { rdi }) as u64; r8 = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x280u64), rbp); bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x158u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru32(rax.wrapping_add(0x28u64))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 15; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x1d8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rsi = rax; rdi = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 9; continue; }
            9 => { rcx = ((crate::simd_ops::ru16(rdi.wrapping_add(0x10u64))) as u32) as u32 as u64; fl_l = ((rcx as u16)) as u16 as u64; fl_r = (0x1u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 13; continue; } bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbp as u32)) as u32); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rbp); crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((rbp as u32)) as u32); bb = 12; continue; }
            12 => { rdx = rdi; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            13 => { fl_l = (((rcx as u16)).wrapping_sub(1)) as u16 as u64; rcx = (rcx & !0xffffu64) | ((((fl_l as u16)) as u16 as u64)); crate::simd_ops::wu16(rdi.wrapping_add(0x10u64), ((rcx as u16)) as u16); bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rbp); crate::simd_ops::wu32(rsi, ((rbp as u32)) as u32); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 15; continue; }
            15 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x158u64))) as u64; r8 = rbx.wrapping_add(0x18); rcx = (crate::simd_ops::ru64(P4)) as u64; fl_l = rbx; fl_r = rbx; r8 = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbp } else { r8 }) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x158u64), rbp); crate::simd_ops::wu32(rbx.wrapping_add(0x160u64), ((rbp as u32)) as u32); bb = 16; continue; }
            16 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x304u64))) as u32) as u32 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x40u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 19; continue; } bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x378u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 19; continue; } bb = 18; continue; }
            18 => { crate::simd_ops::wu32(rbx.wrapping_add(0x378u64), (0xffffffffu32) as u32); bb = 19; continue; }
            19 => { rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xfeu8))) as u8 as u64)); fl_l = ((((rax as u8)) | (0x4u8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x304u64), ((rax as u8)) as u8); bb = 20; continue; }
            20 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141597340(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x1439315d8; const P2: u64 = 0x141597370;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    r8 = rsp.wrapping_add(0x30);
    rbx = rcx;
    rcx = P1;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { crate::simd_ops::wu64(rax, rbx); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141597370(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14153e940; const P1: u64 = 0x14183acc8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r9 = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; r10 = 0; fl_l = 0; fl_r = 0; rdi = r8; rbx = rcx; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { r11 = (crate::simd_ops::ru64(rcx)) as u64; bb = 2; continue; }
            2 => { rcx = ((r9 as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(1u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_add((r10 as u32))) as u32 as u64; rax = ((((rcx as u32)) as i32 as i64) as u64) as u64; r8 = ((rax.wrapping_mul(8)) as u64) as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(r11))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 3; continue; }
            3 => { rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; fl_l = rax; fl_r = rdx; rcx = (if (((fl_l as u64)) <= ((fl_r as u64))) { (r10 as u32) } else { (rcx as u32) }) as u32 as u64; r10 = ((rcx as u32)) as u32 as u64; rcx = ((r9.wrapping_sub(0x1u64)) as u32) as u32 as u64; rcx = (if (((fl_l as u64)) <= ((fl_r as u64))) { (r9 as u32) } else { (rcx as u32) }) as u32 as u64; r9 = ((rcx as u32)) as u32 as u64; fl_l = (((r9 as u32)).wrapping_shr(1u32)) as u32 as u64; r9 = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu8(rdi, (0x0u8) as u8); rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = ((((r10 as u32)) as i32 as i64) as u64) as u64; rdi = ((rcx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; bb = 5; continue; }
            5 => { rsi = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 6; continue; }
            6 => { rdi = ((rdi).wrapping_sub(rcx)) as u64; rdi = ((((rdi) as i64).wrapping_shr(3u32) as u64)) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 17; continue; } bb = 8; continue; }
            8 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 17; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rax = ((rdi as u32)) as u32 as u64; rsi = ((rax.wrapping_mul(8)) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rdx = ((rdx).wrapping_add(rsi)) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (((rax as u32)).wrapping_sub((rdi as u32))) as u32 as u64; rcx = rdx.wrapping_add(0x8); rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x8u64), (1) as u32); rax = (crate::simd_ops::ru64(rbx)) as u64; rax = ((rax).wrapping_add(rsi)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            12 => { crate::simd_ops::wu8(rdi, (0x1u8) as u8); rcx = (crate::simd_ops::ru64(rbx)) as u64; rdi = ((r8.wrapping_add(rcx)) as u64) as u64; bb = 5; continue; }
            13 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 17; continue; } bb = 15; continue; }
            15 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rbx)) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            17 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415974b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1439315f8; const P1: u64 = 0x40; const P2: u64 = 0x1439315e8; const P3: u64 = 0x1439315f0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(P0)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { bb = 6; continue; }
            5 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rcx.wrapping_add(P1), (0x0u64) as u64); fl_l = (crate::simd_ops::ru64(P0)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::lk_add_u32(P2, (1) as u32)) as u32 as u64; crate::simd_ops::wu64(P0, rcx); crate::simd_ops::wu64(P3, rcx); return rax; }
            8 => { rax = (crate::simd_ops::ru64(P3)) as u64; crate::simd_ops::wu64(rax.wrapping_add(P1), rcx); fl_l = (crate::simd_ops::lk_add_u32(P2, (1) as u32)) as u32 as u64; crate::simd_ops::wu64(P3, rcx); bb = 9; continue; }
            9 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141597520(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x305; const P1: u64 = 0x58; const P2: u64 = 0x8; const P3: u64 = 0x1415242e0; const P4: u64 = 0x110;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rdi = rdx; r10 = rcx; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_or_u8(rax.wrapping_add(P0), (0x10u8) as u8)) as u8 as u64; fl_r = fl_l; bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rdx)) as u64; rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); rcx = ((rcx).wrapping_add(0x18u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbx; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi, rcx); rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { xmm0 = 0u128; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rsp.wrapping_add(0x20); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu128(rsp.wrapping_add(0x28u64), (xmm0) as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), (0x1u32) as u32); crate::simd_ops::wu16(rsp.wrapping_add(0x48u64), (0x100u16) as u16); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415975f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931690; const P1: u64 = 0x141594c30; const P2: u64 = 0x378; const P3: u64 = 0x1415970c0; const P4: u64 = 0x230; const P5: u64 = 0x1415efd80;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(P0)) as u64; rdi = rcx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rsi = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rdx = rdi; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(P2), ((rsi as u32)) as u32); rdx = rsp.wrapping_add(0x38); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rsi as u32)) as u32); rcx = rbx; crate::simd_ops::wu32(rsp.wrapping_add(0x3cu64), (0x4u32) as u32); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 5; continue; }
            5 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 6; continue; }
            6 => { rcx = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P5, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141597670(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x230; const P1: u64 = 0x143931690; const P2: u64 = 0x143931680; const P3: u64 = 0x143931688;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), (0x0u64) as u64);
    fl_l = (crate::simd_ops::ru64(P1)) as u64; fl_r = (0x0u64) as u64;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { fl_l = (crate::simd_ops::lk_add_u32(P2, (1) as u32)) as u32 as u64; crate::simd_ops::wu64(P1, rcx); crate::simd_ops::wu64(P3, rcx); return rax; }
    rax = (crate::simd_ops::ru64(P3)) as u64;
    crate::simd_ops::wu64(rax.wrapping_add(P0), rcx);
    fl_l = (crate::simd_ops::lk_add_u32(P2, (1) as u32)) as u32 as u64;
    crate::simd_ops::wu64(P3, rcx);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415976c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931658; const P1: u64 = 0x1426452b0; const P2: u64 = 0x143931640; const P3: u64 = 0x141524330; const P4: u64 = 0x143931638; const P5: u64 = 0x143931630; const P6: u64 = 0x143931654; const P7: u64 = 0x141595250; const P8: u64 = 0x143931690; const P9: u64 = 0x143931688; const P10: u64 = 0x143931680; const P11: u64 = 0x14160c0e0; const P12: u64 = 0x1415242e0; const P13: u64 = 0x143931648; const P14: u64 = 0x143931650; const P15: u64 = 0x1426452a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbp = rcx; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); rcx = P0; xmm6 = (xmm3) as u128; rdi = ((r8 as u32)) as u32 as u64; rsi = ((rdx as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(P2)) as u64; r14 = 0; fl_l = 0; fl_r = 0; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rdx = ((r14.wrapping_add(0x20u64)) as u32) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(P2)) as u64; crate::simd_ops::wu64(rax, rcx); crate::simd_ops::wu64(P2, rax); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(P4)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(P5, r9); bb = 6; continue; }
            5 => { crate::simd_ops::wu64(rax, r9); r9 = (crate::simd_ops::ru64(P2)) as u64; bb = 6; continue; }
            6 => { crate::simd_ops::wu64(P4, r9); rax = (crate::simd_ops::ru64(r9)) as u64; crate::simd_ops::wu64(P2, rax); crate::simd_ops::wu64(r9, r14); crate::simd_ops::lk_add_u32(P6, (1) as u32); rax = (crate::simd_ops::ru64(P4)) as u64; fl_l = ((rax).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) != 0) { bb = 38; continue; } bb = 7; continue; }
            7 => { rcx = P0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(P6)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 30; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rbx = (crate::simd_ops::ru64(P5)) as u64; bb = 9; continue; }
            9 => { xmm3 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32) as u128; r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x14u64))) as u32 as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 21; continue; } bb = 10; continue; }
            10 => { r8 = (crate::simd_ops::ru64(P8)) as u64; rdx = r14; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = r8; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 11; continue; }
            11 => { fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 12; continue; }
            12 => { rdx = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x230u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 13; continue; }
            13 => { bb = 19; continue; }
            14 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x230u64))) as u64; fl_l = rcx; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu64(P8, rax); bb = 18; continue; }
            17 => { crate::simd_ops::wu64(rdx.wrapping_add(0x230u64), rax); bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(P9)) as u64; fl_l = rcx; fl_r = rax; rax = (if (((fl_l as u64)) == ((fl_r as u64))) { rdx } else { rax }) as u64; fl_l = (crate::simd_ops::lk_sub_u32(P10, (1) as u32)) as u32 as u64; crate::simd_ops::wu64(P9, rax); bb = 19; continue; }
            19 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rcx = rbx.wrapping_add(0x18); rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x58u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbx; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 21; continue; }
            21 => { rbx = (crate::simd_ops::ru64(P5)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru64(P13)) as u64; crate::simd_ops::wu64(P5, rax); rax = (crate::simd_ops::ru64(P4)) as u64; fl_l = rbx; fl_r = rax; rax = (if (((fl_l as u64)) == ((fl_r as u64))) { r14 } else { rax }) as u64; crate::simd_ops::wu64(P4, rax); fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 25; continue; } bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru32(P14)) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 25; continue; } bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru64(P2)) as u64; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(P2, rbx); bb = 26; continue; }
            25 => { rdx = rbx; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru32(P6)) as u32 as u64; rbx = (crate::simd_ops::ru64(P5)) as u64; fl_l = (((rax as u32)).wrapping_sub(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(P6, ((rax as u32)) as u32); bb = 28; continue; }
            27 => { rax = (crate::simd_ops::ru32(P6)) as u32 as u64; bb = 28; continue; }
            28 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 9; continue; } bb = 29; continue; }
            29 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 30; continue; }
            30 => { rcx = P0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P15), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(P2)) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 33; continue; } bb = 31; continue; }
            31 => { rdx = ((r8.wrapping_add(0x20u64)) as u32) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 35; continue; } bb = 32; continue; }
            32 => { rcx = (crate::simd_ops::ru64(P2)) as u64; crate::simd_ops::wu64(rax, rcx); crate::simd_ops::wu64(P2, rax); bb = 33; continue; }
            33 => { rax = (crate::simd_ops::ru64(P4)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 36; continue; } bb = 34; continue; }
            34 => { crate::simd_ops::wu64(P5, r8); bb = 37; continue; }
            35 => { rax = r14; bb = 38; continue; }
            36 => { crate::simd_ops::wu64(rax, r8); r8 = (crate::simd_ops::ru64(P2)) as u64; bb = 37; continue; }
            37 => { crate::simd_ops::wu64(P4, r8); rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(P2, rax); crate::simd_ops::wu64(r8, r14); crate::simd_ops::lk_add_u32(P6, (1) as u32); rax = (crate::simd_ops::ru64(P4)) as u64; fl_l = ((rax).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 38; continue; }
            38 => { rcx = P0; crate::simd_ops::wu32(rax.wrapping_add(0x10u64), ((xmm6 as u32)) as u32); crate::simd_ops::wu64(rax, rbp); crate::simd_ops::wu32(rax.wrapping_add(0x8u64), ((rsi as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(0xcu64), ((rdi as u32)) as u32); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P15), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415979b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931690; const P1: u64 = 0x143426890; const P2: u64 = 0x141599420; const P3: u64 = 0x141524330; const P4: u64 = 0x1415242e0; const P5: u64 = 0x141598310;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); rbx = (crate::simd_ops::ru64(P0)) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rsi); crate::simd_ops::wu64(rax.wrapping_sub(0x18u64), rdi); crate::simd_ops::wu64(rax.wrapping_sub(0x20u64), r13); r13 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rax.wrapping_sub(0x28u64), r15); r15 = rcx; rsi = ((r13 as u32)) as u32 as u64; r14 = ((r13 as u32)) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_sub(0x50u64), ((rsi as u32)) as u32); r12 = ((r13 as u32)) as u32 as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x38u64), (xmm6) as u128); crate::simd_ops::wu64(rax.wrapping_sub(0x58u64), r13); crate::simd_ops::wu64(rax.wrapping_sub(0x4cu64), r13); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 1; continue; }
            1 => { xmm6 = ((crate::simd_ops::ru32(P1)) as u32) as u128; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(0x58u64))) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 3; continue; }
            3 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rdi as u32)) as u32 as u64; let d = ((rsi as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(r14.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 7; continue; }
            7 => { rbp = ((rdi as u32)) as u32 as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x1f8u64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) > f32::from_bits(crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) }).to_bits() as u128); crate::simd_ops::wu32(rax.wrapping_add(0x10u64), ((xmm0 as u32)) as u32); bb = 24; continue; }
            9 => { rbp = ((rdi as u32)) as u32 as u64; bb = 10; continue; }
            10 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = ((r12 as u32)) as u32 as u64; xmm1 = 0u128; xmm0 = 0u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); rax = ((rsi as u32)) as u32 as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rdx = ((rsi as u32)) as u32 as u64; rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r12 = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rsi = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 13; continue; }
            13 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 29; continue; } bb = 14; continue; }
            14 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbp as u32)) as u32 as u64; let d = ((rsi as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rbp = ((rdx as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 15; continue; } bb = 17; continue; }
            17 => { bb = 19; continue; }
            18 => { rcx = rax.wrapping_add(0x10); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 23; continue; } bb = 19; continue; }
            19 => { rdx = 0x18; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { rcx = r13; bb = 22; continue; }
            21 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(rbp.wrapping_mul(8)))) as u64; r12 = (((r12 as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rdi); crate::simd_ops::wu64(r14.wrapping_add(rbp.wrapping_mul(8)), rcx); fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r12 as u32)) as u32); bb = 22; continue; }
            22 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x1f8u64))) as u32 as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); bb = 24; continue; }
            24 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x230u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 25; continue; }
            25 => { rdx = (crate::simd_ops::ru64(r15)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 26; continue; }
            26 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(r15.wrapping_add(0x8u64), ((r13 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(r15, r13); crate::simd_ops::wu32(r15.wrapping_add(0xcu64), ((r13 as u32)) as u32); bb = 27; continue; }
            27 => { fl_l = ((r12 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r15.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 30; continue; } bb = 28; continue; }
            28 => { r13 = 0x1; r12 = ((r13 as u32)) as u32 as u64; bb = 53; continue; }
            29 => { r12 = 0x34; bb = 72; continue; }
            30 => { rbx = (crate::simd_ops::ru32(r15.wrapping_add(0x8u64))) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rdx = ((r12 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 52; continue; } bb = 31; continue; }
            31 => { rax = (crate::simd_ops::ru64(r15)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 51; continue; } bb = 32; continue; }
            32 => { fl_l = rax; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 51; continue; } bb = 33; continue; }
            33 => { r11 = r13; fl_l = rbx; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 45; continue; } bb = 34; continue; }
            34 => { r13 = 0x10; r10 = rbx.wrapping_sub(0x4); rbp = 0x20; r10 = ((r10).wrapping_shr(2u32)) as u64; r14 = (0xfffffffffffffff0u64) as u64; rdx = rdi.wrapping_add(0x10); r13 = ((r13).wrapping_sub(rdi)) as u64; rbp = ((rbp).wrapping_sub(rdi)) as u64; r14 = ((r14).wrapping_sub(rdi)) as u64; fl_l = ((r10).wrapping_add(1)) as u64; r10 = ((fl_l as u64)) as u64; r11 = ((r10.wrapping_mul(4)) as u64) as u64; bb = 35; continue; }
            35 => { rcx = rdx.wrapping_sub(0x10); r8 = rdx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 37; continue; } bb = 36; continue; }
            36 => { crate::simd_ops::wu64(rcx, (0xffffffffffffffffu64) as u64); crate::simd_ops::wu32(rdx.wrapping_sub(0x8u64), (0xbf800000u32) as u32); bb = 37; continue; }
            37 => { rax = (crate::simd_ops::ru64(r15)) as u64; r9 = ((r14.wrapping_add(rdx)) as u64) as u64; xmm0 = (crate::simd_ops::ru128(r9.wrapping_add(rax))) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 39; continue; } bb = 38; continue; }
            38 => { crate::simd_ops::wu64(rdx, (0xffffffffffffffffu64) as u64); crate::simd_ops::wu32(r8.wrapping_add(0x8u64), (0xbf800000u32) as u32); bb = 39; continue; }
            39 => { rax = (crate::simd_ops::ru64(r15)) as u64; r8 = rdx.wrapping_add(0x10); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(r9).wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 41; continue; } bb = 40; continue; }
            40 => { crate::simd_ops::wu64(r8, (0xffffffffffffffffu64) as u64); crate::simd_ops::wu32(rdx.wrapping_add(0x18u64), (0xbf800000u32) as u32); bb = 41; continue; }
            41 => { rax = (crate::simd_ops::ru64(r15)) as u64; rcx = ((rdx.wrapping_add(r13)) as u64) as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rax))) as u128; crate::simd_ops::wu128(r8, (xmm0) as u128); r8 = rdx.wrapping_add(0x20); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 43; continue; } bb = 42; continue; }
            42 => { crate::simd_ops::wu64(r8, (0xffffffffffffffffu64) as u64); crate::simd_ops::wu32(rdx.wrapping_add(0x28u64), (0xbf800000u32) as u32); bb = 43; continue; }
            43 => { rax = (crate::simd_ops::ru64(r15)) as u64; rcx = ((rdx.wrapping_add(rbp)) as u64) as u64; rdx = ((rdx).wrapping_add(0x40u64)) as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rax))) as u128; crate::simd_ops::wu128(r8, (xmm0) as u128); fl_l = r10; fl_r = (0x1u64) as u64; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 35; continue; } bb = 44; continue; }
            44 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 45; continue; }
            45 => { fl_l = r11; fl_r = rbx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 50; continue; } bb = 46; continue; }
            46 => { rcx = r11; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(rdi)) as u64; fl_l = rbx; fl_r = r11; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 47; continue; }
            47 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 49; continue; } bb = 48; continue; }
            48 => { crate::simd_ops::wu64(rcx, (0xffffffffffffffffu64) as u64); crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), (0xbf800000u32) as u32); bb = 49; continue; }
            49 => { rax = (crate::simd_ops::ru64(r15)) as u64; rax = ((rax).wrapping_sub(rdi)) as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(rcx))) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); rcx = ((rcx).wrapping_add(0x10u64)) as u64; fl_l = rbx; fl_r = (0x1u64) as u64; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 47; continue; } bb = 50; continue; }
            50 => { rdx = (crate::simd_ops::ru64(r15)) as u64; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 51; continue; }
            51 => { crate::simd_ops::wu64(r15, rdi); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(r15.wrapping_add(0xcu64), ((r12 as u32)) as u32); bb = 52; continue; }
            52 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; r12 = 0x34; r13 = 0x1; r12 = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (r13 as u32) } else { (r12 as u32) }) as u32 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 71; continue; } bb = 53; continue; }
            53 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 71; continue; } bb = 54; continue; }
            54 => { rdi = (crate::simd_ops::ru64(r14)) as u64; rbx = 0; fl_l = 0; fl_r = 0; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 60; continue; } bb = 55; continue; }
            55 => { rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 58; continue; } bb = 56; continue; }
            56 => { rdi = (crate::simd_ops::ru64(r14.wrapping_add(rbx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 55; continue; } bb = 57; continue; }
            57 => { bb = 60; continue; }
            58 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 71; continue; } bb = 59; continue; }
            59 => { bb = 60; continue; }
            60 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rbp = (crate::simd_ops::ru32(r15.wrapping_add(0x8u64))) as u32 as u64; xmm0 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32) as u128; rcx = (crate::simd_ops::ru32(r15.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = ((rbp as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((xmm0 as u32)) as u32); fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 63; continue; } bb = 61; continue; }
            61 => { rdx = ((rcx as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rcx as u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rcx = r15; rdx = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (r13 as u32) } else { (rdx as u32) }) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 66; continue; } bb = 62; continue; }
            62 => { rax = (crate::simd_ops::ru32(r15.wrapping_add(0x8u64))) as u32 as u64; bb = 63; continue; }
            63 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r15.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 66; continue; } bb = 64; continue; }
            64 => { rcx = ((rax as u32)) as u32 as u64; rax = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; crate::simd_ops::wu32(r15.wrapping_add(0x8u64), ((rax as u32)) as u32); fl_l = ((rcx).wrapping_add(crate::simd_ops::ru64(r15))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 66; continue; } bb = 65; continue; }
            65 => { xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu64(rcx, (0xffffffffffffffffu64) as u64); crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), (0xbf800000u32) as u32); crate::simd_ops::wu128(rcx, (xmm0) as u128); bb = 66; continue; }
            66 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 70; continue; } bb = 67; continue; }
            67 => { rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 70; continue; } bb = 68; continue; }
            68 => { rdi = (crate::simd_ops::ru64(r14.wrapping_add(rbx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 67; continue; } bb = 69; continue; }
            69 => { bb = 60; continue; }
            70 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 60; continue; } bb = 71; continue; }
            71 => { r13 = 0; fl_l = 0; fl_r = 0; bb = 72; continue; }
            72 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 78; continue; } bb = 73; continue; }
            73 => { rdi = r14; bb = 74; continue; }
            74 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 77; continue; } bb = 75; continue; }
            75 => { bb = 76; continue; }
            76 => { rbx = (crate::simd_ops::ru64(rdx)) as u64; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 76; continue; } bb = 77; continue; }
            77 => { crate::simd_ops::wu64(rdi, r13); rdi = ((rdi).wrapping_add(0x8u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 74; continue; } bb = 78; continue; }
            78 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 80; continue; } bb = 79; continue; }
            79 => { rdx = r14; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 80; continue; }
            80 => { rax = ((r12 as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x68u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141597e30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f78; const P1: u64 = 0x143427080; const P2: u64 = 0x1415f3790; const P3: u64 = 0x143931690; const P4: u64 = 0x20; const P5: u64 = 0x1f8; const P6: u64 = 0x230;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x38u64)) as u64; rdx = rcx; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); rcx = (crate::simd_ops::ru64(P0)) as u64; xmm6 = ((crate::simd_ops::ru32(P1)) as u32) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(P3)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(P5))) as u32) as u128; fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { xmm6 = (xmm0) as u128; bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P6))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 6; continue; }
            6 => { xmm0 = (xmm6) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x38u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141597e90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142689164; const P1: u64 = 0x1434268d0; const P2: u64 = 0x1415d7950;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0; let mut xmm11: u128 = 0; let mut xmm12: u128 = 0; let mut xmm13: u128 = 0; let mut xmm14: u128 = 0; let mut xmm15: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 37>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x110u64)) as u64; rbx = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; rsi = r8; rax = (crate::simd_ops::ru32(r8.wrapping_add(0x18u64))) as u32 as u64; rdi = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm15) as u128); rbp = rcx; xmm15 = ((crate::simd_ops::ru32(P0)) as u32) as u128; r9 = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu128(r11.wrapping_sub(0x28u64), (xmm6) as u128); crate::simd_ops::wu128(r11.wrapping_sub(0x38u64), (xmm7) as u128); xmm7 = ((crate::simd_ops::ru32(P1)) as u32) as u128; crate::simd_ops::wu128(r11.wrapping_sub(0x48u64), (xmm8) as u128); crate::simd_ops::wu128(r11.wrapping_sub(0x58u64), (xmm9) as u128); crate::simd_ops::wu128(r11.wrapping_sub(0x68u64), (xmm10) as u128); crate::simd_ops::wu128(r11.wrapping_sub(0x78u64), (xmm11) as u128); crate::simd_ops::wu128(r11.wrapping_sub(0x88u64), (xmm12) as u128); crate::simd_ops::wu128(r11.wrapping_sub(0x98u64), (xmm13) as u128); crate::simd_ops::wu128(r11.wrapping_sub(0xa8u64), (xmm14) as u128); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 3; continue; }
            3 => { xmm4 = (crate::simd_ops::ru128(rax.wrapping_add(0x78u64))) as u128; rdx = 0; fl_l = 0; fl_r = 0; xmm5 = ((crate::simd_ops::ru64(rax.wrapping_add(0x88u64))) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), ((xmm5 as u64)) as u64); xmm6 = (xmm4) as u128; xmm6 = crate::simd_ops::vunpckhpd(xmm6, xmm4); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm6) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm4) as u128); fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 9; continue; } bb = 4; continue; }
            4 => { xmm1 = ((crate::simd_ops::ru32(rax.wrapping_add(0x90u64))) as u32) as u128; rcx = rbp.wrapping_add(0x50); rax = ((rdi.wrapping_sub(0x4u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x148u64), ((xmm1 as u32)) as u32); rax = (((rax as u32)).wrapping_shr(2u32)) as u32 as u64; xmm0 = (xmm7) as u128; fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(xmm1 as u32)).to_bits() as u128); r9 = ((rax as u32)) as u32 as u64; rdx = ((rax.wrapping_mul(4)) as u32) as u32 as u64; rax = ((rax.wrapping_mul(4)) as u64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x140u64), ((xmm0 as u32)) as u32); bb = 6; continue; }
            5 => { xmm5 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64) as u128; bb = 6; continue; }
            6 => { xmm2 = (crate::simd_ops::ru128(rcx.wrapping_sub(0x38u64))) as u128; xmm3 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x140u64))) as u32) as u128; xmm0 = (xmm4) as u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) - f64::from_bits(xmm2 as u64)).to_bits() as u128); xmm1 = (xmm6) as u128; xmm2 = crate::simd_ops::vunpckhpd(xmm2, xmm2); xmm11 = 0u128; xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) - f64::from_bits(xmm2 as u64)).to_bits() as u128); xmm13 = 0u128; xmm2 = (crate::simd_ops::ru128(rcx)) as u128; xmm7 = 0u128; xmm14 = 0u128; xmm14 = (xmm14 & !0xffff_ffffu128) | (((f64::from_bits(xmm0 as u64)) as f32).to_bits() as u128); xmm13 = (xmm13 & !0xffff_ffffu128) | (((f64::from_bits(xmm1 as u64)) as f32).to_bits() as u128); xmm0 = (xmm5) as u128; xmm14 = (xmm14 & !0xffff_ffffu128) | ((f32::from_bits(xmm14 as u32) * f32::from_bits(xmm14 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) - f64::from_bits(crate::simd_ops::ru64(rcx.wrapping_sub(0x28u64)))).to_bits() as u128); xmm1 = (xmm6) as u128; xmm13 = (xmm13 & !0xffff_ffffu128) | ((f32::from_bits(xmm13 as u32) * f32::from_bits(xmm13 as u32)).to_bits() as u128); xmm8 = 0u128; xmm10 = 0u128; xmm12 = crate::simd_ops::vcvtpd2ps(xmm0); xmm0 = (xmm4) as u128; xmm13 = (xmm13 & !0xffff_ffffu128) | ((f32::from_bits(xmm13 as u32) + f32::from_bits(xmm14 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) - f64::from_bits(xmm2 as u64)).to_bits() as u128); xmm12 = (xmm12 & !0xffff_ffffu128) | ((f32::from_bits(xmm12 as u32) * f32::from_bits(xmm12 as u32)).to_bits() as u128); xmm2 = crate::simd_ops::vunpckhpd(xmm2, xmm2); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) - f64::from_bits(xmm2 as u64)).to_bits() as u128); xmm2 = (crate::simd_ops::ru128(rcx.wrapping_add(0x38u64))) as u128; xmm11 = (xmm11 & !0xffff_ffffu128) | (((f64::from_bits(xmm0 as u64)) as f32).to_bits() as u128); xmm0 = (xmm5) as u128; xmm13 = (xmm13 & !0xffff_ffffu128) | ((f32::from_bits(xmm13 as u32) + f32::from_bits(xmm12 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) - f64::from_bits(crate::simd_ops::ru64(rcx.wrapping_add(0x10u64)))).to_bits() as u128); xmm10 = (xmm10 & !0xffff_ffffu128) | (((f64::from_bits(xmm1 as u64)) as f32).to_bits() as u128); xmm1 = (xmm6) as u128; xmm11 = (xmm11 & !0xffff_ffffu128) | ((f32::from_bits(xmm11 as u32) * f32::from_bits(xmm11 as u32)).to_bits() as u128); xmm9 = crate::simd_ops::vcvtpd2ps(xmm0); xmm0 = (xmm4) as u128; xmm10 = (xmm10 & !0xffff_ffffu128) | ((f32::from_bits(xmm10 as u32) * f32::from_bits(xmm10 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) - f64::from_bits(xmm2 as u64)).to_bits() as u128); xmm9 = (xmm9 & !0xffff_ffffu128) | ((f32::from_bits(xmm9 as u32) * f32::from_bits(xmm9 as u32)).to_bits() as u128); xmm2 = crate::simd_ops::vunpckhpd(xmm2, xmm2); xmm11 = (xmm11 & !0xffff_ffffu128) | ((f32::from_bits(xmm11 as u32) + f32::from_bits(xmm10 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) - f64::from_bits(xmm2 as u64)).to_bits() as u128); xmm2 = (crate::simd_ops::ru128(rcx.wrapping_add(0x70u64))) as u128; xmm8 = (xmm8 & !0xffff_ffffu128) | (((f64::from_bits(xmm0 as u64)) as f32).to_bits() as u128); xmm0 = (xmm5) as u128; xmm11 = (xmm11 & !0xffff_ffffu128) | ((f32::from_bits(xmm11 as u32) + f32::from_bits(xmm9 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) - f64::from_bits(crate::simd_ops::ru64(rcx.wrapping_add(0x48u64)))).to_bits() as u128); xmm7 = (xmm7 & !0xffff_ffffu128) | (((f64::from_bits(xmm1 as u64)) as f32).to_bits() as u128); xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64) as u128; xmm5 = 0u128; xmm6 = crate::simd_ops::vcvtpd2ps(xmm0); xmm0 = (xmm4) as u128; xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) * f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) - f64::from_bits(xmm2 as u64)).to_bits() as u128); xmm8 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm2 = crate::simd_ops::vunpckhpd(xmm2, xmm2); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) - f64::from_bits(xmm2 as u64)).to_bits() as u128); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm2 = 0u128; xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) + f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | (((f64::from_bits(xmm0 as u64)) as f32).to_bits() as u128); xmm0 = ((crate::simd_ops::ru64(r8.wrapping_add(0x88u64))) as u64) as u128; xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) + f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) - f64::from_bits(crate::simd_ops::ru64(rcx.wrapping_add(0x80u64)))).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm13 as u32)).sqrt().to_bits() as u128); xmm4 = crate::simd_ops::vcvtpd2ps(xmm1); xmm1 = crate::simd_ops::vcvtpd2ps(xmm0); xmm0 = 0u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm11 as u32)).sqrt().to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) < f32::from_bits(xmm15 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm15 as u32) }).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) < f32::from_bits(xmm0 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm0 as u32) }).to_bits() as u128); xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32)).sqrt().to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) < f32::from_bits(xmm0 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm0 as u32) }).to_bits() as u128); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) + f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm0 = 0u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); rcx = ((rcx).wrapping_add(0xe0u64)) as u64; xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32)).sqrt().to_bits() as u128); xmm4 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) < f32::from_bits(xmm0 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm0 as u32) }).to_bits() as u128); xmm15 = (xmm2) as u128; fl_l = r9; fl_r = (0x1u64) as u64; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 7; continue; }
            7 => { xmm7 = ((crate::simd_ops::ru32(P1)) as u32) as u128; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 14; continue; } bb = 8; continue; }
            8 => { xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x148u64))) as u32) as u128; bb = 11; continue; }
            9 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 10; continue; }
            10 => { xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(0x90u64))) as u32) as u128; rax = 0; fl_l = 0; fl_r = 0; bb = 11; continue; }
            11 => { rcx = (((rax) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); rax = ((rdi as u32)) as u32 as u64; rcx = ((rcx).wrapping_add(0x18u64)) as u64; rcx = ((rcx).wrapping_add(rbp)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; bb = 12; continue; }
            12 => { xmm2 = (crate::simd_ops::ru128(rcx)) as u128; xmm0 = ((crate::simd_ops::ru64(r8.wrapping_add(0x88u64))) as u64) as u128; xmm1 = (xmm6) as u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) - f64::from_bits(crate::simd_ops::ru64(rcx.wrapping_add(0x10u64)))).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm4 as u64) - f64::from_bits(xmm2 as u64)).to_bits() as u128); xmm5 = 0u128; xmm2 = crate::simd_ops::vunpckhpd(xmm2, xmm2); rcx = ((rcx).wrapping_add(0x38u64)) as u64; xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) - f64::from_bits(xmm2 as u64)).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | (((f64::from_bits(xmm4 as u64)) as f32).to_bits() as u128); xmm4 = 0u128; xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | (((f64::from_bits(xmm1 as u64)) as f32).to_bits() as u128); xmm1 = crate::simd_ops::vcvtpd2ps(xmm0); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm0 = 0u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) + f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32)).sqrt().to_bits() as u128); xmm4 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) < f32::from_bits(xmm15 as u32) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(xmm15 as u32) }).to_bits() as u128); xmm15 = (xmm0) as u128; fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 13; continue; }
            13 => { xmm7 = ((crate::simd_ops::ru32(P1)) as u32) as u128; bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 15; continue; }
            15 => { xmm14 = (crate::simd_ops::ru128(rsp.wrapping_add(0x80u64))) as u128; xmm13 = (crate::simd_ops::ru128(rsp.wrapping_add(0x90u64))) as u128; xmm12 = (crate::simd_ops::ru128(rsp.wrapping_add(0xa0u64))) as u128; xmm11 = (crate::simd_ops::ru128(rsp.wrapping_add(0xb0u64))) as u128; xmm10 = (crate::simd_ops::ru128(rsp.wrapping_add(0xc0u64))) as u128; xmm9 = (crate::simd_ops::ru128(rsp.wrapping_add(0xd0u64))) as u128; xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0xe0u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0xf0u64))) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x100u64))) as u128; bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x130u64))) as u64; xmm0 = (xmm15) as u128; xmm15 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; rsp = ((rsp).wrapping_add(0x110u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141598270(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415c0bf0; const P1: u64 = 0x10; const P2: u64 = 0x8; const P3: u64 = 0x240;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rdx; rbx = r8; rdx = rsp.wrapping_add(0x40); rdi = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 1; continue; }
            1 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rcx = ((rdi.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x9; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = rsi; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu32(rbx, (0x2u32) as u32); bb = 5; continue; }
            5 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { crate::simd_ops::wu32(rbx, (0x1u32) as u32); bb = 7; continue; }
            7 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            8 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 9; continue; }
            9 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141598310(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524340; const P1: u64 = 0x141524330; const P2: u64 = 0x1415242e0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rbx = rcx; rax = (crate::simd_ops::ru64(rcx)) as u64; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rdx = ((rbp as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = rdx; rdx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 25; continue; }
            3 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 6; continue; }
            6 => { fl_l = rax; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 25; continue; } bb = 7; continue; }
            7 => { r11 = 0; fl_l = 0; fl_r = 0; fl_l = rsi; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 19; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r12); r10 = rsi.wrapping_sub(0x4); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r14); rdx = rdi.wrapping_add(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r15); r14 = 0x20; r15 = 0x10; r10 = ((r10).wrapping_shr(2u32)) as u64; r12 = (0xfffffffffffffff0u64) as u64; r15 = ((r15).wrapping_sub(rdi)) as u64; r14 = ((r14).wrapping_sub(rdi)) as u64; r12 = ((r12).wrapping_sub(rdi)) as u64; fl_l = ((r10).wrapping_add(1)) as u64; r10 = ((fl_l as u64)) as u64; r11 = ((r10.wrapping_mul(4)) as u64) as u64; bb = 9; continue; }
            9 => { rcx = rdx.wrapping_sub(0x10); r8 = rdx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rcx, (0xffffffffffffffffu64) as u64); crate::simd_ops::wu32(rdx.wrapping_sub(0x8u64), (0xbf800000u32) as u32); bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx)) as u64; r9 = ((r12.wrapping_add(rdx)) as u64) as u64; xmm0 = (crate::simd_ops::ru128(r9.wrapping_add(rax))) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rdx, (0xffffffffffffffffu64) as u64); crate::simd_ops::wu32(r8.wrapping_add(0x8u64), (0xbf800000u32) as u32); bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rbx)) as u64; r8 = rdx.wrapping_add(0x10); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(r9).wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu64(r8, (0xffffffffffffffffu64) as u64); crate::simd_ops::wu32(rdx.wrapping_add(0x18u64), (0xbf800000u32) as u32); bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((r15.wrapping_add(rdx)) as u64) as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rax))) as u128; crate::simd_ops::wu128(r8, (xmm0) as u128); r8 = rdx.wrapping_add(0x20); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu64(r8, (0xffffffffffffffffu64) as u64); crate::simd_ops::wu32(rdx.wrapping_add(0x28u64), (0xbf800000u32) as u32); bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((r14.wrapping_add(rdx)) as u64) as u64; rdx = ((rdx).wrapping_add(0x40u64)) as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rax))) as u128; crate::simd_ops::wu128(r8, (xmm0) as u128); fl_l = r10; fl_r = (0x1u64) as u64; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 18; continue; }
            18 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 19; continue; }
            19 => { fl_l = r11; fl_r = rsi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 24; continue; } bb = 20; continue; }
            20 => { rcx = r11; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(rdi)) as u64; fl_l = rsi; fl_r = r11; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 21; continue; }
            21 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { crate::simd_ops::wu64(rcx, (0xffffffffffffffffu64) as u64); crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), (0xbf800000u32) as u32); bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rax = ((rax).wrapping_sub(rdi)) as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(rcx))) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); rcx = ((rcx).wrapping_add(0x10u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 24; continue; }
            24 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 25; continue; }
            25 => { crate::simd_ops::wu64(rbx, rdi); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rbp as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415984f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xffffffffffffffe8; const P2: u64 = 0xffffffffffffffd8; const P3: u64 = 0xffffffffffffffec; const P4: u64 = 0x306; const P5: u64 = 0xfffffffffffffff0; const P6: u64 = 0xffffffffffffffc8; const P7: u64 = 0xfffffffffffffff1; const P8: u64 = 0x210; const P9: u64 = 0x360; const P10: u64 = 0x108; const P11: u64 = 0x211;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(P0), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rax.wrapping_add(P1), ((xmm2 as u32)) as u32); rdi = ((r9 as u32)) as u32 as u64; crate::simd_ops::wu64(rax.wrapping_add(P2), r8); crate::simd_ops::wu32(rax.wrapping_add(P3), (0x1u32) as u32); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P4))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; crate::simd_ops::wu8(rax.wrapping_add(P5), ((r8 as u8)) as u8); crate::simd_ops::wu64(rax.wrapping_add(P6), rdx); if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu8(rax.wrapping_add(P7), (0x1u8) as u8); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x98u64))) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu8(rsp.wrapping_add(0x49u64), ((r8 as u8)) as u8); bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P8))) as u8 as u64; fl_r = (0x40u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r8); bb = 6; continue; }
            5 => { rax = ((rbx.wrapping_add(P9)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rsp.wrapping_add(0x20); r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbx); r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P10))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; rcx = (rcx & !0xffu64) | (((if (((fl_l as u32)) != ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 7; continue; }
            7 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x50; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x50; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x80u64))) as u32) as u32 as u64; fl_l = (crate::simd_ops::lk_or_u8(rbx.wrapping_add(P11), (0x1u8) as u8)) as u8 as u64; fl_r = fl_l; bb = 10; continue; }
            10 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rax = 0x3; crate::simd_ops::wu32(rcx, (0x20u32) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            12 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rcx = rbx.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415985d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1439315c4; const P1: u64 = 0x1439315c8; const P2: u64 = 0x1439315cc; const P3: u64 = 0x141524330; const P4: u64 = 0x143931648; const P5: u64 = 0x34; const P6: u64 = 0x143931650; const P7: u64 = 0x143931640; const P8: u64 = 0x20; const P9: u64 = 0x143931620; const P10: u64 = 0x14158e530;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = 0;
    fl_l = 0; fl_r = 0;
    rdx = 0x800;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(P0, ((rbx as u32)) as u32);
    crate::simd_ops::wu32(P1, ((rbx as u32)) as u32);
    crate::simd_ops::wu32(P2, ((rbx as u32)) as u32);
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(P4, rax);
    rcx = rax;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { rax = ((rbx.wrapping_add(P5)) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    crate::simd_ops::wu32(P6, (0x40u32) as u32);
    rdx = ((rbx as u32)) as u32 as u64;
    crate::simd_ops::wu64(P7, rax);
    loop { rax = ((rcx.wrapping_add(P8)) as u64) as u64; rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu64(rcx, rax); r8 = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P6)) as u32 as u64; rcx = rax; if !((((fl_l as u32)) < ((fl_r as u32)))) { break; } }
    crate::simd_ops::wu64(r8, rbx);
    crate::simd_ops::wu64(P9, rbx);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P10, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141598650(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426ea8; const P1: u64 = 0x141594660; const P2: u64 = 0x1415948f0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x88u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm7) as u128); xmm7 = (xmm2) as u128; crate::simd_ops::wu128(r11.wrapping_sub(0x58u64), (xmm8) as u128); r15 = r9; r13 = (((rdx as u16)) as u32) as u32 as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rdx = (rdx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; xmm8 = ((crate::simd_ops::ru32(P0)) as u32) as u128; rbp = 0x2; r12 = ((crate::simd_ops::ru8(rsp.wrapping_add(0xd8u64))) as u32) as u32 as u64; crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rbx); rbx = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(r11.wrapping_add(0x20u64), rdi); crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), r14); r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rbp as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0xb0u64), ((rdx as u8)) as u8); fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(r11.wrapping_add(0x18u64), rsi); crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm6) as u128); bb = 3; continue; }
            3 => { rsi = (crate::simd_ops::ru64(rbx)) as u64; fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; fl_r = r15; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x305u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 16; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x211u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 16; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x306u64))) as u8 as u64; fl_r = (0x6u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 16; continue; } bb = 8; continue; }
            8 => { xmm6 = ((crate::simd_ops::ru32(rsi.wrapping_add(0x310u64))) as u32) as u128; r14 = (r14 & !0xffffu64) | (((((r14 as u16)).wrapping_add(1)) as u16 as u64)); fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm7 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 16; continue; } bb = 9; continue; }
            9 => { rax = ((rbp as u32)) as u32 as u64; fl_l = ((r12 as u8)) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 14; continue; } bb = 10; continue; }
            10 => { rdx = rsp.wrapping_add(0x20); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 13; continue; } bb = 11; continue; }
            11 => { fl_l = ((r14 as u16)) as u16 as u64; fl_r = ((r13 as u16)) as u16 as u64; if (((fl_l as u16)) > ((fl_r as u16))) { bb = 15; continue; } bb = 12; continue; }
            12 => { rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu8(rsp.wrapping_add(0xb0u64), ((rdx as u8)) as u8); bb = 16; continue; }
            13 => { rdx = ((crate::simd_ops::ru8(rsp.wrapping_add(0xb0u64))) as u32) as u32 as u64; bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); xmm8 = (xmm6) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); bb = 16; continue; }
            15 => { rdx = ((crate::simd_ops::ru8(rsp.wrapping_add(0xb0u64))) as u32) as u32 as u64; bb = 16; continue; }
            16 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 17; continue; }
            17 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; bb = 18; continue; }
            18 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; rcx = (((r14 as u16)) as u32) as u32 as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 30; continue; } bb = 19; continue; }
            19 => { fl_l = ((xmm7 as u32)) as u32 as u64; fl_r = ((xmm8 as u32)) as u32 as u64; xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x2cu64))) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((xmm0 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), ((xmm1 as u32)) as u32); if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 28; continue; } bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0xd0u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 22; continue; } bb = 21; continue; }
            21 => { fl_l = ((xmm7 as u32)) as u32 as u64; fl_r = ((xmm8 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 28; continue; } bb = 22; continue; }
            22 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x1d8u64))) as u64; crate::simd_ops::wu64(rax, rcx); fl_l = ((r12 as u8)) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 26; continue; } bb = 24; continue; }
            24 => { rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 27; continue; } bb = 25; continue; }
            25 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 27; continue; } bb = 26; continue; }
            26 => { rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0xe8u64))) as u32 as u64; rcx = r8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 27; continue; }
            27 => { rbp = 0x1; rax = ((rbp as u32)) as u32 as u64; bb = 31; continue; }
            28 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 27; continue; } bb = 29; continue; }
            29 => { rax = 0x50; fl_l = ((r12 as u8)) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; rbp = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (rax as u32) } else { (rbp as u32) }) as u32 as u64; rax = ((rbp as u32)) as u32 as u64; bb = 31; continue; }
            30 => { rax = 0x1; bb = 31; continue; }
            31 => { xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; rsp = ((rsp).wrapping_add(0x88u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141598850(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931658; const P1: u64 = 0x1426452b0; const P2: u64 = 0x143931654; const P3: u64 = 0x143931630; const P4: u64 = 0x141595250; const P5: u64 = 0x143931690; const P6: u64 = 0x143931688; const P7: u64 = 0x143931680; const P8: u64 = 0x14160c0e0; const P9: u64 = 0x1415242e0; const P10: u64 = 0x143931648; const P11: u64 = 0x143931638; const P12: u64 = 0x143931650; const P13: u64 = 0x143931640; const P14: u64 = 0x1426452a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rcx = P0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(P2)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 23; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = (crate::simd_ops::ru64(P3)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rdi = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { xmm3 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32) as u128; r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x14u64))) as u32 as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 3; continue; }
            3 => { r8 = (crate::simd_ops::ru64(P5)) as u64; rdx = rdi; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = r8; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rdx = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x230u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 6; continue; }
            6 => { bb = 12; continue; }
            7 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x230u64))) as u64; fl_l = rcx; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(P5, rax); bb = 11; continue; }
            10 => { crate::simd_ops::wu64(rdx.wrapping_add(0x230u64), rax); bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(P6)) as u64; fl_l = rcx; fl_r = rax; rax = (if (((fl_l as u64)) == ((fl_r as u64))) { rdx } else { rax }) as u64; fl_l = (crate::simd_ops::lk_sub_u32(P7, (1) as u32)) as u32 as u64; crate::simd_ops::wu64(P6, rax); bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rcx = rbx.wrapping_add(0x18); rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x58u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbx; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(P3)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru64(P10)) as u64; crate::simd_ops::wu64(P3, rax); rax = (crate::simd_ops::ru64(P11)) as u64; fl_l = rbx; fl_r = rax; rax = (if (((fl_l as u64)) == ((fl_r as u64))) { rdi } else { rax }) as u64; crate::simd_ops::wu64(P11, rax); fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 18; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru32(P12)) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 18; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(P13)) as u64; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(P13, rbx); bb = 19; continue; }
            18 => { rdx = rbx; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru32(P2)) as u32 as u64; rbx = (crate::simd_ops::ru64(P3)) as u64; fl_l = (((rax as u32)).wrapping_sub(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(P2, ((rax as u32)) as u32); bb = 21; continue; }
            20 => { rax = (crate::simd_ops::ru32(P2)) as u32 as u64; bb = 21; continue; }
            21 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 2; continue; } bb = 22; continue; }
            22 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 23; continue; }
            23 => { rcx = P0; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return crate::simd_ops::tail_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P14), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141598a00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415c0bf0; const P1: u64 = 0x14159f4e0; const P2: u64 = 0x143426ea4; const P3: u64 = 0x143426e64; const P4: u64 = 0x141605350; const P5: u64 = 0x141597e90; const P6: u64 = 0x143931610; const P7: u64 = 0x143931680; const P8: u64 = 0x1439315c4; const P9: u64 = 0x143931600; const P10: u64 = 0x143931608; const P11: u64 = 0x143426ea8; const P12: u64 = 0x141594660; const P13: u64 = 0x1415948f0; const P14: u64 = 0x1415c6160; const P15: u64 = 0x1415c6110; const P16: u64 = 0x1415242e0; const P17: u64 = 0x1415c63b0; const P18: u64 = 0x1415c6280; const P19: u64 = 0x141592360; const P20: u64 = 0x141596200; const P21: u64 = 0x1415c2680; const P22: u64 = 0x141596a90; const P23: u64 = 0x141596d80; const P24: u64 = 0x143931690; const P25: u64 = 0x143931688; const P26: u64 = 0x141594650; const P27: u64 = 0x1401e4990; const P28: u64 = 0x143930f90; const P29: u64 = 0x14159b5e0; const P30: u64 = 0x1401de410;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 43>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), (r8) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rdx) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rax.wrapping_sub(0x58u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x118u64)) as u64; rbx = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; r12 = (rdx) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x58u64), (xmm6) as u128); rdx = ((rbp.wrapping_add(0x78u64)) as u64) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x68u64), (xmm7) as u128); r13 = (r8) as u64; rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdi = (rcx) as u64; rsi = ((rax as u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), (rax) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x40u64), ((rax as u8)) as u8); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r14 = ((rsi.wrapping_add(0x1u64)) as u32) as u32 as u64; r15 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 1; continue; }
            1 => { fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rcx = ((rdi.wrapping_add(0x10u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x9u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = (rbx) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x240u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { r15 = (0x2u32) as u32 as u64; crate::simd_ops::wu8(rbp.wrapping_add(0x78u64), ((r14 as u8)) as u8); bb = 9; continue; }
            5 => { r15 = ((r14 as u32)) as u32 as u64; bb = 8; continue; }
            6 => { crate::simd_ops::wu8(rbp.wrapping_add(0x78u64), ((r14 as u8)) as u8); rcx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu8(rbp.wrapping_add(0x78u64), ((rsi as u8)) as u8); bb = 9; continue; }
            9 => { r14 = (crate::simd_ops::ru64(r13.wrapping_add(0x10u64))) as u64; rdx = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; r8 = (r14) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x4cu64), ((rsi as u32)) as u32); rcx = (rdi) as u64; xmm6 = 0u128; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm7 = ((crate::simd_ops::ru32(P2)) as u32) as u128; xmm0 = ((crate::simd_ops::ru32(P3)) as u32) as u128; xmm2 = ((crate::simd_ops::ru32(rax)) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x7cu64), ((xmm2 as u32)) as u32); xmm2 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(xmm6 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm6 as u32) }).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x80u64), ((xmm1 as u32)) as u32); fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm7 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { xmm2 = (xmm7) as u128; bb = 11; continue; }
            11 => { rax = ((xmm2 as u32)) as u32 as u64; fl_l = ((((rax as u32)) >> (31u32)) & 1) as u64; rax = (((rax as u32)) & !((1u32) << (31u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x7f800000u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((xmm2 as u32)) as u32); bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = ((rsp.wrapping_add(0x4cu64)) as u64) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0xf8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 22; continue; } bb = 14; continue; }
            14 => { xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x4cu64))) as u32) as u128; rax = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(core::ptr::read_unaligned((rax.wrapping_add(0x78u64)) as *const u32))).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x4cu64), ((xmm0 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x80u64))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 22; continue; } bb = 15; continue; }
            15 => { rdi = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; rcx = (r14) as u64; rbx = ((crate::simd_ops::ru16(rax.wrapping_add(0x38u64))) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = (rax) as u64; rdx = ((rbx as u32)) as u32 as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm1 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x4cu64))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(core::ptr::read_unaligned((rbp.wrapping_sub(0x80u64)) as *const u32))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(core::ptr::read_unaligned((rsp.wrapping_add(0x7cu64)) as *const u32))).to_bits() as u128); bb = 18; continue; }
            17 => { xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x7cu64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(core::ptr::read_unaligned((rbp.wrapping_sub(0x80u64)) as *const u32))).to_bits() as u128); bb = 18; continue; }
            18 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) > f32::from_bits(xmm6 as u32) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(xmm6 as u32) }).to_bits() as u128); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm7 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 20; continue; } bb = 19; continue; }
            19 => { xmm7 = (xmm0) as u128; bb = 20; continue; }
            20 => { rax = ((xmm7 as u32)) as u32 as u64; fl_l = ((((rax as u32)) >> (31u32)) & 1) as u64; rax = (((rax as u32)) & !((1u32) << (31u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x7f800000u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 22; continue; } bb = 21; continue; }
            21 => { crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((xmm7 as u32)) as u32); bb = 23; continue; }
            22 => { xmm7 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x70u64))) as u32) as u128; bb = 23; continue; }
            23 => { rax = ((crate::simd_ops::ru16(P6)) as u32) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x44u64), ((rax as u16)) as u16); crate::simd_ops::wu32(rbp.wrapping_sub(0x78u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(P7)) as u32 as u64; rax = (((rax as u32)).wrapping_sub(crate::simd_ops::ru32(P8))) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x21u32) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 45; continue; } bb = 24; continue; }
            24 => { rbx = (crate::simd_ops::ru64(P9)) as u64; rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rcx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (rax) as u64); rdx = (((rax as u16)) as u32) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0xe0u64), (xmm8) as u128); rax = (crate::simd_ops::ru32(P10)) as u32 as u64; xmm8 = ((crate::simd_ops::ru32(P11)) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), (0x2u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((rcx as u32)) as u32); rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 35; continue; } bb = 25; continue; }
            25 => { rsi = ((crate::simd_ops::ru16(rsp.wrapping_add(0x44u64))) as u32) as u32 as u64; r13 = (0x1u32) as u32 as u64; bb = 26; continue; }
            26 => { r14 = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x305u64))) as u8 as u64; fl_r = ((r13 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 33; continue; } bb = 27; continue; }
            27 => { fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x211u64))) as u8 as u64; fl_r = ((r13 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 33; continue; } bb = 28; continue; }
            28 => { fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x306u64))) as u8 as u64; fl_r = (0x6u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 33; continue; } bb = 29; continue; }
            29 => { xmm6 = ((crate::simd_ops::ru32(r14.wrapping_add(0x310u64))) as u32) as u128; rdx = (rdx & !0xffffu64) | (((((rdx as u16)).wrapping_add(1)) as u16 as u64)); fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm7 as u32)) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x44u64), ((rdx as u16)) as u16); if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 33; continue; } bb = 30; continue; }
            30 => { rdx = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x50u64))) as u32 as u64; rdx = ((crate::simd_ops::ru16(rsp.wrapping_add(0x44u64))) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 32; continue; } bb = 31; continue; }
            31 => { fl_l = ((rdx as u16)) as u16 as u64; fl_r = ((rsi as u16)) as u16 as u64; rcx = (((rcx as u8)) as u32) as u32 as u64; rcx = (if (((fl_l as u16)) <= ((fl_r as u16))) { (r13 as u32) } else { (rcx as u32) }) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((rcx as u32)) as u32); bb = 33; continue; }
            32 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (r14) as u64); xmm8 = (xmm6) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), ((rax as u32)) as u32); bb = 33; continue; }
            33 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 26; continue; } bb = 34; continue; }
            34 => { rsi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x70u64))) as u64; r13 = (crate::simd_ops::ru64(rbp.wrapping_add(0x70u64))) as u64; r12 = (crate::simd_ops::ru64(rbp.wrapping_add(0x68u64))) as u64; bb = 35; continue; }
            35 => { rax = (((rdx as u16)) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbp.wrapping_sub(0x78u64))) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 42; continue; } bb = 36; continue; }
            36 => { fl_l = ((xmm7 as u32)) as u32 as u64; fl_r = ((xmm8 as u32)) as u32 as u64; xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x64u64))) as u32) as u128; crate::simd_ops::wu32(rbp.wrapping_sub(0x70u64), ((xmm0 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x68u64))) as u32) as u128; crate::simd_ops::wu32(rbp.wrapping_sub(0x68u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x6cu64), ((xmm1 as u32)) as u32); if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 43; continue; } bb = 37; continue; }
            37 => { fl_l = ((xmm7 as u32)) as u32 as u64; fl_r = ((xmm8 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 43; continue; } bb = 38; continue; }
            38 => { r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x70u64))) as u64; fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 43; continue; } bb = 39; continue; }
            39 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x68u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 42; continue; } bb = 40; continue; }
            40 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 42; continue; } bb = 41; continue; }
            41 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = (r8) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 42; continue; }
            42 => { xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0xe0u64))) as u128; r14 = (0x1u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), ((r14 as u32)) as u32); bb = 46; continue; }
            43 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 42; continue; } bb = 44; continue; }
            44 => { xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0xe0u64))) as u128; r14 = (0x1u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), (0x50u32) as u32); bb = 46; continue; }
            45 => { r14 = (0x1u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), ((r14 as u32)) as u32); bb = 46; continue; }
            46 => { rax = (crate::simd_ops::ru32(r12.wrapping_add(0x18u64))) as u32 as u64; rdi = (rdi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0xf0u64))) as u128; rax = ((((rax as u32)) & (0xffff0000u32))) as u32 as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x100u64))) as u128; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x80000u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 67; continue; } bb = 47; continue; }
            47 => { rbx = (crate::simd_ops::ru64(r13.wrapping_add(0x20u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 49; continue; } bb = 48; continue; }
            48 => { rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 60; continue; }
            49 => { rcx = (r12) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P14) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r12 = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 51; continue; } bb = 50; continue; }
            50 => { rbx = ((rax.wrapping_add(0x34u64)) as u32) as u32 as u64; bb = 61; continue; }
            51 => { r10 = (crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32 as u64; r9 = ((rbx.wrapping_add(0x8u64)) as u64) as u64; rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rax = ((rdi as u32)) as u32 as u64; fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 59; continue; } bb = 52; continue; }
            52 => { r11 = (crate::simd_ops::ru32(r12.wrapping_add(0x4u64))) as u32 as u64; bb = 53; continue; }
            53 => { fl_l = ((r11 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r9)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 58; continue; } bb = 54; continue; }
            54 => { rcx = (crate::simd_ops::ru32(r9.wrapping_add(0x4u64))) as u32 as u64; rdx = (crate::simd_ops::ru32(r9.wrapping_add(0x1cu64))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(16u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 65; continue; } bb = 55; continue; }
            55 => { rdx = (crate::simd_ops::ru64(r9.wrapping_add(0x8u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 64; continue; } bb = 56; continue; }
            56 => { rdx = (crate::simd_ops::ru32(r9.wrapping_add(0x18u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 58; continue; } bb = 57; continue; }
            57 => { r8 = (crate::simd_ops::ru64(r9.wrapping_add(0x10u64))) as u64; fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 63; continue; } bb = 58; continue; }
            58 => { r9 = ((r9).wrapping_add(0x20u64)) as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 53; continue; } bb = 59; continue; }
            59 => { rcx = (r12) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P15) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (r12) as u64; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P16) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 60; continue; }
            60 => { rbx = (0x2u32) as u32 as u64; r12 = (rdi) as u64; bb = 61; continue; }
            61 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; crate::simd_ops::wu8(rbp.wrapping_add(0x70u64), (0x0u8) as u8); rdi = (rdi & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 116; continue; } bb = 62; continue; }
            62 => { bb = 68; continue; }
            63 => { rax = (crate::simd_ops::ru32(r12)) as u32 as u64; r9 = ((rsp.wrapping_add(0x60u64)) as u64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x60u64))) as u32 as u64; rax = ((((rax as u32)) & (0xfffff98cu32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), ((rdx as u32)) as u32); rax = ((((rax as u32)) | (0x18cu32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x64u64), (0xffffffffu32) as u32); rdx = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x6cu64), ((rax as u32)) as u32); rcx = (r12) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P17) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = ((r14 as u32)) as u32 as u64; bb = 61; continue; }
            64 => { crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x1u8) as u8); r9 = (rdx) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x1u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0xffffffffu32) as u32); bb = 66; continue; }
            65 => { r9 = (crate::simd_ops::ru64(r9.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x1u8) as u8); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((rdi as u8)) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rdx as u32)) as u32); bb = 66; continue; }
            66 => { rdx = (crate::simd_ops::ru32(r12)) as u32 as u64; r8 = ((rcx as u32)) as u32 as u64; rcx = (r12) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P18) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = ((r14 as u32)) as u32 as u64; bb = 61; continue; }
            67 => { crate::simd_ops::wu8(rbp.wrapping_add(0x70u64), ((rdi as u8)) as u8); bb = 68; continue; }
            68 => { rcx = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; rdx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdx) as u64); r9 = (r13) as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x60u64))) as u64; r8 = (r12) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rsi = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 112; continue; } bb = 69; continue; }
            69 => { fl_l = (crate::simd_ops::ru32(r13)) as u32 as u64; fl_r = (0x0u32) as u32 as u64; rax = ((rbp.wrapping_add(0x70u64)) as u64) as u64; r9 = ((crate::simd_ops::ru8(rbp.wrapping_add(0x78u64))) as u32) as u32 as u64; r14 = ((r13.wrapping_add(0xb0u64)) as u64) as u64; xmm1 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x4cu64))) as u32) as u128; rcx = ((rsi.wrapping_add(0x18u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); rax = ((rsp.wrapping_add(0x48u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (r14) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 71; continue; } bb = 70; continue; }
            70 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 72; continue; }
            71 => { r8 = (crate::simd_ops::ru64(r13.wrapping_add(0x88u64))) as u64; r8 = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; bb = 72; continue; }
            72 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P19) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 116; continue; } bb = 73; continue; }
            73 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x70u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 76; continue; } bb = 74; continue; }
            74 => { fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 76; continue; } bb = 75; continue; }
            75 => { rcx = (rcx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 77; continue; }
            76 => { rcx = (rcx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 77; continue; }
            77 => { xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x70u64))) as u32) as u128; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x306u64))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; rax = (crate::simd_ops::ru64(r13.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x40u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), (rdx) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x3cu64), (0x1u32) as u32); crate::simd_ops::wu8(rbp.wrapping_sub(0x38u64), ((rdx as u8)) as u8); crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), (rax) as u64); if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 79; continue; } bb = 78; continue; }
            78 => { crate::simd_ops::wu8(rbp.wrapping_sub(0x37u64), (0x1u8) as u8); fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 80; continue; } bb = 79; continue; }
            79 => { crate::simd_ops::wu8(rbp.wrapping_sub(0x37u64), ((rdx as u8)) as u8); bb = 80; continue; }
            80 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x210u64))) as u8 as u64; fl_r = (0x40u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 82; continue; } bb = 81; continue; }
            81 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x58u64), (rdx) as u64); bb = 83; continue; }
            82 => { rax = ((rsi.wrapping_add(0x360u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x58u64), (rax) as u64); bb = 83; continue; }
            83 => { r12 = (crate::simd_ops::ru64(rbp.wrapping_add(0x60u64))) as u64; rdx = ((rbp.wrapping_sub(0x60u64)) as u64) as u64; r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), (rsi) as u64); r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rcx = (r12) as u64; rax = (crate::simd_ops::ru64(r12)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x108u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x2u32) as u32 as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u32)) != ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); if (((fl_l as u32)) == ((fl_r as u32))) { bb = 87; continue; } bb = 84; continue; }
            84 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x50u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 86; continue; } bb = 85; continue; }
            85 => { fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x44u64))) as u32 as u64; fl_r = (0x50u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 87; continue; } bb = 86; continue; }
            86 => { rcx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x78u64))) as u32) as u32 as u64; fl_l = (crate::simd_ops::lk_or_u8(rsi.wrapping_add(0x211u64), (0x1u8) as u8)) as u8 as u64; fl_r = fl_l; bb = 87; continue; }
            87 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 89; continue; } bb = 88; continue; }
            88 => { rcx = (rcx & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x20u32) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x40u64), ((rcx as u8)) as u8); rbx = (0x3u32) as u32 as u64; bb = 117; continue; }
            89 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; rcx = ((rsi.wrapping_add(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rcx = (rcx & !0xffu64) | (((0x1u8) as u8 as u64)); rbx = ((rax as u32)) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x40u64), ((rcx as u8)) as u8); r15 = (r15 & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 107; continue; } bb = 90; continue; }
            90 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x210u64))) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; rcx = ((rsi.wrapping_add(0x18u64)) as u64) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 92; continue; } bb = 91; continue; }
            91 => { rdx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x68u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 94; continue; }
            92 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x70u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x210u64))) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 94; continue; } bb = 93; continue; }
            93 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; rcx = ((rsi.wrapping_add(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x78u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 94; continue; }
            94 => { rdx = ((r13.wrapping_add(0x190u64)) as u64) as u64; r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P20) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = (crate::simd_ops::ru64(r13.wrapping_add(0x88u64))) as u64; fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 98; continue; } bb = 95; continue; }
            95 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x90u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 97; continue; } bb = 96; continue; }
            96 => { r8 = ((r8).wrapping_add(0x28u64)) as u64; rdx = (0x1u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P21) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 97; continue; }
            97 => { rax = (crate::simd_ops::ru64(r13.wrapping_add(0x88u64))) as u64; rcx = ((rsi.wrapping_add(0x158u64)) as u64) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rcx) as u64); bb = 98; continue; }
            98 => { rbx = (crate::simd_ops::ru32(r13.wrapping_add(0x80u64))) as u32 as u64; rcx = (rsi) as u64; rdi = (crate::simd_ops::ru64(r13.wrapping_add(0x18u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P22) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rbx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rdx = (rdi) as u64; rcx = (rsi) as u64; r8 = (r8 & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P23) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 103; continue; } bb = 99; continue; }
            99 => { rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsi.wrapping_add(0x230u64), (rdi) as u64); fl_l = (crate::simd_ops::ru64(P24)) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 101; continue; } bb = 100; continue; }
            100 => { crate::simd_ops::wu64(P24, (rsi) as u64); bb = 102; continue; }
            101 => { rax = (crate::simd_ops::ru64(P25)) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x230u64), (rsi) as u64); bb = 102; continue; }
            102 => { fl_l = (crate::simd_ops::lk_add_u32(P7, (1) as u32)) as u32 as u64; crate::simd_ops::wu64(P25, (rsi) as u64); bb = 103; continue; }
            103 => { fl_l = (crate::simd_ops::ru64(r13.wrapping_add(0x88u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 105; continue; } bb = 104; continue; }
            104 => { rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P26) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(r13.wrapping_add(0x88u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), (rax) as u64); rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P27) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(r13.wrapping_add(0x88u64))) as u64; crate::simd_ops::wu64(rcx, (rax) as u64); bb = 105; continue; }
            105 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 132; continue; } bb = 106; continue; }
            106 => { rcx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x40u64))) as u32) as u32 as u64; bb = 107; continue; }
            107 => { fl_l = ((r15 as u8)) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 117; continue; } bb = 108; continue; }
            108 => { rax = (crate::simd_ops::ru64(r13.wrapping_add(0x10u64))) as u64; fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 120; continue; } bb = 109; continue; }
            109 => { fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x40u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 111; continue; } bb = 110; continue; }
            110 => { fl_l = (crate::simd_ops::lk_or_u8(rsi.wrapping_add(0x305u64), (0x10u8) as u8)) as u8 as u64; fl_r = fl_l; bb = 111; continue; }
            111 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; rcx = ((rsi.wrapping_add(0x18u64)) as u64) as u64; rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x58u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (rsi) as u64; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P16) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 122; continue; }
            112 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 115; continue; } bb = 113; continue; }
            113 => { fl_l = (r12) as u64 as u64; fl_r = (r12) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 115; continue; } bb = 114; continue; }
            114 => { rcx = (r12) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P15) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (r12) as u64; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P16) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 115; continue; }
            115 => { rbx = (0x2u32) as u32 as u64; bb = 116; continue; }
            116 => { r12 = (crate::simd_ops::ru64(rbp.wrapping_add(0x60u64))) as u64; rcx = (rcx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 117; continue; }
            117 => { rdx = (crate::simd_ops::ru32(r13.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 108; continue; } bb = 118; continue; }
            118 => { fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 108; continue; } bb = 119; continue; }
            119 => { r9 = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; rax = ((rbp.wrapping_add(0x60u64)) as u64) as u64; rcx = (crate::simd_ops::ru64(P28)) as u64; r8 = (rsi) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P29) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 109; continue; }
            120 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 122; continue; } bb = 121; continue; }
            121 => { rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), (rax) as u64); rax = (crate::simd_ops::ru64(r12)) as u64; rdx = ((rbp.wrapping_sub(0x60u64)) as u64) as u64; xmm0 = 0u128; crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), (rcx) as u64); rcx = (r12) as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x40u64), (0x0u32) as u32); crate::simd_ops::wu128(rbp.wrapping_sub(0x58u64), (xmm0) as u128); crate::simd_ops::wu32(rbp.wrapping_sub(0x3cu64), (0x1u32) as u32); crate::simd_ops::wu16(rbp.wrapping_sub(0x38u64), (0x100u16) as u16); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x110u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 122; continue; }
            122 => { rax = ((crate::simd_ops::ru8(r13.wrapping_add(0x2f8u64))) as u32) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 130; continue; } bb = 123; continue; }
            123 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 131; continue; } bb = 124; continue; }
            124 => { rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x20u32) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 129; continue; } bb = 125; continue; }
            125 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 128; continue; } bb = 126; continue; }
            126 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 131; continue; } bb = 127; continue; }
            127 => { crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x25u32) as u32); bb = 131; continue; }
            128 => { crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x24u32) as u32); bb = 131; continue; }
            129 => { crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x23u32) as u32); bb = 131; continue; }
            130 => { rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; rax = (!((rax as u32))) as u32 as u64; rax = ((((rax as u32)) & (0x1u32))) as u32 as u64; fl_l = ((((rax as u32)) | (0x1eu32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rax as u32)) as u32); bb = 131; continue; }
            131 => { r8 = (crate::simd_ops::ru64(r13.wrapping_add(0x10u64))) as u64; rax = ((r13.wrapping_add(0x38u64)) as u64) as u64; rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; r9 = ((r13.wrapping_add(0x20u64)) as u64) as u64; rcx = (r12) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); r8 = (crate::simd_ops::ru64(r8.wrapping_add(0x58u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P30) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 132; continue; }
            132 => { rax = ((rbx as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x118u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141599270(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1439315c0; const P1: u64 = 0x1439315c1; const P2: u64 = 0x143931690; const P3: u64 = 0x1415963a0; const P4: u64 = 0x211; const P5: u64 = 0x230; const P6: u64 = 0x1439315d8; const P7: u64 = 0x1439315e0; const P8: u64 = 0x1415a3d70;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(P1)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(P2)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_and_u8(rbx.wrapping_add(P4), (0xfeu8) as u8); rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu8(P0, (0x0u8) as u8); bb = 8; continue; }
            5 => { rax = (crate::simd_ops::ru64(P2)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { bb = 7; continue; }
            7 => { crate::simd_ops::lk_and_u8(rax.wrapping_add(P4), (0xfeu8) as u8); rax = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(P6)) as u64; r8 = (crate::simd_ops::ru32(P7)) as u32 as u64; rbx = rdx; rcx = ((rdx.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 9; continue; }
            9 => { bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru32(P7)) as u32 as u64; rdx = (crate::simd_ops::ru64(P6)) as u64; bb = 12; continue; }
            12 => { rax = ((r8 as u32)) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rcx = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 13; continue; }
            13 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141599340(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1439315d0; const P1: u64 = 0x1439315c2; const P2: u64 = 0x1439315f8; const P3: u64 = 0x40; const P4: u64 = 0x1439315f0; const P5: u64 = 0x1439315e8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (crate::simd_ops::ru64(P0)) as u64; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu8(P1, (0x1u8) as u8); return rax; }
            2 => { r9 = (crate::simd_ops::ru64(P2)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = r9; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 3; continue; }
            3 => { fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = rax; rax = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 5; continue; }
            5 => { return rax; }
            6 => { r8 = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; fl_l = rax; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(P2, r8); bb = 9; continue; }
            8 => { crate::simd_ops::wu64(rdx.wrapping_add(P3), r8); bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(P4)) as u64; fl_l = rax; fl_r = rcx; rcx = (if (((fl_l as u64)) == ((fl_r as u64))) { rdx } else { rcx }) as u64; fl_l = (crate::simd_ops::lk_sub_u32(P5, (1) as u32)) as u32 as u64; crate::simd_ops::wu64(P4, rcx); bb = 10; continue; }
            10 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415993b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1439315d8; const P1: u64 = 0x1439315e0; const P2: u64 = 0xffffffffffffffff; const P3: u64 = 0x8; const P4: u64 = 0x14183acc8;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; r9 = (crate::simd_ops::ru64(P0)) as u64; rdx = rcx; r8 = (crate::simd_ops::ru32(P1)) as u32 as u64; rcx = r9; rax = ((r9.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; r10 = ((r9.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = r9; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = ((rcx).wrapping_add(0x8u64)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { fl_l = rcx; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 4; continue; }
            4 => { rax = ((r8.wrapping_add(P2)) as u32) as u32 as u64; rax = ((r9.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = ((rax).wrapping_sub(rcx)) as u64; rdx = ((rcx.wrapping_add(P3)) as u64) as u64; rax = ((((rax as u32)) & (0xfffffff8u32))) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru32(P1)) as u32 as u64; bb = 6; continue; }
            6 => { fl_l = (((r8 as u32)).wrapping_sub(1)) as u32 as u64; r8 = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(P1, ((r8 as u32)) as u32); bb = 7; continue; }
            7 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415995a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1439315f8; const P1: u64 = 0x1439315d0; const P2: u64 = 0x1439315c2; const P3: u64 = 0x1439315f0; const P4: u64 = 0x1439315e8; const P5: u64 = 0x143931690; const P6: u64 = 0x141594f30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(P0)) as u64; rbp = (((rdx as u8)) as u32) as u32 as u64; rsi = rcx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r14); r14 = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rdi = rbx; crate::simd_ops::wu64(P1, rbx); crate::simd_ops::wu8(P2, ((r14 as u8)) as u8); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x24u64), (1) as u32); rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = rsi; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x88u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(P2)) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(P1, r14); if (((fl_l as u8)) == ((fl_r as u8))) { bb = 12; continue; } bb = 3; continue; }
            3 => { r8 = (crate::simd_ops::ru64(P0)) as u64; rdx = r14; crate::simd_ops::wu8(P2, ((r14 as u8)) as u8); rax = r8; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { fl_l = rax; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rdx = rax; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 6; continue; }
            6 => { bb = 12; continue; }
            7 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(P0, rcx); bb = 11; continue; }
            10 => { crate::simd_ops::wu64(rdx.wrapping_add(0x40u64), rcx); bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(P3)) as u64; fl_l = rax; fl_r = rcx; rcx = (if (((fl_l as u64)) == ((fl_r as u64))) { rdx } else { rcx }) as u64; fl_l = (crate::simd_ops::lk_sub_u32(P4, (1) as u32)) as u32 as u64; crate::simd_ops::wu64(P3, rcx); bb = 12; continue; }
            12 => { rcx = rdi.wrapping_add(0x10); rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 15; continue; }
            15 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(P5)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 17; continue; }
            17 => { bb = 18; continue; }
            18 => { rcx = rbx; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x230u64))) as u64; fl_l = ((rbp as u8)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { rdx = rsi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 21; continue; }
            20 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 21; continue; }
            21 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 18; continue; } bb = 22; continue; }
            22 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415996f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931690; const P1: u64 = 0x141594c30; const P2: u64 = 0x1415970c0; const P3: u64 = 0x1d8; const P4: u64 = 0x143930f08; const P5: u64 = 0x1415ab5b0; const P6: u64 = 0x230; const P7: u64 = 0x1415efd80;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(P0)) as u64; rdi = rcx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rsi = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rdx = rdi; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rsi as u32)) as u32); rdx = rsp.wrapping_add(0x38); crate::simd_ops::wu32(rsp.wrapping_add(0x3cu64), (0x4u32) as u32); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; r9 = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(P4)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 5; continue; }
            5 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 6; continue; }
            6 => { rcx = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P7, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141599780(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931690; const P1: u64 = 0x141594cb0; const P2: u64 = 0x1415970c0; const P3: u64 = 0x1d8; const P4: u64 = 0x143930f08; const P5: u64 = 0x1415ab5b0; const P6: u64 = 0x230; const P7: u64 = 0x1415eff20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(P0)) as u64; rdi = rcx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rsi = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rdx = rdi; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rsi as u32)) as u32); rdx = rsp.wrapping_add(0x38); crate::simd_ops::wu32(rsp.wrapping_add(0x3cu64), (0x4u32) as u32); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; r9 = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(P4)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 5; continue; }
            5 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 6; continue; }
            6 => { rcx = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P7, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141599810(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14158e760; const P1: u64 = 0x143931690; const P2: u64 = 0x143931688; const P3: u64 = 0x143931680; const P4: u64 = 0x14160c0e0; const P5: u64 = 0x1415242e0; const P6: u64 = 0x143931640; const P7: u64 = 0x143931630; const P8: u64 = 0x143931648; const P9: u64 = 0x143931638; const P10: u64 = 0x143931650; const P11: u64 = 0x143931654; const P12: u64 = 0x143931600; const P13: u64 = 0x143931608; const P14: u64 = 0x14393160c; const P15: u64 = 0x1439315d8; const P16: u64 = 0x1439315e0; const P17: u64 = 0x1439315e4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(P1)) as u64; rdi = 0; fl_l = 0; fl_r = 0; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rax = rbx; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x230u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(P1, rdi); crate::simd_ops::wu64(P2, rdi); bb = 5; continue; }
            4 => { crate::simd_ops::wu64(P1, rax); bb = 5; continue; }
            5 => { crate::simd_ops::lk_sub_u32(P3, (1) as u32); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rcx = rbx.wrapping_add(0x18); rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x58u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbx; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(P1)) as u64; rax = rbx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(P6)) as u64; crate::simd_ops::wu64(P1, rdi); crate::simd_ops::wu64(P2, rdi); crate::simd_ops::wu32(P3, ((rdi as u32)) as u32); bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(P7)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = (crate::simd_ops::ru64(P8)) as u64; crate::simd_ops::wu64(P7, rax); rax = (crate::simd_ops::ru64(P9)) as u64; fl_l = rcx; fl_r = rax; rax = (if (((fl_l as u64)) == ((fl_r as u64))) { rdi } else { rax }) as u64; crate::simd_ops::wu64(P9, rax); fl_l = rcx; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(P10)) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rax = ((rax).wrapping_add(r8)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rcx, rdx); rdx = rcx; fl_l = (crate::simd_ops::lk_sub_u32(P11, (1) as u32)) as u32 as u64; crate::simd_ops::wu64(P6, rcx); bb = 7; continue; }
            11 => { rdx = rcx; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(P6)) as u64; fl_l = (crate::simd_ops::lk_sub_u32(P11, (1) as u32)) as u32 as u64; bb = 7; continue; }
            12 => { rcx = (crate::simd_ops::ru64(P8)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru32(P10)) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(P8)) as u64; bb = 16; continue; }
            16 => { rdx = rbx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 13; continue; } bb = 17; continue; }
            17 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { rdx = rcx; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(P8, rdi); bb = 19; continue; }
            19 => { rdx = (crate::simd_ops::ru64(P12)) as u64; xmm0 = 0u128; crate::simd_ops::wu32(P10, ((rdi as u32)) as u32); crate::simd_ops::wu64(P6, rdi); crate::simd_ops::wu128(P7, (xmm0) as u128); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(P13, ((rdi as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(P12, rdi); crate::simd_ops::wu32(P14, ((rdi as u32)) as u32); bb = 21; continue; }
            21 => { rdx = (crate::simd_ops::ru64(P15)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(P16, ((rdi as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(P15, rdi); crate::simd_ops::wu32(P17, ((rdi as u32)) as u32); bb = 23; continue; }
            23 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415999f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931680; const P1: u64 = 0x1439315c4; const P2: u64 = 0x143931610; const P3: u64 = 0x143931608; const P4: u64 = 0x143931600; const P5: u64 = 0x143426ea8; const P6: u64 = 0x141594660; const P7: u64 = 0x1415948f0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x88u64)) as u64; crate::simd_ops::wu32(rdx, (0x21u32) as u32); rax = (crate::simd_ops::ru32(P0)) as u32 as u64; rax = (((rax as u32)).wrapping_sub(crate::simd_ops::ru32(P1))) as u32 as u64; r15 = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm7) as u128); xmm7 = (xmm0) as u128; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 23; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(P3)) as u32 as u64; crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbx); rbx = (crate::simd_ops::ru64(P4)) as u64; crate::simd_ops::wu64(r11.wrapping_add(0x18u64), rbp); rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), rdi); crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), r14); r14 = (r14 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu128(r11.wrapping_sub(0x58u64), (xmm8) as u128); xmm8 = ((crate::simd_ops::ru32(P5)) as u32) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x2u32) as u32); fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), rsi); crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm6) as u128); bb = 3; continue; }
            3 => { rsi = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x305u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 11; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x211u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 11; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x306u64))) as u8 as u64; fl_r = (0x6u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 11; continue; } bb = 6; continue; }
            6 => { xmm6 = ((crate::simd_ops::ru32(rsi.wrapping_add(0x310u64))) as u32) as u128; rbp = (rbp & !0xffffu64) | (((((rbp as u16)).wrapping_add(1)) as u16 as u64)); fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm7 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 11; continue; } bb = 7; continue; }
            7 => { rdx = rsp.wrapping_add(0xa8); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = ((rbp as u16)) as u16 as u64; fl_r = ((r15 as u16)) as u16 as u64; if (((fl_l as u16)) > ((fl_r as u16))) { bb = 11; continue; } bb = 9; continue; }
            9 => { r14 = (r14 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 11; continue; }
            10 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); xmm8 = (xmm6) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); bb = 11; continue; }
            11 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 12; continue; }
            12 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; bb = 13; continue; }
            13 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rax = (((rbp as u16)) as u32) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 21; continue; } bb = 14; continue; }
            14 => { fl_l = ((xmm7 as u32)) as u32 as u64; fl_r = ((xmm8 as u32)) as u32 as u64; xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x24u64))) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((xmm0 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((xmm1 as u32)) as u32); if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 20; continue; } bb = 15; continue; }
            15 => { fl_l = ((xmm7 as u32)) as u32 as u64; fl_r = ((xmm8 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 20; continue; } bb = 16; continue; }
            16 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x38u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 21; continue; } bb = 18; continue; }
            18 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 21; continue; } bb = 19; continue; }
            19 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = r8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 21; continue; }
            20 => { rax = 0x50; fl_l = ((r14 as u8)) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { rax = 0x1; bb = 22; continue; }
            22 => { xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; rsp = ((rsp).wrapping_add(0x88u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            23 => { xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; rax = 0x1; rsp = ((rsp).wrapping_add(0x88u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141599bb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14159f4e0; const P1: u64 = 0x143426ea4; const P2: u64 = 0x141605350; const P3: u64 = 0x141597e90;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbx = rdx; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm6) as u128); rsi = rcx; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm7) as u128); rcx = rbx; crate::simd_ops::wu32(r9, (0x0u32) as u32); rdx = rsp.wrapping_add(0x60); r14 = r9; rbp = r8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm6 = ((crate::simd_ops::ru32(P1)) as u32) as u128; xmm7 = 0u128; xmm1 = ((crate::simd_ops::ru32(rax)) as u32) as u128; xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as u32) as u128; crate::simd_ops::wu32(rsi, (0x42480000u32) as u32); crate::simd_ops::wu32(rsi.wrapping_add(0xcu64), ((xmm1 as u32)) as u32); xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(xmm7 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm7 as u32) }).to_bits() as u128); crate::simd_ops::wu32(rsi.wrapping_add(0x10u64), ((xmm0 as u32)) as u32); fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm1 = (xmm6) as u128; bb = 2; continue; }
            2 => { rax = ((xmm1 as u32)) as u32 as u64; fl_l = ((((rax as u32)) >> (31u32)) & 1) as u64; rax = (((rax as u32)) & !((1u32) << (31u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7f800000; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu32(rsi, ((xmm1 as u32)) as u32); bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = r14; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0xf8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(0x78u64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(r14))).to_bits() as u128); crate::simd_ops::wu32(r14, ((xmm0 as u32)) as u32); fl_l = ((xmm7 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 12; continue; } bb = 6; continue; }
            6 => { rbx = ((crate::simd_ops::ru16(rax.wrapping_add(0x38u64))) as u32) as u32 as u64; rcx = rbp; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rdi); rdi = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rax; rdx = ((rbx as u32)) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm3 = ((crate::simd_ops::ru32(r14)) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm3 as u32)) as u32 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; xmm1 = ((crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32) as u128; xmm2 = ((crate::simd_ops::ru32(rsi.wrapping_add(0xcu64))) as u32) as u128; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 8; continue; }
            8 => { xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(xmm7 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm7 as u32) }).to_bits() as u128); fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { xmm6 = (xmm2) as u128; bb = 10; continue; }
            10 => { rax = ((xmm6 as u32)) as u32 as u64; fl_l = ((((rax as u32)) >> (31u32)) & 1) as u64; rax = (((rax as u32)) & !((1u32) << (31u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7f800000; rax = rsi; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 13; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu32(rsi, ((xmm6 as u32)) as u32); bb = 13; continue; }
            12 => { rax = rsi; bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141599ce0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x8; const P2: u64 = 0x4c; const P3: u64 = 0x68; const P4: u64 = 0x20; const P5: u64 = 0x18; const P6: u64 = 0x1426452a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 10; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rcx = ((rcx).wrapping_add(0x18u64)) as u64; rdi = ((rdx as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rsi.wrapping_add(P1))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 2; continue; }
            2 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rdi as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rsi)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 6; continue; }
            6 => { rax = 0; fl_l = 0; fl_r = 0; bb = 7; continue; }
            7 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(P4), (1) as u32)) as u32 as u64; bb = 9; continue; }
            9 => { rcx = ((rsi.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 10; continue; }
            10 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141599d60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x141599da0;
    let (mut rax, mut r10) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x78u64))) as u32 as u64;
    r10 = r9;
    crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rax as u32)) as u32);
    r9 = ((rdx.wrapping_add(P0)) as u64) as u64;
    rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x70u64))) as u32 as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r10);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r8);
    r8 = (crate::simd_ops::ru64(rdx)) as u64;
    rdx = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141599da0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x141524330; const P2: u64 = 0x1415864b0; const P3: u64 = 0x143426890; const P4: u64 = 0x14159b460; const P5: u64 = 0x1426452a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rcx; rbp = r9; rcx = ((rcx).wrapping_add(0x18u64)) as u64; r12 = r8; r13 = ((rdx as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = 0x70; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 1; continue; }
            1 => { xmm0 = 0u128; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu128(rax, (xmm0) as u128); r14 = 0x1; crate::simd_ops::wu128(rax.wrapping_add(0x10u64), (xmm0) as u128); crate::simd_ops::wu128(rax.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::wu128(rax.wrapping_add(0x30u64), (xmm0) as u128); crate::simd_ops::wu128(rax.wrapping_add(0x40u64), (xmm0) as u128); crate::simd_ops::wu128(rax.wrapping_add(0x50u64), (xmm0) as u128); crate::simd_ops::wu128(rax.wrapping_add(0x60u64), (xmm0) as u128); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rcx); crate::simd_ops::wu64(rax.wrapping_add(0x40u64), rcx); crate::simd_ops::wu32(rax.wrapping_add(0x48u64), ((rcx as u32)) as u32); crate::simd_ops::wu64(rax.wrapping_add(0x38u64), rcx); crate::simd_ops::wu32(rax.wrapping_add(0x20u64), ((r14 as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x78u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x24u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rdi.wrapping_add(0x28u64), r12); crate::simd_ops::wu64(rdi.wrapping_add(0x30u64), rcx); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x4cu64), ((r13 as u32)) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x40u64), rax); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x10u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x48u64), ((rax as u32)) as u32); rbp = (crate::simd_ops::ru64(rbp)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            3 => { fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::lk_add_u32(rbp, (1) as u32)) as u32 as u64; bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x38u64), rbp); rbp = rsi.wrapping_add(0x10); crate::simd_ops::wu64(rdi.wrapping_add(0x58u64), rax); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x70u64))) as u32 as u64; rax = ((((rax as u32)) & (0xfff02004u32))) as u32 as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x50u64), rcx); fl_l = rcx; fl_r = rcx; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { crate::simd_ops::ru32(rsp.wrapping_add(0x70u64)) } else { (rax as u32) }) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x60u64), ((rax as u32)) as u32); rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rbp)) as u32 as u64; xmm1 = 0u128; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rdx) as i64) as f32).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P3)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x4cu64))) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x68u64), rcx); rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rax.wrapping_add(rdx.wrapping_mul(8)), rdi); fl_l = (crate::simd_ops::lk_add_u32(rbp, (1) as u32)) as u32 as u64; bb = 11; continue; }
            10 => { r14 = 0x34; bb = 11; continue; }
            11 => { rcx = rsi.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = ((r14 as u32)) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141599f20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141599da0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x68u64)) as u64;
    rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xa0u64))) as u32 as u64;
    xmm0 = 0u128;
    crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x98u64))) as u32 as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9);
    r9 = rsp.wrapping_add(0x40);
    crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128);
    crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), (0x0u32) as u32);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x68u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141599f70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x8; const P2: u64 = 0x4c; const P3: u64 = 0x68; const P4: u64 = 0x60; const P5: u64 = 0x50; const P6: u64 = 0x18; const P7: u64 = 0x1426452a8; const P8: u64 = 0x142645790; const P9: u64 = 0x143931a58; const P10: u64 = 0x40; const P11: u64 = 0x142645748;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rbx = ((rdx as u32)) as u32 as u64; rcx = ((rcx).wrapping_add(0x18u64)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbx as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::lk_and_u32(rax.wrapping_add(P4), (0xfff00000u32) as u32)) as u32 as u64; fl_r = fl_l; crate::simd_ops::wu64(rax.wrapping_add(P5), (0x0u64) as u64); bb = 7; continue; }
            7 => { rcx = ((rdi.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(P9)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P10))) as u64; rdx = 0xffffffff; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P11), rax), rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159a1b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415242e0; const P1: u64 = 0x143930f68; const P2: u64 = 0x1415d0c90; const P3: u64 = 0x14160c070; const P4: u64 = 0x1415af340; const P5: u64 = 0x1415864b0; const P6: u64 = 0x142645550; const P7: u64 = 0x1426452a8; const P8: u64 = 0x142645560;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; fl_l = (crate::simd_ops::ru32(r8.wrapping_add(0x18u64))) as u32 as u64; fl_r = 0x0; rbx = r8; rdi = ((rdx as u32)) as u32 as u64; rsi = rcx; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 35; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(r8.wrapping_add(0x20u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 35; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbp); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), r14); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), r15); r15 = 0; fl_l = 0; fl_r = 0; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(r8.wrapping_add(0x18u64), ((r15 as u32)) as u32); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), r15); crate::simd_ops::wu32(rbx.wrapping_add(0x1cu64), ((r15 as u32)) as u32); bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x60u64))) as u32 as u64; fl_r = 0x400000; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rdx = ((rdi as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; r14 = (crate::simd_ops::ru64(rbx.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru32(rbx.wrapping_add(0x60u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x24u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rdi as u32)) as u32); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 16; continue; } bb = 7; continue; }
            7 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rdi as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rsi)) as u64; r8 = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; rdx = r15; rax = (crate::simd_ops::ru64(r8)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x4cu64))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rdx = rax; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x68u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 11; continue; }
            11 => { bb = 16; continue; }
            12 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x68u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rdx.wrapping_add(0x68u64), rax); bb = 15; continue; }
            14 => { crate::simd_ops::wu64(r8, rax); bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::lk_sub_u32(rsi.wrapping_add(0x10u64), (1) as u32)) as u32 as u64; bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x4cu64))) as u32 as u64; rdx = rsp.wrapping_add(0x38); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rax as u32)) as u32); rcx = rbx; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((r15 as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x50u64), (0xffu8) as u8); crate::simd_ops::wu8(rsp.wrapping_add(0x58u64), (0xffu8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r15); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x60u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 20; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x64u64))) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rdx = (rdx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 21; continue; }
            20 => { rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 21; continue; }
            21 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x60u64), ((rcx as u32)) as u32); if (((fl_l as u32)) == ((fl_r as u32))) { bb = 25; continue; } bb = 22; continue; }
            22 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 26; continue; } bb = 23; continue; }
            23 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 26; continue; } bb = 24; continue; }
            24 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x64u64))) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 26; continue; } bb = 25; continue; }
            25 => { rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            26 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x60u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 28; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rdi; rcx = 0x4; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 28; continue; }
            28 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 29; continue; }
            29 => { rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 30; continue; }
            30 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 31; continue; }
            31 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((r15 as u32)) as u32); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, r15); crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((r15 as u32)) as u32); bb = 32; continue; }
            32 => { rdx = rbx; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; fl_l = ((rbp as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 34; continue; } bb = 33; continue; }
            33 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x40u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rsi.wrapping_add(0x18); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); rdx = rsp.wrapping_add(0x20); rcx = 0x1; rax = crate::simd_ops::call_stk(r14, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x40u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rsp = ((rsp).wrapping_add(0x70u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            34 => { rcx = rsi.wrapping_add(0x18); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rsp = ((rsp).wrapping_add(0x70u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            35 => { rcx = ((rcx).wrapping_add(0x18u64)) as u64; rsp = ((rsp).wrapping_add(0x70u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159a3d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x8; const P2: u64 = 0x4c; const P3: u64 = 0x68; const P4: u64 = 0x58; const P5: u64 = 0x18; const P6: u64 = 0x1426452a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = rcx; rdi = ((rdx as u32)) as u32 as u64; rcx = ((rcx).wrapping_add(0x18u64)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(r14.wrapping_add(P1))) as u32 as u64; rsi = 0; fl_l = 0; fl_r = 0; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rdi as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(r14)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = rsi; bb = 5; continue; }
            5 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rsi = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; bb = 7; continue; }
            7 => { rcx = ((r14.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = rsi; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159a450(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x8; const P2: u64 = 0x4c; const P3: u64 = 0x68; const P4: u64 = 0x24; const P5: u64 = 0x18; const P6: u64 = 0x1426452a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = rcx; rdi = ((rdx as u32)) as u32 as u64; rcx = ((rcx).wrapping_add(0x18u64)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(r14.wrapping_add(P1))) as u32 as u64; rsi = 0; fl_l = 0; fl_r = 0; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rdi as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(r14)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = rsi; bb = 5; continue; }
            5 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rsi = (crate::simd_ops::ru32(rax.wrapping_add(P4))) as u32 as u64; bb = 7; continue; }
            7 => { rcx = ((r14.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = ((rsi as u32)) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159a4d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x8; const P2: u64 = 0x4c; const P3: u64 = 0x68; const P4: u64 = 0x28; const P5: u64 = 0x18; const P6: u64 = 0x1426452a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rcx; rdi = ((rdx as u32)) as u32 as u64; rcx = ((rcx).wrapping_add(0x18u64)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rsi.wrapping_add(P1))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rdi as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rsi)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdi = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; bb = 8; continue; }
            7 => { rdi = (0xffffffffffffffffu64) as u64; bb = 8; continue; }
            8 => { rcx = ((rsi.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = rdi; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159a550(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x8; const P2: u64 = 0x4c; const P3: u64 = 0x68; const P4: u64 = 0x10; const P5: u64 = 0x18; const P6: u64 = 0x1426452a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rcx; rdi = ((rdx as u32)) as u32 as u64; rcx = ((rcx).wrapping_add(0x18u64)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rsi.wrapping_add(P1))) as u32 as u64; r9 = 0; fl_l = 0; fl_r = 0; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rdi as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rsi)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 5; continue; }
            5 => { bb = 7; continue; }
            6 => { rax = r9; bb = 7; continue; }
            7 => { fl_l = rax; fl_r = rax; rdi = ((rax.wrapping_add(P4)) as u64) as u64; rcx = ((rsi.wrapping_add(P5)) as u64) as u64; rdi = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r9 } else { rdi }) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = rdi; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159a6d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc; const P1: u64 = 0x8; const P2: u64 = 0x141524340; const P3: u64 = 0x141524330; const P4: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rbx = rcx; rax = (crate::simd_ops::ru64(rcx)) as u64; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rdx = ((rbp as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(3u32)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = rdx; rdx = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 11; continue; }
            3 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 12; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 6; continue; }
            6 => { fl_l = rax; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 7; continue; }
            7 => { rdx = 0; fl_l = 0; fl_r = 0; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(rdx.wrapping_mul(8)), rcx); rdx = ((rdx).wrapping_add(1)) as u64; fl_l = rdx; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 9; continue; } bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rbx, rdi); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rbx.wrapping_add(P0), ((rbp as u32)) as u32); bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159a780(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645290; const P1: u64 = 0x40; const P2: u64 = 0x1; const P3: u64 = 0x142645560; const P4: u64 = 0x48; const P5: u64 = 0x143930f98; const P6: u64 = 0x10; const P7: u64 = 0x8; const P8: u64 = 0xc; const P9: u64 = 0x14159a6d0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rdi = 0x1; rdx = ((rdi as u32)) as u32 as u64; r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P1), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((rdi.wrapping_add(P2)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rcx = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rsi = 0; fl_l = 0; fl_r = 0; rax = ((rbx.wrapping_add(P4)) as u64) as u64; crate::simd_ops::wu64(P5, rax); crate::simd_ops::wu32(rbx.wrapping_add(P6), ((rsi as u32)) as u32); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P7))) as u32 as u64; fl_r = 0xb; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P8))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xb; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = 0xb; rcx = rbx; rdx = (((rdx as u32)).wrapping_sub((rax as u32))) as u32 as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdi = 0x2; rax = ((rdi as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { rax = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P7), (0xbu32) as u32); bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = ((rax as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu64(rcx.wrapping_add(rdx.wrapping_mul(8)), rsi); fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(P7))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 7; continue; } bb = 8; continue; }
            8 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = ((rdi as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159a850(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x142645550; const P2: u64 = 0x1426452a8; const P3: u64 = 0x142645560;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rsi = rcx; rdi = ((r9 as u32)) as u32 as u64; rcx = ((rcx).wrapping_add(0x18u64)) as u64; r15 = ((r8 as u32)) as u32 as u64; rbx = ((rdx as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 10; continue; } bb = 1; continue; }
            1 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbx as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rsi)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x4cu64))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x68u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 5; continue; }
            5 => { rax = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x60u64))) as u8 as u64; fl_r = (0x40u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { r14 = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x58u64))) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(0x24u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x3cu64), ((rdi as u32)) as u32); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((rdx as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((r15 as u32)) as u32); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rsi.wrapping_add(0x18); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rdx = rsp.wrapping_add(0x20); rcx = 0x40; rax = crate::simd_ops::call_stk(r14, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x40u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64; crate::simd_ops::wu32(rdi, ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x44u64))) as u32 as u64; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); bb = 11; continue; }
            10 => { rcx = rsi.wrapping_add(0x18); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159a970(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x142645550; const P2: u64 = 0x1426452a8; const P3: u64 = 0x142645560;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm6) as u128); rsi = rcx; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm7) as u128); rcx = ((rcx).wrapping_add(0x18u64)) as u64; xmm7 = (xmm2) as u128; xmm6 = (xmm3) as u128; rdi = ((rdx as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rdi as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rsi)) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x4cu64))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x68u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x60u64))) as u8 as u64; fl_r = (0x8u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x24u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xa0u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xa8u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0xb0u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x48u64), ((rax as u8)) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((xmm7 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x3cu64), ((xmm6 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rdi as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsi.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rsp.wrapping_add(0x20); rcx = 0x8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(rbx, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x40u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            8 => { rcx = rsi.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; r11 = rsp.wrapping_add(0x70); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159aa90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x8; const P2: u64 = 0x4c; const P3: u64 = 0x68; const P4: u64 = 0x60; const P5: u64 = 0x58; const P6: u64 = 0x40; const P7: u64 = 0x28; const P8: u64 = 0x50; const P9: u64 = 0x142645550; const P10: u64 = 0x18; const P11: u64 = 0x1426452a8; const P12: u64 = 0x142645560;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = rcx; rbp = r9; rcx = ((rcx).wrapping_add(0x18u64)) as u64; r14 = ((r8 as u32)) as u32 as u64; rbx = ((rdx as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbx as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P4))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P6))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); rdx = (crate::simd_ops::ru64(rax.wrapping_add(P7))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rax.wrapping_add(P8))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdx); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((r14 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbp); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P10)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P11), rax), rax, rcx, rdx, r8, r9, rsp); rdx = rsp.wrapping_add(0x20); rcx = 0x2; rax = crate::simd_ops::call_stk(rbx, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            8 => { rcx = rdi.wrapping_add(0x18); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P11), rax), rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159ab70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x8; const P2: u64 = 0x4c; const P3: u64 = 0x68; const P4: u64 = 0x60; const P5: u64 = 0x50; const P6: u64 = 0x28; const P7: u64 = 0x58; const P8: u64 = 0x24; const P9: u64 = 0x4; const P10: u64 = 0x40; const P11: u64 = 0x142645550; const P12: u64 = 0x1426452a8; const P13: u64 = 0x142645560;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rdi = rcx; r14 = r8; rcx = ((rcx).wrapping_add(0x18u64)) as u64; rbx = ((rdx as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbx as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P4))) as u8 as u64; fl_r = (0x4u8) as u8 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbp); if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { rbp = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P7))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdx); rdx = (crate::simd_ops::ru32(rax.wrapping_add(P8))) as u32 as u64; rax = (crate::simd_ops::ru32(r14)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r14.wrapping_add(P9))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x3cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(r14.wrapping_add(P1))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P10))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rbx as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((rdx as u32)) as u32); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P11), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x18); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp); rdx = rsp.wrapping_add(0x20); rcx = 0x4; rax = crate::simd_ops::call_stk(rbp, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P13), rax), rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            8 => { rcx = rdi.wrapping_add(0x18); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159ac60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x142645550; const P2: u64 = 0x1426452a8; const P3: u64 = 0x142645560;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 13; continue; } bb = 1; continue; }
            1 => { rax = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); r14 = rcx; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rax.wrapping_sub(0x10u64), rdi); rdi = rdx.wrapping_add(0x8); crate::simd_ops::wu64(rax.wrapping_sub(0x18u64), r15); r15 = ((r8 as u32)) as u32 as u64; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; rcx = r14.wrapping_add(0x18); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbx as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(r14)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x4cu64))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x68u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 6; continue; }
            6 => { rax = 0; fl_l = 0; fl_r = 0; bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x60u64))) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { rsi = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x24u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_sub(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_sub(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x3cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rbx as u32)) as u32); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rcx = r14.wrapping_add(0x18); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rdx = rsp.wrapping_add(0x20); rcx = 0x4; rax = crate::simd_ops::call_stk(rsi, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x40u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            10 => { rcx = r14.wrapping_add(0x18); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rdi = ((rdi).wrapping_add(0x20u64)) as u64; fl_l = r15; fl_r = (0x1u64) as u64; r15 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 12; continue; }
            12 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 13; continue; }
            13 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159ad70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x142645550; const P2: u64 = 0x1426452a8; const P3: u64 = 0x142645560;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rdi = rcx; r14 = r8; rcx = ((rcx).wrapping_add(0x18u64)) as u64; rbx = ((rdx as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 10; continue; } bb = 1; continue; }
            1 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbx as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x4cu64))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x68u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 5; continue; }
            5 => { rax = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { rbp = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x60u64))) as u32 as u64; fl_r = 0x10000; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x58u64))) as u64; xmm0 = ((crate::simd_ops::ru64(r14)) as u64) as u128; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdx); rdx = (crate::simd_ops::ru32(rax.wrapping_add(0x24u64))) as u32 as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((rdx as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rbx as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), ((xmm0 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rdi.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rsp.wrapping_add(0x20); rcx = 0x10000; (rax, xmm0) = crate::simd_ops::call_stk_xmm(rbp, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            10 => { rcx = rdi.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159ae70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x142645550; const P2: u64 = 0x1426452a8; const P3: u64 = 0x142645560;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rdi = rcx; r15 = r9; rcx = ((rcx).wrapping_add(0x18u64)) as u64; r14 = ((r8 as u32)) as u32 as u64; rbx = ((rdx as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbx as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x4cu64))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x68u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { rbp = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x60u64))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x58u64))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; xmm0 = (crate::simd_ops::ru128(r15)) as u128; rax = (crate::simd_ops::ru32(r15.wrapping_add(0x20u64))) as u32 as u64; xmm1 = (crate::simd_ops::ru128(r15.wrapping_add(0x10u64))) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x34u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdx); crate::simd_ops::wu128(rsp.wrapping_add(0x44u64), (xmm1) as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rbx as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x54u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), ((r14 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (0x0u64) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rdi.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rsp.wrapping_add(0x20); rcx = ((r14 as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(rbp, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            8 => { rcx = rdi.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { r11 = rsp.wrapping_add(0x70); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159af70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x8; const P2: u64 = 0x4c; const P3: u64 = 0x68; const P4: u64 = 0x60; const P5: u64 = 0x50; const P6: u64 = 0x58; const P7: u64 = 0x40; const P8: u64 = 0x28; const P9: u64 = 0x24; const P10: u64 = 0x142645550; const P11: u64 = 0x18; const P12: u64 = 0x1426452a8; const P13: u64 = 0x142645560;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = rcx; rbx = ((rdx as u32)) as u32 as u64; rcx = ((rcx).wrapping_add(0x18u64)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbx as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P4))) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { rbp = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P7))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); rdx = (crate::simd_ops::ru64(rax.wrapping_add(P8))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdx); rdx = (crate::simd_ops::ru32(rax.wrapping_add(P9))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((rdx as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rbx as u32)) as u32); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P11)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp); rdx = rsp.wrapping_add(0x20); rcx = 0x80; rax = crate::simd_ops::call_stk(rbp, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P13), rax), rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            9 => { rcx = rdi.wrapping_add(0x18); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159b040(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x142645550; const P2: u64 = 0x1426452a8; const P3: u64 = 0x142645560;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rdi = rcx; r15 = r9; rcx = ((rcx).wrapping_add(0x18u64)) as u64; r14 = r8; rbx = ((rdx as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbx as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x4cu64))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x68u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { rbp = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x60u64))) as u32 as u64; fl_r = 0x2000; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x58u64))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; xmm0 = (crate::simd_ops::ru128(r14)) as u128; rax = (crate::simd_ops::ru32(r14.wrapping_add(0x20u64))) as u32 as u64; xmm1 = (crate::simd_ops::ru128(r14.wrapping_add(0x10u64))) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x34u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdx); crate::simd_ops::wu128(rsp.wrapping_add(0x44u64), (xmm1) as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rbx as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x54u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), (0x2000u32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r15); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rdi.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rsp.wrapping_add(0x20); rcx = 0x2000; (rax, xmm0) = crate::simd_ops::call_stk_xmm(rbp, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            8 => { rcx = rdi.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { r11 = rsp.wrapping_add(0x70); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159b140(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x8; const P2: u64 = 0x4c; const P3: u64 = 0x68; const P4: u64 = 0x60; const P5: u64 = 0x58; const P6: u64 = 0x28; const P7: u64 = 0x10; const P8: u64 = 0x24; const P9: u64 = 0x14; const P10: u64 = 0x40; const P11: u64 = 0x50; const P12: u64 = 0x142645550; const P13: u64 = 0x1426452a8; const P14: u64 = 0x142645560;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; r14 = r8; rcx = ((rcx).wrapping_add(0x18u64)) as u64; rbx = ((rdx as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbx as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P4))) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; crate::simd_ops::wu64(r14, rdx); rdx = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; crate::simd_ops::wu64(r14.wrapping_add(P1), rdx); crate::simd_ops::wu32(r14.wrapping_add(P7), ((rbx as u32)) as u32); rdx = (crate::simd_ops::ru32(rax.wrapping_add(P8))) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(P9), ((rdx as u32)) as u32); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P10))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(P11))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x18); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P13), rax), rax, rcx, rdx, r8, r9, rsp); rdx = r14; rcx = 0x10; rax = crate::simd_ops::call_stk(rbx, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P14), rax), rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            8 => { rcx = rdi.wrapping_add(0x18); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P13), rax), rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159b210(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x8; const P2: u64 = 0x4c; const P3: u64 = 0x68; const P4: u64 = 0x50; const P5: u64 = 0x60; const P6: u64 = 0x58; const P7: u64 = 0x40; const P8: u64 = 0x28; const P9: u64 = 0x24; const P10: u64 = 0x142645550; const P11: u64 = 0x18; const P12: u64 = 0x1426452a8; const P13: u64 = 0x142645560;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = rcx; rbx = ((rdx as u32)) as u32 as u64; rcx = ((rcx).wrapping_add(0x18u64)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbx as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { rbp = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P5))) as u8 as u64; fl_r = (0x20u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P7))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); rdx = (crate::simd_ops::ru64(rax.wrapping_add(P8))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdx); rdx = (crate::simd_ops::ru32(rax.wrapping_add(P9))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((rdx as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rbx as u32)) as u32); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P11)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp); rdx = rsp.wrapping_add(0x20); rcx = 0x20; rax = crate::simd_ops::call_stk(rbp, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P13), rax), rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            9 => { rcx = rdi.wrapping_add(0x18); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159b2e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x14159a1b0; const P2: u64 = 0x14183acc8; const P3: u64 = 0x1426452a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = rcx; rbp = r8; rcx = ((rcx).wrapping_add(0x18u64)) as u64; rdi = ((rdx as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); r9 = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 15; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(r14)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = ((rdi as u32)) as u32 as u64; let d = ((r9 as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rbx = (crate::simd_ops::ru64(rbx.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x4cu64))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x68u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 5; continue; }
            5 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 7; continue; }
            7 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; rcx = r8; rax = ((r8.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rcx = ((rcx).wrapping_add(0x8u64)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 10; continue; }
            10 => { r8 = rbx; rdx = ((rdi as u32)) as u32 as u64; rcx = r14; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            11 => { rax = ((rdx.wrapping_sub(0x1u64)) as u32) as u32 as u64; rax = ((r8.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = ((rax).wrapping_sub(rcx)) as u64; rdx = rcx.wrapping_add(0x8); rax = ((((rax as u32)) & (0xfffffff8u32))) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::lk_sub_u32(rbx.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; bb = 14; continue; }
            14 => { r8 = rbx; rdx = ((rdi as u32)) as u32 as u64; rcx = r14; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            15 => { rcx = r14.wrapping_add(0x18); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159b3d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x8; const P2: u64 = 0x4c; const P3: u64 = 0x68; const P4: u64 = 0x20; const P5: u64 = 0x14159a1b0; const P6: u64 = 0x18; const P7: u64 = 0x1426452a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rbx = ((rdx as u32)) as u32 as u64; rcx = ((rcx).wrapping_add(0x18u64)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbx as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } r8 = (crate::simd_ops::ru64(r8.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P2))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { r8 = (crate::simd_ops::ru64(r8.wrapping_add(P3))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::lk_sub_u32(r8.wrapping_add(P4), (1) as u32); rdx = ((rbx as u32)) as u32 as u64; rcx = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P5, rax, rcx, rdx, r8, r9, rsp); }
            7 => { rcx = ((rdi.wrapping_add(P6)) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159b460(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142689450; const P1: u64 = 0x141524330; const P2: u64 = 0x1415242e0;
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
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbp); rdx = ((rbx.wrapping_mul(8)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rsi); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r12); rsi = rbx; r12 = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r13); r13 = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); r15 = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(rdi, rax); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rdi)) as u64; rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { r8 = 0; fl_l = 0; fl_r = 0; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((r8 as u32)) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rdi, r13); rax = 0x2; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((r12 as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((r15 as u32)) as u32); bb = 26; continue; }
            9 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { fl_l = rdx; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rdx = rdi; rcx = rdi.wrapping_add(0xc); rax = ((rbx as u32)) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdx, rbp); rbp = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu32(rcx, ((rbx as u32)) as u32); bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 16; continue; } bb = 14; continue; }
            14 => { rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rdx, r8); rbx = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; bb = 16; continue; }
            16 => { fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 17; continue; }
            17 => { fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 23; continue; } bb = 18; continue; }
            18 => { r10 = r13; r11 = r12; bb = 19; continue; }
            19 => { rcx = (crate::simd_ops::ru64(r10)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 20; continue; }
            20 => { bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4cu64))) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(0x68u64))) as u64; let d = ((rbp as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rdi)) as u64; r8 = ((rdx as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x68u64), rdx); rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rax.wrapping_add(r8.wrapping_mul(8)), rcx); rcx = r9; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 21; continue; } bb = 22; continue; }
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
pub unsafe extern "win64" fn fn_14159b5e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x14153e940; const P2: u64 = 0x1415af330; const P3: u64 = 0x1426452a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = r9; r12 = r8; rbp = ((rdx as u32)) as u32 as u64; rdi = rcx; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rbx = rcx.wrapping_add(0x18); rcx = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r15); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; r15 = 0x1; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 23; continue; } bb = 2; continue; }
            2 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbp as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rdi)) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x4cu64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x68u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 6; continue; }
            6 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 7; continue; }
            7 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 8; continue; }
            8 => { rbp = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r14); fl_l = ((rbp as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rcx = rdi.wrapping_add(0x10); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 21; continue; } bb = 10; continue; }
            10 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 21; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x18u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rdx, r12); fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x30u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 20; continue; } bb = 13; continue; }
            13 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rdi.wrapping_add(0x30u64), rsi); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x60u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 19; continue; } bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x64u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 19; continue; } bb = 16; continue; }
            16 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x60u64), ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 18; continue; } bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x64u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 20; continue; } bb = 18; continue; }
            18 => { rcx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x60u64))) as u32 as u64; crate::simd_ops::wu32(rax, ((rcx as u32)) as u32); bb = 22; continue; }
            19 => { fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x60u64), ((rax as u32)) as u32); bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x60u64))) as u32 as u64; crate::simd_ops::wu32(rax, ((rcx as u32)) as u32); bb = 22; continue; }
            21 => { r15 = 0x2; bb = 22; continue; }
            22 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 23; continue; }
            23 => { rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = ((r15 as u32)) as u32 as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            24 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = 0x2; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159b730(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x1415242e0; const P4: u64 = 0x1c; const P5: u64 = 0x68; const P6: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rbp = 0; fl_l = 0; fl_r = 0; rsi = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 16; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = ((rbp as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); rdi = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdi = (crate::simd_ops::ru64(rcx.wrapping_add(rbx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 5; continue; }
            5 => { bb = 7; continue; }
            6 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(P2), ((rbp as u32)) as u32); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(P1), rbp); crate::simd_ops::wu32(rdi.wrapping_add(P4), ((rbp as u32)) as u32); bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64; rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru32(rsi.wrapping_add(P0))) as u32 as u64; bb = 11; continue; }
            11 => { rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 14; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rbx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 13; continue; }
            13 => { bb = 7; continue; }
            14 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 16; continue; }
            16 => { rdx = (crate::simd_ops::ru64(rsi)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsi.wrapping_add(P0), ((rbp as u32)) as u32); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi, rbp); crate::simd_ops::wu32(rsi.wrapping_add(P6), ((rbp as u32)) as u32); bb = 18; continue; }
            18 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159b7f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x8; const P2: u64 = 0x4c; const P3: u64 = 0x68; const P4: u64 = 0x28; const P5: u64 = 0x24; const P6: u64 = 0x20; const P7: u64 = 0x18; const P8: u64 = 0x1426452a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = ((r9 as u32)) as u32 as u64; r14 = r8; rdi = ((rdx as u32)) as u32 as u64; rsi = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { rcx = ((rcx).wrapping_add(0x18u64)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rsi.wrapping_add(P1))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 2; continue; }
            2 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rdi as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rsi)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 6; continue; }
            6 => { rax = 0; fl_l = 0; fl_r = 0; bb = 7; continue; }
            7 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P5))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::lk_add_u32(rax.wrapping_add(P6), (1) as u32); rcx = ((rsi.wrapping_add(P7)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 13; continue; }
            11 => { rcx = ((rsi.wrapping_add(P7)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159b8a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x20; const P2: u64 = 0x78; const P3: u64 = 0x1; const P4: u64 = 0x9; const P5: u64 = 0x10;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbx = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); rdi = rcx; xmm6 = 0u128; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 14; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 14; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 4; continue; }
            4 => { r11 = 0xfffffff8; bb = 5; continue; }
            5 => { r9 = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 6; continue; }
            6 => { r10 = ((crate::simd_ops::ru16(r9)) as u32) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; bb = 7; continue; }
            7 => { r8 = ((rdx.wrapping_add(P3)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru16(r9.wrapping_add(r8.wrapping_mul(2)))) as u16 as u64; fl_r = ((rbx as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rdx = ((r8 as u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 7; continue; } bb = 9; continue; }
            9 => { bb = 12; continue; }
            10 => { rcx = ((r10.wrapping_mul(2).wrapping_add(P4)) as u32) as u32 as u64; r8 = ((rdx as u32)) as u32 as u64; rcx = (((rcx) & (r11))) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(r9)) as u64; fl_l = ((r8).wrapping_add(rcx)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(crate::simd_ops::ru32(r8))).to_bits() as u128); bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 13; continue; }
            13 => { xmm0 = (xmm6) as u128; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            14 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; xmm0 = 0u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159b970(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524340; const P1: u64 = 0x141524330; const P2: u64 = 0x1415242e0; const P3: u64 = 0x143930f44;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rsi = r8; r12 = rdx; rbx = rcx; r13 = 0x1; r9 = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rax = ((rax).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rcx, rax); r15 = ((r9 as u32)) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 25; continue; } bb = 1; continue; }
            1 => { r14 = 0; fl_l = 0; fl_r = 0; fl_l = ((r15 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r8.wrapping_add(0x54u64))) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 14; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(r8.wrapping_add(0x48u64))) as u64; rdx = ((r9 as u32)) as u32 as u64; rbp = (crate::simd_ops::ru32(r8.wrapping_add(0x50u64))) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rdx = ((rdx).wrapping_shl(3u32)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { r8 = rdx; rdx = rax; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 4; continue; }
            4 => { bb = 12; continue; }
            5 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x48u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { fl_l = rax; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 8; continue; }
            8 => { rdx = r14; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x48u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(rdx.wrapping_mul(8)), rcx); rdx = ((rdx).wrapping_add(1)) as u64; fl_l = rdx; fl_r = rbp; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 9; continue; } bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x48u64))) as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rsi.wrapping_add(0x48u64), rdi); rax = (((r13 as u8)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x54u64), ((r15 as u32)) as u32); bb = 12; continue; }
            12 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rcx = 0x34; rcx = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (r13 as u32) } else { (rcx as u32) }) as u32 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = ((rcx as u32)) as u32 as u64; bb = 26; continue; }
            14 => { fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 25; continue; } bb = 15; continue; }
            15 => { bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rbp = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = ((rax).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu64(rbx, rax); r13 = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(rbx, rax); r10 = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(rbx, rax); rcx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rax = ((rax).wrapping_add(1)) as u64; r9 = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu64(rbx, rax); r9 = ((((r9 as u32)) & (0x7fu32))) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if (((((fl_l as u8)) & ((fl_r as u8))) as i8) >= 0) { bb = 19; continue; } bb = 17; continue; }
            17 => { bb = 18; continue; }
            18 => { r8 = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rcx = ((r9 as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(7u32)) as u32 as u64; rax = ((rax).wrapping_add(1)) as u64; rdx = ((r8 as u32)) as u32 as u64; crate::simd_ops::wu64(rbx, rax); rdx = ((((rdx as u32)) & (0x7fu32))) as u32 as u64; r9 = ((rdx as u32)) as u32 as u64; r9 = ((((r9 as u32)) | ((rcx as u32)))) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if (((((fl_l as u8)) & ((fl_r as u8))) as i8) < 0) { bb = 18; continue; } bb = 19; continue; }
            19 => { r8 = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = ((rax).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu64(rbx, rax); rdx = rax.wrapping_add(0x1); r11 = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; crate::simd_ops::wu64(rbx, rdx); rdi = ((crate::simd_ops::ru16(rdx)) as u32) as u32 as u64; rdx = ((rdx).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rbx, rdx); fl_l = ((r10 as u8)) as u8 as u64; fl_r = (0x6u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 21; continue; } bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::ru32(P3)) as u32 as u64; fl_r = 0x1; rcx = 0x2; rax = 0x5; rcx = (if (((fl_l as u32)) == ((fl_r as u32))) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; bb = 22; continue; }
            21 => { rcx = (((r10 as u8)) as u32) as u32 as u64; bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rdi as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdx); rdx = ((rbp as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r11 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r8 as u32)) as u32); r8 = (((r13 as u8)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r9 as u32)) as u32); r9 = (((rcx as u8)) as u32) as u32 as u64; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x1c0u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 26; continue; } bb = 23; continue; }
            23 => { rax = ((((rdi as u32)) as i32).wrapping_mul((0xfffffff4u32) as i32) as u32) as u32 as u64; rcx = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(2u32)) as u64; r14 = (((r14 as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::lk_add_u64(rbx, (rcx) as u64); crate::simd_ops::lk_add_u32(r12, ((rax as u32)) as u32); fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 16; continue; } bb = 24; continue; }
            24 => { r13 = 0x1; bb = 25; continue; }
            25 => { rax = ((r13 as u32)) as u32 as u64; bb = 26; continue; }
            26 => { r11 = rsp.wrapping_add(0x50); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159bb80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x28; const P3: u64 = 0x12; const P4: u64 = 0x13;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu16(rcx.wrapping_add(P0), ((rdx as u16)) as u16);
    crate::simd_ops::wu64(rcx, rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    rax = rcx;
    crate::simd_ops::wu8(rcx.wrapping_add(P3), ((r8 as u8)) as u8);
    crate::simd_ops::wu8(rcx.wrapping_add(P4), ((r9 as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159bba0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xffffffffffffffe0; const P2: u64 = 0xffffffffffffffc8; const P3: u64 = 0xffffffffffffffd0; const P4: u64 = 0xffffffffffffffd8; const P5: u64 = 0xfffffffffffffff0; const P6: u64 = 0xffffffffffffffe8; const P7: u64 = 0x1415ae300; const P8: u64 = 0x142689748; const P9: u64 = 0x70; const P10: u64 = 0x78; const P11: u64 = 0x80;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rax = rsp;
    crate::simd_ops::wu64(rax.wrapping_add(P0), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x50u64)) as u64;
    rdi = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu8(rax.wrapping_add(P1), (0xffu8) as u8);
    rdx = ((rax.wrapping_add(P2)) as u64) as u64;
    crate::simd_ops::wu64(rax.wrapping_add(P2), rdi);
    crate::simd_ops::wu32(rax.wrapping_add(P3), ((rdi as u32)) as u32);
    rbx = rcx;
    crate::simd_ops::wu32(rax.wrapping_add(P4), ((rdi as u32)) as u32);
    crate::simd_ops::wu64(rax.wrapping_add(P5), rdi);
    crate::simd_ops::wu8(rax.wrapping_add(P6), (0xffu8) as u8);
    rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp);
    rax = P8;
    crate::simd_ops::wu64(rbx.wrapping_add(P9), rdi);
    crate::simd_ops::wu64(rbx, rax);
    rax = rbx;
    crate::simd_ops::wu64(rbx.wrapping_add(P10), rdi);
    crate::simd_ops::wu32(rbx.wrapping_add(P11), ((rdi as u32)) as u32);
    crate::simd_ops::wu32(rbx.wrapping_add(0x98u64), ((rdi as u32)) as u32);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64;
    rsp = ((rsp).wrapping_add(0x50u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159bc00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142689730; const P1: u64 = 0x142676f50; const P2: u64 = 0x8; const P3: u64 = 0x1415bd140; const P4: u64 = 0x142689798; const P5: u64 = 0x28; const P6: u64 = 0x10; const P7: u64 = 0x1415beb50; const P8: u64 = 0x1426897c0; const P9: u64 = 0x142677588; const P10: u64 = 0x40; const P11: u64 = 0x142689a10; const P12: u64 = 0x142689a38;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = P0; rsi = rcx; crate::simd_ops::wu64(rcx, rax); rax = P1; crate::simd_ops::wu64(rcx.wrapping_add(P2), rax); rcx = ((rcx).wrapping_add(0x10u64)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = P4; rcx = ((rsi.wrapping_add(P5)) as u64) as u64; crate::simd_ops::wu64(rsi.wrapping_add(P6), rax); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = P8; crate::simd_ops::wu64(rsi, rax); rcx = 0; fl_l = 0; fl_r = 0; rax = P9; crate::simd_ops::wu64(rsi.wrapping_add(P10), rcx); crate::simd_ops::wu64(rsi.wrapping_add(P2), rax); rax = P11; crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), rax); rax = P12; crate::simd_ops::wu64(rsi.wrapping_add(0x28u64), rax); rax = rsi.wrapping_add(0xa8); crate::simd_ops::wu64(rax, rsi); crate::simd_ops::wu64(rsi.wrapping_add(0xb8u64), rax); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rcx); rax = 0xfc00; crate::simd_ops::lk_and_u16(rsi.wrapping_add(0xd2u64), ((rax as u16)) as u16); rax = ((crate::simd_ops::ru8(rsi.wrapping_add(0xd4u64))) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x40u8))) as u8 as u64)); crate::simd_ops::wu64(rsi.wrapping_add(0x48u64), rcx); rax = (rax & !0xffu64) | ((((((rax as u8)) | (0x10u8))) as u8 as u64)); crate::simd_ops::wu64(rsi.wrapping_add(0x50u64), rcx); crate::simd_ops::lk_and_u8(rsi.wrapping_add(0xd6u64), (0xf8u8) as u8); crate::simd_ops::wu8(rsi.wrapping_add(0xd4u64), ((rax as u8)) as u8); rax = rsi; crate::simd_ops::wu64(rsi.wrapping_add(0x58u64), rcx); crate::simd_ops::wu64(rsi.wrapping_add(0x60u64), rcx); crate::simd_ops::wu64(rsi.wrapping_add(0x68u64), rcx); crate::simd_ops::wu64(rsi.wrapping_add(0x70u64), rcx); crate::simd_ops::wu64(rsi.wrapping_add(0x78u64), rcx); crate::simd_ops::wu64(rsi.wrapping_add(0x80u64), rcx); crate::simd_ops::wu64(rsi.wrapping_add(0x88u64), rcx); crate::simd_ops::wu64(rsi.wrapping_add(0x90u64), rcx); crate::simd_ops::wu64(rsi.wrapping_add(0x98u64), rcx); crate::simd_ops::wu64(rsi.wrapping_add(0xc0u64), rcx); crate::simd_ops::wu64(rsi.wrapping_add(0xc8u64), rcx); crate::simd_ops::wu8(rsi.wrapping_add(0xd5u64), ((rcx as u8)) as u8); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159bd10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r9 = 0; fl_l = 0; fl_r = 0; r8 = 0x2; crate::simd_ops::wu64(rcx.wrapping_add(0x7000u64), r9); crate::simd_ops::wu32(rcx.wrapping_add(0x7008u64), ((r9 as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x7010u64), r9); bb = 1; continue; }
            1 => { rdx = ((r8.wrapping_sub(0x2u64)) as u32) as u32 as u64; rax = (((rdx) as i64).wrapping_mul((0xe0u64) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x7010u64))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rcx.wrapping_add(0x7010u64), rax); crate::simd_ops::wu64(rax, r9); bb = 4; continue; }
            3 => { crate::simd_ops::wu64(rax, rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x7010u64), rax); bb = 4; continue; }
            4 => { crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x7008u64), (1) as u32); rdx = ((r8.wrapping_sub(0x1u64)) as u32) as u32 as u64; rax = (((rdx) as i64).wrapping_mul((0xe0u64) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x7010u64))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rcx.wrapping_add(0x7010u64), rax); crate::simd_ops::wu64(rax, r9); bb = 7; continue; }
            6 => { crate::simd_ops::wu64(rax, rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x7010u64), rax); bb = 7; continue; }
            7 => { crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x7008u64), (1) as u32); rax = ((r8 as u32)) as u32 as u64; rdx = (((rax) as i64).wrapping_mul((0xe0u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x7010u64))) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rcx.wrapping_add(0x7010u64), rdx); crate::simd_ops::wu64(rdx, r9); bb = 10; continue; }
            9 => { crate::simd_ops::wu64(rdx, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x7010u64), rdx); bb = 10; continue; }
            10 => { crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x7008u64), (1) as u32); rdx = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = (((rdx) as i64).wrapping_mul((0xe0u64) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x7010u64))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rcx.wrapping_add(0x7010u64), rax); crate::simd_ops::wu64(rax, r9); bb = 13; continue; }
            12 => { crate::simd_ops::wu64(rax, rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x7010u64), rax); bb = 13; continue; }
            13 => { crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x7008u64), (1) as u32); rdx = ((r8.wrapping_add(0x2u64)) as u32) as u32 as u64; rax = (((rdx) as i64).wrapping_mul((0xe0u64) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x7010u64))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rcx.wrapping_add(0x7010u64), rax); crate::simd_ops::wu64(rax, r9); bb = 16; continue; }
            15 => { crate::simd_ops::wu64(rax, rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x7010u64), rax); bb = 16; continue; }
            16 => { crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x7008u64), (1) as u32); rdx = ((r8.wrapping_add(0x3u64)) as u32) as u32 as u64; rax = (((rdx) as i64).wrapping_mul((0xe0u64) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x7010u64))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { crate::simd_ops::wu64(rcx.wrapping_add(0x7010u64), rax); crate::simd_ops::wu64(rax, r9); bb = 19; continue; }
            18 => { crate::simd_ops::wu64(rax, rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x7010u64), rax); bb = 19; continue; }
            19 => { crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x7008u64), (1) as u32); rdx = ((r8.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = (((rdx) as i64).wrapping_mul((0xe0u64) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x7010u64))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { crate::simd_ops::wu64(rcx.wrapping_add(0x7010u64), rax); crate::simd_ops::wu64(rax, r9); bb = 22; continue; }
            21 => { crate::simd_ops::wu64(rax, rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x7010u64), rax); bb = 22; continue; }
            22 => { crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x7008u64), (1) as u32); rdx = ((r8.wrapping_add(0x5u64)) as u32) as u32 as u64; rax = (((rdx) as i64).wrapping_mul((0xe0u64) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x7010u64))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { crate::simd_ops::wu64(rcx.wrapping_add(0x7010u64), rax); crate::simd_ops::wu64(rax, r9); bb = 25; continue; }
            24 => { crate::simd_ops::wu64(rax, rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x7010u64), rax); bb = 25; continue; }
            25 => { crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x7008u64), (1) as u32); rdx = ((r8.wrapping_add(0x6u64)) as u32) as u32 as u64; rax = (((rdx) as i64).wrapping_mul((0xe0u64) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x7010u64))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 27; continue; } bb = 26; continue; }
            26 => { crate::simd_ops::wu64(rcx.wrapping_add(0x7010u64), rax); crate::simd_ops::wu64(rax, r9); bb = 28; continue; }
            27 => { crate::simd_ops::wu64(rax, rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x7010u64), rax); bb = 28; continue; }
            28 => { crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x7008u64), (1) as u32); rdx = ((r8.wrapping_add(0x7u64)) as u32) as u32 as u64; rax = (((rdx) as i64).wrapping_mul((0xe0u64) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x7010u64))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 30; continue; } bb = 29; continue; }
            29 => { crate::simd_ops::wu64(rcx.wrapping_add(0x7010u64), rax); crate::simd_ops::wu64(rax, r9); bb = 31; continue; }
            30 => { crate::simd_ops::wu64(rax, rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x7010u64), rax); bb = 31; continue; }
            31 => { crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x7008u64), (1) as u32); rdx = ((r8.wrapping_add(0x8u64)) as u32) as u32 as u64; rax = (((rdx) as i64).wrapping_mul((0xe0u64) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x7010u64))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 33; continue; } bb = 32; continue; }
            32 => { crate::simd_ops::wu64(rcx.wrapping_add(0x7010u64), rax); crate::simd_ops::wu64(rax, r9); bb = 34; continue; }
            33 => { crate::simd_ops::wu64(rax, rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x7010u64), rax); bb = 34; continue; }
            34 => { crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x7008u64), (1) as u32); rdx = ((r8.wrapping_add(0x9u64)) as u32) as u32 as u64; rax = (((rdx) as i64).wrapping_mul((0xe0u64) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x7010u64))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 36; continue; } bb = 35; continue; }
            35 => { crate::simd_ops::wu64(rcx.wrapping_add(0x7010u64), rax); crate::simd_ops::wu64(rax, r9); bb = 37; continue; }
            36 => { crate::simd_ops::wu64(rax, rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x7010u64), rax); bb = 37; continue; }
            37 => { crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x7008u64), (1) as u32); rdx = ((r8.wrapping_add(0xau64)) as u32) as u32 as u64; rax = (((rdx) as i64).wrapping_mul((0xe0u64) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x7010u64))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 39; continue; } bb = 38; continue; }
            38 => { crate::simd_ops::wu64(rcx.wrapping_add(0x7010u64), rax); crate::simd_ops::wu64(rax, r9); bb = 40; continue; }
            39 => { crate::simd_ops::wu64(rax, rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x7010u64), rax); bb = 40; continue; }
            40 => { crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x7008u64), (1) as u32); rdx = ((r8.wrapping_add(0xbu64)) as u32) as u32 as u64; rax = (((rdx) as i64).wrapping_mul((0xe0u64) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x7010u64))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 42; continue; } bb = 41; continue; }
            41 => { crate::simd_ops::wu64(rcx.wrapping_add(0x7010u64), rax); crate::simd_ops::wu64(rax, r9); bb = 43; continue; }
            42 => { crate::simd_ops::wu64(rax, rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x7010u64), rax); bb = 43; continue; }
            43 => { crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x7008u64), (1) as u32); rdx = ((r8.wrapping_add(0xcu64)) as u32) as u32 as u64; rax = (((rdx) as i64).wrapping_mul((0xe0u64) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x7010u64))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 45; continue; } bb = 44; continue; }
            44 => { crate::simd_ops::wu64(rcx.wrapping_add(0x7010u64), rax); crate::simd_ops::wu64(rax, r9); bb = 46; continue; }
            45 => { crate::simd_ops::wu64(rax, rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x7010u64), rax); bb = 46; continue; }
            46 => { crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x7008u64), (1) as u32); rdx = ((r8.wrapping_add(0xdu64)) as u32) as u32 as u64; rax = (((rdx) as i64).wrapping_mul((0xe0u64) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x7010u64))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 48; continue; } bb = 47; continue; }
            47 => { crate::simd_ops::wu64(rcx.wrapping_add(0x7010u64), rax); crate::simd_ops::wu64(rax, r9); bb = 49; continue; }
            48 => { crate::simd_ops::wu64(rax, rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x7010u64), rax); bb = 49; continue; }
            49 => { crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x7008u64), (1) as u32); r8 = (((r8 as u32)).wrapping_add(0x10u32)) as u32 as u64; rax = ((r8.wrapping_sub(0x2u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 1; continue; } bb = 50; continue; }
            50 => { rax = rcx; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159c0b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426897c0; const P1: u64 = 0x142677588; const P2: u64 = 0x142689a10; const P3: u64 = 0x142689a38; const P4: u64 = 0x1415bd8a0; const P5: u64 = 0x141599340; const P6: u64 = 0x1415a2050; const P7: u64 = 0x143931698; const P8: u64 = 0x14159da30; const P9: u64 = 0x1415242e0; const P10: u64 = 0x141607a50; const P11: u64 = 0x1416079b0; const P12: u64 = 0x14160e340; const P13: u64 = 0x14159dbb0; const P14: u64 = 0x1415c2b30; const P15: u64 = 0x1415aee60; const P16: u64 = 0x14160e9b0; const P17: u64 = 0x1415beb80; const P18: u64 = 0x1415bead0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = P0; rbx = rcx; crate::simd_ops::wu64(rcx, rax); rax = P1; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rax); rax = P2; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); rax = P3; crate::simd_ops::wu64(rcx.wrapping_add(0x28u64), rax); rcx = ((rcx).wrapping_add(0x28u64)) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rbp = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x70u64))) as u64; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 23; continue; } bb = 1; continue; }
            1 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x78u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x70u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0xd0u64))) as u16 as u64; fl_r = ((rbp as u16)) as u16 as u64; if ((((fl_l as u16)) as i16) > (((fl_r as u16)) as i16)) { bb = 12; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0xd4u64))) as u16 as u64; fl_r = ((rbp as u16)) as u16 as u64; if ((((fl_l as u16)) as i16) > (((fl_r as u16)) as i16)) { bb = 12; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0xb8u64))) as u16 as u64; fl_r = ((rbp as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 12; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0xbau64))) as u16 as u64; fl_r = ((rbp as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 12; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0xd8u64))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rcx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x70u64))) as u64; rcx = P7; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(0x70u64), rbp); bb = 12; continue; }
            12 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x80u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x70u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0xd0u64))) as u16 as u64; fl_r = ((rbp as u16)) as u16 as u64; if ((((fl_l as u16)) as i16) > (((fl_r as u16)) as i16)) { bb = 23; continue; } bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0xd4u64))) as u16 as u64; fl_r = ((rbp as u16)) as u16 as u64; if ((((fl_l as u16)) as i16) > (((fl_r as u16)) as i16)) { bb = 23; continue; } bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0xb8u64))) as u16 as u64; fl_r = ((rbp as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 23; continue; } bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0xbau64))) as u16 as u64; fl_r = ((rbp as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 23; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 23; continue; } bb = 19; continue; }
            19 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 23; continue; } bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0xd8u64))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 22; continue; }
            22 => { rcx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x70u64))) as u64; rcx = P7; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(0x70u64), rbp); bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x78u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x78u64), rbp); bb = 25; continue; }
            25 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x80u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x80u64), rbp); bb = 27; continue; }
            27 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x88u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 28; continue; }
            28 => { rcx = 0x2; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 29; continue; }
            29 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x68u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 30; continue; }
            30 => { rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x68u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 31; continue; }
            31 => { rcx = rdi; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rdx = rdi; rcx = 0x2; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 32; continue; }
            32 => { crate::simd_ops::wu64(rbx.wrapping_add(0x68u64), rbp); bb = 33; continue; }
            33 => { rcx = rbx.wrapping_add(0xa8); rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x60u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 38; continue; } bb = 34; continue; }
            34 => { fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 36; continue; } bb = 35; continue; }
            35 => { rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rcx = 0x2; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 36; continue; }
            36 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x60u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 38; continue; } bb = 37; continue; }
            37 => { rcx = 0x2; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 38; continue; }
            38 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x70u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 61; continue; } bb = 39; continue; }
            39 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0xd8u64))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 41; continue; } bb = 40; continue; }
            40 => { rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 41; continue; }
            41 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x70u64))) as u64; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 60; continue; } bb = 42; continue; }
            42 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x78u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 51; continue; } bb = 43; continue; }
            43 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x70u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 51; continue; } bb = 44; continue; }
            44 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0xd0u64))) as u16 as u64; fl_r = ((rbp as u16)) as u16 as u64; if ((((fl_l as u16)) as i16) > (((fl_r as u16)) as i16)) { bb = 51; continue; } bb = 45; continue; }
            45 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0xd4u64))) as u16 as u64; fl_r = ((rbp as u16)) as u16 as u64; if ((((fl_l as u16)) as i16) > (((fl_r as u16)) as i16)) { bb = 51; continue; } bb = 46; continue; }
            46 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0xb8u64))) as u16 as u64; fl_r = ((rbp as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 51; continue; } bb = 47; continue; }
            47 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0xbau64))) as u16 as u64; fl_r = ((rbp as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 51; continue; } bb = 48; continue; }
            48 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 51; continue; } bb = 49; continue; }
            49 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 51; continue; } bb = 50; continue; }
            50 => { rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); bb = 51; continue; }
            51 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x80u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 60; continue; } bb = 52; continue; }
            52 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x70u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 60; continue; } bb = 53; continue; }
            53 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0xd0u64))) as u16 as u64; fl_r = ((rbp as u16)) as u16 as u64; if ((((fl_l as u16)) as i16) > (((fl_r as u16)) as i16)) { bb = 60; continue; } bb = 54; continue; }
            54 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0xd4u64))) as u16 as u64; fl_r = ((rbp as u16)) as u16 as u64; if ((((fl_l as u16)) as i16) > (((fl_r as u16)) as i16)) { bb = 60; continue; } bb = 55; continue; }
            55 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0xb8u64))) as u16 as u64; fl_r = ((rbp as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 60; continue; } bb = 56; continue; }
            56 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0xbau64))) as u16 as u64; fl_r = ((rbp as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 60; continue; } bb = 57; continue; }
            57 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 60; continue; } bb = 58; continue; }
            58 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 60; continue; } bb = 59; continue; }
            59 => { rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); bb = 60; continue; }
            60 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x70u64))) as u64; rcx = P7; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x70u64), rbp); bb = 61; continue; }
            61 => { rcx = rbx.wrapping_add(0x48); rax = crate::simd_ops::call_stk(P14, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P15, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0xb8); rax = crate::simd_ops::call_stk(P16, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0xa8); rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x90u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 63; continue; } bb = 62; continue; }
            62 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 63; continue; }
            63 => { rcx = rbx.wrapping_add(0x48); crate::simd_ops::wu64(rbx.wrapping_add(0x90u64), rbp); rax = crate::simd_ops::call_stk(P14, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P17, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x10); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P18, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159c440(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142689748; const P1: u64 = 0x1415aef20; const P2: u64 = 0x1415ae370; const P3: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = ((rdx as u32)) as u32 as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x10000u64) as u64);
    rax = P0;
    r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64));
    rdx = rsp.wrapping_add(0x30);
    crate::simd_ops::wu64(rcx, rax);
    rdi = rcx;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0xa0; rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159c4e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f88; const P1: u64 = 0x1426452b0; const P2: u64 = 0x1426452a8; const P3: u64 = 0x142645560;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = 0; fl_l = 0; fl_r = 0; r13 = rdx; r12 = rcx; r15 = ((rax as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 57; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; rcx = ((r15 as u32)) as u32 as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(r12.wrapping_add(0x8u64))) as u32 as u64; r14 = ((rax as u32)) as u32 as u64; r14 = ((r14).wrapping_shl(4u32)) as u64; r14 = ((r14).wrapping_add(crate::simd_ops::ru64(r12))) as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(r12.wrapping_add(0x8u64), ((rax as u32)) as u32); rbp = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 51; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x5u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbx = (crate::simd_ops::ru64(P0)) as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 29; continue; } bb = 4; continue; }
            4 => { bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x3c8u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 16; continue; } bb = 6; continue; }
            6 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x3c8u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 7; continue; }
            7 => { rsi = rbx.wrapping_add(0x3d0); bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x410u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 18; continue; } bb = 9; continue; }
            9 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbp as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x408u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 10; continue; }
            10 => { bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 12; continue; }
            12 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 13; continue; }
            13 => { rax = 0; fl_l = 0; fl_r = 0; rdi = ((rax as u32)) as u32 as u64; bb = 14; continue; }
            14 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 19; continue; }
            16 => { rsi = rbx.wrapping_add(0x3d0); rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x3c8u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 8; continue; } bb = 17; continue; }
            17 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            18 => { rax = 0; fl_l = 0; fl_r = 0; rdi = ((rax as u32)) as u32 as u64; bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x3c8u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 26; continue; } bb = 20; continue; }
            20 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x3c8u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 23; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x3c8u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 20; continue; } bb = 22; continue; }
            22 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(r14, rdi); bb = 52; continue; }
            23 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 27; continue; } bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x3f8u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 27; continue; } bb = 25; continue; }
            25 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x400u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(r14, rdi); bb = 52; continue; }
            26 => { rsi = rbx.wrapping_add(0x3d0); rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 27; continue; }
            27 => { rcx = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(r14, rdi); bb = 52; continue; }
            28 => { bb = 29; continue; }
            29 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x370u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 40; continue; } bb = 30; continue; }
            30 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x370u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 29; continue; } bb = 31; continue; }
            31 => { rsi = rbx.wrapping_add(0x378); bb = 32; continue; }
            32 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x3b8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 42; continue; } bb = 33; continue; }
            33 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbp as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x3b0u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 37; continue; } bb = 34; continue; }
            34 => { bb = 35; continue; }
            35 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 38; continue; } bb = 36; continue; }
            36 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 35; continue; } bb = 37; continue; }
            37 => { rax = 0; fl_l = 0; fl_r = 0; rdi = ((rax as u32)) as u32 as u64; bb = 38; continue; }
            38 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 43; continue; } bb = 39; continue; }
            39 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 43; continue; }
            40 => { rsi = rbx.wrapping_add(0x378); rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x370u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 32; continue; } bb = 41; continue; }
            41 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 29; continue; }
            42 => { rax = 0; fl_l = 0; fl_r = 0; rdi = ((rax as u32)) as u32 as u64; bb = 43; continue; }
            43 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x370u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 50; continue; } bb = 44; continue; }
            44 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x370u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 47; continue; } bb = 45; continue; }
            45 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x370u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 44; continue; } bb = 46; continue; }
            46 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(r14, rdi); bb = 52; continue; }
            47 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 27; continue; } bb = 48; continue; }
            48 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x3a0u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 27; continue; } bb = 49; continue; }
            49 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x3a8u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(r14, rdi); bb = 52; continue; }
            50 => { rsi = rbx.wrapping_add(0x378); rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(r14, rdi); bb = 52; continue; }
            51 => { rcx = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(r14, (0x0u64) as u64); bb = 52; continue; }
            52 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 55; continue; } bb = 53; continue; }
            53 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 55; continue; } bb = 54; continue; }
            54 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 55; continue; }
            55 => { rcx = 0; fl_l = 0; fl_r = 0; r15 = (((r15 as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu16(r14.wrapping_add(0x8u64), ((rcx as u16)) as u16); fl_l = ((r15 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r13.wrapping_add(0x10u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 56; continue; }
            56 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 57; continue; }
            57 => { rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159c7c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x310; const P1: u64 = 0x318; const P2: u64 = 0x8; const P3: u64 = 0x14159c840; const P4: u64 = 0x141597340; const P5: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64) as u128; rdi = rdx; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; rbx = rcx; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; rdx = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsi, rdi); rax = 0x1; fl_l = (crate::simd_ops::lk_add_u16(rbx.wrapping_add(P5), (1) as u16)) as u16 as u64; bb = 5; continue; }
            4 => { rax = 0x2; bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159c840(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14159d090; const P1: u64 = 0x14153e940; const P2: u64 = 0x14183acc8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; xmm0 = ((crate::simd_ops::ru64(rdx)) as u64) as u128; r8 = rsp.wrapping_add(0x48); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; rbx = rcx; rdx = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rdi = rax; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { rdi = ((rdi).wrapping_sub(crate::simd_ops::ru64(rbx))) as u64; rdi = ((((rdi) as i64).wrapping_shr(3u32) as u64)) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 3; continue; }
            3 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 11; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rax = ((rdi as u32)) as u32 as u64; rsi = ((rax.wrapping_mul(8)) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rdx = ((rdx).wrapping_add(rsi)) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (((rax as u32)).wrapping_sub((rdi as u32))) as u32 as u64; rcx = rdx.wrapping_add(0x8); rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x8u64), (1) as u32); rax = (crate::simd_ops::ru64(rbx)) as u64; rax = ((rax).wrapping_add(rsi)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rbx)) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159c930(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524330; const P1: u64 = 0x1415c2bf0; const P2: u64 = 0x1415aef80;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x58u64))) as u64; r15 = 0; fl_l = 0; fl_r = 0; rbp = (((r9 as u8)) as u32) as u32 as u64; rdi = (((r8 as u8)) as u32) as u32 as u64; r14 = ((rdx as u32)) as u32 as u64; rbx = rcx; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rdx = ((rax.wrapping_add(0x28u64)) as u32) as u32 as u64; rcx = ((rax.wrapping_add(0x2u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rax.wrapping_add(0x21u64), ((r15 as u32)) as u32); crate::simd_ops::wu16(rax.wrapping_add(0x25u64), ((r15 as u16)) as u16); crate::simd_ops::wu8(rax.wrapping_add(0x27u64), ((r15 as u8)) as u8); crate::simd_ops::wu64(rax, r15); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), r15); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), r15); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), r15); crate::simd_ops::wu8(rax.wrapping_add(0x20u64), ((r15 as u8)) as u8); bb = 4; continue; }
            3 => { rax = r15; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rbx.wrapping_add(0x58u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = 0x34; bb = 19; continue; }
            6 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xc0u64))) as u32 as u64; rcx = rbx.wrapping_add(0x48); r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32 as u64; r9 = ((r14 as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r15); rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xb0u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xa8u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rsi); rsi = (crate::simd_ops::ru32(rsp.wrapping_add(0xa0u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rsi as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((rbp as u8)) as u8); crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((rdi as u8)) as u8); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 18; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { rdx = ((rax.wrapping_add(0x28u64)) as u32) as u32 as u64; rcx = ((rax.wrapping_add(0x2u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu32(rax.wrapping_add(0x21u64), ((r15 as u32)) as u32); crate::simd_ops::wu16(rax.wrapping_add(0x25u64), ((r15 as u16)) as u16); crate::simd_ops::wu8(rax.wrapping_add(0x27u64), ((r15 as u8)) as u8); crate::simd_ops::wu64(rax, r15); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), r15); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), r15); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), r15); crate::simd_ops::wu8(rax.wrapping_add(0x20u64), ((r15 as u8)) as u8); bb = 11; continue; }
            10 => { rax = r15; bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rbx.wrapping_add(0x58u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rax)) as u64; rdx = rsi; fl_l = (((rcx) >> (((rdx) as u32 & 63))) & 1) as u64; r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x70u64))) as u64; if ((fl_l as u64) != 0) { bb = 15; continue; } bb = 13; continue; }
            13 => { fl_l = (((rcx) >> (((rdx) as u32 & 63))) & 1) as u64; rcx = ((rcx) | ((1u64) << (((rdx) as u32 & 63)))) as u64; crate::simd_ops::wu64(rax, rcx); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            15 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x20u64), (0x1u8) as u8); bb = 17; continue; }
            17 => { rax = 0x1; bb = 18; continue; }
            18 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; bb = 19; continue; }
            19 => { r11 = rsp.wrapping_add(0x60); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159cab0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70; const P1: u64 = 0xd8; const P2: u64 = 0x1415974b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rbx = rcx;
    fl_l = rax; fl_r = rax;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P1))) as u8 as u64; fl_r = (0x2u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) != 0)) { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_l = (crate::simd_ops::lk_or_u8(rax.wrapping_add(P1), (0x2u8) as u8)) as u8 as u64; fl_r = fl_l; }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159caf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70; const P1: u64 = 0x80; const P2: u64 = 0xd8; const P3: u64 = 0x1439315c0; const P4: u64 = 0xd2; const P5: u64 = 0xd6; const P6: u64 = 0x14159d880; const P7: u64 = 0x1415a0270; const P8: u64 = 0x78; const P9: u64 = 0x1415a03a0;
    let (mut rax, mut fl_l, mut fl_r, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64; rsi = rdx; rdi = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdx)) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(P3, (0x1u8) as u8); rax = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = ((crate::simd_ops::ru16(rax.wrapping_add(P4))) as u32) as u32 as u64; rdx = ((crate::simd_ops::ru16(rax.wrapping_add(P5))) as u32) as u32 as u64; bb = 5; continue; }
            4 => { rcx = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { crate::simd_ops::wu16(rsp.wrapping_add(0x32u64), ((rcx as u16)) as u16); rcx = rdi; crate::simd_ops::wu16(rsp.wrapping_add(0x30u64), ((rdx as u16)) as u16); rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rsi)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsi, (0x0u64) as u64); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            8 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P8))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159cbb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x60; const P1: u64 = 0x18; const P2: u64 = 0x2; const P3: u64 = 0x141524330; const P4: u64 = 0x10; const P5: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rsi = ((r8 as u32)) as u32 as u64; rdi = ((rdx as u32)) as u32 as u64; rbx = rcx; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rdx = ((rax.wrapping_add(P1)) as u32) as u32 as u64; rcx = ((rax.wrapping_add(P2)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = 0; fl_l = 0; fl_r = 0; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rax, ((rcx as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(P4), ((rcx as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(P0), rax); bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 8; continue; } bb = 4; continue; }
            4 => { rdx = ((rdi.wrapping_mul(8)) as u32) as u32 as u64; crate::simd_ops::wu32(rax, ((rsi as u32)) as u32); rcx = 0x2; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P5), rax); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = 0x34; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { crate::simd_ops::wu64(rbx.wrapping_add(P0), rcx); bb = 5; continue; }
            7 => { crate::simd_ops::wu32(rax.wrapping_add(P4), ((rdi as u32)) as u32); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0x1; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159cc50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434268d0; const P1: u64 = 0x142689180; const P2: u64 = 0x143930f44; const P3: u64 = 0x1415317a0; const P4: u64 = 0x141594710;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 29>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0xb0u64)) as u64; r10 = ((crate::simd_ops::ru8(rcx.wrapping_add(0xd4u64))) as u32) as u32 as u64; rsi = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; r15 = r9; r11 = (crate::simd_ops::ru32(rcx.wrapping_add(0x20u64))) as u32 as u64; r8 = rdx; r10 = (r10 & !0xffu64) | (((((r10 as u8)).wrapping_shr(7u32)) as u8 as u64)); r13 = rcx; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm9) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm10) as u128); crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((r10 as u8)) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r11 as u32)) as u32); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { r14 = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; r12 = rax.wrapping_add(0x1); r14 = ((r14).wrapping_add(0x8u64)) as u64; rcx = 0xfffffff8; r14 = (((r14) & (rcx))) as u64; fl_l = ((r14).wrapping_add(rax)) as u64; fl_r = fl_l; r14 = ((fl_l as u64)) as u64; bb = 3; continue; }
            2 => { r12 = rsi; r14 = rsi; bb = 3; continue; }
            3 => { r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x58u64))) as u32 as u64; rbp = 0x1; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x110u64))) as u64; xmm9 = 0u128; xmm10 = ((crate::simd_ops::ru32(P0)) as u32) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x100u64), rdi); crate::simd_ops::wu128(rsp.wrapping_add(0xa0u64), (xmm6) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x90u64), (xmm7) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x80u64), (xmm8) as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((r9 as u32)) as u32); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x18u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; rdx = ((rdx).wrapping_add(1)) as u64; fl_l = ((rdx).wrapping_add(rcx)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 7; continue; }
            6 => { rdx = rsi; bb = 7; continue; }
            7 => { fl_l = r12; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 42; continue; } bb = 8; continue; }
            8 => { rdx = ((crate::simd_ops::ru8(r12)) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xb; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 29; continue; } bb = 9; continue; }
            9 => { rax = ((rdx as u32)) as u32 as u64; rsi = ((rdx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffe0u32))) as u32 as u64; rsi = ((rsi).wrapping_shr(5u32)) as u64; rcx = ((rdx as u32)) as u32 as u64; rdi = rbp; rcx = (((rcx as u32)).wrapping_sub((rax as u32))) as u32 as u64; rdi = ((rdi).wrapping_shl(((rcx as u32) & 63))) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0x90u64))) as u32 as u64; fl_l = rdi; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 10; continue; }
            10 => { xmm7 = ((crate::simd_ops::ru32(r14)) as u32) as u128; rcx = P1; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(rdx.wrapping_mul(4)))) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = 0x1a; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 22; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru32(P2)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 28; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0x9cu64))) as u32 as u64; fl_l = rdi; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 13; continue; }
            13 => { fl_l = ((xmm7 as u32)) as u32 as u64; fl_r = ((xmm9 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 22; continue; } bb = 14; continue; }
            14 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rsp.wrapping_add(0xf0u64), (0x0u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rcx as u32)) as u32); r8 = rsp.wrapping_add(0xf0); crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), ((rdx as u8)) as u8); xmm8 = (xmm9) as u128; crate::simd_ops::wu8(rsp.wrapping_add(0x32u64), ((r10 as u8)) as u8); rdx = rsp.wrapping_add(0x40); crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), ((rcx as u32)) as u32); xmm6 = (xmm9) as u128; crate::simd_ops::wu8(rsp.wrapping_add(0x31u64), (0x1u8) as u8); rcx = rbx.wrapping_add(0x68); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((r11 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0xf0u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 17; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu32(rax.wrapping_add(0xcu64), (0x0u32) as u32); bb = 17; continue; }
            17 => { xmm8 = ((crate::simd_ops::ru32(rax.wrapping_add(0xcu64))) as u32) as u128; xmm6 = (xmm8) as u128; xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(xmm7 as u32)).to_bits() as u128); crate::simd_ops::wu32(rax.wrapping_add(0xcu64), ((xmm6 as u32)) as u32); bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0xa8u64))) as u32 as u64; fl_l = rdi; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 21; continue; } bb = 19; continue; }
            19 => { xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(rbp.wrapping_mul(4)))) as u32) as u128; fl_l = ((xmm8 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 25; continue; } bb = 20; continue; }
            20 => { fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 26; continue; } bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::lk_or_u8(rbx.wrapping_add(0xb7u64), (0x4u8) as u8)) as u8 as u64; fl_r = fl_l; r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x24u64))) as u32 as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0xf8u64))) as u64; bb = 22; continue; }
            22 => { rbp = 0x1; bb = 23; continue; }
            23 => { rsi = 0; fl_l = 0; fl_r = 0; bb = 24; continue; }
            24 => { r10 = ((crate::simd_ops::ru8(rsp.wrapping_add(0x20u64))) as u32) as u32 as u64; r12 = ((r12).wrapping_add(1)) as u64; r11 = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; fl_l = ((r14).wrapping_add(0x10u64)) as u64; fl_r = fl_l; r14 = ((fl_l as u64)) as u64; bb = 4; continue; }
            25 => { fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; bb = 26; continue; }
            26 => { r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x24u64))) as u32 as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0xf8u64))) as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 22; continue; } bb = 27; continue; }
            27 => { crate::simd_ops::wu32(rbx.wrapping_add(rbp.wrapping_mul(4)), ((xmm6 as u32)) as u32); bb = 22; continue; }
            28 => { xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) + f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_add(rbp.wrapping_mul(4))))).to_bits() as u128); crate::simd_ops::wu32(rbx.wrapping_add(rbp.wrapping_mul(4)), ((xmm7 as u32)) as u32); bb = 22; continue; }
            29 => { xmm0 = ((crate::simd_ops::ru32(r14)) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm10 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 24; continue; } bb = 30; continue; }
            30 => { fl_l = (crate::simd_ops::ru64(r13.wrapping_add(0xc0u64))) as u64; fl_r = r8; rax = ((r9 as u32)) as u32 as u64; rdi = (crate::simd_ops::ru32(r15.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rsi as u32)) as u32 as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r13); r9 = ((rcx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xfffffffcu32))) as u32 as u64; r9 = ((((r9 as u32)) | ((rax as u32)))) as u32 as u64; rax = (crate::simd_ops::ru64(r15)) as u64; r8 = ((r9 as u32)) as u32 as u64; rcx = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; r9 = ((((r9 as u32)) & (0xffffff3fu32))) as u32 as u64; r9 = ((((r9 as u32)) | (0x3cu32))) as u32 as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((r9 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), ((r9 as u32)) as u32); fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 34; continue; } bb = 31; continue; }
            31 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = r13; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 33; continue; } bb = 32; continue; }
            32 => { rdx = ((r8 as u32)) as u32 as u64; rdx = ((((rdx as u32)) ^ (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))))) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 34; continue; } bb = 33; continue; }
            33 => { rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 31; continue; } bb = 34; continue; }
            34 => { fl_l = rax; fl_r = rcx; rdx = rax.wrapping_add(0x10); rdx = (if (((fl_l as u64)) == ((fl_r as u64))) { rsi } else { rdx }) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 37; continue; } bb = 35; continue; }
            35 => { crate::simd_ops::wu32(rdx, ((xmm0 as u32)) as u32); bb = 36; continue; }
            36 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0xf8u64))) as u64; r12 = ((r12).wrapping_add(1)) as u64; r10 = ((crate::simd_ops::ru8(rsp.wrapping_add(0x20u64))) as u32) as u32 as u64; fl_l = ((r14).wrapping_add(0x10u64)) as u64; fl_r = fl_l; r14 = ((fl_l as u64)) as u64; r11 = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; bb = 4; continue; }
            37 => { rax = (crate::simd_ops::ru32(r15.wrapping_add(0xcu64))) as u32 as u64; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 39; continue; } bb = 38; continue; }
            38 => { rdx = ((rax as u32)) as u32 as u64; rcx = r15; rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdx = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rbp as u32) } else { (rdx as u32) }) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x24u64))) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 36; continue; } bb = 39; continue; }
            39 => { rax = (crate::simd_ops::ru32(r15.wrapping_add(0xcu64))) as u32 as u64; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 36; continue; } bb = 40; continue; }
            40 => { rcx = (crate::simd_ops::ru32(r15.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0xf8u64))) as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(r15.wrapping_add(0x8u64), ((rax as u32)) as u32); rcx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(r15)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 41; continue; }
            41 => { xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; r10 = ((crate::simd_ops::ru8(rsp.wrapping_add(0x20u64))) as u32) as u32 as u64; r12 = ((r12).wrapping_add(1)) as u64; r11 = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; crate::simd_ops::wu128(rdx, (xmm0) as u128); rax = (crate::simd_ops::ru32(r14)) as u32 as u64; fl_l = ((r14).wrapping_add(0x10u64)) as u64; fl_r = fl_l; r14 = ((fl_l as u64)) as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x10u64), ((rax as u32)) as u32); bb = 4; continue; }
            42 => { xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x80u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x90u64))) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0xa0u64))) as u128; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x100u64))) as u64; xmm9 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; xmm10 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; rsp = ((rsp).wrapping_add(0xb0u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159cfe0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc0; const P1: u64 = 0x14159cc50; const P2: u64 = 0xc8; const P3: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbp = rdx; rdi = r9; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rsi = r8; rbx = rcx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { r9 = r8; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); r8 = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64; r8 = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 4; continue; }
            4 => { bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = ((rax).wrapping_add(0x10u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 7; continue; }
            7 => { r9 = 0; fl_l = 0; fl_r = 0; rdx = ((rax.wrapping_add(P3)) as u64) as u64; fl_l = rax; fl_r = rcx; rdx = (if (((fl_l as u64)) == ((fl_r as u64))) { r9 } else { rdx }) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; r9 = rsi; rcx = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159d090(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut fl_k: u32 = 0; let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x10u64)) as u64; r10 = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; r11 = 0; fl_l = 0; fl_r = 0; rsi = r8; r9 = rdx; rbx = rcx; fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { fl_k = 2; bb = 15; continue; } bb = 1; continue; }
            1 => { r14 = (crate::simd_ops::ru64(rcx)) as u64; xmm1 = ((crate::simd_ops::ru32(rdx)) as u32) as u128; fl_k = 2; bb = 2; continue; }
            2 => { rdi = ((r10 as u32)) as u32 as u64; rdi = (((rdi as u32)).wrapping_shr(1u32)) as u32 as u64; rdi = (((rdi as u32)).wrapping_add((r11 as u32))) as u32 as u64; r8 = ((((rdi as u32)) as i32 as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(r8.wrapping_mul(8)))) as u64; xmm0 = ((crate::simd_ops::ru64(rax.wrapping_add(0x310u64))) as u64) as u128; rdx = (crate::simd_ops::ru32(rax.wrapping_add(0x318u64))) as u32 as u64; crate::simd_ops::wu64(rsp, ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru64(rsp)) as u64; xmm0 = (((rcx as u32)) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 12; continue; } bb = 3; continue; }
            3 => { rax = rcx; rax = ((rax).wrapping_shr(32u32)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r9.wrapping_add(0x4u64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r9.wrapping_add(0x8u64))) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 17; continue; } bb = 5; continue; }
            5 => { fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 12; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x12u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rax = (crate::simd_ops::ru32(r9.wrapping_add(0x8u64))) as u32 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 7; continue; }
            7 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = ((rcx).wrapping_shr(32u32)) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r9.wrapping_add(0x4u64))) as u32 as u64; bb = 9; continue; }
            9 => { rcx = (rcx & !0xffu64) | (((if (((fl_l as u32)) < ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_k = 0; bb = 14; continue; }
            10 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { fl_k = 0; bb = 13; continue; } bb = 11; continue; }
            11 => { rcx = ((rcx).wrapping_shr(32u32)) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r9.wrapping_add(0x4u64))) as u32 as u64; fl_k = 0; bb = 13; continue; }
            12 => { fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; fl_k = 1; bb = 13; continue; }
            13 => { rcx = (rcx & !0xffu64) | (((if (match fl_k { 0 => (((fl_l as u32)) > ((fl_r as u32))), 1 => (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))), _ => false }) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 14; continue; }
            14 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; rax = ((rdi.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (r11 as u32) } else { (rax as u32) }) as u32 as u64; r11 = ((rax as u32)) as u32 as u64; rax = ((r10.wrapping_sub(0x1u64)) as u32) as u32 as u64; rax = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (r10 as u32) } else { (rax as u32) }) as u32 as u64; r10 = ((rax as u32)) as u32 as u64; fl_l = (((r10 as u32)).wrapping_shr(1u32)) as u32 as u64; r10 = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { fl_k = 3; bb = 2; continue; } fl_k = 3; bb = 15; continue; }
            15 => { crate::simd_ops::wu8(rsi, (0x0u8) as u8); rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((((r11 as u32)) as i32 as i64) as u64) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x10u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            17 => { crate::simd_ops::wu8(rsi, (0x1u8) as u8); rax = (crate::simd_ops::ru64(rbx)) as u64; rax = ((rax.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_k = 0; bb = 16; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159d1a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14160f0e0; const P1: u64 = 0x14160eaa0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = ((r8 as u32)) as u32 as u64; r15 = ((rdx as u32)) as u32 as u64; r8 = rcx; rbx = rcx; rcx = ((rcx).wrapping_add(0xb8u64)) as u64; rdx = r9; r12 = r9; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (((rsi as u8)) as u32) as u32 as u64; rcx = (((r15 as u8)) as u32) as u32 as u64; rdx = (rdx & !0xffu64) | (((!((rdx as u8))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & ((rsi as u8)))) as u8 as u64)); rdx = (rdx & !0xffu64) | ((((((rdx as u8)) & (crate::simd_ops::ru8(rax.wrapping_add(0x20u64))))) as u8 as u64)); fl_l = ((((rdx as u8)) | ((rcx as u8)))) as u8 as u64; fl_r = fl_l; rdx = (rdx & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rax.wrapping_add(0x20u64), ((rdx as u8)) as u8); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x70u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x70u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { fl_l = r12; fl_r = r12; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { r8 = ((rsi as u32)) as u32 as u64; rcx = rbx.wrapping_add(0xb8); rdx = ((r15 as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x70u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x70u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xa; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x78u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x98u64))) as u32 as u64; fl_l = rax; fl_r = (0x1f000u64) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r14); rdi = ((rbp.wrapping_add(0x1u64)) as u32) as u32 as u64; r14 = ((rbp.wrapping_add(0x1du64)) as u32) as u32 as u64; bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = (((rax) >> (((r14) as u32 & 63))) & 1) as u64; if ((fl_l as u64) != 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; rcx = rbx.wrapping_add(0x28); rdx = ((rbp.wrapping_add(0x1du64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = ((rdi as u32)) as u32 as u64; rax = (!((rax as u32))) as u32 as u64; fl_l = ((((rsi as u32)) & ((rax as u32)))) as u32 as u64; fl_r = fl_l; rsi = ((fl_l as u32)) as u32 as u64; bb = 15; continue; }
            15 => { rbp = (((rbp as u32)).wrapping_add(1)) as u32 as u64; rdi = (((rdi as u32)).rotate_left(1u32)) as u32 as u64; r14 = ((r14).wrapping_add(1)) as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 11; continue; } bb = 16; continue; }
            16 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { rax = (0x200000000u64) as u64; fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; rcx = rbx.wrapping_add(0x28); rdx = 0x21; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { fl_l = ((((rsi as u32)) & (0xffffffefu32))) as u32 as u64; fl_r = fl_l; rsi = ((fl_l as u32)) as u32 as u64; bb = 20; continue; }
            20 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru64(rbx)) as u64; r9 = r12; r8 = ((rsi as u32)) as u32 as u64; rdx = ((r15 as u32)) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0xf0u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 22; continue; }
            22 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159d300(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70; const P1: u64 = 0xd0; const P2: u64 = 0xd4; const P3: u64 = 0xb8; const P4: u64 = 0xba; const P5: u64 = 0x18; const P6: u64 = 0x8; const P7: u64 = 0xd8; const P8: u64 = 0x141599340; const P9: u64 = 0x1415a2050; const P10: u64 = 0x143931698; const P11: u64 = 0x14159da30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rcx; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(P1))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if ((((fl_l as u16)) as i16) > (((fl_r as u16)) as i16)) { bb = 10; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(P2))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if ((((fl_l as u16)) as i16) > (((fl_r as u16)) as i16)) { bb = 10; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(P3))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 10; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(P4))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 10; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P6))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P7))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rcx = rbx; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rcx = P10; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x70u64), (0x0u64) as u64); bb = 10; continue; }
            10 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159d380(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14159dcb0; const P1: u64 = 0x1415a3c20; const P2: u64 = 0x141599340; const P3: u64 = 0x1415a2050; const P4: u64 = 0x143931698; const P5: u64 = 0x14159da30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x70u64))) as u64; rdi = rdx; rbx = rcx; rax = (crate::simd_ops::ru64(r8)) as u64; r8 = (crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32 as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; r8 = ((r8).wrapping_add(rax)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = ((rax).wrapping_add(0x10u64)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rsi = 0; fl_l = 0; fl_r = 0; rcx = rax.wrapping_add(0x8); fl_l = rax; fl_r = r8; rcx = (if (((fl_l as u64)) == ((fl_r as u64))) { rsi } else { rcx }) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0x98u64))) as u16 as u64; fl_r = ((rsi as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 9; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0x9au64))) as u16 as u64; fl_r = ((rsi as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x70u64))) as u64; rdx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x70u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0xd0u64))) as u16 as u64; fl_r = ((rsi as u16)) as u16 as u64; if ((((fl_l as u16)) as i16) > (((fl_r as u16)) as i16)) { bb = 19; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0xd4u64))) as u16 as u64; fl_r = ((rsi as u16)) as u16 as u64; if ((((fl_l as u16)) as i16) > (((fl_r as u16)) as i16)) { bb = 19; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0xb8u64))) as u16 as u64; fl_r = ((rsi as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 19; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0xbau64))) as u16 as u64; fl_r = ((rsi as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 19; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 19; continue; } bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 19; continue; } bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0xd8u64))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 18; continue; }
            18 => { rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x70u64))) as u64; rcx = P4; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x70u64), rsi); bb = 19; continue; }
            19 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159d480(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70; const P1: u64 = 0xd8; const P2: u64 = 0x1439315c0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(P2, (0x1u8) as u8); }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159d4a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524330; const P1: u64 = 0x14159bba0; const P2: u64 = 0x1415a04e0; const P3: u64 = 0x14153ea00; const P4: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = ((crate::simd_ops::ru8(rcx.wrapping_add(0xd4u64))) as u32) as u32 as u64; rbp = rdx; rsi = (((rdi as u8)) as u32) as u32 as u64; r15 = rcx; rdi = (rdi & !0xffu64) | (((((rdi as u8)).wrapping_shr(1u32)) as u8 as u64)); rdx = 0xa0; rcx = 0; fl_l = 0; fl_r = 0; rdi = (rdi & !0xffu64) | ((((((rdi as u8)) & (0x1u8))) as u8 as u64)); r12 = (((r9 as u16)) as u32) as u32 as u64; r14 = r8; rsi = (rsi & !0xffu64) | ((((((rsi as u8)) & (0x1u8))) as u8 as u64)); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 1; continue; }
            1 => { rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((rdi as u8)) as u8); r9 = (((r12 as u16)) as u32) as u32 as u64; r8 = rbp; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((rsi as u8)) as u8); rdx = r15; rcx = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rsi = (crate::simd_ops::ru64(r15.wrapping_add(0x70u64))) as u64; rcx = (crate::simd_ops::ru64(rsi)) as u64; r15 = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rdx = ((r15 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = ((rcx).wrapping_add(0x10u64)) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 5; continue; }
            5 => { r12 = 0; fl_l = 0; fl_r = 0; rdi = rcx.wrapping_add(0x8); fl_l = rcx; fl_r = rdx; rdi = (if (((fl_l as u64)) == ((fl_r as u64))) { r12 } else { rdi }) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rdi, rbx); rax = ((r12.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu64(r14, rdi); bb = 22; continue; }
            7 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0xcu64))) as u32 as u64; fl_l = r15; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(r14, rdi); bb = 18; continue; }
            10 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0xcu64))) as u32 as u64; fl_l = r15; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 17; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rax as u32)) as u32); fl_l = ((rcx).wrapping_add(crate::simd_ops::ru64(rsi))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), r12); bb = 13; continue; }
            13 => { rax = r12; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = rcx.wrapping_add(0x8); crate::simd_ops::wu64(rcx, rbp); crate::simd_ops::wu64(rax, rbx); bb = 15; continue; }
            15 => { crate::simd_ops::wu64(r14, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 16; continue; }
            16 => { rax = 0x1; bb = 22; continue; }
            17 => { crate::simd_ops::wu64(r14, r12); bb = 18; continue; }
            18 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x70u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(0x78u64), ((r12 as u32)) as u32); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x70u64), r12); crate::simd_ops::wu32(rbx.wrapping_add(0x7cu64), ((r12 as u32)) as u32); bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 21; continue; }
            21 => { rax = 0x2; bb = 22; continue; }
            22 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159d630(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70; const P1: u64 = 0xd4; const P2: u64 = 0xd6; const P3: u64 = 0xd0; const P4: u64 = 0xb8; const P5: u64 = 0xba; const P6: u64 = 0x18; const P7: u64 = 0x8; const P8: u64 = 0xd8; const P9: u64 = 0x141599340; const P10: u64 = 0x1415a2050; const P11: u64 = 0x143931698; const P12: u64 = 0x14159da30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = (((rdx as u16)) as u32) as u32 as u64; rbx = rcx; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::lk_sub_u16(rax.wrapping_add(P1), (1) as u16); fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_sub_u16(rax.wrapping_add(P2), (1) as u16)) as u16 as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = (crate::simd_ops::ru16(rax.wrapping_add(P3))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if ((((fl_l as u16)) as i16) > (((fl_r as u16)) as i16)) { bb = 12; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(P1))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if ((((fl_l as u16)) as i16) > (((fl_r as u16)) as i16)) { bb = 12; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(P4))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 12; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(P5))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 12; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P7))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P8))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rcx = rbx; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rcx = P11; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x70u64), (0x0u64) as u64); bb = 12; continue; }
            12 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 15; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x80u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = 0xfffe; rdx = 0x1; rdi = (rdi & !0xffffu64) | ((((((rdi as u16)) & ((rax as u16)))) as u16 as u64)); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x120u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x78u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (((rdi as u16)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x120u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            17 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159d710(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70; const P1: u64 = 0xd4; const P2: u64 = 0xd6; const P3: u64 = 0xd0; const P4: u64 = 0xb8; const P5: u64 = 0xba; const P6: u64 = 0x18; const P7: u64 = 0x8; const P8: u64 = 0xd8; const P9: u64 = 0x141599340; const P10: u64 = 0x1415a2050; const P11: u64 = 0x143931698; const P12: u64 = 0x14159da30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rcx; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::lk_sub_u16(rax.wrapping_add(P1), (1) as u16); fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_sub_u16(rax.wrapping_add(P2), (1) as u16)) as u16 as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = (crate::simd_ops::ru16(rax.wrapping_add(P3))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if ((((fl_l as u16)) as i16) > (((fl_r as u16)) as i16)) { bb = 12; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(P1))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if ((((fl_l as u16)) as i16) > (((fl_r as u16)) as i16)) { bb = 12; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(P4))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 12; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(P5))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 12; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P7))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P8))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rcx = rbx; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rcx = P11; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x70u64), (0x0u64) as u64); bb = 12; continue; }
            12 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159d7b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x20; const P4: u64 = 0xffffffffffffffe8; const P5: u64 = 0x1; const P6: u64 = 0x120; const P7: u64 = 0x110;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { rax = rsp; crate::simd_ops::wu32(rax.wrapping_add(P0), ((r8 as u32)) as u32); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x58u64)) as u64; crate::simd_ops::wu64(rax.wrapping_add(P1), rbp); rbx = rdx; crate::simd_ops::wu64(rax.wrapping_add(P2), rsi); r14 = rcx; crate::simd_ops::wu64(rax.wrapping_add(P3), rdi); rdi = 0; fl_l = 0; fl_r = 0; rbp = ((((r8 as u16)) as i16 as i32) as u32) as u32 as u64; rsi = (((rdi as u16)) as u32) as u32 as u64; crate::simd_ops::wu64(rax.wrapping_add(P4), r15); r15 = ((rdi.wrapping_add(P5)) as u32) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = ((r15 as u32)) as u32 as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (rsi & !0xffffu64) | (((((rsi as u16)).wrapping_add(1)) as u16 as u64)); rax = (((rsi as u16)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 3; continue; } bb = 4; continue; }
            4 => { rsi = (((crate::simd_ops::ru16(rsp.wrapping_add(0x82u64))) as i16 as i32) as u32) as u32 as u64; xmm0 = 0u128; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), ((r15 as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x49u64), ((r15 as u8)) as u8); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = rsp.wrapping_add(0x20); rcx = rbx; crate::simd_ops::wu8(rsp.wrapping_add(0x48u64), (0x0u8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r14); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P7))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (rdi & !0xffffu64) | (((((rdi as u16)).wrapping_add(1)) as u16 as u64)); rax = (((rdi as u16)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 6; continue; } bb = 7; continue; }
            7 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rsp = ((rsp).wrapping_add(0x58u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 8; continue; }
            8 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159d880(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x80; const P1: u64 = 0x78; const P2: u64 = 0x70; const P3: u64 = 0xd6; const P4: u64 = 0xd2; const P5: u64 = 0x14159d7b0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu32(rsp.wrapping_add(0x10u64), ((rdx as u32)) as u32); rsp = ((rsp).wrapping_sub(0x28u64)) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 1; continue; }
            1 => { r9 = ((crate::simd_ops::ru16(rsp.wrapping_add(0x3au64))) as u32) as u32 as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; r8 = ((rcx.wrapping_add(P1)) as u64) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(P3))) as u16 as u64; fl_r = ((rdx as u16)) as u16 as u64; crate::simd_ops::wu16(rax.wrapping_add(P3), ((fl_l as u16).wrapping_sub(fl_r as u16)) as u16); bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(r8)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru16(rcx.wrapping_add(P4))) as u16 as u64; fl_r = ((r9 as u16)) as u16 as u64; crate::simd_ops::wu16(rcx.wrapping_add(P4), ((fl_l as u16).wrapping_sub(fl_r as u16)) as u16); bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(r8)) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 8; continue; }
            8 => { r8 = ((rdx as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return crate::simd_ops::tail_stk(P5, rax, rcx, rdx, r8, r9, rsp); }
            9 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159d900(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141599340; const P1: u64 = 0x1415a2050; const P2: u64 = 0x143931698; const P3: u64 = 0x14159da30;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(0x70u64))) as u64; r10 = r8; rbx = rcx; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::lk_sub_u16(r9.wrapping_add(0xd0u64), (1) as u16); fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_sub_u16(r9.wrapping_add(0xd2u64), (1) as u16)) as u16 as u64; bb = 3; continue; }
            3 => { fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(r9.wrapping_add(0xc0u64))) as u64; r8 = (crate::simd_ops::ru32(r9.wrapping_add(0xc8u64))) as u32 as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; r8 = ((r8).wrapping_add(rax)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rax)) as u64; fl_l = rdx; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = ((rax).wrapping_add(0x10u64)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 7; continue; }
            7 => { bb = 17; continue; }
            8 => { rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub(1)) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(0x8u64), ((rcx as u32)) as u32); fl_l = (crate::simd_ops::ru8(r9.wrapping_add(0xd8u64))) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 17; continue; } bb = 9; continue; }
            9 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 17; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(r9.wrapping_add(0xc0u64))) as u64; rcx = (crate::simd_ops::ru32(r9.wrapping_add(0xc8u64))) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 12; continue; }
            12 => { rax = ((rax).wrapping_add(0x10u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 13; continue; }
            13 => { bb = 17; continue; }
            14 => { rcx = ((rcx).wrapping_add(0xfffffffffffffff0u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { xmm0 = (crate::simd_ops::ru128(rcx)) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::lk_sub_u32(r9.wrapping_add(0xc8u64), (1) as u32)) as u32 as u64; bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x70u64))) as u64; fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0xd0u64))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if ((((fl_l as u16)) as i16) > (((fl_r as u16)) as i16)) { bb = 26; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0xd4u64))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if ((((fl_l as u16)) as i16) > (((fl_r as u16)) as i16)) { bb = 26; continue; } bb = 19; continue; }
            19 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0xb8u64))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 26; continue; } bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0xbau64))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 26; continue; } bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 26; continue; } bb = 22; continue; }
            22 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 26; continue; } bb = 23; continue; }
            23 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0xd8u64))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 25; continue; } bb = 24; continue; }
            24 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 25; continue; }
            25 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x70u64))) as u64; rcx = P2; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0x70u64), (0x0u64) as u64); bb = 26; continue; }
            26 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159da30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415aef20; const P1: u64 = 0x1415242e0; const P2: u64 = 0x142689748; const P3: u64 = 0x1415ae370;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; rbp = rcx; rcx = rdx.wrapping_add(0x20); rdi = rdx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; rdx = rsp.wrapping_add(0x30); rcx = rdi.wrapping_add(0x20); rbx = (((rax as u8)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r8.wrapping_add(0x30u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r8 = (((rbx as u8)) as u32) as u32 as u64; rcx = rdi.wrapping_add(0x20); rdx = rax; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x90u64))) as u64; rbx = 0; fl_l = 0; fl_r = 0; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(0x98u64), ((rbx as u32)) as u32); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(0x90u64), rbx); crate::simd_ops::wu32(rdi.wrapping_add(0x9cu64), ((rbx as u32)) as u32); bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), rbx); rdx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbx as u32)) as u32); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rbx); crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((rbx as u32)) as u32); bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0xc0u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(0xc8u64), ((rbx as u32)) as u32); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(0xc0u64), rbx); crate::simd_ops::wu32(rdi.wrapping_add(0xccu64), ((rbx as u32)) as u32); bb = 6; continue; }
            6 => { rax = P2; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x10000u64) as u64); r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu64(rdi.wrapping_add(0x20u64), rax); rdx = rsp.wrapping_add(0x30); rcx = rdi.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; rcx = rbx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { rax = rdx.wrapping_add(0x7000); fl_l = rdi; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = rdi; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = rdx; rdx = (crate::simd_ops::ru64(rax)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x7010u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rdx.wrapping_add(0x7010u64), rdi); crate::simd_ops::wu64(rdi, rbx); bb = 14; continue; }
            13 => { crate::simd_ops::wu64(rdi, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x7010u64), rdi); bb = 14; continue; }
            14 => { crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x7008u64), (1) as u32); fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x7008u64))) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 19; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x7000u64))) as u64; fl_l = rdx; fl_r = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), rax); bb = 18; continue; }
            17 => { crate::simd_ops::wu64(rcx.wrapping_add(0x7000u64), rax); bb = 18; continue; }
            18 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 19; continue; }
            19 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159dbb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70; const P1: u64 = 0xd8; const P2: u64 = 0x141599340; const P3: u64 = 0x1415a2050; const P4: u64 = 0x143931698; const P5: u64 = 0x14159da30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rbx = rcx;
    fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P1))) as u8 as u64; fl_r = (0x2u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    rcx = P4;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rbx.wrapping_add(P0), (0x0u64) as u64);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159dc00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xfffffffffffffff0; const P1: u64 = 0x1415aee10; const P2: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = ((rcx.wrapping_add(P0)) as u64) as u64;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rbx; fl_r = rbx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0x2; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159dc40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x58; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x1415242e0; const P4: u64 = 0x1c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rbx = 0;
    fl_l = 0; fl_r = 0;
    rsi = rcx;
    fl_l = rdi; fl_r = rdi;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(P2), ((rbx as u32)) as u32); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(P1), rbx); crate::simd_ops::wu32(rdi.wrapping_add(P4), ((rbx as u32)) as u32); } rdx = rdi; rcx = 0x2; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi.wrapping_add(P0), rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rbx);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159dcb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70; const P1: u64 = 0x78; const P2: u64 = 0x1415242e0; const P3: u64 = 0x7c; const P4: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; rdi = rcx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rsi = 0; fl_l = 0; fl_r = 0; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(P1), ((rsi as u32)) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P0), rsi); crate::simd_ops::wu32(rbx.wrapping_add(P3), ((rsi as u32)) as u32); bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rdi, rsi); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159dd20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1439316a0; const P1: u64 = 0x141524330; const P2: u64 = 0x14159bd10; const P3: u64 = 0x1415ae300; const P4: u64 = 0x142689748; const P5: u64 = 0x14159ee20; const P6: u64 = 0x1415a04e0; const P7: u64 = 0x14159dd20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x70u64))) as u64; fl_r = (0x0u64) as u64; rsi = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 18; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbx); rbx = (crate::simd_ops::ru64(P0)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rbp); rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rdi); rdi = ((rbp as u32)) as u32 as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x7010u64))) as u64; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x7000u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rdx = 0x7018; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 5; continue; }
            5 => { rcx = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(P0)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(P0, rbx); crate::simd_ops::wu64(rbx.wrapping_add(0x7000u64), rbp); bb = 8; continue; }
            7 => { crate::simd_ops::wu64(rbx.wrapping_add(0x7000u64), rax); crate::simd_ops::wu64(P0, rbx); bb = 8; continue; }
            8 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x7010u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rax; fl_r = rax; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbp } else { rax }) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x7010u64), rax); fl_l = (crate::simd_ops::lk_sub_u32(rbx.wrapping_add(0x7008u64), (1) as u32)) as u32 as u64; bb = 10; continue; }
            10 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 11; continue; }
            11 => { rdx = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rdi, rbp); rcx = rdi.wrapping_add(0x20); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rbp); crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rbp as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rbp as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x48u64), (0xffu8) as u8); crate::simd_ops::wu8(rsp.wrapping_add(0x50u64), (0xffu8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbp); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(0x90u64), rbp); rax = P4; crate::simd_ops::wu64(rdi.wrapping_add(0x20u64), rax); rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi.wrapping_add(0x98u64), rbp); rcx = rsi; crate::simd_ops::wu32(rdi.wrapping_add(0xa0u64), ((rbp as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0xb8u64), ((rbp as u32)) as u32); rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0xd8u64))) as u32) as u32 as u64; crate::simd_ops::wu64(rdi.wrapping_add(0xc0u64), rbp); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xf9u8))) as u8 as u64)); crate::simd_ops::wu64(rdi.wrapping_add(0xc8u64), rbp); rax = (rax & !0xffu64) | ((((((rax as u8)) | (0x1u8))) as u8 as u64)); crate::simd_ops::wu8(rdi.wrapping_add(0xd8u64), ((rax as u8)) as u8); crate::simd_ops::wu64(rdi.wrapping_add(0xd0u64), rbp); crate::simd_ops::wu64(rsi.wrapping_add(0x70u64), rdi); rdi = ((crate::simd_ops::ru8(rsi.wrapping_add(0xd4u64))) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x70u64))) as u64; r8 = (((rdi as u8)) as u32) as u32 as u64; r8 = (r8 & !0xffu64) | (((((r8 as u8)).wrapping_shr(1u32)) as u8 as u64)); rdx = (((rdi as u8)) as u32) as u32 as u64; r8 = (r8 & !0xffu64) | ((((((r8 as u8)) & (0x1u8))) as u8 as u64)); rdx = (rdx & !0xffu64) | ((((((rdx as u8)) & (0x1u8))) as u8 as u64)); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((r8 as u8)) as u8); r9 = (((rax as u16)) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((rdx as u8)) as u8); rcx = rbx.wrapping_add(0x20); rdx = rsi; r8 = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdi = (rdi & !0xffu64) | (((((rdi as u8)).wrapping_shr(4u32)) as u8 as u64)); rdi = (rdi & !0xffu64) | ((((((rdi as u8)) ^ (crate::simd_ops::ru8(rbx.wrapping_add(0xd8u64))))) as u8 as u64)); rdi = (rdi & !0xffu64) | ((((((rdi as u8)) & (0x1u8))) as u8 as u64)); crate::simd_ops::wu64(rbx.wrapping_add(0xd0u64), rbp); crate::simd_ops::lk_xor_u8(rbx.wrapping_add(0xd8u64), ((rdi as u8)) as u8); rbx = (rbx & !0xffu64) | (((0x1u8) as u8 as u64)); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x80u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rbx = (((rbx as u8)) as u32) as u32 as u64; rbx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (rbp as u32) } else { (rbx as u32) }) as u32 as u64; bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x78u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rbx = (((rbx as u8)) as u32) as u32 as u64; rbx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (rbp as u32) } else { (rbx as u32) }) as u32 as u64; bb = 15; continue; }
            15 => { rax = (((rbx as u8)) as u32) as u32 as u64; bb = 16; continue; }
            16 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            17 => { crate::simd_ops::wu64(rsi.wrapping_add(0x70u64), rdi); rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 16; continue; }
            18 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x60u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159df30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x58; const P1: u64 = 0x28; const P2: u64 = 0x2; const P3: u64 = 0x141524330; const P4: u64 = 0x21; const P5: u64 = 0x25; const P6: u64 = 0x27; const P7: u64 = 0x8; const P8: u64 = 0x10; const P9: u64 = 0x18; const P10: u64 = 0x20; const P11: u64 = 0x70; const P12: u64 = 0x1415aef80;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rcx; rdi = ((rdx as u32)) as u32 as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rdx = ((rax.wrapping_add(P1)) as u32) as u32 as u64; rcx = ((rax.wrapping_add(P2)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = 0; fl_l = 0; fl_r = 0; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rax.wrapping_add(P4), ((rcx as u32)) as u32); crate::simd_ops::wu16(rax.wrapping_add(P5), ((rcx as u16)) as u16); crate::simd_ops::wu8(rax.wrapping_add(P6), ((rcx as u8)) as u8); crate::simd_ops::wu64(rax, rcx); crate::simd_ops::wu64(rax.wrapping_add(P7), rcx); crate::simd_ops::wu64(rax.wrapping_add(P8), rcx); crate::simd_ops::wu64(rax.wrapping_add(P9), rcx); crate::simd_ops::wu8(rax.wrapping_add(P10), ((rcx as u8)) as u8); bb = 4; continue; }
            3 => { rax = rcx; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rbx.wrapping_add(P0), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(P7))) as u64; rdx = rdi; fl_l = (((rcx) >> (((rdx) as u32 & 63))) & 1) as u64; r8 = (crate::simd_ops::ru64(rbx.wrapping_add(P11))) as u64; if ((fl_l as u64) != 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = (((rcx) >> (((rdx) as u32 & 63))) & 1) as u64; rcx = ((rcx) | ((1u64) << (((rdx) as u32 & 63)))) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rcx); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159dfc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x68; const P1: u64 = 0x98; const P2: u64 = 0xffffffffffffffba; const P3: u64 = 0x141524330; const P4: u64 = 0x10; const P5: u64 = 0x20; const P6: u64 = 0x30; const P7: u64 = 0x40; const P8: u64 = 0x141607930; const P9: u64 = 0x38;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64; rdi = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdx = 0x48; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rax = (0x20020000ffeu64) as u64; crate::simd_ops::lk_or_u64(rcx.wrapping_add(P1), (rax) as u64); rcx = ((rdx.wrapping_add(P2)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { xmm0 = 0u128; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu128(rax, (xmm0) as u128); crate::simd_ops::wu128(rax.wrapping_add(P4), (xmm0) as u128); crate::simd_ops::wu128(rax.wrapping_add(P5), (xmm0) as u128); crate::simd_ops::wu128(rax.wrapping_add(P6), (xmm0) as u128); crate::simd_ops::wu64(rax.wrapping_add(P7), rcx); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = 0; fl_l = 0; fl_r = 0; rax = 0x1; crate::simd_ops::wu32(rbx.wrapping_add(P9), ((rcx as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(P7), rcx); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(rbx, ((rcx as u32)) as u32); crate::simd_ops::wu64(rdi.wrapping_add(P0), rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0x34; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            4 => { rax = 0x1; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159e060(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14158f5a0; const P1: u64 = 0x1415242e0; const P2: u64 = 0x140000000; const P3: u64 = 0x143426fd0; const P4: u64 = 0x1415d9480; const P5: u64 = 0x143930fa0; const P6: u64 = 0x1415c3fb0; const P7: u64 = 0x141529ed0; const P8: u64 = 0x143930f44; const P9: u64 = 0x1415317a0; const P10: u64 = 0x1415a3500;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 55>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x20u64), r9); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rbp = rax.wrapping_sub(0xb8); rsp = ((rsp).wrapping_sub(0x1b0u64)) as u64; crate::simd_ops::wu64(rax.wrapping_sub(0x10u64), rbx); rbx = r9; crate::simd_ops::wu64(rax.wrapping_sub(0x18u64), rsi); rsi = rdx; crate::simd_ops::wu64(rax.wrapping_sub(0x20u64), rdi); rdi = rcx; crate::simd_ops::wu64(rax.wrapping_sub(0x30u64), r13); rcx = rsp.wrapping_add(0x60); r13 = r8; crate::simd_ops::wu8(r8.wrapping_add(0x4u64), (0x2u8) as u8); crate::simd_ops::wu32(r8, (0x0u32) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x38u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { xmm0 = (crate::simd_ops::ru128(rbx)) as u128; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0xb0u64))) as u64; rcx = rsp.wrapping_add(0x60); xmm1 = (crate::simd_ops::ru128(rbx.wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(0x10u64), (xmm1) as u128); xmm1 = (crate::simd_ops::ru128(rbx.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(0x30u64), (xmm1) as u128); xmm1 = (crate::simd_ops::ru128(rbx.wrapping_add(0x50u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(0x40u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(0x60u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(0x50u64), (xmm1) as u128); xmm1 = (crate::simd_ops::ru128(rbx.wrapping_add(0x80u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(0x60u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(0x70u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(0x70u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(0x90u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(0x80u64), (xmm1) as u128); xmm1 = (crate::simd_ops::ru128(rbx.wrapping_add(0xa0u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(0x90u64), (xmm0) as u128); crate::simd_ops::wu128(rcx.wrapping_add(0xa0u64), (xmm1) as u128); crate::simd_ops::wu64(rcx.wrapping_add(0xb0u64), rax); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x68u64))) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x38u64), (xmm0) as u128); crate::simd_ops::wu64(rbp.wrapping_sub(0x38u64), rax); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x70u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x30u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x74u64))) as u32 as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x70u64), (0x0u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x68u64), (0x0u64) as u64); rbx = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x2cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 35; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x180u64), r14); r14 = P2; crate::simd_ops::wu128(rsp.wrapping_add(0x130u64), (xmm9) as u128); xmm9 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x120u64), (xmm10) as u128); xmm10 = ((crate::simd_ops::ru32(P3)) as u32) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x190u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x178u64), r15); crate::simd_ops::wu128(rsp.wrapping_add(0x160u64), (xmm6) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x150u64), (xmm7) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x140u64), (xmm8) as u128); bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0xdu64))) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x7; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru32(rax)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 33; continue; } bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; r14 = (((crate::simd_ops::ru32(r14.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x26895f0u64))) as i32 as i64) as u64) as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = 0x4a; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 32; continue; } bb = 9; continue; }
            9 => { rax = ((r14 as u32)) as u32 as u64; r12 = ((r14 as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffe0u32))) as u32 as u64; r12 = ((r12).wrapping_shr(5u32)) as u64; rcx = ((r14 as u32)) as u32 as u64; r15 = 0x1; rcx = (((rcx as u32)).wrapping_sub((rax as u32))) as u32 as u64; r15 = ((r15).wrapping_shl(((rcx as u32) & 63))) as u64; rax = (crate::simd_ops::ru32(rbp.wrapping_add(r12.wrapping_mul(4)).wrapping_sub(0x10u64))) as u32 as u64; fl_l = r15; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(P5)) as u64; r9 = rsp.wrapping_add(0x30); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x0u32) as u32); r8 = rdi; rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0xdu64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x4cu64), ((rax as u8)) as u8); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (0x0u64) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru64(rbx)) as u64; xmm7 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32) as u128; fl_l = ((xmm7 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 12; continue; } bb = 11; continue; }
            11 => { xmm6 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x38u64))) as u32) as u128; bb = 14; continue; }
            12 => { fl_l = ((xmm7 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 15; continue; } bb = 13; continue; }
            13 => { xmm6 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x3cu64))) as u32) as u128; bb = 14; continue; }
            14 => { fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm10 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 19; continue; } bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x28u64))) as u32 as u64; r8 = rsp.wrapping_add(0x34); xmm1 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32) as u128; rcx = rdi.wrapping_add(0x18); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; xmm6 = (xmm0) as u128; crate::simd_ops::wu32(rdi.wrapping_add(0x28u64), ((rax as u32)) as u32); bb = 18; continue; }
            17 => { xmm6 = (xmm9) as u128; bb = 18; continue; }
            18 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x14u64))) as u32) as u32 as u64; rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0xc8u64))) as u64; crate::simd_ops::wu32(rdi.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x30u64), ((xmm7 as u32)) as u32); rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x14u64))) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x38u64), ((xmm6 as u32)) as u32); fl_l = (crate::simd_ops::lk_xor_u8(rdi.wrapping_add(0x14u64), (0x1u8) as u8)) as u8 as u64; fl_r = fl_l; bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = P2; rdx = (crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu32(r13, ((rdx as u32)) as u32); rdi = (crate::simd_ops::ru32(rcx.wrapping_add(r14.wrapping_mul(4)).wrapping_add(0x2689180u64))) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1a; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 31; continue; } bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::ru32(P8)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 42; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(r12.wrapping_mul(4)).wrapping_sub(0x4u64))) as u32 as u64; fl_l = r15; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 42; continue; } bb = 22; continue; }
            22 => { fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm9 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 31; continue; } bb = 23; continue; }
            23 => { rax = ((crate::simd_ops::ru8(r13.wrapping_add(0x4u64))) as u32) as u32 as u64; r8 = rbp.wrapping_add(0xd0); rcx = (crate::simd_ops::ru32(r13.wrapping_add(0x8u64))) as u32 as u64; xmm8 = (xmm9) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x0u32) as u32); xmm7 = (xmm9) as u128; crate::simd_ops::wu8(rsp.wrapping_add(0x39u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r13.wrapping_add(0x5u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x3au64), ((rax as u8)) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x54u64), ((rdx as u32)) as u32); rdx = rsp.wrapping_add(0x50); crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), ((rcx as u32)) as u32); rcx = rbp.wrapping_sub(0x38); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((r14 as u8)) as u8); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x38u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0xd0u64), (0x0u8) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 24; continue; }
            24 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0xd0u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 26; continue; } bb = 25; continue; }
            25 => { crate::simd_ops::wu32(rax.wrapping_add(0xcu64), (0x0u32) as u32); bb = 26; continue; }
            26 => { xmm8 = ((crate::simd_ops::ru32(rax.wrapping_add(0xcu64))) as u32) as u128; xmm7 = (xmm8) as u128; xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) + f32::from_bits(xmm6 as u32)).to_bits() as u128); crate::simd_ops::wu32(rax.wrapping_add(0xcu64), ((xmm7 as u32)) as u32); bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(r12.wrapping_mul(4)).wrapping_add(0x8u64))) as u32 as u64; fl_l = r15; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 30; continue; } bb = 28; continue; }
            28 => { xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(rdi.wrapping_mul(4)).wrapping_add(0x60u64))) as u32) as u128; fl_l = ((xmm8 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 38; continue; } bb = 29; continue; }
            29 => { fl_l = ((xmm7 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 39; continue; } bb = 30; continue; }
            30 => { fl_l = (crate::simd_ops::lk_or_u8(rbp.wrapping_add(0x17u64), (0x4u8) as u8)) as u8 as u64; fl_r = fl_l; bb = 31; continue; }
            31 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0xc0u64))) as u64; bb = 32; continue; }
            32 => { r14 = P2; bb = 33; continue; }
            33 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 34; continue; }
            34 => { xmm10 = (crate::simd_ops::ru128(rsp.wrapping_add(0x120u64))) as u128; xmm9 = (crate::simd_ops::ru128(rsp.wrapping_add(0x130u64))) as u128; xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x140u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x150u64))) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x160u64))) as u128; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x178u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x180u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x190u64))) as u64; bb = 35; continue; }
            35 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xd8u64))) as u64; rdx = rsp.wrapping_add(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x38u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x188u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x198u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x1a0u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x1a8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 37; continue; } bb = 36; continue; }
            36 => { rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 37; continue; }
            37 => { rsp = ((rsp).wrapping_add(0x1b0u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            38 => { fl_l = ((xmm7 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; bb = 39; continue; }
            39 => { r14 = P2; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 41; continue; } bb = 40; continue; }
            40 => { crate::simd_ops::wu32(rsp.wrapping_add(rdi.wrapping_mul(4)).wrapping_add(0x60u64), ((xmm7 as u32)) as u32); bb = 41; continue; }
            41 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0xc0u64))) as u64; bb = 33; continue; }
            42 => { xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(rdi.wrapping_mul(4)).wrapping_add(0x60u64)))).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(rdi.wrapping_mul(4)).wrapping_add(0x60u64), ((xmm6 as u32)) as u32); bb = 31; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159e520(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415c3010; const P1: u64 = 0x143930fa0; const P2: u64 = 0x1415c4320;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x58u64))) as u64; rdi = r8; r8 = 0; fl_l = 0; fl_r = 0; rbp = rdx; r14 = rcx; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { r11 = (crate::simd_ops::ru64(rax)) as u64; bb = 3; continue; }
            2 => { r11 = r8; bb = 3; continue; }
            3 => { r9 = (crate::simd_ops::ru64(rcx.wrapping_add(0x90u64))) as u64; rbx = 0xfffffffc; crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((r8 as u32)) as u32); fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { r10 = ((crate::simd_ops::ru8(r9)) as u32) as u32 as u64; rcx = ((r8 as u32)) as u32 as u64; bb = 5; continue; }
            5 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r9))) as u8 as u64; fl_r = (0xcu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 20; continue; } bb = 6; continue; }
            6 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 7; continue; }
            7 => { rdx = r8; bb = 8; continue; }
            8 => { fl_l = rdx; fl_r = rdx; rax = rsp.wrapping_add(0x50); rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rdx } else { rax }) as u64; rax = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rdi, ((rax as u32)) as u32); r9 = (crate::simd_ops::ru64(r14.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((r8 as u32)) as u32); fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { r10 = ((crate::simd_ops::ru8(r9)) as u32) as u32 as u64; rcx = ((r8 as u32)) as u32 as u64; bb = 10; continue; }
            10 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r9))) as u8 as u64; fl_r = (0xdu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 21; continue; } bb = 11; continue; }
            11 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 10; continue; } bb = 12; continue; }
            12 => { rdx = r8; bb = 13; continue; }
            13 => { fl_l = rdx; fl_r = rdx; rax = rsp.wrapping_add(0x50); rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rdx } else { rax }) as u64; rax = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x50u64))) as u32) as u128; crate::simd_ops::wu32(rdi.wrapping_add(0x4u64), ((xmm0 as u32)) as u32); r9 = (crate::simd_ops::ru64(r14.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((r8 as u32)) as u32); fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 14; continue; }
            14 => { r10 = ((crate::simd_ops::ru8(r9)) as u32) as u32 as u64; rcx = ((r8 as u32)) as u32 as u64; bb = 15; continue; }
            15 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r9))) as u8 as u64; fl_r = (0x49u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 22; continue; } bb = 16; continue; }
            16 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 15; continue; } bb = 17; continue; }
            17 => { rdx = r8; bb = 18; continue; }
            18 => { fl_l = rdx; fl_r = rdx; rax = rsp.wrapping_add(0x50); rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rdx } else { rax }) as u64; rax = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x50u64))) as u32) as u128; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((xmm0 as u32)) as u32); fl_l = (((r11) >> (24u32)) & 1) as u64; if ((fl_l as u64) == 0) { bb = 23; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((r8 as u32)) as u32); bb = 30; continue; }
            20 => { rax = ((r10.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = (((rax) & (rbx))) as u64; fl_l = ((rax).wrapping_add(r9)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; bb = 8; continue; }
            21 => { rax = ((r10.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = (((rax) & (rbx))) as u64; fl_l = ((rax).wrapping_add(r9)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; bb = 13; continue; }
            22 => { rax = ((r10.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = (((rax) & (rbx))) as u64; fl_l = ((rax).wrapping_add(r9)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; bb = 18; continue; }
            23 => { r9 = (crate::simd_ops::ru64(r14.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((r8 as u32)) as u32); fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 24; continue; }
            24 => { r10 = ((crate::simd_ops::ru8(r9)) as u32) as u32 as u64; rcx = ((r8 as u32)) as u32 as u64; bb = 25; continue; }
            25 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r9))) as u8 as u64; fl_r = (0xeu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 28; continue; } bb = 26; continue; }
            26 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 25; continue; } bb = 27; continue; }
            27 => { bb = 29; continue; }
            28 => { rax = ((r10.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = (((rax) & (rbx))) as u64; fl_l = ((rax).wrapping_add(r9)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; r8 = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; bb = 29; continue; }
            29 => { fl_l = r8; fl_r = r8; rax = rsp.wrapping_add(0x50); rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { r8 } else { rax }) as u64; rax = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x50u64))) as u32) as u128; crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((xmm0 as u32)) as u32); bb = 30; continue; }
            30 => { rax = (0x4000000010c0000u64) as u64; fl_l = rax; fl_r = r11; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 47; continue; } bb = 31; continue; }
            31 => { r9 = (crate::simd_ops::ru64(r14.wrapping_add(0x48u64))) as u64; rdx = rsp.wrapping_add(0x50); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x50u64))) as u32 as u64; rcx = r14.wrapping_add(0x48); r8 = 0x12; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rsi); rax = ((r9.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru32(r14.wrapping_add(0x50u64))) as u32 as u64; r8 = (crate::simd_ops::ru64(r14.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rax)) as u64; rax = ((r8.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 35; continue; } bb = 32; continue; }
            32 => { rcx = ((rdx as u32)) as u32 as u64; bb = 33; continue; }
            33 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = ((rcx as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; fl_r = 0x12; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 35; continue; } bb = 34; continue; }
            34 => { rcx = (crate::simd_ops::ru64(P1)) as u64; r8 = rbp; rdx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rdi))).to_bits() as u128); rbx = ((rbx).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu32(rdi, ((xmm0 as u32)) as u32); rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x50u64))) as u32 as u64; r8 = (crate::simd_ops::ru64(r14.wrapping_add(0x48u64))) as u64; rdx = ((rcx as u32)) as u32 as u64; rax = ((r8.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 33; continue; } bb = 35; continue; }
            35 => { rax = ((rdx as u32)) as u32 as u64; rcx = ((r8.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 39; continue; } bb = 36; continue; }
            36 => { rcx = ((rdx as u32)) as u32 as u64; bb = 37; continue; }
            37 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = ((rcx as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; fl_r = 0x13; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 39; continue; } bb = 38; continue; }
            38 => { rcx = (crate::simd_ops::ru64(P1)) as u64; r8 = rbp; rdx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rdi.wrapping_add(0x4u64)))).to_bits() as u128); rbx = ((rbx).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x4u64), ((xmm0 as u32)) as u32); rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x50u64))) as u32 as u64; r8 = (crate::simd_ops::ru64(r14.wrapping_add(0x48u64))) as u64; rdx = ((rcx as u32)) as u32 as u64; rax = ((r8.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 37; continue; } bb = 39; continue; }
            39 => { rax = ((rdx as u32)) as u32 as u64; r9 = rbx; rdx = rsp.wrapping_add(0x50); rcx = ((r8.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; r8 = 0x18; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); rcx = r14.wrapping_add(0x48); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru32(r14.wrapping_add(0x50u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rax)) as u64; rax = ((rcx.wrapping_add(r9.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 43; continue; } bb = 40; continue; }
            40 => { r8 = ((r9 as u32)) as u32 as u64; bb = 41; continue; }
            41 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; r9 = ((r8 as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; fl_r = 0x18; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 43; continue; } bb = 42; continue; }
            42 => { rcx = (crate::simd_ops::ru64(P1)) as u64; r8 = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rdi.wrapping_add(0xcu64)))).to_bits() as u128); rbx = ((rbx).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((xmm0 as u32)) as u32); r8 = (crate::simd_ops::ru32(r14.wrapping_add(0x50u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x48u64))) as u64; r9 = ((r8 as u32)) as u32 as u64; rax = ((rcx.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 41; continue; } bb = 43; continue; }
            43 => { rax = ((r9 as u32)) as u32 as u64; rdx = rsp.wrapping_add(0x50); r9 = rbx; r8 = 0x3a; rcx = ((rcx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); rcx = r14.wrapping_add(0x48); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x50u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(0x48u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 46; continue; } bb = 44; continue; }
            44 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; fl_r = 0x3a; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 46; continue; } bb = 45; continue; }
            45 => { rcx = (crate::simd_ops::ru64(P1)) as u64; r8 = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rdi.wrapping_add(0x8u64)))).to_bits() as u128); rbx = ((rbx).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((xmm0 as u32)) as u32); rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x50u64))) as u32 as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(0x48u64))) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 44; continue; } bb = 46; continue; }
            46 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; bb = 47; continue; }
            47 => { rcx = ((crate::simd_ops::ru8(r14.wrapping_add(0xd5u64))) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_shr(2u32)) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) ^ (crate::simd_ops::ru8(rdi.wrapping_add(0x10u64))))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0x7u8))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) ^ (crate::simd_ops::ru8(rdi.wrapping_add(0x10u64))))) as u8 as u64)); crate::simd_ops::wu8(rdi.wrapping_add(0x10u64), ((rcx as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0xd5u64))) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shl(2u32)) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) ^ ((rcx as u8)))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x8u8))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) ^ ((rcx as u8)))) as u8 as u64)); crate::simd_ops::wu8(rdi.wrapping_add(0x10u64), ((rax as u8)) as u8); rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159e8e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x98; const P1: u64 = 0x78; const P2: u64 = 0x68; const P3: u64 = 0xffffffffffffffcd; const P4: u64 = 0x141524330; const P5: u64 = 0x10; const P6: u64 = 0x20; const P7: u64 = 0x30; const P8: u64 = 0x141607930;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rsi = rdx; rcx = (0x20020000ffeu64) as u64; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rbx = rax; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 4; continue; }
            4 => { rdx = 0x38; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rcx = ((rdx.wrapping_add(P3)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { xmm0 = 0u128; rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu128(rdi, (xmm0) as u128); rcx = rdi; crate::simd_ops::wu128(rdi.wrapping_add(P5), (xmm0) as u128); crate::simd_ops::wu128(rdi.wrapping_add(P6), (xmm0) as u128); crate::simd_ops::wu64(rdi.wrapping_add(P7), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; xmm0 = (crate::simd_ops::ru128(rcx)) as u128; crate::simd_ops::wu128(rdi, (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(P5))) as u128; crate::simd_ops::wu128(rdi.wrapping_add(P5), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(P6))) as u128; crate::simd_ops::wu128(rdi.wrapping_add(P6), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rcx.wrapping_add(P7))) as u64) as u128; crate::simd_ops::wu64(rdi.wrapping_add(P7), ((xmm1 as u64)) as u64); crate::simd_ops::wu64(rsi, rdi); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rsi, (0x0u64) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rdx, (0x0u64) as u64); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159e9c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415c2f80; const P1: u64 = 0x143930fa0; const P2: u64 = 0x1415c4320; const P3: u64 = 0x1434264e4;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x58u64))) as u64; rbx = r8; rsi = rdx; r14 = rcx; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_l = rax; fl_r = (0x700000u64) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 2; continue; }
            2 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); crate::simd_ops::wu64(r8, rax); rdx = rsp.wrapping_add(0x58); crate::simd_ops::wu32(r8.wrapping_add(0x8u64), ((rax as u32)) as u32); rcx = ((rcx).wrapping_add(0x48u64)) as u64; r8 = ((rax.wrapping_add(0x14u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru32(r14.wrapping_add(0x50u64))) as u32 as u64; r8 = (crate::simd_ops::ru64(r14.wrapping_add(0x48u64))) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rcx = ((r8.wrapping_add(r9.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { r10 = ((r9 as u32)) as u32 as u64; bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rax)) as u64; r9 = ((r10 as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; fl_r = 0x14; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(P1)) as u64; r8 = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rbx))).to_bits() as u128); rax = ((rax).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); crate::simd_ops::wu32(rbx, ((xmm0 as u32)) as u32); r10 = (crate::simd_ops::ru32(r14.wrapping_add(0x50u64))) as u32 as u64; r8 = (crate::simd_ops::ru64(r14.wrapping_add(0x48u64))) as u64; r9 = ((r10 as u32)) as u32 as u64; rcx = ((r8.wrapping_add(r10.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 6; continue; }
            6 => { rcx = ((r9 as u32)) as u32 as u64; rdx = ((r8.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 7; continue; }
            7 => { r10 = ((r9 as u32)) as u32 as u64; bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rax)) as u64; r9 = ((r10 as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; fl_r = 0x15; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(P1)) as u64; r8 = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_add(0x8u64)))).to_bits() as u128); rax = ((rax).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((xmm0 as u32)) as u32); r10 = (crate::simd_ops::ru32(r14.wrapping_add(0x50u64))) as u32 as u64; r8 = (crate::simd_ops::ru64(r14.wrapping_add(0x48u64))) as u64; r9 = ((r10 as u32)) as u32 as u64; rcx = ((r8.wrapping_add(r10.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 10; continue; }
            10 => { rcx = ((r9 as u32)) as u32 as u64; rdx = ((r8.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; fl_r = 0x16; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(P1)) as u64; r8 = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_add(0x4u64)))).to_bits() as u128); rax = ((rax).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), ((xmm0 as u32)) as u32); rdx = (crate::simd_ops::ru32(r14.wrapping_add(0x50u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x48u64))) as u64; rdx = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(r14)) as u64; rdx = rsp.wrapping_add(0x40); xmm0 = 0u128; rcx = r14; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((xmm0 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0xf8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { xmm2 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(crate::simd_ops::ru32(P3))).to_bits() as u128); xmm0 = (xmm2) as u128; xmm1 = (xmm2) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rbx))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_add(0x8u64)))).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_add(0x4u64)))).to_bits() as u128); crate::simd_ops::wu32(rbx, ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), ((xmm2 as u32)) as u32); bb = 15; continue; }
            15 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159eb80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143815054; const P1: u64 = 0x143930f88; const P2: u64 = 0x1426452b0; const P3: u64 = 0x1426452a8; const P4: u64 = 0x142645560;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xd4u64))) as u8 as u64; fl_r = (0x8u8) as u8 as u64; r14 = rdx; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x78u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x80u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0xd4u64))) as u8 as u64; fl_r = (0x8u8) as u8 as u64; rcx = rax; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 1; continue; } bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x90u64))) as u64; r15 = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru32(P0)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((r15 as u32)) as u32 as u64; bb = 6; continue; }
            6 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0x39u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 21; continue; } bb = 7; continue; }
            7 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 6; continue; } bb = 8; continue; }
            8 => { rdx = r15; bb = 9; continue; }
            9 => { fl_l = rdx; fl_r = rdx; rax = rsp.wrapping_add(0x20); rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rdx } else { rax }) as u64; rbp = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 36; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); rbx = (crate::simd_ops::ru64(P1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rsi); bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x370u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 22; continue; } bb = 12; continue; }
            12 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x370u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 13; continue; }
            13 => { rsi = rbx.wrapping_add(0x378); bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x3b8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 24; continue; } bb = 15; continue; }
            15 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbp as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x3b0u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 19; continue; } bb = 17; continue; }
            17 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 16; continue; } bb = 18; continue; }
            18 => { rdi = r15; bb = 19; continue; }
            19 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 25; continue; }
            21 => { rdx = 0xfffffffc; rax = ((r9.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = (((rax) & (rdx))) as u64; fl_l = ((rax).wrapping_add(r8)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; bb = 9; continue; }
            22 => { rsi = rbx.wrapping_add(0x378); rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x370u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 14; continue; } bb = 23; continue; }
            23 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            24 => { rdi = r15; bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x370u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 33; continue; } bb = 26; continue; }
            26 => { bb = 27; continue; }
            27 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x370u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 30; continue; } bb = 28; continue; }
            28 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x370u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 27; continue; } bb = 29; continue; }
            29 => { bb = 34; continue; }
            30 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 35; continue; } bb = 31; continue; }
            31 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x3a0u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 35; continue; } bb = 32; continue; }
            32 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x3a8u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 35; continue; }
            33 => { rsi = rbx.wrapping_add(0x378); bb = 34; continue; }
            34 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 35; continue; }
            35 => { rcx = (crate::simd_ops::ru64(r14)) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(r14, rdi); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; bb = 37; continue; }
            36 => { rcx = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(r14, r15); bb = 37; continue; }
            37 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 40; continue; } bb = 38; continue; }
            38 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 40; continue; } bb = 39; continue; }
            39 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 40; continue; }
            40 => { crate::simd_ops::wu16(r14.wrapping_add(0x8u64), ((r15 as u16)) as u16); rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159ed60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14160eca0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r9 = rcx;
    fl_l = ((rcx).wrapping_add(0xb8u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159ed70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x80; const P1: u64 = 0x78;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; return rax; }
            5 => { rax = 0; fl_l = 0; fl_r = 0; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159eda0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x78; const P1: u64 = 0x14159eda0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rax = 0;
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159edc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x58; const P1: u64 = 0x28; const P2: u64 = 0x2; const P3: u64 = 0x141524330; const P4: u64 = 0x21; const P5: u64 = 0x25; const P6: u64 = 0x27; const P7: u64 = 0x8; const P8: u64 = 0x10; const P9: u64 = 0x18; const P10: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rbx = rcx;
    fl_l = rax; fl_r = rax;
    if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rdx = ((rax.wrapping_add(P1)) as u32) as u32 as u64;
    rcx = ((rax.wrapping_add(P2)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { crate::simd_ops::wu32(rax.wrapping_add(P4), ((rcx as u32)) as u32); crate::simd_ops::wu16(rax.wrapping_add(P5), ((rcx as u16)) as u16); crate::simd_ops::wu8(rax.wrapping_add(P6), ((rcx as u8)) as u8); crate::simd_ops::wu64(rax, rcx); crate::simd_ops::wu64(rax.wrapping_add(P7), rcx); crate::simd_ops::wu64(rax.wrapping_add(P8), rcx); crate::simd_ops::wu64(rax.wrapping_add(P9), rcx); crate::simd_ops::wu8(rax.wrapping_add(P10), ((rcx as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(P0), rax); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = rcx;
    crate::simd_ops::wu64(rbx.wrapping_add(P0), rcx);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159ee20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xd2; const P1: u64 = 0x58; const P2: u64 = 0x48; const P3: u64 = 0x50; const P4: u64 = 0x8; const P5: u64 = 0x143930fa0; const P6: u64 = 0x1415c4320;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x58u64)) as u64; r8 = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64; rax = 0x3ff; r8 = (r8 & !0xffffu64) | ((((((r8 as u16)) & ((rax as u16)))) as u16 as u64)); r10 = rdx; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; r9 = rcx; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_l = (((rax) >> (16u32)) & 1) as u64; if ((fl_l as u64) == 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { fl_l = ((r8 as u16)) as u16 as u64; fl_r = ((r8 as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rax = (crate::simd_ops::ru32(r9.wrapping_add(P3))) as u32 as u64; r9 = ((rcx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64; fl_r = 0x10; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rcx = ((rcx).wrapping_add(0x8u64)) as u64; fl_l = rcx; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 6; continue; }
            6 => { rax = (((r8 as u16)) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x58u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { rcx = (crate::simd_ops::ru64(P5)) as u64; r8 = rsp.wrapping_add(0x20); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0xffu8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); crate::simd_ops::wu8(rsp.wrapping_add(0x40u64), (0xffu8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm0 as u32)))) as u32 as u64; rsp = ((rsp).wrapping_add(0x58u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159eec0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f88; const P1: u64 = 0x1426452b0; const P2: u64 = 0x1426452a8; const P3: u64 = 0x142645560;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); r15 = rdx; crate::simd_ops::wu32(rdx, (0x0u32) as u32); rdx = (0x20020000ffeu64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbp); bb = 1; continue; }
            1 => { rax = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x78u64))) as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x98u64))) as u64; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } bb = 3; continue; }
            3 => { r8 = (crate::simd_ops::ru64(rax.wrapping_add(0x90u64))) as u64; r14 = (r14 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x0u32) as u32); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0x46u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 20; continue; } bb = 6; continue; }
            6 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 7; continue; }
            7 => { rdx = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(P0)) as u64; rax = rsp.wrapping_add(0x20); fl_l = rdx; fl_r = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rsi); rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rdx } else { rax }) as u64; rbp = (crate::simd_ops::ru32(rax)) as u32 as u64; bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x210u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 21; continue; } bb = 10; continue; }
            10 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x210u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 11; continue; }
            11 => { rsi = rbx.wrapping_add(0x218); bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x258u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 23; continue; } bb = 13; continue; }
            13 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbp as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x250u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 14; continue; }
            14 => { bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 18; continue; } bb = 16; continue; }
            16 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 15; continue; } bb = 17; continue; }
            17 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 18; continue; }
            18 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 19; continue; }
            19 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 24; continue; }
            20 => { rdx = 0xfffffffc; rax = ((r9.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = (((rax) & (rdx))) as u64; fl_l = ((rax).wrapping_add(r8)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; bb = 8; continue; }
            21 => { rsi = rbx.wrapping_add(0x218); rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x210u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 12; continue; } bb = 22; continue; }
            22 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            23 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x210u64))) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 32; continue; } bb = 25; continue; }
            25 => { bb = 26; continue; }
            26 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x210u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 29; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x210u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 26; continue; } bb = 28; continue; }
            28 => { bb = 33; continue; }
            29 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 34; continue; } bb = 30; continue; }
            30 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x240u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 34; continue; } bb = 31; continue; }
            31 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x248u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 34; continue; }
            32 => { rsi = rbx.wrapping_add(0x218); bb = 33; continue; }
            33 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 34; continue; }
            34 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 41; continue; } bb = 35; continue; }
            35 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x1f8u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 39; continue; } bb = 36; continue; }
            36 => { rax = ((rax).wrapping_add(0x2u64)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = (crate::simd_ops::ru64(rdi.wrapping_add(rax.wrapping_mul(8)))) as u64; rcx = ((rdi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 39; continue; } bb = 37; continue; }
            37 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 39; continue; } bb = 38; continue; }
            38 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; r14 = (r14 & !0xffu64) | (((0x1u8) as u8 as u64)); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru32(r8.wrapping_add(rdx.wrapping_mul(4)))) as u32 as u64; crate::simd_ops::wu32(r15, ((rax as u32)) as u32); bb = 39; continue; }
            39 => { rdx = 0xffffffff; let xa_src: u32 = (rdx as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rdx = (xa_old) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 41; continue; } bb = 40; continue; }
            40 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 41; continue; }
            41 => { rax = (((r14 as u8)) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159f0c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xd4; const P1: u64 = 0x80; const P2: u64 = 0x98; const P3: u64 = 0x78; const P4: u64 = 0x60; const P5: u64 = 0x14159c4e0; const P6: u64 = 0x1415ca5a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); rdi = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rbx as u32)) as u32); fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 14; continue; } bb = 1; continue; }
            1 => { rcx = (0x40000000000u64) as u64; bb = 2; continue; }
            2 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdi = rax; bb = 2; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 19; continue; } bb = 10; continue; }
            10 => { bb = 11; continue; }
            11 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rdi = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 13; continue; }
            13 => { bb = 19; continue; }
            14 => { rbx = rcx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 15; continue; }
            15 => { bb = 16; continue; }
            16 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 18; continue; }
            18 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 16; continue; } bb = 20; continue; }
            20 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = 0x1; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159f190(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x80; const P1: u64 = 0x100; const P2: u64 = 0x78;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx)) as u64; return crate::simd_ops::call_fwd(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } bb = 3; continue; }
            3 => { rax = 0; fl_l = 0; fl_r = 0; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159f1c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415d90a0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = ((r8 as u32)) as u32 as u64;
    rcx = ((rcx).wrapping_add(0x40u64)) as u64;
    r8 = rdx;
    rdx = ((rax as u32)) as u32 as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = 0x1;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159f1e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xd4; const P1: u64 = 0x80; const P2: u64 = 0x98; const P3: u64 = 0x78; const P4: u64 = 0x60; const P5: u64 = 0x10; const P6: u64 = 0x1415ca5a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = 0; fl_l = 0; fl_r = 0; rbx = ((rdi as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 13; continue; } bb = 1; continue; }
            1 => { rdx = (0x40000000000u64) as u64; bb = 2; continue; }
            2 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = rax; bb = 2; continue; }
            7 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdi = (crate::simd_ops::ru32(rax.wrapping_add(P5))) as u32 as u64; bb = 9; continue; }
            9 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 17; continue; } bb = 10; continue; }
            10 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 12; continue; }
            12 => { bb = 17; continue; }
            13 => { rbx = rcx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { fl_l = (((rdi as u32)).wrapping_add(crate::simd_ops::ru32(rax.wrapping_add(P5)))) as u32 as u64; fl_r = fl_l; rdi = ((fl_l as u32)) as u32 as u64; bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 14; continue; } bb = 18; continue; }
            18 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = ((rdi as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159f2a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x68; const P1: u64 = 0x78;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = ((rax).wrapping_add(0x38u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; return rax; }
            4 => { rax = 0; fl_l = 0; fl_r = 0; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159f2d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14159e520; const P1: u64 = 0x14159b8a0; const P2: u64 = 0x143930fa0; const P3: u64 = 0x14153ad70;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rbx); r14 = r9; rbx = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbp); rcx = (0x20020000ffeu64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), r15); rbp = rdx; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x98u64))) as u64; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x78u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rbx = rax; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x98u64))) as u64; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } bb = 3; continue; }
            3 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru16(rbx.wrapping_add(0xd0u64))) as u32) as u32 as u64; rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu16(r14.wrapping_add(0x14u64), ((rax as u16)) as u16); r15 = (0x400000000000u64) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rsi = rbx.wrapping_add(0x28); bb = 7; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; rsi = rbx.wrapping_add(0x28); rdx = 0x2e; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = 0x2e; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm6) as u128); rcx = rsi; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm7) as u128); xmm6 = 0u128; xmm7 = 0u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = 0x2e; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm6 = (xmm0) as u128; bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(P2)) as u64; rdx = rbx.wrapping_add(0x48); r9 = rbp; r8 = 0x2e; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 12; continue; }
            12 => { crate::simd_ops::lk_and_u8(r14.wrapping_add(0x14u64), (0xbfu8) as u8); rax = ((rdi as u32)) as u32 as u64; fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm7 as u32)) as u32 as u64; rcx = 0x40; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; rax = (if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { (rcx as u32) } else { (rax as u32) }) as u32 as u64; fl_l = (crate::simd_ops::lk_or_u8(r14.wrapping_add(0x14u64), ((rax as u8)) as u8)) as u8 as u64; fl_r = fl_l; bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; rsi = 0xfffffffc; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_l = (((rax) >> (23u32)) & 1) as u64; if ((fl_l as u64) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(P2)) as u64; rdx = rbx.wrapping_add(0x48); r9 = rbp; r8 = 0x17; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(r14, ((xmm0 as u32)) as u32); bb = 22; continue; }
            16 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), (0x42c80000u32) as u32); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 17; continue; }
            17 => { r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((rdi as u32)) as u32 as u64; bb = 18; continue; }
            18 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0x47u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 27; continue; } bb = 19; continue; }
            19 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 18; continue; } bb = 20; continue; }
            20 => { rdx = rdi; bb = 21; continue; }
            21 => { fl_l = rdx; fl_r = rdx; rax = rsp.wrapping_add(0x60); rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rdx } else { rax }) as u64; rax = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(r14, ((rax as u32)) as u32); bb = 22; continue; }
            22 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x90u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rdi as u32)) as u32); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 23; continue; }
            23 => { r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((rdi as u32)) as u32 as u64; bb = 24; continue; }
            24 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0x46u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 28; continue; } bb = 25; continue; }
            25 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 24; continue; } bb = 26; continue; }
            26 => { bb = 29; continue; }
            27 => { rax = ((r9.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = (((rax) & (rsi))) as u64; fl_l = ((rax).wrapping_add(r8)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; bb = 21; continue; }
            28 => { rax = ((r9.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = (((rax) & (rsi))) as u64; fl_l = ((rax).wrapping_add(r8)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; rdi = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; bb = 29; continue; }
            29 => { fl_l = rdi; fl_r = rdi; rax = rsp.wrapping_add(0x60); rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rdi } else { rax }) as u64; rax = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x4u64), ((rax as u32)) as u32); rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159f4e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14159f4e0; const P1: u64 = 0x143426e64; const P2: u64 = 0x143814f8c; const P3: u64 = 0x14159b8a0; const P4: u64 = 0x143930fa0; const P5: u64 = 0x14153ad70; const P6: u64 = 0x143814f90;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x78u64)) as u64; rbx = rcx; rsi = rdx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x78u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x98u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rsi; rsp = ((rsp).wrapping_add(0x78u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { xmm0 = ((crate::simd_ops::ru32(P1)) as u32) as u128; rax = (crate::simd_ops::ru32(P2)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rbp); rbp = 0xfffffffc; crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), rdi); rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r8); r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm6) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm7) as u128); xmm7 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0xa8u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0xacu64), (0x0u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rdi as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rdi as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0xffu8) as u8); crate::simd_ops::wu8(rsp.wrapping_add(0x40u64), (0xffu8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); crate::simd_ops::wu32(rsp.wrapping_add(0x90u64), ((rax as u32)) as u32); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((rdi as u32)) as u32 as u64; bb = 5; continue; }
            5 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0x7u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 21; continue; } bb = 6; continue; }
            6 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 7; continue; }
            7 => { rdx = rdi; bb = 8; continue; }
            8 => { fl_l = rdx; fl_r = rdx; rax = rsp.wrapping_add(0x90); rcx = rbx.wrapping_add(0x28); xmm6 = (xmm7) as u128; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rdx } else { rax }) as u64; rdx = 0x11; rax = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x90u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = 0x11; rcx = rbx.wrapping_add(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm6 = (xmm0) as u128; bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_l = (((rax) >> (17u32)) & 1) as u64; if ((fl_l as u64) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(P4)) as u64; rdx = rbx.wrapping_add(0x48); r9 = rsp.wrapping_add(0x20); r8 = 0x11; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 13; continue; }
            13 => { xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x90u64)))).to_bits() as u128); rax = ((xmm6 as u32)) as u32 as u64; fl_l = ((((rax as u32)) >> (31u32)) & 1) as u64; rax = (((rax as u32)) & !((1u32) << (31u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7f800000; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 15; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu32(rsp.wrapping_add(0xa8u64), ((xmm6 as u32)) as u32); bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0xd4u64))) as u8 as u64; fl_r = (0x20u8) as u8 as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 24; continue; } bb = 16; continue; }
            16 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x90u64))) as u64; rax = (crate::simd_ops::ru32(P6)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x90u64), ((rax as u32)) as u32); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 17; continue; }
            17 => { r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((rdi as u32)) as u32 as u64; bb = 18; continue; }
            18 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0x8u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 22; continue; } bb = 19; continue; }
            19 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 18; continue; } bb = 20; continue; }
            20 => { bb = 23; continue; }
            21 => { rax = ((r9.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = (((rax) & (rbp))) as u64; fl_l = ((rax).wrapping_add(r8)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; bb = 8; continue; }
            22 => { rax = ((r9.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = (((rax) & (rbp))) as u64; fl_l = ((rax).wrapping_add(r8)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; rdi = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; bb = 23; continue; }
            23 => { fl_l = rdi; fl_r = rdi; rax = rsp.wrapping_add(0x90); rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rdi } else { rax }) as u64; xmm7 = ((crate::simd_ops::ru32(rax)) as u32) as u128; bb = 24; continue; }
            24 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rax = ((xmm7 as u32)) as u32 as u64; fl_l = ((((rax as u32)) >> (31u32)) & 1) as u64; rax = (((rax as u32)) & !((1u32) << (31u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7f800000; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 26; continue; } bb = 25; continue; }
            25 => { crate::simd_ops::wu32(rsp.wrapping_add(0xacu64), ((xmm7 as u32)) as u32); bb = 26; continue; }
            26 => { xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64) as u128; rax = rsi; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; crate::simd_ops::wu64(rsi, ((xmm0 as u64)) as u64); rsp = ((rsp).wrapping_add(0x78u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159f700(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140000000; const P1: u64 = 0x143930f44; const P2: u64 = 0x1415317a0; const P3: u64 = 0x14159e060;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0xa8u64)) as u64; r8 = (crate::simd_ops::ru32(rcx.wrapping_add(0x20u64))) as u32 as u64; rdi = rdx; r12 = ((crate::simd_ops::ru8(rcx.wrapping_add(0xd4u64))) as u32) as u32 as u64; r13 = rcx; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x90u64))) as u64; r12 = (r12 & !0xffu64) | (((((r12 as u8)).wrapping_shr(7u32)) as u8 as u64)); crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm9) as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x0u32) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x2cu64), (0x1u8) as u8); crate::simd_ops::wu8(rsp.wrapping_add(0xf8u64), ((r12 as u8)) as u8); crate::simd_ops::wu8(rsp.wrapping_add(0x2du64), ((r12 as u8)) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r8 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r8 as u32)) as u32); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rbp = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; r14 = rax.wrapping_add(0x1); rbp = ((rbp).wrapping_add(0x4u64)) as u64; rcx = 0xfffffffc; rbp = (((rbp) & (rcx))) as u64; fl_l = ((rbp).wrapping_add(rax)) as u64; fl_r = fl_l; rbp = ((fl_l as u64)) as u64; bb = 3; continue; }
            2 => { r14 = 0; fl_l = 0; fl_r = 0; rbp = 0; fl_l = 0; fl_r = 0; bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0xe8u64), rbx); r15 = P0; crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), rsi); xmm9 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x90u64), (xmm6) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x80u64), (xmm7) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm8) as u128); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(r13.wrapping_add(0x90u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; rdx = ((rdx).wrapping_add(1)) as u64; fl_l = ((rdx).wrapping_add(rcx)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 7; continue; }
            6 => { rdx = 0; fl_l = 0; fl_r = 0; bb = 7; continue; }
            7 => { fl_l = r14; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 29; continue; } bb = 8; continue; }
            8 => { rdx = ((crate::simd_ops::ru8(r14)) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x4a; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 28; continue; } bb = 9; continue; }
            9 => { rax = ((rdx as u32)) as u32 as u64; rsi = ((rdx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffe0u32))) as u32 as u64; rsi = ((rsi).wrapping_shr(5u32)) as u64; rcx = ((rdx as u32)) as u32 as u64; rbx = 0x1; rcx = (((rcx as u32)).wrapping_sub((rax as u32))) as u32 as u64; rbx = ((rbx).wrapping_shl(((rcx as u32) & 63))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0x90u64))) as u32 as u64; fl_l = rbx; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 10; continue; }
            10 => { r15 = (crate::simd_ops::ru32(r15.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(0x2689180u64))) as u32 as u64; xmm7 = ((crate::simd_ops::ru32(rbp)) as u32) as u128; fl_l = ((r15 as u32)) as u32 as u64; fl_r = 0x1a; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 27; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru32(P1)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 26; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0x9cu64))) as u32 as u64; fl_l = rbx; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 13; continue; }
            13 => { fl_l = ((xmm7 as u32)) as u32 as u64; fl_r = ((xmm9 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 27; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x0u32) as u32); rcx = rdi.wrapping_add(0x68); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((rdx as u8)) as u8); xmm8 = (xmm9) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((r8 as u32)) as u32); rdx = rsp.wrapping_add(0x48); crate::simd_ops::wu8(rsp.wrapping_add(0x39u64), (0x1u8) as u8); r8 = rsp.wrapping_add(0xe0); crate::simd_ops::wu8(rsp.wrapping_add(0x3au64), ((r12 as u8)) as u8); xmm6 = (xmm9) as u128; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x38u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0xe0u64), (0x0u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x4cu64), (0x0u32) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0xe0u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 17; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu32(rax.wrapping_add(0xcu64), (0x0u32) as u32); bb = 17; continue; }
            17 => { xmm8 = ((crate::simd_ops::ru32(rax.wrapping_add(0xcu64))) as u32) as u128; xmm6 = (xmm8) as u128; xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(xmm7 as u32)).to_bits() as u128); crate::simd_ops::wu32(rax.wrapping_add(0xcu64), ((xmm6 as u32)) as u32); bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0xa8u64))) as u32 as u64; fl_l = rbx; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::lk_or_u8(rdi.wrapping_add(0xb7u64), (0x4u8) as u8); r15 = P0; r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; r14 = ((r14).wrapping_add(1)) as u64; fl_l = ((rbp).wrapping_add(0x4u64)) as u64; fl_r = fl_l; rbp = ((fl_l as u64)) as u64; bb = 4; continue; }
            20 => { xmm0 = ((crate::simd_ops::ru32(rdi.wrapping_add(r15.wrapping_mul(4)))) as u32) as u128; fl_l = ((xmm8 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 23; continue; } bb = 21; continue; }
            21 => { fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 24; continue; } bb = 22; continue; }
            22 => { crate::simd_ops::lk_or_u8(rdi.wrapping_add(0xb7u64), (0x4u8) as u8); r15 = P0; r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; r14 = ((r14).wrapping_add(1)) as u64; fl_l = ((rbp).wrapping_add(0x4u64)) as u64; fl_r = fl_l; rbp = ((fl_l as u64)) as u64; bb = 4; continue; }
            23 => { fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; bb = 24; continue; }
            24 => { r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 27; continue; } bb = 25; continue; }
            25 => { r14 = ((r14).wrapping_add(1)) as u64; crate::simd_ops::wu32(rdi.wrapping_add(r15.wrapping_mul(4)), ((xmm6 as u32)) as u32); fl_l = ((rbp).wrapping_add(0x4u64)) as u64; fl_r = fl_l; rbp = ((fl_l as u64)) as u64; r15 = P0; bb = 4; continue; }
            26 => { xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) + f32::from_bits(crate::simd_ops::ru32(rdi.wrapping_add(r15.wrapping_mul(4))))).to_bits() as u128); crate::simd_ops::wu32(rdi.wrapping_add(r15.wrapping_mul(4)), ((xmm7 as u32)) as u32); bb = 27; continue; }
            27 => { r15 = P0; bb = 28; continue; }
            28 => { r14 = ((r14).wrapping_add(1)) as u64; fl_l = ((rbp).wrapping_add(0x4u64)) as u64; fl_r = fl_l; rbp = ((fl_l as u64)) as u64; bb = 4; continue; }
            29 => { fl_l = (crate::simd_ops::ru32(r13.wrapping_add(0x50u64))) as u32 as u64; fl_r = 0x0; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 31; continue; } bb = 30; continue; }
            30 => { rdx = r13.wrapping_add(0x48); r9 = rdi; r8 = rsp.wrapping_add(0x28); rcx = r12; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x2du64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0xf8u64), ((rax as u8)) as u8); bb = 31; continue; }
            31 => { fl_l = (crate::simd_ops::ru8(r13.wrapping_add(0x30u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 64; continue; } bb = 32; continue; }
            32 => { rax = (crate::simd_ops::ru64(r13.wrapping_add(0x28u64))) as u64; rcx = r13.wrapping_add(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x78u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 64; continue; } bb = 33; continue; }
            33 => { rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 64; continue; } bb = 34; continue; }
            34 => { rdx = 0xfffffff8; bb = 35; continue; }
            35 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; r13 = (crate::simd_ops::ru32(rbx.wrapping_add(0x38u64))) as u32 as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 37; continue; } bb = 36; continue; }
            36 => { rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64; r14 = rcx.wrapping_add(0x2); r12 = ((rax.wrapping_mul(2).wrapping_add(0x9u64)) as u64) as u64; r12 = (((r12) & (rdx))) as u64; fl_l = ((r12).wrapping_add(rcx)) as u64; fl_r = fl_l; r12 = ((fl_l as u64)) as u64; bb = 38; continue; }
            37 => { r14 = 0; fl_l = 0; fl_r = 0; r12 = 0; fl_l = 0; fl_r = 0; bb = 38; continue; }
            38 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 40; continue; } bb = 39; continue; }
            39 => { rdx = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64; fl_l = ((rdx).wrapping_add(1)) as u64; rdx = ((fl_l as u64)) as u64; rdx = ((rcx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; bb = 41; continue; }
            40 => { rdx = 0; fl_l = 0; fl_r = 0; bb = 41; continue; }
            41 => { fl_l = r14; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 63; continue; } bb = 42; continue; }
            42 => { rax = ((crate::simd_ops::ru16(r14)) as u32) as u32 as u64; rdx = (((crate::simd_ops::ru32(r15.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x26895f0u64))) as i32 as i64) as u64) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x4a; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 62; continue; } bb = 43; continue; }
            43 => { rax = ((rdx as u32)) as u32 as u64; rbp = ((rdx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffe0u32))) as u32 as u64; rbp = ((rbp).wrapping_shr(5u32)) as u64; rcx = ((rdx as u32)) as u32 as u64; rsi = 0x1; rcx = (((rcx as u32)).wrapping_sub((rax as u32))) as u32 as u64; rsi = ((rsi).wrapping_shl(((rcx as u32) & 63))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(rbp.wrapping_mul(4)).wrapping_add(0x90u64))) as u32 as u64; fl_l = rsi; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 62; continue; } bb = 44; continue; }
            44 => { r15 = (crate::simd_ops::ru32(r15.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(0x2689180u64))) as u32 as u64; xmm7 = ((crate::simd_ops::ru32(r12)) as u32) as u128; fl_l = ((r15 as u32)) as u32 as u64; fl_r = 0x1a; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 61; continue; } bb = 45; continue; }
            45 => { fl_l = (crate::simd_ops::ru32(P1)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 60; continue; } bb = 46; continue; }
            46 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(rbp.wrapping_mul(4)).wrapping_add(0x9cu64))) as u32 as u64; fl_l = rsi; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 60; continue; } bb = 47; continue; }
            47 => { fl_l = ((xmm7 as u32)) as u32 as u64; fl_r = ((xmm9 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 61; continue; } bb = 48; continue; }
            48 => { rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0xf8u64))) as u32) as u32 as u64; rcx = rdi.wrapping_add(0x68); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x0u32) as u32); r8 = rsp.wrapping_add(0xe0); crate::simd_ops::wu8(rsp.wrapping_add(0x3au64), ((rax as u8)) as u8); xmm8 = (xmm9) as u128; crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((rdx as u8)) as u8); xmm6 = (xmm9) as u128; crate::simd_ops::wu8(rsp.wrapping_add(0x39u64), (0x3u8) as u8); rdx = rsp.wrapping_add(0x48); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x38u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0xe0u64), (0x0u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x4cu64), ((r13 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 52; continue; } bb = 49; continue; }
            49 => { fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0xe0u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 51; continue; } bb = 50; continue; }
            50 => { crate::simd_ops::wu32(rax.wrapping_add(0xcu64), (0x0u32) as u32); bb = 51; continue; }
            51 => { xmm8 = ((crate::simd_ops::ru32(rax.wrapping_add(0xcu64))) as u32) as u128; xmm6 = (xmm7) as u128; xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(xmm8 as u32)).to_bits() as u128); crate::simd_ops::wu32(rax.wrapping_add(0xcu64), ((xmm6 as u32)) as u32); bb = 52; continue; }
            52 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(rbp.wrapping_mul(4)).wrapping_add(0xa8u64))) as u32 as u64; fl_l = rsi; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 54; continue; } bb = 53; continue; }
            53 => { crate::simd_ops::lk_or_u8(rdi.wrapping_add(0xb7u64), (0x4u8) as u8); r15 = P0; r14 = ((r14).wrapping_add(0x2u64)) as u64; fl_l = ((r12).wrapping_add(0x10u64)) as u64; fl_r = fl_l; r12 = ((fl_l as u64)) as u64; bb = 38; continue; }
            54 => { xmm0 = ((crate::simd_ops::ru32(rdi.wrapping_add(r15.wrapping_mul(4)))) as u32) as u128; fl_l = ((xmm8 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 57; continue; } bb = 55; continue; }
            55 => { fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 58; continue; } bb = 56; continue; }
            56 => { crate::simd_ops::lk_or_u8(rdi.wrapping_add(0xb7u64), (0x4u8) as u8); r15 = P0; r14 = ((r14).wrapping_add(0x2u64)) as u64; fl_l = ((r12).wrapping_add(0x10u64)) as u64; fl_r = fl_l; r12 = ((fl_l as u64)) as u64; bb = 38; continue; }
            57 => { fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; bb = 58; continue; }
            58 => { if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 61; continue; } bb = 59; continue; }
            59 => { r14 = ((r14).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu32(rdi.wrapping_add(r15.wrapping_mul(4)), ((xmm6 as u32)) as u32); fl_l = ((r12).wrapping_add(0x10u64)) as u64; fl_r = fl_l; r12 = ((fl_l as u64)) as u64; r15 = P0; bb = 38; continue; }
            60 => { xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) + f32::from_bits(crate::simd_ops::ru32(rdi.wrapping_add(r15.wrapping_mul(4))))).to_bits() as u128); crate::simd_ops::wu32(rdi.wrapping_add(r15.wrapping_mul(4)), ((xmm7 as u32)) as u32); bb = 61; continue; }
            61 => { r15 = P0; bb = 62; continue; }
            62 => { r14 = ((r14).wrapping_add(0x2u64)) as u64; fl_l = ((r12).wrapping_add(0x10u64)) as u64; fl_r = fl_l; r12 = ((fl_l as u64)) as u64; bb = 38; continue; }
            63 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdx = 0xfffffff8; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 35; continue; } bb = 64; continue; }
            64 => { xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x80u64))) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x90u64))) as u128; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xe8u64))) as u64; xmm9 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; rsp = ((rsp).wrapping_add(0xa8u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159fbf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415bd700; const P1: u64 = 0x80; const P2: u64 = 0x1415ca3f0; const P3: u64 = 0x78; const P4: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rdi = (((r9 as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(0x28u64)) as u64; rsi = r8; rbp = ((rdx as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { r8 = rsi; rdx = ((rbp as u32)) as u32 as u64; rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { rcx = ((rbx.wrapping_add(P4)) as u64) as u64; r8 = rsi; rdx = ((rbp as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 7; continue; }
            7 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            9 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 8; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14159fc80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f88; const P1: u64 = 0x1426452b0; const P2: u64 = 0x1426452a8; const P3: u64 = 0x142645560; const P4: u64 = 0x1434264e4;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = ((rsp).wrapping_sub(0x48u64)) as u64; xmm0 = 0u128; rax = rcx; crate::simd_ops::wu128(rdx, (xmm0) as u128); r13 = rcx; rcx = 0; fl_l = 0; fl_r = 0; rbp = rdx; crate::simd_ops::wu128(rdx.wrapping_add(0x10u64), (xmm0) as u128); crate::simd_ops::wu128(rdx.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::wu64(rdx.wrapping_add(0x30u64), rcx); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 66; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rbx); rcx = (0x20020000ffeu64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r15); bb = 2; continue; }
            2 => { r15 = rax; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x78u64))) as u64; fl_l = (crate::simd_ops::ru64(r15.wrapping_add(0x98u64))) as u64; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = ((crate::simd_ops::ru8(r15.wrapping_add(0xd5u64))) as u32) as u32 as u64; r12 = 0; fl_l = 0; fl_r = 0; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shr(2u32)) as u8 as u64)); r10 = 0xfffffffc; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r12 as u32)) as u32); crate::simd_ops::wu8(rdx.wrapping_add(0x4u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r13.wrapping_add(0xd0u64))) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); crate::simd_ops::wu8(rdx.wrapping_add(0x5u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r13.wrapping_add(0xd0u64))) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shr(7u32)) as u8 as u64)); crate::simd_ops::wu8(rdx.wrapping_add(0x6u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r13.wrapping_add(0xd0u64))) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shr(3u32)) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); crate::simd_ops::wu8(rdx.wrapping_add(0x7u64), ((rax as u8)) as u8); r8 = (crate::simd_ops::ru64(r13.wrapping_add(0x90u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((r12 as u32)) as u32 as u64; bb = 6; continue; }
            6 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0x46u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 20; continue; } bb = 7; continue; }
            7 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 6; continue; } bb = 8; continue; }
            8 => { rdx = r12; bb = 9; continue; }
            9 => { rdi = (crate::simd_ops::ru64(P0)) as u64; rax = rsp.wrapping_add(0x20); fl_l = rdx; fl_r = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rsi); rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rdx } else { rax }) as u64; r14 = (crate::simd_ops::ru32(rax)) as u32 as u64; bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x210u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 21; continue; } bb = 11; continue; }
            11 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rdi.wrapping_add(0x210u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 12; continue; }
            12 => { rsi = rdi.wrapping_add(0x218); bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x258u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 23; continue; } bb = 14; continue; }
            14 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r14 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x250u64))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 18; continue; } bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 15; continue; } bb = 17; continue; }
            17 => { rbx = r12; bb = 18; continue; }
            18 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 19; continue; }
            19 => { fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 24; continue; }
            20 => { rax = ((r9.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = (((rax) & (r10))) as u64; fl_l = ((rax).wrapping_add(r8)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; bb = 9; continue; }
            21 => { rsi = rdi.wrapping_add(0x218); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x210u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 13; continue; } bb = 22; continue; }
            22 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            23 => { rbx = r12; bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x210u64))) as u32 as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 32; continue; } bb = 25; continue; }
            25 => { bb = 26; continue; }
            26 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rdi.wrapping_add(0x210u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 29; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x210u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 26; continue; } bb = 28; continue; }
            28 => { bb = 33; continue; }
            29 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 34; continue; } bb = 30; continue; }
            30 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x240u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 34; continue; } bb = 31; continue; }
            31 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x248u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 34; continue; }
            32 => { rsi = rdi.wrapping_add(0x218); bb = 33; continue; }
            33 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 34; continue; }
            34 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 59; continue; } bb = 35; continue; }
            35 => { rax = ((crate::simd_ops::ru8(r13.wrapping_add(0xd0u64))) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shr(6u32)) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x1u8))) as u8 as u64)); crate::simd_ops::wu8(rbp.wrapping_add(0x8u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x20bu64))) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x1u8))) as u8 as u64)); crate::simd_ops::wu8(rbp.wrapping_add(0x9u64), ((rax as u8)) as u8); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x20bu64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 37; continue; } bb = 36; continue; }
            36 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0xcu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x14u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x24u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x18u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x28u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x1cu64), ((rax as u32)) as u32); bb = 37; continue; }
            37 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x1f8u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 41; continue; } bb = 38; continue; }
            38 => { rax = ((rax).wrapping_add(0x2u64)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = (crate::simd_ops::ru64(rbx.wrapping_add(rax.wrapping_mul(8)))) as u64; rdx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 41; continue; } bb = 39; continue; }
            39 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 41; continue; } bb = 40; continue; }
            40 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(0x2u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru32(r8.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x24u64), ((rcx as u32)) as u32); bb = 41; continue; }
            41 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x1f9u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 45; continue; } bb = 42; continue; }
            42 => { rax = ((rax).wrapping_add(0x2u64)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(rax.wrapping_mul(8)))) as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 45; continue; } bb = 43; continue; }
            43 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 45; continue; } bb = 44; continue; }
            44 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x28u64), ((rax as u32)) as u32); bb = 45; continue; }
            45 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x1fau64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 49; continue; } bb = 46; continue; }
            46 => { rax = ((rax).wrapping_add(0x2u64)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(rax.wrapping_mul(8)))) as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 49; continue; } bb = 47; continue; }
            47 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 49; continue; } bb = 48; continue; }
            48 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x2cu64), ((rax as u32)) as u32); bb = 49; continue; }
            49 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x1fbu64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 53; continue; } bb = 50; continue; }
            50 => { rax = ((rax).wrapping_add(0x2u64)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(rax.wrapping_mul(8)))) as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 53; continue; } bb = 51; continue; }
            51 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 53; continue; } bb = 52; continue; }
            52 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x30u64), ((rax as u32)) as u32); bb = 53; continue; }
            53 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x1fcu64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 57; continue; } bb = 54; continue; }
            54 => { rax = ((rax).wrapping_add(0x2u64)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = (crate::simd_ops::ru64(rbx.wrapping_add(rax.wrapping_mul(8)))) as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 57; continue; } bb = 55; continue; }
            55 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 57; continue; } bb = 56; continue; }
            56 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru32(r8.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x34u64), ((rax as u32)) as u32); bb = 57; continue; }
            57 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 59; continue; } bb = 58; continue; }
            58 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 59; continue; }
            59 => { r8 = (crate::simd_ops::ru64(r15.wrapping_add(0x90u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((r12 as u32)) as u32); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 65; continue; } bb = 60; continue; }
            60 => { r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((r12 as u32)) as u32 as u64; bb = 61; continue; }
            61 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0xeu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 64; continue; } bb = 62; continue; }
            62 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 61; continue; } bb = 63; continue; }
            63 => { bb = 65; continue; }
            64 => { rax = ((r9.wrapping_add(0x4u64)) as u32) as u32 as u64; rdx = 0xfffffffc; rax = (((rax) & (rdx))) as u64; fl_l = ((rax).wrapping_add(r8)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; r12 = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; bb = 65; continue; }
            65 => { fl_l = r12; fl_r = r12; rax = rsp.wrapping_add(0x60); rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { r12 } else { rax }) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rax as u32)) as u32); rax = 0x1; xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P4))).to_bits() as u128); crate::simd_ops::wu32(rbp, ((xmm0 as u32)) as u32); rsp = ((rsp).wrapping_add(0x48u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            66 => { rax = 0xf; rsp = ((rsp).wrapping_add(0x48u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415a00a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu64(rdx, (0x10000u64) as u64);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415a00b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524340; const P1: u64 = 0x141524330; const P2: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rdi = rcx; rbp = ((rdx as u32)) as u32 as u64; rcx = 0x1; rbp = (((rbp as u32)).wrapping_shr(1u32)) as u32 as u64; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rsi = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rbp = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rcx as u32) } else { (rbp as u32) }) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; rdx = ((rbp as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = rdx; rdx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 2; continue; }
            2 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 13; continue; }
            3 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 5; continue; }
            5 => { fl_l = rax; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 6; continue; }
            6 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { rax = rbx; r8 = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rax, r8); crate::simd_ops::wu32(rax.wrapping_add(0x8u64), ((r8 as u32)) as u32); bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rdx = rax; rdx = ((rdx).wrapping_sub(rbx)) as u64; xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(rcx))) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); rax = ((rax).wrapping_add(0x10u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rdi, rbx); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((rbp as u32)) as u32); bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415a0190(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14159dd20; const P1: u64 = 0x70; const P2: u64 = 0xd4; const P3: u64 = 0xd6; const P4: u64 = 0x80; const P5: u64 = 0x118; const P6: u64 = 0x78;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (((rdx as u16)) as u32) as u32 as u64; rsi = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = (((rax as u8)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(P1))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::lk_add_u16(rax.wrapping_add(P2), (1) as u16); fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u16(rax.wrapping_add(P3), (1) as u16)) as u16 as u64; bb = 3; continue; }
            3 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P4))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = 0xfffe; rdx = 0x1; rdi = (rdi & !0xffffu64) | ((((((rdi as u16)) & ((rax as u16)))) as u16 as u64)); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((((rbx as u8)) & ((rax as u8)))) as u8 as u64; fl_r = fl_l; rbx = (rbx & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P6))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { r8 = (crate::simd_ops::ru64(rcx)) as u64; rdx = (((rdi as u16)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r8.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (rax & !0xffu64) | ((((((rax as u8)) & ((rbx as u8)))) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            8 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = (((rbx as u8)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415a0230(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14159dd20; const P1: u64 = 0x70; const P2: u64 = 0xd4; const P3: u64 = 0xd6;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = (((rdx as u8)) as u32) as u32 as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    fl_l = rcx; fl_r = rcx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    crate::simd_ops::lk_add_u16(rcx.wrapping_add(P2), (1) as u16);
    fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { fl_l = (crate::simd_ops::lk_add_u16(rcx.wrapping_add(P3), (1) as u16)) as u16 as u64; }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415a0270(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14159d7b0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); crate::simd_ops::wu32(rsp.wrapping_add(0x18u64), ((r8 as u32)) as u32); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rbx = ((r8 as u32)) as u32 as u64; r14 = rdx; r15 = rcx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { rdi = 0; fl_l = 0; fl_r = 0; rbp = ((((rbx as u16)) as i16 as i32) as u32) as u32 as u64; rsi = (((rdi as u16)) as u32) as u32 as u64; r12 = ((rdi.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(r14)) as u64; rdx = ((r12 as u32)) as u32 as u64; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x118u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 4; continue; }
            4 => { rsi = (rsi & !0xffffu64) | (((((rsi as u16)).wrapping_add(1)) as u16 as u64)); rax = (((rsi as u16)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 3; continue; } bb = 5; continue; }
            5 => { rsi = (((crate::simd_ops::ru16(rsp.wrapping_add(0x92u64))) as i16 as i32) as u32) as u32 as u64; xmm0 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), ((r12 as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x49u64), ((r12 as u8)) as u8); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(r14)) as u64; rdx = rsp.wrapping_add(0x20); r9 = (((r12 as u8)) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x48u64), (0x0u8) as u8); r8 = (((r12 as u8)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r15); rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x108u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { rdi = (rdi & !0xffffu64) | (((((rdi as u16)).wrapping_add(1)) as u16 as u64)); rax = (((rdi as u16)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 7; continue; } bb = 9; continue; }
            9 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 10; continue; }
            10 => { r11 = rsp.wrapping_add(0x50); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x48u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            11 => { crate::simd_ops::wu16(rsp.wrapping_add(0x88u64), ((rsi as u16)) as u16); rdx = r14; crate::simd_ops::wu16(rsp.wrapping_add(0x8au64), ((rdi as u16)) as u16); rcx = r15; r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x88u64))) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 10; continue; }
            12 => { crate::simd_ops::wu16(rsp.wrapping_add(0x88u64), ((rbx as u16)) as u16); rdx = r14; crate::simd_ops::wu16(rsp.wrapping_add(0x8au64), ((rdi as u16)) as u16); rcx = r15; r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x88u64))) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 10; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415a03a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70; const P1: u64 = 0xd6; const P2: u64 = 0xd2; const P3: u64 = 0x80; const P4: u64 = 0x78; const P5: u64 = 0x1415a0270;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u16(rax.wrapping_add(P1), ((rdx as u16)) as u16)) as u16 as u64; fl_r = fl_l; bb = 3; continue; }
            3 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = ((rdx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(16u32)) as u32 as u64; fl_l = (crate::simd_ops::lk_add_u16(r8.wrapping_add(P2), ((rax as u16)) as u16)) as u16 as u64; fl_r = fl_l; bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } bb = 7; continue; }
            7 => { return rax; }
            8 => { r8 = ((rdx as u32)) as u32 as u64; rdx = rax; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415a03f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14159dd20; const P1: u64 = 0x1415a00b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = r8; rsi = (((rdx as u8)) as u32) as u32 as u64; rbx = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x70u64))) as u64; fl_l = 0; fl_r = ((rax as u8)) as u8 as u64; rax = (rax & !0xffu64) | (((0u8.wrapping_sub(fl_r as u8)) as u8 as u64)); rax = (if (((fl_l as u8)) < ((fl_r as u8))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64; rax = (((rax as u32)).wrapping_add(0x2u32)) as u32 as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::lk_add_u16(rcx.wrapping_add(0xd0u64), (1) as u16); fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u16(rcx.wrapping_add(0xd2u64), (1) as u16)) as u16 as u64; bb = 3; continue; }
            3 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 4; continue; }
            4 => { rsi = (crate::simd_ops::ru32(rcx.wrapping_add(0xc8u64))) as u32 as u64; rbx = rcx.wrapping_add(0xc0); rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = ((rsi as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 6; continue; }
            6 => { rax = ((rax).wrapping_add(0x10u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 7; continue; }
            7 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 17; continue; } bb = 9; continue; }
            9 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 17; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); fl_l = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx, rdx); crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((rdx as u32)) as u32); bb = 12; continue; }
            12 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rcx, rdi); crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), (0x1u32) as u32); bb = 14; continue; }
            14 => { rax = 0x1; bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            16 => { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 14; continue; }
            17 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0x34; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415a04e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415ae930; const P1: u64 = 0x1415aed50; const P2: u64 = 0x14159eda0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x80u64))) as u32) as u32 as u64; rsi = 0; fl_l = 0; fl_r = 0; r14 = rdx; crate::simd_ops::wu8(rcx.wrapping_add(0x82u64), ((rax as u8)) as u8); rdi = rcx; rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x88u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(0x83u64), ((rax as u8)) as u8); crate::simd_ops::wu16(rcx.wrapping_add(0x80u64), ((r9 as u16)) as u16); fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x40u64))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rsi as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r8); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rsi as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0xffu8) as u8); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu8(rsp.wrapping_add(0x40u64), (0xffu8) as u8); crate::simd_ops::wu128(rcx.wrapping_add(0x8u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(0x18u64), (xmm1) as u128); crate::simd_ops::wu128(rcx.wrapping_add(0x28u64), (xmm0) as u128); bb = 2; continue; }
            2 => { fl_l = ((r9 as u16)) as u16 as u64; fl_r = ((r9 as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rsp.wrapping_add(0x60); rcx = rdi; rbx = (((rax as u8)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (((rbx as u8)) as u32) as u32 as u64; rdx = r14; r8 = rax; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(r14)) as u64; r8 = rsp.wrapping_add(0x60); rdx = rdi; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (0x10000u64) as u64); rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x210u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = r14; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { rbx = (((crate::simd_ops::ru8(r14.wrapping_add(0xd4u64))) as i8 as i32) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x78u64))) as u64; rbx = (((((rbx as u32)) as i32).wrapping_shr(7u32) as u32)) as u32 as u64; rbx = (((rbx as u32)).wrapping_add(0x2u32)) as u32 as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru32(r14.wrapping_add(0x20u64))) as u32 as u64; rcx = ((((rbx as u32)) as i32 as i64) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rcx = ((rcx).wrapping_shl(29u32)) as u64; rax = ((rsi as u32)) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rcx = (((rcx) | (rax))) as u64; rcx = ((rcx).wrapping_shl(32u32)) as u64; rcx = (((rcx) | (rdx))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x90u64), rcx); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415a0610(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xd4; const P1: u64 = 0x20; const P2: u64 = 0x8; const P3: u64 = 0x4;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r8 = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64; r9 = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; r8 = (r8 & !0xffu64) | (((((r8 as u8)).wrapping_shr(7u32)) as u8 as u64)); rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rax)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = ((r8 as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rax.wrapping_add(P3))) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = ((rax).wrapping_add(0x8u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { rdx = 0; fl_l = 0; fl_r = 0; fl_l = rax; fl_r = rcx; rdx = (if (((fl_l as u64)) != ((fl_r as u64))) { rax } else { rdx }) as u64; fl_l = rdx; fl_r = rdx; rax = (rax & !0xffu64) | (((if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415a0660(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x78; const P2: u64 = 0x240;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = 0;
    fl_l = 0; fl_r = 0;
    rax = ((rcx.wrapping_add(P0)) as u64) as u64;
    fl_l = rcx; fl_r = rcx;
    rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r8 } else { rax }) as u64;
    fl_l = rax; fl_r = rdx;
    if (((fl_l as u64)) == ((fl_r as u64))) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; return crate::simd_ops::call_fwd(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415a0690(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xd5; const P1: u64 = 0x78; const P2: u64 = 0xd6;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x20u8) as u8 as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rax; rax = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x20u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P2))) as u32) as u32 as u64; fl_l = ((((rax as u8)) & (0x1u8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415a06c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xd5; const P1: u64 = 0x78; const P2: u64 = 0x90; const P3: u64 = 0x143815024; const P4: u64 = 0x1; const P5: u64 = 0x4;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x40u8) as u8 as u64; rbx = rdx; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; r8 = rcx; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { r8 = rax; rax = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; fl_l = (crate::simd_ops::ru8(r8.wrapping_add(P0))) as u8 as u64; fl_r = (0x40u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 1; continue; } bb = 3; continue; }
            3 => { r9 = (crate::simd_ops::ru64(r8.wrapping_add(P2))) as u64; r11 = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru32(P3)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x8u64), ((rax as u32)) as u32); fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 4; continue; }
            4 => { r10 = ((crate::simd_ops::ru8(r9)) as u32) as u32 as u64; rcx = ((r11 as u32)) as u32 as u64; bb = 5; continue; }
            5 => { rdx = ((rcx.wrapping_add(P4)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r9))) as u8 as u64; fl_r = (0x2du8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 7; continue; }
            7 => { bb = 9; continue; }
            8 => { rax = ((r10.wrapping_add(P5)) as u32) as u32 as u64; rdx = 0xfffffffc; rax = (((rax) & (rdx))) as u64; fl_l = ((rax).wrapping_add(r9)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; r11 = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; bb = 9; continue; }
            9 => { fl_l = r11; fl_r = r11; rax = rsp.wrapping_add(0x8); rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { r11 } else { rax }) as u64; rax = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(r8.wrapping_add(P0))) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; fl_l = (((rax as u8)).wrapping_shr(7u32)) as u8 as u64; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415a0760(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70; const P1: u64 = 0xa8; const P2: u64 = 0x14160e090; const P3: u64 = 0xb8; const P4: u64 = 0x14160eaa0; const P5: u64 = 0xf0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdi = ((r8 as u32)) as u32 as u64; rsi = ((rdx as u32)) as u32 as u64; rbx = rcx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rcx = ((rbx.wrapping_add(P1)) as u64) as u64; r8 = ((rdi as u32)) as u32 as u64; rdx = ((rsi as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = ((rbx.wrapping_add(P3)) as u64) as u64; r8 = ((rdi as u32)) as u32 as u64; rdx = ((rsi as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx)) as u64; r9 = 0; fl_l = 0; fl_r = 0; r8 = ((rdi as u32)) as u32 as u64; rdx = ((rsi as u32)) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = 0x1; bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415a07e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1d8; const P1: u64 = 0x20; const P2: u64 = 0xa8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x60u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rdi = ((rdx as u32)) as u32 as u64;
    crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm6) as u128);
    rbx = rcx;
    crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm7) as u128);
    xmm6 = (xmm3) as u128;
    xmm7 = (xmm2) as u128;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; rdx = rsp.wrapping_add(0x20); crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((rax as u32)) as u32); rcx = rbx; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((xmm7 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), ((xmm6 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rdi as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x0u8) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64;
    xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128;
    xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128;
    rsp = ((rsp).wrapping_add(0x60u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415a0850(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x58; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x8; const P4: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rsi = ((r8 as u32)) as u32 as u64; rbp = (((rdx as u8)) as u32) as u32 as u64; rdi = rcx; rbx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; r9 = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; r9 = ((r9).wrapping_shl(4u32)) as u64; r9 = ((r9).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; r8 = ((rsi as u32)) as u32 as u64; rdx = (((rbp as u8)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; rbx = ((rbx).wrapping_add(0x10u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rcx.wrapping_add(P1)))) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415a08d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x58; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x8; const P4: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rsi = r8; rbp = (((rdx as u8)) as u32) as u32 as u64; rdi = rcx; rbx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; r9 = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; r9 = ((r9).wrapping_shl(4u32)) as u64; r9 = ((r9).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; r8 = rsi; rdx = (((rbp as u8)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; rbx = ((rbx).wrapping_add(0x10u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rcx.wrapping_add(P1)))) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}
