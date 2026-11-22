#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn fn_142106ce0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143863780; const P1: u64 = 0x14210d870;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; r15 = 0; fl_l = 0; fl_r = 0; rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; rbx = rcx; r8 = (crate::simd_ops::ru64(rdi.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x60u64))) as u64; r14 = ((rdi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rsi = P0; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; r9 = 0x3fe8; rbp = (((rdx as u8)) as u32) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru32(rsi.wrapping_add(rbp.wrapping_mul(4)))) as u32 as u64; rsi = (((rsi as u32)).wrapping_add(0x8u32)) as u32 as u64; rax = crate::simd_ops::call_stk(rax, rax, rcx, rdx, r8, r9, rsp); r8 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rax, r15); rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), r15); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), r15); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), r15); crate::simd_ops::wu32(rax.wrapping_add(0x20u64), (0x3fe8u32) as u32); crate::simd_ops::wu32(rax.wrapping_add(0x24u64), ((rsi as u32)) as u32); crate::simd_ops::wu64(rax.wrapping_add(0x28u64), r15); rax = 0x3fb0; rcx = ((((rsi as u32)) as i32 as i64) as u64) as u64; let d = (rcx) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } crate::simd_ops::wu32(r8.wrapping_add(0x34u64), ((r15 as u32)) as u32); rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul(((rsi as u32)) as i32) as u32) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x30u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rdi.wrapping_add(rbp.wrapping_mul(8)).wrapping_add(0x60u64), r8); bb = 3; continue; }
            3 => { rdx = (((crate::simd_ops::ru32(r8.wrapping_add(0x30u64))) as i32 as i64) as u64) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = rdx.wrapping_add(0x38); rcx = ((rcx).wrapping_add(r8)) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r8.wrapping_add(0x24u64))) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x30u64), ((rdx as u32)) as u32); bb = 6; continue; }
            5 => { rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x28u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x28u64), rax); bb = 6; continue; }
            6 => { crate::simd_ops::lk_add_u32(r8.wrapping_add(0x34u64), (1) as u32); crate::simd_ops::wu64(rcx, r8); fl_l = (crate::simd_ops::ru64(r8.wrapping_add(0x28u64))) as u64; fl_r = r15; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru32(r8.wrapping_add(0x30u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 11; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x60u64), rax); rax = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rax, r15); bb = 10; continue; }
            10 => { crate::simd_ops::wu64(r8.wrapping_add(0x8u64), r15); bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = rcx.wrapping_add(0x8); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            12 => { rdx = 0x4; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x142106df6, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142106e00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18; const P2: u64 = 0x10; const P3: u64 = 0x20;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r10 = r9; r11 = rcx; r9 = (crate::simd_ops::ru64(r9.wrapping_add(P0))) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(r10)) as u64; crate::simd_ops::wu64(r9, rax); bb = 2; continue; }
            2 => { r9 = (crate::simd_ops::ru64(r10)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(r10.wrapping_add(P0))) as u64; crate::simd_ops::wu64(r9.wrapping_add(P0), rax); bb = 6; continue; }
            4 => { rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x28u64))) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_r = r10; rcx = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(r10.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rcx, rax); bb = 6; continue; }
            6 => { r11 = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(r10.wrapping_add(P1))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(r10.wrapping_add(P2))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P2), rax); bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(r10.wrapping_add(P2))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(r10.wrapping_add(P1))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P1), rax); bb = 13; continue; }
            11 => { fl_l = (crate::simd_ops::ru64(r8)) as u64; fl_r = r10; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(r10.wrapping_add(P1))) as u64; crate::simd_ops::wu64(r8, rax); bb = 13; continue; }
            13 => { r8 = (((crate::simd_ops::ru32(r10.wrapping_add(P3))) as i32 as i64) as u64) as u64; r9 = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rdx = r10; return crate::simd_ops::tail_stk(({ let t = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142106e90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x10; const P2: u64 = 0x20;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r10 = r8; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(r10.wrapping_add(P1))) as u64; crate::simd_ops::wu64(r8.wrapping_add(P1), rax); bb = 3; continue; }
            3 => { r8 = (crate::simd_ops::ru64(r10.wrapping_add(P1))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(r10.wrapping_add(P0))) as u64; crate::simd_ops::wu64(r8.wrapping_add(P0), rax); bb = 7; continue; }
            5 => { fl_l = (crate::simd_ops::ru64(rdx)) as u64; fl_r = r10; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(r10.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rdx, rax); bb = 7; continue; }
            7 => { r8 = (((crate::simd_ops::ru32(r10.wrapping_add(P2))) as i32 as i64) as u64) as u64; r9 = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rdx = r10; return crate::simd_ops::call_fwd(({ let t = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142106ef0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143863780; const P1: u64 = 0x142106f50;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = (((r9 as u8)) as u32) as u32 as u64;
    rax = P0;
    r9 = ((crate::simd_ops::ru8(rsp.wrapping_add(0x60u64))) as u32) as u32 as u64;
    rdi = rdx;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    r11 = r8;
    r8 = 0x3fe8;
    rax = (crate::simd_ops::ru32(rax.wrapping_add(rbx.wrapping_mul(4)))) as u32 as u64;
    r9 = ((rax.wrapping_add(r9.wrapping_mul(8))) as u32) as u32 as u64;
    rax = 0x3fb0;
    r10 = ((((r9 as u32)) as i32 as i64) as u64) as u64;
    let d = (r10) as u128; let n = ((rdx as u128) << 64) | (rax as u128);
    if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; }
    rdx = r11;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rdi.wrapping_add(rbx.wrapping_mul(8)), rax);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142106f50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x10; const P2: u64 = 0x20; const P3: u64 = 0x8; const P4: u64 = 0x28; const P5: u64 = 0x24; const P6: u64 = 0x34; const P7: u64 = 0x30; const P8: u64 = 0x14210d870;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rcx; rbp = ((((r8 as u32)) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = ((r9 as u32)) as u32 as u64; rbx = rdx; r9 = rbp; r8 = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = crate::simd_ops::call_stk(rax, rax, rcx, rdx, r8, r9, rsp); r10 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rax.wrapping_add(P2), ((rbp as u32)) as u32); crate::simd_ops::wu64(rax, rcx); crate::simd_ops::wu64(rax.wrapping_add(P3), rcx); crate::simd_ops::wu64(rax.wrapping_add(P1), rcx); crate::simd_ops::wu64(rax.wrapping_add(P0), rcx); crate::simd_ops::wu64(rax.wrapping_add(P4), rcx); crate::simd_ops::wu32(rax.wrapping_add(P5), ((rdi as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x50u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(P6), ((rcx as u32)) as u32); rax = ((((rax as u32)) as i32).wrapping_mul(((rdi as u32)) as i32) as u32) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(P7), ((rax as u32)) as u32); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(r10.wrapping_add(P0), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rax.wrapping_add(P1), r10); bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rbx, r10); bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = r10; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { rdx = 0x4; rcx = rsi; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x142106ff7, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142107000(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143863780; const P1: u64 = 0x142106ce0; const P2: u64 = 0x14183acbc; const P3: u64 = 0x142106b70; const P4: u64 = 0x14210d870;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; rax = r9.wrapping_sub(0x1); r15 = rcx; rbx = r9; rcx = P0; rdi = r8; r12 = rdx; rbp = 0xffffffff; fl_l = rax; fl_r = (0x200u64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (((crate::simd_ops::ru8(r9.wrapping_add(rcx).wrapping_add(0x80u64))) as i8 as i32) as u32) as u32 as u64; bb = 3; continue; }
            2 => { rdx = ((rbp as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rax = r8.wrapping_sub(0x1); fl_l = rax; fl_r = (0x200u64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rbp = (((crate::simd_ops::ru8(r8.wrapping_add(rcx).wrapping_add(0x80u64))) as i8 as i32) as u32) as u32 as u64; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 11; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { rdx = r12; rax = crate::simd_ops::call_stk(rax, rax, rcx, rdx, r8, r9, rsp); r14 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 20; continue; } bb = 8; continue; }
            8 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 21; continue; } bb = 9; continue; }
            9 => { bb = 20; continue; }
            10 => { r8 = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(rax, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            11 => { rcx = r15; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { r14 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 21; continue; } bb = 14; continue; }
            14 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 15; continue; }
            15 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { fl_l = rdi; fl_r = rbx; r8 = rbx; rdx = r12; rcx = r14; r8 = (if (((fl_l as u64)) < ((fl_r as u64))) { rdi } else { r8 }) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            17 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { r8 = r12; rdx = ((rbp as u32)) as u32 as u64; rcx = r15; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            19 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; r9 = 0; fl_l = 0; fl_r = 0; r8 = rdi; rdx = r12; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            20 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = rbx; rax = ((rax).wrapping_sub(rdi)) as u64; rbx = ((rbx).wrapping_sub(rdi)) as u64; crate::simd_ops::lk_add_u64(rsi.wrapping_add(0x48u64), (rax) as u64); rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x60u64))) as u32) as u32 as u64; crate::simd_ops::lk_add_u64(rsi.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x270u64), (rbx) as u64); rax = r14; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            21 => { rdx = 0x4; rcx = r15; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x142107137, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142107450(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0xffffffffffffffff; const P2: u64 = 0x143863800; const P3: u64 = 0x142106ce0; const P4: u64 = 0x10; const P5: u64 = 0x48; const P6: u64 = 0x14210d870;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = ((rdx.wrapping_add(P1)) as u64) as u64; rbp = (((r8 as u8)) as u32) as u32 as u64; rbx = rdx; rsi = rcx; fl_l = rax; fl_r = (0x200u64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = P2; rdx = (((crate::simd_ops::ru8(rdx.wrapping_add(rax))) as i8 as i32) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            3 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; r9 = rbx; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(rax, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::lk_add_u64(rdi.wrapping_add(P5), (rbx) as u64); rax = rcx; crate::simd_ops::lk_add_u64(rdi.wrapping_add(rbp.wrapping_mul(8)).wrapping_add(0x270u64), (rbx) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { rdx = 0x4; rcx = rsi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1421074e4, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142107510(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0xffffffffffffffff; const P2: u64 = 0x143863800; const P3: u64 = 0x142106b70; const P4: u64 = 0x10; const P5: u64 = 0x48; const P6: u64 = 0x270;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = ((r8.wrapping_add(P1)) as u64) as u64; rsi = (((r9 as u8)) as u32) as u32 as u64; rdi = r8; r10 = rdx; fl_l = rax; fl_r = (0x200u64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = P2; rdx = (((crate::simd_ops::ru8(r8.wrapping_add(rax))) as i8 as i32) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { r8 = r10; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; r9 = 0; fl_l = 0; fl_r = 0; rdx = r10; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { crate::simd_ops::lk_sub_u64(rbx.wrapping_add(P5), (rdi) as u64); crate::simd_ops::lk_sub_u64(rbx.wrapping_add(rsi.wrapping_mul(8)).wrapping_add(P6), (rdi) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142107580(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646a40; const P1: u64 = 0x142646a58; const P2: u64 = 0x142986ff0; const P3: u64 = 0x1433e1218; const P4: u64 = 0x1421153b0; const P5: u64 = 0x1433e0648; const P6: u64 = 0x142103330; const P7: u64 = 0x1420ec940; const P8: u64 = 0x1429868d0; const P9: u64 = 0x1420ec690; const P10: u64 = 0x1433de034; const P11: u64 = 0x142118cd0; const P12: u64 = 0x1433ddf74; const P13: u64 = 0x1433e0640; const P14: u64 = 0x1433e11f8; const P15: u64 = 0x142116b30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rdi = rcx; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; rbx = (((rbx) ^ (rdi))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = ((((rax as u32)) as i32 as i64) as u64) as u64; r14 = P2; rbx = (((rbx) ^ (rdx))) as u64; rax = (0x5851f42d4c957f2du64) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rbx = (((rbx) as i64).wrapping_mul((rax) as i64) as u64) as u64; rax = (0x399d2694695129deu64) as u64; rbx = ((rbx).wrapping_add(rax)) as u64; rax = r14; r15 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdx.wrapping_add(0xc20u64), rbx); rbp = ((r15 as u32)) as u32 as u64; bb = 1; continue; }
            1 => { rbp = (((rbp as u32)).wrapping_add(1)) as u32 as u64; rax = rax.wrapping_add(0x10); fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = r15; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { r9 = 0x1; r8 = P3; rdx = 0xffffd8f0; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x1u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rdi.wrapping_add(0x1u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rdi); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rsi = rdi.wrapping_add(0x8); r8 = 0x4; rdx = P5; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(rsi)) as u64; r8 = rsp.wrapping_add(0x30); rdx = rbx.wrapping_sub(0x18); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rcx = rdi; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x5u32) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = P8; rdx = rax.wrapping_add(0x18); crate::simd_ops::wu64(rsi, rdx); fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x6; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 6; continue; }
            6 => { r9 = ((rbp as u32)) as u32 as u64; crate::simd_ops::wu64(rsi, rax); r8 = P5; rdx = 0xffffd8ee; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 19; continue; } bb = 7; continue; }
            7 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x1u64))) as u32) as u32 as u64; rbp = rsi; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rbp = rdi.wrapping_add(0x8); fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rdi.wrapping_add(0x1u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rdi); bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = P5; r8 = 0x4; rcx = rdi; xmm0 = (crate::simd_ops::ru128(rax.wrapping_sub(0x18u64))) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_sub(0x8u64))) as u64) as u128; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rbx = (crate::simd_ops::ru64(rsi)) as u64; rbx = ((rbx).wrapping_add(0x18u64)) as u64; crate::simd_ops::wu64(rsi, rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(rsi)) as u64; rdx = rbx.wrapping_sub(0x48); r9 = ((r9).wrapping_sub(0x18u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); r8 = rsp.wrapping_add(0x30); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x5u32) as u32); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::lk_add_u64(rsi, (0xffffffffffffffe8u64) as u64)) as u64; fl_r = fl_l; rdx = (crate::simd_ops::ru64(rsi)) as u64; rsi = rbp; bb = 10; continue; }
            10 => { rax = rdx.wrapping_sub(0x18); fl_l = rax; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 14; continue; } bb = 11; continue; }
            11 => { rcx = rax.wrapping_sub(0x18); bb = 12; continue; }
            12 => { xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rdx = (crate::simd_ops::ru64(rsi)) as u64; rcx = rcx.wrapping_add(0x18); rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 12; continue; } bb = 13; continue; }
            13 => { rsi = rdi.wrapping_add(0x8); bb = 14; continue; }
            14 => { rax = rdx.wrapping_sub(0x18); crate::simd_ops::wu64(rsi, rax); rax = P10; bb = 15; continue; }
            15 => { rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; r9 = 0; fl_l = 0; fl_r = 0; r8 = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(r14)) as u64; r8 = (0xffffffffffffffffu64) as u64; rbx = (crate::simd_ops::ru64(rsi)) as u64; bb = 16; continue; }
            16 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 16; continue; } bb = 17; continue; }
            17 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(rsi)) as u64; r8 = rsp.wrapping_add(0x30); r9 = ((r9).wrapping_sub(0x18u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rdx = rbx.wrapping_sub(0x30); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x5u32) as u32); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsi)) as u64; r14 = ((r14).wrapping_add(0x10u64)) as u64; rcx = ((rcx).wrapping_add(0xffffffffffffffe8u64)) as u64; crate::simd_ops::wu64(rsi, rcx); rax = (crate::simd_ops::ru64(r14)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 15; continue; } bb = 18; continue; }
            18 => { crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), (0x3u32) as u32); rdx = P12; rax = (0x400921fb54442d18u64) as u64; r8 = 0x2; crate::simd_ops::wu64(rcx, rax); rcx = rdi; rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rbx = ((rbx).wrapping_add(0x18u64)) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rdx = rbx.wrapping_sub(0x30); r9 = ((r9).wrapping_sub(0x18u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); r8 = rsp.wrapping_add(0x30); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x5u32) as u32); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rdx = P13; rax = ((rax).wrapping_sub(0x18u64)) as u64; rcx = (0x7ff0000000000000u64) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rax); r8 = 0x4; crate::simd_ops::wu64(rax, rcx); rcx = rdi; crate::simd_ops::wu32(rax.wrapping_add(0x10u64), (0x3u32) as u32); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rbx = ((rbx).wrapping_add(0x18u64)) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rdx = rbx.wrapping_sub(0x30); r9 = ((r9).wrapping_sub(0x18u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); r8 = rsp.wrapping_add(0x30); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x5u32) as u32); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::lk_sub_u64(rdi.wrapping_add(0x8u64), (0x18u64) as u64); rax = 0x1; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            19 => { r8 = P5; rcx = rdi; rdx = P14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P15, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1421078fe, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142107910(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1429868d0; const P2: u64 = 0x8; const P3: u64 = 0x1420ecd00; const P4: u64 = 0x1426466c8; const P5: u64 = 0x2; const P6: u64 = 0x1421158a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rcx; rcx = P1; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rcx }) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { xmm0 = ((crate::simd_ops::ru64(rcx)) as u64) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; crate::simd_ops::wu64(rax, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rax.wrapping_add(P0), (0x3u32) as u32); rax = 0x1; crate::simd_ops::lk_add_u64(rbx.wrapping_add(P2), (0x18u64) as u64); rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { rdx = 0x1; rcx = rbx; r8 = ((rdx.wrapping_add(P5)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x14210797e, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142107990(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1429868d0; const P2: u64 = 0x8; const P3: u64 = 0x1420ecd00; const P4: u64 = 0x143426a48; const P5: u64 = 0x143427098; const P6: u64 = 0x2; const P7: u64 = 0x1421158a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rcx; rcx = P1; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rdx = ((rbx.wrapping_add(P2)) as u64) as u64; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rcx }) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 2; continue; }
            2 => { rdx = ((rbx.wrapping_add(P2)) as u64) as u64; bb = 3; continue; }
            3 => { xmm0 = ((crate::simd_ops::ru64(rcx)) as u64) as u128; xmm2 = 0u128; fl_l = ((xmm0 as u64)) as u64; fl_r = ((xmm2 as u64)) as u64; if !(f64::from_bits(fl_l) > f64::from_bits(fl_r)) { bb = 5; continue; } bb = 4; continue; }
            4 => { xmm0 = ((crate::simd_ops::ru64(P4)) as u64) as u128; bb = 6; continue; }
            5 => { xmm1 = crate::simd_ops::vcmpsd(xmm0, (xmm0 as u64), (xmm2 as u64), 0x1u8); xmm0 = ((crate::simd_ops::ru64(P5)) as u64) as u128; xmm0 = crate::simd_ops::vblendvpd(xmm2, xmm0, xmm1); bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu64(rax, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rax.wrapping_add(P0), (0x3u32) as u32); rax = 0x1; crate::simd_ops::lk_add_u64(rdx, (0x18u64) as u64); rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { rdx = 0x1; rcx = rbx; r8 = ((rdx.wrapping_add(P6)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x142107a2f, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142107a40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1429868d0; const P1: u64 = 0x1420ecd00; const P2: u64 = 0x1433e06b8; const P3: u64 = 0x142116bc0; const P4: u64 = 0x1421158a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rdi = P0; rbx = rcx; rcx = rdi; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_r = rdx; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { crate::simd_ops::ru64(rbx.wrapping_add(0x10u64)) } else { rcx }) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(0x18u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm7) as u128); xmm7 = ((crate::simd_ops::ru64(rcx)) as u64) as u128; fl_l = rax; fl_r = rdx; rcx = rdi; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rcx }) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(0x30u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm6) as u128); xmm6 = ((crate::simd_ops::ru64(rcx)) as u64) as u128; fl_l = rax; fl_r = rdx; rdi = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdi }) as u64; fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = rsp.wrapping_add(0x20); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { xmm1 = ((crate::simd_ops::ru64(rdi)) as u64) as u128; fl_l = ((xmm1 as u64)) as u64; fl_r = ((xmm6 as u64)) as u64; if !(f64::from_bits(fl_l) >= f64::from_bits(fl_r)) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; xmm0 = (xmm6 & !0xffff_ffff_ffff_ffffu128) | ((if f64::from_bits(xmm6 as u64) > f64::from_bits(xmm7 as u64) { f64::from_bits(xmm6 as u64) } else { f64::from_bits(xmm7 as u64) }).to_bits() as u128); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu32(rax.wrapping_add(0x10u64), (0x3u32) as u32); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((if f64::from_bits(xmm1 as u64) < f64::from_bits(xmm0 as u64) { f64::from_bits(xmm1 as u64) } else { f64::from_bits(xmm0 as u64) }).to_bits() as u128); crate::simd_ops::wu64(rax, ((xmm1 as u64)) as u64); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0x18u64) as u64); rax = 0x1; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            10 => { r8 = P2; rdx = 0x3; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x142107b4a, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            11 => { rdx = 0x3; rcx = rbx; r8 = ((rdx as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x142107b5b, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            12 => { rdx = 0x1; rcx = rbx; r8 = ((rdx.wrapping_add(0x2u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x142107b6d, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            13 => { rdx = 0x2; rcx = rbx; r8 = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x142107b7f, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142107b90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1429868d0; const P1: u64 = 0x1420ecd00; const P2: u64 = 0x142107ce0; const P3: u64 = 0x1421158a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rbx = P0; rdi = rcx; rcx = rbx; fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; fl_r = rdx; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { crate::simd_ops::ru64(rdi.wrapping_add(0x10u64)) } else { rcx }) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(0x18u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm6) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm7) as u128); xmm7 = ((crate::simd_ops::ru64(rcx)) as u64) as u128; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 4; continue; }
            4 => { fl_l = rax; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 10; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rdx = rsp.wrapping_add(0x20); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; bb = 9; continue; }
            9 => { xmm6 = ((crate::simd_ops::ru64(rax)) as u64) as u128; bb = 11; continue; }
            10 => { xmm6 = 0u128; bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(0x30u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 17; continue; } bb = 12; continue; }
            12 => { fl_l = rax; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 17; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rdx = rsp.wrapping_add(0x20); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 16; continue; }
            16 => { xmm0 = ((crate::simd_ops::ru64(rax)) as u64) as u128; bb = 18; continue; }
            17 => { xmm0 = 0u128; bb = 18; continue; }
            18 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; xmm2 = (xmm0 & !0xffff_ffffu128) | (((f64::from_bits(xmm0 as u64)) as f32).to_bits() as u128); xmm0 = (xmm7 & !0xffff_ffffu128) | (((f64::from_bits(xmm7 as u64)) as f32).to_bits() as u128); xmm1 = (xmm6 & !0xffff_ffffu128) | (((f64::from_bits(xmm6 as u64)) as f32).to_bits() as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; rax = 0x1; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), (0x3u32) as u32); xmm1 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | (((f32::from_bits(xmm0 as u32)) as f64).to_bits() as u128); crate::simd_ops::wu64(rbx, ((xmm1 as u64)) as u64); crate::simd_ops::lk_add_u64(rdi.wrapping_add(0x8u64), (0x18u64) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            19 => { rdx = 0x1; rcx = rdi; r8 = ((rdx.wrapping_add(0x2u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x142107cb6, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            20 => { rdx = 0x2; rcx = rdi; r8 = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x142107cc8, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            21 => { rdx = 0x3; rcx = rdi; r8 = ((rdx as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x142107cd9, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142107ce0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142986df0; const P1: u64 = 0x1434268d0; const P2: u64 = 0x143426d6c; const P3: u64 = 0x143426e04; const P4: u64 = 0x143426dd4; const P5: u64 = 0x142108000;
    let (mut rax, mut r10, mut r11, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0; let mut xmm11: u128 = 0; let mut xmm12: u128 = 0; let mut xmm13: u128 = 0; let mut xmm14: u128 = 0; let mut xmm15: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0xc0u64)) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x18u64), (xmm6) as u128); rdi = P0; crate::simd_ops::wu128(rax.wrapping_sub(0x28u64), (xmm7) as u128); xmm7 = ((crate::simd_ops::ru32(P1)) as u32) as u128; crate::simd_ops::wu128(rax.wrapping_sub(0x38u64), (xmm8) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x48u64), (xmm9) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x58u64), (xmm10) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x68u64), (xmm11) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x78u64), (xmm12) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm13) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm14) as u128); xmm5 = 0u128; xmm5 = crate::simd_ops::vroundss(xmm5, (xmm1 as u32), 0x1u8); xmm14 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm5 as u32)).to_bits() as u128); rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm5 as u32)))) as u32 as u64; rdx = (((rax as u8)) as u32) as u32 as u64; xmm4 = 0u128; xmm4 = crate::simd_ops::vroundss(xmm4, (xmm2 as u32), 0x1u8); xmm9 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm4 as u32)).to_bits() as u128); rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm4 as u32)))) as u32 as u64; rbx = (((rax as u8)) as u32) as u32 as u64; xmm6 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm15) as u128); xmm6 = crate::simd_ops::vroundss(xmm6, (xmm0 as u32), 0x1u8); xmm11 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm0 = (xmm11 & !0xffff_ffffu128) | ((f32::from_bits(xmm11 as u32) * f32::from_bits(crate::simd_ops::ru32(P2))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(crate::simd_ops::ru32(P3))).to_bits() as u128); xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm11 as u32)).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(crate::simd_ops::ru32(P4))).to_bits() as u128); rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm6 as u32)))) as u32 as u64; rcx = (((rax as u8)) as u32) as u32 as u64; xmm0 = (xmm11 & !0xffff_ffffu128) | ((f32::from_bits(xmm11 as u32) * f32::from_bits(xmm11 as u32)).to_bits() as u128); xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm11 as u32)).to_bits() as u128); xmm8 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); rax = ((crate::simd_ops::ru8(rcx.wrapping_add(rdi))) as u32) as u32 as u64; xmm2 = (xmm14 & !0xffff_ffffu128) | ((f32::from_bits(xmm14 as u32) * f32::from_bits(crate::simd_ops::ru32(P2))).to_bits() as u128); xmm0 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(crate::simd_ops::ru32(P3))).to_bits() as u128); r11 = ((crate::simd_ops::ru8(rcx.wrapping_add(rdi).wrapping_add(0x1u64))) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_add((rdx as u32))) as u32 as u64; xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm14 as u32)).to_bits() as u128); xmm3 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(crate::simd_ops::ru32(P4))).to_bits() as u128); r8 = ((rax as u32)) as u32 as u64; r11 = (((r11 as u32)).wrapping_add((rdx as u32))) as u32 as u64; rax = ((crate::simd_ops::ru8(rax.wrapping_add(rdi).wrapping_add(0x1u64))) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_add((rbx as u32))) as u32 as u64; xmm2 = (xmm14 & !0xffff_ffffu128) | ((f32::from_bits(xmm14 as u32) * f32::from_bits(xmm14 as u32)).to_bits() as u128); xmm0 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm14 as u32)).to_bits() as u128); xmm12 = (xmm9 & !0xffff_ffffu128) | ((f32::from_bits(xmm9 as u32) - f32::from_bits(xmm7 as u32)).to_bits() as u128); rcx = ((crate::simd_ops::ru8(rax.wrapping_add(rdi).wrapping_add(0x1u64))) as u32) as u32 as u64; xmm13 = (xmm14 & !0xffff_ffffu128) | ((f32::from_bits(xmm14 as u32) - f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm15 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm3 = (xmm12) as u128; xmm2 = (xmm13) as u128; xmm1 = (xmm11) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0xd0u64), ((xmm9 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0xd8u64), ((xmm8 as u32)) as u32); r10 = ((rax as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru8(r8.wrapping_add(rdi))) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_add((rbx as u32))) as u32 as u64; xmm3 = (xmm12) as u128; xmm2 = (xmm14) as u128; xmm1 = (xmm11) as u128; rcx = ((crate::simd_ops::ru8(rax.wrapping_add(rdi).wrapping_add(0x1u64))) as u32) as u32 as u64; xmm10 = (xmm0) as u128; r9 = ((rax as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru8(r11.wrapping_add(rdi))) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_add((rbx as u32))) as u32 as u64; xmm9 = (xmm11 & !0xffff_ffffu128) | ((f32::from_bits(xmm11 as u32) - f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm3 = (xmm12) as u128; xmm2 = (xmm14) as u128; rcx = ((crate::simd_ops::ru8(rax.wrapping_add(rdi).wrapping_add(0x1u64))) as u32) as u32 as u64; xmm1 = (xmm9) as u128; xmm6 = (xmm0) as u128; r8 = ((rax as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm2 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm3 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm8 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm3 = ((crate::simd_ops::ru32(rsp.wrapping_add(0xd0u64))) as u32) as u128; rcx = ((crate::simd_ops::ru8(r10.wrapping_add(rdi))) as u32) as u32 as u64; xmm2 = (xmm13) as u128; xmm1 = (xmm11) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm3 = ((crate::simd_ops::ru32(rsp.wrapping_add(0xd0u64))) as u32) as u128; rcx = ((crate::simd_ops::ru8(r9.wrapping_add(rdi))) as u32) as u32 as u64; xmm2 = (xmm14) as u128; xmm1 = (xmm11) as u128; xmm7 = (xmm0) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm3 = ((crate::simd_ops::ru32(rsp.wrapping_add(0xd0u64))) as u32) as u128; rcx = ((crate::simd_ops::ru8(r8.wrapping_add(rdi))) as u32) as u32 as u64; xmm2 = (xmm14) as u128; xmm1 = (xmm9) as u128; xmm6 = (xmm0) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru8(r11.wrapping_add(rdi).wrapping_add(0x1u64))) as u32) as u32 as u64; xmm11 = ((crate::simd_ops::ru32(rsp.wrapping_add(0xd8u64))) as u32) as u128; xmm14 = ((crate::simd_ops::ru32(rsp.wrapping_add(0xd0u64))) as u32) as u128; xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm11 as u32)).to_bits() as u128); rax = (((rax as u32)).wrapping_add((rbx as u32))) as u32 as u64; xmm6 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm3 = (xmm14) as u128; xmm2 = (xmm13) as u128; rcx = ((crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u32) as u32 as u64; xmm1 = (xmm9) as u128; r8 = ((rax as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru8(r8.wrapping_add(rdi).wrapping_add(0x1u64))) as u32) as u32 as u64; xmm4 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm5 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(xmm11 as u32)).to_bits() as u128); xmm0 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) + f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm4 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm5 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(xmm15 as u32)).to_bits() as u128); xmm3 = (xmm12) as u128; xmm2 = (xmm13) as u128; xmm1 = (xmm9) as u128; xmm6 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) + f32::from_bits(xmm6 as u32)).to_bits() as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r11 = rsp.wrapping_add(0xc0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; xmm7 = (crate::simd_ops::ru128(r11.wrapping_sub(0x20u64))) as u128; xmm9 = (crate::simd_ops::ru128(r11.wrapping_sub(0x40u64))) as u128; xmm12 = (crate::simd_ops::ru128(r11.wrapping_sub(0x70u64))) as u128; xmm13 = (crate::simd_ops::ru128(r11.wrapping_sub(0x80u64))) as u128; xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm10 as u32)).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm11 as u32)).to_bits() as u128); xmm11 = (crate::simd_ops::ru128(r11.wrapping_sub(0x60u64))) as u128; xmm3 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm10 as u32)).to_bits() as u128); xmm10 = (crate::simd_ops::ru128(r11.wrapping_sub(0x50u64))) as u128; xmm0 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm15 as u32)).to_bits() as u128); xmm0 = (xmm14 & !0xffff_ffffu128) | ((f32::from_bits(xmm14 as u32) * f32::from_bits(crate::simd_ops::ru32(P2))).to_bits() as u128); xmm15 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(crate::simd_ops::ru32(P3))).to_bits() as u128); xmm8 = (crate::simd_ops::ru128(r11.wrapping_sub(0x30u64))) as u128; xmm4 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm14 as u32)).to_bits() as u128); xmm3 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(crate::simd_ops::ru32(P4))).to_bits() as u128); xmm0 = (xmm14 & !0xffff_ffffu128) | ((f32::from_bits(xmm14 as u32) * f32::from_bits(xmm14 as u32)).to_bits() as u128); xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm14 as u32)).to_bits() as u128); xmm14 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; xmm2 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm3 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm0 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm6 = (crate::simd_ops::ru128(r11.wrapping_sub(0x10u64))) as u128; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142108000(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xfffffffffffffff4; const P1: u64 = 0x143428760;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0xfu8))) as u8 as u64)); crate::simd_ops::wu128(rsp.wrapping_add(0x10u64), (xmm6) as u128); rax = (((rcx as u8)) as u32) as u32 as u64; crate::simd_ops::wu128(rsp, (xmm7) as u128); xmm4 = (((rax as u32)) as u32) as u128; rax = 0x8; xmm0 = (((rax as u32)) as u32) as u128; xmm5 = crate::simd_ops::vpcmpgtd(xmm0, xmm4); xmm7 = (xmm3) as u128; xmm3 = crate::simd_ops::vblendvps(xmm2, xmm1, xmm5); xmm6 = (xmm1) as u128; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if (((fl_l as u8)) < ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((rcx.wrapping_add(P0)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xfdu8))) as u8 as u64)); rdx = 0; fl_l = 0; fl_r = 0; rax = (((rax as u8)) as u32) as u32 as u64; xmm1 = (((rdx as u32)) as u32) as u128; xmm0 = (((rax as u32)) as u32) as u128; xmm2 = crate::simd_ops::vpcmpeqd(xmm0, xmm1); xmm2 = crate::simd_ops::vblendvps(xmm7, xmm6, xmm2); bb = 2; continue; }
            2 => { xmm0 = ((crate::simd_ops::ru32(P1)) as u32) as u128; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { xmm3 = (xmm3) ^ (xmm0); bb = 4; continue; }
            4 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { xmm0 = (xmm2) ^ (xmm0); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp)) as u128; rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp)) as u128; xmm0 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm3 as u32)).to_bits() as u128); rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421080f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1429868d0; const P2: u64 = 0x8; const P3: u64 = 0x1420ecd00; const P4: u64 = 0x18; const P5: u64 = 0xc20; const P6: u64 = 0x2; const P7: u64 = 0x1421158a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut xmm0: u128 = 0; let mut xmm1: u128 = 0; let mut xmm2: u128 = 0; let mut xmm3: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rcx; rcx = P1; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rcx }) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::cvt_f64_i32((f64::from_bits(crate::simd_ops::ru64(rcx))))) as u32 as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; rax = (0x5851f42d4c957f2du64) as u64; rcx = (((rcx) as i64).wrapping_mul((rax) as i64) as u64) as u64; rax = (0x399d2694695129deu64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P5), rcx); rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { rdx = 0x1; rcx = rbx; r8 = ((rdx.wrapping_add(P6)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x14210816a, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142108180(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1429868d0; const P1: u64 = 0x1420ecd00; const P2: u64 = 0x142646720; const P3: u64 = 0x1433e06e8; const P4: u64 = 0x142116b30; const P5: u64 = 0x1421158a0; const P6: u64 = 0x1433e0708; const P7: u64 = 0x142116bc0; const P8: u64 = 0x1433e0720;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rbx = rcx; rbp = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; rdx = r8; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; rdx = ((rdx).wrapping_sub(rcx)) as u64; let prod = (rax as i64 as i128).wrapping_mul((rdx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 15; continue; } bb = 1; continue; }
            1 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 2; continue; }
            2 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 20; continue; } bb = 3; continue; }
            3 => { fl_l = rcx; fl_r = r8; rdi = P0; rax = rdi; rax = (if (((fl_l as u64)) < ((fl_r as u64))) { rcx } else { rax }) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = rsp.wrapping_add(0x20); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 5; continue; }
            5 => { rsi = (crate::simd_ops::cvt_f64_i32((f64::from_bits(crate::simd_ops::ru64(rax))))) as u32 as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdi = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdi }) as u64; fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = rsp.wrapping_add(0x20); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 7; continue; }
            7 => { r10 = (crate::simd_ops::cvt_f64_i32((f64::from_bits(crate::simd_ops::ru64(rdi))))) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 23; continue; } bb = 8; continue; }
            8 => { r10 = (((r10 as u32)).wrapping_sub((rsi as u32))) as u32 as u64; fl_l = ((r10 as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 22; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xc20u64))) as u64; rdx = (0x5851f42d4c957f2du64) as u64; rax = rcx; r8 = rcx; rax = (((rax) as i64).wrapping_mul((rdx) as i64) as u64) as u64; r8 = ((r8).wrapping_shr(45u32)) as u64; rax = ((rax).wrapping_add(0x69u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0xc20u64), rax); rax = rcx; rax = ((rax).wrapping_shr(27u32)) as u64; r8 = ((((r8 as u32)) ^ ((rax as u32)))) as u32 as u64; rcx = ((rcx).wrapping_shr(59u32)) as u64; rax = ((rcx as u32)) as u32 as u64; rcx = ((rcx as u32)) as u32 as u64; rax = (0u32.wrapping_sub((rax as u32))) as u32 as u64; rax = ((((rax as u32)) & (0x1fu32))) as u32 as u64; rdx = (((r8 as u32)).wrapping_shl(((((rax as u32)) as u32) & 31))) as u32 as u64; rax = ((r8 as u32)) as u32 as u64; rcx = ((rax).wrapping_shr((((rcx) as u32) & 63))) as u64; rdx = (((rdx) | (rcx))) as u64; rcx = ((r10.wrapping_add(0x1u64)) as u32) as u32 as u64; rdx = (((rdx) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rdx = ((rdx).wrapping_shr(32u32)) as u64; fl_l = (((rdx as u32)).wrapping_add((rsi as u32))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; bb = 10; continue; }
            10 => { r9 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | (((((rdx as u32)) as i32) as f64).to_bits() as u128); crate::simd_ops::wu64(r9, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(r9.wrapping_add(0x10u64), (0x3u32) as u32); bb = 19; continue; }
            11 => { fl_l = rcx; fl_r = r8; rdi = P0; rdi = (if (((fl_l as u64)) < ((fl_r as u64))) { rcx } else { rdi }) as u64; fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rdx = rsp.wrapping_add(0x20); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 13; continue; }
            13 => { r10 = (crate::simd_ops::cvt_f64_i32((f64::from_bits(crate::simd_ops::ru64(rdi))))) as u32 as u64; fl_l = ((r10 as u32)) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 25; continue; } bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xc20u64))) as u64; rdx = (0x5851f42d4c957f2du64) as u64; rax = rcx; r8 = rcx; rax = (((rax) as i64).wrapping_mul((rdx) as i64) as u64) as u64; r8 = ((r8).wrapping_shr(45u32)) as u64; rax = ((rax).wrapping_add(0x69u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0xc20u64), rax); rax = rcx; rax = ((rax).wrapping_shr(27u32)) as u64; r8 = ((((r8 as u32)) ^ ((rax as u32)))) as u32 as u64; rcx = ((rcx).wrapping_shr(59u32)) as u64; rax = ((rcx as u32)) as u32 as u64; rcx = ((rcx as u32)) as u32 as u64; rax = (0u32.wrapping_sub((rax as u32))) as u32 as u64; rax = ((((rax as u32)) & (0x1fu32))) as u32 as u64; rdx = (((r8 as u32)).wrapping_shl(((((rax as u32)) as u32) & 31))) as u32 as u64; rax = ((r8 as u32)) as u32 as u64; rcx = ((rax).wrapping_shr((((rcx) as u32) & 63))) as u64; rdx = (((rdx) | (rcx))) as u64; rax = ((((r10 as u32)) as i32 as i64) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((rax) as i64) as u64) as u64; rdx = ((rdx).wrapping_shr(32u32)) as u64; fl_l = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rdx = ((fl_l as u32)) as u32 as u64; bb = 10; continue; }
            15 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xc20u64))) as u64; rdx = (0x5851f42d4c957f2du64) as u64; r9 = rcx; rax = rcx; rax = ((rax).wrapping_shr(27u32)) as u64; r8 = rcx; rcx = (((rcx) as i64).wrapping_mul((rdx) as i64) as u64) as u64; r9 = ((r9).wrapping_shr(45u32)) as u64; r9 = ((((r9 as u32)) ^ ((rax as u32)))) as u32 as u64; r8 = ((r8).wrapping_shr(59u32)) as u64; rcx = ((rcx).wrapping_add(0x69u64)) as u64; rax = rcx; rax = (((rax) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rdx = rcx; rax = ((rax).wrapping_add(0x69u64)) as u64; rdx = ((rdx).wrapping_shr(45u32)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0xc20u64), rax); rax = rcx; rax = ((rax).wrapping_shr(27u32)) as u64; rdx = ((((rdx as u32)) ^ ((rax as u32)))) as u32 as u64; rcx = ((rcx).wrapping_shr(59u32)) as u64; rax = ((rcx as u32)) as u32 as u64; rcx = ((rcx as u32)) as u32 as u64; rax = (0u32.wrapping_sub((rax as u32))) as u32 as u64; rax = ((((rax as u32)) & (0x1fu32))) as u32 as u64; r10 = (((rdx as u32)).wrapping_shl(((((rax as u32)) as u32) & 31))) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; rcx = ((rax).wrapping_shr((((rcx) as u32) & 63))) as u64; r10 = (((r10) | (rcx))) as u64; rax = ((r8 as u32)) as u32 as u64; rax = (0u32.wrapping_sub((rax as u32))) as u32 as u64; r10 = ((r10).wrapping_shl(32u32)) as u64; rax = ((((rax as u32)) & (0x1fu32))) as u32 as u64; rcx = (((r9 as u32)).wrapping_shl(((((rax as u32)) as u32) & 31))) as u32 as u64; r10 = (((r10) | (rcx))) as u64; rax = ((r9 as u32)) as u32 as u64; rcx = ((r8 as u32)) as u32 as u64; rdx = ((rax).wrapping_shr((((rcx) as u32) & 63))) as u64; fl_l = (((r10) | (rdx))) as u64; fl_r = fl_l; r10 = ((fl_l as u64)) as u64; xmm0 = 0u128; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((r10) as i64) as f64).to_bits() as u128); bb = 18; continue; }
            17 => { rax = r10; r10 = ((((r10 as u32)) & (0x1u32))) as u32 as u64; rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = (((rax) | (r10))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) + f64::from_bits(xmm0 as u64)).to_bits() as u128); bb = 18; continue; }
            18 => { rdx = 0xffffffc0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rax, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rax.wrapping_add(0x10u64), (0x3u32) as u32); bb = 19; continue; }
            19 => { crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0x18u64) as u64); rax = 0x1; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            20 => { rdx = P3; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x14210846f, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            21 => { rdx = 0x2; rcx = rbx; r8 = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x142108481, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            22 => { r8 = P6; rdx = 0x2; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x142108496, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            23 => { r8 = P8; rdx = 0x2; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1421084ab, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            24 => { rdx = 0x1; rcx = rbx; r8 = ((rdx.wrapping_add(0x2u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1421084bd, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            25 => { r8 = P8; rdx = 0x1; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1421084d2, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421084e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x1429868d0; const P3: u64 = 0x1420ecd00; const P4: u64 = 0x142116c20; const P5: u64 = 0x2; const P6: u64 = 0x1421158a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rdi = rcx; rdx = ((rdx).wrapping_sub(r9)) as u64; let prod = (rax as i64 as i128).wrapping_mul((rdx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rsi = rdx; rdx = ((rcx.wrapping_add(P1)) as u64) as u64; rsi = ((((rsi) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rsi; rax = ((rax).wrapping_shr(63u32)) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; rax = P2; fl_l = r9; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rax = (if (((fl_l as u64)) < ((fl_r as u64))) { r9 } else { rax }) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rdx = rsp.wrapping_add(0x20); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 2; continue; }
            2 => { rdx = ((rdi.wrapping_add(P1)) as u64) as u64; bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); rbx = 0x2; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm6) as u128); xmm6 = ((crate::simd_ops::ru64(rax)) as u64) as u128; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = ((rbx as u32)) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; xmm6 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((if f64::from_bits(xmm0 as u64) > f64::from_bits(xmm6 as u64) { f64::from_bits(xmm0 as u64) } else { f64::from_bits(xmm6 as u64) }).to_bits() as u128); fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 4; continue; } bb = 5; continue; }
            5 => { rdx = ((rdi.wrapping_add(P1)) as u64) as u64; bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(rax, ((xmm6 as u64)) as u64); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu32(rax.wrapping_add(P0), (0x3u32) as u32); rax = 0x1; crate::simd_ops::lk_add_u64(rdx, (0x18u64) as u64); rsp = ((rsp).wrapping_add(0x50u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { rdx = 0x1; rcx = rdi; r8 = ((rdx.wrapping_add(P5)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1421085c0, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421085d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x1429868d0; const P3: u64 = 0x1420ecd00; const P4: u64 = 0x142116c20; const P5: u64 = 0x2; const P6: u64 = 0x1421158a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rdi = rcx; rdx = ((rdx).wrapping_sub(r9)) as u64; let prod = (rax as i64 as i128).wrapping_mul((rdx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rsi = rdx; rdx = ((rcx.wrapping_add(P1)) as u64) as u64; rsi = ((((rsi) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rsi; rax = ((rax).wrapping_shr(63u32)) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; rax = P2; fl_l = r9; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rax = (if (((fl_l as u64)) < ((fl_r as u64))) { r9 } else { rax }) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rdx = rsp.wrapping_add(0x20); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 2; continue; }
            2 => { rdx = ((rdi.wrapping_add(P1)) as u64) as u64; bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); rbx = 0x2; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm6) as u128); xmm6 = ((crate::simd_ops::ru64(rax)) as u64) as u128; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = ((rbx as u32)) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; xmm6 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((if f64::from_bits(xmm0 as u64) < f64::from_bits(xmm6 as u64) { f64::from_bits(xmm0 as u64) } else { f64::from_bits(xmm6 as u64) }).to_bits() as u128); fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 4; continue; } bb = 5; continue; }
            5 => { rdx = ((rdi.wrapping_add(P1)) as u64) as u64; bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(rax, ((xmm6 as u64)) as u64); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu32(rax.wrapping_add(P0), (0x3u32) as u32); rax = 0x1; crate::simd_ops::lk_add_u64(rdx, (0x18u64) as u64); rsp = ((rsp).wrapping_add(0x50u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { rdx = 0x1; rcx = rdi; r8 = ((rdx.wrapping_add(P5)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1421086b0, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421086c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1429868d0; const P2: u64 = 0x8; const P3: u64 = 0x1420ecd00; const P4: u64 = 0x142646720; const P5: u64 = 0x2; const P6: u64 = 0x1421158a0; const P7: u64 = 0x1;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = P1; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = rax; fl_r = rdx; rbx = rcx; rcx = rdi; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rcx }) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_r = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rsi); rdi = (if (((fl_l as u64)) < ((fl_r as u64))) { crate::simd_ops::ru64(rbx.wrapping_add(P0)) } else { rdi }) as u64; rsi = (crate::simd_ops::cvt_f64_i32((f64::from_bits(crate::simd_ops::ru64(rcx))))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = rsp.wrapping_add(0x20); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { xmm0 = ((crate::simd_ops::ru64(rdi)) as u64) as u128; rdx = ((rsi as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rax, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rax.wrapping_add(P0), (0x3u32) as u32); rax = 0x1; crate::simd_ops::lk_add_u64(rbx.wrapping_add(P2), (0x18u64) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { rdx = 0x1; rcx = rbx; r8 = ((rdx.wrapping_add(P5)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x142108779, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { rdx = 0x2; rcx = rbx; r8 = ((rdx.wrapping_add(P7)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x14210878b, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421087a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1429868d0; const P2: u64 = 0x8; const P3: u64 = 0x1420ecd00; const P4: u64 = 0x1426465b0; const P5: u64 = 0x2; const P6: u64 = 0x1421158a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rcx; rcx = P1; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rcx }) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { xmm0 = ((crate::simd_ops::ru64(rcx)) as u64) as u128; rdx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; crate::simd_ops::wu64(rax, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rax.wrapping_add(P0), (0x3u32) as u32); crate::simd_ops::lk_add_u64(rbx.wrapping_add(P2), (0x18u64) as u64); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((crate::simd_ops::ru32(rsp.wrapping_add(0x50u64))) as i32) as f64).to_bits() as u128); crate::simd_ops::wu64(rax, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rax.wrapping_add(P0), (0x3u32) as u32); rax = 0x2; crate::simd_ops::lk_add_u64(rbx.wrapping_add(P2), (0x18u64) as u64); rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { rdx = 0x1; rcx = rbx; r8 = ((rdx.wrapping_add(P5)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x142108831, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142108840(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1429868d0; const P2: u64 = 0x8; const P3: u64 = 0x1420ecd00; const P4: u64 = 0x143426920; const P5: u64 = 0x2; const P6: u64 = 0x1421158a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rcx; rcx = P1; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rcx }) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; xmm0 = ((crate::simd_ops::ru64(rcx)) as u64) as u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) * f64::from_bits(crate::simd_ops::ru64(P4))).to_bits() as u128); crate::simd_ops::wu64(rax, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rax.wrapping_add(P0), (0x3u32) as u32); rax = 0x1; crate::simd_ops::lk_add_u64(rbx.wrapping_add(P2), (0x18u64) as u64); rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { rdx = 0x1; rcx = rbx; r8 = ((rdx.wrapping_add(P5)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1421088b0, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421088c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1429868d0; const P2: u64 = 0x8; const P3: u64 = 0x1420ecd00; const P4: u64 = 0x143426920; const P5: u64 = 0x2; const P6: u64 = 0x1421158a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rcx; rcx = P1; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rcx }) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; xmm0 = ((crate::simd_ops::ru64(rcx)) as u64) as u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) / f64::from_bits(crate::simd_ops::ru64(P4))).to_bits() as u128); crate::simd_ops::wu64(rax, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rax.wrapping_add(P0), (0x3u32) as u32); rax = 0x1; crate::simd_ops::lk_add_u64(rbx.wrapping_add(P2), (0x18u64) as u64); rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { rdx = 0x1; rcx = rbx; r8 = ((rdx.wrapping_add(P5)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x142108930, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142108a40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1429868d0; const P1: u64 = 0x1420ecd00; const P2: u64 = 0x143426ac0; const P3: u64 = 0x1425476ca; const P4: u64 = 0x143426b90; const P5: u64 = 0x14183ad5e; const P6: u64 = 0x14183ad7c; const P7: u64 = 0x1421158a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rdi = P0; rbx = rcx; rcx = rdi; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_r = rdx; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { crate::simd_ops::ru64(rbx.wrapping_add(0x10u64)) } else { rcx }) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm6) as u128); rax = ((rax).wrapping_add(0x18u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm7) as u128); xmm6 = ((crate::simd_ops::ru64(rcx)) as u64) as u128; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 15; continue; } bb = 4; continue; }
            4 => { fl_l = rax; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 15; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = rsp.wrapping_add(0x20); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 8; continue; }
            8 => { xmm7 = ((crate::simd_ops::ru64(rax)) as u64) as u128; fl_l = ((xmm7 as u64)) as u64; fl_r = (crate::simd_ops::ru64(P2)) as u64; if (f64::from_bits(fl_l).is_nan() || f64::from_bits(fl_r).is_nan()) { bb = 11; continue; } bb = 9; continue; }
            9 => { if (f64::from_bits(fl_l) != f64::from_bits(fl_r) && !f64::from_bits(fl_l).is_nan() && !f64::from_bits(fl_r).is_nan()) { bb = 11; continue; } bb = 10; continue; }
            10 => { xmm0 = (xmm6) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 16; continue; }
            11 => { fl_l = ((xmm7 as u64)) as u64; fl_r = (crate::simd_ops::ru64(P4)) as u64; if (f64::from_bits(fl_l).is_nan() || f64::from_bits(fl_r).is_nan()) { bb = 14; continue; } bb = 12; continue; }
            12 => { if (f64::from_bits(fl_l) != f64::from_bits(fl_r) && !f64::from_bits(fl_l).is_nan() && !f64::from_bits(fl_r).is_nan()) { bb = 14; continue; } bb = 13; continue; }
            13 => { xmm0 = (xmm6) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 16; continue; }
            14 => { xmm0 = (xmm6) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm6 = (xmm0) as u128; xmm0 = (xmm7) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (xmm6 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm6 as u64) / f64::from_bits(xmm0 as u64)).to_bits() as u128); bb = 16; continue; }
            15 => { xmm0 = (xmm6) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu64(rax, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rax.wrapping_add(0x10u64), (0x3u32) as u32); rax = 0x1; crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0x18u64) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            17 => { rdx = 0x2; rcx = rbx; r8 = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x142108b60, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            18 => { rdx = 0x1; rcx = rbx; r8 = ((rdx.wrapping_add(0x2u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x142108b72, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142108c70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1429868d0; const P2: u64 = 0x8; const P3: u64 = 0x1420ecd00; const P4: u64 = 0x1425476dc; const P5: u64 = 0x2; const P6: u64 = 0x1421158a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rcx; rcx = P1; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rcx }) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { xmm1 = ((crate::simd_ops::ru64(rcx)) as u64) as u128; xmm0 = 0u128; fl_l = ((xmm0 as u64)) as u64; fl_r = ((xmm1 as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; if (f64::from_bits(fl_l) > f64::from_bits(fl_r)) { bb = 4; continue; } bb = 3; continue; }
            3 => { xmm0 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64)).sqrt().to_bits() as u128); bb = 5; continue; }
            4 => { xmm0 = (xmm1) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rdi, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rdi.wrapping_add(P0), (0x3u32) as u32); rax = 0x1; crate::simd_ops::lk_add_u64(rbx.wrapping_add(P2), (0x18u64) as u64); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { rdx = 0x1; rcx = rbx; r8 = ((rdx.wrapping_add(P5)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x142108cfb, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142108d10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1429868d0; const P2: u64 = 0x8; const P3: u64 = 0x1420ecd00; const P4: u64 = 0x1426465a8; const P5: u64 = 0x2; const P6: u64 = 0x1421158a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rcx; rcx = P1; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rcx }) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { xmm0 = ((crate::simd_ops::ru64(rcx)) as u64) as u128; rdx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64) as u128; crate::simd_ops::wu64(rax, ((xmm1 as u64)) as u64); crate::simd_ops::wu32(rax.wrapping_add(P0), (0x3u32) as u32); crate::simd_ops::lk_add_u64(rbx.wrapping_add(P2), (0x18u64) as u64); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; crate::simd_ops::wu64(rax, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rax.wrapping_add(P0), (0x3u32) as u32); rax = 0x2; crate::simd_ops::lk_add_u64(rbx.wrapping_add(P2), (0x18u64) as u64); rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { rdx = 0x1; rcx = rbx; r8 = ((rdx.wrapping_add(P5)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x142108d9d, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142108ea0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1429868d0; const P2: u64 = 0x8; const P3: u64 = 0x1420ecd00; const P4: u64 = 0x2; const P5: u64 = 0x1421158a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rcx; rcx = P1; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rcx }) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; xmm0 = 0u128; xmm0 = crate::simd_ops::vroundsd(xmm0, crate::simd_ops::ru64(rcx), 0x1u8); crate::simd_ops::wu64(rax, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rax.wrapping_add(P0), (0x3u32) as u32); rax = 0x1; crate::simd_ops::lk_add_u64(rbx.wrapping_add(P2), (0x18u64) as u64); rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { rdx = 0x1; rcx = rbx; r8 = ((rdx.wrapping_add(P4)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x142108f0e, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142108f20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1429868d0; const P2: u64 = 0x8; const P3: u64 = 0x1420ecd00; const P4: u64 = 0x2; const P5: u64 = 0x1421158a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rcx; rcx = P1; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rcx }) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; xmm0 = 0u128; xmm0 = crate::simd_ops::vroundsd(xmm0, crate::simd_ops::ru64(rcx), 0x2u8); crate::simd_ops::wu64(rax, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rax.wrapping_add(P0), (0x3u32) as u32); rax = 0x1; crate::simd_ops::lk_add_u64(rbx.wrapping_add(P2), (0x18u64) as u64); rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { rdx = 0x1; rcx = rbx; r8 = ((rdx.wrapping_add(P4)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x142108f8e, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142109510(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1429868d0; const P2: u64 = 0x8; const P3: u64 = 0x1420ecd00; const P4: u64 = 0x143428730; const P5: u64 = 0x2; const P6: u64 = 0x1421158a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rcx; rcx = P1; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rcx }) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; xmm0 = ((crate::simd_ops::ru64(rcx)) as u64) as u128; xmm0 = (xmm0) & (crate::simd_ops::ru128(P4)); crate::simd_ops::wu64(rax, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rax.wrapping_add(P0), (0x3u32) as u32); rax = 0x1; crate::simd_ops::lk_add_u64(rbx.wrapping_add(P2), (0x18u64) as u64); rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { rdx = 0x1; rcx = rbx; r8 = ((rdx.wrapping_add(P5)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x142109580, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142109590(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (0x5851f42d4c957f2du64) as u64;
    rdx = (((rdx) as i64).wrapping_mul((rax) as i64) as u64) as u64;
    rax = (0x399d2694695129deu64) as u64;
    fl_l = ((rdx).wrapping_add(rax)) as u64; fl_r = fl_l;
    rdx = ((fl_l as u64)) as u64;
    crate::simd_ops::wu64(rcx, rdx);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421095c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    r8 = (0x5851f42d4c957f2du64) as u64;
    rax = rdx;
    rax = (((rax) as i64).wrapping_mul((r8) as i64) as u64) as u64;
    rax = ((rax).wrapping_add(0x69u64)) as u64;
    crate::simd_ops::wu64(rcx, rax);
    rax = rdx;
    rcx = rdx;
    rax = ((rax).wrapping_shr(45u32)) as u64;
    rdx = ((rdx).wrapping_shr(27u32)) as u64;
    rax = ((((rax as u32)) ^ ((rdx as u32)))) as u32 as u64;
    rcx = ((rcx).wrapping_shr(59u32)) as u64;
    rax = (((rax as u32)).rotate_right(((rcx as u32) & 31))) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142109640(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1429868d0; const P1: u64 = 0x142116d40; const P2: u64 = 0x1420fc210; const P3: u64 = 0x14210c770; const P4: u64 = 0x1426cdcb0; const P5: u64 = 0x142103330;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rbx = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); rsi = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); crate::simd_ops::wu32(rax.wrapping_add(0x10u64), (0x0u32) as u32); fl_l = (crate::simd_ops::lk_add_u64(rcx.wrapping_add(0x8u64), (0x18u64) as u64)) as u64; fl_r = fl_l; bb = 1; continue; }
            1 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as u32) as u32 as u64; rdi = rbx.wrapping_add(0x18); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x1u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rbx); bb = 3; continue; }
            3 => { rdx = 0xffffd8ee; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = rbx; r8 = ((r8).wrapping_sub(0x18u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 4; continue; }
            4 => { rax = rcx.wrapping_add(0x18); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); fl_l = rcx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x6; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x4u64), (0x1u8) as u8); bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0xffffffffffffffe8u64) as u64)) as u64; fl_r = fl_l; bb = 1; continue; }
            8 => { rax = rcx.wrapping_sub(0x18); rsi = rdi; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); rcx = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x40u64))) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x48u64))) as u64; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rsi = rbx.wrapping_add(0x18); bb = 10; continue; }
            10 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rbp = rbx.wrapping_add(0x18); fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x1u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rbx); bb = 13; continue; }
            12 => { rbp = rsi; bb = 13; continue; }
            13 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = P4; r8 = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rax); crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), (0x5u32) as u32); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0x18u64) as u64); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as u32) as u32 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru64(rsi)) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x1u64), ((rax as u8)) as u8); rsi = rbp; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rbx); bb = 15; continue; }
            15 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (((crate::simd_ops::ru32(rdx.wrapping_sub(0x8u64))) as i32 as i64) as u64) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x6; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 19; continue; } bb = 16; continue; }
            16 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x8; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 18; continue; } bb = 17; continue; }
            17 => { rcx = rax; rax = (crate::simd_ops::ru64(rsi)) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0xaa0u64))) as u64; bb = 20; continue; }
            18 => { rax = (crate::simd_ops::ru64(rdx.wrapping_sub(0x18u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; bb = 20; continue; }
            19 => { rax = (crate::simd_ops::ru64(rdx.wrapping_sub(0x18u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; bb = 20; continue; }
            20 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { crate::simd_ops::wu64(rdx, r8); crate::simd_ops::wu32(rdx.wrapping_add(0x10u64), (0x6u32) as u32); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = ((rdx).wrapping_add(0x18u64)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rdx); bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru64(rdx.wrapping_sub(0x18u64))) as u64; rcx = rbx; rdx = 0xffffd8ee; crate::simd_ops::wu8(rax.wrapping_add(0x4u64), (0x1u8) as u8); crate::simd_ops::lk_sub_u64(rbx.wrapping_add(0x8u64), (0x30u64) as u64); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = 0xffffd8ee; rcx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(0x4u64), (0x1u8) as u8); rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(0x5u64), (0x1u8) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142109810(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1433e0988; const P1: u64 = 0x141026600; const P2: u64 = 0x142646900; const P3: u64 = 0x142109880;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdx = P0;
    rbx = r8;
    rdi = rcx;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = rdi;
    rcx = 0x3a;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp);
    rdx = rbx;
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rdx = rdi;
    rcx = 0x2c;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp);
    rdx = rdi;
    rcx = 0xa;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142109880(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140000000; const P1: u64 = 0x1433e0958; const P2: u64 = 0x141026600; const P3: u64 = 0x14210a610; const P4: u64 = 0x1433e0954; const P5: u64 = 0x14210a3a0; const P6: u64 = 0x1433e08d0; const P7: u64 = 0x1433e08c8; const P8: u64 = 0x1433e0988; const P9: u64 = 0x1433e08b8; const P10: u64 = 0x1433e08a8; const P11: u64 = 0x1433e0898; const P12: u64 = 0x14210a680; const P13: u64 = 0x1433e091c; const P14: u64 = 0x1433e0868; const P15: u64 = 0x1433e0900; const P16: u64 = 0x142109e50; const P17: u64 = 0x1433e07b0; const P18: u64 = 0x1433e0830; const P19: u64 = 0x1433e0820; const P20: u64 = 0x1433e07a0; const P21: u64 = 0x1433e0790; const P22: u64 = 0x142646900; const P23: u64 = 0x1430782bc; const P24: u64 = 0x1430782b4; const P25: u64 = 0x1433e0760; const P26: u64 = 0x1433e0750; const P27: u64 = 0x1433e08f8;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rbx = rdx; rax = (((rax as u32)).wrapping_add(0xfffffffbu32)) as u32 as u64; rdi = rcx; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x6; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 37; continue; } bb = 1; continue; }
            1 => { rcx = P0; rax = (rax as u32 as i32 as i64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); r8 = (crate::simd_ops::ru32(rcx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x2109c24u64))) as u32 as u64; fl_l = ((r8).wrapping_add(rcx)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; j_tgt = r8; bb = 0xffff_fffe; continue; }
            2 => { r9 = (crate::simd_ops::ru32(rdx.wrapping_add(0x14u64))) as u32 as u64; rcx = rdi; r8 = ((crate::simd_ops::ru8(rdx.wrapping_add(0x2u64))) as u32) as u32 as u64; r9 = (((r9 as u32)).wrapping_add(0x19u32)) as u32 as u64; rdx = P1; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x14u64))) as u32 as u64; rdx = rbx.wrapping_add(0x18); rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = P4; bb = 36; continue; }
            3 => { rcx = rdi; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P5, rax, rcx, rdx, r8, r9, rsp); }
            4 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x4u64))) as u32) as u32 as u64; rdx = 0x20; fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x3u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r8 = ((crate::simd_ops::ru8(rbx.wrapping_add(0x2u64))) as u32) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u32) as u32 as u64; rax = 0x30; rax = (if (((fl_l as u8)) == ((fl_r as u8))) { (rdx as u32) } else { (rax as u32) }) as u32 as u64; rdx = P6; r9 = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u32) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = P7; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdx = P8; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x3u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 9; continue; } bb = 5; continue; }
            5 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = P9; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x4u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 35; continue; } bb = 8; continue; }
            8 => { rdx = P10; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = rbx.wrapping_add(0x30); bb = 13; continue; }
            9 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { r8 = ((r8).wrapping_add(0x18u64)) as u64; rdx = P9; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rdx = P11; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rdx = P8; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x4u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 35; continue; } bb = 12; continue; }
            12 => { rdx = P10; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = rbx.wrapping_add(0x20); bb = 13; continue; }
            13 => { r8 = ((crate::simd_ops::ru8(rbx.wrapping_add(0x4u64))) as u32) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rdx = P13; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 35; continue; }
            14 => { r9 = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rcx = rdi; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x3u64))) as u32) as u32 as u64; r9 = (((r9 as u32)).wrapping_add(0x10u32)) as u32 as u64; r8 = ((crate::simd_ops::ru8(rdx.wrapping_add(0x2u64))) as u32) as u32 as u64; rdx = P14; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 35; continue; } bb = 15; continue; }
            15 => { rdx = P15; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; bb = 34; continue; }
            16 => { rcx = rdi; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P16, rax, rcx, rdx, r8, r9, rsp); }
            17 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x68u64))) as u32 as u64; rax = (((rax as u32)).wrapping_add(crate::simd_ops::ru32(rdx.wrapping_add(0x70u64)))) as u32 as u64; r8 = ((crate::simd_ops::ru8(rdx.wrapping_add(0x2u64))) as u32) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u32) as u32 as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x60u64))) as u32 as u64; rcx = (((rcx as u32)).wrapping_add(crate::simd_ops::ru32(rdx.wrapping_add(0x6cu64)))) as u32 as u64; rax = (((rax as u32)).wrapping_add(0x22u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_add(crate::simd_ops::ru32(rdx.wrapping_add(0x64u64)))) as u32 as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(2))) as u32) as u32 as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x74u64))) as u32 as u64; rdx = P17; r9 = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u32) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbp = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 22; continue; } bb = 18; continue; }
            18 => { rdx = P18; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rcx = rdi; r8 = (crate::simd_ops::ru32(rdx.wrapping_add(0x14u64))) as u32 as u64; rdx = ((rdx).wrapping_add(0x18u64)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { r8 = (crate::simd_ops::ru32(r8)) as u32 as u64; bb = 21; continue; }
            20 => { r8 = ((rbp as u32)) as u32 as u64; bb = 21; continue; }
            21 => { rdx = P19; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 22; continue; }
            22 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x70u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 24; continue; } bb = 23; continue; }
            23 => { rdx = P20; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r8 = (((crate::simd_ops::ru32(rbx.wrapping_add(0x70u64))) as i32 as i64) as u64) as u64; rcx = rdi; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rdx = P13; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 24; continue; }
            24 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x64u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 35; continue; } bb = 25; continue; }
            25 => { rdx = P21; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x64u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 31; continue; } bb = 26; continue; }
            26 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); rsi = rbp; bb = 27; continue; }
            27 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 28; continue; }
            28 => { rdx = rdi; rcx = 0x2c; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P22), rax), rax, rcx, rdx, r8, r9, rsp); bb = 29; continue; }
            29 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rdx = P8; rcx = rdi; r8 = (crate::simd_ops::ru64(r8.wrapping_add(rsi.wrapping_mul(8)))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbp = (((rbp as u32)).wrapping_add(1)) as u32 as u64; rsi = ((rsi).wrapping_add(1)) as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x64u64))) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 27; continue; } bb = 30; continue; }
            30 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 31; continue; }
            31 => { rdx = P13; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 35; continue; }
            32 => { r8 = ((crate::simd_ops::ru8(rbx.wrapping_add(0x2u64))) as u32) as u32 as u64; rax = rdx.wrapping_add(0x10); fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_r = rax; rdx = P23; rcx = P24; r9 = 0x28; rcx = (if (((fl_l as u64)) != ((fl_r as u64))) { rdx } else { rcx }) as u64; rdx = P25; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x5; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 35; continue; } bb = 33; continue; }
            33 => { rdx = P26; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; r8 = (crate::simd_ops::ru64(r8)) as u64; bb = 34; continue; }
            34 => { rdx = P8; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 35; continue; }
            35 => { rdx = P27; bb = 36; continue; }
            36 => { rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 37; continue; }
            37 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x142109880u64) { 0x1f => { bb = 1; continue; } 0x3b => { bb = 2; continue; } 0x73 => { bb = 3; continue; } 0x8a => { bb = 4; continue; } 0xe2 => { bb = 5; continue; } 0xeb => { bb = 6; continue; } 0xfa => { bb = 7; continue; } 0x104 => { bb = 8; continue; } 0x119 => { bb = 9; continue; } 0x126 => { bb = 10; continue; } 0x139 => { bb = 11; continue; } 0x165 => { bb = 12; continue; } 0x178 => { bb = 13; continue; } 0x199 => { bb = 14; continue; } 0x1c8 => { bb = 15; continue; } 0x1e0 => { bb = 16; continue; } 0x1f7 => { bb = 17; continue; } 0x232 => { bb = 18; continue; } 0x25e => { bb = 19; continue; } 0x263 => { bb = 20; continue; } 0x266 => { bb = 21; continue; } 0x275 => { bb = 22; continue; } 0x27a => { bb = 23; continue; } 0x2a8 => { bb = 24; continue; } 0x2b1 => { bb = 25; continue; } 0x2c5 => { bb = 26; continue; } 0x2d0 => { bb = 27; continue; } 0x2d5 => { bb = 28; continue; } 0x2e3 => { bb = 29; continue; } 0x304 => { bb = 30; continue; } 0x309 => { bb = 31; continue; } 0x31a => { bb = 32; continue; } 0x35d => { bb = 33; continue; } 0x373 => { bb = 34; continue; } 0x382 => { bb = 35; continue; } 0x389 => { bb = 36; continue; } 0x396 => { bb = 37; continue; } _ => { return crate::simd_ops::tail_stk(j_tgt, rax, rcx, rdx, r8, r9, rsp); } } }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142109c50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x2; const P1: u64 = 0x10; const P2: u64 = 0x8; const P3: u64 = 0x1430782b4; const P4: u64 = 0x1430782bc; const P5: u64 = 0x1433e0760; const P6: u64 = 0x141026600; const P7: u64 = 0x1433e0750; const P8: u64 = 0x1433e0988; const P9: u64 = 0x1433e08f8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r8 = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64; rax = ((rdx.wrapping_add(P1)) as u64) as u64; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; fl_r = rax; r9 = P3; rbx = rcx; rdi = rdx; rcx = P4; r9 = (if (((fl_l as u64)) != ((fl_r as u64))) { rcx } else { r9 }) as u64; rdx = P5; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); rcx = rbx; r9 = 0x28; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64; fl_r = 0x5; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = P7; rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; rdx = P8; rcx = rbx; r8 = (crate::simd_ops::ru64(r8)) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rdx = P9; rcx = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P6, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142109cf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1433e07b0; const P1: u64 = 0x141026600; const P2: u64 = 0x1433e0830; const P3: u64 = 0x14210a610; const P4: u64 = 0x1433e0820; const P5: u64 = 0x1433e07a0; const P6: u64 = 0x14210a680; const P7: u64 = 0x1433e091c; const P8: u64 = 0x1433e0790; const P9: u64 = 0x142646900; const P10: u64 = 0x1433e0988; const P11: u64 = 0x1433e08f8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x68u64))) as u32 as u64; rbx = rcx; rax = (((rax as u32)).wrapping_add(crate::simd_ops::ru32(rdx.wrapping_add(0x70u64)))) as u32 as u64; rsi = rdx; r8 = ((crate::simd_ops::ru8(rdx.wrapping_add(0x2u64))) as u32) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u32) as u32 as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x60u64))) as u32 as u64; rcx = (((rcx as u32)).wrapping_add(crate::simd_ops::ru32(rdx.wrapping_add(0x6cu64)))) as u32 as u64; rax = (((rax as u32)).wrapping_add(0x22u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_add(crate::simd_ops::ru32(rdx.wrapping_add(0x64u64)))) as u32 as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(2))) as u32) as u32 as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x74u64))) as u32 as u64; rdx = P0; r9 = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u32) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbp = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x40u64))) as u64; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rdx = P2; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x40u64))) as u64; rcx = rbx; r8 = (crate::simd_ops::ru32(rdx.wrapping_add(0x14u64))) as u32 as u64; rdx = ((rdx).wrapping_add(0x18u64)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru32(r8)) as u32 as u64; bb = 4; continue; }
            3 => { r8 = ((rbp as u32)) as u32 as u64; bb = 4; continue; }
            4 => { rdx = P4; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x70u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = P5; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r8 = (((crate::simd_ops::ru32(rsi.wrapping_add(0x70u64))) as i32 as i64) as u64) as u64; rcx = rbx; rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdx = P7; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x64u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 15; continue; } bb = 8; continue; }
            8 => { rdx = P8; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x64u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 14; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rdi = rbp; bb = 10; continue; }
            10 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rdx = rbx; rcx = 0x2c; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { r8 = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; rdx = P10; rcx = rbx; r8 = (crate::simd_ops::ru64(r8.wrapping_add(rdi.wrapping_mul(8)))) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbp = (((rbp as u32)).wrapping_add(1)) as u32 as u64; rdi = ((rdi).wrapping_add(1)) as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsi.wrapping_add(0x64u64))) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 10; continue; } bb = 13; continue; }
            13 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 14; continue; }
            14 => { rdx = P7; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            15 => { rdx = P11; rcx = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142109e50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1433e0840; const P1: u64 = 0x141026600; const P2: u64 = 0x1433e08c8; const P3: u64 = 0x1433e0988; const P4: u64 = 0x1433e0830; const P5: u64 = 0x14210a610; const P6: u64 = 0x1433e0820; const P7: u64 = 0x1433e0810; const P8: u64 = 0x142646900; const P9: u64 = 0x1433e091c; const P10: u64 = 0x1433e0800; const P11: u64 = 0x1433de094; const P12: u64 = 0x1433e07f0; const P13: u64 = 0x1433e07e8; const P14: u64 = 0x1426cdcb0; const P15: u64 = 0x1433e07e0; const P16: u64 = 0x1433c7abc; const P17: u64 = 0x1433e08f8; const P18: u64 = 0x1433e07d4;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4cu64))) as u32 as u64; r15 = rdx; rdi = rcx; r9 = ((rax.wrapping_add(rax.wrapping_mul(4))) as u32) as u32 as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x48u64))) as u32 as u64; r9 = ((r9.wrapping_add(0x10u64)) as u32) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(3u32)) as u32 as u64; r9 = ((r8.wrapping_add(r9.wrapping_mul(8))) as u32) as u32 as u64; r8 = ((crate::simd_ops::ru8(rdx.wrapping_add(0x2u64))) as u32) as u32 as u64; rdx = P0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = P2; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(r15.wrapping_add(0x58u64))) as u64; rdx = P3; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(r15.wrapping_add(0x40u64))) as u64; rdx = (crate::simd_ops::ru64(r15.wrapping_add(0x20u64))) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 9; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x7; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = ((rax).wrapping_add(0x28u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { bb = 9; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x3u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = P4; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rcx = rdi; r8 = (crate::simd_ops::ru32(rdx.wrapping_add(0x14u64))) as u32 as u64; rdx = ((rdx).wrapping_add(0x18u64)) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x7cu64))) as u32 as u64; rdx = P6; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(r15.wrapping_add(0x30u64))) as u64; fl_l = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 47; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbp); rdx = P7; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rsi); rcx = rdi; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r13); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; r13 = (0x2aaaaaaaaaaaaaabu64) as u64; rbx = (crate::simd_ops::ru64(r15.wrapping_add(0x30u64))) as u64; rax = r13; rcx = ((rcx).wrapping_sub(rbx)) as u64; rbp = (rbp & !0xffu64) | (((0x1u8) as u8 as u64)); let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rsi = rdx; rsi = ((((rsi) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rsi; rax = ((rax).wrapping_shr(63u32)) as u64; fl_l = ((rsi).wrapping_add(rax)) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 16; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x5; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 15; continue; } bb = 12; continue; }
            12 => { fl_l = ((rbp as u8)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rdx = rdi; rcx = 0x2c; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { r8 = (crate::simd_ops::ru64(rbx)) as u64; rdx = P3; rbp = (rbp & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            15 => { rbx = ((rbx).wrapping_add(0x18u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 16; continue; }
            16 => { rdx = P9; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(r15.wrapping_add(0x40u64))) as u64; rdx = P10; rcx = rdi; r12 = (r12 & !0xffu64) | (((0x1u8) as u8 as u64)); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rsi = (crate::simd_ops::ru64(r15.wrapping_add(0x30u64))) as u64; fl_l = rsi; fl_r = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 46; continue; } bb = 17; continue; }
            17 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x5; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 44; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(r15.wrapping_add(0x20u64))) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 23; continue; } bb = 20; continue; }
            20 => { bb = 21; continue; }
            21 => { fl_l = rsi; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rbx = ((rbx).wrapping_add(0x28u64)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 21; continue; } bb = 23; continue; }
            23 => { fl_l = ((r12 as u8)) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 25; continue; } bb = 24; continue; }
            24 => { rdx = rdi; rcx = 0x2c; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; r12 = (r12 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 33; continue; } bb = 26; continue; }
            26 => { rcx = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x3u64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 28; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x28u64))) as u64; r8 = P11; fl_l = rax; fl_r = rax; rdx = P12; rcx = rdi; r8 = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { r8 }) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 44; continue; }
            28 => { rbp = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; rdx = P13; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x40u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 29; continue; }
            29 => { r8 = (crate::simd_ops::ru32(rdx.wrapping_add(0x14u64))) as u32 as u64; rcx = rdi; rdx = ((rdx).wrapping_add(0x18u64)) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 30; continue; }
            30 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x48u64))) as u64; r9 = rax.wrapping_add(0x18); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 32; continue; } bb = 31; continue; }
            31 => { r9 = P14; bb = 32; continue; }
            32 => { r8 = (crate::simd_ops::ru32(rbp.wrapping_add(0x7cu64))) as u32 as u64; rdx = P15; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 44; continue; }
            33 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x7; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 43; continue; } bb = 34; continue; }
            34 => { rcx = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x3u64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 43; continue; } bb = 35; continue; }
            35 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 37; continue; } bb = 36; continue; }
            36 => { r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rax.wrapping_add(0x10u64)))) as u64; r8 = ((((r8) as i64).wrapping_shr(2u32) as u64)) as u64; fl_l = (((r8 as u32)).wrapping_sub(1)) as u32 as u64; r8 = ((fl_l as u32)) as u32 as u64; bb = 37; continue; }
            37 => { rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x68u64))) as i32 as i64) as u64) as u64; r10 = 0; fl_l = 0; fl_r = 0; r9 = 0; fl_l = 0; fl_r = 0; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 43; continue; } bb = 38; continue; }
            38 => { r14 = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; rdx = rsi; rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(rbx))) as u64; rax = r13; let prod = (rax as i64 as i128).wrapping_mul((rdx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; r11 = rcx; rcx = r14.wrapping_add(0x8); rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; fl_l = ((rdx).wrapping_add(rax)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 39; continue; }
            39 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x8u64))) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 42; continue; } bb = 40; continue; }
            40 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 42; continue; } bb = 41; continue; }
            41 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 48; continue; } bb = 42; continue; }
            42 => { r10 = (((r10 as u32)).wrapping_add(1)) as u32 as u64; r9 = ((r9).wrapping_add(1)) as u64; rcx = ((rcx).wrapping_add(0x18u64)) as u64; fl_l = r9; fl_r = r11; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 39; continue; } bb = 43; continue; }
            43 => { rdx = P16; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 44; continue; }
            44 => { rsi = ((rsi).wrapping_add(0x18u64)) as u64; fl_l = rsi; fl_r = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 18; continue; } bb = 45; continue; }
            45 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 46; continue; }
            46 => { rdx = P9; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 47; continue; }
            47 => { rdx = P17; rcx = rdi; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
            48 => { rax = ((((r10 as u32)) as i32 as i64) as u64) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((r14.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 43; continue; } bb = 49; continue; }
            49 => { r8 = (crate::simd_ops::ru64(r8)) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 43; continue; } bb = 50; continue; }
            50 => { r8 = ((r8).wrapping_add(0x18u64)) as u64; rdx = P18; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 44; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210a1e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x3; const P2: u64 = 0x2; const P3: u64 = 0x1433e0868; const P4: u64 = 0x141026600; const P5: u64 = 0x8; const P6: u64 = 0x1433e0900; const P7: u64 = 0x1433e0988; const P8: u64 = 0x1433e08f8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    r9 = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    rdi = rdx;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64;
    r9 = (((r9 as u32)).wrapping_add(0x10u32)) as u32 as u64;
    r8 = ((crate::simd_ops::ru8(rdx.wrapping_add(P2))) as u32) as u32 as u64;
    rbx = rcx;
    rdx = P3;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32);
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64; fl_r = (0x0u64) as u64;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rdx = P6; rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64; rdx = P7; rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
    rdx = P8;
    rcx = rbx;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P4, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210a260(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1433e08d0; const P1: u64 = 0x141026600; const P2: u64 = 0x1433e08c8; const P3: u64 = 0x1433e0988; const P4: u64 = 0x1433e08b8; const P5: u64 = 0x1433e08a8; const P6: u64 = 0x1433e0898; const P7: u64 = 0x14210a680; const P8: u64 = 0x1433e091c; const P9: u64 = 0x1433e08f8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x3u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rax = 0x20; r9 = 0x30; rdi = rdx; r9 = (if (((fl_l as u8)) == ((fl_r as u8))) { (rax as u32) } else { (r9 as u32) }) as u32 as u64; rbx = rcx; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x4u64))) as u32) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u32) as u32 as u64; r9 = ((r9.wrapping_add(r8.wrapping_mul(8))) as u32) as u32 as u64; r8 = ((crate::simd_ops::ru8(rdx.wrapping_add(0x2u64))) as u32) as u32 as u64; rdx = P0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = P2; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rdx = P3; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x3u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = P4; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x4u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 4; continue; }
            4 => { rdx = P5; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = rdi.wrapping_add(0x30); bb = 9; continue; }
            5 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { r8 = ((r8).wrapping_add(0x18u64)) as u64; rdx = P4; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rdx = P6; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rdx = P3; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x4u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rdx = P5; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = rdi.wrapping_add(0x20); bb = 9; continue; }
            9 => { r8 = ((crate::simd_ops::ru8(rdi.wrapping_add(0x4u64))) as u32) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rdx = P8; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rdx = P9; rcx = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210a3a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142986720; const P1: u64 = 0x1433e0930; const P2: u64 = 0x141026600; const P3: u64 = 0x1433e0920; const P4: u64 = 0x142646900; const P5: u64 = 0x1433e0988; const P6: u64 = 0x1433c7abc; const P7: u64 = 0x1433e091c; const P8: u64 = 0x1433e0910; const P9: u64 = 0x14210a680; const P10: u64 = 0x1433e0900; const P11: u64 = 0x1433e08f8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x38u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbp); rsi = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rdi); rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r12); rbx = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); r14 = P0; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rbp as u32)) as u32 as u64; bb = 3; continue; }
            2 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x6u64))) as u32) as u32 as u64; rcx = 0x30; rcx = (((rcx as u32)).wrapping_shl(((((rax as u32)) as u32) & 31))) as u32 as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; rdi = rdx.wrapping_add(0x8); r8 = ((crate::simd_ops::ru8(rdx.wrapping_add(0x2u64))) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_add(0x2u32)) as u32 as u64; rdx = P1; rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u32) as u32 as u64; r9 = ((rcx.wrapping_add(rax.wrapping_mul(8))) as u32) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r13); rdx = P3; rcx = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = ((crate::simd_ops::ru8(rsi.wrapping_add(0x6u64))) as u32) as u32 as u64; r13 = 0x1; rax = (((r13 as u32)).wrapping_shl(((((rax as u32)) as u32) & 31))) as u32 as u64; r15 = (r15 & !0xffu64) | (((0x1u8) as u8 as u64)); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 19; continue; } bb = 5; continue; }
            5 => { r14 = rbp; bb = 6; continue; }
            6 => { rdi = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; rdi = ((rdi).wrapping_add(r14)) as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 17; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x28u64))) as u32 as u64; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x5u8) as u8 as u64; if (((fl_l as u8)) >= ((fl_r as u8))) { bb = 9; continue; } bb = 8; continue; }
            8 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x5; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 17; continue; } bb = 9; continue; }
            9 => { fl_l = ((r15 as u8)) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdx = rbx; rcx = 0x2c; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x28u64))) as u32 as u64; r15 = (r15 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; rcx = rbx; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x5u8) as u8 as u64; if (((fl_l as u8)) < ((fl_r as u8))) { bb = 13; continue; } bb = 12; continue; }
            12 => { r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rdx = P5; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            13 => { rdx = P6; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { rdx = rbx; rcx = 0x2c; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x5; rcx = rbx; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 16; continue; } bb = 15; continue; }
            15 => { r8 = (crate::simd_ops::ru64(rdi)) as u64; rdx = P5; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            16 => { rdx = P6; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            17 => { rax = ((crate::simd_ops::ru8(rsi.wrapping_add(0x6u64))) as u32) as u32 as u64; rbp = (((rbp as u32)).wrapping_add(1)) as u32 as u64; rax = (((r13 as u32)).wrapping_shl(((((rax as u32)) as u32) & 31))) as u32 as u64; r14 = ((r14).wrapping_add(0x30u64)) as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 6; continue; } bb = 18; continue; }
            18 => { rdi = rsi.wrapping_add(0x8); bb = 19; continue; }
            19 => { rdx = P7; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_r = 0x0; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 22; continue; } bb = 21; continue; }
            21 => { rdx = P8; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r8 = (((crate::simd_ops::ru32(rdi)) as i32 as i64) as u64) as u64; rcx = rbx; rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rdx = P7; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 22; continue; }
            22 => { fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 24; continue; } bb = 23; continue; }
            23 => { rdx = P10; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; rdx = P5; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 24; continue; }
            24 => { rdx = P11; rcx = rbx; rsp = ((rsp).wrapping_add(0x38u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210a5b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14; const P1: u64 = 0x2; const P2: u64 = 0x1433e0958; const P3: u64 = 0x141026600; const P4: u64 = 0x18; const P5: u64 = 0x14210a610; const P6: u64 = 0x1433e0954;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    r9 = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    rbx = rdx;
    r8 = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64;
    r9 = (((r9 as u32)).wrapping_add(0x19u32)) as u32 as u64;
    rdx = P2;
    rdi = rcx;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    r8 = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64;
    rdx = ((rbx.wrapping_add(P4)) as u64) as u64;
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rdx = P6;
    rcx = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210a610(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646900;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = r8; rsi = rdx; rbp = rcx; rbx = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rax = (((crate::simd_ops::ru8(rsi.wrapping_add(rbx))) as i8 as i32) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x20u8) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x5cu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x22u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (rax & !0xffu64) | (((0x3fu8) as u8 as u64)); bb = 7; continue; }
            7 => { rcx = (((rax as u8)) as u32) as u32 as u64; rdx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(1)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 9; continue; }
            9 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210a680(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x142646900; const P2: u64 = 0x1433e0988; const P3: u64 = 0x141026600;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = r8; rbx = rdx; rbp = rcx; rsi = (rsi & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; fl_r = 0x5; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = rbp; rcx = 0x2c; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { r8 = (crate::simd_ops::ru64(rbx)) as u64; rdx = P2; rsi = (rsi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rcx = rbp; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rbx = ((rbx).wrapping_add(0x18u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 8; continue; }
            8 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210a700(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1433e0988; const P1: u64 = 0x141026600;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    r8 = rdx;
    rdx = P0;
    return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210a7a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1; const P1: u64 = 0x18; const P2: u64 = 0x21; const P3: u64 = 0x20; const P4: u64 = 0x8; const P5: u64 = 0x10; const P6: u64 = 0x14210c5f0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r8 = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x7u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rax = ((crate::simd_ops::ru8(r9.wrapping_add(P2))) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_sub(1)) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) <= ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = ((crate::simd_ops::ru8(r9.wrapping_add(P3))) as u32) as u32 as u64; r8 = (r8 & !0xffu64) | ((((((r8 as u8)) & (0xf8u8))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x3u8))) as u8 as u64)); fl_l = ((((rax as u8)) | ((r8 as u8)))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rdx.wrapping_add(P0), ((rax as u8)) as u8); return rax; }
            3 => { r9 = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64; r8 = (r8 & !0xffu64) | ((((((r8 as u8)) | (0x4u8))) as u8 as u64)); crate::simd_ops::wu8(rdx.wrapping_add(P0), ((r8 as u8)) as u8); fl_l = (crate::simd_ops::ru32(r9.wrapping_add(P5))) as u32 as u64; fl_r = 0x5; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 8; continue; } bb = 4; continue; }
            4 => { r9 = (crate::simd_ops::ru64(r9)) as u64; fl_l = (crate::simd_ops::ru8(r9.wrapping_add(P0))) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P2))) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_sub(1)) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) <= ((fl_r as u8))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P3))) as u32) as u32 as u64; r8 = (r8 & !0xffu64) | ((((((r8 as u8)) & (0xf8u8))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x3u8))) as u8 as u64)); fl_l = ((((rax as u8)) | ((r8 as u8)))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rdx.wrapping_add(P0), ((rax as u8)) as u8); return rax; }
            7 => { rdx = r9; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210a820(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14210ab30; const P1: u64 = 0x14210b360; const P2: u64 = 0x142103040;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; rdi = rcx; rcx = ((crate::simd_ops::ru8(rbx.wrapping_add(0x21u64))) as u32) as u32 as u64; rax = ((rcx.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) <= ((fl_r as u8))) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { bb = 5; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x260u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x268u64), rax); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), rax); crate::simd_ops::wu64(rbx.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), rax); crate::simd_ops::wu8(rbx.wrapping_add(0x21u64), (0x4u8) as u8); bb = 5; continue; }
            5 => { rdx = (0xffffffffffffffffu64) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x21u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0xa90u64))) as u64; rcx = rbx.wrapping_add(0xa78); fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { crate::simd_ops::wu8(rax.wrapping_add(0x3u64), (0x0u8) as u8); rax = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x21u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 12; continue; } bb = 10; continue; }
            10 => { bb = 11; continue; }
            11 => { rdx = (0xffffffffffffffffu64) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x21u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 11; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; r9 = (crate::simd_ops::ru32(rax.wrapping_add(0xcu64))) as u32 as u64; rcx = ((r9 as u32)) as u32 as u64; r8 = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; rax = ((r9 as u32)) as u32 as u64; rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64; rdx = ((((rdx as u32)) & (0x3u32))) as u32 as u64; rax = (((rax as u32)).wrapping_add((rdx as u32))) as u32 as u64; rax = (((((rax as u32)) as i32).wrapping_shr(2u32) as u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 17; continue; } bb = 13; continue; }
            13 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x40; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = ((rcx as u32)) as u32 as u64; rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64; rax = (((rax as u32)).wrapping_sub((rdx as u32))) as u32 as u64; rax = (((((rax as u32)) as i32).wrapping_shr(1u32) as u32)) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64; rdx = ((((rdx as u32)) & (0x3u32))) as u32 as u64; rax = (((rax as u32)).wrapping_add((rdx as u32))) as u32 as u64; rax = (((((rax as u32)) as i32).wrapping_shr(2u32) as u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 13; continue; } bb = 15; continue; }
            15 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rdx = ((rcx as u32)) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            17 => { r11 = (crate::simd_ops::ru64(rbx.wrapping_add(0x48u64))) as u64; rax = (0x47ae147ae147ae15u64) as u64; r8 = (((crate::simd_ops::ru32(rbx.wrapping_add(0x54u64))) as i32 as i64) as u64) as u64; r10 = r11; r9 = (((crate::simd_ops::ru32(rbx.wrapping_add(0x50u64))) as i32 as i64) as u64) as u64; rcx = ((r8 as u32)) as u32 as u64; let prod = (rax as u128).wrapping_mul((r11) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rcx = ((((rcx as u32)) as i32).wrapping_mul(((r9 as u32)) as i32) as u32) as u32 as u64; rax = 0x51eb851f; r10 = ((r10).wrapping_sub(rdx)) as u64; r10 = ((r10).wrapping_shr(1u32)) as u64; r10 = ((r10).wrapping_add(rdx)) as u64; r10 = ((r10).wrapping_shr(6u32)) as u64; let prod = (rax as u32 as i32 as i64).wrapping_mul(((rcx as u32)) as i32 as i64) as u64; rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; r10 = (((r10) as i64).wrapping_mul((r9) as i64) as u64) as u64; rdx = (((((rdx as u32)) as i32).wrapping_shr(5u32) as u32)) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add(0xffffff9cu32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(31u32)) as u32 as u64; rax = (((rax as u32)).wrapping_add((rdx as u32))) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = (rax as u32 as i32 as i64) as u64; rax = (((rax) as i64).wrapping_mul((r11) as i64) as u64) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x1128u64), r10); let d = (r8) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } crate::simd_ops::wu64(rbx.wrapping_add(0x40u64), rax); fl_l = rax; fl_r = r11; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 19; continue; } bb = 18; continue; }
            18 => { crate::simd_ops::wu64(rbx.wrapping_add(0x40u64), r11); bb = 19; continue; }
            19 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210a9a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1138; const P1: u64 = 0x1140; const P2: u64 = 0x143426858; const P3: u64 = 0x1118; const P4: u64 = 0x1120; const P5: u64 = 0x1110; const P6: u64 = 0x1128; const P7: u64 = 0x1130; const P8: u64 = 0x1090; const P9: u64 = 0x1114; const P10: u64 = 0x1434269e0; const P11: u64 = 0x1434269a0; const P12: u64 = 0x143426d38;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { xmm2 = ((crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64) as u128; xmm1 = (xmm2 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm2 as u64) - f64::from_bits(crate::simd_ops::ru64(rcx.wrapping_add(P1)))).to_bits() as u128); xmm0 = ((crate::simd_ops::ru64(P2)) as u64) as u128; fl_l = ((xmm0 as u64)) as u64; fl_r = ((xmm1 as u64)) as u64; r9 = rdx; r8 = rcx; if !(f64::from_bits(fl_l) > f64::from_bits(fl_r)) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = rdx; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { r10 = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; rcx = r10; fl_l = rcx; fl_r = (crate::simd_ops::ru64(r8.wrapping_add(P4))) as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; xmm0 = 0u128; if (((((fl_l as u64)).wrapping_sub((fl_r as u64))) as i64) < 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rcx) as i64) as f64).to_bits() as u128); bb = 5; continue; }
            4 => { rax = rcx; rcx = ((((rcx as u32)) & (0x1u32))) as u32 as u64; rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = (((rax) | (rcx))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) + f64::from_bits(xmm0 as u64)).to_bits() as u128); bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(r8.wrapping_add(P5))) as u32 as u64; r10 = ((r10).wrapping_sub(crate::simd_ops::ru64(r8.wrapping_add(P6)))) as u64; rax = ((((rax as u32)) & (0x1fu32))) as u32 as u64; xmm1 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) / f64::from_bits(xmm1 as u64)).to_bits() as u128); xmm0 = (xmm2 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm2 as u64) - f64::from_bits(crate::simd_ops::ru64(r8.wrapping_add(P7)))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm0 as u64)).to_bits() as u128); rcx = (crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(4)).wrapping_add(P8))) as u32 as u64; r10 = ((r10).wrapping_shr(10u32)) as u64; crate::simd_ops::wu32(r8.wrapping_add(rax.wrapping_mul(4)).wrapping_add(P8), ((r10 as u32)) as u32); rax = ((r10 as u32)) as u32 as u64; crate::simd_ops::lk_add_u32(r8.wrapping_add(P5), (1) as u32); rax = (((rax as u32)).wrapping_sub((rcx as u32))) as u32 as u64; crate::simd_ops::lk_add_u32(r8.wrapping_add(P9), ((rax as u32)) as u32); rcx = r9; rdx = (crate::simd_ops::cvt_f64_i64((f64::from_bits(xmm1 as u64)))) as u64; xmm0 = 0u128; xmm1 = 0u128; xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((((crate::simd_ops::ru32(r8.wrapping_add(P9))) as i32) as f64).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | (((((r10 as u32)) as i32) as f64).to_bits() as u128); xmm2 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) * f64::from_bits(crate::simd_ops::ru64(P10))).to_bits() as u128); xmm0 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(crate::simd_ops::ru64(P11))).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm2 as u64) + f64::from_bits(xmm0 as u64)).to_bits() as u128); xmm1 = (xmm2 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm2 as u64) * f64::from_bits(crate::simd_ops::ru64(P12))).to_bits() as u128); rax = (crate::simd_ops::cvt_f64_i64((f64::from_bits(xmm1 as u64)))) as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(0x48u64))) as u64; rcx = ((rcx).wrapping_sub(rdx)) as u64; fl_l = rcx; fl_r = rax; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 7; continue; } bb = 6; continue; }
            6 => { fl_l = rcx; fl_r = r9; rax = rcx; rax = (if ((((fl_l as u64)) as i64) > (((fl_r as u64)) as i64)) { r9 } else { rax }) as u64; bb = 7; continue; }
            7 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210aab0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1110; const P1: u64 = 0x1118; const P2: u64 = 0x1128; const P3: u64 = 0x1090; const P4: u64 = 0x1114; const P5: u64 = 0x1434269e0; const P6: u64 = 0x1434269a0; const P7: u64 = 0x143426d38;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    rax = ((((rax as u32)) & (0x1fu32))) as u32 as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rcx.wrapping_add(P2)))) as u64;
    r8 = ((r8).wrapping_shr(10u32)) as u64;
    xmm0 = 0u128;
    rdx = (crate::simd_ops::ru32(rcx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(P3), ((r8 as u32)) as u32);
    rax = ((r8 as u32)) as u32 as u64;
    crate::simd_ops::lk_add_u32(rcx.wrapping_add(P0), (1) as u32);
    rax = (((rax as u32)).wrapping_sub((rdx as u32))) as u32 as u64;
    fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(P4), ((rax as u32)) as u32)) as u32 as u64; fl_r = fl_l;
    xmm1 = 0u128;
    xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((((crate::simd_ops::ru32(rcx.wrapping_add(P4))) as i32) as f64).to_bits() as u128);
    xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | (((((r8 as u32)) as i32) as f64).to_bits() as u128);
    xmm2 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) * f64::from_bits(crate::simd_ops::ru64(P5))).to_bits() as u128);
    xmm0 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(crate::simd_ops::ru64(P6))).to_bits() as u128);
    xmm2 = (xmm2 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm2 as u64) + f64::from_bits(xmm0 as u64)).to_bits() as u128);
    xmm1 = (xmm2 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm2 as u64) * f64::from_bits(crate::simd_ops::ru64(P7))).to_bits() as u128);
    rax = (crate::simd_ops::cvt_f64_i64((f64::from_bits(xmm1 as u64)))) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210ada0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14210b5d0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r9 = (((crate::simd_ops::ru32(rdx.wrapping_add(0x24u64))) as i32 as i64) as u64) as u64; r8 = rdx.wrapping_add(0x38); rax = (((crate::simd_ops::ru32(rdx.wrapping_add(0x20u64))) as i32 as i64) as u64) as u64; r13 = rcx; rcx = (((crate::simd_ops::ru32(rdx.wrapping_add(0x30u64))) as i32 as i64) as u64) as u64; rax = ((rax).wrapping_sub(0x38u64)) as u64; rbp = rdx; rcx = ((rcx).wrapping_add(r9)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = ((rcx).wrapping_add(r8)) as u64; let d = (r9) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } r14 = r9; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rcx); rsi = (crate::simd_ops::ru32(rbp.wrapping_add(0x34u64))) as u32 as u64; rbx = rcx; rax = ((((rax as u32)) as i32).wrapping_mul(((r9 as u32)) as i32) as u32) as u32 as u64; rdi = ((((rax as u32)) as i32 as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_add(0x18u64))) as u64; rdi = ((rdi).wrapping_add(r8)) as u64; r15 = ((crate::simd_ops::ru8(rax.wrapping_add(0x20u64))) as u32) as u32 as u64; r12 = ((r15 as u32)) as u32 as u64; r15 = ((((r15 as u32)) & (0x3u32))) as u32 as u64; r12 = ((((r12 as u32)) ^ (0x3u32))) as u32 as u64; fl_l = rcx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rbx)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rcx = ((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as u32) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((((rax as u32)) ^ (0x3u32))) as u32 as u64; fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0xf8u8))) as u8 as u64)); fl_l = ((((rcx as u8)) | ((r15 as u8)))) as u8 as u64; fl_r = fl_l; rcx = (rcx & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x1u64), ((rcx as u8)) as u8); bb = 6; continue; }
            5 => { r8 = rbp; rdx = rbx; rcx = r13; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x1; rsi = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 6; continue; }
            6 => { rbx = ((rbx).wrapping_add(r14)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; r9 = ((r14 as u32)) as u32 as u64; bb = 8; continue; }
            8 => { rdi = (((rdi as u32)).wrapping_sub((rcx as u32))) as u32 as u64; rax = ((rdi as u32)) as u32 as u64; rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64; let d = ((r9 as u32)) as i32 as i64; let n = (((rdx as u32 as u64) << 32) | (rax as u32 as u64)) as i64; if d != 0 { rax = n.wrapping_div(d) as i32 as u32 as u64; rdx = n.wrapping_rem(d) as i32 as u32 as u64; } bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            10 => { rbx = (((rbx as u32)).wrapping_sub(crate::simd_ops::ru32(rsp.wrapping_add(0x68u64)))) as u32 as u64; rax = ((rbx as u32)) as u32 as u64; rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64; let d = ((r14 as u32)) as i32 as i64; let n = (((rdx as u32 as u64) << 32) | (rax as u32 as u64)) as i64; if d != 0 { rax = n.wrapping_div(d) as i32 as u32 as u64; rdx = n.wrapping_rem(d) as i32 as u32 as u64; } fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; bb = 9; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210ae90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14210c5f0; const P1: u64 = 0x14210bb20; const P2: u64 = 0x14210b3f0; const P3: u64 = 0x1420fcb60; const P4: u64 = 0x142646320; const P5: u64 = 0x1420fbac0; const P6: u64 = 0x14210ca90;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rbp = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r13); r15 = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rbp); rdi = ((r14 as u32)) as u32 as u64; r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0xa90u64))) as u64; r10 = rbp.wrapping_add(0xa78); fl_l = r9; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rdi = ((rdi).wrapping_add(0x28u64)) as u64; fl_l = (crate::simd_ops::ru8(r9.wrapping_add(0x1u64))) as u8 as u64; fl_r = (0x7u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(r9.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x5; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x1u64))) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = rbp; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { r9 = (crate::simd_ops::ru64(r9.wrapping_add(0x18u64))) as u64; fl_l = r9; fl_r = r10; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 7; continue; }
            7 => { rbx = r14; fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = rbp; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(rax)) as u64; fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64; r12 = ((rbx.wrapping_add(rdi)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); crate::simd_ops::wu64(rbp.wrapping_add(0x38u64), r14); fl_l = (crate::simd_ops::ru8(r15.wrapping_add(0x1u64))) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdx = r15; rcx = rbp; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rcx = rbp; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = r14; fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 12; continue; }
            12 => { bb = 13; continue; }
            13 => { rcx = rbp; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(rax)) as u64; fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64; r12 = ((r12).wrapping_add(rbx)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); rbx = r14; crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), r14); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = rbp; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(rax)) as u64; fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 16; continue; }
            16 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64; r12 = ((r12).wrapping_add(rbx)) as u64; r13 = r14; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 54; continue; } bb = 17; continue; }
            17 => { crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rsi); rbp = 0x3; r10 = 0x1; bb = 18; continue; }
            18 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x6u64))) as u32) as u32 as u64; r8 = (((crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as i32 as i64) as u64) as u64; rax = (((r10 as u32)).wrapping_shl(((((rax as u32)) as u32) & 31))) as u32 as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(1)) as u64; rax = ((r8.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r13 = ((r13.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 19; continue; }
            19 => { r9 = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; fl_l = ((r9).wrapping_shl(3u32)) as u64; r9 = ((fl_l as u64)) as u64; bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; r9 = r9.wrapping_sub(0x18); rcx = ((r9.wrapping_add(rax)) as u64) as u64; r8 = ((r8).wrapping_sub(1)) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x5; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 25; continue; } bb = 21; continue; }
            21 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = (crate::simd_ops::ru8(rdx)) as u8 as u64; fl_r = (0x5u8) as u8 as u64; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x1u64))) as u32) as u32 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 23; continue; } bb = 22; continue; }
            22 => { fl_l = ((((rax as u8)) & (0xfcu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rdx.wrapping_add(0x1u64), ((rax as u8)) as u8); bb = 25; continue; }
            23 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 25; continue; } bb = 24; continue; }
            24 => { crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), ((r14 as u32)) as u32); bb = 25; continue; }
            25 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 20; continue; } bb = 26; continue; }
            26 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x6u64))) as u32) as u32 as u64; rsi = ((r14 as u32)) as u32 as u64; rax = (((r10 as u32)).wrapping_shl(((((rax as u32)) as u32) & 31))) as u32 as u64; r8 = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 38; continue; } bb = 27; continue; }
            27 => { r9 = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; fl_l = ((r9).wrapping_shl(4u32)) as u64; r9 = ((fl_l as u64)) as u64; bb = 28; continue; }
            28 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; r9 = r9.wrapping_sub(0x30); r8 = ((r8).wrapping_sub(1)) as u64; fl_l = (crate::simd_ops::ru32(r9.wrapping_add(rax).wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 37; continue; } bb = 29; continue; }
            29 => { rcx = (crate::simd_ops::ru32(r9.wrapping_add(rax).wrapping_add(0x28u64))) as u32 as u64; rcx = ((((rcx as u32)) & (0xfu32))) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x5u8) as u8 as u64; if (((fl_l as u8)) < ((fl_r as u8))) { bb = 33; continue; } bb = 30; continue; }
            30 => { rdx = (crate::simd_ops::ru64(r9.wrapping_add(rax).wrapping_add(0x18u64))) as u64; fl_l = (crate::simd_ops::ru8(rdx)) as u8 as u64; fl_r = (0x5u8) as u8 as u64; rcx = ((crate::simd_ops::ru8(rdx.wrapping_add(0x1u64))) as u32) as u32 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 32; continue; } bb = 31; continue; }
            31 => { fl_l = ((((rcx as u8)) & (0xfcu8))) as u8 as u64; fl_r = fl_l; rcx = (rcx & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rdx.wrapping_add(0x1u64), ((rcx as u8)) as u8); bb = 33; continue; }
            32 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 43; continue; } bb = 33; continue; }
            33 => { fl_l = (crate::simd_ops::ru32(r9.wrapping_add(rax).wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x5; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 36; continue; } bb = 34; continue; }
            34 => { rdx = (crate::simd_ops::ru64(r9.wrapping_add(rax))) as u64; fl_l = (crate::simd_ops::ru8(rdx)) as u8 as u64; fl_r = (0x5u8) as u8 as u64; rcx = ((crate::simd_ops::ru8(rdx.wrapping_add(0x1u64))) as u32) as u32 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 42; continue; } bb = 35; continue; }
            35 => { fl_l = ((((rcx as u8)) & (0xfcu8))) as u8 as u64; fl_r = fl_l; rcx = (rcx & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rdx.wrapping_add(0x1u64), ((rcx as u8)) as u8); bb = 36; continue; }
            36 => { fl_l = (((rsi as u32)).wrapping_add(1)) as u32 as u64; rsi = ((fl_l as u32)) as u32 as u64; bb = 37; continue; }
            37 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 28; continue; } bb = 38; continue; }
            38 => { r11 = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; fl_l = r11; fl_r = r11; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 52; continue; } bb = 39; continue; }
            39 => { rbx = ((crate::simd_ops::ru8(r11.wrapping_add(0x3u64))) as u32) as u32 as u64; fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 52; continue; } bb = 40; continue; }
            40 => { rdx = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; rcx = r11; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb50u64))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 45; continue; } bb = 41; continue; }
            41 => { fl_l = ((((rbx as u8)) | (0x4u8))) as u8 as u64; fl_r = fl_l; rbx = (rbx & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(r11.wrapping_add(0x3u64), ((rbx as u8)) as u8); bb = 51; continue; }
            42 => { fl_l = ((rbp as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 36; continue; } bb = 43; continue; }
            43 => { rdx = (crate::simd_ops::ru32(r9.wrapping_add(rax).wrapping_add(0x28u64))) as u32 as u64; rcx = ((rdx as u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0xfu32))) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(rax).wrapping_add(0x10u64), ((r14 as u32)) as u32); fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x5u8) as u8 as u64; if (((fl_l as u8)) < ((fl_r as u8))) { bb = 37; continue; } bb = 44; continue; }
            44 => { rdx = ((((rdx as u32)) & (0xfffffffcu32))) as u32 as u64; fl_l = ((((rdx as u32)) | (0xcu32))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(rax).wrapping_add(0x28u64), ((rdx as u32)) as u32); bb = 37; continue; }
            45 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 51; continue; } bb = 46; continue; }
            46 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x5; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 51; continue; } bb = 47; continue; }
            47 => { rcx = (crate::simd_ops::ru64(rax)) as u64; fl_l = ((rcx).wrapping_add(0x18u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 51; continue; } bb = 48; continue; }
            48 => { rdx = 0x73; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 51; continue; } bb = 49; continue; }
            49 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x6u64))) as u32) as u32 as u64; rax = (((rbp as u32)).wrapping_shl(((((rax as u32)) as u32) & 31))) as u32 as u64; rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64; rdx = ((((rdx as u32)) & (0x7u32))) as u32 as u64; rax = (((rax as u32)).wrapping_add((rdx as u32))) as u32 as u64; rax = (((((rax as u32)) as i32).wrapping_shr(3u32) as u32)) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 51; continue; } bb = 50; continue; }
            50 => { r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; r9 = ((rsi as u32)) as u32 as u64; rdx = rdi; rcx = r15; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 51; continue; }
            51 => { r10 = 0x1; bb = 52; continue; }
            52 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 18; continue; } bb = 53; continue; }
            53 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; bb = 54; continue; }
            54 => { crate::simd_ops::wu64(rbp.wrapping_add(0x38u64), r14); r12 = ((r12).wrapping_add(r13)) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0xa90u64))) as u64; rdi = rax.wrapping_add(0xa78); fl_l = rcx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 60; continue; } bb = 55; continue; }
            55 => { bb = 56; continue; }
            56 => { rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; r14 = ((r14).wrapping_add(0x28u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x3u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 58; continue; } bb = 57; continue; }
            57 => { crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x0u8) as u8); bb = 59; continue; }
            58 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x1u64))) as u8 as u64; fl_r = (0x3u8) as u8 as u64; rdx = rcx; rcx = r15; r8 = (r8 & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 59; continue; }
            59 => { rcx = rbx; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 56; continue; } bb = 60; continue; }
            60 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x260u64))) as u64; crate::simd_ops::lk_xor_u8(rbp.wrapping_add(0x20u64), (0x3u8) as u8); crate::simd_ops::wu64(rbp.wrapping_add(0x268u64), rax); rax = ((r14.wrapping_add(r12)) as u64) as u64; crate::simd_ops::wu8(rbp.wrapping_add(0x21u64), (0x4u8) as u8); rsp = ((rsp).wrapping_add(0x28u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210b1e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14210c5f0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; rbx = 0; fl_l = 0; fl_r = 0; rdi = rcx; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0xa90u64))) as u64; rsi = rdx.wrapping_add(0xa78); fl_l = rax; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { r10 = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; rbx = ((rbx).wrapping_add(0x28u64)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x3u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu8(rax.wrapping_add(0x3u64), (0x0u8) as u8); bb = 13; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; r9 = rax.wrapping_add(0x10); r8 = ((crate::simd_ops::ru8(rax.wrapping_add(0x1u64))) as u32) as u32 as u64; crate::simd_ops::wu64(r10.wrapping_add(0x10u64), rcx); rdx = (crate::simd_ops::ru64(r9)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x18u64), rcx); fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 13; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; xmm0 = (crate::simd_ops::ru128(rcx)) as u128; crate::simd_ops::wu128(r9, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(r9.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rdx = ((crate::simd_ops::ru8(rax.wrapping_add(0x1u64))) as u32) as u32 as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), r9); fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x7u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 13; continue; } bb = 6; continue; }
            6 => { r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rcx = ((crate::simd_ops::ru8(r8.wrapping_add(0x21u64))) as u32) as u32 as u64; rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_sub(1)) as u8 as u64)); fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) <= ((fl_r as u8))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = ((crate::simd_ops::ru8(r8.wrapping_add(0x20u64))) as u32) as u32 as u64; rdx = (rdx & !0xffu64) | ((((((rdx as u8)) & (0xf8u8))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0x3u8))) as u8 as u64)); fl_l = ((((rcx as u8)) | ((rdx as u8)))) as u8 as u64; fl_r = fl_l; rcx = (rcx & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rax.wrapping_add(0x1u64), ((rcx as u8)) as u8); bb = 13; continue; }
            8 => { rdx = (rdx & !0xffu64) | ((((((rdx as u8)) | (0x4u8))) as u8 as u64)); crate::simd_ops::wu8(rax.wrapping_add(0x1u64), ((rdx as u8)) as u8); fl_l = (crate::simd_ops::ru32(r9.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x5; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 13; continue; } bb = 9; continue; }
            9 => { r11 = (crate::simd_ops::ru64(r9)) as u64; fl_l = (crate::simd_ops::ru8(r11.wrapping_add(0x1u64))) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 10; continue; }
            10 => { r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rcx = ((crate::simd_ops::ru8(r8.wrapping_add(0x21u64))) as u32) as u32 as u64; rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_sub(1)) as u8 as u64)); fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) <= ((fl_r as u8))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = ((crate::simd_ops::ru8(r8.wrapping_add(0x20u64))) as u32) as u32 as u64; rdx = (rdx & !0xffu64) | ((((((rdx as u8)) & (0xf8u8))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0x3u8))) as u8 as u64)); fl_l = ((((rcx as u8)) | ((rdx as u8)))) as u8 as u64; fl_r = fl_l; rcx = (rcx & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rax.wrapping_add(0x1u64), ((rcx as u8)) as u8); bb = 13; continue; }
            12 => { rdx = r11; rcx = r8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 13; continue; }
            13 => { rax = r10; fl_l = r10; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 14; continue; }
            14 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210b300(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xa90; const P1: u64 = 0xa78; const P2: u64 = 0x1; const P3: u64 = 0x8; const P4: u64 = 0x10; const P5: u64 = 0x14210c5f0; const P6: u64 = 0x18;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r11 = ((rcx.wrapping_add(P1)) as u64) as u64; r10 = 0; fl_l = 0; fl_r = 0; rbx = rcx; fl_l = r9; fl_r = r11; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { r10 = ((r10).wrapping_add(0x28u64)) as u64; fl_l = (crate::simd_ops::ru8(r9.wrapping_add(P2))) as u8 as u64; fl_r = (0x7u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(r9.wrapping_add(P3))) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P4))) as u32 as u64; fl_r = 0x5; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P2))) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { r9 = (crate::simd_ops::ru64(r9.wrapping_add(P6))) as u64; fl_l = r9; fl_r = r11; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 7; continue; }
            7 => { rax = r10; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210b360(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0xa70; const P2: u64 = 0x28; const P3: u64 = 0x30; const P4: u64 = 0x38; const P5: u64 = 0x1; const P6: u64 = 0x14210c5f0; const P7: u64 = 0x58; const P8: u64 = 0xc08; const P9: u64 = 0xbf8; const P10: u64 = 0x14210b3f0; const P11: u64 = 0x21;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = 0; fl_l = 0; fl_r = 0; r9 = rcx; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P2), rax); crate::simd_ops::wu64(rbx.wrapping_add(P3), rax); crate::simd_ops::wu64(rbx.wrapping_add(P4), rax); fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P5))) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(P7))) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P5))) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(r9.wrapping_add(P0))) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P8))) as u32 as u64; fl_r = 0x5; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 7; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rax.wrapping_add(P9))) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P5))) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rcx = rbx; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rbx.wrapping_add(P11), (0x1u8) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210b3f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140000000; const P1: u64 = 0x14210c5f0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r11 = (rcx) as u64; r10 = ((rcx.wrapping_add(0xaa0u64)) as u64) as u64; rbx = (0xau32) as u32 as u64; rdi = ((P0) as u64) as u64; bb = 1; continue; }
            1 => { r9 = (crate::simd_ops::ru64(r10)) as u64; fl_l = (r9) as u64 as u64; fl_r = (r9) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 2; continue; }
            2 => { r8 = ((crate::simd_ops::ru8(r9.wrapping_add(0x1u64))) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru8(r9)) as u32) as u32 as u64; r8 = (r8 & !0xffu64) | ((((((r8 as u8)) & (0xfcu8))) as u8 as u64)); rax = (((rax as u32)).wrapping_add(0xfffffffbu32)) as u32 as u64; crate::simd_ops::wu8(r9.wrapping_add(0x1u64), ((r8 as u8)) as u8); fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x6u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 18; continue; } bb = 4; continue; }
            4 => { rax = (rax as u32 as i32 as i64) as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x210b4e4u64))) as u32 as u64; fl_l = ((rcx).wrapping_add(rdi)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; j_tgt = (rcx) as u64; bb = 0xffff_fffe; continue; }
            5 => { rdx = (crate::simd_ops::ru64(r9.wrapping_add(0x8u64))) as u64; r8 = (r8 & !0xffu64) | ((((((r8 as u8)) | (0x4u8))) as u8 as u64)); crate::simd_ops::wu8(r9.wrapping_add(0x1u64), ((r8 as u8)) as u8); fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x1u64))) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 7; continue; }
            7 => { rcx = (r11) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 18; continue; }
            8 => { rax = (crate::simd_ops::ru64(r9.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = (0x5u32) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 11; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x1u64))) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = (r11) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 11; continue; }
            11 => { rax = ((r9.wrapping_add(0x10u64)) as u64) as u64; fl_l = (crate::simd_ops::ru64(r9.wrapping_add(0x8u64))) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 18; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::lk_or_u8(r9.wrapping_add(0x1u64), (0x4u8) as u8)) as u8 as u64; fl_r = fl_l; bb = 18; continue; }
            13 => { rax = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(r9.wrapping_add(0x8u64), (rax) as u64); bb = 17; continue; }
            14 => { rax = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(r9.wrapping_add(0x28u64), (rax) as u64); bb = 17; continue; }
            15 => { rax = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(r9.wrapping_add(0x68u64), (rax) as u64); bb = 17; continue; }
            16 => { rax = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(r9.wrapping_add(0x58u64), (rax) as u64); bb = 17; continue; }
            17 => { crate::simd_ops::wu64(r11.wrapping_add(0x28u64), (r9) as u64); bb = 18; continue; }
            18 => { r10 = ((r10).wrapping_add(0x8u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (0x1u64) as u64 as u64; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 19; continue; }
            19 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x14210b3f0u64) { 0x20 => { bb = 1; continue; } 0x2c => { bb = 2; continue; } 0x3b => { bb = 3; continue; } 0x53 => { bb = 4; continue; } 0x61 => { bb = 5; continue; } 0x72 => { bb = 6; continue; } 0x78 => { bb = 7; continue; } 0x82 => { bb = 8; continue; } 0x8c => { bb = 9; continue; } 0x95 => { bb = 10; continue; } 0x9d => { bb = 11; continue; } 0xa7 => { bb = 12; continue; } 0xae => { bb = 13; continue; } 0xb8 => { bb = 14; continue; } 0xc2 => { bb = 15; continue; } 0xcc => { bb = 16; continue; } 0xd4 => { bb = 17; continue; } 0xd8 => { bb = 18; continue; } 0xe6 => { bb = 19; continue; } _ => { let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(j_tgt, rcx, rdx, r8, r9, s5, s6, s7, s8); } } }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210b510(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14210b5d0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = r8;
    r8 = rdx;
    rdx = rax;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64));
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210b530(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0xc; const P2: u64 = 0x8; const P3: u64 = 0x142103040;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r10 = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64; r8 = ((r10 as u32)) as u32 as u64; r9 = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; rax = ((r10 as u32)) as u32 as u64; rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64; rdx = ((((rdx as u32)) & (0x3u32))) as u32 as u64; rax = (((rax as u32)).wrapping_add((rdx as u32))) as u32 as u64; rax = (((((rax as u32)) as i32).wrapping_shr(2u32) as u32)) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x40; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = ((r8 as u32)) as u32 as u64; rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64; rax = (((rax as u32)).wrapping_sub((rdx as u32))) as u32 as u64; rax = (((((rax as u32)) as i32).wrapping_shr(1u32) as u32)) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64; rdx = ((((rdx as u32)) & (0x3u32))) as u32 as u64; rax = (((rax as u32)).wrapping_add((rdx as u32))) as u32 as u64; rax = (((((rax as u32)) as i32).wrapping_shr(2u32) as u32)) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 1; continue; } bb = 3; continue; }
            3 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = ((r8 as u32)) as u32 as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210b590(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0xc; const P2: u64 = 0x8; const P3: u64 = 0x142103040;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    r9 = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64;
    rax = ((r9 as u32)) as u32 as u64;
    rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64;
    rdx = ((((rdx as u32)) & (0x3u32))) as u32 as u64;
    rax = (((rax as u32)).wrapping_add((rdx as u32))) as u32 as u64;
    rax = (((((rax as u32)) as i32).wrapping_shr(2u32) as u32)) as u32 as u64;
    fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P2))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if (((fl_l as u32)) >= ((fl_r as u32))) { return rax; }
    fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x40;
    if !(((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32))) { rax = ((r9 as u32)) as u32 as u64; rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64; rax = (((rax as u32)).wrapping_sub((rdx as u32))) as u32 as u64; fl_l = (((((rax as u32)) as i32).wrapping_shr(1u32) as u32)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210b8b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1420fcb60; const P1: u64 = 0x142646320; const P2: u64 = 0x1420fbac0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x28u64)) as u64; r15 = 0; fl_l = 0; fl_r = 0; rdi = rdx; rbp = rcx; r14 = ((r15 as u32)) as u32 as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 37; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r12); r12 = ((r15.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r13); r13 = ((r15.wrapping_add(0x3u64)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); bb = 2; continue; }
            2 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x6u64))) as u32) as u32 as u64; r8 = (((crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as i32 as i64) as u64) as u64; rax = (((r12 as u32)).wrapping_shl(((((rax as u32)) as u32) & 31))) as u32 as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(1)) as u64; rax = ((r8.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r14 = ((r14.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 3; continue; }
            3 => { r9 = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; fl_l = ((r9).wrapping_shl(3u32)) as u64; r9 = ((fl_l as u64)) as u64; bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; r9 = r9.wrapping_sub(0x18); rcx = ((r9.wrapping_add(rax)) as u64) as u64; r8 = ((r8).wrapping_sub(1)) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x5; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 9; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = (crate::simd_ops::ru8(rdx)) as u8 as u64; fl_r = (0x5u8) as u8 as u64; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x1u64))) as u32) as u32 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 6; continue; }
            6 => { fl_l = ((((rax as u8)) & (0xfcu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rdx.wrapping_add(0x1u64), ((rax as u8)) as u8); bb = 9; continue; }
            7 => { fl_l = ((r13 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), ((r15 as u32)) as u32); bb = 9; continue; }
            9 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 10; continue; }
            10 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x6u64))) as u32) as u32 as u64; rsi = ((r15 as u32)) as u32 as u64; rax = (((r12 as u32)).wrapping_shl(((((rax as u32)) as u32) & 31))) as u32 as u64; r8 = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 22; continue; } bb = 11; continue; }
            11 => { r9 = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; fl_l = ((r9).wrapping_shl(4u32)) as u64; r9 = ((fl_l as u64)) as u64; bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; r9 = r9.wrapping_sub(0x30); r8 = ((r8).wrapping_sub(1)) as u64; fl_l = (crate::simd_ops::ru32(r9.wrapping_add(rax).wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 21; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru32(r9.wrapping_add(rax).wrapping_add(0x28u64))) as u32 as u64; rcx = ((((rcx as u32)) & (0xfu32))) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x5u8) as u8 as u64; if (((fl_l as u8)) < ((fl_r as u8))) { bb = 17; continue; } bb = 14; continue; }
            14 => { rdx = (crate::simd_ops::ru64(r9.wrapping_add(rax).wrapping_add(0x18u64))) as u64; fl_l = (crate::simd_ops::ru8(rdx)) as u8 as u64; fl_r = (0x5u8) as u8 as u64; rcx = ((crate::simd_ops::ru8(rdx.wrapping_add(0x1u64))) as u32) as u32 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 16; continue; } bb = 15; continue; }
            15 => { fl_l = ((((rcx as u8)) & (0xfcu8))) as u8 as u64; fl_r = fl_l; rcx = (rcx & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rdx.wrapping_add(0x1u64), ((rcx as u8)) as u8); bb = 17; continue; }
            16 => { fl_l = ((r13 as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 27; continue; } bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru32(r9.wrapping_add(rax).wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x5; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 20; continue; } bb = 18; continue; }
            18 => { rdx = (crate::simd_ops::ru64(r9.wrapping_add(rax))) as u64; fl_l = (crate::simd_ops::ru8(rdx)) as u8 as u64; fl_r = (0x5u8) as u8 as u64; rcx = ((crate::simd_ops::ru8(rdx.wrapping_add(0x1u64))) as u32) as u32 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 26; continue; } bb = 19; continue; }
            19 => { fl_l = ((((rcx as u8)) & (0xfcu8))) as u8 as u64; fl_r = fl_l; rcx = (rcx & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rdx.wrapping_add(0x1u64), ((rcx as u8)) as u8); bb = 20; continue; }
            20 => { fl_l = (((rsi as u32)).wrapping_add(1)) as u32 as u64; rsi = ((fl_l as u32)) as u32 as u64; bb = 21; continue; }
            21 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 22; continue; }
            22 => { r11 = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; fl_l = r11; fl_r = r11; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 35; continue; } bb = 23; continue; }
            23 => { rbx = ((crate::simd_ops::ru8(r11.wrapping_add(0x3u64))) as u32) as u32 as u64; fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 35; continue; } bb = 24; continue; }
            24 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; rcx = r11; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb50u64))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 29; continue; } bb = 25; continue; }
            25 => { fl_l = ((((rbx as u8)) | (0x4u8))) as u8 as u64; fl_r = fl_l; rbx = (rbx & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(r11.wrapping_add(0x3u64), ((rbx as u8)) as u8); bb = 35; continue; }
            26 => { fl_l = ((r13 as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 20; continue; } bb = 27; continue; }
            27 => { rdx = (crate::simd_ops::ru32(r9.wrapping_add(rax).wrapping_add(0x28u64))) as u32 as u64; rcx = ((rdx as u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0xfu32))) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(rax).wrapping_add(0x10u64), ((r15 as u32)) as u32); fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x5u8) as u8 as u64; if (((fl_l as u8)) < ((fl_r as u8))) { bb = 21; continue; } bb = 28; continue; }
            28 => { rdx = ((((rdx as u32)) & (0xfffffffcu32))) as u32 as u64; fl_l = ((((rdx as u32)) | (0xcu32))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(rax).wrapping_add(0x28u64), ((rdx as u32)) as u32); bb = 21; continue; }
            29 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 35; continue; } bb = 30; continue; }
            30 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x5; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 35; continue; } bb = 31; continue; }
            31 => { rcx = (crate::simd_ops::ru64(rax)) as u64; fl_l = ((rcx).wrapping_add(0x18u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 35; continue; } bb = 32; continue; }
            32 => { rdx = 0x73; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 35; continue; } bb = 33; continue; }
            33 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x6u64))) as u32) as u32 as u64; rax = (((r13 as u32)).wrapping_shl(((((rax as u32)) as u32) & 31))) as u32 as u64; rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64; rdx = ((((rdx as u32)) & (0x7u32))) as u32 as u64; rax = (((rax as u32)).wrapping_add((rdx as u32))) as u32 as u64; rax = (((((rax as u32)) as i32).wrapping_shr(3u32) as u32)) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 35; continue; } bb = 34; continue; }
            34 => { r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; r9 = ((rsi as u32)) as u32 as u64; rdx = rdi; rcx = rbp; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 35; continue; }
            35 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 36; continue; }
            36 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 37; continue; }
            37 => { rax = r14; rsp = ((rsp).wrapping_add(0x28u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210bac0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x5u8) as u8 as u64;
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xfcu8))) as u8 as u64)); crate::simd_ops::wu8(rcx.wrapping_add(P0), ((rax as u8)) as u8); rax = 0; fl_l = 0; fl_r = 0; return rax; }
    fl_l = ((((rax as u32)) & (0x3u32))) as u32 as u64; fl_r = fl_l;
    rax = ((fl_l as u32)) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210bae0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x14210bb20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = 0;
    fl_l = 0; fl_r = 0;
    rdi = rcx;
    fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = rbx;
    if (((fl_l as u64)) == ((fl_r as u64))) { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    loop { rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(rax)) as u64; fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rax = rbx;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210bf30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x8; const P2: u64 = 0x20; const P3: u64 = 0x10; const P4: u64 = 0x30; const P5: u64 = 0x4c; const P6: u64 = 0x14210dae0; const P7: u64 = 0x48; const P8: u64 = 0x14210d8d0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rcx; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; r8 = r9; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; fl_l = r9; fl_r = rcx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { fl_l = rdx; fl_r = (crate::simd_ops::ru64(r8.wrapping_add(P3))) as u64; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { crate::simd_ops::ru64(r8.wrapping_add(P3)) } else { rdx }) as u64; r8 = ((r8).wrapping_add(0x28u64)) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(rbx.wrapping_add(P4)))) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; r8 = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64; let prod = (rax as i64 as i128).wrapping_mul((rdx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdi = rdx; rdi = ((((rdi) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rdi; rax = ((rax).wrapping_shr(63u32)) as u64; rdi = ((rdi).wrapping_add(rax)) as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x4e20; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 10; continue; } bb = 4; continue; }
            4 => { rcx = ((rcx).wrapping_sub(r9)) as u64; rax = (0x6666666666666667u64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(4u32) as u64)) as u64; rcx = rdx; rcx = ((rcx).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 7; continue; } bb = 5; continue; }
            5 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x10; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = ((r8 as u32)) as u32 as u64; rcx = rbx; rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64; rax = (((rax as u32)).wrapping_sub((rdx as u32))) as u32 as u64; rax = (((((rax as u32)) as i32).wrapping_shr(1u32) as u32)) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P7))) as u32 as u64; rcx = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xaa; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64; rcx = rbx; rax = (((rax as u32)).wrapping_sub((rdx as u32))) as u32 as u64; rax = (((((rax as u32)) as i32).wrapping_shr(1u32) as u32)) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210c010(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x48; const P1: u64 = 0x30; const P2: u64 = 0x8; const P3: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (((crate::simd_ops::ru32(rcx.wrapping_add(P0))) as i32 as i64) as u64) as u64;
    rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    r8 = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64;
    fl_l = rax; fl_r = r8;
    if (((fl_l as u64)) >= ((fl_r as u64))) { return rax; }
    rcx = 0;
    fl_l = 0; fl_r = 0;
    loop { crate::simd_ops::wu32(rax.wrapping_add(P3), ((rcx as u32)) as u32); rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = rax; fl_r = r8; if !((((fl_l as u64)) < ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210c050(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14210c5f0; const P1: u64 = 0x140000000;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); r11 = rcx; rbx = rdx; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x58u64))) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x1u64))) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x70u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::lk_and_u8(rax.wrapping_add(0x1u64), (0xfcu8) as u8)) as u8 as u64; fl_r = fl_l; bb = 4; continue; }
            4 => { r9 = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; fl_l = r9; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 25; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rdi = P1; bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(r9.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x5; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 23; continue; } bb = 7; continue; }
            7 => { r10 = (crate::simd_ops::ru64(r9)) as u64; r8 = ((crate::simd_ops::ru8(r10.wrapping_add(0x1u64))) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 23; continue; } bb = 8; continue; }
            8 => { rax = ((crate::simd_ops::ru8(r10)) as u32) as u32 as u64; r8 = (r8 & !0xffu64) | ((((((r8 as u8)) & (0xfcu8))) as u8 as u64)); rax = (((rax as u32)).wrapping_add(0xfffffffbu32)) as u32 as u64; crate::simd_ops::wu8(r10.wrapping_add(0x1u64), ((r8 as u8)) as u8); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x6; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 23; continue; } bb = 9; continue; }
            9 => { rax = (rax as u32 as i32 as i64) as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x210c198u64))) as u32 as u64; fl_l = ((rcx).wrapping_add(rdi)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; j_tgt = rcx; bb = 0xffff_fffe; continue; }
            10 => { rdx = (crate::simd_ops::ru64(r10.wrapping_add(0x8u64))) as u64; r8 = (r8 & !0xffu64) | ((((((r8 as u8)) | (0x4u8))) as u8 as u64)); crate::simd_ops::wu8(r10.wrapping_add(0x1u64), ((r8 as u8)) as u8); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x1u64))) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 23; continue; } bb = 12; continue; }
            12 => { rcx = r11; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 23; continue; }
            13 => { rax = (crate::simd_ops::ru64(r10.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x5; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 16; continue; } bb = 14; continue; }
            14 => { rdx = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x1u64))) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = r11; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            16 => { rax = r10.wrapping_add(0x10); fl_l = (crate::simd_ops::ru64(r10.wrapping_add(0x8u64))) as u64; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 23; continue; } bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::lk_or_u8(r10.wrapping_add(0x1u64), (0x4u8) as u8)) as u8 as u64; fl_r = fl_l; bb = 23; continue; }
            18 => { rax = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(r10.wrapping_add(0x8u64), rax); bb = 22; continue; }
            19 => { rax = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(r10.wrapping_add(0x28u64), rax); bb = 22; continue; }
            20 => { rax = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(r10.wrapping_add(0x68u64), rax); bb = 22; continue; }
            21 => { rax = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(r10.wrapping_add(0x58u64), rax); bb = 22; continue; }
            22 => { crate::simd_ops::wu64(r11.wrapping_add(0x28u64), r10); bb = 23; continue; }
            23 => { r9 = ((r9).wrapping_add(0x18u64)) as u64; fl_l = r9; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 24; continue; }
            24 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 25; continue; }
            25 => { r9 = (crate::simd_ops::ru64(rbx.wrapping_add(0x60u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 26; continue; }
            26 => { fl_l = (crate::simd_ops::ru8(r9.wrapping_add(0x1u64))) as u8 as u64; fl_r = (0x3u8) as u8 as u64; crate::simd_ops::wu8(r9.wrapping_add(0x3u64), (0x1u8) as u8); if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 28; continue; } bb = 27; continue; }
            27 => { rdx = r9; rcx = r11; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 28; continue; }
            28 => { r9 = (crate::simd_ops::ru64(r9.wrapping_add(0x20u64))) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 26; continue; } bb = 29; continue; }
            29 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x14210c050u64) { 0x19 => { bb = 1; continue; } 0x1e => { bb = 2; continue; } 0x27 => { bb = 3; continue; } 0x2b => { bb = 4; continue; } 0x39 => { bb = 5; continue; } 0x45 => { bb = 6; continue; } 0x50 => { bb = 7; continue; } 0x62 => { bb = 8; continue; } 0x7a => { bb = 9; continue; } 0x88 => { bb = 10; continue; } 0x99 => { bb = 11; continue; } 0x9f => { bb = 12; continue; } 0xa9 => { bb = 13; continue; } 0xb3 => { bb = 14; continue; } 0xbc => { bb = 15; continue; } 0xc4 => { bb = 16; continue; } 0xce => { bb = 17; continue; } 0xd5 => { bb = 18; continue; } 0xdf => { bb = 19; continue; } 0xe9 => { bb = 20; continue; } 0xf3 => { bb = 21; continue; } 0xfb => { bb = 22; continue; } 0xff => { bb = 23; continue; } 0x10d => { bb = 24; continue; } 0x112 => { bb = 25; continue; } 0x120 => { bb = 26; continue; } 0x12c => { bb = 27; continue; } 0x137 => { bb = 28; continue; } 0x140 => { bb = 29; continue; } _ => { return crate::simd_ops::tail_stk(j_tgt, rax, rcx, rdx, r8, r9, rsp); } } }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210c1c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1; const P2: u64 = 0x14210c5f0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r11 = rdx; rbx = rcx; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(r11.wrapping_add(0x3u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 3; continue; }
            3 => { r9 = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru8(r11.wrapping_add(0x4u64))) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; if (((fl_l as u8)) <= ((fl_r as u8))) { bb = 18; continue; } bb = 4; continue; }
            4 => { r10 = r11.wrapping_add(0x30); bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(r10.wrapping_add(P0))) as u32 as u64; fl_r = 0x5; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 8; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(r10)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rax = ((crate::simd_ops::ru8(r11.wrapping_add(0x4u64))) as u32) as u32 as u64; r9 = (((r9 as u32)).wrapping_add(1)) as u32 as u64; r10 = ((r10).wrapping_add(0x18u64)) as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 5; continue; } bb = 9; continue; }
            9 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            10 => { rdx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { r9 = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru8(r11.wrapping_add(0x4u64))) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; if (((fl_l as u8)) <= ((fl_r as u8))) { bb = 18; continue; } bb = 13; continue; }
            13 => { r10 = r11.wrapping_add(0x20); bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru32(r10.wrapping_add(P0))) as u32 as u64; fl_r = 0x5; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 17; continue; } bb = 15; continue; }
            15 => { rdx = (crate::simd_ops::ru64(r10)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            17 => { rax = ((crate::simd_ops::ru8(r11.wrapping_add(0x4u64))) as u32) as u32 as u64; r9 = (((r9 as u32)).wrapping_add(1)) as u32 as u64; r10 = ((r10).wrapping_add(0x18u64)) as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 14; continue; } bb = 18; continue; }
            18 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210c280(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14210c5f0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x40u64))) as u64; r9 = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rdi = rcx; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::lk_and_u8(rax.wrapping_add(0x1u64), (0xfcu8) as u8)) as u8 as u64; fl_r = fl_l; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x48u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::lk_and_u8(rax.wrapping_add(0x1u64), (0xfcu8) as u8)) as u8 as u64; fl_r = fl_l; bb = 4; continue; }
            4 => { r10 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); r11 = ((r10 as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x70u64))) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 10; continue; } bb = 5; continue; }
            5 => { rbx = ((r10 as u32)) as u32 as u64; bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(r9.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(rax).wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x5; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 9; continue; } bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(rax))) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x1u64))) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = rdi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { r11 = (((r11 as u32)).wrapping_add(1)) as u32 as u64; rbx = ((rbx).wrapping_add(0x18u64)) as u64; fl_l = ((r11 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r9.wrapping_add(0x70u64))) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 6; continue; } bb = 10; continue; }
            10 => { rcx = ((r10 as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(r9.wrapping_add(0x6cu64))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 15; continue; } bb = 11; continue; }
            11 => { r8 = r10; bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(r9.wrapping_add(0x38u64))) as u64; rdx = (crate::simd_ops::ru64(r8.wrapping_add(rax))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::lk_and_u8(rdx.wrapping_add(0x1u64), (0xfcu8) as u8)) as u8 as u64; fl_r = fl_l; bb = 14; continue; }
            14 => { rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; r8 = ((r8).wrapping_add(0x8u64)) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r9.wrapping_add(0x6cu64))) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 12; continue; } bb = 15; continue; }
            15 => { r11 = ((r10 as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(r9.wrapping_add(0x64u64))) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 21; continue; } bb = 16; continue; }
            16 => { rbx = r10; bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(r9.wrapping_add(0x18u64))) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(rax))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x1u64))) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { rcx = rdi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            20 => { r11 = (((r11 as u32)).wrapping_add(1)) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = ((r11 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r9.wrapping_add(0x64u64))) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 17; continue; } bb = 21; continue; }
            21 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = (crate::simd_ops::ru32(r9.wrapping_add(0x68u64))) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 26; continue; } bb = 22; continue; }
            22 => { rdx = r10; bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru64(r9.wrapping_add(0x30u64))) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(rax))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 24; continue; }
            24 => { fl_l = (crate::simd_ops::lk_and_u8(rcx.wrapping_add(0x1u64), (0xfcu8) as u8)) as u8 as u64; fl_r = fl_l; bb = 25; continue; }
            25 => { r10 = (((r10 as u32)).wrapping_add(1)) as u32 as u64; rdx = ((rdx).wrapping_add(0x18u64)) as u64; fl_l = ((r10 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r9.wrapping_add(0x68u64))) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 23; continue; } bb = 26; continue; }
            26 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210c590(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x3; const P2: u64 = 0xb50; const P3: u64 = 0x1420fcb60;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { r11 = ((crate::simd_ops::ru8(rbx.wrapping_add(P1))) as u32) as u32 as u64; fl_l = ((r11 as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = ((((r11 as u8)) | (0x4u8))) as u8 as u64; fl_r = fl_l; r11 = (r11 & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(P1), ((r11 as u8)) as u8); bb = 4; continue; }
            4 => { rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; fl_r = 0x5; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rax)) as u64; rax = ((rax).wrapping_add(0x18u64)) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210c710(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rax = ((rdx as u32)) as u32 as u64;
    rax = ((((rax as u32)) & (0xfu32))) as u32 as u64;
    fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x5u8) as u8 as u64;
    if !((((fl_l as u8)) < ((fl_r as u8)))) { rdx = ((((rdx as u32)) & (0xfffffffcu32))) as u32 as u64; fl_l = ((((rdx as u32)) | (0xcu32))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rdx as u32)) as u32); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210c730(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1421068a0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rdx = rcx;
    return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210c740(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x21; const P2: u64 = 0x20; const P3: u64 = 0x1; const P4: u64 = 0x14210c5f0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_sub(1)) as u8 as u64));
    fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64;
    if !((((fl_l as u8)) <= ((fl_r as u8)))) { rcx = ((crate::simd_ops::ru8(rcx.wrapping_add(P2))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P3))) as u32) as u32 as u64; rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0x3u8))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xf8u8))) as u8 as u64)); fl_l = ((((rcx as u8)) | ((rax as u8)))) as u8 as u64; fl_r = fl_l; rcx = (rcx & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rdx.wrapping_add(P3), ((rcx as u8)) as u8); return rax; }
    rdx = r8;
    return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210c990(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x1; const P2: u64 = 0x30;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = (crate::simd_ops::lk_and_u8(rdx.wrapping_add(P1), (0xfbu8) as u8)) as u8 as u64; fl_r = fl_l;
    rax = (crate::simd_ops::ru64(r9.wrapping_add(P2))) as u64;
    crate::simd_ops::wu64(r8, rax);
    crate::simd_ops::wu64(r9.wrapping_add(P2), rdx);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210c9b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x21; const P2: u64 = 0x14210c5f0; const P3: u64 = 0x1; const P4: u64 = 0x30; const P5: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = (crate::simd_ops::ru8(r9.wrapping_add(P1))) as u8 as u64; fl_r = (0x2u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rdx = r8; rcx = r9; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    fl_l = (crate::simd_ops::lk_and_u8(rdx.wrapping_add(P3), (0xfbu8) as u8)) as u8 as u64; fl_r = fl_l;
    rax = (crate::simd_ops::ru64(r9.wrapping_add(P4))) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rax);
    crate::simd_ops::wu64(r9.wrapping_add(P4), rdx);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210c9e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x2; const P2: u64 = 0x142107140; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x1; const P6: u64 = 0x28; const P7: u64 = 0x3; const P8: u64 = 0x10; const P9: u64 = 0x5;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rsi = ((((rdx as u32)) as i32 as i64) as u64) as u64;
    rdi = r8;
    r8 = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rbx = rcx;
    rax = ((rsi.wrapping_add(P1)) as u64) as u64;
    rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rdx = ((rdx).wrapping_shl(3u32)) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    r8 = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64;
    rdx = ((crate::simd_ops::ru8(r8.wrapping_add(P4))) as u32) as u32 as u64;
    rdx = (rdx & !0xffu64) | ((((((rdx as u8)) & (0x3u8))) as u8 as u64));
    crate::simd_ops::wu8(rax, (0x7u8) as u8);
    crate::simd_ops::wu8(rax.wrapping_add(P5), ((rdx as u8)) as u8);
    rcx = ((crate::simd_ops::ru8(rbx.wrapping_add(P0))) as u32) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P1), ((rcx as u8)) as u8);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu8(rax.wrapping_add(P0), ((rsi as u8)) as u8);
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    crate::simd_ops::wu64(rax.wrapping_add(P3), rcx);
    crate::simd_ops::wu64(rax.wrapping_add(P4), rcx);
    crate::simd_ops::wu64(rax.wrapping_add(P6), rcx);
    crate::simd_ops::wu8(rax.wrapping_add(P7), (0x1u8) as u8);
    crate::simd_ops::wu64(rax.wrapping_add(P8), rdi);
    crate::simd_ops::wu16(rax.wrapping_add(P9), (0x28u16) as u16);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210ca60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x2; const P1: u64 = 0x1421072a0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    r9 = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r8);
    r8 = 0x28;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210ca90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x10; const P2: u64 = 0x8; const P3: u64 = 0x1; const P4: u64 = 0x21; const P5: u64 = 0x20; const P6: u64 = 0x14210c5f0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r9 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; r10 = ((rdx.wrapping_add(P1)) as u64) as u64; rax = (crate::simd_ops::ru64(r10)) as u64; r11 = rcx; crate::simd_ops::wu64(r9.wrapping_add(P1), rax); r9 = (crate::simd_ops::ru64(r10)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; crate::simd_ops::wu64(r9.wrapping_add(P0), rax); fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(r10, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64) as u128; crate::simd_ops::wu64(r10.wrapping_add(P1), ((xmm1 as u64)) as u64); rcx = ((crate::simd_ops::ru8(rdx.wrapping_add(P3))) as u32) as u32 as u64; crate::simd_ops::wu64(rdx.wrapping_add(P2), r10); fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x7u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 8; continue; } bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(P4))) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_sub(1)) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) > ((fl_r as u8))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rcx = (rcx & !0xffu64) | ((((((rcx as u8)) | (0x4u8))) as u8 as u64)); crate::simd_ops::wu8(rdx.wrapping_add(P3), ((rcx as u8)) as u8); fl_l = (crate::simd_ops::ru32(r10.wrapping_add(P1))) as u32 as u64; fl_r = 0x5; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 8; continue; } bb = 4; continue; }
            4 => { r9 = (crate::simd_ops::ru64(r10)) as u64; fl_l = (crate::simd_ops::ru8(r9.wrapping_add(P3))) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { r8 = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(P4))) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_sub(1)) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) <= ((fl_r as u8))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = ((crate::simd_ops::ru8(r8.wrapping_add(P5))) as u32) as u32 as u64; rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0xf8u8))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x3u8))) as u8 as u64)); fl_l = ((((rax as u8)) | ((rcx as u8)))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rdx.wrapping_add(P3), ((rax as u8)) as u8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { rdx = r9; rcx = r8; return crate::simd_ops::tail_fwd_xmm(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); }
            8 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210cb30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x3; const P1: u64 = 0x4; const P2: u64 = 0x2; const P3: u64 = 0x1421072a0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    r11 = r8;
    rax = 0x20;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r11);
    r10 = 0x30;
    r10 = (if (((fl_l as u8)) == ((fl_r as u8))) { (rax as u32) } else { (r10 as u32) }) as u32 as u64;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64;
    r9 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    r8 = ((r10.wrapping_add(r9.wrapping_mul(8))) as u64) as u64;
    r9 = ((crate::simd_ops::ru8(rdx.wrapping_add(P2))) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210cb70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x2; const P1: u64 = 0x60; const P2: u64 = 0x10; const P3: u64 = 0x142107510; const P4: u64 = 0x64; const P5: u64 = 0x18; const P6: u64 = 0x70; const P7: u64 = 0x8; const P8: u64 = 0x20; const P9: u64 = 0x74; const P10: u64 = 0x68; const P11: u64 = 0x30; const P12: u64 = 0x1421072a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r9 = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64; rsi = r8; r8 = (((crate::simd_ops::ru32(rdx.wrapping_add(P1))) as i32 as i64) as u64) as u64; rbx = rdx; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; rdi = rcx; r8 = ((r8).wrapping_shl(2u32)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r8 = (((crate::simd_ops::ru32(rbx.wrapping_add(P4))) as i32 as i64) as u64) as u64; rcx = rdi; r9 = ((crate::simd_ops::ru8(rbx.wrapping_add(P0))) as u32) as u32 as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = (((crate::simd_ops::ru32(rbx.wrapping_add(P6))) as i32 as i64) as u64) as u64; rcx = rdi; r9 = ((crate::simd_ops::ru8(rbx.wrapping_add(P0))) as u32) as u32 as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P7))) as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P8))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { r8 = (((crate::simd_ops::ru32(rbx.wrapping_add(P9))) as i32 as i64) as u64) as u64; rcx = rdi; r9 = ((crate::simd_ops::ru8(rbx.wrapping_add(P0))) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rax = (((crate::simd_ops::ru32(rbx.wrapping_add(P10))) as i32 as i64) as u64) as u64; rcx = rdi; r9 = ((crate::simd_ops::ru8(rbx.wrapping_add(P0))) as u32) as u32 as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P11))) as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r8 = (((crate::simd_ops::ru32(rbx.wrapping_add(0x6cu64))) as i32 as i64) as u64) as u64; rcx = rdi; r9 = ((crate::simd_ops::ru8(rbx.wrapping_add(0x2u64))) as u32) as u32 as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x50u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { r8 = (((crate::simd_ops::ru32(rbx.wrapping_add(0x60u64))) as i32 as i64) as u64) as u64; rcx = rdi; r9 = ((crate::simd_ops::ru8(rbx.wrapping_add(0x2u64))) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { r9 = ((crate::simd_ops::ru8(rbx.wrapping_add(0x2u64))) as u32) as u32 as u64; r8 = 0x88; rdx = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rcx = rdi; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210cc70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x68; const P1: u64 = 0x30; const P2: u64 = 0x8; const P3: u64 = 0x4;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rax = (((crate::simd_ops::ru32(rcx.wrapping_add(P0))) as i32 as i64) as u64) as u64; r10 = 0; fl_l = 0; fl_r = 0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; r11 = rax; r9 = ((r10 as u32)) as u32 as u64; rax = ((rbx.wrapping_add(P2)) as u64) as u64; bb = 2; continue; }
            2 => { rcx = ((crate::simd_ops::ru8(rax.wrapping_add(P2))) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rax)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rax.wrapping_add(P3))) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 7; continue; } bb = 5; continue; }
            5 => { r10 = (((r10 as u32)).wrapping_add(1)) as u32 as u64; r9 = ((r9).wrapping_add(1)) as u64; rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = r9; fl_r = r11; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 2; continue; } bb = 6; continue; }
            6 => { rax = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; return rax; }
            7 => { rax = ((((r10 as u32)) as i32 as i64) as u64) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((rbx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210cce0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x60; const P1: u64 = 0x18; const P2: u64 = 0x8; const P3: u64 = 0x20; const P4: u64 = 0x4; const P5: u64 = 0x142107140; const P6: u64 = 0x1; const P7: u64 = 0x2; const P8: u64 = 0x3; const P9: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = ((rcx.wrapping_add(P0)) as u64) as u64; rbp = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rdi = rdx; rsi = rcx; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; fl_r = rdi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rbx = ((rax.wrapping_add(P3)) as u64) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 5; continue; }
            5 => { r8 = ((crate::simd_ops::ru8(rcx.wrapping_add(P4))) as u32) as u32 as u64; rdx = 0x28; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P1))) as u64; r8 = rax; rdx = ((crate::simd_ops::ru8(rcx.wrapping_add(P3))) as u32) as u32 as u64; crate::simd_ops::wu8(rax, (0xbu8) as u8); fl_l = ((((rdx as u8)) & (0x3u8))) as u8 as u64; fl_r = fl_l; rdx = (rdx & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rax.wrapping_add(P6), ((rdx as u8)) as u8); rcx = ((crate::simd_ops::ru8(rsi.wrapping_add(P4))) as u32) as u32 as u64; crate::simd_ops::wu8(rax.wrapping_add(P7), ((rcx as u8)) as u8); crate::simd_ops::wu8(rax.wrapping_add(P8), (0x0u8) as u8); crate::simd_ops::wu64(rax.wrapping_add(P2), rdi); rcx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rax.wrapping_add(P3), rcx); rcx = rbp.wrapping_add(0xa78); crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rax.wrapping_add(P9), rcx); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0xa90u64))) as u64; crate::simd_ops::wu64(r8.wrapping_add(P1), rax); crate::simd_ops::wu64(rax.wrapping_add(P9), r8); rax = r8; crate::simd_ops::wu64(rbp.wrapping_add(0xa90u64), r8); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210cda0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x60; const P1: u64 = 0x8; const P2: u64 = 0x20; const P3: u64 = 0x10; const P4: u64 = 0x18; const P5: u64 = 0x1; const P6: u64 = 0x21; const P7: u64 = 0x14210c5f0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r11 = rdx; r10 = rcx; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; fl_r = r11; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 12; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(r8.wrapping_add(P2))) as u64; rdx = ((r8.wrapping_add(P3)) as u64) as u64; crate::simd_ops::wu64(r10.wrapping_add(P0), rax); rcx = (crate::simd_ops::ru64(r8.wrapping_add(P4))) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P3), rax); rax = (crate::simd_ops::ru64(r8.wrapping_add(P4))) as u64; rcx = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P4), rax); rax = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_add(P3), ((xmm1 as u64)) as u64); rax = ((crate::simd_ops::ru8(r8.wrapping_add(P5))) as u32) as u32 as u64; crate::simd_ops::wu64(r8.wrapping_add(P1), rdx); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x7u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 11; continue; } bb = 4; continue; }
            4 => { r9 = (crate::simd_ops::ru64(r10.wrapping_add(P4))) as u64; rcx = ((crate::simd_ops::ru8(r9.wrapping_add(P6))) as u32) as u32 as u64; rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_sub(1)) as u8 as u64)); fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) <= ((fl_r as u8))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = ((crate::simd_ops::ru8(r9.wrapping_add(P2))) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xf8u8))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0x3u8))) as u8 as u64)); fl_l = ((((rcx as u8)) | ((rax as u8)))) as u8 as u64; fl_r = fl_l; rcx = (rcx & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(r8.wrapping_add(P5), ((rcx as u8)) as u8); bb = 11; continue; }
            6 => { rax = (rax & !0xffu64) | ((((((rax as u8)) | (0x4u8))) as u8 as u64)); crate::simd_ops::wu8(r8.wrapping_add(P5), ((rax as u8)) as u8); fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64; fl_r = 0x5; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 11; continue; } bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P5))) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { r9 = (crate::simd_ops::ru64(r10.wrapping_add(P4))) as u64; rcx = ((crate::simd_ops::ru8(r9.wrapping_add(P6))) as u32) as u32 as u64; rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_sub(1)) as u8 as u64)); fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) <= ((fl_r as u8))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = ((crate::simd_ops::ru8(r9.wrapping_add(P2))) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xf8u8))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0x3u8))) as u8 as u64)); fl_l = ((((rcx as u8)) | ((rax as u8)))) as u8 as u64; fl_r = fl_l; rcx = (rcx & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(r8.wrapping_add(P5), ((rcx as u8)) as u8); bb = 11; continue; }
            10 => { rcx = r9; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            11 => { r8 = (crate::simd_ops::ru64(r10.wrapping_add(P0))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 12; continue; }
            12 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210ce90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x142107140; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x1; const P5: u64 = 0x2; const P6: u64 = 0x8; const P7: u64 = 0x6c; const P8: u64 = 0x60; const P9: u64 = 0x10; const P10: u64 = 0x84; const P11: u64 = 0x38;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    r8 = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rdx = 0x88;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64;
    r8 = ((crate::simd_ops::ru8(rdx.wrapping_add(P3))) as u32) as u32 as u64;
    r8 = (r8 & !0xffu64) | ((((((r8 as u8)) & (0x3u8))) as u8 as u64));
    crate::simd_ops::wu8(rax, (0xau8) as u8);
    crate::simd_ops::wu8(rax.wrapping_add(P4), ((r8 as u8)) as u8);
    rcx = ((crate::simd_ops::ru8(rbx.wrapping_add(P0))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P5), ((rcx as u8)) as u8);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rax.wrapping_add(P6), rcx);
    crate::simd_ops::wu64(rax.wrapping_add(P7), rcx);
    crate::simd_ops::wu64(rax.wrapping_add(P2), rcx);
    crate::simd_ops::wu64(rax.wrapping_add(P8), rcx);
    crate::simd_ops::wu64(rax.wrapping_add(P9), rcx);
    crate::simd_ops::wu32(rax.wrapping_add(P10), ((rcx as u32)) as u32);
    crate::simd_ops::wu64(rax.wrapping_add(P11), rcx);
    crate::simd_ops::wu64(rax.wrapping_add(0x74u64), rcx);
    crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rcx);
    crate::simd_ops::wu64(rax.wrapping_add(0x28u64), rcx);
    crate::simd_ops::wu32(rax.wrapping_add(0x68u64), ((rcx as u32)) as u32);
    crate::simd_ops::wu64(rax.wrapping_add(0x30u64), rcx);
    crate::simd_ops::wu64(rax.wrapping_add(0x40u64), rcx);
    crate::simd_ops::wu64(rax.wrapping_add(0x48u64), rcx);
    crate::simd_ops::wu64(rax.wrapping_add(0x50u64), rcx);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210cfc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4c; const P1: u64 = 0x20; const P2: u64 = 0x40; const P3: u64 = 0x14210dae0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x4e20;
    if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { return rax; }
    r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    rax = (0x6666666666666667u64) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rcx.wrapping_add(P2)))) as u64;
    let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128;
    rax = prod as u64; rdx = (prod >> 64) as u64;
    rdx = ((((rdx) as i64).wrapping_shr(4u32) as u64)) as u64;
    rax = rdx;
    rax = ((rax).wrapping_shr(63u32)) as u64;
    rdx = ((rdx).wrapping_add(rax)) as u64;
    rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64;
    fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x4e20;
    if !(((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32))) { rdx = 0x4e20; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210d010(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xffffffffffffffe8; const P2: u64 = 0xfffffffffffffff8; const P3: u64 = 0x10; const P4: u64 = 0x28; const P5: u64 = 0x48; const P6: u64 = 0x14210d8d0; const P7: u64 = 0x14210dd80;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rbx = rcx;
    xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(P1))) as u128;
    crate::simd_ops::wu128(rax, (xmm0) as u128);
    xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64) as u128;
    crate::simd_ops::wu64(rax.wrapping_add(P3), ((xmm1 as u64)) as u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    crate::simd_ops::wu128(rax.wrapping_add(P1), (xmm0) as u128);
    xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64) as u128;
    crate::simd_ops::wu64(rax.wrapping_add(P2), ((xmm1 as u64)) as u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64;
    rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rcx.wrapping_add(P0)))) as u64;
    fl_l = rax; fl_r = (0x18u64) as u64;
    if ((((fl_l as u64)) as i64) > (((fl_r as u64)) as i64)) { crate::simd_ops::lk_add_u64(rbx.wrapping_add(P0), (0x18u64) as u64); r8 = 0x1; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rcx = rbx; rdx = ((rdx).wrapping_add(0xffffffffffffffd0u64)) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
    rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P5))) as u32 as u64;
    fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1;
    if !(((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32))) { fl_l = (((rdx as u32)).wrapping_add((rdx as u32))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; } else { fl_l = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rdx = ((fl_l as u32)) as u32 as u64; }
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    crate::simd_ops::lk_add_u64(rbx.wrapping_add(P0), (0x18u64) as u64);
    r8 = 0x1;
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    rcx = rbx;
    rdx = ((rdx).wrapping_add(0xffffffffffffffd0u64)) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210d090(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x52; const P1: u64 = 0x50;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rdx = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    fl_l = (crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u16 as u64; fl_r = ((rdx as u16)) as u16 as u64;
    rax = (rax & !0xffu64) | (((if (((fl_l as u16)) <= ((fl_r as u16))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210d0b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x52; const P1: u64 = 0x50; const P2: u64 = 0x3; const P3: u64 = 0x1433e0998; const P4: u64 = 0x14210e640;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) > ((fl_r as u16))) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu8(rcx.wrapping_add(P2), (0x6u8) as u8); rax = 0xffffffff; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            2 => { rdx = P3; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14210d0d8, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210d0e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x52; const P1: u64 = 0x50; const P2: u64 = 0x3; const P3: u64 = 0x8; const P4: u64 = 0x10; const P5: u64 = 0x1433e09d0; const P6: u64 = 0x14210e640;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64; r8 = rcx; fl_l = (crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) > ((fl_r as u16))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; crate::simd_ops::wu8(r8.wrapping_add(P2), (0x1u8) as u8); rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(P3))) as u64; rcx = ((rcx).wrapping_shl(3u32)) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(P4), rax); rax = 0xffffffff; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            2 => { rdx = P5; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14210d122, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210d130(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1433e0a08; const P1: u64 = 0x142103330; const P2: u64 = 0x14210d8d0; const P3: u64 = 0x1433e0a50; const P4: u64 = 0x14210d4f0; const P5: u64 = 0x14210d8a0; const P6: u64 = 0x14210d4c0; const P7: u64 = 0x14210d3a0; const P8: u64 = 0x14210d2c0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x3u64))) as u32) as u32 as u64; rbx = rcx; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 1; continue; }
            1 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x6u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 2; continue; }
            2 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x40u64))) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; rdx = P0; r8 = 0x25; rdi = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rdi); bb = 5; continue; }
            5 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rax); crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), (0x5u32) as u32); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rbx.wrapping_add(0x8u64)))) as u64; fl_l = rax; fl_r = (0x18u64) as u64; if ((((fl_l as u64)) as i64) > (((fl_r as u64)) as i64)) { bb = 9; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32 as u64; rcx = rbx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = (((rdx as u32)).wrapping_add((rdx as u32))) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0x18u64) as u64); rax = 0x2; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            8 => { rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0x18u64) as u64); rax = 0x2; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            10 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 11; continue; }
            11 => { rax = ((crate::simd_ops::ru16(rdx.wrapping_add(0x50u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rcx.wrapping_add(0x50u64), ((rax as u16)) as u16); rcx = 0xc8; fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rcx as u16)) as u16 as u64; if (((fl_l as u16)) < ((fl_r as u16))) { bb = 14; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; rdx = P3; r8 = 0x10; rcx = rbx; rdi = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rdi); bb = 5; continue; }
            13 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu16(rcx.wrapping_add(0x50u64), ((rax as u16)) as u16); bb = 14; continue; }
            14 => { rax = (rax & !0xffffu64) | (((((rax as u16)).wrapping_add(1)) as u16 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x5u64), (0x1u8) as u8); crate::simd_ops::wu16(rbx.wrapping_add(0x50u64), ((rax as u16)) as u16); crate::simd_ops::wu16(rbx.wrapping_add(0x52u64), ((rax as u16)) as u16); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x1u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rbx); bb = 16; continue; }
            16 => { rax = ((((r8 as u32)) as i32 as i64) as u64) as u64; rdx = P4; r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(3u32)) as u64; r8 = ((r8).wrapping_sub(rcx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); r8 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 19; continue; } bb = 17; continue; }
            17 => { rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { crate::simd_ops::wu8(rbx.wrapping_add(0x3u64), ((r8 as u8)) as u8); rdx = P7; r8 = rax; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); r8 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 17; continue; } bb = 19; continue; }
            19 => { rdx = ((r8 as u32)) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_sub_u16(rbx.wrapping_add(0x50u64), (1) as u16); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x3u64))) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210d2c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x52; const P1: u64 = 0x50; const P2: u64 = 0x5; const P3: u64 = 0x8; const P4: u64 = 0x3; const P5: u64 = 0x14210d670; const P6: u64 = 0x20; const P7: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rbx = rcx;
    crate::simd_ops::wu16(rcx.wrapping_add(P1), ((rax as u16)) as u16);
    crate::simd_ops::wu8(rcx.wrapping_add(P2), (0x0u8) as u8);
    fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; crate::simd_ops::wu8(rcx.wrapping_add(P4), ((rdx as u8)) as u8); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P7), rax); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P4))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if (((fl_l as u8)) != ((fl_r as u8))) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P6))) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64;
    fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P7))) as u64; fl_r = rcx;
    if !((((fl_l as u64)) >= ((fl_r as u64)))) { crate::simd_ops::wu64(rax.wrapping_add(P7), rcx); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210d320(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x8; const P2: u64 = 0x142103330; const P3: u64 = 0x10; const P4: u64 = 0x28; const P5: u64 = 0x48; const P6: u64 = 0x14210d8d0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rcx; r8 = (0xffffffffffffffffu64) as u64; rdi = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P1), rdi); bb = 1; continue; }
            1 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rax); crate::simd_ops::wu32(rdi.wrapping_add(P3), (0x5u32) as u32); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rbx.wrapping_add(P1)))) as u64; fl_l = rax; fl_r = (0x18u64) as u64; if ((((fl_l as u64)) as i64) > (((fl_r as u64)) as i64)) { bb = 7; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64; rcx = rbx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = (((rdx as u32)).wrapping_add((rdx as u32))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; bb = 6; continue; }
            5 => { fl_l = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rdx = ((fl_l as u32)) as u32 as u64; bb = 6; continue; }
            6 => { rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { crate::simd_ops::lk_add_u64(rbx.wrapping_add(P1), (0x18u64) as u64); rax = 0x2; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210d3a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14210d670; const P1: u64 = 0x14210cda0; const P2: u64 = 0x1420f6ae0; const P3: u64 = 0x1420f2500; const P4: u64 = 0x1420ede50;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; r14 = rdx; rbx = rcx; rbp = (crate::simd_ops::ru64(rax)) as u64; rax = ((crate::simd_ops::ru16(rcx.wrapping_add(0x52u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rcx.wrapping_add(0x50u64), ((rax as u16)) as u16); crate::simd_ops::lk_and_u32(rdx.wrapping_add(0x24u64), (0xfffffffdu32) as u32); rdi = ((crate::simd_ops::ru8(rcx.wrapping_add(0x3u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x0u8) as u8); fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rdx = ((rdi as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(r14)) as u64; rdx = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rcx = rbx; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x10u64), rax); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x20u64))) as u64; r14 = ((r14).wrapping_sub(crate::simd_ops::ru64(rbx.wrapping_add(0x40u64)))) as u64; rax = crate::simd_ops::call_stk(rax, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rcx = rbx; rdx = ((rdx).wrapping_add(r14)) as u64; rdi = ((((rax as u32)) as i32 as i64) as u64) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), rdx); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(3u32)) as u64; rdx = ((rdx).wrapping_sub(rcx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x3u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 11; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 11; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rcx = rbx; rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x3u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(rax, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x3u64))) as u8 as u64; fl_r = (0x6u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = (rax as u32 as i32 as i64) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(3u32)) as u64; rdx = ((rdx).wrapping_sub(rcx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x6u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            9 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x3u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 3; continue; } bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210d4c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x40; const P2: u64 = 0x24;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P2))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = ((rax).wrapping_sub(0x28u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210d4f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1420f6b90; const P1: u64 = 0x1420f6ae0; const P2: u64 = 0x1420f2500; const P3: u64 = 0x1420ede50; const P4: u64 = 0x1433e0a30; const P5: u64 = 0x14210e640;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x3u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbx = rcx; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = rdx; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 2; continue; }
            2 => { rdx = ((rdx).wrapping_add(0xffffffffffffffe8u64)) as u64; r8 = 0xffffffff; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 18; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; rdi = rbx.wrapping_add(0x20); fl_l = (crate::simd_ops::lk_or_u32(rax.wrapping_add(0x24u64), (0x1u32) as u32)) as u32 as u64; fl_r = fl_l; bb = 8; continue; }
            4 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; rdi = rcx.wrapping_add(0x20); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x0u8) as u8); rax = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x3u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            7 => { rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x3u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 18; continue; } bb = 9; continue; }
            9 => { bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 18; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rcx = rbx; rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x3u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 14; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(rax, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x3u64))) as u8 as u64; fl_r = (0x6u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 18; continue; } bb = 13; continue; }
            13 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = (rax as u32 as i32 as i64) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(3u32)) as u64; rdx = ((rdx).wrapping_sub(rcx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            14 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x6u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            16 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x3u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 18; continue; }
            18 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            19 => { rdx = P4; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14210d5e1, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210d5f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x3; const P1: u64 = 0x20; const P2: u64 = 0x40; const P3: u64 = 0x8; const P4: u64 = 0x1420f6ae0; const P5: u64 = 0x6; const P6: u64 = 0x1420f2500; const P7: u64 = 0x1420ede50;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbx = rcx; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 10; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 10; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; rcx = rbx; rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(rax, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P0))) as u8 as u64; fl_r = (0x6u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; rax = (rax as u32 as i32 as i64) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(3u32)) as u64; rdx = ((rdx).wrapping_sub(rcx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P5))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            8 => { rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 10; continue; }
            10 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210d670(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x16; const P1: u64 = 0x1433e0400; const P2: u64 = 0x1433e0418; const P3: u64 = 0x142103330; const P4: u64 = 0x10; const P5: u64 = 0x8; const P6: u64 = 0xffffffffffffffe8; const P7: u64 = 0xfffffffffffffff8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = r8; rdi = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x2; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 1; continue; }
            1 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 2; continue; }
            2 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 4; continue; }
            4 => { r8 = ((rdx.wrapping_add(P0)) as u32) as u32 as u64; rdx = P1; bb = 6; continue; }
            5 => { r8 = 0x11; rdx = P2; bb = 6; continue; }
            6 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu32(rbx.wrapping_add(P4), (0x5u32) as u32); rbx = ((rbx).wrapping_add(0x18u64)) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P5), rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P5))) as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(P6))) as u128; crate::simd_ops::wu128(r8, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(P7))) as u64) as u128; crate::simd_ops::wu64(r8.wrapping_add(P4), ((xmm1 as u64)) as u64); bb = 8; continue; }
            8 => { rbx = ((rbx).wrapping_add(0x18u64)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P5), rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210d710(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; r9 = rcx; r8 = ((r8).wrapping_sub(rdx)) as u64; r10 = rdx; r11 = (0x2aaaaaaaaaaaaaabu64) as u64; rax = r11; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(r9.wrapping_add(0x30u64))) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(r9.wrapping_add(0x8u64), rcx); rcx = (crate::simd_ops::ru64(r9.wrapping_add(0x60u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rax = r11; rdx = ((rdx).wrapping_sub(r10)) as u64; let prod = (rax as i64 as i128).wrapping_mul((rdx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(r9.wrapping_add(0x30u64))) as u64; rdx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rdx = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rdx); rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 3; continue; }
            3 => { r8 = (crate::simd_ops::ru64(r9.wrapping_add(0x40u64))) as u64; fl_l = r8; fl_r = (crate::simd_ops::ru64(r9.wrapping_add(0x20u64))) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; rax = r11; rcx = ((rcx).wrapping_sub(r10)) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(r9.wrapping_add(0x30u64))) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = r11; crate::simd_ops::wu64(r8.wrapping_add(0x10u64), rcx); rcx = (crate::simd_ops::ru64(r8)) as u64; rcx = ((rcx).wrapping_sub(r10)) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(r9.wrapping_add(0x30u64))) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = r11; crate::simd_ops::wu64(r8, rcx); rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; rcx = ((rcx).wrapping_sub(r10)) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rcx = (crate::simd_ops::ru64(r9.wrapping_add(0x30u64))) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rdx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rdx = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x8u64), rdx); r8 = ((r8).wrapping_add(0x28u64)) as u64; fl_l = r8; fl_r = (crate::simd_ops::ru64(r9.wrapping_add(0x20u64))) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(r9.wrapping_add(0x10u64))) as u64; rax = r11; rcx = ((rcx).wrapping_sub(r10)) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; fl_l = ((rdx).wrapping_add(rax)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(r9.wrapping_add(0x30u64))) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(r9.wrapping_add(0x10u64), rcx); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210d870(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411f9b80; const P1: u64 = 0x1437fda88; const P2: u64 = 0x14183acce;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x58u64)) as u64; r8 = ((rdx as u32)) as u32 as u64; rdx = rcx; rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = P1; rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14210d895, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210d8a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = ((rsp).wrapping_sub(0x38u64)) as u64; rax = rdx; rdx = r8; rax = crate::simd_ops::call_stk(rax, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            1 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; bb = 2; continue; }
            2 => { rsp = ((rsp).wrapping_add(0x38u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210d8d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142107000; const P1: u64 = 0x1421074f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rsi = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; rbp = ((rdx.wrapping_add(0x5u64)) as u32) as u32 as u64; r14 = ((((rdx as u32)) as i32 as i64) as u64) as u64; rax = (0xaaaaaaaaaaaaaaau64) as u64; rbx = rcx; rdi = r14.wrapping_add(0x5); fl_l = rdi; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 10; continue; } bb = 1; continue; }
            1 => { rax = (((crate::simd_ops::ru32(rcx.wrapping_add(0x48u64))) as i32 as i64) as u64) as u64; r9 = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; r9 = ((r9).wrapping_shl(3u32)) as u64; rdx = rsi; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x2u64))) as u32) as u32 as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((rax as u8)) as u8); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r8 = (((crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as i32 as i64) as u64) as u64; r9 = rax; crate::simd_ops::wu64(rbx.wrapping_add(0x30u64), rax); fl_l = r8; fl_r = rdi; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 4; continue; } bb = 2; continue; }
            2 => { rdx = r8.wrapping_add(0x1); rdi = ((rdi).wrapping_sub(r8)) as u64; rdx = ((r8.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rdx = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; rax = 0; fl_l = 0; fl_r = 0; bb = 3; continue; }
            3 => { crate::simd_ops::wu32(rdx, ((rax as u32)) as u32); rdx = rdx.wrapping_add(0x18); fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x60u64))) as u64; rax = ((r14.wrapping_add(r14.wrapping_mul(2))) as u64) as u64; rcx = ((r9.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x48u64), ((rbp as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), rcx); r9 = (0x2aaaaaaaaaaaaaabu64) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = r9; rcx = ((rcx).wrapping_sub(rsi)) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rcx); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; rax = r9; rcx = ((rcx).wrapping_sub(rsi)) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x8u64), rcx); r8 = (crate::simd_ops::ru64(r8.wrapping_add(0x20u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 7; continue; }
            7 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; fl_l = r8; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; rax = r9; rcx = ((rcx).wrapping_sub(rsi)) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = r9; crate::simd_ops::wu64(r8.wrapping_add(0x10u64), rcx); rcx = (crate::simd_ops::ru64(r8)) as u64; rcx = ((rcx).wrapping_sub(rsi)) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = r9; crate::simd_ops::wu64(r8, rcx); rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; rcx = ((rcx).wrapping_sub(rsi)) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rdx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rdx = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x8u64), rdx); r8 = ((r8).wrapping_add(0x28u64)) as u64; fl_l = r8; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = r9; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rcx = ((rcx).wrapping_sub(rsi)) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rcx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            10 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14210daca, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210dae0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x4c; const P2: u64 = 0x2; const P3: u64 = 0x142107000; const P4: u64 = 0x20; const P5: u64 = 0xffffffffffffffd8; const P6: u64 = 0x38; const P7: u64 = 0x1421074f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbp = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = rcx; rcx = (0x666666666666666u64) as u64; rsi = ((((rdx as u32)) as i32 as i64) as u64) as u64; fl_l = rsi; fl_r = rcx; rcx = rdi; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((rsi.wrapping_add(rsi.wrapping_mul(4))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rbx = ((rax.wrapping_mul(8)) as u64) as u64; rdx = rbp; rax = (((crate::simd_ops::ru32(rdi.wrapping_add(P1))) as i32 as i64) as u64) as u64; r9 = rbx; r8 = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((crate::simd_ops::ru8(rdi.wrapping_add(P2))) as u32) as u32 as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((rax as u8)) as u8); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; r8 = rax; rcx = ((rcx).wrapping_sub(rbp)) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P0), rax); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = (0x6666666666666667u64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; crate::simd_ops::wu32(rdi.wrapping_add(P1), ((rsi as u32)) as u32); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdx = ((((rdx) as i64).wrapping_shr(4u32) as u64)) as u64; rcx = rdx; rcx = ((rcx).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; rcx = ((r8.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = ((rbx.wrapping_add(P5)) as u64) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P4), rcx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = ((rax).wrapping_add(r8)) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P6), rax); rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14210db9c, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210dcd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x50; const P1: u64 = 0x1433e0a50; const P2: u64 = 0x14210e640; const P3: u64 = 0x14210d870;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64; rdx = 0xc8; fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rdx as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rdx = 0xe1; fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rdx as u16)) as u16 as u64; if (((fl_l as u16)) >= ((fl_r as u16))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            3 => { rdx = P1; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14210dcfd, rcx, rsp); return rax; }
            4 => { rdx = 0x5; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14210dd08, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210dd10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4c; const P1: u64 = 0x14210dae0; const P2: u64 = 0x20; const P3: u64 = 0x14210d870; const P4: u64 = 0x1433e0a68; const P5: u64 = 0x14210e640;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rdx = 0x57e4; rbx = rcx; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 5; continue; } bb = 1; continue; }
            1 => { rcx = ((rax.wrapping_add(rax)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x4e20; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = 0x4e20; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rdx = (if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { (rcx as u32) } else { (rdx as u32) }) as u32 as u64; bb = 3; continue; }
            3 => { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; fl_r = 0x4e20; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rax = ((rax).wrapping_add(0x28u64)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P2), rax); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rdx = 0x5; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14210dd66, rcx, rsp); return rax; }
            6 => { rdx = P4; rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14210dd76, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210dd80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x50; const P1: u64 = 0x1420f6b90; const P2: u64 = 0x20; const P3: u64 = 0x24; const P4: u64 = 0x1; const P5: u64 = 0x5; const P6: u64 = 0x18; const P7: u64 = 0x30; const P8: u64 = 0x68; const P9: u64 = 0x6; const P10: u64 = 0x1420f2500; const P11: u64 = 0x1420ede50; const P12: u64 = 0x14210c770; const P13: u64 = 0x1433e0a50; const P14: u64 = 0x14210e640; const P15: u64 = 0x14210d870;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64; rbx = rcx; rax = (rax & !0xffffu64) | (((((rax as u16)).wrapping_add(1)) as u16 as u64)); crate::simd_ops::wu16(rcx.wrapping_add(P0), ((rax as u16)) as u16); rcx = 0xc8; fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rcx as u16)) as u16 as u64; if (((fl_l as u16)) < ((fl_r as u16))) { bb = 3; continue; } bb = 1; continue; }
            1 => { if (((fl_l as u16)) == ((fl_r as u16))) { bb = 14; continue; } bb = 2; continue; }
            2 => { rcx = 0xe1; fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rcx as u16)) as u16 as u64; if (((fl_l as u16)) >= ((fl_r as u16))) { bb = 15; continue; } bb = 3; continue; }
            3 => { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 11; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); crate::simd_ops::lk_or_u32(rax.wrapping_add(P3), (0x1u32) as u32); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P4))) as u32) as u32 as u64; rdi = ((crate::simd_ops::ru8(rbx.wrapping_add(P5))) as u32) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(P5), (0x1u8) as u8); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(P4), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(P7))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P8), rax); crate::simd_ops::wu64(rcx.wrapping_add(P7), rbx); bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P9))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rcx = rbx; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            8 => { rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu8(rbx.wrapping_add(0x5u64), (0x0u8) as u8); bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rax = 0xffff; crate::simd_ops::lk_add_u16(rbx.wrapping_add(0x50u64), ((rax as u16)) as u16); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x40u64))) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x48u64))) as u64; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P12, rax, rcx, rdx, r8, r9, rsp); }
            13 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            14 => { rdx = P13; rcx = rbx; rax = crate::simd_ops::call_stk(P14, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14210de4c, rcx, rsp); return rax; }
            15 => { rdx = 0x5; rcx = rbx; rax = crate::simd_ops::call_stk(P15, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14210de5a, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210de70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x48; const P1: u64 = 0x14210d8d0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    r9 = ((rdx as u32)) as u32 as u64;
    fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64;
    rax = ((r8.wrapping_add(rdx)) as u32) as u32 as u64;
    rdx = ((r8.wrapping_add(r8)) as u32) as u32 as u64;
    rdx = (if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { (rax as u32) } else { (rdx as u32) }) as u32 as u64;
    return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210de90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x3; const P4: u64 = 0x18; const P5: u64 = 0x40; const P6: u64 = 0x14; const P7: u64 = 0x142105cd0; const P8: u64 = 0x14210e350; const P9: u64 = 0x1433e0bb8; const P10: u64 = 0x142105ae0; const P11: u64 = 0x142119540;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 39>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x130u64)) as u64; rsi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = rdx; rbx = rcx; rax = (crate::simd_ops::ru64(rsi.wrapping_add(P1))) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; fl_r = 0x7; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P3))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; rcx = rsp.wrapping_add(0x30); r8 = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; r9 = (crate::simd_ops::ru32(r8.wrapping_add(P6))) as u32 as u64; r8 = ((r8).wrapping_add(0x18u64)) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rdx = rsi; r10 = rax; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); r9 = ((rax as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); r8 = r10; rdx = P9; rcx = rbx; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            3 => { rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { r11 = rsp.wrapping_add(0x130); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210df20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x3; const P1: u64 = 0x28; const P2: u64 = 0x18; const P3: u64 = 0x48;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { return rax; }
            3 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; rax = 0; fl_l = 0; fl_r = 0; fl_l = rdx; fl_r = rdx; rcx = ((rdx.wrapping_add(P2)) as u64) as u64; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rcx } else { rax }) as u64; bb = 4; continue; }
            4 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210df50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143863a08; const P1: u64 = 0x142117750; const P2: u64 = 0x14210e090; const P3: u64 = 0x14210d8d0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rsi = 0; fl_l = 0; fl_r = 0; r10 = ((((rdx as u32)) as i32 as i64) as u64) as u64; rbp = r9; r14 = r8; rbx = rcx; rdi = ((rsi as u32)) as u32 as u64; r11 = ((rsi as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = ((r10 as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = ((r10.wrapping_add(r10.wrapping_mul(2))) as u64) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; bb = 7; continue; }
            4 => { r11 = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; rax = (0x6666666666666667u64) as u64; rcx = r11; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rbx.wrapping_add(0x40u64)))) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(4u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 17; continue; } bb = 5; continue; }
            5 => { rcx = ((r10.wrapping_add(r10.wrapping_mul(4))) as u64) as u64; rcx = ((rcx).wrapping_shl(3u32)) as u64; fl_l = r11; fl_r = rcx; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rax = (crate::simd_ops::ru64(r11.wrapping_add(0x8u64))) as u64; bb = 6; continue; }
            6 => { rdi = (crate::simd_ops::ru64(rax)) as u64; bb = 7; continue; }
            7 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 8; continue; }
            8 => { r9 = rdi; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r11); r8 = rbp; rdx = r14; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r8 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 9; continue; }
            9 => { rdx = ((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as u32) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = ((((rdx as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rdx = (rdx & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x1u64), ((rdx as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rbx); bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rax, r8); crate::simd_ops::wu32(rax.wrapping_add(0x10u64), (0x7u32) as u32); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rbx.wrapping_add(0x8u64)))) as u64; fl_l = rax; fl_r = (0x18u64) as u64; if ((((fl_l as u64)) as i64) > (((fl_r as u64)) as i64)) { bb = 16; continue; } bb = 12; continue; }
            12 => { rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32 as u64; rcx = rbx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = (((rdx as u32)).wrapping_add((rdx as u32))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; bb = 15; continue; }
            14 => { fl_l = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rdx = ((fl_l as u32)) as u32 as u64; bb = 15; continue; }
            15 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0x18u64) as u64)) as u64; fl_r = fl_l; bb = 17; continue; }
            17 => { fl_l = rdi; fl_r = rdi; bb = 18; continue; }
            18 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (rsi & !0xffu64) | (((if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = ((rsi as u32)) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210e090(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1433e0d54; const P1: u64 = 0x1433e0d58; const P2: u64 = 0x1433dd0d0; const P3: u64 = 0x140000000; const P4: u64 = 0x1433de094; const P5: u64 = 0x142105cd0;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; r14 = 0; fl_l = 0; fl_r = 0; rdi = r9; rsi = r8; rbx = rdx; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 37; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbp); r9 = P0; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r12); r12 = P1; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r13); r13 = P2; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); r15 = P3; bb = 2; continue; }
            2 => { rax = ((((rax as u8)) as i8 as i32) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_add(0xffffff9fu32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x14; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 35; continue; } bb = 3; continue; }
            3 => { rax = (rax as u32 as i32 as i64) as u64; rcx = (crate::simd_ops::ru32(r15.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x210e2c4u64))) as u32 as u64; fl_l = ((rcx).wrapping_add(r15)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; j_tgt = rcx; bb = 0xffff_fffe; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x3u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = P4; crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), r12); crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), r13); crate::simd_ops::wu32(rsi.wrapping_add(0x20u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rsi.wrapping_add(0x18u64), rax); bb = 35; continue; }
            6 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), r9); r8 = rdx.wrapping_add(0x18); crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), r8); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x7cu64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x20u64), ((rcx as u32)) as u32); rcx = rsi.wrapping_add(0x38); r9 = (crate::simd_ops::ru32(rdx.wrapping_add(0x14u64))) as u32 as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); r9 = P0; crate::simd_ops::wu64(rsi.wrapping_add(0x18u64), rax); bb = 35; continue; }
            7 => { fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x7; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 16; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x3u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 16; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rax.wrapping_add(0x10u64)))) as u64; rcx = ((((rcx) as i64).wrapping_shr(2u32) as u64)) as u64; fl_l = (((rcx as u32)).wrapping_sub(1)) as u32 as u64; rcx = ((fl_l as u32)) as u32 as u64; bb = 12; continue; }
            12 => { r8 = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; r9 = (crate::simd_ops::ru64(r8.wrapping_add(0x20u64))) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rdx = 0; fl_l = 0; fl_r = 0; r9 = P0; crate::simd_ops::wu32(rsi.wrapping_add(0x24u64), ((rdx as u32)) as u32); bb = 35; continue; }
            14 => { rax = (((crate::simd_ops::ru32(r8.wrapping_add(0x78u64))) as i32 as i64) as u64) as u64; rdx = ((((rcx as u32)) as i32 as i64) as u64) as u64; rcx = ((((rdx) as i64).wrapping_shr((((rax) as u32) & 63)) as u64)) as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(0x28u64))) as u64; rdx = ((crate::simd_ops::ru8(r9.wrapping_add(rdx))) as u32) as u32 as u64; r9 = P0; fl_l = (((rdx as u32)).wrapping_add(crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4))))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x24u64), ((rdx as u32)) as u32); bb = 35; continue; }
            15 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x3u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rdx = 0xffffffff; crate::simd_ops::wu32(rsi.wrapping_add(0x24u64), ((rdx as u32)) as u32); bb = 35; continue; }
            17 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(0x7cu64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x24u64), ((rdx as u32)) as u32); bb = 35; continue; }
            18 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x4u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x28u64), ((rax as u8)) as u8); bb = 35; continue; }
            19 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x3u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 21; continue; } bb = 20; continue; }
            20 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rsi.wrapping_add(0x2au64), (0x1u8) as u8); crate::simd_ops::wu8(rsi.wrapping_add(0x29u64), ((rax as u8)) as u8); bb = 35; continue; }
            21 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rcx = ((crate::simd_ops::ru8(rax.wrapping_add(0x86u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x2au64), ((rcx as u8)) as u8); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rax = ((crate::simd_ops::ru8(rax.wrapping_add(0x85u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x29u64), ((rax as u8)) as u8); bb = 35; continue; }
            22 => { fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x3u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 27; continue; } bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 26; continue; } bb = 25; continue; }
            25 => { rax = 0; fl_l = 0; fl_r = 0; bb = 26; continue; }
            26 => { crate::simd_ops::wu64(rsi, rax); bb = 35; continue; }
            27 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; bb = 32; continue; }
            28 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x3u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 31; continue; } bb = 29; continue; }
            29 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 30; continue; }
            30 => { crate::simd_ops::wu64(rsi, rax); bb = 35; continue; }
            31 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; bb = 32; continue; }
            32 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 33; continue; }
            33 => { fl_l = ((rax).wrapping_add(0x18u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsi, rax); bb = 35; continue; }
            34 => { r14 = rdi; bb = 35; continue; }
            35 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 2; continue; } bb = 36; continue; }
            36 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 37; continue; }
            37 => { rax = r14; rsp = ((rsp).wrapping_add(0x28u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x14210e090u64) { 0x21 => { bb = 1; continue; } 0x60 => { bb = 2; continue; } 0x6f => { bb = 3; continue; } 0x7e => { bb = 4; continue; } 0x84 => { bb = 5; continue; } 0xa3 => { bb = 6; continue; } 0xde => { bb = 7; continue; } 0xe3 => { bb = 8; continue; } 0xed => { bb = 9; continue; } 0xf6 => { bb = 10; continue; } 0xff => { bb = 11; continue; } 0x10d => { bb = 12; continue; } 0x11a => { bb = 13; continue; } 0x12b => { bb = 14; continue; } 0x152 => { bb = 15; continue; } 0x158 => { bb = 16; continue; } 0x165 => { bb = 17; continue; } 0x174 => { bb = 18; continue; } 0x180 => { bb = 19; continue; } 0x186 => { bb = 20; continue; } 0x191 => { bb = 21; continue; } 0x1af => { bb = 22; continue; } 0x1b4 => { bb = 23; continue; } 0x1c1 => { bb = 24; continue; } 0x1ca => { bb = 25; continue; } 0x1cc => { bb = 26; continue; } 0x1d1 => { bb = 27; continue; } 0x1d7 => { bb = 28; continue; } 0x1dd => { bb = 29; continue; } 0x1e6 => { bb = 30; continue; } 0x1eb => { bb = 31; continue; } 0x1ef => { bb = 32; continue; } 0x1f8 => { bb = 33; continue; } 0x201 => { bb = 34; continue; } 0x204 => { bb = 35; continue; } 0x213 => { bb = 36; continue; } 0x227 => { bb = 37; continue; } _ => { return crate::simd_ops::tail_stk(j_tgt, rax, rcx, rdx, r8, r9, rsp); } } }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210e320(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x3; const P3: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64; fl_r = 0x7;
    if (((fl_l as u32)) != ((fl_r as u32))) { rax = 0; fl_l = 0; fl_r = 0; return rax; }
    rax = (crate::simd_ops::ru64(rax)) as u64;
    fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; return rax; }
    rax = 0;
    fl_l = 0; fl_r = 0;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210e350(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x20; const P4: u64 = 0x78; const P5: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = r8; fl_r = r8;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rax.wrapping_add(P2)))) as u64; r8 = ((((r8) as i64).wrapping_shr(2u32) as u64)) as u64; fl_l = (((r8 as u32)).wrapping_sub(1)) as u32 as u64; r8 = ((fl_l as u32)) as u32 as u64; }
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64;
    fl_l = r9; fl_r = r9;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { rax = 0; fl_l = 0; fl_r = 0; return rax; }
    rax = (((crate::simd_ops::ru32(rcx.wrapping_add(P4))) as i32 as i64) as u64) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P5))) as u64;
    r8 = ((((r8 as u32)) as i32 as i64) as u64) as u64;
    rdx = ((((r8) as i64).wrapping_shr((((rax) as u32) & 63)) as u64)) as u64;
    rax = ((crate::simd_ops::ru8(r9.wrapping_add(r8))) as u32) as u32 as u64;
    fl_l = (((rax as u32)).wrapping_add(crate::simd_ops::ru32(rcx.wrapping_add(rdx.wrapping_mul(4))))) as u32 as u64; fl_r = fl_l;
    rax = ((fl_l as u32)) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210e3b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x8; const P2: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; rdx = (crate::simd_ops::ru64(rcx)) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rcx.wrapping_add(P2)))) as u64; rax = ((((rax) as i64).wrapping_shr(2u32) as u64)) as u64; fl_l = (((rax as u32)).wrapping_sub(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; return rax; }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210e3f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x78; const P2: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = r9; fl_r = r9;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { rax = 0; fl_l = 0; fl_r = 0; return rax; }
    rax = (((crate::simd_ops::ru32(rcx.wrapping_add(P1))) as i32 as i64) as u64) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64;
    r8 = ((((rdx as u32)) as i32 as i64) as u64) as u64;
    rdx = ((((r8) as i64).wrapping_shr((((rax) as u32) & 63)) as u64)) as u64;
    rax = ((crate::simd_ops::ru8(r9.wrapping_add(r8))) as u32) as u32 as u64;
    fl_l = (((rax as u32)).wrapping_add(crate::simd_ops::ru32(rcx.wrapping_add(rdx.wrapping_mul(4))))) as u32 as u64; fl_r = fl_l;
    rax = ((fl_l as u32)) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210e420(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1420f83d0; const P1: u64 = 0x18; const P2: u64 = 0x1433e0bf0; const P3: u64 = 0x14210e640;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = r8; rdi = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r8 = ((rbx.wrapping_add(P1)) as u64) as u64; r9 = rax; rdx = P2; rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14210e44b, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210e460(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1420f83f0; const P1: u64 = 0x18; const P2: u64 = 0x142986560; const P3: u64 = 0x1433e0cb0; const P4: u64 = 0x14210e640; const P5: u64 = 0x1433e0c80;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = ((((r9 as u32)) as i32 as i64) as u64) as u64; rbx = r8; rbp = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = rbp; rsi = ((rax.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r8 = P2; rax = ((rax).wrapping_add(0x18u64)) as u64; r8 = (crate::simd_ops::ru64(r8.wrapping_add(rdi.wrapping_mul(8)))) as u64; r9 = rsi; r8 = ((r8).wrapping_add(0x2u64)) as u64; rcx = rbp; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = P3; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14210e4bb, rcx, rsp); return rax; }
            2 => { rdx = P5; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14210e4cd, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210e4e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1420f83f0; const P1: u64 = 0x10; const P2: u64 = 0x14; const P3: u64 = 0x18; const P4: u64 = 0x1433e0c40; const P5: u64 = 0x14210e640; const P6: u64 = 0x1433e0c20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = r8; rbx = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = rsi; rcx = rbx; rdi = rax; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(P1))) as u32 as u64; fl_r = 0x5; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 1; continue; }
            1 => { r9 = (crate::simd_ops::ru64(rsi)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(r9.wrapping_add(P2))) as u32 as u64; fl_r = 0x40; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { r9 = ((r9).wrapping_add(0x18u64)) as u64; r8 = ((rdi.wrapping_add(P3)) as u64) as u64; rdx = P4; rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14210e534, rcx, rsp); return rax; }
            4 => { r9 = ((rax.wrapping_add(P3)) as u64) as u64; rcx = rbx; r8 = ((rdi.wrapping_add(P3)) as u64) as u64; rdx = P6; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14210e54c, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210e560(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1420f83d0; const P1: u64 = 0x14339f760; const P2: u64 = 0x14339f75c; const P3: u64 = 0x14339f768; const P4: u64 = 0x1433e0c60; const P5: u64 = 0x14210e640;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rsi = ((r9 as u32)) as u32 as u64; rbx = r8; rdi = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = rdi; rbp = rax; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0xf; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { r9 = P1; bb = 3; continue; }
            2 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x10; rcx = P2; r9 = P3; r9 = (if (((fl_l as u32)) == ((fl_r as u32))) { rcx } else { r9 }) as u64; bb = 3; continue; }
            3 => { r8 = rbp; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rdx = P4; rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14210e5ca, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210e640(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x18; const P2: u64 = 0x20; const P3: u64 = 0x1405eb9f0; const P4: u64 = 0x14210de90; const P5: u64 = 0x14210d870;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 69>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(P0), rdx); crate::simd_ops::wu64(rax.wrapping_add(P1), r8); crate::simd_ops::wu64(rax.wrapping_add(P2), r9); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x220u64)) as u64; rbx = rcx; r9 = ((rax.wrapping_add(P1)) as u64) as u64; r8 = rdx; rcx = rsp.wrapping_add(0x20); rdx = 0x200; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = rsp.wrapping_add(0x20); rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = 0x2; rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14210e68a, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210e6e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1420f83d0; const P1: u64 = 0x1433e0ce0; const P2: u64 = 0x14210e640;
    let (mut rax, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = r8; rsi = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = rdi; rcx = rsi; rbx = rax; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r9 = rax; rdx = P1; r8 = rbx; rcx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14210e71d, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210e720(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1429868d0; const P1: u64 = 0x1420ecc90; const P2: u64 = 0x14210c770; const P3: u64 = 0x1420ecd00; const P4: u64 = 0x1421160a0; const P5: u64 = 0x14183acbc; const P6: u64 = 0x1433e0bb0; const P7: u64 = 0x14210df50; const P8: u64 = 0x1433e0d78; const P9: u64 = 0x1433e0d80; const P10: u64 = 0x142103230; const P11: u64 = 0x142103330; const P12: u64 = 0x142118a60; const P13: u64 = 0x1433e0dd8; const P14: u64 = 0x142116bc0; const P15: u64 = 0x1421158a0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 125>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x2c0); rsp = ((rsp).wrapping_sub(0x3c0u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; r13 = P0; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rbx = rcx; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = r13; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x9; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { r12 = (crate::simd_ops::ru64(rax)) as u64; rsi = 0x3; rcx = 0x18; r15 = ((rcx as u32)) as u32 as u64; r14 = ((rsi.wrapping_sub(0x1u64)) as u32) as u32 as u64; bb = 5; continue; }
            4 => { rsi = 0x2; rcx = 0; fl_l = 0; fl_r = 0; r12 = rbx; r15 = 0; fl_l = 0; fl_r = 0; r14 = ((rsi.wrapping_sub(0x1u64)) as u32) as u32 as u64; bb = 5; continue; }
            5 => { rdi = ((rax.wrapping_add(rcx)) as u64) as u64; fl_l = rdi; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 17; continue; } bb = 6; continue; }
            6 => { fl_l = rdi; fl_r = r13; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 17; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x5; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 15; continue; } bb = 9; continue; }
            9 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x1u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rbx); bb = 11; continue; }
            11 => { rdx = rdi; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 73; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x40u64))) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x48u64))) as u64; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdi = r13; rcx = ((rcx).wrapping_add(r15)) as u64; fl_l = rcx; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdi = (if (((fl_l as u64)) < ((fl_r as u64))) { rcx } else { rdi }) as u64; bb = 15; continue; }
            15 => { rdi = (crate::simd_ops::ru64(rdi)) as u64; fl_l = ((rdi).wrapping_add(0x18u64)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 73; continue; } bb = 16; continue; }
            16 => { bb = 18; continue; }
            17 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 18; continue; }
            18 => { rax = ((rsi as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = ((rax).wrapping_add(0xffffffffffffffe8u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 24; continue; } bb = 19; continue; }
            19 => { fl_l = rax; fl_r = r13; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 24; continue; } bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 24; continue; } bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rdx = rbp.wrapping_add(0x160); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 74; continue; } bb = 23; continue; }
            23 => { r14 = (crate::simd_ops::cvt_f64_i32((f64::from_bits(crate::simd_ops::ru64(rax))))) as u32 as u64; bb = 25; continue; }
            24 => { r14 = 0; fl_l = 0; fl_r = 0; fl_l = rbx; fl_r = r12; r14 = (r14 & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 25; continue; }
            25 => { fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 72; continue; } bb = 26; continue; }
            26 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rcx = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); rdx = rbp.wrapping_add(0x140); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (0x0u64) as u64); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 34; continue; } bb = 27; continue; }
            27 => { rbx = (0xffffffffffffffffu64) as u64; bb = 28; continue; }
            28 => { rbx = ((rbx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(rbx))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 28; continue; } bb = 29; continue; }
            29 => { fl_l = rbx; fl_r = (0x200u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 31; continue; } bb = 30; continue; }
            30 => { rax = rbx; rcx = rsp.wrapping_add(0x20); rax = ((rax).wrapping_sub(rdx)) as u64; r8 = 0xffffffff; rdx = rsp.wrapping_add(0x40); rdx = ((rdx).wrapping_add(rax)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 31; continue; }
            31 => { r8 = rbx; rdx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rcx = ((rcx).wrapping_add(rbx)) as u64; rax = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); rax = ((rax).wrapping_sub(rcx)) as u64; fl_l = rax; fl_r = (0x1u64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 33; continue; } bb = 32; continue; }
            32 => { rcx = ((rcx).wrapping_sub(rdx)) as u64; r8 = 0xffffffff; rdx = rcx.wrapping_add(0x1); rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 33; continue; }
            33 => { crate::simd_ops::wu8(rcx, (0xau8) as u8); fl_l = (crate::simd_ops::lk_add_u64(rsp.wrapping_add(0x20u64), (1) as u64)) as u64; bb = 34; continue; }
            34 => { r9 = rbp.wrapping_add(0x180); rdx = ((r14 as u32)) as u32 as u64; r8 = P6; rcx = r12; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 64; continue; } bb = 35; continue; }
            35 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x188u64))) as u64; fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = (0x43u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 37; continue; } bb = 36; continue; }
            36 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x1u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 63; continue; } bb = 37; continue; }
            37 => { fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x190u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 43; continue; } bb = 38; continue; }
            38 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x198u64))) as u64; rbx = (0xffffffffffffffffu64) as u64; bb = 39; continue; }
            39 => { rbx = ((rbx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(rbx))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 39; continue; } bb = 40; continue; }
            40 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = r8; rax = ((rax).wrapping_sub(rcx)) as u64; fl_l = rax; fl_r = rbx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 42; continue; } bb = 41; continue; }
            41 => { rdx = rbx; rdx = ((rdx).wrapping_sub(r8)) as u64; r8 = 0xffffffff; rdx = ((rdx).wrapping_add(rcx)) as u64; rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 42; continue; }
            42 => { r8 = rbx; rdx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = ((r9).wrapping_add(rbx)) as u64; fl_r = fl_l; r9 = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); bb = 44; continue; }
            43 => { r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 44; continue; }
            44 => { r8 = (crate::simd_ops::ru32(rbp.wrapping_add(0x1a4u64))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 52; continue; } bb = 45; continue; }
            45 => { rbx = rbp.wrapping_add(0x160); bb = 46; continue; }
            46 => { rbx = ((rbx).wrapping_sub(1)) as u64; rax = 0xcccccccd; let prod = (rax as u32 as u64).wrapping_mul(((r8 as u32)) as u32 as u64); rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; fl_l = (rdx != 0) as u64; rdx = (((rdx as u32)).wrapping_shr(3u32)) as u32 as u64; rax = (((rdx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shl(2u32)) as u8 as u64)); rcx = ((rax.wrapping_add(rdx)) as u32) as u32 as u64; rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_add((rcx as u8))) as u8 as u64)); r8 = (r8 & !0xffu64) | (((((r8 as u8)).wrapping_sub((rcx as u8))) as u8 as u64)); r8 = (r8 & !0xffu64) | (((((r8 as u8)).wrapping_add(0x30u8)) as u8 as u64)); crate::simd_ops::wu8(rbx, ((r8 as u8)) as u8); r8 = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 46; continue; } bb = 47; continue; }
            47 => { fl_l = r9; fl_r = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 49; continue; } bb = 48; continue; }
            48 => { rdx = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; r8 = ((rdx.wrapping_sub(0x2u64)) as u32) as u32 as u64; rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 49; continue; }
            49 => { crate::simd_ops::wu8(r9, (0x3au8) as u8); rdi = rbp.wrapping_add(0x160); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rdi = ((rdi).wrapping_sub(rbx)) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rcx = ((rcx).wrapping_add(1)) as u64; rax = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); rax = ((rax).wrapping_sub(rcx)) as u64; fl_l = rax; fl_r = rdi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 51; continue; } bb = 50; continue; }
            50 => { rcx = ((rcx).wrapping_sub(rdx)) as u64; r8 = 0xffffffff; rdx = ((rdi.wrapping_add(rcx)) as u64) as u64; rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 51; continue; }
            51 => { r8 = rdi; rdx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = ((r9).wrapping_add(rdi)) as u64; fl_r = fl_l; r9 = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); bb = 52; continue; }
            52 => { fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x180u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 60; continue; } bb = 53; continue; }
            53 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = rcx; rax = ((rax).wrapping_sub(r9)) as u64; fl_l = rax; fl_r = (0xau64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 55; continue; } bb = 54; continue; }
            54 => { r9 = ((r9).wrapping_sub(rcx)) as u64; r8 = 0xffffffff; rcx = rsp.wrapping_add(0x20); rdx = r9.wrapping_add(0xa); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 55; continue; }
            55 => { xmm0 = ((crate::simd_ops::ru64(P8)) as u64) as u128; crate::simd_ops::wu64(r9, ((xmm0 as u64)) as u64); rax = ((crate::simd_ops::ru16(P9)) as u32) as u32 as u64; rbx = (0xffffffffffffffffu64) as u64; crate::simd_ops::wu16(r9.wrapping_add(0x8u64), ((rax as u16)) as u16); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x180u64))) as u64; fl_l = ((rcx).wrapping_add(0xau64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); bb = 56; continue; }
            56 => { rbx = ((rbx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(rbx))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 56; continue; } bb = 57; continue; }
            57 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = rdx; rax = ((rax).wrapping_sub(rcx)) as u64; fl_l = rax; fl_r = rbx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 59; continue; } bb = 58; continue; }
            58 => { rcx = ((rcx).wrapping_sub(rdx)) as u64; r8 = 0xffffffff; rdx = ((rbx.wrapping_add(rcx)) as u64) as u64; rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 59; continue; }
            59 => { r8 = rbx; rdx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = ((r9).wrapping_add(rbx)) as u64; fl_r = fl_l; r9 = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); bb = 60; continue; }
            60 => { fl_l = r9; fl_r = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 62; continue; } bb = 61; continue; }
            61 => { rdx = 0x1; rcx = rsp.wrapping_add(0x20); r8 = ((rdx.wrapping_sub(0x2u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 62; continue; }
            62 => { crate::simd_ops::wu8(r9, (0xau8) as u8); fl_l = (crate::simd_ops::lk_add_u64(rsp.wrapping_add(0x20u64), (1) as u64)) as u64; bb = 63; continue; }
            63 => { r14 = (((r14 as u32)).wrapping_add(1)) as u32 as u64; r9 = rbp.wrapping_add(0x180); rdx = ((r14 as u32)) as u32 as u64; r8 = P6; rcx = r12; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 35; continue; } bb = 64; continue; }
            64 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 70; continue; } bb = 65; continue; }
            65 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x40u64))) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x48u64))) as u64; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 67; continue; } bb = 66; continue; }
            66 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 67; continue; }
            67 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rsi = 0x5; rcx = rbx; rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = r8; fl_r = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 69; continue; } bb = 68; continue; }
            68 => { rdx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi.wrapping_sub(0x18u64), rax); crate::simd_ops::wu32(rdi.wrapping_sub(0x8u64), ((rsi as u32)) as u32); bb = 71; continue; }
            69 => { r8 = ((r8).wrapping_sub(r14)) as u64; rdx = r14.wrapping_add(0x18); r8 = ((r8).wrapping_sub(0x18u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi.wrapping_sub(0x18u64), rax); crate::simd_ops::wu32(rdi.wrapping_sub(0x8u64), ((rsi as u32)) as u32); bb = 71; continue; }
            70 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = rsp.wrapping_add(0x20); r8 = ((r8).wrapping_sub(rcx)) as u64; rdx = rsp.wrapping_add(0x40); r8 = ((r8).wrapping_sub(0x20u64)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 71; continue; }
            71 => { r11 = rsp.wrapping_add(0x3c0); rax = 0x1; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            72 => { r8 = P13; rdx = ((rsi as u32)) as u32 as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x14210ec5c, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            73 => { r8 = 0x5; rdx = ((r14 as u32)) as u32 as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P15, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x14210ec6e, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            74 => { r8 = 0x3; rdx = ((rsi as u32)) as u32 as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P15, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x14210ec7f, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210ec90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1429868d0; const P1: u64 = 0x14210d8d0; const P2: u64 = 0x1420ecd00; const P3: u64 = 0x1420ecc90; const P4: u64 = 0x14210c770; const P5: u64 = 0x14210df50; const P6: u64 = 0x142118a60; const P7: u64 = 0x1426cdcb0; const P8: u64 = 0x142118910; const P9: u64 = 0x142117440; const P10: u64 = 0x1433e0d90; const P11: u64 = 0x142116bc0; const P12: u64 = 0x1421158a0; const P13: u64 = 0x1433e0db8; const P14: u64 = 0x1433e0dd8; const P15: u64 = 0x1433e0da0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 51>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x180u64)) as u64; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; r14 = P0; r12 = 0; fl_l = 0; fl_r = 0; rbx = rcx; fl_l = rdi; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 11; continue; } bb = 1; continue; }
            1 => { fl_l = rdi; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x9; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(rdi)) as u64; r15 = ((r12.wrapping_add(0x1u64)) as u32) as u32 as u64; rbp = rdi; fl_l = rcx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rdi.wrapping_add(0x8u64)))) as u64; fl_l = rax; fl_r = (0x18u64) as u64; if ((((fl_l as u64)) as i64) > (((fl_r as u64)) as i64)) { bb = 9; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x48u64))) as u32 as u64; rcx = rdi; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 7; continue; } bb = 6; continue; }
            6 => { fl_l = (((rdx as u32)).wrapping_add((rdx as u32))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; bb = 8; continue; }
            7 => { fl_l = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rdx = ((fl_l as u32)) as u32 as u64; bb = 8; continue; }
            8 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 12; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); bb = 12; continue; }
            11 => { r15 = ((r12 as u32)) as u32 as u64; rbp = rbx; bb = 12; continue; }
            12 => { rax = ((r15 as u32)) as u32 as u64; rcx = r14; rdi = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdx = ((rax.wrapping_add(rdi.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { rdx } else { rcx }) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 28; continue; } bb = 13; continue; }
            13 => { rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 28; continue; } bb = 14; continue; }
            14 => { fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 64; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 64; continue; } bb = 16; continue; }
            16 => { fl_l = rax; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 64; continue; } bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x7; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 64; continue; } bb = 18; continue; }
            18 => { rcx = ((rcx).wrapping_sub(rax)) as u64; rax = (0xd555555555555555u64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdi = rdx; rdi = ((((rdi) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rdi; rax = ((rax).wrapping_shr(63u32)) as u64; fl_l = ((rdi).wrapping_add(rax)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 19; continue; }
            19 => { rax = ((r15.wrapping_add(0x1u64)) as u32) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rsi = ((r15.wrapping_add(0x2u64)) as u32) as u32 as u64; rcx = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; rdx = r14; fl_l = rcx; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rcx } else { rdx }) as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x5; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 26; continue; } bb = 20; continue; }
            20 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x1u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rbx); bb = 22; continue; }
            22 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 63; continue; } bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x40u64))) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x48u64))) as u64; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 25; continue; } bb = 24; continue; }
            24 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; r8 = rsi.wrapping_sub(0x1); r8 = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; rdx = r14; r8 = ((rax.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { r8 } else { rdx }) as u64; bb = 26; continue; }
            26 => { r14 = (crate::simd_ops::ru64(rdx)) as u64; fl_l = ((r14).wrapping_add(0x18u64)) as u64; fl_r = fl_l; r14 = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 63; continue; } bb = 27; continue; }
            27 => { bb = 33; continue; }
            28 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdx = ((rax.wrapping_add(rdi.wrapping_mul(8))) as u64) as u64; rax = r14; fl_l = rdx; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = (if (((fl_l as u64)) < ((fl_r as u64))) { rdx } else { rax }) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 31; continue; } bb = 29; continue; }
            29 => { rdx = rsp.wrapping_add(0x20); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 31; continue; } bb = 30; continue; }
            30 => { rdi = ((r12 as u32)) as u32 as u64; bb = 19; continue; }
            31 => { rdi = (crate::simd_ops::cvt_f64_i32((f64::from_bits(crate::simd_ops::ru64(rax))))) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 65; continue; } bb = 32; continue; }
            32 => { bb = 19; continue; }
            33 => { r9 = rsp.wrapping_add(0x40); r8 = r14; rdx = ((rdi as u32)) as u32 as u64; rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 61; continue; } bb = 34; continue; }
            34 => { rcx = ((crate::simd_ops::ru8(r14)) as u32) as u32 as u64; rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rdi = ((r12 as u32)) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x38u64), ((rax as u16)) as u16); xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 60; continue; } bb = 35; continue; }
            35 => { bb = 36; continue; }
            36 => { rdx = ((((rcx as u8)) as i8 as i64) as u64) as u64; rax = ((rdx.wrapping_sub(0x61u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1a; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 39; continue; } bb = 37; continue; }
            37 => { fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(rdx).wrapping_sub(0x41u64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 66; continue; } bb = 38; continue; }
            38 => { crate::simd_ops::wu8(rsp.wrapping_add(rdx).wrapping_sub(0x41u64), (0x1u8) as u8); bb = 39; continue; }
            39 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x61u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 58; continue; } bb = 40; continue; }
            40 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x66u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 55; continue; } bb = 41; continue; }
            41 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x6cu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 54; continue; } bb = 42; continue; }
            42 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x6eu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 49; continue; } bb = 43; continue; }
            43 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x73u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 62; continue; } bb = 44; continue; }
            44 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 46; continue; } bb = 45; continue; }
            45 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(0x10u64), ((r12 as u32)) as u32); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0x18u64) as u64); fl_l = (((rdi as u32)).wrapping_add(1)) as u32 as u64; rdi = ((fl_l as u32)) as u32 as u64; bb = 59; continue; }
            46 => { r8 = (0xffffffffffffffffu64) as u64; bb = 47; continue; }
            47 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 47; continue; } bb = 48; continue; }
            48 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (((rdi as u32)).wrapping_add(1)) as u32 as u64; rdi = ((fl_l as u32)) as u32 as u64; bb = 59; continue; }
            49 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdx = P7; fl_l = rax; fl_r = rax; rdx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { rdx }) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 51; continue; } bb = 50; continue; }
            50 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(0x10u64), ((r12 as u32)) as u32); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0x18u64) as u64); fl_l = (((rdi as u32)).wrapping_add(1)) as u32 as u64; rdi = ((fl_l as u32)) as u32 as u64; bb = 59; continue; }
            51 => { r8 = (0xffffffffffffffffu64) as u64; bb = 52; continue; }
            52 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 52; continue; } bb = 53; continue; }
            53 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (((rdi as u32)).wrapping_add(1)) as u32 as u64; rdi = ((fl_l as u32)) as u32 as u64; bb = 59; continue; }
            54 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((crate::simd_ops::ru32(rsp.wrapping_add(0x64u64))) as i32) as f64).to_bits() as u128); crate::simd_ops::wu64(rax, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rax.wrapping_add(0x10u64), (0x3u32) as u32); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0x18u64) as u64); fl_l = (((rdi as u32)).wrapping_add(1)) as u32 as u64; rdi = ((fl_l as u32)) as u32 as u64; bb = 59; continue; }
            55 => { fl_l = rbp; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 57; continue; } bb = 56; continue; }
            56 => { rdx = ((rdi as u32)) as u32 as u64; rcx = rbx; rdx = (!((rdx as u32))) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (((rdi as u32)).wrapping_add(1)) as u32 as u64; rdi = ((fl_l as u32)) as u32 as u64; bb = 59; continue; }
            57 => { r8 = 0x1; rdx = rbx; rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (((rdi as u32)).wrapping_add(1)) as u32 as u64; rdi = ((fl_l as u32)) as u32 as u64; bb = 59; continue; }
            58 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x69u64))) as u32) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | (((((rax as u32)) as i32) as f64).to_bits() as u128); crate::simd_ops::wu64(rcx, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), (0x3u32) as u32); rax = ((r12 as u32)) as u32 as u64; crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0x18u64) as u64); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x6au64))) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = (rax & !0xffu64) | (((if (((fl_l as u8)) != ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), (0x1u32) as u32); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0x18u64) as u64); fl_l = (((rdi as u32)).wrapping_add(0x2u32)) as u32 as u64; fl_r = fl_l; rdi = ((fl_l as u32)) as u32 as u64; bb = 59; continue; }
            59 => { rcx = ((crate::simd_ops::ru8(r14.wrapping_add(0x1u64))) as u32) as u32 as u64; r14 = ((r14).wrapping_add(1)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 36; continue; } bb = 60; continue; }
            60 => { rax = ((rdi as u32)) as u32 as u64; bb = 61; continue; }
            61 => { r11 = rsp.wrapping_add(0x180); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            62 => { rdx = ((r15.wrapping_add(0x2u64)) as u32) as u32 as u64; rcx = rbx; r8 = P10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x14210f06b, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            63 => { r8 = 0x5; rdx = ((rsi as u32)) as u32 as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x14210f07c, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            64 => { rdx = ((r15.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = rbx; r8 = P13; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x14210f090, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            65 => { r8 = P14; rcx = rbx; rdx = ((r15.wrapping_add(0x1u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x14210f0a4, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            66 => { rdx = ((r15.wrapping_add(0x2u64)) as u32) as u32 as u64; rcx = rbx; r8 = P15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x14210f0b8, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210f0c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x1429868d0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    if (((fl_l as u64)) >= ((fl_r as u64))) { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdx, ((rax as u32)) as u32); rax = rcx; return rax; }
    r8 = P2;
    fl_l = rax; fl_r = r8;
    if (((fl_l as u64)) == ((fl_r as u64))) { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdx, ((rax as u32)) as u32); rax = rcx; return rax; }
    fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; fl_r = 0x9;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rax = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu32(rdx, (0x1u32) as u32); return rax; }
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rdx, ((rax as u32)) as u32);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210f380(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1429868d0; const P1: u64 = 0x142987350; const P2: u64 = 0x142117440; const P3: u64 = 0x142103d30; const P4: u64 = 0x1433dfc54; const P5: u64 = 0x1421168e0; const P6: u64 = 0x142987360; const P7: u64 = 0x1433e0e10; const P8: u64 = 0x142116b30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rbx = P0; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_r = rdx; r9 = rcx; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { crate::simd_ops::ru64(rcx.wrapping_add(0x10u64)) } else { rbx }) as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x9; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 2; continue; }
            2 => { fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { r8 = (crate::simd_ops::ru64(P1)) as u64; bb = 16; continue; }
            4 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x3u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 9; continue; } bb = 5; continue; }
            5 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x6u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 15; continue; } bb = 6; continue; }
            6 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x3u64))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x10u64), (0x1u32) as u32); if (((fl_l as u8)) < ((fl_r as u8))) { bb = 13; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu32(rdx, (0x0u32) as u32); rax = (0x2aaaaaaaaaaaaaabu64) as u64; crate::simd_ops::lk_add_u64(rcx.wrapping_add(0x8u64), (0x18u64) as u64); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rbx.wrapping_add(0x10u64)))) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { r8 = 0x1; rdx = r9; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = 0x2; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            13 => { crate::simd_ops::wu32(rdx, (0x1u32) as u32); crate::simd_ops::lk_add_u64(rcx.wrapping_add(0x8u64), (0x18u64) as u64); rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = 0x1; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            14 => { r8 = P4; rdx = 0x1; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14210f47c, rcx, rsp); return rax; }
            15 => { r8 = (crate::simd_ops::ru64(P6)) as u64; bb = 16; continue; }
            16 => { rdx = P7; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14210f490, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210f4a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x52; const P1: u64 = 0x50; const P2: u64 = 0x8; const P3: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    fl_l = (crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64;
    rax = 0x1;
    r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64;
    rdx = (rdx & !0xffu64) | (((if (((fl_l as u16)) <= ((fl_r as u16))) { 1u8 } else { 0u8 }) as u8 as u64));
    crate::simd_ops::wu32(r8, ((rdx as u32)) as u32);
    crate::simd_ops::wu32(r8.wrapping_add(P3), (0x1u32) as u32);
    fl_l = (crate::simd_ops::lk_add_u64(rcx.wrapping_add(P2), (0x18u64) as u64)) as u64; fl_r = fl_l;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210f4d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1; const P1: u64 = 0x18; const P2: u64 = 0x30; const P3: u64 = 0x68; const P4: u64 = 0x8; const P5: u64 = 0x10; const P6: u64 = 0xa70;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rcx.wrapping_add(P0), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P3), rax); crate::simd_ops::wu64(rdx.wrapping_add(P2), rcx); }
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64;
    crate::simd_ops::wu64(rax, rcx);
    crate::simd_ops::wu32(rax.wrapping_add(P5), (0x9u32) as u32);
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    rdx = ((rdx).wrapping_add(0x18u64)) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rdx);
    fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; fl_r = rcx;
    rax = 0x1;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { crate::simd_ops::wu32(rdx.wrapping_add(P5), (0x0u32) as u32); fl_l = (crate::simd_ops::lk_add_u64(rcx.wrapping_add(P4), (0x18u64) as u64)) as u64; fl_r = fl_l; }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210f530(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x52; const P3: u64 = 0x50; const P4: u64 = 0x3; const P5: u64 = 0x1433e09d0; const P6: u64 = 0x14210e640;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; rdx = r9; r8 = rcx; rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(rcx.wrapping_add(P1)))) as u64; let prod = (rax as i64 as i128).wrapping_mul((rdx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P2))) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru16(rcx.wrapping_add(P3))) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) > ((fl_r as u16))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; crate::simd_ops::wu8(r8.wrapping_add(P4), (0x1u8) as u8); rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = 0xffffffff; rcx = ((rcx).wrapping_shl(3u32)) as u64; r9 = ((r9).wrapping_sub(rcx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(P1), r9); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            2 => { rdx = P5; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14210f594, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210f5a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14210f5f0; const P1: u64 = 0x14210f780; const P2: u64 = 0x14210f840; const P3: u64 = 0x142118cd0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = P1;
    r9 = 0x1;
    r8 = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rdx = P2;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rax = 0x1;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210f5f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1429868d0; const P1: u64 = 0x14210c770; const P2: u64 = 0x142107140; const P3: u64 = 0x1421042d0; const P4: u64 = 0x1421158a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rbx = rcx; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 12; continue; } bb = 1; continue; }
            1 => { rsi = P0; fl_l = rax; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x7; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x40u64))) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x48u64))) as u64; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x1u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rbx); bb = 7; continue; }
            7 => { r8 = ((crate::simd_ops::ru8(rbx.wrapping_add(0x4u64))) as u32) as u32 as u64; rdx = 0x80; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rdi = rax; rdx = ((crate::simd_ops::ru8(rcx.wrapping_add(0x20u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rax, (0x9u8) as u8); rdx = (rdx & !0xffu64) | ((((((rdx as u8)) & (0x3u8))) as u8 as u64)); crate::simd_ops::wu8(rax.wrapping_add(0x1u64), ((rdx as u8)) as u8); rdx = rbx; rcx = ((crate::simd_ops::ru8(rbx.wrapping_add(0x4u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rax.wrapping_add(0x2u64), ((rcx as u8)) as u8); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rcx); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(0x3u64), ((rax as u32)) as u32); rcx = rdi; crate::simd_ops::wu64(rdi.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rdi.wrapping_add(0x48u64), rax); crate::simd_ops::wu64(rdi.wrapping_add(0x58u64), rax); crate::simd_ops::wu64(rdi.wrapping_add(0x60u64), rax); crate::simd_ops::wu64(rdi.wrapping_add(0x50u64), rax); crate::simd_ops::wu64(rdi.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rdi.wrapping_add(0x40u64), rax); crate::simd_ops::wu64(rdi.wrapping_add(0x70u64), rax); crate::simd_ops::wu64(rdi.wrapping_add(0x78u64), rax); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x4u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x4u64), ((rax as u8)) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x58u64), rax); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x6u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x6u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rax, rdi); crate::simd_ops::wu32(rax.wrapping_add(0x10u64), (0x9u32) as u32); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0x18u64) as u64); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_add(0x1060u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = rdi; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(r8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x1u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rdi.wrapping_add(0x1u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rdi); bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rsi }) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; xmm0 = (crate::simd_ops::ru128(rsi)) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); crate::simd_ops::lk_add_u64(rdi.wrapping_add(0x8u64), (0x18u64) as u64); rax = 0x1; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            12 => { rdx = 0x1; r8 = ((rdx.wrapping_add(0x6u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x14210f76f, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210f780(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142116d40; const P1: u64 = 0x10; const P2: u64 = 0x3; const P3: u64 = 0x18; const P4: u64 = 0x1080; const P5: u64 = 0x1420f6d60; const P6: u64 = 0x52; const P7: u64 = 0x50; const P8: u64 = 0x14210fbe0; const P9: u64 = 0x1421193a0; const P10: u64 = 0x1421159f0; const P11: u64 = 0x142118840; const P12: u64 = 0x142117120; const P13: u64 = 0x1421173b0; const P14: u64 = 0x1433e0998; const P15: u64 = 0x14210e640;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = 0xffffd8ed; rbx = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64; fl_r = 0x9; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = 0; fl_l = 0; fl_r = 0; bb = 3; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rax)) as u64; bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P2))) as u8 as u64; fl_r = (0x6u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { r8 = rdx; rdx = rcx; rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rax = ((crate::simd_ops::ru16(rbx.wrapping_add(P6))) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru16(rbx.wrapping_add(P7))) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) > ((fl_r as u16))) { bb = 13; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu8(rbx.wrapping_add(P2), (0x6u8) as u8); rax = 0xffffffff; bb = 8; continue; }
            8 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            9 => { rcx = rbx; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 8; continue; } bb = 10; continue; }
            10 => { rdx = 0xffffffff; rcx = rbx; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rdx = 0x1; rcx = rbx; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rdx = 0xfffffffe; rcx = rbx; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rcx = rbx; rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14210f824, rcx, rsp); return rax; }
            13 => { rdx = P14; rcx = rbx; rax = crate::simd_ops::call_stk(P15, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14210f834, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210f840(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142116d40; const P1: u64 = 0x10; const P2: u64 = 0x8; const P3: u64 = 0x14210fcf0; const P4: u64 = 0x18; const P5: u64 = 0x1080; const P6: u64 = 0x1420f6d60; const P7: u64 = 0x52; const P8: u64 = 0x50; const P9: u64 = 0x3; const P10: u64 = 0x1421193a0; const P11: u64 = 0x1421159f0; const P12: u64 = 0x142118840; const P13: u64 = 0x142117120; const P14: u64 = 0x1421173b0; const P15: u64 = 0x1433e0998; const P16: u64 = 0x14210e640;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = 0xffffd8ed; rbx = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64; fl_r = 0x9; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 3; continue; }
            2 => { rdi = (crate::simd_ops::ru64(rax)) as u64; bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rbx.wrapping_add(P1)))) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rcx = rbx; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; r8 = rdx; r8 = ((r8).wrapping_shr(63u32)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rdx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xfffffffe; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { r8 = rdi; rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rax = ((crate::simd_ops::ru16(rbx.wrapping_add(P7))) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru16(rbx.wrapping_add(P8))) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) > ((fl_r as u16))) { bb = 13; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu8(rbx.wrapping_add(P9), (0x6u8) as u8); rax = 0xffffffff; bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            9 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 8; continue; } bb = 10; continue; }
            10 => { rdx = 0xffffffff; rcx = rbx; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rdx = 0x1; rcx = rbx; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rdx = 0xfffffffe; rcx = rbx; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rcx = rbx; rax = crate::simd_ops::call_stk(P14, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14210f90f, rcx, rsp); return rax; }
            13 => { rdx = P15; rcx = rbx; rax = crate::simd_ops::call_stk(P16, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14210f91f, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210f930(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1421193a0; const P1: u64 = 0x1421159f0; const P2: u64 = 0x142118840; const P3: u64 = 0x142117120; const P4: u64 = 0x1421173b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((rdx as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rdx = 0xffffffff; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = 0x1; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = 0xfffffffe; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14210f97d, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210f990(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1429868d0; const P2: u64 = 0x8; const P3: u64 = 0x3; const P4: u64 = 0x18; const P5: u64 = 0x1080; const P6: u64 = 0x1420f6d60; const P7: u64 = 0x52; const P8: u64 = 0x50; const P9: u64 = 0x14210fbe0; const P10: u64 = 0x14210fb20; const P11: u64 = 0x1433dfc54; const P12: u64 = 0x1421168e0; const P13: u64 = 0x1433e0998; const P14: u64 = 0x14210e640;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = P1; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rbx = rcx; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = 0x9; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P3))) as u8 as u64; fl_r = (0x6u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { r8 = rdx; rdx = rcx; rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rax = ((crate::simd_ops::ru16(rbx.wrapping_add(P7))) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru16(rbx.wrapping_add(P8))) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) > ((fl_r as u16))) { bb = 9; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu8(rbx.wrapping_add(P3), (0x6u8) as u8); rax = 0xffffffff; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rdx = ((rax as u32)) as u32 as u64; rcx = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P10, rax, rcx, rdx, r8, r9, rsp); }
            8 => { r8 = P11; rdx = 0x1; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14210fa1c, rcx, rsp); return rax; }
            9 => { rdx = P13; rcx = rbx; rax = crate::simd_ops::call_stk(P14, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14210fa2c, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210fa40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1429868d0; const P1: u64 = 0x10; const P2: u64 = 0x8; const P3: u64 = 0x14210fcf0; const P4: u64 = 0x18; const P5: u64 = 0x1080; const P6: u64 = 0x1420f6d60; const P7: u64 = 0x52; const P8: u64 = 0x50; const P9: u64 = 0x3; const P10: u64 = 0x14210fb20; const P11: u64 = 0x1433dfc54; const P12: u64 = 0x1421168e0; const P13: u64 = 0x1433e0998; const P14: u64 = 0x14210e640;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rax = P0; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_l = rcx; fl_r = rdx; rax = (if (((fl_l as u64)) < ((fl_r as u64))) { rcx } else { rax }) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64; fl_r = 0x9; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 1; continue; }
            1 => { rdi = (crate::simd_ops::ru64(rax)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 2; continue; }
            2 => { rdx = ((rdx).wrapping_sub(rcx)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rdx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rcx = rbx; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; r8 = rdx; r8 = ((r8).wrapping_shr(63u32)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rdx = rdi; r8 = (((r8 as u32)).wrapping_sub(1)) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xfffffffe; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { r8 = rdi; rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rax = ((crate::simd_ops::ru16(rbx.wrapping_add(P7))) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru16(rbx.wrapping_add(P8))) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) > ((fl_r as u16))) { bb = 9; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu8(rbx.wrapping_add(P9), (0x6u8) as u8); rax = 0xffffffff; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { rdx = ((rax as u32)) as u32 as u64; rcx = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P10, rax, rcx, rdx, r8, r9, rsp); }
            8 => { r8 = P11; rdx = 0x1; rcx = rbx; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14210fb07, rcx, rsp); return rax; }
            9 => { rdx = P13; rcx = rbx; rax = crate::simd_ops::call_stk(P14, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14210fb17, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210fb20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x1; const P3: u64 = 0x18; const P4: u64 = 0x30; const P5: u64 = 0x68; const P6: u64 = 0xffffffffffffffd0; const P7: u64 = 0xffffffffffffffe8; const P8: u64 = 0x142118840;
    let (mut rax, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r11 = ((rdx as u32)) as u32 as u64; r8 = rcx; crate::simd_ops::wu32(rax.wrapping_add(P1), (0x1u32) as u32); fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rax, (0x0u32) as u32); crate::simd_ops::lk_add_u64(rcx.wrapping_add(P0), (0x18u64) as u64); rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P2))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(r8.wrapping_add(P2), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64; crate::simd_ops::wu64(r8.wrapping_add(P5), rax); crate::simd_ops::wu64(rcx.wrapping_add(P4), r8); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rdx = ((rax.wrapping_add(P6)) as u64) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rcx = ((rax.wrapping_add(P7)) as u64) as u64; bb = 5; continue; }
            5 => { xmm0 = (crate::simd_ops::ru128(rcx)) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64) as u128; crate::simd_ops::wu64(rax.wrapping_add(P1), ((xmm1 as u64)) as u64); rax = ((rax).wrapping_sub(0x18u64)) as u64; rcx = ((rcx.wrapping_add(P7)) as u64) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_add(P1), ((xmm1 as u64)) as u64); rax = 0x2; rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { crate::simd_ops::wu32(rax, (0x1u32) as u32); rdx = (!((rdx as u32))) as u32 as u64; crate::simd_ops::lk_add_u64(rcx.wrapping_add(P0), (0x18u64) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((r11.wrapping_add(0x1u64)) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210fbe0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x3; const P1: u64 = 0x1421173d0; const P2: u64 = 0x142117440; const P3: u64 = 0x8; const P4: u64 = 0x10; const P5: u64 = 0x1; const P6: u64 = 0x142117650; const P7: u64 = 0x1433e0e30; const P8: u64 = 0x142116b30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; rdi = rdx; rbx = rcx; if (((fl_l as u8)) < ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = 0x2; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r8 = 0x1; rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = 0xffffffff; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdx.wrapping_add(P4)))) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rcx = rbx; rsi = rdx; rsi = ((((rsi) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rsi; rax = ((rax).wrapping_shr(63u32)) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; rdx = ((rsi.wrapping_add(P5)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { r8 = ((rsi as u32)) as u32 as u64; rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = ((rsi as u32)) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rdx = P7; rcx = rbx; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14210fc8a, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210fca0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x1080; const P2: u64 = 0x1420f6d60; const P3: u64 = 0x52; const P4: u64 = 0x50; const P5: u64 = 0x3; const P6: u64 = 0x1433e0998; const P7: u64 = 0x14210e640;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r8 = rdx; rbx = rcx; rdx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rax = ((crate::simd_ops::ru16(rbx.wrapping_add(P3))) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru16(rbx.wrapping_add(P4))) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) > ((fl_r as u16))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu8(rbx.wrapping_add(P5), (0x6u8) as u8); rax = 0xffffffff; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rdx = P6; rcx = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14210fce9, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210fcf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142987350; const P1: u64 = 0x142987360; const P2: u64 = 0x142987370; const P3: u64 = 0x142987368; const P4: u64 = 0x1433e0e70; const P5: u64 = 0x1421177d0; const P6: u64 = 0x142117650; const P7: u64 = 0x142117440; const P8: u64 = 0x14210d130; const P9: u64 = 0x1433e0e50; const P10: u64 = 0x142116b30; const P11: u64 = 0x1433e0e30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x3u64))) as u32) as u32 as u64; rsi = ((r8 as u32)) as u32 as u64; rbx = rdx; rdi = rcx; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } bb = 1; continue; }
            1 => { fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(P0)) as u64; bb = 10; continue; }
            3 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x6u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru64(P1)) as u64; bb = 10; continue; }
            5 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { r8 = (crate::simd_ops::ru64(P2)) as u64; bb = 10; continue; }
            7 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x40u64))) as u64; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 9; continue; }
            9 => { r8 = (crate::simd_ops::ru64(P3)) as u64; bb = 10; continue; }
            10 => { rdx = P4; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = 0xffffffff; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            11 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 12; continue; }
            12 => { rdx = ((rsi as u32)) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 23; continue; } bb = 13; continue; }
            13 => { r8 = ((rsi as u32)) as u32 as u64; rdx = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x6u64))) as u32) as u32 as u64; r8 = ((rsi as u32)) as u32 as u64; rdx = rdi; crate::simd_ops::wu8(rbx.wrapping_add(0x6u64), ((rax as u8)) as u8); rcx = rbx; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 18; continue; } bb = 15; continue; }
            15 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x6; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = 0xfffffffe; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            17 => { r8 = 0x1; rdx = rdi; rcx = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rax = 0xffffffff; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            18 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rbx.wrapping_add(0x10u64)))) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rsi = rdx; rsi = ((((rsi) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rsi; rax = ((rax).wrapping_shr(63u32)) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 22; continue; } bb = 19; continue; }
            19 => { rdx = ((rsi.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x28; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 21; continue; } bb = 20; continue; }
            20 => { rcx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 24; continue; } bb = 21; continue; }
            21 => { r8 = ((rsi as u32)) as u32 as u64; rdx = rdi; rcx = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 22; continue; }
            22 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = ((rsi as u32)) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            23 => { rdx = P9; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14210fe5c, rcx, rsp); return rax; }
            24 => { rdx = P11; rcx = rdi; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14210fe6c, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210fe80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1429868d0; const P2: u64 = 0x10; const P3: u64 = 0x142987350; const P4: u64 = 0x3; const P5: u64 = 0x142987358; const P6: u64 = 0x142987360; const P7: u64 = 0x142987370; const P8: u64 = 0x40; const P9: u64 = 0x20; const P10: u64 = 0x142987368; const P11: u64 = 0x1; const P12: u64 = 0x142118a60; const P13: u64 = 0x1433dfc54; const P14: u64 = 0x1421168e0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r8 = P1; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; fl_r = r9; r8 = (if (((fl_l as u64)) < ((fl_r as u64))) { crate::simd_ops::ru64(rcx.wrapping_add(P2)) } else { r8 }) as u64; fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P2))) as u32 as u64; fl_r = 0x9; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 18; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru64(r8)) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 2; continue; }
            2 => { fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(P3)) as u64; bb = 13; continue; }
            4 => { rax = ((crate::simd_ops::ru8(r8.wrapping_add(P4))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(P5)) as u64; bb = 13; continue; }
            6 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x6u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(P6)) as u64; bb = 13; continue; }
            8 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(P7)) as u64; bb = 13; continue; }
            10 => { rax = (crate::simd_ops::ru64(r8.wrapping_add(P8))) as u64; fl_l = (crate::simd_ops::ru64(r8.wrapping_add(P9))) as u64; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(P6)) as u64; bb = 13; continue; }
            12 => { rax = (crate::simd_ops::ru64(r8.wrapping_add(P2))) as u64; fl_l = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; fl_r = rax; rdx = (crate::simd_ops::ru64(P5)) as u64; rdx = (if (((fl_l as u64)) == ((fl_r as u64))) { crate::simd_ops::ru64(P10) } else { rdx }) as u64; bb = 13; continue; }
            13 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu32(r9.wrapping_add(P2), ((rdx as u32)) as u32); rax = ((rdx.wrapping_add(P11)) as u32) as u32 as u64; crate::simd_ops::lk_add_u64(rcx.wrapping_add(0x8u64), (0x18u64) as u64); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            15 => { r8 = (0xffffffffffffffffu64) as u64; bb = 16; continue; }
            16 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 16; continue; } bb = 17; continue; }
            17 => { rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rax = 0x1; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            18 => { r8 = P13; rdx = 0x1; rax = crate::simd_ops::call_stk(P14, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14210ff5a, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14210ff70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1429872d0; const P1: u64 = 0x1433e1218; const P2: u64 = 0x1421153b0; const P3: u64 = 0x1433e0740; const P4: u64 = 0x142103330; const P5: u64 = 0x1420ec940; const P6: u64 = 0x1429868d0; const P7: u64 = 0x1420ec690; const P8: u64 = 0x143311fbc; const P9: u64 = 0x142118cd0; const P10: u64 = 0x14210f990; const P11: u64 = 0x1430ba62c; const P12: u64 = 0x14210fa40; const P13: u64 = 0x1433e11f8; const P14: u64 = 0x142116b30;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; r14 = P0; r13 = 0; fl_l = 0; fl_r = 0; rax = r14; rbp = ((r13 as u32)) as u32 as u64; rdi = rcx; bb = 1; continue; }
            1 => { rbp = (((rbp as u32)).wrapping_add(1)) as u32 as u64; rax = rax.wrapping_add(0x10); fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = r13; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { r9 = 0x1; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), r12); r8 = P1; rdx = 0xffffd8f0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x1u64))) as u32) as u32 as u64; r12 = rdi.wrapping_add(0x1); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rdi.wrapping_add(0x1u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rdi); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rsi = rdi.wrapping_add(0x8); r8 = 0x9; rdx = P3; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(rsi)) as u64; r8 = rsp.wrapping_add(0x30); rdx = rbx.wrapping_sub(0x18); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rcx = rdi; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x5u32) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = P6; rdx = rax.wrapping_add(0x18); crate::simd_ops::wu64(rsi, rdx); fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x6; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 6; continue; }
            6 => { r9 = ((rbp as u32)) as u32 as u64; crate::simd_ops::wu64(rsi, rax); r8 = P3; rdx = 0xffffd8ee; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 19; continue; } bb = 7; continue; }
            7 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x1u64))) as u32) as u32 as u64; rbp = rsi; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rbp = rdi.wrapping_add(0x8); fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(r12, ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rdi); bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = P3; r8 = 0x9; rcx = rdi; xmm0 = (crate::simd_ops::ru128(rax.wrapping_sub(0x18u64))) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_sub(0x8u64))) as u64) as u128; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rbx = (crate::simd_ops::ru64(rsi)) as u64; rbx = ((rbx).wrapping_add(0x18u64)) as u64; crate::simd_ops::wu64(rsi, rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(rsi)) as u64; rdx = rbx.wrapping_sub(0x48); r9 = ((r9).wrapping_sub(0x18u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); r8 = rsp.wrapping_add(0x30); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x5u32) as u32); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::lk_add_u64(rsi, (0xffffffffffffffe8u64) as u64)) as u64; fl_r = fl_l; rdx = (crate::simd_ops::ru64(rsi)) as u64; rsi = rbp; bb = 10; continue; }
            10 => { rax = rdx.wrapping_sub(0x18); fl_l = rax; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 14; continue; } bb = 11; continue; }
            11 => { rcx = rax.wrapping_sub(0x18); bb = 12; continue; }
            12 => { xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rdx = (crate::simd_ops::ru64(rsi)) as u64; rcx = rcx.wrapping_add(0x18); rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 12; continue; } bb = 13; continue; }
            13 => { rsi = rdi.wrapping_add(0x8); bb = 14; continue; }
            14 => { rax = rdx.wrapping_sub(0x18); crate::simd_ops::wu64(rsi, rax); rax = P8; bb = 15; continue; }
            15 => { rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; r9 = 0; fl_l = 0; fl_r = 0; r8 = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r13); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(r14)) as u64; r8 = (0xffffffffffffffffu64) as u64; rbx = (crate::simd_ops::ru64(rsi)) as u64; bb = 16; continue; }
            16 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = ((r13 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 16; continue; } bb = 17; continue; }
            17 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(rsi)) as u64; r8 = rsp.wrapping_add(0x30); r9 = ((r9).wrapping_sub(0x18u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rdx = rbx.wrapping_sub(0x30); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x5u32) as u32); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::lk_add_u64(rsi, (0xffffffffffffffe8u64) as u64); r14 = ((r14).wrapping_add(0x10u64)) as u64; rax = (crate::simd_ops::ru64(r14)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 15; continue; } bb = 18; continue; }
            18 => { rax = P10; r9 = 0; fl_l = 0; fl_r = 0; r8 = P11; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rdx = P12; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rdx = P11; r8 = 0x6; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r8 = rsp.wrapping_add(0x30); r9 = ((r9).wrapping_sub(0x18u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rdx = rbx.wrapping_sub(0x30); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x5u32) as u32); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_r = (0x18u64) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64); r11 = rsp.wrapping_add(0x50); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rax = 0x1; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            19 => { r8 = P3; rcx = rdi; rdx = P13; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x14211024f, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142110260(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x4; const P2: u64 = 0x1; const P3: u64 = 0x18; const P4: u64 = 0x21; const P5: u64 = 0x20; const P6: u64 = 0x14210c5f0;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x58u64))) as u32 as u64; fl_r = 0x2; r10 = rdx; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 12; continue; } bb = 1; continue; }
            1 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 12; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; fl_r = 0x6; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; fl_r = 0x6; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 4; continue; }
            4 => { r9 = (crate::simd_ops::ru64(r8)) as u64; fl_l = (crate::simd_ops::ru8(r9.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 12; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru64(r9.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 6; continue; }
            6 => { rdx = ((crate::simd_ops::ru8(r9.wrapping_add(P2))) as u32) as u32 as u64; r8 = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(r9.wrapping_add(P0), r8); fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(r8.wrapping_add(P2))) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P4))) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_sub(1)) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) <= ((fl_r as u8))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P5))) as u32) as u32 as u64; rdx = (rdx & !0xffu64) | ((((((rdx as u8)) & (0xf8u8))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x3u8))) as u8 as u64)); fl_l = ((((rax as u8)) | ((rdx as u8)))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(r9.wrapping_add(P2), ((rax as u8)) as u8); bb = 11; continue; }
            10 => { rdx = r8; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { crate::simd_ops::wu64(r10, r9); rax = 0x1; crate::simd_ops::wu32(r10.wrapping_add(P0), (0x6u32) as u32); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            12 => { rax = 0xffffffff; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142110300(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x18; const P3: u64 = 0xaa0; const P4: u64 = 0xbd8; const P5: u64 = 0x1420fcb60; const P6: u64 = 0x1429868d0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x58u64))) as u32 as u64; fl_r = 0x1; rbx = rdx; rdx = rcx; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 15; continue; } bb = 1; continue; }
            1 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 15; continue; } bb = 2; continue; }
            2 => { rax = (((crate::simd_ops::ru32(r8.wrapping_add(P0))) as i32 as i64) as u64) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x6; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(r8)) as u64; r11 = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; bb = 7; continue; }
            4 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x8; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(r8)) as u64; r11 = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; bb = 7; continue; }
            6 => { rcx = rax; rax = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; r11 = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P3))) as u64; bb = 7; continue; }
            7 => { fl_l = r11; fl_r = r11; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; rcx = r11; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            9 => { rax = P6; bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64) as u128; crate::simd_ops::wu64(rbx.wrapping_add(P0), ((xmm1 as u64)) as u64); rax = 0x1; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            12 => { rax = 0x1; fl_l = r11; fl_r = r11; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rbx, r11); crate::simd_ops::wu32(rbx.wrapping_add(P0), (0x6u32) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            14 => { crate::simd_ops::wu32(rbx.wrapping_add(P0), (0x0u32) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            15 => { rax = 0xffffffff; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421103d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; fl_r = 0x2;
    r10 = rdx;
    if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { rax = 0xffffffff; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x1;
    if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { rax = 0xffffffff; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; fl_r = 0x3;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; xmm0 = 0u128; r9 = (crate::simd_ops::cvt_f64_i32((f64::from_bits(crate::simd_ops::ru64(rax))))) as u32 as u64; rax = (crate::simd_ops::cvt_f64_i64((f64::from_bits(crate::simd_ops::ru64(r8))))) as u64; rcx = ((r9 as u32)) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(P0), (0x3u32) as u32); rcx = ((((rcx as u32)) & (0x1fu32))) as u32 as u64; r9 = (((((r9 as u32)) as i32).wrapping_shr(5u32) as u32)) as u32 as u64; rdx = (((rax as u32)).wrapping_shr(((((rcx as u32)) as u32) & 31))) as u32 as u64; rax = 0xfffffffe; rax = (((rax as u32)).wrapping_shl(((((r9 as u32)) as u32) & 31))) as u32 as u64; fl_l = ((!((rax as u32))) & ((rdx as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; rax = ((rax as u32)) as u32 as u64; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); crate::simd_ops::wu64(r10, ((xmm0 as u64)) as u64); rax = 0x1; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = 0xffffffff;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142110440(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1420fcf40; const P2: u64 = 0x14;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x58u64))) as u32 as u64; fl_r = 0x1; rbx = rdx; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 6; continue; } bb = 1; continue; }
            1 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 6; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x6; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(r8)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | (((((rax as u32)) as i32) as f64).to_bits() as u128); crate::simd_ops::wu64(rbx, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rbx.wrapping_add(P0), (0x3u32) as u32); rax = 0x1; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            4 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x5; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(r8)) as u64; xmm0 = 0u128; rcx = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; rax = 0x1; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rcx) as i64) as f64).to_bits() as u128); crate::simd_ops::wu64(rdx, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rdx.wrapping_add(P0), (0x3u32) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { rax = 0xffffffff; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421104c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x10; const P2: u64 = 0x8; const P3: u64 = 0x18; const P4: u64 = 0x85; const P5: u64 = 0xffffffffffffffff;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; fl_r = 0x1; r10 = rdx; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 1; continue; }
            1 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r11 = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rcx = r11; r9 = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; rcx = ((rcx).wrapping_sub(r9)) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(r9)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P4))) as u32) as u32 as u64; rdx = (((rdx as u32)).wrapping_sub((rax as u32))) as u32 as u64; rax = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64; r9 = ((rdx.wrapping_add(P5)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::cvt_f64_i32((f64::from_bits(crate::simd_ops::ru64(r8))))) as u32 as u64; rax = ((rcx.wrapping_add(P5)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 8; continue; } bb = 4; continue; }
            4 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; rcx = ((((rcx as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = rax; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rax = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; xmm0 = (crate::simd_ops::ru128(r11.wrapping_add(rax.wrapping_mul(8)))) as u128; crate::simd_ops::wu128(r10, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(r11.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(r10.wrapping_add(P1), ((xmm1 as u64)) as u64); rax = 0x1; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x5; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(r8)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P3))) as u8 as u64; fl_r = (0x23u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 7; continue; }
            7 => { xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | (((((r9 as u32)) as i32) as f64).to_bits() as u128); crate::simd_ops::wu64(r10, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(r10.wrapping_add(P1), (0x3u32) as u32); rax = 0x1; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            8 => { rax = 0xffffffff; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142110590(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; fl_r = 0x1;
    if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { rax = 0xffffffff; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x1;
    if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { rax = 0xffffffff; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; fl_r = 0x3;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rax = (crate::simd_ops::cvt_f64_i64((f64::from_bits(crate::simd_ops::ru64(r8))))) as u64; let bit_in: u32 = (rax as u32); if bit_in != 0 { rcx = ((bit_in.trailing_zeros() as u32)) as u32 as u64; } fl_l = bit_in as u64; rax = 0x20; crate::simd_ops::wu32(rdx.wrapping_add(P0), (0x3u32) as u32); xmm0 = 0u128; rax = (if (((fl_l as u32)) != 0) { (rcx as u32) } else { (rax as u32) }) as u32 as u64; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | (((((rax as u32)) as i32) as f64).to_bits() as u128); crate::simd_ops::wu64(rdx, ((xmm0 as u64)) as u64); rax = 0x1; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = 0xffffffff;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421105e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; fl_r = 0x1;
    if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { rax = 0xffffffff; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x1;
    if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { rax = 0xffffffff; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; fl_r = 0x3;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rax = (crate::simd_ops::cvt_f64_i64((f64::from_bits(crate::simd_ops::ru64(r8))))) as u64; let bit_in: u32 = (rax as u32); if bit_in != 0 { rcx = (((31u32.wrapping_sub(bit_in.leading_zeros())) as u32)) as u32 as u64; } fl_l = bit_in as u64; if !((((fl_l as u32)) == 0)) { rax = 0x1f; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; } else { rax = 0x20; } xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | (((((rax as u32)) as i32) as f64).to_bits() as u128); crate::simd_ops::wu64(rdx, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rdx.wrapping_add(P0), (0x3u32) as u32); rax = 0x1; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = 0xffffffff;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142110640(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x28; const P2: u64 = 0x18; const P3: u64 = 0x40; const P4: u64 = 0x30; const P5: u64 = 0x4; const P6: u64 = 0x8; const P7: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x3; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 9; continue; } bb = 1; continue; }
            1 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 9; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 5; continue; }
            5 => { xmm1 = ((crate::simd_ops::ru64(rax)) as u64) as u128; xmm3 = ((crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64) as u128; xmm2 = 0u128; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x4; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P3))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 7; continue; }
            7 => { xmm2 = ((crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64) as u128; bb = 8; continue; }
            8 => { xmm0 = ((crate::simd_ops::ru64(r8)) as u64) as u128; xmm0 = crate::simd_ops::vcvtpd2ps(xmm0); xmm1 = (xmm1 & !0xffff_ffffu128) | (((f64::from_bits(xmm1 as u64)) as f32).to_bits() as u128); crate::simd_ops::wu32(rdx, ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(P5), ((xmm1 as u32)) as u32); xmm0 = (xmm3 & !0xffff_ffffu128) | (((f64::from_bits(xmm3 as u64)) as f32).to_bits() as u128); xmm1 = (xmm2 & !0xffff_ffffu128) | (((f64::from_bits(xmm2 as u64)) as f32).to_bits() as u128); crate::simd_ops::wu32(rdx.wrapping_add(P6), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(P7), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(P0), (0x4u32) as u32); rax = 0x1; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            9 => { rax = 0xffffffff; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421106d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1420fcf40; const P1: u64 = 0x143426a48;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru32(rsp.wrapping_add(0x58u64))) as u32 as u64; rsi = rdx; rbp = rcx; fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 21; continue; } bb = 1; continue; }
            1 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 21; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(r8.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x6; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 21; continue; } bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(r8)) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = ((rax as u32)) as u32 as u64; bb = 11; continue; }
            5 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 21; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 21; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x28u64))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 21; continue; } bb = 8; continue; }
            8 => { xmm0 = ((crate::simd_ops::ru64(rax)) as u64) as u128; fl_l = ((xmm0 as u64)) as u64; fl_r = (crate::simd_ops::ru64(P1)) as u64; if (f64::from_bits(fl_l).is_nan() || f64::from_bits(fl_r).is_nan()) { bb = 21; continue; } bb = 9; continue; }
            9 => { if (f64::from_bits(fl_l) != f64::from_bits(fl_r) && !f64::from_bits(fl_l).is_nan() && !f64::from_bits(fl_r).is_nan()) { bb = 21; continue; } bb = 10; continue; }
            10 => { r8 = (crate::simd_ops::cvt_f64_i32((f64::from_bits(crate::simd_ops::ru64(rax.wrapping_add(0x18u64)))))) as u32 as u64; bb = 11; continue; }
            11 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 21; continue; } bb = 12; continue; }
            12 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 21; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; rcx = ((rcx).wrapping_sub(rsi)) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 21; continue; } bb = 14; continue; }
            14 => { rax = ((r8.wrapping_add(rbx)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1f40; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 21; continue; } bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; r9 = ((((r8 as u32)) as i32 as i64) as u64) as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 18; continue; } bb = 16; continue; }
            16 => { rax = rsi; fl_l = rcx; fl_r = rsi; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rdx = r9; bb = 17; continue; }
            17 => { xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rax))) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rcx.wrapping_add(rax).wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rax = rax.wrapping_add(0x18); fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 18; continue; }
            18 => { rax = ((r9.wrapping_add(r9.wrapping_mul(2))) as u64) as u64; rcx = ((rsi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x20u64))) as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 20; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rcx); bb = 20; continue; }
            20 => { rax = ((r8 as u32)) as u32 as u64; bb = 22; continue; }
            21 => { rax = 0xffffffff; bb = 22; continue; }
            22 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142110820(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x4; const P2: u64 = 0x1420fcf40; const P3: u64 = 0x1; const P4: u64 = 0x1420fb250; const P5: u64 = 0x18; const P6: u64 = 0x21; const P7: u64 = 0x14210c5f0; const P8: u64 = 0x30; const P9: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x58u64))) as u32 as u64; fl_r = 0x2; rdi = rcx; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 1; continue; }
            1 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) > 0) { bb = 11; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; fl_r = 0x6; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(r8)) as u64; fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 11; continue; } bb = 4; continue; }
            4 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbx; rcx = rdi; r8 = ((rax.wrapping_add(P3)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; xmm0 = (crate::simd_ops::ru128(rdx)) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64) as u128; crate::simd_ops::wu64(rax.wrapping_add(P0), ((xmm1 as u64)) as u64); fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = 0x5; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 10; continue; } bb = 5; continue; }
            5 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P3))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P3))) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P6))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 8; continue; }
            8 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            9 => { fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(P3), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(P8))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P9), rax); crate::simd_ops::wu64(rcx.wrapping_add(P8), rbx); bb = 10; continue; }
            10 => { rax = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0xffffffff; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421108f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1420fce50; const P1: u64 = 0x14210c5f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut fl_k: u32 = 0; let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x58u64))) as u32 as u64; fl_r = 0x3; rsi = rcx; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { fl_k = 0; bb = 20; continue; } bb = 1; continue; }
            1 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { fl_k = 0; bb = 20; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(r8.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x6; if (((fl_l as u32)) != ((fl_r as u32))) { fl_k = 0; bb = 20; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { fl_k = 4; bb = 20; continue; } bb = 4; continue; }
            4 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { xmm0 = ((crate::simd_ops::ru64(rbx)) as u64) as u128; fl_l = ((xmm0 as u64)) as u64; fl_r = ((xmm0 as u64)) as u64; fl_k = 3; bb = 11; continue; }
            6 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) != ((fl_r as u32))) { fl_k = 0; bb = 12; continue; } bb = 7; continue; }
            7 => { xmm0 = ((crate::simd_ops::ru32(rbx)) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { fl_k = 2; bb = 20; continue; } bb = 8; continue; }
            8 => { xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { fl_k = 2; bb = 20; continue; } bb = 9; continue; }
            9 => { xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { fl_k = 2; bb = 20; continue; } bb = 10; continue; }
            10 => { xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; fl_k = 2; bb = 11; continue; }
            11 => { if (match fl_k { 2 => (f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()), 3 => (f64::from_bits(fl_l).is_nan() || f64::from_bits(fl_r).is_nan()), _ => false }) { bb = 20; continue; } bb = 12; continue; }
            12 => { rdi = (crate::simd_ops::ru64(r8)) as u64; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x4u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 1; bb = 20; continue; } bb = 13; continue; }
            13 => { xmm0 = (crate::simd_ops::ru128(r8)) as u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rdx = rdi; r8 = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(0x18u64))) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64) as u128; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x28u64))) as u32 as u64; fl_r = 0x5; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { fl_k = 0; bb = 19; continue; } bb = 14; continue; }
            14 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x1u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { fl_k = 5; bb = 19; continue; } bb = 15; continue; }
            15 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x1u64))) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { fl_k = 5; bb = 19; continue; } bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x21u64))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 18; continue; } bb = 17; continue; }
            17 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0x1; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            18 => { fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rdi.wrapping_add(0x1u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x28u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rdi); fl_k = 5; bb = 19; continue; }
            19 => { rax = 0x1; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            20 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0xffffffff; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142110a40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1420fd0a0;
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
    fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; fl_r = 0x6;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rcx = (crate::simd_ops::ru64(r8)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64) as u128; crate::simd_ops::wu64(rbx.wrapping_add(P0), ((xmm1 as u64)) as u64); rax = 0x1; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = 0xffffffff;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142110aa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142105fa0;
    let (mut rax, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x58u64))) as u32 as u64; fl_r = 0x2;
    r11 = rdx;
    if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { rax = 0xffffffff; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x1;
    if !(((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32))) { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rcx = r8; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(r11, ((rax as u32)) as u32); rax = 0x1; crate::simd_ops::wu32(r11.wrapping_add(0x10u64), (0x1u32) as u32); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rax = 0xffffffff;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142110af0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1426466c8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x58u64))) as u32 as u64; fl_r = 0x1;
    rbx = rdx;
    if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { rax = 0xffffffff; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x1;
    if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { rax = 0xffffffff; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; fl_r = 0x3;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { xmm0 = ((crate::simd_ops::ru64(r8)) as u64) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(P0), (0x3u32) as u32); rax = 0x1; crate::simd_ops::wu64(rbx, ((xmm0 as u64)) as u64); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = 0xffffffff;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142110b40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x143426a48; const P2: u64 = 0x143427098;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x18u64)) as u64;
    fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; fl_r = 0x1;
    if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { rax = 0xffffffff; rsp = ((rsp).wrapping_add(0x18u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x1;
    if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { rax = 0xffffffff; rsp = ((rsp).wrapping_add(0x18u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; fl_r = 0x3;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { xmm0 = ((crate::simd_ops::ru64(r8)) as u64) as u128; xmm2 = 0u128; fl_l = ((xmm0 as u64)) as u64; fl_r = ((xmm2 as u64)) as u64; if !(!(f64::from_bits(fl_l) > f64::from_bits(fl_r))) { xmm1 = ((crate::simd_ops::ru64(P1)) as u64) as u128; } else { xmm1 = crate::simd_ops::vcmpsd(xmm0, (xmm0 as u64), (xmm2 as u64), 0x1u8); xmm0 = ((crate::simd_ops::ru64(P2)) as u64) as u128; xmm1 = crate::simd_ops::vblendvpd(xmm2, xmm0, xmm1); } crate::simd_ops::wu64(rdx, ((xmm1 as u64)) as u64); crate::simd_ops::wu32(rdx.wrapping_add(P0), (0x3u32) as u32); rax = 0x1; rsp = ((rsp).wrapping_add(0x18u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = 0xffffffff;
    rsp = ((rsp).wrapping_add(0x18u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142110bb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x28; const P2: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; fl_r = 0x3;
    if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { rax = 0xffffffff; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x1;
    if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { rax = 0xffffffff; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; fl_r = 0x3;
    if (((fl_l as u32)) != ((fl_r as u32))) { rax = 0xffffffff; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; fl_r = 0x3;
    if (((fl_l as u32)) != ((fl_r as u32))) { rax = 0xffffffff; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64; fl_r = 0x3;
    if (((fl_l as u32)) != ((fl_r as u32))) { rax = 0xffffffff; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    xmm0 = ((crate::simd_ops::ru64(rax)) as u64) as u128;
    xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64) as u128;
    fl_l = ((xmm1 as u64)) as u64; fl_r = ((xmm0 as u64)) as u64;
    if !(!(f64::from_bits(fl_l) >= f64::from_bits(fl_r))) { xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((if f64::from_bits(xmm0 as u64) > f64::from_bits(crate::simd_ops::ru64(r8)) { f64::from_bits(xmm0 as u64) } else { f64::from_bits(crate::simd_ops::ru64(r8)) }).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((if f64::from_bits(xmm1 as u64) < f64::from_bits(xmm0 as u64) { f64::from_bits(xmm1 as u64) } else { f64::from_bits(xmm0 as u64) }).to_bits() as u128); crate::simd_ops::wu64(rdx, ((xmm1 as u64)) as u64); crate::simd_ops::wu32(rdx.wrapping_add(P0), (0x3u32) as u32); rax = 0x1; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = 0xffffffff;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142110c10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x28; const P2: u64 = 0x18; const P3: u64 = 0xffffffffffffffff; const P4: u64 = 0x14; const P5: u64 = 0x142103330;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut xmm0: u128 = 0; let mut xmm1: u128 = 0; let mut xmm2: u128 = 0; let mut xmm3: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x58u64))) as u32 as u64; fl_r = 0x3; rbx = rdx; r10 = rcx; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 9; continue; } bb = 1; continue; }
            1 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 9; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; fl_r = 0x5; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::cvt_f64_i32((f64::from_bits(crate::simd_ops::ru64(rax))))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 9; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::cvt_f64_i32((f64::from_bits(crate::simd_ops::ru64(rax.wrapping_add(P2)))))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 9; continue; } bb = 7; continue; }
            7 => { r9 = (crate::simd_ops::ru64(r8)) as u64; rax = ((rcx.wrapping_add(P3)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r9.wrapping_add(P4))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = (((rcx as u32)).wrapping_sub((rdx as u32))) as u32 as u64; rax = ((rdx.wrapping_add(P3)) as u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; rdx = ((((rax as u32)) as i32 as i64) as u64) as u64; r9 = ((r9).wrapping_add(0x18u64)) as u64; r8 = ((((rcx as u32)) as i32 as i64) as u64) as u64; rcx = r10; rdx = ((rdx).wrapping_add(r9)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx, rax); rax = 0x1; crate::simd_ops::wu32(rbx.wrapping_add(P0), (0x5u32) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            9 => { rax = 0xffffffff; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142110cb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1420f83f0; const P1: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x58u64))) as u32 as u64; fl_r = 0x1;
    rbx = rdx;
    if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { rax = 0xffffffff; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x1;
    if !(((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32))) { rdx = r8; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); rax = 0x1; crate::simd_ops::wu32(rbx.wrapping_add(P1), (0x5u32) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = 0xffffffff;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142110d00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x14;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; fl_r = 0x1;
    if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { rax = 0xffffffff; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x1;
    if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { rax = 0xffffffff; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; fl_r = 0x5;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rax = (crate::simd_ops::ru64(r8)) as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((crate::simd_ops::ru32(rax.wrapping_add(P1))) as i32) as f64).to_bits() as u128); crate::simd_ops::wu64(rdx, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rdx.wrapping_add(P0), (0x3u32) as u32); rax = 0x1; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = 0xffffffff;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142110d40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x142103330;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm0: u128 = 0; let mut xmm1: u128 = 0; let mut xmm2: u128 = 0; let mut xmm3: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r10 = (((crate::simd_ops::ru32(rsp.wrapping_add(0x68u64))) as i32 as i64) as u64) as u64; rbx = rdx; r11 = rcx; fl_l = ((r10 as u32)) as u32 as u64; fl_r = 0x8; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 12; continue; } bb = 1; continue; }
            1 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 12; continue; } bb = 2; continue; }
            2 => { fl_l = ((r10 as u32)) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::cvt_f64_i32((f64::from_bits(crate::simd_ops::ru64(r8))))) as u32 as u64; rcx = (((rax as u8)) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((rcx as u8)) as u8); bb = 6; continue; }
            6 => { r8 = r10; fl_l = r10; fl_r = (0x2u64) as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 11; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdx = 0x2; bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::cvt_f64_i32((f64::from_bits(crate::simd_ops::ru64(rcx))))) as u32 as u64; r9 = (((rax as u8)) as u32) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu8(rsp.wrapping_add(rdx).wrapping_add(0x1fu64), ((r9 as u8)) as u8); rcx = ((rcx).wrapping_add(0x18u64)) as u64; rdx = ((rdx).wrapping_add(1)) as u64; fl_l = rdx; fl_r = r8; if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { bb = 8; continue; } bb = 11; continue; }
            11 => { rdx = rsp.wrapping_add(0x20); crate::simd_ops::wu8(rsp.wrapping_add(r10).wrapping_add(0x20u64), (0x0u8) as u8); rcx = r11; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx, rax); rax = 0x1; crate::simd_ops::wu32(rbx.wrapping_add(P0), (0x5u32) as u32); rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            12 => { rax = 0xffffffff; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142110df0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); r10 = ((r9 as u32)) as u32 as u64; rsi = rdx; r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x38u64))) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x2; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 22; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(r8.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x5; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 22; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 22; continue; } bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(r8)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::cvt_f64_i32((f64::from_bits(crate::simd_ops::ru64(rcx))))) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x3; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 7; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x28u64))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { r9 = (crate::simd_ops::cvt_f64_i32((f64::from_bits(crate::simd_ops::ru64(rcx.wrapping_add(0x18u64)))))) as u32 as u64; bb = 8; continue; }
            6 => { r9 = ((rdx as u32)) as u32 as u64; bb = 8; continue; }
            7 => { r9 = ((rax as u32)) as u32 as u64; bb = 8; continue; }
            8 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 22; continue; } bb = 9; continue; }
            9 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 22; continue; } bb = 10; continue; }
            10 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 22; continue; } bb = 11; continue; }
            11 => { r9 = (((r9 as u32)).wrapping_sub((rax as u32))) as u32 as u64; rcx = 0x1; r9 = (((r9 as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; r10 = (if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { (rcx as u32) } else { (r10 as u32) }) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 22; continue; } bb = 12; continue; }
            12 => { r10 = ((((r9 as u32)) as i32 as i64) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); fl_l = r10; fl_r = (0x4u64) as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 17; continue; } bb = 13; continue; }
            13 => { rbx = ((((rax as u32)) as i32 as i64) as u64) as u64; r8 = rdi.wrapping_add(0x19); r11 = r10.wrapping_sub(0x4); r8 = ((r8).wrapping_add(rbx)) as u64; r11 = ((r11).wrapping_shr(2u32)) as u64; rcx = rsi.wrapping_add(0x40); fl_l = ((r11).wrapping_add(1)) as u64; r11 = ((fl_l as u64)) as u64; rdx = ((r11.wrapping_mul(4)) as u64) as u64; bb = 14; continue; }
            14 => { rax = ((crate::simd_ops::ru8(r8.wrapping_sub(0x2u64))) as u32) as u32 as u64; rcx = rcx.wrapping_add(0x60); r8 = r8.wrapping_add(0x4); xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | (((((rax as u32)) as i32) as f64).to_bits() as u128); crate::simd_ops::wu64(rcx.wrapping_sub(0xa0u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rcx.wrapping_sub(0x90u64), (0x3u32) as u32); rax = ((crate::simd_ops::ru8(r8.wrapping_sub(0x5u64))) as u32) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | (((((rax as u32)) as i32) as f64).to_bits() as u128); crate::simd_ops::wu64(rcx.wrapping_sub(0x88u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rcx.wrapping_sub(0x78u64), (0x3u32) as u32); rax = ((crate::simd_ops::ru8(r8.wrapping_sub(0x4u64))) as u32) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | (((((rax as u32)) as i32) as f64).to_bits() as u128); crate::simd_ops::wu64(rcx.wrapping_sub(0x70u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rcx.wrapping_sub(0x60u64), (0x3u32) as u32); rax = ((crate::simd_ops::ru8(r8.wrapping_sub(0x3u64))) as u32) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | (((((rax as u32)) as i32) as f64).to_bits() as u128); crate::simd_ops::wu64(rcx.wrapping_sub(0x58u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rcx.wrapping_sub(0x48u64), (0x3u32) as u32); fl_l = r11; fl_r = (0x1u64) as u64; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 15; continue; }
            15 => { fl_l = rdx; fl_r = r10; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 19; continue; } bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; rax = ((r9 as u32)) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            17 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 21; continue; } bb = 18; continue; }
            18 => { rbx = ((((rax as u32)) as i32 as i64) as u64) as u64; bb = 19; continue; }
            19 => { rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; r8 = ((rsi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = ((rbx.wrapping_add(rdi)) as u64) as u64; bb = 20; continue; }
            20 => { rcx = ((crate::simd_ops::ru8(rdx.wrapping_add(rax).wrapping_add(0x17u64))) as u32) as u32 as u64; rdx = ((rdx).wrapping_add(1)) as u64; crate::simd_ops::wu32(r8.wrapping_add(0x10u64), (0x3u32) as u32); r8 = r8.wrapping_add(0x18); xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | (((((rcx as u32)) as i32) as f64).to_bits() as u128); crate::simd_ops::wu64(r8.wrapping_sub(0x18u64), ((xmm0 as u64)) as u64); fl_l = rdx; fl_r = r10; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 20; continue; } bb = 21; continue; }
            21 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; rax = ((r9 as u32)) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            22 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; rax = 0xffffffff; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142110fa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x10; const P2: u64 = 0xaf0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; fl_r = 0x1;
    rax = rcx;
    if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { rax = 0xffffffff; return rax; }
    fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x1;
    if !(((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32))) { rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; rcx = (((crate::simd_ops::ru32(r8.wrapping_add(P1))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P2))) as u64; rax = 0x1; crate::simd_ops::wu64(rdx, rcx); crate::simd_ops::wu32(rdx.wrapping_add(P1), (0x5u32) as u32); return rax; }
    rax = 0xffffffff;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142110fe0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; fl_r = 0x2;
    if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { rax = 0xffffffff; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x1;
    if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { rax = 0xffffffff; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; fl_r = 0x3;
    if (((fl_l as u32)) != ((fl_r as u32))) { rax = 0xffffffff; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; fl_r = 0x3;
    if (((fl_l as u32)) != ((fl_r as u32))) { rax = 0xffffffff; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rcx = (crate::simd_ops::cvt_f64_i32((f64::from_bits(crate::simd_ops::ru64(rax))))) as u32 as u64;
    fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x20;
    if !((((fl_l as u32)) >= ((fl_r as u32)))) { rax = (crate::simd_ops::cvt_f64_i64((f64::from_bits(crate::simd_ops::ru64(r8))))) as u64; rax = (((rax as u32)).wrapping_shr(((((rcx as u32)) as u32) & 31))) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); crate::simd_ops::wu64(rdx, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rdx.wrapping_add(P0), (0x3u32) as u32); rax = 0x1; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = 0xffffffff;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142111040(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; fl_r = 0x2;
    if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { rax = 0xffffffff; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x1;
    if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { rax = 0xffffffff; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; fl_r = 0x3;
    if (((fl_l as u32)) != ((fl_r as u32))) { rax = 0xffffffff; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; fl_r = 0x3;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rcx = (crate::simd_ops::cvt_f64_i32((f64::from_bits(crate::simd_ops::ru64(rax))))) as u32 as u64; rax = (crate::simd_ops::cvt_f64_i64((f64::from_bits(crate::simd_ops::ru64(r8))))) as u64; rax = (((rax as u32)).rotate_right(((rcx as u32) & 31))) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); crate::simd_ops::wu64(rdx, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rdx.wrapping_add(P0), (0x3u32) as u32); rax = 0x1; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = 0xffffffff;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142111090(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x28; const P2: u64 = 0x18; const P3: u64 = 0x40; const P4: u64 = 0x30; const P5: u64 = 0xffffffffffffffff;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; r11 = rdx; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x3; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 13; continue; } bb = 1; continue; }
            1 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 13; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 5; continue; }
            5 => { r9 = (crate::simd_ops::cvt_f64_i64((f64::from_bits(crate::simd_ops::ru64(r8))))) as u64; r8 = (crate::simd_ops::cvt_f64_i32((f64::from_bits(crate::simd_ops::ru64(rax.wrapping_add(P2)))))) as u32 as u64; r10 = (crate::simd_ops::cvt_f64_i64((f64::from_bits(crate::simd_ops::ru64(rax))))) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x20; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 13; continue; } bb = 7; continue; }
            7 => { r10 = ((((r10 as u32)) & (0x1u32))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rcx as u32)) as u32); rax = (((r10 as u32)).wrapping_shl(((((r8 as u32)) as u32) & 31))) as u32 as u64; fl_l = ((((r9 as u32)) >> ((((r8 as u32)) as u32 & 31))) & 1) as u64; r9 = (((r9 as u32)) & !((1u32) << ((((r8 as u32)) as u32 & 31)))) as u32 as u64; fl_l = ((((rax as u32)) | ((r9 as u32)))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); crate::simd_ops::wu64(rdx, ((xmm0 as u64)) as u64); rax = 0x1; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            8 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P3))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 9; continue; }
            9 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 13; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::cvt_f64_i32((f64::from_bits(crate::simd_ops::ru64(rax.wrapping_add(P4)))))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 13; continue; } bb = 11; continue; }
            11 => { rax = ((rcx.wrapping_add(r8)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x20; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = ((rcx.wrapping_add(P5)) as u32) as u32 as u64; crate::simd_ops::wu32(r11.wrapping_add(0x10u64), (0x3u32) as u32); rcx = 0xfffffffe; rdx = (((rcx as u32)).wrapping_shl(((((rax as u32)) as u32) & 31))) as u32 as u64; rdx = (!((rdx as u32))) as u32 as u64; rax = (((rdx as u32)).wrapping_shl(((((r8 as u32)) as u32) & 31))) as u32 as u64; fl_l = ((!((rax as u32))) & ((r9 as u32))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; rdx = ((((rdx as u32)) & ((r10 as u32)))) as u32 as u64; rax = (((rdx as u32)).wrapping_shl(((((r8 as u32)) as u32) & 31))) as u32 as u64; fl_l = ((((rax as u32)) | ((rcx as u32)))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); crate::simd_ops::wu64(r11, ((xmm0 as u64)) as u64); rax = 0x1; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            13 => { rax = 0xffffffff; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142111180(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; fl_r = 0x2;
    if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { rax = 0xffffffff; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x1;
    if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { rax = 0xffffffff; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; fl_r = 0x3;
    if (((fl_l as u32)) != ((fl_r as u32))) { rax = 0xffffffff; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; fl_r = 0x3;
    if (((fl_l as u32)) != ((fl_r as u32))) { rax = 0xffffffff; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rcx = (crate::simd_ops::cvt_f64_i32((f64::from_bits(crate::simd_ops::ru64(rax))))) as u32 as u64;
    fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x20;
    if !((((fl_l as u32)) >= ((fl_r as u32)))) { rax = (crate::simd_ops::cvt_f64_i64((f64::from_bits(crate::simd_ops::ru64(r8))))) as u64; rax = (((rax as u32)).wrapping_shl(((((rcx as u32)) as u32) & 31))) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); crate::simd_ops::wu64(rdx, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rdx.wrapping_add(P0), (0x3u32) as u32); rax = 0x1; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = 0xffffffff;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421111e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; fl_r = 0x2;
    if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { rax = 0xffffffff; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x1;
    if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { rax = 0xffffffff; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; fl_r = 0x3;
    if (((fl_l as u32)) != ((fl_r as u32))) { rax = 0xffffffff; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; fl_r = 0x3;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rcx = (crate::simd_ops::cvt_f64_i32((f64::from_bits(crate::simd_ops::ru64(rax))))) as u32 as u64; rax = (crate::simd_ops::cvt_f64_i64((f64::from_bits(crate::simd_ops::ru64(r8))))) as u64; rax = (((rax as u32)).rotate_left(((rcx as u32) & 31))) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); crate::simd_ops::wu64(rdx, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rdx.wrapping_add(P0), (0x3u32) as u32); rax = 0x1; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = 0xffffffff;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142111230(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x28; const P2: u64 = 0x18; const P3: u64 = 0xffffffffffffffff;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; r10 = rdx; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 13; continue; } bb = 1; continue; }
            1 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 13; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 4; continue; }
            4 => { r9 = (crate::simd_ops::cvt_f64_i64((f64::from_bits(crate::simd_ops::ru64(r8))))) as u64; rdx = (crate::simd_ops::cvt_f64_i32((f64::from_bits(crate::simd_ops::ru64(rax))))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 5; continue; }
            5 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x20; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 13; continue; } bb = 6; continue; }
            6 => { rax = (((r9 as u32)).wrapping_shr(((((rdx as u32)) as u32) & 31))) as u32 as u64; fl_l = ((((rax as u32)) & (0x1u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; bb = 7; continue; }
            7 => { xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); crate::simd_ops::wu64(r10, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(r10.wrapping_add(P0), (0x3u32) as u32); rax = 0x1; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            8 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 9; continue; }
            9 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 13; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::cvt_f64_i32((f64::from_bits(crate::simd_ops::ru64(rax.wrapping_add(P2)))))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 13; continue; } bb = 11; continue; }
            11 => { rax = ((rcx.wrapping_add(rdx)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x20; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = ((rcx.wrapping_add(P3)) as u32) as u32 as u64; rcx = 0xfffffffe; rcx = (((rcx as u32)).wrapping_shl(((((rax as u32)) as u32) & 31))) as u32 as u64; rax = (((r9 as u32)).wrapping_shr(((((rdx as u32)) as u32) & 31))) as u32 as u64; fl_l = ((!((rcx as u32))) & ((rax as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; rax = ((rax as u32)) as u32 as u64; bb = 7; continue; }
            13 => { rax = 0xffffffff; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421112e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r10 = rdx; rdx = (((crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as i32 as i64) as u64) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x2; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 9; continue; } bb = 1; continue; }
            1 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 9; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::cvt_f64_i64((f64::from_bits(crate::simd_ops::ru64(r8))))) as u64; rax = (crate::simd_ops::cvt_f64_i64((f64::from_bits(crate::simd_ops::ru64(rcx))))) as u64; r8 = ((((r8 as u32)) & ((rax as u32)))) as u32 as u64; r9 = rdx; fl_l = rdx; fl_r = (0x3u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rdx = 0x3; fl_l = ((rcx).wrapping_add(0x18u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::cvt_f64_i64((f64::from_bits(crate::simd_ops::ru64(rcx))))) as u64; r8 = ((((r8 as u32)) & ((rax as u32)))) as u32 as u64; rdx = ((rdx).wrapping_add(1)) as u64; rcx = ((rcx).wrapping_add(0x18u64)) as u64; fl_l = rdx; fl_r = r9; if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { bb = 6; continue; } bb = 8; continue; }
            8 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(r10.wrapping_add(P0), (0x1u32) as u32); fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu32(r10, ((rax as u32)) as u32); rax = 0x1; return rax; }
            9 => { rax = 0xffffffff; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142111370(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r10 = (((crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as i32 as i64) as u64) as u64; r11 = rdx; fl_l = ((r10 as u32)) as u32 as u64; fl_r = 0x2; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 9; continue; } bb = 1; continue; }
            1 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 9; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::cvt_f64_i64((f64::from_bits(crate::simd_ops::ru64(r8))))) as u64; rax = (crate::simd_ops::cvt_f64_i64((f64::from_bits(crate::simd_ops::ru64(rcx))))) as u64; r8 = ((((r8 as u32)) ^ ((rax as u32)))) as u32 as u64; fl_l = r10; fl_r = (0x3u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rdx = 0x3; fl_l = ((rcx).wrapping_add(0x18u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::cvt_f64_i64((f64::from_bits(crate::simd_ops::ru64(rcx))))) as u64; r8 = ((((r8 as u32)) ^ ((rax as u32)))) as u32 as u64; rdx = ((rdx).wrapping_add(1)) as u64; rcx = ((rcx).wrapping_add(0x18u64)) as u64; fl_l = rdx; fl_r = r10; if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { bb = 6; continue; } bb = 8; continue; }
            8 => { rax = ((r8 as u32)) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); crate::simd_ops::wu64(r11, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(r11.wrapping_add(0x10u64), (0x3u32) as u32); rax = 0x1; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            9 => { rax = 0xffffffff; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142111400(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r10 = (((crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as i32 as i64) as u64) as u64; r11 = rdx; fl_l = ((r10 as u32)) as u32 as u64; fl_r = 0x2; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 9; continue; } bb = 1; continue; }
            1 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 9; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::cvt_f64_i64((f64::from_bits(crate::simd_ops::ru64(r8))))) as u64; rax = (crate::simd_ops::cvt_f64_i64((f64::from_bits(crate::simd_ops::ru64(rcx))))) as u64; r8 = ((((r8 as u32)) | ((rax as u32)))) as u32 as u64; fl_l = r10; fl_r = (0x3u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rdx = 0x3; fl_l = ((rcx).wrapping_add(0x18u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::cvt_f64_i64((f64::from_bits(crate::simd_ops::ru64(rcx))))) as u64; r8 = ((((r8 as u32)) | ((rax as u32)))) as u32 as u64; rdx = ((rdx).wrapping_add(1)) as u64; rcx = ((rcx).wrapping_add(0x18u64)) as u64; fl_l = rdx; fl_r = r10; if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { bb = 6; continue; } bb = 8; continue; }
            8 => { rax = ((r8 as u32)) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); crate::simd_ops::wu64(r11, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(r11.wrapping_add(0x10u64), (0x3u32) as u32); rax = 0x1; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            9 => { rax = 0xffffffff; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142111490(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; fl_r = 0x1;
    if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { rax = 0xffffffff; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x1;
    if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { rax = 0xffffffff; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; fl_r = 0x3;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rax = (crate::simd_ops::cvt_f64_i64((f64::from_bits(crate::simd_ops::ru64(r8))))) as u64; rax = (!((rax as u32))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(P0), (0x3u32) as u32); xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); crate::simd_ops::wu64(rdx, ((xmm0 as u64)) as u64); rax = 0x1; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = 0xffffffff;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421114e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r10 = (((crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as i32 as i64) as u64) as u64; r11 = rdx; fl_l = ((r10 as u32)) as u32 as u64; fl_r = 0x2; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 9; continue; } bb = 1; continue; }
            1 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 9; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::cvt_f64_i64((f64::from_bits(crate::simd_ops::ru64(r8))))) as u64; rax = (crate::simd_ops::cvt_f64_i64((f64::from_bits(crate::simd_ops::ru64(rcx))))) as u64; r8 = ((((r8 as u32)) & ((rax as u32)))) as u32 as u64; fl_l = r10; fl_r = (0x3u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rdx = 0x3; fl_l = ((rcx).wrapping_add(0x18u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::cvt_f64_i64((f64::from_bits(crate::simd_ops::ru64(rcx))))) as u64; r8 = ((((r8 as u32)) & ((rax as u32)))) as u32 as u64; rdx = ((rdx).wrapping_add(1)) as u64; rcx = ((rcx).wrapping_add(0x18u64)) as u64; fl_l = rdx; fl_r = r10; if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { bb = 6; continue; } bb = 8; continue; }
            8 => { rax = ((r8 as u32)) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); crate::simd_ops::wu64(r11, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(r11.wrapping_add(0x10u64), (0x3u32) as u32); rax = 0x1; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            9 => { rax = 0xffffffff; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142111570(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; fl_r = 0x2;
    if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { rax = 0xffffffff; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x1;
    if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { rax = 0xffffffff; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; fl_r = 0x3;
    if (((fl_l as u32)) != ((fl_r as u32))) { rax = 0xffffffff; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; fl_r = 0x3;
    if (((fl_l as u32)) != ((fl_r as u32))) { rax = 0xffffffff; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rcx = (crate::simd_ops::cvt_f64_i32((f64::from_bits(crate::simd_ops::ru64(rax))))) as u32 as u64;
    fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x20;
    if !((((fl_l as u32)) >= ((fl_r as u32)))) { rax = (crate::simd_ops::cvt_f64_i64((f64::from_bits(crate::simd_ops::ru64(r8))))) as u64; rax = (((((rax as u32)) as i32).wrapping_shr(((((rcx as u32)) as u32) & 31)) as u32)) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); crate::simd_ops::wu64(rdx, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rdx.wrapping_add(P0), (0x3u32) as u32); rax = 0x1; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = 0xffffffff;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}
