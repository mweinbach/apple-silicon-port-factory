#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn fn_141616110(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x58; const P1: u64 = 0x10; const P2: u64 = 0x20; const P3: u64 = 0x141616940; const P4: u64 = 0x18; const P5: u64 = 0x141616050;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rdi = rcx; rcx = rdx; rbx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((rdi.wrapping_add(P0)) as u64) as u64; rdx = rsp.wrapping_add(0x30); xmm0 = (crate::simd_ops::ru128(rax)) as u128; xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(P1))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(P2))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm1) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rax)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = ((rcx).wrapping_add(0x48u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 4; continue; }
            3 => { rcx = ((rdi.wrapping_add(P4)) as u64) as u64; bb = 4; continue; }
            4 => { rdx = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x0u8) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141616190(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xffffffffffffffff;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdi); r9 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; r10 = 0; fl_l = 0; fl_r = 0; r11 = (((rdx as u8)) as u32) as u32 as u64; rbx = rcx; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdi = (crate::simd_ops::ru64(rcx)) as u64; bb = 2; continue; }
            2 => { rax = ((r9 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; rax = (((rax as u32)).wrapping_add((r10 as u32))) as u32 as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rdx = ((rdx.wrapping_mul(8)) as u64) as u64; rcx = ((crate::simd_ops::ru8(rdx.wrapping_add(rdi))) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((r11 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((r11 as u8)) as u8 as u64; rax = (if (((fl_l as u8)) >= ((fl_r as u8))) { (r10 as u32) } else { (rax as u32) }) as u32 as u64; r10 = ((rax as u32)) as u32 as u64; rax = ((r9.wrapping_add(P1)) as u32) as u32 as u64; rax = (if (((fl_l as u8)) >= ((fl_r as u8))) { (r9 as u32) } else { (rax as u32) }) as u32 as u64; r9 = ((rax as u32)) as u32 as u64; fl_l = (((r9 as u32)).wrapping_shr(1u32)) as u32 as u64; r9 = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = ((((r10 as u32)) as i32 as i64) as u64) as u64; crate::simd_ops::wu8(r8, (0x0u8) as u8); rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; return rax; }
            5 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu8(r8, (0x1u8) as u8); rax = (crate::simd_ops::ru64(rbx)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; fl_l = ((rax).wrapping_add(rdx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141616220(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xffffffffffffffff;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdi); r9 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; r10 = 0; fl_l = 0; fl_r = 0; r11 = ((rdx as u32)) as u32 as u64; rbx = rcx; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdi = (crate::simd_ops::ru64(rcx)) as u64; bb = 2; continue; }
            2 => { rax = ((r9 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; rax = (((rax as u32)).wrapping_add((r10 as u32))) as u32 as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rdx = ((rdx.wrapping_mul(8)) as u64) as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(rdi))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; rax = (if (((fl_l as u32)) >= ((fl_r as u32))) { (r10 as u32) } else { (rax as u32) }) as u32 as u64; r10 = ((rax as u32)) as u32 as u64; rax = ((r9.wrapping_add(P1)) as u32) as u32 as u64; rax = (if (((fl_l as u32)) >= ((fl_r as u32))) { (r9 as u32) } else { (rax as u32) }) as u32 as u64; r9 = ((rax as u32)) as u32 as u64; fl_l = (((r9 as u32)).wrapping_shr(1u32)) as u32 as u64; r9 = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = ((((r10 as u32)) as i32 as i64) as u64) as u64; crate::simd_ops::wu8(r8, (0x0u8) as u8); rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; return rax; }
            5 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu8(r8, (0x1u8) as u8); rax = (crate::simd_ops::ru64(rbx)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; fl_l = ((rax).wrapping_add(rdx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416162b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xffffffffffffffff;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdi); r9 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; r10 = 0; fl_l = 0; fl_r = 0; r11 = rdx; rbx = rcx; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdi = (crate::simd_ops::ru64(rcx)) as u64; bb = 2; continue; }
            2 => { rax = ((r9 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; rax = (((rax as u32)).wrapping_add((r10 as u32))) as u32 as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rdx = ((rdx.wrapping_mul(8)) as u64) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(rdi))) as u64; fl_l = rcx; fl_r = r11; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = rcx; fl_r = r11; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { (r10 as u32) } else { (rax as u32) }) as u32 as u64; r10 = ((rax as u32)) as u32 as u64; rax = ((r9.wrapping_add(P1)) as u32) as u32 as u64; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { (r9 as u32) } else { (rax as u32) }) as u32 as u64; r9 = ((rax as u32)) as u32 as u64; fl_l = (((r9 as u32)).wrapping_shr(1u32)) as u32 as u64; r9 = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = ((((r10 as u32)) as i32 as i64) as u64) as u64; crate::simd_ops::wu8(r8, (0x0u8) as u8); rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; return rax; }
            5 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu8(r8, (0x1u8) as u8); rax = (crate::simd_ops::ru64(rbx)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; fl_l = ((rax).wrapping_add(rdx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141616340(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524330; const P1: u64 = 0x141615c60; const P2: u64 = 0x14268f730; const P3: u64 = 0x14268f6f0; const P4: u64 = 0x14268f6b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 2; continue; }
            2 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rdx = 0x98; rcx = 0x2; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 5; continue; }
            5 => { r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rcx = rax; rdx = ((r9.wrapping_add(0x2u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = P2; crate::simd_ops::wu64(rbx, rax); rax = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { rdx = 0x98; rcx = 0x2; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { r9 = (r9 & !0xffu64) | (((0x1u8) as u8 as u64)); rdx = 0x1; r8 = (((r9 as u8)) as u32) as u32 as u64; rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = P3; crate::simd_ops::wu64(rbx, rax); rax = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            8 => { rdx = 0x98; rcx = 0x2; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { r9 = 0; fl_l = 0; fl_r = 0; r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rdx = 0; fl_l = 0; fl_r = 0; rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = P4; crate::simd_ops::wu64(rbx, rax); rax = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            10 => { rbx = 0; fl_l = 0; fl_r = 0; rax = ((rbx as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141616430(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xffffffffffffffff;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdi); r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; r11 = 0; fl_l = 0; fl_r = 0; r10 = (((rdx as u8)) as u32) as u32 as u64; rdi = rcx; r9 = ((r11 as u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rcx)) as u64; bb = 2; continue; }
            2 => { rax = ((r8 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; rax = (((rax as u32)).wrapping_add((r9 as u32))) as u32 as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rcx = ((rbx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; rdx = ((crate::simd_ops::ru8(rbx.wrapping_add(rdx.wrapping_mul(8)))) as u32) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((r10 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((r10 as u8)) as u8 as u64; rax = (if (((fl_l as u8)) >= ((fl_r as u8))) { (r9 as u32) } else { (rax as u32) }) as u32 as u64; r9 = ((rax as u32)) as u32 as u64; rax = ((r8.wrapping_add(P1)) as u32) as u32 as u64; rax = (if (((fl_l as u8)) >= ((fl_r as u8))) { (r8 as u32) } else { (rax as u32) }) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; fl_l = (((r8 as u32)).wrapping_shr(1u32)) as u32 as u64; r8 = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rcx = ((((r9 as u32)) as i32 as i64) as u64) as u64; rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rcx = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; r11 = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { rcx } else { r11 }) as u64; rax = r11; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; return rax; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; r11 = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { rcx } else { r11 }) as u64; rax = r11; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416164e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xffffffffffffffff;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdi); r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; r11 = 0; fl_l = 0; fl_r = 0; r10 = ((rdx as u32)) as u32 as u64; rdi = rcx; r9 = ((r11 as u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rcx)) as u64; bb = 2; continue; }
            2 => { rax = ((r8 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; rax = (((rax as u32)).wrapping_add((r9 as u32))) as u32 as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rcx = ((rbx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(rdx.wrapping_mul(8)))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; rax = (if (((fl_l as u32)) >= ((fl_r as u32))) { (r9 as u32) } else { (rax as u32) }) as u32 as u64; r9 = ((rax as u32)) as u32 as u64; rax = ((r8.wrapping_add(P1)) as u32) as u32 as u64; rax = (if (((fl_l as u32)) >= ((fl_r as u32))) { (r8 as u32) } else { (rax as u32) }) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; fl_l = (((r8 as u32)).wrapping_shr(1u32)) as u32 as u64; r8 = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rcx = ((((r9 as u32)) as i32 as i64) as u64) as u64; rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rcx = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; r11 = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { rcx } else { r11 }) as u64; rax = r11; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; return rax; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; r11 = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { rcx } else { r11 }) as u64; rax = r11; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141616590(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xffffffffffffffff;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdi); r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; r11 = 0; fl_l = 0; fl_r = 0; r10 = rdx; rdi = rcx; r9 = ((r11 as u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rcx)) as u64; bb = 2; continue; }
            2 => { rax = ((r8 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; rax = (((rax as u32)).wrapping_add((r9 as u32))) as u32 as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rcx = ((rbx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdx; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = rdx; fl_r = r10; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { (r9 as u32) } else { (rax as u32) }) as u32 as u64; r9 = ((rax as u32)) as u32 as u64; rax = ((r8.wrapping_add(P1)) as u32) as u32 as u64; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { (r8 as u32) } else { (rax as u32) }) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; fl_l = (((r8 as u32)).wrapping_shr(1u32)) as u32 as u64; r8 = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rcx = ((((r9 as u32)) as i32 as i64) as u64) as u64; rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rcx = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; r11 = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { rcx } else { r11 }) as u64; rax = r11; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; return rax; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; r11 = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { rcx } else { r11 }) as u64; rax = r11; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141616640(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141616430; const P1: u64 = 0x1415fa0e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; r15 = 0; fl_l = 0; fl_r = 0; r14 = rdx; r13 = rcx; fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x8u64))) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; fl_r = r15; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 3; continue; }
            3 => { rdx = (((rdi as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(0x18u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 20; continue; } bb = 5; continue; }
            5 => { rdx = (rdx & !0xffu64) | (((0xffu8) as u8 as u64)); crate::simd_ops::wu8(r14, (0xffu8) as u8); rcx = r13.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 7; continue; }
            7 => { r15 = ((crate::simd_ops::ru8(r14.wrapping_add(0x8u64))) as u32) as u32 as u64; fl_l = ((r15 as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 9; continue; }
            9 => { rdx = (((r15 as u8)) as u32) as u32 as u64; rcx = rax.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { rax = rbp; fl_l = ((r15 as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 18; continue; } bb = 11; continue; }
            11 => { rdx = (rdx & !0xffu64) | (((0xffu8) as u8 as u64)); crate::simd_ops::wu8(r14.wrapping_add(0x8u64), (0xffu8) as u8); rcx = rbp.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; bb = 12; continue; }
            12 => { rax = rbp; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 13; continue; }
            13 => { rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 14; continue; }
            14 => { rcx = rsi.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = rax.wrapping_add(0x8); bb = 17; continue; }
            16 => { rax = 0; fl_l = 0; fl_r = 0; rcx = rsi.wrapping_add(0x8); crate::simd_ops::wu64(r14.wrapping_add(0x10u64), rax); fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x10u64))) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rcx = (if (((fl_l as u8)) == ((fl_r as u8))) { rax } else { rcx }) as u64; bb = 17; continue; }
            17 => { rax = rbp; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { r15 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0xffu8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r15); rcx = rax.wrapping_add(0x8); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(r14.wrapping_add(0x8u64), (xmm0) as u128); fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x10u64))) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; rcx = (if (((fl_l as u8)) == ((fl_r as u8))) { r15 } else { rcx }) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { rax = rcx; bb = 21; continue; }
            20 => { crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0xffu8) as u8); rax = r13.wrapping_add(0x8); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0xffu8) as u8); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r15); xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64) as u128; crate::simd_ops::wu128(r14, (xmm0) as u128); crate::simd_ops::wu64(r14.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); fl_l = (crate::simd_ops::ru8(r13.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rax = (if (((fl_l as u8)) == ((fl_r as u8))) { r15 } else { rax }) as u64; bb = 21; continue; }
            21 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416167c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1416164e0; const P1: u64 = 0x141616640;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rsi = (crate::simd_ops::ru32(rdx)) as u32 as u64; r12 = 0; fl_l = 0; fl_r = 0; rdi = rdx; r13 = rcx; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x18u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; fl_r = r12; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 5; continue; }
            5 => { rdx = ((rsi as u32)) as u32 as u64; rcx = ((rcx).wrapping_add(0x18u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 18; continue; } bb = 7; continue; }
            7 => { rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi, ((r12 as u32)) as u32); rcx = r13.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 12; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x18u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_r = r12; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 12; continue; }
            12 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rcx = rax.wrapping_add(0x18); rbp = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 15; continue; } bb = 13; continue; }
            13 => { rax = rbp; fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 14; continue; }
            14 => { rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((r12 as u32)) as u32); rcx = rbp.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; rax = rbp; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rdx = rdi.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; rax = rbp; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r12 as u32)) as u32); rcx = rax.wrapping_add(0x8); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0xffu8) as u8); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0xffu8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r12); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rdi.wrapping_add(0x8u64), (xmm0) as u128); crate::simd_ops::wu128(rdi.wrapping_add(0x18u64), (xmm1) as u128); fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x10u64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; rcx = (if (((fl_l as u8)) == ((fl_r as u8))) { r12 } else { rcx }) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { rax = rcx; bb = 19; continue; }
            18 => { crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r12 as u32)) as u32); rax = r13.wrapping_add(0x8); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r12 as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0xffu8) as u8); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0xffu8) as u8); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r12); crate::simd_ops::wu128(rdi, (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64) as u128; crate::simd_ops::wu128(rdi.wrapping_add(0x10u64), (xmm1) as u128); crate::simd_ops::wu64(rdi.wrapping_add(0x20u64), ((xmm0 as u64)) as u64); fl_l = (crate::simd_ops::ru8(r13.wrapping_add(0x10u64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; rax = (if (((fl_l as u8)) == ((fl_r as u8))) { r12 } else { rax }) as u64; bb = 19; continue; }
            19 => { r11 = rsp.wrapping_add(0x50); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141616940(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x28; const P5: u64 = 0x141616590; const P6: u64 = 0x1416167c0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; r15 = 0; fl_l = 0; fl_r = 0; rdi = rdx; r14 = rcx; fl_l = (crate::simd_ops::ru64(rdx)) as u64; fl_r = r15; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P2))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P3))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64; fl_r = r15; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; rcx = ((rcx).wrapping_add(0x18u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru64(rdi)) as u64; fl_r = r15; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi, r15); rcx = ((r14.wrapping_add(P2)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = ((rdi.wrapping_add(P0)) as u64) as u64; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); rax = ((r14.wrapping_add(P0)) as u64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r15 as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r15 as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0xffu8) as u8); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu8(rsp.wrapping_add(0x40u64), (0xffu8) as u8); crate::simd_ops::wu128(rdi, (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r15); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(rdi.wrapping_add(P1), (xmm1) as u128); crate::simd_ops::wu128(rdi.wrapping_add(P3), (xmm0) as u128); fl_l = (crate::simd_ops::ru8(r14.wrapping_add(P1))) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; rax = (if (((fl_l as u8)) == ((fl_r as u8))) { r15 } else { rax }) as u64; bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141616a20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141614590;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = rdx;
    rcx = ((rcx).wrapping_add(0x58u64)) as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141616a30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143815b94; const P1: u64 = 0x143815b98; const P2: u64 = 0x143815b9c; const P3: u64 = 0x143815ba0; const P4: u64 = 0x143815ba4; const P5: u64 = 0x143815ba8; const P6: u64 = 0x143930fa0; const P7: u64 = 0x1415c3840; const P8: u64 = 0x143426fd0; const P9: u64 = 0x14268a040; const P10: u64 = 0x1438152ac; const P11: u64 = 0x142677278; const P12: u64 = 0x1434269f8; const P13: u64 = 0x1439310e8; const P14: u64 = 0x143426ea4; const P15: u64 = 0x1434264e4; const P16: u64 = 0x1434268d0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 37>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), (r8) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rax.wrapping_sub(0x5fu64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0xe8u64)) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(0x80u64))) as u64; rsi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; xmm0 = ((crate::simd_ops::ru32(P0)) as u32) as u128; r14 = (r8) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x58u64), (xmm6) as u128); r13 = (rdx) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x68u64), (xmm7) as u128); r15 = (rcx) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x78u64), (xmm8) as u128); r11 = (0xfffffffcu32) as u32 as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x88u64), (xmm9) as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x67u64), ((xmm0 as u32)) as u32); fl_l = (r9) as u64 as u64; fl_r = (r9) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdx = ((crate::simd_ops::ru8(r9)) as u32) as u32 as u64; r8 = ((rsi as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rcx = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(r9))) as u8 as u64; fl_r = (0x9u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 38; continue; } bb = 3; continue; }
            3 => { r8 = ((rcx as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 4; continue; }
            4 => { xmm0 = ((crate::simd_ops::ru32(P1)) as u32) as u128; rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x67u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x6fu64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x67u64), ((rax as u32)) as u32); fl_l = (r9) as u64 as u64; fl_r = (r9) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { r8 = ((crate::simd_ops::ru8(r9)) as u32) as u32 as u64; rcx = ((rsi as u32)) as u32 as u64; bb = 6; continue; }
            6 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r9))) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 39; continue; } bb = 7; continue; }
            7 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 6; continue; } bb = 8; continue; }
            8 => { rdx = (rsi) as u64; bb = 9; continue; }
            9 => { fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; rax = ((rbp.wrapping_add(0x6fu64)) as u64) as u64; r10 = ((r13.wrapping_add(0x20u64)) as u64) as u64; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rdx } else { rax }) as u64; rax = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x6fu64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x6fu64))) as u32) as u128; crate::simd_ops::wu32(r10, ((xmm0 as u32)) as u32); r8 = (crate::simd_ops::ru64(r15.wrapping_add(0x80u64))) as u64; xmm1 = ((crate::simd_ops::ru32(P2)) as u32) as u128; crate::simd_ops::wu32(rbp.wrapping_add(0x6fu64), ((xmm1 as u32)) as u32); fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 10; continue; }
            10 => { r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((rsi as u32)) as u32 as u64; bb = 11; continue; }
            11 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0xbu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 40; continue; } bb = 12; continue; }
            12 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 11; continue; } bb = 13; continue; }
            13 => { xmm0 = ((crate::simd_ops::ru32(P3)) as u32) as u128; rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x6fu64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x7fu64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x6fu64), ((rax as u32)) as u32); fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 14; continue; }
            14 => { r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((rsi as u32)) as u32 as u64; bb = 15; continue; }
            15 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0xcu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 41; continue; } bb = 16; continue; }
            16 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 15; continue; } bb = 17; continue; }
            17 => { rdx = (rsi) as u64; bb = 18; continue; }
            18 => { fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; rax = ((rbp.wrapping_add(0x7fu64)) as u64) as u64; r12 = ((r13.wrapping_add(0x24u64)) as u64) as u64; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rdx } else { rax }) as u64; rax = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x7fu64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x7fu64))) as u32) as u128; crate::simd_ops::wu32(r12, ((xmm0 as u32)) as u32); r8 = (crate::simd_ops::ru64(r15.wrapping_add(0x80u64))) as u64; xmm1 = ((crate::simd_ops::ru32(P4)) as u32) as u128; crate::simd_ops::wu32(rbp.wrapping_add(0x7fu64), ((xmm1 as u32)) as u32); fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 19; continue; }
            19 => { r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((rsi as u32)) as u32 as u64; bb = 20; continue; }
            20 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0xdu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 42; continue; } bb = 21; continue; }
            21 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 20; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x7fu64))) as u32 as u64; xmm0 = ((crate::simd_ops::ru32(P5)) as u32) as u128; crate::simd_ops::wu32(rbp.wrapping_add(0x7fu64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 23; continue; }
            23 => { r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((rsi as u32)) as u32 as u64; bb = 24; continue; }
            24 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0xeu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 43; continue; } bb = 25; continue; }
            25 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 24; continue; } bb = 26; continue; }
            26 => { rdx = (rsi) as u64; bb = 27; continue; }
            27 => { rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x48u64))) as u64; rax = ((rbp.wrapping_add(0x7fu64)) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x8u32) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x79u64), (0x9u32) as u32); rbx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rdx } else { rax }) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x71u64), (r10) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x69u64), (0xau32) as u32); rdi = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x59u64), (0xbu32) as u32); fl_l = ((r14).wrapping_add(0x8u64)) as u64 as u64; fl_r = fl_l; r14 = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x51u64), (r12) as u64); rax = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x7fu64), ((rax as u32)) as u32); rax = ((rbp.wrapping_add(0x67u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rax) as u64); rax = ((rbp.wrapping_add(0x6fu64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x61u64), (rax) as u64); rax = ((rsp.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), (rax) as u64); rax = ((rbp.wrapping_add(0x7fu64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x31u64), (rax) as u64); rax = (crate::simd_ops::ru32(r15.wrapping_add(0x50u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x49u64), (0xcu32) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x39u64), (0xdu32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rcx) as u64); rdx = ((rax.wrapping_mul(8)) as u64) as u64; bb = 28; continue; }
            28 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; r8 = ((rdx.wrapping_add(rcx)) as u64) as u64; r9 = (crate::simd_ops::ru32(rbx)) as u32 as u64; rdx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; rcx = (crate::simd_ops::ru64(P6)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r14) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru32(r15.wrapping_add(0x50u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x48u64))) as u64; rdx = ((rax.wrapping_mul(8)) as u64) as u64; rax = ((rcx.wrapping_add(rdx)) as u64) as u64; fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 30; continue; } bb = 29; continue; }
            29 => { rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = (0x6u32) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 28; continue; } bb = 30; continue; }
            30 => { r8 = (crate::simd_ops::ru64(r15.wrapping_add(0x88u64))) as u64; r10 = (0x5851f42d4c957f2du64) as u64; xmm5 = ((crate::simd_ops::ru32(P8)) as u32) as u128; xmm2 = 0u128; xmm9 = ((crate::simd_ops::ru64(P9)) as u64) as u128; fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 34; continue; } bb = 31; continue; }
            31 => { r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((rsi as u32)) as u32 as u64; bb = 32; continue; }
            32 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0x9u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 44; continue; } bb = 33; continue; }
            33 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 32; continue; } bb = 34; continue; }
            34 => { r11 = (0xfffffffcu32) as u32 as u64; bb = 35; continue; }
            35 => { xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x67u64))) as u32) as u128; bb = 36; continue; }
            36 => { rax = (crate::simd_ops::ru32(P10)) as u32 as u64; xmm1 = 0u128; xmm7 = ((crate::simd_ops::ru64(P11)) as u64) as u128; xmm8 = 0u128; xmm6 = ((crate::simd_ops::ru64(P12)) as u64) as u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | (((f32::from_bits(xmm0 as u32)) as f64).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm0 as u64)).to_bits() as u128); fl_l = ((xmm1 as u64)) as u64; fl_r = ((xmm8 as u64)) as u64; if !(f64::from_bits(fl_l) > f64::from_bits(fl_r)) { bb = 51; continue; } bb = 37; continue; }
            37 => { xmm0 = (xmm6) as u128; bb = 52; continue; }
            38 => { xmm0 = ((crate::simd_ops::ru32(P1)) as u32) as u128; rax = ((rdx.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = (((rax) & (r11))) as u64; rcx = ((r8 as u32)) as u32 as u64; rax = ((rax).wrapping_add(r9)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; rax = ((rbp.wrapping_add(0x67u64)) as u64) as u64; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rdx } else { rax }) as u64; rax = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x6fu64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x67u64), ((rax as u32)) as u32); bb = 5; continue; }
            39 => { rax = ((r8.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = (((rax) & (r11))) as u64; fl_l = ((rax).wrapping_add(r9)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; bb = 9; continue; }
            40 => { xmm0 = ((crate::simd_ops::ru32(P3)) as u32) as u128; rax = ((r9.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = (((rax) & (r11))) as u64; rax = ((rax).wrapping_add(r8)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; rax = ((rbp.wrapping_add(0x6fu64)) as u64) as u64; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rdx } else { rax }) as u64; rax = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x7fu64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x6fu64), ((rax as u32)) as u32); bb = 14; continue; }
            41 => { rax = ((r9.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = (((rax) & (r11))) as u64; fl_l = ((rax).wrapping_add(r8)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; bb = 18; continue; }
            42 => { xmm0 = ((crate::simd_ops::ru32(P5)) as u32) as u128; rax = ((r9.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = (((rax) & (r11))) as u64; rax = ((rax).wrapping_add(r8)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; rax = ((rbp.wrapping_add(0x7fu64)) as u64) as u64; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rdx } else { rax }) as u64; rax = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x7fu64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); bb = 23; continue; }
            43 => { rax = ((r9.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = (((rax) & (r11))) as u64; fl_l = ((rax).wrapping_add(r8)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; bb = 27; continue; }
            44 => { rax = ((r9.wrapping_add(0x4u64)) as u32) as u32 as u64; r11 = (0xfffffffcu32) as u32 as u64; rax = (((rax) & (r11))) as u64; rax = ((rax).wrapping_add(r8)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 35; continue; } bb = 45; continue; }
            45 => { xmm3 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32) as u128; xmm0 = (xmm2) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(core::ptr::read_unaligned((rdx) as *const u32))).to_bits() as u128); fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 47; continue; } bb = 46; continue; }
            46 => { rax = (crate::simd_ops::ru64(P13)) as u64; rax = (((rax) as i64).wrapping_mul((r10) as i64) as u64) as u64; xmm0 = crate::simd_ops::vcvtps2pd(xmm3); rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(P13, (rax) as u64); fl_l = ((rax).wrapping_shr(33u32)) as u64 as u64; rax = ((fl_l as u64)) as u64; xmm1 = (((rax as u32)) as u32) as u128; xmm1 = crate::simd_ops::vcvtdq2pd(xmm1); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm9 as u64)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm0 as u64)).to_bits() as u128); xmm0 = crate::simd_ops::vcvtpd2ps(xmm1); bb = 47; continue; }
            47 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(core::ptr::read_unaligned((rdx) as *const u32))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(core::ptr::read_unaligned((rbp.wrapping_add(0x67u64)) as *const u32))).to_bits() as u128); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x67u64), ((xmm0 as u32)) as u32); if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 49; continue; } bb = 48; continue; }
            48 => { crate::simd_ops::wu32(rbp.wrapping_add(0x67u64), ((xmm2 as u32)) as u32); xmm0 = (xmm2) as u128; bb = 36; continue; }
            49 => { fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm5 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 36; continue; } bb = 50; continue; }
            50 => { crate::simd_ops::wu32(rbp.wrapping_add(0x67u64), ((xmm5 as u32)) as u32); xmm0 = (xmm5) as u128; bb = 36; continue; }
            51 => { xmm0 = (xmm7) as u128; bb = 52; continue; }
            52 => { xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) + f64::from_bits(xmm1 as u64)).to_bits() as u128); rax = (crate::simd_ops::cvt_f64_i32((f64::from_bits(xmm0 as u64)))) as u32 as u64; crate::simd_ops::wu32(r13.wrapping_add(0x1cu64), ((rax as u32)) as u32); r8 = (crate::simd_ops::ru64(r15.wrapping_add(0x88u64))) as u64; fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 56; continue; } bb = 53; continue; }
            53 => { r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((rsi as u32)) as u32 as u64; bb = 54; continue; }
            54 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0xbu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 59; continue; } bb = 55; continue; }
            55 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 54; continue; } bb = 56; continue; }
            56 => { xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x6fu64))) as u32) as u128; bb = 57; continue; }
            57 => { rax = (crate::simd_ops::ru32(P10)) as u32 as u64; xmm1 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | (((f32::from_bits(xmm0 as u32)) as f64).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm0 as u64)).to_bits() as u128); fl_l = ((xmm1 as u64)) as u64; fl_r = ((xmm8 as u64)) as u64; if !(f64::from_bits(fl_l) > f64::from_bits(fl_r)) { bb = 66; continue; } bb = 58; continue; }
            58 => { xmm0 = (xmm6) as u128; bb = 67; continue; }
            59 => { rax = ((r9.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = (((rax) & (r11))) as u64; rax = ((rax).wrapping_add(r8)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 56; continue; } bb = 60; continue; }
            60 => { xmm3 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32) as u128; xmm0 = (xmm2) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(core::ptr::read_unaligned((rdx) as *const u32))).to_bits() as u128); fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 62; continue; } bb = 61; continue; }
            61 => { rax = (crate::simd_ops::ru64(P13)) as u64; rax = (((rax) as i64).wrapping_mul((r10) as i64) as u64) as u64; xmm0 = crate::simd_ops::vcvtps2pd(xmm3); rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(P13, (rax) as u64); fl_l = ((rax).wrapping_shr(33u32)) as u64 as u64; rax = ((fl_l as u64)) as u64; xmm1 = (((rax as u32)) as u32) as u128; xmm1 = crate::simd_ops::vcvtdq2pd(xmm1); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm9 as u64)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm0 as u64)).to_bits() as u128); xmm0 = crate::simd_ops::vcvtpd2ps(xmm1); bb = 62; continue; }
            62 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(core::ptr::read_unaligned((rdx) as *const u32))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(core::ptr::read_unaligned((rbp.wrapping_add(0x6fu64)) as *const u32))).to_bits() as u128); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x6fu64), ((xmm0 as u32)) as u32); if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 64; continue; } bb = 63; continue; }
            63 => { crate::simd_ops::wu32(rbp.wrapping_add(0x6fu64), ((xmm2 as u32)) as u32); xmm0 = (xmm2) as u128; bb = 57; continue; }
            64 => { fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm5 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 57; continue; } bb = 65; continue; }
            65 => { crate::simd_ops::wu32(rbp.wrapping_add(0x6fu64), ((xmm5 as u32)) as u32); xmm0 = (xmm5) as u128; bb = 57; continue; }
            66 => { xmm0 = (xmm7) as u128; bb = 67; continue; }
            67 => { xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) + f64::from_bits(xmm0 as u64)).to_bits() as u128); rax = (crate::simd_ops::cvt_f64_i32((f64::from_bits(xmm1 as u64)))) as u32 as u64; crate::simd_ops::wu32(r13.wrapping_add(0x28u64), ((rax as u32)) as u32); r8 = (crate::simd_ops::ru64(r15.wrapping_add(0x88u64))) as u64; fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 71; continue; } bb = 68; continue; }
            68 => { r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((rsi as u32)) as u32 as u64; bb = 69; continue; }
            69 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0xeu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 74; continue; } bb = 70; continue; }
            70 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 69; continue; } bb = 71; continue; }
            71 => { xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x7fu64))) as u32) as u128; bb = 72; continue; }
            72 => { rax = (crate::simd_ops::ru32(P10)) as u32 as u64; xmm1 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | (((f32::from_bits(xmm0 as u32)) as f64).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm0 as u64)).to_bits() as u128); fl_l = ((xmm1 as u64)) as u64; fl_r = ((xmm8 as u64)) as u64; if !(f64::from_bits(fl_l) > f64::from_bits(fl_r)) { bb = 81; continue; } bb = 73; continue; }
            73 => { xmm0 = (xmm6) as u128; bb = 82; continue; }
            74 => { rax = ((r9.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = (((rax) & (r11))) as u64; rax = ((rax).wrapping_add(r8)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 71; continue; } bb = 75; continue; }
            75 => { xmm3 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32) as u128; xmm0 = (xmm2) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(core::ptr::read_unaligned((rdx) as *const u32))).to_bits() as u128); fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 77; continue; } bb = 76; continue; }
            76 => { rax = (crate::simd_ops::ru64(P13)) as u64; rax = (((rax) as i64).wrapping_mul((r10) as i64) as u64) as u64; xmm0 = crate::simd_ops::vcvtps2pd(xmm3); rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(P13, (rax) as u64); fl_l = ((rax).wrapping_shr(33u32)) as u64 as u64; rax = ((fl_l as u64)) as u64; xmm1 = (((rax as u32)) as u32) as u128; xmm1 = crate::simd_ops::vcvtdq2pd(xmm1); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm9 as u64)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm0 as u64)).to_bits() as u128); xmm0 = crate::simd_ops::vcvtpd2ps(xmm1); bb = 77; continue; }
            77 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(core::ptr::read_unaligned((rdx) as *const u32))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(core::ptr::read_unaligned((rbp.wrapping_add(0x7fu64)) as *const u32))).to_bits() as u128); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x7fu64), ((xmm0 as u32)) as u32); if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 79; continue; } bb = 78; continue; }
            78 => { crate::simd_ops::wu32(rbp.wrapping_add(0x7fu64), ((xmm2 as u32)) as u32); xmm0 = (xmm2) as u128; bb = 72; continue; }
            79 => { fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm5 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 72; continue; } bb = 80; continue; }
            80 => { crate::simd_ops::wu32(rbp.wrapping_add(0x7fu64), ((xmm5 as u32)) as u32); xmm0 = (xmm5) as u128; bb = 72; continue; }
            81 => { xmm0 = (xmm7) as u128; bb = 82; continue; }
            82 => { xmm3 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32) as u128; xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) + f64::from_bits(xmm0 as u64)).to_bits() as u128); fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; rax = (crate::simd_ops::cvt_f64_i32((f64::from_bits(xmm1 as u64)))) as u32 as u64; crate::simd_ops::wu32(r13.wrapping_add(0x2cu64), ((rax as u32)) as u32); if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 98; continue; } bb = 83; continue; }
            83 => { r8 = (crate::simd_ops::ru64(r15.wrapping_add(0x88u64))) as u64; fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 95; continue; } bb = 84; continue; }
            84 => { r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((rsi as u32)) as u32 as u64; bb = 85; continue; }
            85 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0xdu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 88; continue; } bb = 86; continue; }
            86 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 85; continue; } bb = 87; continue; }
            87 => { bb = 95; continue; }
            88 => { rax = ((r9.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = (((rax) & (r11))) as u64; rax = ((rax).wrapping_add(r8)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 95; continue; } bb = 89; continue; }
            89 => { xmm4 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32) as u128; xmm0 = (xmm2) as u128; xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) - f32::from_bits(core::ptr::read_unaligned((rdx) as *const u32))).to_bits() as u128); fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 91; continue; } bb = 90; continue; }
            90 => { rax = (crate::simd_ops::ru64(P13)) as u64; rax = (((rax) as i64).wrapping_mul((r10) as i64) as u64) as u64; xmm0 = crate::simd_ops::vcvtps2pd(xmm4); rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(P13, (rax) as u64); fl_l = ((rax).wrapping_shr(33u32)) as u64 as u64; rax = ((fl_l as u64)) as u64; xmm1 = (((rax as u32)) as u32) as u128; xmm1 = crate::simd_ops::vcvtdq2pd(xmm1); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm9 as u64)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm0 as u64)).to_bits() as u128); xmm0 = crate::simd_ops::vcvtpd2ps(xmm1); bb = 91; continue; }
            91 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(core::ptr::read_unaligned((rdx) as *const u32))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm3 as u32)).to_bits() as u128); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((xmm0 as u32)) as u32); xmm3 = (xmm0) as u128; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 93; continue; } bb = 92; continue; }
            92 => { crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((xmm2 as u32)) as u32); xmm3 = (xmm2) as u128; bb = 95; continue; }
            93 => { fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm5 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 95; continue; } bb = 94; continue; }
            94 => { crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((xmm5 as u32)) as u32); xmm3 = (xmm5) as u128; bb = 95; continue; }
            95 => { rax = (crate::simd_ops::ru32(P10)) as u32 as u64; xmm1 = 0u128; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | (((f32::from_bits(xmm3 as u32)) as f64).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm0 as u64)).to_bits() as u128); fl_l = ((xmm1 as u64)) as u64; fl_r = ((xmm8 as u64)) as u64; if (f64::from_bits(fl_l) > f64::from_bits(fl_r)) { bb = 97; continue; } bb = 96; continue; }
            96 => { xmm6 = (xmm7) as u128; bb = 97; continue; }
            97 => { xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) + f64::from_bits(xmm6 as u64)).to_bits() as u128); rax = (crate::simd_ops::cvt_f64_i32((f64::from_bits(xmm1 as u64)))) as u32 as u64; rax = (((rax as u32)).wrapping_add(crate::simd_ops::ru32(r13.wrapping_add(0x28u64)))) as u32 as u64; fl_l = (((rax as u32)).wrapping_add(crate::simd_ops::ru32(r13.wrapping_add(0x1cu64)))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; bb = 99; continue; }
            98 => { rax = (0x7fffffffu32) as u32 as u64; bb = 99; continue; }
            99 => { crate::simd_ops::wu32(r13.wrapping_add(0x8u64), ((rax as u32)) as u32); r8 = (crate::simd_ops::ru64(r15.wrapping_add(0x88u64))) as u64; xmm5 = ((crate::simd_ops::ru32(P14)) as u32) as u128; fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 111; continue; } bb = 100; continue; }
            100 => { r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((rsi as u32)) as u32 as u64; bb = 101; continue; }
            101 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0xcu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 104; continue; } bb = 102; continue; }
            102 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 101; continue; } bb = 103; continue; }
            103 => { bb = 111; continue; }
            104 => { rax = ((r9.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = (((rax) & (r11))) as u64; rax = ((rax).wrapping_add(r8)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 111; continue; } bb = 105; continue; }
            105 => { xmm3 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32) as u128; xmm0 = (xmm2) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(core::ptr::read_unaligned((rdx) as *const u32))).to_bits() as u128); fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 107; continue; } bb = 106; continue; }
            106 => { rax = (crate::simd_ops::ru64(P13)) as u64; rax = (((rax) as i64).wrapping_mul((r10) as i64) as u64) as u64; xmm0 = crate::simd_ops::vcvtps2pd(xmm3); rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(P13, (rax) as u64); fl_l = ((rax).wrapping_shr(33u32)) as u64 as u64; rax = ((fl_l as u64)) as u64; xmm1 = (((rax as u32)) as u32) as u128; xmm1 = crate::simd_ops::vcvtdq2pd(xmm1); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm9 as u64)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm0 as u64)).to_bits() as u128); xmm0 = crate::simd_ops::vcvtpd2ps(xmm1); bb = 107; continue; }
            107 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(core::ptr::read_unaligned((rdx) as *const u32))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(core::ptr::read_unaligned((r12) as *const u32))).to_bits() as u128); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 109; continue; } bb = 108; continue; }
            108 => { xmm0 = (xmm2) as u128; bb = 110; continue; }
            109 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) < f32::from_bits(xmm5 as u32) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(xmm5 as u32) }).to_bits() as u128); bb = 110; continue; }
            110 => { crate::simd_ops::wu32(r12, ((xmm0 as u32)) as u32); bb = 111; continue; }
            111 => { xmm0 = ((crate::simd_ops::ru32(r12)) as u32) as u128; xmm4 = ((crate::simd_ops::ru32(P15)) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); crate::simd_ops::wu32(r12, ((xmm0 as u32)) as u32); rdx = (crate::simd_ops::ru64(r15.wrapping_add(0x88u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 123; continue; } bb = 112; continue; }
            112 => { r8 = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; bb = 113; continue; }
            113 => { rcx = ((rsi.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rdx))) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 116; continue; } bb = 114; continue; }
            114 => { rsi = ((rcx as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 113; continue; } bb = 115; continue; }
            115 => { bb = 123; continue; }
            116 => { rax = ((r8.wrapping_add(0x4u64)) as u32) as u32 as u64; rcx = ((rsi as u32)) as u32 as u64; rax = (((rax) & (r11))) as u64; rax = ((rax).wrapping_add(rdx)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 123; continue; } bb = 117; continue; }
            117 => { xmm3 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32) as u128; xmm0 = (xmm2) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(core::ptr::read_unaligned((rdx) as *const u32))).to_bits() as u128); fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 119; continue; } bb = 118; continue; }
            118 => { rax = (crate::simd_ops::ru64(P13)) as u64; rax = (((rax) as i64).wrapping_mul((r10) as i64) as u64) as u64; xmm0 = crate::simd_ops::vcvtps2pd(xmm3); rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(P13, (rax) as u64); fl_l = ((rax).wrapping_shr(33u32)) as u64 as u64; rax = ((fl_l as u64)) as u64; xmm1 = (((rax as u32)) as u32) as u128; xmm1 = crate::simd_ops::vcvtdq2pd(xmm1); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm9 as u64)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm0 as u64)).to_bits() as u128); xmm0 = crate::simd_ops::vcvtpd2ps(xmm1); bb = 119; continue; }
            119 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(core::ptr::read_unaligned((rdx) as *const u32))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(core::ptr::read_unaligned((r13.wrapping_add(0x20u64)) as *const u32))).to_bits() as u128); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 121; continue; } bb = 120; continue; }
            120 => { xmm0 = (xmm2) as u128; bb = 122; continue; }
            121 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) < f32::from_bits(xmm5 as u32) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(xmm5 as u32) }).to_bits() as u128); bb = 122; continue; }
            122 => { crate::simd_ops::wu32(r13.wrapping_add(0x20u64), ((xmm0 as u32)) as u32); bb = 123; continue; }
            123 => { xmm0 = ((crate::simd_ops::ru32(r13.wrapping_add(0x20u64))) as u32) as u128; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x77u64))) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); crate::simd_ops::wu32(r13.wrapping_add(0x20u64), ((xmm0 as u32)) as u32); fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0xdau64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 128; continue; } bb = 124; continue; }
            124 => { xmm1 = ((crate::simd_ops::ru32(rax.wrapping_add(0xc8u64))) as u32) as u128; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 128; continue; } bb = 125; continue; }
            125 => { xmm0 = ((crate::simd_ops::ru32(P16)) as u32) as u128; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 127; continue; } bb = 126; continue; }
            126 => { crate::simd_ops::wu32(r13.wrapping_add(0x18u64), ((xmm0 as u32)) as u32); bb = 129; continue; }
            127 => { crate::simd_ops::wu32(r13.wrapping_add(0x18u64), ((xmm1 as u32)) as u32); bb = 129; continue; }
            128 => { crate::simd_ops::wu32(r13.wrapping_add(0x18u64), ((xmm2 as u32)) as u32); bb = 129; continue; }
            129 => { r11 = ((rsp.wrapping_add(0xe8u64)) as u64) as u64; xmm6 = (crate::simd_ops::ru128(r11.wrapping_sub(0x18u64))) as u128; xmm7 = (crate::simd_ops::ru128(r11.wrapping_sub(0x28u64))) as u128; xmm8 = (crate::simd_ops::ru128(r11.wrapping_sub(0x38u64))) as u128; xmm9 = (crate::simd_ops::ru128(r11.wrapping_sub(0x48u64))) as u128; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416173c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143815b78; const P1: u64 = 0x143815b7c; const P2: u64 = 0x143815b80; const P3: u64 = 0x143815b84; const P4: u64 = 0x143815b88; const P5: u64 = 0x143815b8c; const P6: u64 = 0x143815b90; const P7: u64 = 0x143930fa0; const P8: u64 = 0x1415c3840; const P9: u64 = 0x14268a040; const P10: u64 = 0x143426ea4; const P11: u64 = 0x1434264e4; const P12: u64 = 0x1439310e8; const P13: u64 = 0x14268f770; const P14: u64 = 0x1438152ac; const P15: u64 = 0x1434269f8; const P16: u64 = 0x143426fd0; const P17: u64 = 0x142677278; const P18: u64 = 0x143427144; const P19: u64 = 0x143426ed8; const P20: u64 = 0x141619440; const P21: u64 = 0x1434268d0; const P22: u64 = 0x143426d88; const P23: u64 = 0x142674d50; const P24: u64 = 0x14342668c; const P25: u64 = 0x143426780; const P26: u64 = 0x14183ad76;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 37>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), (rbx) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rax.wrapping_sub(0x5fu64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0xf0u64)) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(0x80u64))) as u64; rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; xmm0 = ((crate::simd_ops::ru32(P0)) as u32) as u128; r14 = (r8) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x48u64), (xmm6) as u128); rsi = (rdx) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x67u64), ((xmm0 as u32)) as u32); r15 = (rcx) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x58u64), (xmm7) as u128); r12 = (0xfffffffcu32) as u32 as u64; fl_l = (r9) as u64 as u64; fl_r = (r9) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdx = ((crate::simd_ops::ru8(r9)) as u32) as u32 as u64; r8 = ((rbx as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rcx = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(r9))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 46; continue; } bb = 3; continue; }
            3 => { r8 = ((rcx as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rdx = (rbx) as u64; bb = 5; continue; }
            5 => { fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; rax = ((rbp.wrapping_add(0x67u64)) as u64) as u64; r13 = ((rsi.wrapping_add(0x18u64)) as u64) as u64; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rdx } else { rax }) as u64; rax = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x67u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x67u64))) as u32) as u128; crate::simd_ops::wu32(r13, ((xmm0 as u32)) as u32); r8 = (crate::simd_ops::ru64(r15.wrapping_add(0x80u64))) as u64; xmm1 = ((crate::simd_ops::ru32(P1)) as u32) as u128; crate::simd_ops::wu32(rbp.wrapping_add(0x67u64), ((xmm1 as u32)) as u32); fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((rbx as u32)) as u32 as u64; bb = 7; continue; }
            7 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 47; continue; } bb = 8; continue; }
            8 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 7; continue; } bb = 9; continue; }
            9 => { xmm0 = ((crate::simd_ops::ru32(P2)) as u32) as u128; rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x67u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x7fu64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x67u64), ((rax as u32)) as u32); fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 10; continue; }
            10 => { r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((rbx as u32)) as u32 as u64; bb = 11; continue; }
            11 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 48; continue; } bb = 12; continue; }
            12 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 11; continue; } bb = 13; continue; }
            13 => { rdx = (rbx) as u64; bb = 14; continue; }
            14 => { fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; rax = ((rbp.wrapping_add(0x7fu64)) as u64) as u64; rdi = ((rsi.wrapping_add(0x28u64)) as u64) as u64; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rdx } else { rax }) as u64; rax = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x7fu64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x7fu64))) as u32) as u128; crate::simd_ops::wu32(rdi, ((xmm0 as u32)) as u32); r8 = (crate::simd_ops::ru64(r15.wrapping_add(0x80u64))) as u64; r10 = (crate::simd_ops::ru32(P3)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r10 as u32)) as u32); fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 15; continue; }
            15 => { r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((rbx as u32)) as u32 as u64; bb = 16; continue; }
            16 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0x5u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 49; continue; } bb = 17; continue; }
            17 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 16; continue; } bb = 18; continue; }
            18 => { xmm0 = ((crate::simd_ops::ru32(P4)) as u32) as u128; crate::simd_ops::wu32(rbp.wrapping_add(0x7fu64), ((xmm0 as u32)) as u32); fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 19; continue; }
            19 => { r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((rbx as u32)) as u32 as u64; bb = 20; continue; }
            20 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0x6u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 50; continue; } bb = 21; continue; }
            21 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 20; continue; } bb = 22; continue; }
            22 => { rdx = (rbx) as u64; bb = 23; continue; }
            23 => { fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; rax = ((rbp.wrapping_add(0x7fu64)) as u64) as u64; r11 = ((rsi.wrapping_add(0x2cu64)) as u64) as u64; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rdx } else { rax }) as u64; rax = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x7fu64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x7fu64))) as u32) as u128; crate::simd_ops::wu32(r11, ((xmm0 as u32)) as u32); r8 = (crate::simd_ops::ru64(r15.wrapping_add(0x80u64))) as u64; xmm1 = ((crate::simd_ops::ru32(P5)) as u32) as u128; crate::simd_ops::wu32(rbp.wrapping_add(0x7fu64), ((xmm1 as u32)) as u32); fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 24; continue; }
            24 => { r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((rbx as u32)) as u32 as u64; bb = 25; continue; }
            25 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0x7u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 51; continue; } bb = 26; continue; }
            26 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 25; continue; } bb = 27; continue; }
            27 => { rdx = (rbx) as u64; bb = 28; continue; }
            28 => { fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; rax = ((rbp.wrapping_add(0x7fu64)) as u64) as u64; r12 = ((rsi.wrapping_add(0x30u64)) as u64) as u64; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rdx } else { rax }) as u64; rax = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x7fu64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x7fu64))) as u32) as u128; crate::simd_ops::wu32(r12, ((xmm0 as u32)) as u32); r8 = (crate::simd_ops::ru64(r15.wrapping_add(0x80u64))) as u64; xmm1 = ((crate::simd_ops::ru32(P6)) as u32) as u128; crate::simd_ops::wu32(rbp.wrapping_add(0x7fu64), ((xmm1 as u32)) as u32); fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 29; continue; }
            29 => { r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((rbx as u32)) as u32 as u64; bb = 30; continue; }
            30 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0x8u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 52; continue; } bb = 31; continue; }
            31 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 30; continue; } bb = 32; continue; }
            32 => { rdx = (rbx) as u64; bb = 33; continue; }
            33 => { fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; xmm1 = (((r10 as u32)) as u32) as u128; rcx = ((rbp.wrapping_add(0x67u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), (rdi) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x51u64), (rcx) as u64); rax = ((rbp.wrapping_add(0x7fu64)) as u64) as u64; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rdx } else { rax }) as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x69u64), ((rbx as u32)) as u32); rcx = ((rbp.wrapping_add(0x7fu64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x61u64), (r13) as u64); xmm1 = crate::simd_ops::vcvtdq2ps(xmm1); crate::simd_ops::wu64(rbp.wrapping_sub(0x31u64), (rcx) as u64); rdi = ((rbp.wrapping_sub(0x69u64)) as u64) as u64; rax = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_l = ((r14).wrapping_add(0x8u64)) as u64 as u64; fl_r = fl_l; r14 = ((fl_l as u64)) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x7fu64), ((rax as u32)) as u32); rax = ((rsi.wrapping_add(0x20u64)) as u64) as u64; rsi = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x1u64), (rax) as u64); xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x7fu64))) as u32) as u128; crate::simd_ops::wu32(rax, ((xmm0 as u32)) as u32); rax = (crate::simd_ops::ru32(r15.wrapping_add(0x50u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x7fu64), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x59u64), (0x1u32) as u32); rdx = ((rax.wrapping_mul(8)) as u64) as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x49u64), (0x2u32) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x39u64), (0x3u32) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x29u64), (0x4u32) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x21u64), (r11) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x19u64), (0x5u32) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), (r12) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x9u64), (0x6u32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rcx) as u64); bb = 34; continue; }
            34 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r8 = ((rdx.wrapping_add(rcx)) as u64) as u64; r9 = (crate::simd_ops::ru32(rdi)) as u32 as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = (crate::simd_ops::ru64(P7)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r14) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru32(r15.wrapping_add(0x50u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x48u64))) as u64; rdx = ((rax.wrapping_mul(8)) as u64) as u64; rax = ((rcx.wrapping_add(rdx)) as u64) as u64; fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 36; continue; } bb = 35; continue; }
            35 => { rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; rdi = ((rdi).wrapping_add(0x10u64)) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = (0x7u32) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 34; continue; } bb = 36; continue; }
            36 => { rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(core::ptr::read_unaligned((rbp.wrapping_add(0x7fu64)) as *const u32))))) as u32 as u64; r14 = (crate::simd_ops::ru64(rbp.wrapping_add(0x6fu64))) as u64; r10 = (0x5851f42d4c957f2du64) as u64; xmm5 = ((crate::simd_ops::ru64(P9)) as u64) as u128; xmm6 = 0u128; xmm4 = ((crate::simd_ops::ru32(P10)) as u32) as u128; crate::simd_ops::wu32(r14.wrapping_add(0x24u64), ((rax as u32)) as u32); r8 = (crate::simd_ops::ru64(r15.wrapping_add(0x88u64))) as u64; fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 40; continue; } bb = 37; continue; }
            37 => { r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((rbx as u32)) as u32 as u64; bb = 38; continue; }
            38 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 53; continue; } bb = 39; continue; }
            39 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 38; continue; } bb = 40; continue; }
            40 => { r11 = (0xfffffffcu32) as u32 as u64; bb = 41; continue; }
            41 => { xmm0 = ((crate::simd_ops::ru32(r13)) as u32) as u128; xmm3 = ((crate::simd_ops::ru32(P11)) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); crate::simd_ops::wu32(r13, ((xmm0 as u32)) as u32); r8 = (crate::simd_ops::ru64(r15.wrapping_add(0x88u64))) as u64; fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 66; continue; } bb = 42; continue; }
            42 => { r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((rbx as u32)) as u32 as u64; bb = 43; continue; }
            43 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 59; continue; } bb = 44; continue; }
            44 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 43; continue; } bb = 45; continue; }
            45 => { bb = 66; continue; }
            46 => { rax = ((rdx.wrapping_add(0x4u64)) as u32) as u32 as u64; rcx = ((r8 as u32)) as u32 as u64; rax = (((rax) & (r12))) as u64; fl_l = ((rax).wrapping_add(r9)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; bb = 5; continue; }
            47 => { xmm0 = ((crate::simd_ops::ru32(P2)) as u32) as u128; rax = ((r9.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = (((rax) & (r12))) as u64; rax = ((rax).wrapping_add(r8)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; rax = ((rbp.wrapping_add(0x67u64)) as u64) as u64; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rdx } else { rax }) as u64; rax = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x7fu64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x67u64), ((rax as u32)) as u32); bb = 10; continue; }
            48 => { rax = ((r9.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = (((rax) & (r12))) as u64; fl_l = ((rax).wrapping_add(r8)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; bb = 14; continue; }
            49 => { xmm0 = ((crate::simd_ops::ru32(P4)) as u32) as u128; rax = ((r9.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = (((rax) & (r12))) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x7fu64), ((xmm0 as u32)) as u32); rax = ((rax).wrapping_add(r8)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; rax = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rdx } else { rax }) as u64; r10 = (crate::simd_ops::ru32(rax)) as u32 as u64; bb = 19; continue; }
            50 => { rax = ((r9.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = (((rax) & (r12))) as u64; fl_l = ((rax).wrapping_add(r8)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; bb = 23; continue; }
            51 => { rax = ((r9.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = (((rax) & (r12))) as u64; fl_l = ((rax).wrapping_add(r8)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; bb = 28; continue; }
            52 => { rdx = (0xfffffffcu32) as u32 as u64; rax = ((r9.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = (((rax) & (rdx))) as u64; fl_l = ((rax).wrapping_add(r8)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; bb = 33; continue; }
            53 => { rax = ((r9.wrapping_add(0x4u64)) as u32) as u32 as u64; r11 = (0xfffffffcu32) as u32 as u64; rax = (((rax) & (r11))) as u64; rax = ((rax).wrapping_add(r8)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 41; continue; } bb = 54; continue; }
            54 => { xmm2 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32) as u128; xmm0 = (xmm6) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(core::ptr::read_unaligned((rdx) as *const u32))).to_bits() as u128); fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 56; continue; } bb = 55; continue; }
            55 => { rax = (crate::simd_ops::ru64(P12)) as u64; rax = (((rax) as i64).wrapping_mul((r10) as i64) as u64) as u64; xmm0 = crate::simd_ops::vcvtps2pd(xmm2); rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(P12, (rax) as u64); fl_l = ((rax).wrapping_shr(33u32)) as u64 as u64; rax = ((fl_l as u64)) as u64; xmm1 = (((rax as u32)) as u32) as u128; xmm1 = crate::simd_ops::vcvtdq2pd(xmm1); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm5 as u64)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm0 as u64)).to_bits() as u128); xmm0 = crate::simd_ops::vcvtpd2ps(xmm1); bb = 56; continue; }
            56 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(core::ptr::read_unaligned((rdx) as *const u32))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(core::ptr::read_unaligned((r13) as *const u32))).to_bits() as u128); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 58; continue; } bb = 57; continue; }
            57 => { xmm0 = (xmm6) as u128; crate::simd_ops::wu32(r13, ((xmm0 as u32)) as u32); bb = 41; continue; }
            58 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) < f32::from_bits(xmm4 as u32) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(xmm4 as u32) }).to_bits() as u128); crate::simd_ops::wu32(r13, ((xmm0 as u32)) as u32); bb = 41; continue; }
            59 => { rax = ((r9.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = (((rax) & (r11))) as u64; rax = ((rax).wrapping_add(r8)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 66; continue; } bb = 60; continue; }
            60 => { xmm2 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32) as u128; xmm0 = (xmm6) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(core::ptr::read_unaligned((rdx) as *const u32))).to_bits() as u128); fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 62; continue; } bb = 61; continue; }
            61 => { rax = (crate::simd_ops::ru64(P12)) as u64; rax = (((rax) as i64).wrapping_mul((r10) as i64) as u64) as u64; xmm0 = crate::simd_ops::vcvtps2pd(xmm2); rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(P12, (rax) as u64); fl_l = ((rax).wrapping_shr(33u32)) as u64 as u64; rax = ((fl_l as u64)) as u64; xmm1 = (((rax as u32)) as u32) as u128; xmm1 = crate::simd_ops::vcvtdq2pd(xmm1); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm5 as u64)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm0 as u64)).to_bits() as u128); xmm0 = crate::simd_ops::vcvtpd2ps(xmm1); bb = 62; continue; }
            62 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(core::ptr::read_unaligned((rdx) as *const u32))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(core::ptr::read_unaligned((r14.wrapping_add(0x28u64)) as *const u32))).to_bits() as u128); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 64; continue; } bb = 63; continue; }
            63 => { xmm0 = (xmm6) as u128; bb = 65; continue; }
            64 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) < f32::from_bits(core::ptr::read_unaligned((P13) as *const u32)) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(core::ptr::read_unaligned((P13) as *const u32)) }).to_bits() as u128); bb = 65; continue; }
            65 => { crate::simd_ops::wu32(r14.wrapping_add(0x28u64), ((xmm0 as u32)) as u32); bb = 66; continue; }
            66 => { r8 = (crate::simd_ops::ru64(r15.wrapping_add(0x88u64))) as u64; fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 78; continue; } bb = 67; continue; }
            67 => { r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((rbx as u32)) as u32 as u64; bb = 68; continue; }
            68 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0x7u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 71; continue; } bb = 69; continue; }
            69 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 68; continue; } bb = 70; continue; }
            70 => { bb = 78; continue; }
            71 => { rax = ((r9.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = (((rax) & (r11))) as u64; rax = ((rax).wrapping_add(r8)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 78; continue; } bb = 72; continue; }
            72 => { xmm2 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32) as u128; xmm0 = (xmm6) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(core::ptr::read_unaligned((rdx) as *const u32))).to_bits() as u128); fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 74; continue; } bb = 73; continue; }
            73 => { rax = (crate::simd_ops::ru64(P12)) as u64; rax = (((rax) as i64).wrapping_mul((r10) as i64) as u64) as u64; xmm0 = crate::simd_ops::vcvtps2pd(xmm2); rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(P12, (rax) as u64); fl_l = ((rax).wrapping_shr(33u32)) as u64 as u64; rax = ((fl_l as u64)) as u64; xmm1 = (((rax as u32)) as u32) as u128; xmm1 = crate::simd_ops::vcvtdq2pd(xmm1); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm5 as u64)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm0 as u64)).to_bits() as u128); xmm0 = crate::simd_ops::vcvtpd2ps(xmm1); bb = 74; continue; }
            74 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(core::ptr::read_unaligned((rdx) as *const u32))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(core::ptr::read_unaligned((r12) as *const u32))).to_bits() as u128); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 76; continue; } bb = 75; continue; }
            75 => { xmm0 = (xmm6) as u128; bb = 77; continue; }
            76 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) < f32::from_bits(xmm4 as u32) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(xmm4 as u32) }).to_bits() as u128); bb = 77; continue; }
            77 => { crate::simd_ops::wu32(r12, ((xmm0 as u32)) as u32); bb = 78; continue; }
            78 => { xmm0 = ((crate::simd_ops::ru32(r12)) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); crate::simd_ops::wu32(r12, ((xmm0 as u32)) as u32); r8 = (crate::simd_ops::ru64(r15.wrapping_add(0x88u64))) as u64; fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 90; continue; } bb = 79; continue; }
            79 => { r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((rbx as u32)) as u32 as u64; bb = 80; continue; }
            80 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0x6u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 83; continue; } bb = 81; continue; }
            81 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 80; continue; } bb = 82; continue; }
            82 => { bb = 90; continue; }
            83 => { rax = ((r9.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = (((rax) & (r11))) as u64; rax = ((rax).wrapping_add(r8)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 90; continue; } bb = 84; continue; }
            84 => { xmm2 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32) as u128; xmm0 = (xmm6) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(core::ptr::read_unaligned((rdx) as *const u32))).to_bits() as u128); fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 86; continue; } bb = 85; continue; }
            85 => { rax = (crate::simd_ops::ru64(P12)) as u64; rax = (((rax) as i64).wrapping_mul((r10) as i64) as u64) as u64; xmm0 = crate::simd_ops::vcvtps2pd(xmm2); rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(P12, (rax) as u64); fl_l = ((rax).wrapping_shr(33u32)) as u64 as u64; rax = ((fl_l as u64)) as u64; xmm1 = (((rax as u32)) as u32) as u128; xmm1 = crate::simd_ops::vcvtdq2pd(xmm1); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm5 as u64)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm0 as u64)).to_bits() as u128); xmm0 = crate::simd_ops::vcvtpd2ps(xmm1); bb = 86; continue; }
            86 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(core::ptr::read_unaligned((rdx) as *const u32))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(core::ptr::read_unaligned((r14.wrapping_add(0x2cu64)) as *const u32))).to_bits() as u128); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 88; continue; } bb = 87; continue; }
            87 => { xmm0 = (xmm6) as u128; bb = 89; continue; }
            88 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) < f32::from_bits(xmm4 as u32) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(xmm4 as u32) }).to_bits() as u128); bb = 89; continue; }
            89 => { crate::simd_ops::wu32(r14.wrapping_add(0x2cu64), ((xmm0 as u32)) as u32); bb = 90; continue; }
            90 => { xmm0 = ((crate::simd_ops::ru32(r14.wrapping_add(0x2cu64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); crate::simd_ops::wu32(r14.wrapping_add(0x2cu64), ((xmm0 as u32)) as u32); r8 = (crate::simd_ops::ru64(r15.wrapping_add(0x88u64))) as u64; fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 94; continue; } bb = 91; continue; }
            91 => { r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((rbx as u32)) as u32 as u64; bb = 92; continue; }
            92 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 97; continue; } bb = 93; continue; }
            93 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 92; continue; } bb = 94; continue; }
            94 => { xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x67u64))) as u32) as u128; bb = 95; continue; }
            95 => { rax = (crate::simd_ops::ru32(P14)) as u32 as u64; xmm2 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | (((f32::from_bits(xmm0 as u32)) as f64).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); xmm1 = 0u128; xmm2 = (xmm2 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm2 as u64) * f64::from_bits(xmm0 as u64)).to_bits() as u128); fl_l = ((xmm2 as u64)) as u64; fl_r = ((xmm1 as u64)) as u64; if !(f64::from_bits(fl_l) > f64::from_bits(fl_r)) { bb = 104; continue; } bb = 96; continue; }
            96 => { xmm0 = ((crate::simd_ops::ru64(P15)) as u64) as u128; bb = 105; continue; }
            97 => { rax = ((r9.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = (((rax) & (r11))) as u64; rax = ((rax).wrapping_add(r8)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 94; continue; } bb = 98; continue; }
            98 => { xmm2 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32) as u128; xmm0 = (xmm6) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(core::ptr::read_unaligned((rdx) as *const u32))).to_bits() as u128); fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 100; continue; } bb = 99; continue; }
            99 => { rax = (crate::simd_ops::ru64(P12)) as u64; rax = (((rax) as i64).wrapping_mul((r10) as i64) as u64) as u64; xmm0 = crate::simd_ops::vcvtps2pd(xmm2); rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(P12, (rax) as u64); fl_l = ((rax).wrapping_shr(33u32)) as u64 as u64; rax = ((fl_l as u64)) as u64; xmm1 = (((rax as u32)) as u32) as u128; xmm1 = crate::simd_ops::vcvtdq2pd(xmm1); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm5 as u64)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm0 as u64)).to_bits() as u128); xmm0 = crate::simd_ops::vcvtpd2ps(xmm1); bb = 100; continue; }
            100 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(core::ptr::read_unaligned((rdx) as *const u32))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(core::ptr::read_unaligned((rbp.wrapping_add(0x67u64)) as *const u32))).to_bits() as u128); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x67u64), ((xmm0 as u32)) as u32); if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 102; continue; } bb = 101; continue; }
            101 => { crate::simd_ops::wu32(rbp.wrapping_add(0x67u64), ((xmm6 as u32)) as u32); xmm0 = (xmm6) as u128; bb = 95; continue; }
            102 => { xmm1 = ((crate::simd_ops::ru32(P16)) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 95; continue; } bb = 103; continue; }
            103 => { crate::simd_ops::wu32(rbp.wrapping_add(0x67u64), ((xmm1 as u32)) as u32); xmm0 = (xmm1) as u128; bb = 95; continue; }
            104 => { xmm0 = ((crate::simd_ops::ru64(P17)) as u64) as u128; bb = 105; continue; }
            105 => { xmm2 = (xmm2 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm2 as u64) + f64::from_bits(xmm0 as u64)).to_bits() as u128); rax = (crate::simd_ops::cvt_f64_i32((f64::from_bits(xmm2 as u64)))) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x1cu64), ((rax as u32)) as u32); rdx = (crate::simd_ops::ru64(r15.wrapping_add(0x88u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 116; continue; } bb = 106; continue; }
            106 => { r8 = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; bb = 107; continue; }
            107 => { rcx = ((rbx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rdx))) as u8 as u64; fl_r = (0x8u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 110; continue; } bb = 108; continue; }
            108 => { rbx = ((rcx as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 107; continue; } bb = 109; continue; }
            109 => { bb = 116; continue; }
            110 => { rax = ((r8.wrapping_add(0x4u64)) as u32) as u32 as u64; rcx = ((rbx as u32)) as u32 as u64; rax = (((rax) & (r11))) as u64; rax = ((rax).wrapping_add(rdx)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 116; continue; } bb = 111; continue; }
            111 => { xmm2 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32) as u128; xmm0 = (xmm6) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(core::ptr::read_unaligned((rdx) as *const u32))).to_bits() as u128); fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 113; continue; } bb = 112; continue; }
            112 => { rax = (crate::simd_ops::ru64(P12)) as u64; rax = (((rax) as i64).wrapping_mul((r10) as i64) as u64) as u64; xmm0 = crate::simd_ops::vcvtps2pd(xmm2); rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(P12, (rax) as u64); fl_l = ((rax).wrapping_shr(33u32)) as u64 as u64; rax = ((fl_l as u64)) as u64; xmm1 = (((rax as u32)) as u32) as u128; xmm1 = crate::simd_ops::vcvtdq2pd(xmm1); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm5 as u64)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm0 as u64)).to_bits() as u128); xmm0 = crate::simd_ops::vcvtpd2ps(xmm1); bb = 113; continue; }
            113 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(core::ptr::read_unaligned((rdx) as *const u32))).to_bits() as u128); xmm1 = ((crate::simd_ops::ru32(P18)) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(core::ptr::read_unaligned((r14.wrapping_add(0x20u64)) as *const u32))).to_bits() as u128); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 115; continue; } bb = 114; continue; }
            114 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) < f32::from_bits(core::ptr::read_unaligned((P19) as *const u32)) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(core::ptr::read_unaligned((P19) as *const u32)) }).to_bits() as u128); xmm1 = (xmm0) as u128; bb = 115; continue; }
            115 => { crate::simd_ops::wu32(r14.wrapping_add(0x20u64), ((xmm1 as u32)) as u32); bb = 116; continue; }
            116 => { rbx = (crate::simd_ops::ru64(P12)) as u64; r8 = ((r14.wrapping_add(0x24u64)) as u64) as u64; rdx = (crate::simd_ops::ru32(P14)) as u32 as u64; rcx = ((r14.wrapping_add(0x38u64)) as u64) as u64; rbx = (((rbx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rbx = ((rbx).wrapping_add(1)) as u64; crate::simd_ops::wu64(P12, (rbx) as u64); xmm7 = ((crate::simd_ops::ru32(r14.wrapping_add(0x20u64))) as u32) as u128; rbx = ((rbx).wrapping_shr(33u32)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P20) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::ru32(r14.wrapping_add(0x24u64))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; xmm0 = (crate::simd_ops::ru128(r14.wrapping_add(0x38u64))) as u128; xmm2 = (crate::simd_ops::ru128(r14.wrapping_add(0x48u64))) as u128; xmm4 = ((crate::simd_ops::ru32(P21)) as u32) as u128; xmm3 = ((crate::simd_ops::ru32(P22)) as u32) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x38u64), (xmm0) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x48u64), (xmm2) as u128); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 118; continue; } bb = 117; continue; }
            117 => { xmm1 = (xmm3) as u128; bb = 119; continue; }
            118 => { xmm1 = (xmm4) as u128; bb = 119; continue; }
            119 => { xmm0 = (xmm1) as u128; rax = ((xmm2 as u64)) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm7 as u32)).to_bits() as u128); rax = ((rax).wrapping_shr(32u32)) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(core::ptr::read_unaligned((P23) as *const u32))).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), ((xmm0 as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 121; continue; } bb = 120; continue; }
            120 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(core::ptr::read_unaligned((P24) as *const u32))).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), ((xmm0 as u32)) as u32); bb = 125; continue; }
            121 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 123; continue; } bb = 122; continue; }
            122 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(core::ptr::read_unaligned((P25) as *const u32))).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), ((xmm0 as u32)) as u32); bb = 125; continue; }
            123 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x5u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 125; continue; } bb = 124; continue; }
            124 => { rcx = (0x5851f42d4c957f2du64) as u64; rbx = (((rbx) as i64).wrapping_mul((rcx) as i64) as u64) as u64; fl_l = ((rbx).wrapping_add(1)) as u64 as u64; rbx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x79u64), (rbx) as u64); xmm2 = (crate::simd_ops::ru128(rsp.wrapping_add(0x48u64))) as u128; bb = 125; continue; }
            125 => { fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 127; continue; } bb = 126; continue; }
            126 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), ((xmm0 as u32)) as u32); bb = 127; continue; }
            127 => { fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 129; continue; } bb = 128; continue; }
            128 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm1 as u32)).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), ((xmm0 as u32)) as u32); bb = 129; continue; }
            129 => { xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x38u64))) as u128; crate::simd_ops::wu128(r14.wrapping_add(0x38u64), (xmm1) as u128); crate::simd_ops::wu128(r14.wrapping_add(0x48u64), (xmm2) as u128); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 131; continue; } bb = 130; continue; }
            130 => { xmm1 = (xmm3) as u128; bb = 133; continue; }
            131 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x5u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 134; continue; } bb = 132; continue; }
            132 => { xmm1 = (xmm4) as u128; bb = 133; continue; }
            133 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P26) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(r14.wrapping_add(0x44u64), ((xmm0 as u32)) as u32); bb = 134; continue; }
            134 => { r11 = ((rsp.wrapping_add(0xf0u64)) as u64) as u64; crate::simd_ops::wu32(r14.wrapping_add(0x8u64), (0x7fffffffu32) as u32); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x50u64))) as u64; xmm6 = (crate::simd_ops::ru128(r11.wrapping_sub(0x10u64))) as u128; xmm7 = (crate::simd_ops::ru128(r11.wrapping_sub(0x20u64))) as u128; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141617de0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143815bb0; const P1: u64 = 0x1438152ac; const P2: u64 = 0x1434269f8; const P3: u64 = 0x142677278; const P4: u64 = 0x143815bb4; const P5: u64 = 0x143815b74; const P6: u64 = 0x143815bb8; const P7: u64 = 0x143815bbc; const P8: u64 = 0x143930fa0; const P9: u64 = 0x1415c3840; const P10: u64 = 0x143426d10; const P11: u64 = 0x14268a040; const P12: u64 = 0x1439310e8; const P13: u64 = 0x14342668c; const P14: u64 = 0x1415ac1a0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm8: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp; rsp = ((rsp).wrapping_sub(0x80u64)) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(0x80u64))) as u64; rbx = 0; fl_l = 0; fl_r = 0; xmm0 = ((crate::simd_ops::ru32(P0)) as u32) as u128; r15 = r8; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm6) as u128); r10 = rdx; crate::simd_ops::wu32(rbp.wrapping_add(0x40u64), ((xmm0 as u32)) as u32); r13 = rcx; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm8) as u128); r11 = 0xfffffffc; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdx = ((crate::simd_ops::ru8(r9)) as u32) as u32 as u64; r8 = ((rbx as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rcx = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(r9))) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } bb = 3; continue; }
            3 => { r8 = ((rcx as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rdx = rbx; bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(P1)) as u32 as u64; rcx = rbp.wrapping_add(0x40); xmm6 = ((crate::simd_ops::ru64(P2)) as u64) as u128; fl_l = rdx; fl_r = rdx; xmm2 = 0u128; xmm8 = 0u128; rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rdx } else { rcx }) as u64; xmm2 = (xmm2 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); xmm0 = ((crate::simd_ops::ru32(rcx)) as u32) as u128; xmm1 = crate::simd_ops::vcvtps2pd(xmm0); xmm2 = (xmm2 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm2 as u64) * f64::from_bits(xmm1 as u64)).to_bits() as u128); fl_l = ((xmm2 as u64)) as u64; fl_r = ((xmm8 as u64)) as u64; if !(f64::from_bits(fl_l) > f64::from_bits(fl_r)) { bb = 8; continue; } bb = 6; continue; }
            6 => { xmm0 = (xmm6) as u128; bb = 9; continue; }
            7 => { rax = ((rdx.wrapping_add(0x4u64)) as u32) as u32 as u64; rcx = ((r8 as u32)) as u32 as u64; rax = (((rax) & (r11))) as u64; fl_l = ((rax).wrapping_add(r9)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; bb = 5; continue; }
            8 => { xmm0 = ((crate::simd_ops::ru64(P3)) as u64) as u128; bb = 9; continue; }
            9 => { xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) + f64::from_bits(xmm2 as u64)).to_bits() as u128); rax = (crate::simd_ops::cvt_f64_i32((f64::from_bits(xmm0 as u64)))) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(0x18u64), ((rax as u32)) as u32); r8 = (crate::simd_ops::ru64(r13.wrapping_add(0x80u64))) as u64; r14 = (crate::simd_ops::ru32(P4)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x50u64), ((r14 as u32)) as u32); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 10; continue; }
            10 => { r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((rbx as u32)) as u32 as u64; bb = 11; continue; }
            11 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0x11u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 48; continue; } bb = 12; continue; }
            12 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 11; continue; } bb = 13; continue; }
            13 => { r12 = (crate::simd_ops::ru32(P5)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x58u64), ((r12 as u32)) as u32); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 14; continue; }
            14 => { r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((rbx as u32)) as u32 as u64; bb = 15; continue; }
            15 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 49; continue; } bb = 16; continue; }
            16 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 15; continue; } bb = 17; continue; }
            17 => { xmm0 = ((crate::simd_ops::ru32(P6)) as u32) as u128; crate::simd_ops::wu32(rbp.wrapping_add(0x40u64), ((xmm0 as u32)) as u32); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 18; continue; }
            18 => { r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((rbx as u32)) as u32 as u64; bb = 19; continue; }
            19 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0x12u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 50; continue; } bb = 20; continue; }
            20 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 19; continue; } bb = 21; continue; }
            21 => { xmm0 = ((crate::simd_ops::ru32(P7)) as u32) as u128; rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x40u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x58u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x40u64), ((rax as u32)) as u32); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 22; continue; }
            22 => { r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((rbx as u32)) as u32 as u64; bb = 23; continue; }
            23 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0x13u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 51; continue; } bb = 24; continue; }
            24 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 23; continue; } bb = 25; continue; }
            25 => { rdx = rbx; bb = 26; continue; }
            26 => { rcx = (crate::simd_ops::ru64(r13.wrapping_add(0x48u64))) as u64; rax = rbp.wrapping_add(0x58); fl_l = rdx; fl_r = rdx; crate::simd_ops::wu32(rbp.wrapping_sub(0x40u64), (0xeu32) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x30u64), (0xfu32) as u32); rsi = rbp.wrapping_sub(0x40); rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rdx } else { rax }) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), rcx); rdi = ((rbx as u32)) as u32 as u64; fl_l = ((r15).wrapping_add(0x8u64)) as u64; fl_r = fl_l; r15 = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x58u64), ((rax as u32)) as u32); rax = rbp.wrapping_add(0x40); crate::simd_ops::wu64(rbp.wrapping_sub(0x38u64), rax); rax = rbp.wrapping_add(0x58); crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), rax); rax = (crate::simd_ops::ru32(r13.wrapping_add(0x50u64))) as u32 as u64; rdx = ((rax.wrapping_mul(8)) as u64) as u64; bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; r8 = ((rdx.wrapping_add(rcx)) as u64) as u64; r9 = (crate::simd_ops::ru32(rsi)) as u32 as u64; rdx = rbp.wrapping_sub(0x48); rcx = (crate::simd_ops::ru64(P8)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(r13.wrapping_add(0x50u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(r13.wrapping_add(0x48u64))) as u64; rdx = ((rax.wrapping_mul(8)) as u64) as u64; rax = ((rcx.wrapping_add(rdx)) as u64) as u64; fl_l = (crate::simd_ops::ru64(rbp.wrapping_sub(0x48u64))) as u64; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 29; continue; } bb = 28; continue; }
            28 => { rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; rsi = ((rsi).wrapping_add(0x10u64)) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x2; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 27; continue; } bb = 29; continue; }
            29 => { r8 = (crate::simd_ops::ru64(r13.wrapping_add(0x88u64))) as u64; r10 = (0x5851f42d4c957f2du64) as u64; xmm4 = ((crate::simd_ops::ru32(P10)) as u32) as u128; xmm3 = 0u128; xmm5 = ((crate::simd_ops::ru64(P11)) as u64) as u128; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 30; continue; }
            30 => { r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((rbx as u32)) as u32 as u64; bb = 31; continue; }
            31 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0x12u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 52; continue; } bb = 32; continue; }
            32 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 31; continue; } bb = 33; continue; }
            33 => { r11 = 0xfffffffc; bb = 34; continue; }
            34 => { xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x40u64))) as u32) as u128; bb = 35; continue; }
            35 => { rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x20u64), ((xmm0 as u32)) as u32); r8 = (crate::simd_ops::ru64(r13.wrapping_add(0x88u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 39; continue; } bb = 36; continue; }
            36 => { r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((rbx as u32)) as u32 as u64; bb = 37; continue; }
            37 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0x13u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 59; continue; } bb = 38; continue; }
            38 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 37; continue; } bb = 39; continue; }
            39 => { xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x58u64))) as u32) as u128; bb = 40; continue; }
            40 => { rax = (crate::simd_ops::ru32(P1)) as u32 as u64; xmm1 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | (((f32::from_bits(xmm0 as u32)) as f64).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm0 as u64)).to_bits() as u128); fl_l = ((xmm1 as u64)) as u64; fl_r = ((xmm8 as u64)) as u64; if (f64::from_bits(fl_l) > f64::from_bits(fl_r)) { bb = 42; continue; } bb = 41; continue; }
            41 => { xmm6 = ((crate::simd_ops::ru64(P3)) as u64) as u128; bb = 42; continue; }
            42 => { xmm6 = (xmm6 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm6 as u64) + f64::from_bits(xmm1 as u64)).to_bits() as u128); rdi = 0x7fffffff; rax = (crate::simd_ops::cvt_f64_i32((f64::from_bits(xmm6 as u64)))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x24u64), ((rax as u32)) as u32); fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 71; continue; } bb = 43; continue; }
            43 => { rdx = (crate::simd_ops::ru64(r13.wrapping_add(0x88u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 70; continue; } bb = 44; continue; }
            44 => { r8 = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; bb = 45; continue; }
            45 => { rcx = ((rbx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rdx))) as u8 as u64; fl_r = (0x11u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 66; continue; } bb = 46; continue; }
            46 => { rbx = ((rcx as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 45; continue; } bb = 47; continue; }
            47 => { r14 = ((((r14 as u32)) as i32).wrapping_mul((crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as i32) as u32) as u32 as u64; bb = 72; continue; }
            48 => { r12 = (crate::simd_ops::ru32(P5)) as u32 as u64; rax = ((r9.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = (((rax) & (r11))) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x58u64), ((r12 as u32)) as u32); rax = ((rax).wrapping_add(r8)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; fl_l = rdx; fl_r = rdx; rax = rbp.wrapping_sub(0x50); rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rdx } else { rax }) as u64; r14 = (crate::simd_ops::ru32(rax)) as u32 as u64; bb = 14; continue; }
            49 => { xmm0 = ((crate::simd_ops::ru32(P6)) as u32) as u128; rax = ((r9.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = (((rax) & (r11))) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x40u64), ((xmm0 as u32)) as u32); rax = ((rax).wrapping_add(r8)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; fl_l = rdx; fl_r = rdx; rax = rbp.wrapping_add(0x58); rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rdx } else { rax }) as u64; r12 = (crate::simd_ops::ru32(rax)) as u32 as u64; bb = 18; continue; }
            50 => { xmm0 = ((crate::simd_ops::ru32(P7)) as u32) as u128; rax = ((r9.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = (((rax) & (r11))) as u64; rax = ((rax).wrapping_add(r8)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; fl_l = rdx; fl_r = rdx; rax = rbp.wrapping_add(0x40); rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rdx } else { rax }) as u64; rax = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x58u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x40u64), ((rax as u32)) as u32); bb = 22; continue; }
            51 => { rax = ((r9.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = (((rax) & (r11))) as u64; fl_l = ((rax).wrapping_add(r8)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; bb = 26; continue; }
            52 => { rax = ((r9.wrapping_add(0x4u64)) as u32) as u32 as u64; r11 = 0xfffffffc; rax = (((rax) & (r11))) as u64; rax = ((rax).wrapping_add(r8)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 34; continue; } bb = 53; continue; }
            53 => { xmm2 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32) as u128; xmm0 = (xmm3) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(crate::simd_ops::ru32(rdx))).to_bits() as u128); fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm3 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 55; continue; } bb = 54; continue; }
            54 => { rax = (crate::simd_ops::ru64(P12)) as u64; rax = (((rax) as i64).wrapping_mul((r10) as i64) as u64) as u64; xmm0 = crate::simd_ops::vcvtps2pd(xmm2); rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(P12, rax); fl_l = ((rax).wrapping_shr(33u32)) as u64; rax = ((fl_l as u64)) as u64; xmm1 = (((rax as u32)) as u32) as u128; xmm1 = crate::simd_ops::vcvtdq2pd(xmm1); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm5 as u64)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm0 as u64)).to_bits() as u128); xmm0 = crate::simd_ops::vcvtpd2ps(xmm1); bb = 55; continue; }
            55 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rdx))).to_bits() as u128); xmm1 = ((crate::simd_ops::ru32(P13)) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rbp.wrapping_add(0x40u64)))).to_bits() as u128); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x40u64), ((xmm0 as u32)) as u32); if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 57; continue; } bb = 56; continue; }
            56 => { crate::simd_ops::wu32(rbp.wrapping_add(0x40u64), ((xmm1 as u32)) as u32); xmm0 = (xmm1) as u128; bb = 35; continue; }
            57 => { fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm4 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 35; continue; } bb = 58; continue; }
            58 => { crate::simd_ops::wu32(rbp.wrapping_add(0x40u64), ((xmm4 as u32)) as u32); xmm0 = (xmm4) as u128; bb = 35; continue; }
            59 => { rax = ((r9.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = (((rax) & (r11))) as u64; rax = ((rax).wrapping_add(r8)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 39; continue; } bb = 60; continue; }
            60 => { xmm2 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32) as u128; xmm0 = (xmm3) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(crate::simd_ops::ru32(rdx))).to_bits() as u128); fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm3 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 62; continue; } bb = 61; continue; }
            61 => { rax = (crate::simd_ops::ru64(P12)) as u64; rax = (((rax) as i64).wrapping_mul((r10) as i64) as u64) as u64; xmm0 = crate::simd_ops::vcvtps2pd(xmm2); rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(P12, rax); fl_l = ((rax).wrapping_shr(33u32)) as u64; rax = ((fl_l as u64)) as u64; xmm1 = (((rax as u32)) as u32) as u128; xmm1 = crate::simd_ops::vcvtdq2pd(xmm1); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm5 as u64)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm0 as u64)).to_bits() as u128); xmm0 = crate::simd_ops::vcvtpd2ps(xmm1); bb = 62; continue; }
            62 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rdx))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rbp.wrapping_add(0x58u64)))).to_bits() as u128); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm3 as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x58u64), ((xmm0 as u32)) as u32); if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 64; continue; } bb = 63; continue; }
            63 => { crate::simd_ops::wu32(rbp.wrapping_add(0x58u64), ((xmm3 as u32)) as u32); xmm0 = (xmm3) as u128; bb = 40; continue; }
            64 => { fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm4 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 40; continue; } bb = 65; continue; }
            65 => { crate::simd_ops::wu32(rbp.wrapping_add(0x58u64), ((xmm4 as u32)) as u32); xmm0 = (xmm4) as u128; bb = 40; continue; }
            66 => { rax = ((r8.wrapping_add(0x4u64)) as u32) as u32 as u64; rcx = ((rbx as u32)) as u32 as u64; rax = (((rax) & (r11))) as u64; rax = ((rax).wrapping_add(rdx)) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 70; continue; } bb = 67; continue; }
            67 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = (((r14 as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 69; continue; } bb = 68; continue; }
            68 => { r14 = 0x1; r14 = ((((r14 as u32)) as i32).wrapping_mul((crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as i32) as u32) as u32 as u64; bb = 72; continue; }
            69 => { rax = 0x64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; r14 = (if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { (rax as u32) } else { (r14 as u32) }) as u32 as u64; bb = 70; continue; }
            70 => { r14 = ((((r14 as u32)) as i32).wrapping_mul((crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as i32) as u32) as u32 as u64; bb = 72; continue; }
            71 => { r14 = ((rdi as u32)) as u32 as u64; bb = 72; continue; }
            72 => { crate::simd_ops::wu32(rsi.wrapping_add(0x1cu64), ((r14 as u32)) as u32); fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; rdi = (if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { (r14 as u32) } else { (rdi as u32) }) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rdi as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; rsp = ((rsp).wrapping_add(0x80u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141618380(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141616940; const P1: u64 = 0xa8; const P2: u64 = 0xc; const P3: u64 = 0xda; const P4: u64 = 0xc8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rcx = ((rcx).wrapping_add(0x58u64)) as u64; rbx = r8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32) as u128; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rbx, ((xmm0 as u32)) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            4 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P3))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 5; continue; }
            5 => { xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(P4))) as u32) as u128; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rbx, ((xmm0 as u32)) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { xmm0 = 0u128; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rbx, ((xmm0 as u32)) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { crate::simd_ops::wu32(rbx, (0x0u32) as u32); rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141618400(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
            9 => { crate::simd_ops::wu64(rax, r8); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), r8); bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rdx = rax; rdx = ((rdx).wrapping_sub(rbx)) as u64; xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(rcx))) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); rax = ((rax).wrapping_add(0x10u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rdi, rbx); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((rbp as u32)) as u32); bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416185e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524340; const P1: u64 = 0x141524330; const P2: u64 = 0x141615e90; const P3: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rdi = rcx; r15 = ((rdx as u32)) as u32 as u64; rcx = 0x1; r15 = (((r15 as u32)).wrapping_shr(1u32)) as u32 as u64; r15 = (((r15 as u32)).wrapping_add((rdx as u32))) as u32 as u64; rsi = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; r15 = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rcx as u32) } else { (r15 as u32) }) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi)) as u64; r15 = (((r15 as u32)).wrapping_add((rdx as u32))) as u32 as u64; rdx = ((r15.wrapping_add(r15.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(3u32)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = rdx; rdx = rcx; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 13; continue; } bb = 2; continue; }
            2 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 5; continue; }
            5 => { fl_l = rax; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 6; continue; }
            6 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rbx = rbp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); r14 = (0xffffffffffffffe0u64) as u64; r14 = ((r14).wrapping_sub(rbp)) as u64; r12 = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { rcx = rbx.wrapping_sub(0x20); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu32(rcx, ((r12 as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), r12); crate::simd_ops::wu8(rbx.wrapping_sub(0x10u64), ((r12 as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_sub(0x8u64), r12); crate::simd_ops::wu64(rbx, r12); bb = 10; continue; }
            10 => { rdx = ((r14.wrapping_add(rbx)) as u64) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x28u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 11; continue; }
            11 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 12; continue; }
            12 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rdi, rbp); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((r15 as u32)) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141618800(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524340; const P1: u64 = 0x141524330; const P2: u64 = 0x141615f90; const P3: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rdi = rcx; r15 = ((rdx as u32)) as u32 as u64; rcx = 0x1; r15 = (((r15 as u32)).wrapping_shr(1u32)) as u32 as u64; r15 = (((r15 as u32)).wrapping_add((rdx as u32))) as u32 as u64; rsi = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; r15 = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rcx as u32) } else { (r15 as u32) }) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi)) as u64; r15 = (((r15 as u32)).wrapping_add((rdx as u32))) as u32 as u64; rdx = ((r15.wrapping_add(r15.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(3u32)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = rdx; rdx = rcx; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 13; continue; } bb = 2; continue; }
            2 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 5; continue; }
            5 => { fl_l = rax; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 6; continue; }
            6 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rbx = rbp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); r14 = (0xffffffffffffffe0u64) as u64; r14 = ((r14).wrapping_sub(rbp)) as u64; r12 = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { rcx = rbx.wrapping_sub(0x20); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rcx, r12); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), r12); crate::simd_ops::wu8(rbx.wrapping_sub(0x10u64), ((r12 as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_sub(0x8u64), r12); crate::simd_ops::wu64(rbx, r12); bb = 10; continue; }
            10 => { rdx = ((r14.wrapping_add(rbx)) as u64) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x28u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 11; continue; }
            11 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 12; continue; }
            12 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rdi, rbp); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((r15 as u32)) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141618910(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415c4520;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rdi = rdx; rsi = rcx; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 17; continue; } bb = 1; continue; }
            1 => { rax = ((rax).wrapping_shl(4u32)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = (crate::simd_ops::ru64(rcx)) as u64; rax = ((rax).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbp); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 6; continue; }
            6 => { rax = ((rax).wrapping_add(0x10u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 7; continue; }
            7 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x11u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x8u64), ((rax as u8)) as u8); rbp = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rax as u32)) as u32); fl_l = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rcx, r14); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), ((r14 as u8)) as u8); xmm0 = (crate::simd_ops::ru128(rbx)) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); rcx = ((crate::simd_ops::ru8(rdi.wrapping_add(0x10u64))) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xfcu8) as u8 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0xfeu8))) as u8 as u64)); fl_l = ((((rax as u8)) | ((rcx as u8)))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rdi.wrapping_add(0x10u64), ((rax as u8)) as u8); bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rdx = (crate::simd_ops::ru64(rsi)) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(0xfffffffffffffff0u64)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { xmm0 = (crate::simd_ops::ru128(rdx)) as u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); bb = 14; continue; }
            14 => { crate::simd_ops::lk_sub_u32(rsi.wrapping_add(0x8u64), (1) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rsi))) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 15; continue; }
            15 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 16; continue; }
            16 => { rcx = ((crate::simd_ops::ru8(rdi.wrapping_add(0x10u64))) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xfcu8) as u8 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0xfeu8))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) | ((rcx as u8)))) as u8 as u64)); crate::simd_ops::wu8(rdi.wrapping_add(0x10u64), ((rax as u8)) as u8); rcx = ((crate::simd_ops::ru8(rsi.wrapping_add(0x10u64))) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xfcu8) as u8 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0xfeu8))) as u8 as u64)); fl_l = ((((rax as u8)) | ((rcx as u8)))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rsi.wrapping_add(0x10u64), ((rax as u8)) as u8); bb = 17; continue; }
            17 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141618a40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x41; const P1: u64 = 0x30; const P2: u64 = 0x10; const P3: u64 = 0x1416144d0;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    crate::simd_ops::lk_add_u8(rcx.wrapping_add(P0), (1) as u8);
    rbx = ((rcx.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::lk_add_u8(rbx.wrapping_add(P2), (0x4u8) as u8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rcx = rbx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdx);
    rdx = rsp.wrapping_add(0x20);
    crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((xmm2 as u32)) as u32);
    crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((xmm3 as u32)) as u32);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u32) as u32 as u64;
    rcx = (((rax as u8)) as u32) as u32 as u64;
    rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x3u8))) as u8 as u64));
    rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_shr(2u32)) as u8 as u64));
    rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_sub(1)) as u8 as u64));
    rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_shl(2u32)) as u8 as u64));
    rcx = (rcx & !0xffu64) | ((((((rcx as u8)) | ((rax as u8)))) as u8 as u64));
    crate::simd_ops::wu8(rbx.wrapping_add(P2), ((rcx as u8)) as u8);
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141618a90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xbc; const P1: u64 = 0x10; const P2: u64 = 0x1415242e0;
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
    if !((((fl_l as u32)) != ((fl_r as u32)))) { r8 = (crate::simd_ops::ru64(rcx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = ((rdi as u32)) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141618ad0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141616590; const P1: u64 = 0x141618910; const P2: u64 = 0x14161a710; const P3: u64 = 0x143930f70; const P4: u64 = 0x1401de410; const P5: u64 = 0x141618a90; const P6: u64 = 0x1416164e0; const P7: u64 = 0x141616430; const P8: u64 = 0x1415fa0e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r12 = 0; fl_l = 0; fl_r = 0; rbp = rdx; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rsi = rcx; rdi = ((r12 as u32)) as u32 as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x18u64))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x20u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x28u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64; fl_r = r12; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rcx = ((rcx).wrapping_add(0x60u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x8u64))) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 23; continue; } bb = 7; continue; }
            7 => { bb = 16; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x68u64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; rdi = rcx.wrapping_add(0x60); rdi = (if (((fl_l as u8)) == ((fl_r as u8))) { r12 } else { rdi }) as u64; rcx = ((rcx).wrapping_add(0x70u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 24; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x18u64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x20u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 24; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x28u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 24; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64; fl_r = r12; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 24; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x10u64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; rcx = rax.wrapping_add(0x8); if (((fl_l as u8)) != ((fl_r as u8))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = r12; bb = 16; continue; }
            16 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 18; continue; }
            18 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = rax.wrapping_add(0x48); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { rdx = rsi.wrapping_add(0x18); bb = 21; continue; }
            21 => { rcx = ((rcx).wrapping_add(0x48u64)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = rbp.wrapping_add(0x8); rcx = rsi.wrapping_add(0x58); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; r8 = rbp.wrapping_add(0x8); rcx = (crate::simd_ops::ru64(P3)) as u64; r9 = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbp; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 23; continue; }
            23 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            24 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x10u64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 26; continue; } bb = 25; continue; }
            25 => { rdi = rax.wrapping_add(0x8); bb = 26; continue; }
            26 => { rcx = rax.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 27; continue; }
            27 => { rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0x18u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 32; continue; } bb = 28; continue; }
            28 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x20u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 32; continue; } bb = 29; continue; }
            29 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x28u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 32; continue; } bb = 30; continue; }
            30 => { fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64; fl_r = r12; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 32; continue; } bb = 31; continue; }
            31 => { rax = ((rax).wrapping_add(0x8u64)) as u64; rcx = r12; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x8u64))) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; rcx = (if (((fl_l as u8)) != ((fl_r as u8))) { rax } else { rcx }) as u64; bb = 16; continue; }
            32 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x10u64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 34; continue; } bb = 33; continue; }
            33 => { rdi = rax.wrapping_add(0x8); bb = 34; continue; }
            34 => { rcx = rax.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 35; continue; }
            35 => { rdx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x20u64))) as u32) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 39; continue; } bb = 36; continue; }
            36 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x28u64))) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 39; continue; } bb = 37; continue; }
            37 => { fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64; fl_r = r12; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 39; continue; } bb = 38; continue; }
            38 => { rax = ((rax).wrapping_add(0x8u64)) as u64; rcx = r12; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x8u64))) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; rcx = (if (((fl_l as u8)) != ((fl_r as u8))) { rax } else { rcx }) as u64; bb = 16; continue; }
            39 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x10u64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 41; continue; } bb = 40; continue; }
            40 => { rdi = rax.wrapping_add(0x8); bb = 41; continue; }
            41 => { rcx = rax.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 42; continue; }
            42 => { rdx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x28u64))) as u32) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 45; continue; } bb = 43; continue; }
            43 => { fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64; fl_r = r12; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 45; continue; } bb = 44; continue; }
            44 => { rax = ((rax).wrapping_add(0x8u64)) as u64; rcx = r12; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x8u64))) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; rcx = (if (((fl_l as u8)) != ((fl_r as u8))) { rax } else { rcx }) as u64; bb = 16; continue; }
            45 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x10u64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 47; continue; } bb = 46; continue; }
            46 => { rdi = rax.wrapping_add(0x8); bb = 47; continue; }
            47 => { rcx = rax.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 48; continue; }
            48 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 49; continue; }
            49 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x10u64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 51; continue; } bb = 50; continue; }
            50 => { rdi = rax.wrapping_add(0x8); bb = 51; continue; }
            51 => { rcx = rax.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; rcx = rax.wrapping_add(0x8); rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r12 } else { rcx }) as u64; bb = 16; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141618d20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415bd1d0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rdx = 0x7;
    return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141618d30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x58; const P1: u64 = 0x10; const P2: u64 = 0x20; const P3: u64 = 0x141616940; const P4: u64 = 0x1415c4b70; const P5: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rdi = rcx; rcx = rdx; rbx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((rdi.wrapping_add(P0)) as u64) as u64; rdx = rsp.wrapping_add(0x20); xmm0 = (crate::simd_ops::ru128(rax)) as u128; xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(P1))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(P2))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm1) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rax)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = ((rcx).wrapping_add(0x48u64)) as u64; rdx = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            3 => { rcx = ((rdi.wrapping_add(P5)) as u64) as u64; rdx = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141619050(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141616220; const P1: u64 = 0x1416185e0; const P2: u64 = 0x14183acc8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = r8; rbp = ((rdx as u32)) as u32 as u64; rdi = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rbx)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rcx = rax; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 20; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 2; continue; }
            2 => { rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdi))) as u64; rax = (0x6666666666666667u64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rsi = rdx; rsi = ((((rsi) as i64).wrapping_shr(4u32) as u64)) as u64; rax = rsi; rax = ((rax).wrapping_shr(63u32)) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 4; continue; }
            4 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 18; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rdi)) as u64; rax = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r14); rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; r14 = ((rcx.wrapping_mul(8)) as u64) as u64; rbx = ((rbx).wrapping_add(r14)) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (((rax as u32)).wrapping_sub((rsi as u32))) as u32 as u64; rcx = rbx.wrapping_add(0x28); rdx = rbx; r8 = ((rax.wrapping_add(rax.wrapping_mul(4))) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(3u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); crate::simd_ops::wu8(rbx.wrapping_add(0x10u64), ((rax as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rax); crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), rax); bb = 9; continue; }
            9 => { crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x8u64), (1) as u32); rcx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = ((rcx).wrapping_add(r14)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 15; continue; }
            10 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 12; continue; }
            12 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 18; continue; } bb = 13; continue; }
            13 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rax as u32)) as u32); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 14; continue; }
            14 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rax); crate::simd_ops::wu8(rcx.wrapping_add(0x10u64), ((rax as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), rax); bb = 15; continue; }
            15 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu32(rcx, ((rbp as u32)) as u32); bb = 17; continue; }
            17 => { rax = rcx; bb = 19; continue; }
            18 => { rax = 0; fl_l = 0; fl_r = 0; bb = 19; continue; }
            19 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 20; continue; }
            20 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141619190(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1416162b0; const P1: u64 = 0x141618800; const P2: u64 = 0x14183acc8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = r8; rbp = rdx; rdi = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rbx)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rcx = rax; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 20; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 2; continue; }
            2 => { rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdi))) as u64; rax = (0x6666666666666667u64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rsi = rdx; rsi = ((((rsi) as i64).wrapping_shr(4u32) as u64)) as u64; rax = rsi; rax = ((rax).wrapping_shr(63u32)) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 4; continue; }
            4 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 18; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rdi)) as u64; rax = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r14); rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; r14 = ((rcx.wrapping_mul(8)) as u64) as u64; rbx = ((rbx).wrapping_add(r14)) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (((rax as u32)).wrapping_sub((rsi as u32))) as u32 as u64; rcx = rbx.wrapping_add(0x28); rdx = rbx; r8 = ((rax.wrapping_add(rax.wrapping_mul(4))) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(3u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); crate::simd_ops::wu8(rbx.wrapping_add(0x10u64), ((rax as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rax); crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), rax); bb = 9; continue; }
            9 => { crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x8u64), (1) as u32); rcx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = ((rcx).wrapping_add(r14)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 15; continue; }
            10 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 12; continue; }
            12 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 18; continue; } bb = 13; continue; }
            13 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rax as u32)) as u32); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 14; continue; }
            14 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rax); crate::simd_ops::wu8(rcx.wrapping_add(0x10u64), ((rax as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), rax); bb = 15; continue; }
            15 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu64(rcx, rbp); bb = 17; continue; }
            17 => { rax = rcx; bb = 19; continue; }
            18 => { rax = 0; fl_l = 0; fl_r = 0; bb = 19; continue; }
            19 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 20; continue; }
            20 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416192e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524340; const P1: u64 = 0x14183acbc; const P2: u64 = 0x1415242e0; const P3: u64 = 0x141615120;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); crate::simd_ops::wu32(rsp.wrapping_add(0x18u64), ((r8 as u32)) as u32); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = ((crate::simd_ops::ru8(rdx.wrapping_add(0x4u64))) as u32) as u32 as u64; rdi = rdx.wrapping_add(0x5); rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x80u64))) as u64; rsi = rcx; fl_l = ((rbx as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r14 = ((rbx.wrapping_add(0x4u64)) as u32) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; r14 = ((((r14 as u32)) & (0xfffffffcu32))) as u32 as u64; r8 = ((r14.wrapping_add(rbx.wrapping_mul(4))) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r15 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 2; continue; }
            2 => { rcx = rax.wrapping_add(0x1); crate::simd_ops::wu8(rax, ((rbx as u8)) as u8); r8 = ((rbx as u32)) as u32 as u64; rdx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdi = ((rdi).wrapping_add(rbx)) as u64; rcx = ((r14 as u32)) as u32 as u64; rbx = ((rbx.wrapping_mul(4)) as u64) as u64; rcx = ((rcx).wrapping_add(r15)) as u64; r8 = rbx; rdx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rdi).wrapping_add(rbx)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x80u64), r15); bb = 5; continue; }
            3 => { rax = rdi; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi.wrapping_add(0x80u64), (0x0u64) as u64); bb = 5; continue; }
            5 => { rax = rdi; bb = 6; continue; }
            6 => { rbx = ((crate::simd_ops::ru8(rdi)) as u32) as u32 as u64; rdi = rax.wrapping_add(0x1); rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x88u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); fl_l = ((rbx as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { r15 = ((rbx.wrapping_add(0x4u64)) as u32) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; r15 = ((((r15 as u32)) & (0xfffffffcu32))) as u32 as u64; r8 = ((r15.wrapping_add(rbx.wrapping_mul(8))) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r14 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = rax.wrapping_add(0x1); crate::simd_ops::wu8(rax, ((rbx as u8)) as u8); r8 = ((rbx as u32)) as u32 as u64; rdx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdi = ((rdi).wrapping_add(rbx)) as u64; rcx = ((r15 as u32)) as u32 as u64; rbx = ((rbx.wrapping_mul(8)) as u64) as u64; rcx = ((rcx).wrapping_add(r14)) as u64; r8 = rbx; rdx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = ((rbx.wrapping_add(rdi)) as u64) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x88u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); bb = 12; continue; }
            9 => { rax = 0x2; bb = 13; continue; }
            10 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi.wrapping_add(0x88u64), (0x0u64) as u64); bb = 12; continue; }
            12 => { r8 = rsi; rdx = rsp.wrapping_add(0x50); rcx = rsp.wrapping_add(0x48); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141619440(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434268d0; const P1: u64 = 0x1434269f8; const P2: u64 = 0x14183ad7c; const P3: u64 = 0x143428740; const P4: u64 = 0x14183ad70; const P5: u64 = 0x142674d68; const P6: u64 = 0x14183ad6a; const P7: u64 = 0x143426ac0; const P8: u64 = 0x143426a48; const P9: u64 = 0x143426d88; const P10: u64 = 0x142674d58;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rsi); xmm2 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm8) as u128); rdi = r8; xmm8 = ((crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32) as u128; rbx = rcx; fl_l = ((xmm8 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; rsi = ((rdx as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm1 = ((crate::simd_ops::ru32(P0)) as u32) as u128; xmm4 = (xmm1) as u128; bb = 3; continue; }
            2 => { xmm1 = ((crate::simd_ops::ru32(r8.wrapping_add(0x4u64))) as u32) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm6) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm7) as u128); xmm7 = 0u128; xmm7 = (xmm7 & !0xffff_ffff_ffff_ffffu128) | ((((rsi) as i64) as f64).to_bits() as u128); xmm1 = crate::simd_ops::vcvtps2pd(xmm1); xmm6 = (xmm7) as u128; xmm6 = (xmm6 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm6 as u64) * f64::from_bits(crate::simd_ops::ru64(P1))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((if f64::from_bits(xmm1 as u64) < f64::from_bits(xmm7 as u64) { f64::from_bits(xmm1 as u64) } else { f64::from_bits(xmm7 as u64) }).to_bits() as u128); xmm0 = (xmm6) as u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) / f64::from_bits(xmm1 as u64)).to_bits() as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm1 = crate::simd_ops::vcvtps2pd(xmm8); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) * f64::from_bits(xmm1 as u64)).to_bits() as u128); xmm0 = (xmm0) ^ (crate::simd_ops::ru128(P3)); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) * f64::from_bits(xmm6 as u64)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) / f64::from_bits(xmm7 as u64)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) * f64::from_bits(crate::simd_ops::ru64(P5))).to_bits() as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm2 = ((crate::simd_ops::ru64(P7)) as u64) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; xmm2 = (xmm2 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm2 as u64) - f64::from_bits(xmm0 as u64)).to_bits() as u128); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; xmm1 = (xmm2) as u128; xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm2 as u64)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) - f64::from_bits(crate::simd_ops::ru64(P8))).to_bits() as u128); xmm0 = crate::simd_ops::vsqrtpd(xmm1); xmm1 = ((crate::simd_ops::ru32(P0)) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) - f64::from_bits(xmm2 as u64)).to_bits() as u128); xmm2 = crate::simd_ops::vcvtpd2ps(xmm0); xmm4 = (xmm2) as u128; xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); bb = 3; continue; }
            3 => { xmm0 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x4u64))) as u32) as u128; xmm3 = 0u128; xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((((rsi) as i64) as f32).to_bits() as u128); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm3 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(xmm3 as u32)).to_bits() as u128); bb = 6; continue; }
            5 => { xmm0 = (xmm1) as u128; bb = 6; continue; }
            6 => { crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), ((xmm0 as u32)) as u32); fl_l = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_r = 0x0; xmm1 = ((crate::simd_ops::ru32(P9)) as u32) as u128; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), ((xmm0 as u32)) as u32); bb = 8; continue; }
            8 => { crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), ((xmm4 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((xmm2 as u32)) as u32); rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x14u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 9; continue; }
            9 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P10))).to_bits() as u128); bb = 13; continue; }
            11 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 14; continue; } bb = 12; continue; }
            12 => { xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); bb = 13; continue; }
            13 => { crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((xmm0 as u32)) as u32); rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; bb = 14; continue; }
            14 => { crate::simd_ops::wu32(rbx.wrapping_add(0x14u64), ((rax as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416195b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x90; const P1: u64 = 0x80; const P2: u64 = 0x143815b74; const P3: u64 = 0x1; const P4: u64 = 0x4;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r8 = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (((r8 as u8)) as u32) as u32 as u64; return rax; }
            2 => { r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rax = (crate::simd_ops::ru32(P2)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x8u64), ((rax as u32)) as u32); fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { r10 = ((crate::simd_ops::ru8(r9)) as u32) as u32 as u64; rcx = ((r8 as u32)) as u32 as u64; bb = 4; continue; }
            4 => { rdx = ((rcx.wrapping_add(P3)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r9))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 4; continue; } bb = 6; continue; }
            6 => { rdx = r8; rax = rsp.wrapping_add(0x8); fl_l = rdx; fl_r = rdx; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rdx } else { rax }) as u64; fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; rax = (rax & !0xffu64) | (((if (((fl_l as u32)) != ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); return rax; }
            7 => { rdx = 0xfffffffc; rax = ((r10.wrapping_add(P4)) as u32) as u32 as u64; rax = (((rax) & (rdx))) as u64; rax = ((rax).wrapping_add(r9)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; fl_l = rdx; fl_r = rdx; rax = rsp.wrapping_add(0x8); rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rdx } else { rax }) as u64; fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; rax = (rax & !0xffu64) | (((if (((fl_l as u32)) != ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141619630(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x80; const P1: u64 = 0x143815bac; const P2: u64 = 0x1; const P3: u64 = 0x4; const P4: u64 = 0x8; const P5: u64 = 0x2c; const P6: u64 = 0xd;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x18u64)) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r11 = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru32(P1)) as u32 as u64; r10 = rdx; crate::simd_ops::wu32(rsp, ((rax as u32)) as u32); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((r11 as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rdx = ((rcx.wrapping_add(P2)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { bb = 2; continue; }
            5 => { rax = ((r9.wrapping_add(P3)) as u32) as u32 as u64; rdx = 0xfffffffc; rax = (((rax) & (rdx))) as u64; fl_l = ((rax).wrapping_add(r8)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; r11 = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; bb = 6; continue; }
            6 => { fl_l = r11; fl_r = r11; rax = rsp; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { r11 } else { rax }) as u64; rcx = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = ((crate::simd_ops::ru8(r10.wrapping_add(P4))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 16; continue; } bb = 7; continue; }
            7 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(r10.wrapping_add(P5))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 15; continue; } bb = 10; continue; }
            10 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x18u64)) as u64; return rax; }
            11 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 12; continue; }
            12 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 15; continue; } bb = 13; continue; }
            13 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x90u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 10; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru8(r10.wrapping_add(P6))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 10; continue; } bb = 15; continue; }
            15 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x18u64)) as u64; return rax; }
            16 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rax = (rax & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); rsp = ((rsp).wrapping_add(0x18u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416196d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xd;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x90u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) > ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
            4 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416196f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x20; const P2: u64 = 0x8; const P3: u64 = 0x10; const P4: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rcx, (0x1u32) as u32); bb = 4; continue; }
            3 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 17; continue; } bb = 4; continue; }
            4 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64; r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdx.wrapping_add(P0), r9); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu32(rdx.wrapping_add(P2), ((r9 as u32)) as u32); bb = 7; continue; }
            6 => { crate::simd_ops::wu32(rdx.wrapping_add(P3), ((r9 as u32)) as u32); crate::simd_ops::wu8(rdx.wrapping_add(P4), (0xffu8) as u8); bb = 7; continue; }
            7 => { r8 = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 17; continue; } bb = 9; continue; }
            9 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 17; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu32(rcx, (0x2u32) as u32); bb = 12; continue; }
            11 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x2; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 17; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu32(rdx.wrapping_add(P3), ((r9 as u32)) as u32); crate::simd_ops::wu8(rdx.wrapping_add(P4), (0xffu8) as u8); crate::simd_ops::wu8(rdx.wrapping_add(P1), (0xffu8) as u8); crate::simd_ops::wu32(rdx.wrapping_add(P2), ((r9 as u32)) as u32); rax = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru64(rdx)) as u64; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu32(rcx, (0x3u32) as u32); crate::simd_ops::wu64(rdx, r9); return rax; }
            15 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 17; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu64(rdx, r9); bb = 17; continue; }
            17 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141619780(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1;
    rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141619790(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = ((rsp).wrapping_sub(0x38u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x20u64))) as u32 as u64; r12 = 0; fl_l = 0; fl_r = 0; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); r14 = ((((rax as u32)) as i32 as i64) as u64) as u64; r13 = rcx; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) < 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); r15 = ((r14.wrapping_add(r14.wrapping_mul(4))) as u64) as u64; fl_l = ((r15).wrapping_shl(3u32)) as u64; r15 = ((fl_l as u64)) as u64; bb = 2; continue; }
            2 => { rbp = (crate::simd_ops::ru64(r13.wrapping_add(0x18u64))) as u64; rbp = ((rbp).wrapping_add(r15)) as u64; rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x20u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; rbx = ((((rax as u32)) as i32 as i64) as u64) as u64; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) < 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { rsi = ((rbx.wrapping_add(rbx.wrapping_mul(4))) as u64) as u64; fl_l = ((rsi).wrapping_shl(3u32)) as u64; rsi = ((fl_l as u64)) as u64; bb = 4; continue; }
            4 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; rdi = ((rdi).wrapping_add(rsi)) as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x10u64), ((r12 as u8)) as u8); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(0x20u64), ((r12 as u32)) as u32); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), r12); crate::simd_ops::wu32(rdi.wrapping_add(0x24u64), ((r12 as u32)) as u32); bb = 6; continue; }
            6 => { rsi = ((rsi).wrapping_sub(0x28u64)) as u64; fl_l = rbx; fl_r = (0x1u64) as u64; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((((fl_l as u64)).wrapping_sub((fl_r as u64))) as i64) >= 0) { bb = 4; continue; } bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbp.wrapping_add(0x20u64), ((r12 as u32)) as u32); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbp.wrapping_add(0x18u64), r12); crate::simd_ops::wu32(rbp.wrapping_add(0x24u64), ((r12 as u32)) as u32); bb = 9; continue; }
            9 => { r15 = ((r15).wrapping_sub(0x28u64)) as u64; crate::simd_ops::wu8(rbp.wrapping_add(0x10u64), ((r12 as u8)) as u8); fl_l = r14; fl_r = (0x1u64) as u64; r14 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((((fl_l as u64)).wrapping_sub((fl_r as u64))) as i64) >= 0) { bb = 2; continue; } bb = 10; continue; }
            10 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(r13.wrapping_add(0x18u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(r13.wrapping_add(0x20u64), ((r12 as u32)) as u32); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(r13.wrapping_add(0x18u64), r12); crate::simd_ops::wu32(r13.wrapping_add(0x24u64), ((r12 as u32)) as u32); bb = 13; continue; }
            13 => { crate::simd_ops::wu8(r13.wrapping_add(0x10u64), ((r12 as u8)) as u8); rsp = ((rsp).wrapping_add(0x38u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416198b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x1415242e0; const P4: u64 = 0x24;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; r14 = 0; fl_l = 0; fl_r = 0; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = ((((rax as u32)) as i32 as i64) as u64) as u64; rbp = rcx; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) < 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); rsi = ((rbx.wrapping_add(rbx.wrapping_mul(4))) as u64) as u64; fl_l = ((rsi).wrapping_shl(3u32)) as u64; rsi = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); bb = 2; continue; }
            2 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; rdi = ((rdi).wrapping_add(rsi)) as u64; crate::simd_ops::wu8(rdi.wrapping_add(P1), ((r14 as u8)) as u8); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(P0), ((r14 as u32)) as u32); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(P2), r14); crate::simd_ops::wu32(rdi.wrapping_add(P4), ((r14 as u32)) as u32); bb = 4; continue; }
            4 => { rsi = ((rsi).wrapping_sub(0x28u64)) as u64; fl_l = rbx; fl_r = (0x1u64) as u64; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((((fl_l as u64)).wrapping_sub((fl_r as u64))) as i64) >= 0) { bb = 2; continue; } bb = 5; continue; }
            5 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbp.wrapping_add(0x20u64), ((r14 as u32)) as u32); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbp.wrapping_add(0x18u64), r14); crate::simd_ops::wu32(rbp.wrapping_add(0x24u64), ((r14 as u32)) as u32); bb = 8; continue; }
            8 => { crate::simd_ops::wu8(rbp.wrapping_add(0x10u64), ((r14 as u8)) as u8); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141619960(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415242e0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x20u64))) as u32 as u64; rbp = rcx; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r11.wrapping_sub(0x10u64), rbx); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; rbx = ((((rax as u32)) as i32 as i64) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rbx); if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) < 0) { bb = 23; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), rsi); rsi = ((rbx.wrapping_add(rbx.wrapping_mul(4))) as u64) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), rdi); crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), r12); crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), r13); fl_l = ((rsi).wrapping_shl(3u32)) as u64; rsi = ((fl_l as u64)) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), r14); crate::simd_ops::wu64(r11.wrapping_sub(0x40u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rsi); bb = 2; continue; }
            2 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; rdi = ((rdi).wrapping_add(rsi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; r12 = ((((rax as u32)) as i32 as i64) as u64) as u64; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) < 0) { bb = 19; continue; } bb = 3; continue; }
            3 => { rbx = ((r12.wrapping_add(r12.wrapping_mul(4))) as u64) as u64; fl_l = ((rbx).wrapping_shl(3u32)) as u64; rbx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rbx); bb = 4; continue; }
            4 => { r13 = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; r13 = ((r13).wrapping_add(rbx)) as u64; rax = (crate::simd_ops::ru32(r13.wrapping_add(0x20u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; r14 = ((((rax as u32)) as i32 as i64) as u64) as u64; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) < 0) { bb = 15; continue; } bb = 5; continue; }
            5 => { r15 = ((r14.wrapping_add(r14.wrapping_mul(4))) as u64) as u64; fl_l = ((r15).wrapping_shl(3u32)) as u64; r15 = ((fl_l as u64)) as u64; bb = 6; continue; }
            6 => { rbp = (crate::simd_ops::ru64(r13.wrapping_add(0x18u64))) as u64; rbp = ((rbp).wrapping_add(r15)) as u64; rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x20u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; rbx = ((((rax as u32)) as i32 as i64) as u64) as u64; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) < 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { rsi = ((rbx.wrapping_add(rbx.wrapping_mul(4))) as u64) as u64; fl_l = ((rsi).wrapping_shl(3u32)) as u64; rsi = ((fl_l as u64)) as u64; bb = 8; continue; }
            8 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; rdi = ((rdi).wrapping_add(rsi)) as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x10u64), (0x0u8) as u8); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu32(rdi.wrapping_add(0x20u64), ((rcx as u32)) as u32); rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), rcx); crate::simd_ops::wu32(rdi.wrapping_add(0x24u64), ((rcx as u32)) as u32); bb = 10; continue; }
            10 => { rsi = ((rsi).wrapping_sub(0x28u64)) as u64; fl_l = rbx; fl_r = (0x1u64) as u64; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((((fl_l as u64)).wrapping_sub((fl_r as u64))) as i64) >= 0) { bb = 8; continue; } bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu32(rbp.wrapping_add(0x20u64), ((rcx as u32)) as u32); rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x18u64), rcx); crate::simd_ops::wu32(rbp.wrapping_add(0x24u64), ((rcx as u32)) as u32); bb = 13; continue; }
            13 => { r15 = ((r15).wrapping_sub(0x28u64)) as u64; crate::simd_ops::wu8(rbp.wrapping_add(0x10u64), (0x0u8) as u8); fl_l = r14; fl_r = (0x1u64) as u64; r14 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((((fl_l as u64)).wrapping_sub((fl_r as u64))) as i64) >= 0) { bb = 6; continue; } bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 15; continue; }
            15 => { rdx = (crate::simd_ops::ru64(r13.wrapping_add(0x18u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu32(r13.wrapping_add(0x20u64), ((rcx as u32)) as u32); rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r13.wrapping_add(0x18u64), rcx); crate::simd_ops::wu32(r13.wrapping_add(0x24u64), ((rcx as u32)) as u32); bb = 17; continue; }
            17 => { rbx = ((rbx).wrapping_sub(0x28u64)) as u64; crate::simd_ops::wu8(r13.wrapping_add(0x10u64), (0x0u8) as u8); fl_l = r12; fl_r = (0x1u64) as u64; r12 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rbx); if (((((fl_l as u64)).wrapping_sub((fl_r as u64))) as i64) >= 0) { bb = 4; continue; } bb = 18; continue; }
            18 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; bb = 19; continue; }
            19 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { crate::simd_ops::wu32(rdi.wrapping_add(0x20u64), ((rcx as u32)) as u32); rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), rcx); crate::simd_ops::wu32(rdi.wrapping_add(0x24u64), ((rcx as u32)) as u32); bb = 21; continue; }
            21 => { rbx = ((rbx).wrapping_sub(1)) as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x10u64), (0x0u8) as u8); rsi = ((rsi).wrapping_sub(0x28u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rsi); fl_l = rbx; fl_r = rbx; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) >= 0) { bb = 2; continue; } bb = 22; continue; }
            22 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; bb = 23; continue; }
            23 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 24; continue; }
            24 => { crate::simd_ops::wu32(rbp.wrapping_add(0x20u64), ((rcx as u32)) as u32); rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x18u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x24u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x10u64), ((rax as u8)) as u8); rsp = ((rsp).wrapping_add(0x70u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            25 => { crate::simd_ops::wu8(rbp.wrapping_add(0x10u64), (0x0u8) as u8); rsp = ((rsp).wrapping_add(0x70u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141619b80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415242e0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x20u64))) as u32 as u64; rdi = rcx; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), r12); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; r12 = ((((rax as u32)) as i32 as i64) as u64) as u64; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) < 0) { bb = 17; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(r11.wrapping_add(0x18u64), rbx); rbx = ((r12.wrapping_add(r12.wrapping_mul(4))) as u64) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x10u64), rbp); crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), rsi); crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), r13); fl_l = ((rbx).wrapping_shl(3u32)) as u64; rbx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), r14); crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rbx); bb = 2; continue; }
            2 => { r13 = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; r13 = ((r13).wrapping_add(rbx)) as u64; rax = (crate::simd_ops::ru32(r13.wrapping_add(0x20u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; r14 = ((((rax as u32)) as i32 as i64) as u64) as u64; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) < 0) { bb = 13; continue; } bb = 3; continue; }
            3 => { r15 = ((r14.wrapping_add(r14.wrapping_mul(4))) as u64) as u64; fl_l = ((r15).wrapping_shl(3u32)) as u64; r15 = ((fl_l as u64)) as u64; bb = 4; continue; }
            4 => { rbp = (crate::simd_ops::ru64(r13.wrapping_add(0x18u64))) as u64; rbp = ((rbp).wrapping_add(r15)) as u64; rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x20u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; rbx = ((((rax as u32)) as i32 as i64) as u64) as u64; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) < 0) { bb = 9; continue; } bb = 5; continue; }
            5 => { rsi = ((rbx.wrapping_add(rbx.wrapping_mul(4))) as u64) as u64; fl_l = ((rsi).wrapping_shl(3u32)) as u64; rsi = ((fl_l as u64)) as u64; bb = 6; continue; }
            6 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; rdi = ((rdi).wrapping_add(rsi)) as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x10u64), (0x0u8) as u8); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu32(rdi.wrapping_add(0x20u64), ((rcx as u32)) as u32); rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), rcx); crate::simd_ops::wu32(rdi.wrapping_add(0x24u64), ((rcx as u32)) as u32); bb = 8; continue; }
            8 => { rsi = ((rsi).wrapping_sub(0x28u64)) as u64; fl_l = rbx; fl_r = (0x1u64) as u64; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((((fl_l as u64)).wrapping_sub((fl_r as u64))) as i64) >= 0) { bb = 6; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu32(rbp.wrapping_add(0x20u64), ((rcx as u32)) as u32); rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x18u64), rcx); crate::simd_ops::wu32(rbp.wrapping_add(0x24u64), ((rcx as u32)) as u32); bb = 11; continue; }
            11 => { r15 = ((r15).wrapping_sub(0x28u64)) as u64; crate::simd_ops::wu8(rbp.wrapping_add(0x10u64), (0x0u8) as u8); fl_l = r14; fl_r = (0x1u64) as u64; r14 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((((fl_l as u64)).wrapping_sub((fl_r as u64))) as i64) >= 0) { bb = 4; continue; } bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; bb = 13; continue; }
            13 => { rdx = (crate::simd_ops::ru64(r13.wrapping_add(0x18u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu32(r13.wrapping_add(0x20u64), ((rcx as u32)) as u32); rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r13.wrapping_add(0x18u64), rcx); crate::simd_ops::wu32(r13.wrapping_add(0x24u64), ((rcx as u32)) as u32); bb = 15; continue; }
            15 => { rbx = ((rbx).wrapping_sub(0x28u64)) as u64; crate::simd_ops::wu8(r13.wrapping_add(0x10u64), (0x0u8) as u8); fl_l = r12; fl_r = (0x1u64) as u64; r12 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rbx); if (((((fl_l as u64)).wrapping_sub((fl_r as u64))) as i64) >= 0) { bb = 2; continue; } bb = 16; continue; }
            16 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; bb = 17; continue; }
            17 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { crate::simd_ops::wu32(rdi.wrapping_add(0x20u64), ((rcx as u32)) as u32); rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), rax); crate::simd_ops::wu32(rdi.wrapping_add(0x24u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rdi.wrapping_add(0x10u64), ((rax as u8)) as u8); rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            19 => { crate::simd_ops::wu8(rdi.wrapping_add(0x10u64), (0x0u8) as u8); rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141619d00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143815b70; const P1: u64 = 0x141619190; const P2: u64 = 0x141619050; const P3: u64 = 0x141618f00; const P4: u64 = 0x141618db0; const P5: u64 = 0x141618400; const P6: u64 = 0x14183acc8; const P7: u64 = 0x141625a40; const P8: u64 = 0x141624210; const P9: u64 = 0x141618a90; const P10: u64 = 0x1415c2360; const P11: u64 = 0x1415c2230; const P12: u64 = 0x1415c2af0; const P13: u64 = 0x14161a710; const P14: u64 = 0x141614f90; const P15: u64 = 0x141624330;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 29>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x7); rsp = ((rsp).wrapping_sub(0xa8u64)) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x77u64))) as u64; r12 = r8; rdi = 0; fl_l = 0; fl_r = 0; r10 = rdx; r13 = rcx; r15 = 0x2; crate::simd_ops::wu8(rax, (0x0u8) as u8); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64; crate::simd_ops::wu8(rax, (0x0u8) as u8); r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x80u64))) as u64; rax = (crate::simd_ops::ru32(P0)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x41u64), ((rax as u32)) as u32); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((rdi as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 3; continue; }
            3 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rdx = rdi; bb = 5; continue; }
            5 => { rcx = ((crate::simd_ops::ru8(r12.wrapping_add(0x8u64))) as u32) as u32 as u64; rax = rbp.wrapping_sub(0x41); r8 = ((crate::simd_ops::ru8(r12.wrapping_add(0x9u64))) as u32) as u32 as u64; fl_l = rdx; fl_r = rdx; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rdx } else { rax }) as u64; rbx = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = ((rcx.wrapping_add(0x80u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru32(r10.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; rbx = (if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { crate::simd_ops::ru32(r10.wrapping_add(0x10u64)) } else { (rbx as u32) }) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x7du64), ((rbx as u32)) as u32); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xcfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xb0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = ((crate::simd_ops::ru8(r12.wrapping_add(0xcu64))) as u32) as u32 as u64; bb = 10; continue; }
            8 => { rdx = 0xfffffffc; rax = ((r9.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = (((rax) & (rdx))) as u64; fl_l = ((rax).wrapping_add(r8)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; bb = 5; continue; }
            9 => { rax = (rax & !0xffu64) | (((0xffu8) as u8 as u64)); bb = 10; continue; }
            10 => { r10 = (crate::simd_ops::ru64(r12)) as u64; r11 = (crate::simd_ops::ru64(r12.wrapping_add(0x30u64))) as u64; rcx = (crate::simd_ops::ru32(r12.wrapping_add(0x24u64))) as u32 as u64; rdx = (crate::simd_ops::ru32(r12.wrapping_add(0x20u64))) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x71u64), r10); crate::simd_ops::wu64(rbp.wrapping_sub(0x49u64), r11); crate::simd_ops::wu32(rbp.wrapping_sub(0x69u64), ((rcx as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x61u64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_sub(0x59u64), ((r8 as u8)) as u8); crate::simd_ops::wu8(rbp.wrapping_sub(0x51u64), ((rax as u8)) as u8); fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 13; continue; } bb = 11; continue; }
            11 => { fl_l = r11; fl_r = r11; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 27; continue; } bb = 12; continue; }
            12 => { rbx = 0x1; crate::simd_ops::wu32(rbp.wrapping_sub(0x7du64), ((rbx as u32)) as u32); bb = 14; continue; }
            13 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 27; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x49u64), rdi); r11 = rdi; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x69u64), ((rcx as u32)) as u32); bb = 17; continue; }
            16 => { rdx = ((rdi as u32)) as u32 as u64; r8 = (r8 & !0xffu64) | (((0xffu8) as u8 as u64)); crate::simd_ops::wu32(rbp.wrapping_sub(0x61u64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_sub(0x59u64), ((r8 as u8)) as u8); bb = 17; continue; }
            17 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 21; continue; } bb = 18; continue; }
            18 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 27; continue; } bb = 19; continue; }
            19 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 27; continue; } bb = 20; continue; }
            20 => { rdx = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x7du64), ((r15 as u32)) as u32); rcx = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x61u64), ((rdx as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x69u64), ((rcx as u32)) as u32); r8 = (((rax as u8)) as u32) as u32 as u64; crate::simd_ops::wu8(rbp.wrapping_sub(0x59u64), ((rax as u8)) as u8); crate::simd_ops::wu8(rbp.wrapping_sub(0x51u64), ((rax as u8)) as u8); bb = 23; continue; }
            21 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 27; continue; } bb = 22; continue; }
            22 => { r8 = (r8 & !0xffu64) | (((0xffu8) as u8 as u64)); crate::simd_ops::wu32(rbp.wrapping_sub(0x7du64), ((rbx as u32)) as u32); rax = (((r8 as u8)) as u32) as u32 as u64; crate::simd_ops::wu8(rbp.wrapping_sub(0x59u64), ((r8 as u8)) as u8); rdx = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu8(rbp.wrapping_sub(0x51u64), ((rax as u8)) as u8); rcx = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x61u64), ((rdx as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x69u64), ((rcx as u32)) as u32); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 25; continue; } bb = 23; continue; }
            23 => { fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 27; continue; } bb = 24; continue; }
            24 => { crate::simd_ops::wu32(rbp.wrapping_sub(0x7du64), (0x3u32) as u32); bb = 26; continue; }
            25 => { crate::simd_ops::wu32(rbp.wrapping_sub(0x7du64), ((rbx as u32)) as u32); fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x3; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 27; continue; } bb = 26; continue; }
            26 => { r10 = rdi; crate::simd_ops::wu64(rbp.wrapping_sub(0x71u64), rdi); bb = 27; continue; }
            27 => { crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((rdi as u8)) as u8); rsi = r13.wrapping_add(0x58); crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), rsi); r14 = rdi; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 37; continue; } bb = 28; continue; }
            28 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 37; continue; } bb = 29; continue; }
            29 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 37; continue; } bb = 30; continue; }
            30 => { fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 37; continue; } bb = 31; continue; }
            31 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 37; continue; } bb = 32; continue; }
            32 => { fl_l = r11; fl_r = r11; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 37; continue; } bb = 33; continue; }
            33 => { fl_l = (crate::simd_ops::ru8(r13.wrapping_add(0x68u64))) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; rbx = r13.wrapping_add(0x60); if (((fl_l as u8)) != ((fl_r as u8))) { bb = 36; continue; } bb = 34; continue; }
            34 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 36; continue; } bb = 35; continue; }
            35 => { crate::simd_ops::wu64(rbx, rdi); bb = 36; continue; }
            36 => { crate::simd_ops::wu8(rbx.wrapping_add(0x8u64), (0x1u8) as u8); bb = 102; continue; }
            37 => { fl_l = (crate::simd_ops::ru8(r13.wrapping_add(0x68u64))) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; r14 = r13.wrapping_add(0x60); rcx = r13.wrapping_add(0x70); rdx = r10; r8 = rsp.wrapping_add(0x20); r14 = (if (((fl_l as u8)) == ((fl_r as u8))) { rdi } else { r14 }) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 148; continue; } bb = 38; continue; }
            38 => { rdx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x69u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 43; continue; } bb = 39; continue; }
            39 => { fl_l = (crate::simd_ops::ru32(rbp.wrapping_sub(0x61u64))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 43; continue; } bb = 40; continue; }
            40 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_sub(0x59u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 43; continue; } bb = 41; continue; }
            41 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_sub(0x51u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 43; continue; } bb = 42; continue; }
            42 => { fl_l = (crate::simd_ops::ru64(rbp.wrapping_sub(0x49u64))) as u64; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 64; continue; } bb = 43; continue; }
            43 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x79u64), r14); fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x10u64))) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 45; continue; } bb = 44; continue; }
            44 => { r14 = rax.wrapping_add(0x8); crate::simd_ops::wu64(rbp.wrapping_sub(0x79u64), r14); bb = 45; continue; }
            45 => { rcx = rax.wrapping_add(0x18); r8 = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 47; continue; } bb = 46; continue; }
            46 => { rbx = rdi; bb = 101; continue; }
            47 => { rdx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x61u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 51; continue; } bb = 48; continue; }
            48 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_sub(0x59u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 51; continue; } bb = 49; continue; }
            49 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_sub(0x51u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 51; continue; } bb = 50; continue; }
            50 => { fl_l = (crate::simd_ops::ru64(rbp.wrapping_sub(0x49u64))) as u64; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 64; continue; } bb = 51; continue; }
            51 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x10u64))) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 53; continue; } bb = 52; continue; }
            52 => { r14 = rax.wrapping_add(0x8); crate::simd_ops::wu64(rbp.wrapping_sub(0x79u64), r14); bb = 53; continue; }
            53 => { rcx = rax.wrapping_add(0x18); r8 = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 55; continue; } bb = 54; continue; }
            54 => { rbx = rdi; bb = 101; continue; }
            55 => { rdx = ((crate::simd_ops::ru8(rbp.wrapping_sub(0x59u64))) as u32) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 58; continue; } bb = 56; continue; }
            56 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_sub(0x51u64))) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 58; continue; } bb = 57; continue; }
            57 => { fl_l = (crate::simd_ops::ru64(rbp.wrapping_sub(0x49u64))) as u64; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 64; continue; } bb = 58; continue; }
            58 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x10u64))) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 60; continue; } bb = 59; continue; }
            59 => { r14 = rax.wrapping_add(0x8); crate::simd_ops::wu64(rbp.wrapping_sub(0x79u64), r14); bb = 60; continue; }
            60 => { rcx = rax.wrapping_add(0x18); r8 = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 62; continue; } bb = 61; continue; }
            61 => { rbx = rdi; bb = 101; continue; }
            62 => { rdx = ((crate::simd_ops::ru8(rbp.wrapping_sub(0x51u64))) as u32) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 67; continue; } bb = 63; continue; }
            63 => { fl_l = (crate::simd_ops::ru64(rbp.wrapping_sub(0x49u64))) as u64; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 67; continue; } bb = 64; continue; }
            64 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x10u64))) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; rbx = rax.wrapping_add(0x8); if (((fl_l as u8)) != ((fl_r as u8))) { bb = 66; continue; } bb = 65; continue; }
            65 => { crate::simd_ops::wu64(rbx, rdi); bb = 66; continue; }
            66 => { crate::simd_ops::wu8(rbx.wrapping_add(0x8u64), (0x1u8) as u8); bb = 101; continue; }
            67 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x10u64))) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 69; continue; } bb = 68; continue; }
            68 => { r14 = rax.wrapping_add(0x8); crate::simd_ops::wu64(rbp.wrapping_sub(0x79u64), r14); bb = 69; continue; }
            69 => { rcx = rax.wrapping_add(0x18); r8 = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 71; continue; } bb = 70; continue; }
            70 => { rbx = rdi; bb = 101; continue; }
            71 => { r15 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x49u64))) as u64; fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 97; continue; } bb = 72; continue; }
            72 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x10u64))) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 74; continue; } bb = 73; continue; }
            73 => { fl_l = ((rax).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x79u64), rax); bb = 74; continue; }
            74 => { r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32 as u64; r9 = ((rdi as u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 78; continue; } bb = 75; continue; }
            75 => { r11 = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; bb = 76; continue; }
            76 => { rdx = ((r8 as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((r9 as u32))) as u32 as u64; rcx = ((((rdx as u32)) as i32 as i64) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(r11)) as u64; fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = r15; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 96; continue; } bb = 77; continue; }
            77 => { rax = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { (r9 as u32) } else { (rax as u32) }) as u32 as u64; r9 = ((rax as u32)) as u32 as u64; rax = ((r8.wrapping_sub(0x1u64)) as u32) as u32 as u64; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { (r8 as u32) } else { (rax as u32) }) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; fl_l = (((r8 as u32)).wrapping_shr(1u32)) as u32 as u64; r8 = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 76; continue; } bb = 78; continue; }
            78 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; r14 = (crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x24u64))) as u32 as u64; rsi = ((((r9 as u32)) as i32 as i64) as u64) as u64; rsi = ((rsi).wrapping_shl(4u32)) as u64; fl_l = ((rcx).wrapping_add(rsi)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 87; continue; } bb = 79; continue; }
            79 => { rsi = ((((rsi) as i64).wrapping_shr(4u32) as u64)) as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 81; continue; } bb = 80; continue; }
            80 => { rcx = rbx.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 94; continue; } bb = 81; continue; }
            81 => { fl_l = ((r14 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x24u64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 94; continue; } bb = 82; continue; }
            82 => { r14 = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32 as u64; r13 = ((rsi as u32)) as u32 as u64; r13 = ((r13).wrapping_shl(4u32)) as u64; r14 = ((r14).wrapping_add(r13)) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 84; continue; } bb = 83; continue; }
            83 => { rax = (((rax as u32)).wrapping_sub((rsi as u32))) as u32 as u64; rcx = r14.wrapping_add(0x10); rax = (((rax as u32)).wrapping_shl(4u32)) as u32 as u64; rdx = r14; r8 = ((rax as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 84; continue; }
            84 => { fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 86; continue; } bb = 85; continue; }
            85 => { crate::simd_ops::wu64(r14, rdi); crate::simd_ops::wu64(r14.wrapping_add(0x8u64), rdi); bb = 86; continue; }
            86 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x20u64), (1) as u32); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = ((rcx).wrapping_add(r13)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; r13 = (crate::simd_ops::ru64(rbp.wrapping_add(0x4fu64))) as u64; bb = 92; continue; }
            87 => { fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 89; continue; } bb = 88; continue; }
            88 => { rcx = rbx.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 94; continue; } bb = 89; continue; }
            89 => { fl_l = ((r14 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x24u64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 94; continue; } bb = 90; continue; }
            90 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x20u64), ((rax as u32)) as u32); fl_l = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x18u64)))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 92; continue; } bb = 91; continue; }
            91 => { crate::simd_ops::wu64(rcx, rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rdi); bb = 92; continue; }
            92 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 95; continue; } bb = 93; continue; }
            93 => { crate::simd_ops::wu64(rcx, r15); bb = 95; continue; }
            94 => { rcx = rdi; bb = 95; continue; }
            95 => { rsi = r13.wrapping_add(0x58); bb = 96; continue; }
            96 => { r14 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x79u64))) as u64; rbx = rcx.wrapping_add(0x8); r12 = (crate::simd_ops::ru64(rbp.wrapping_add(0x5fu64))) as u64; fl_l = rcx; fl_r = rcx; rbx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rdi } else { rbx }) as u64; bb = 100; continue; }
            97 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x8u64))) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 99; continue; } bb = 98; continue; }
            98 => { crate::simd_ops::wu64(rbx, rdi); bb = 99; continue; }
            99 => { crate::simd_ops::wu8(rbx.wrapping_add(0x8u64), (0x1u8) as u8); bb = 100; continue; }
            100 => { r15 = 0x2; bb = 101; continue; }
            101 => { rax = ((crate::simd_ops::ru8(rbp.wrapping_sub(0x51u64))) as u32) as u32 as u64; bb = 102; continue; }
            102 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 148; continue; } bb = 103; continue; }
            103 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x77u64))) as u64; rsi = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rcx, ((rdi as u8)) as u8); rcx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x7du64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 108; continue; } bb = 104; continue; }
            104 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 108; continue; } bb = 105; continue; }
            105 => { fl_l = (crate::simd_ops::ru8(r12.wrapping_add(0x8u64))) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 108; continue; } bb = 106; continue; }
            106 => { rax = (crate::simd_ops::ru64(r13)) as u64; rdx = r12; rcx = r13; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 136; continue; } bb = 107; continue; }
            107 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x7du64))) as u32 as u64; bb = 109; continue; }
            108 => { fl_l = (crate::simd_ops::ru32(r12.wrapping_add(0x2cu64))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 136; continue; } bb = 109; continue; }
            109 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 115; continue; } bb = 110; continue; }
            110 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0xc0u64))) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 115; continue; } bb = 111; continue; }
            111 => { fl_l = (crate::simd_ops::ru8(r13.wrapping_add(0x94u64))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 114; continue; } bb = 112; continue; }
            112 => { rax = (crate::simd_ops::ru32(r12.wrapping_add(0x2cu64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xfffffffd; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 114; continue; } bb = 113; continue; }
            113 => { r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x57u64))) as u64; rdx = r12; rcx = rsi; r8 = (crate::simd_ops::ru64(r8)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64; crate::simd_ops::wu8(rax, (0x1u8) as u8); bb = 114; continue; }
            114 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdi = rsi; bb = 120; continue; }
            115 => { fl_l = (crate::simd_ops::ru32(r12.wrapping_add(0x2cu64))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 117; continue; } bb = 116; continue; }
            116 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 136; continue; } bb = 117; continue; }
            117 => { r8 = ((rcx as u32)) as u32 as u64; r9 = rbp.wrapping_sub(0x71); rcx = (crate::simd_ops::ru32(r13.wrapping_add(0x90u64))) as u32 as u64; rdx = r13; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 119; continue; } bb = 118; continue; }
            118 => { r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x57u64))) as u64; rdx = r12; rcx = rax; r8 = (crate::simd_ops::ru64(r8)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64; crate::simd_ops::wu8(rax, (0x1u8) as u8); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x77u64))) as u64; crate::simd_ops::wu8(rax, (0x1u8) as u8); bb = 119; continue; }
            119 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdi = rsi; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 137; continue; } bb = 120; continue; }
            120 => { fl_l = rax; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 124; continue; } bb = 121; continue; }
            121 => { crate::simd_ops::lk_add_u32(rsi.wrapping_add(0xbcu64), (1) as u32); rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 123; continue; } bb = 122; continue; }
            122 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 123; continue; }
            123 => { crate::simd_ops::wu64(rbx, rsi); bb = 124; continue; }
            124 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 134; continue; } bb = 125; continue; }
            125 => { fl_l = (crate::simd_ops::ru64(rdx)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 127; continue; } bb = 126; continue; }
            126 => { rcx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; bb = 127; continue; }
            127 => { rcx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 129; continue; } bb = 128; continue; }
            128 => { r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x57u64))) as u64; r8 = r12; rdx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; r15 = ((rax as u32)) as u32 as u64; bb = 129; continue; }
            129 => { fl_l = ((r15 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 135; continue; } bb = 130; continue; }
            130 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 135; continue; } bb = 131; continue; }
            131 => { rcx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 133; continue; } bb = 132; continue; }
            132 => { rdx = ((rax as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x6fu64))) as u64; crate::simd_ops::wu64(rax, rdi); bb = 140; continue; }
            133 => { crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x6fu64))) as u64; crate::simd_ops::wu64(rax, rdi); bb = 140; continue; }
            134 => { r15 = 0x1; bb = 135; continue; }
            135 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x6fu64))) as u64; crate::simd_ops::wu64(rax, rdi); bb = 140; continue; }
            136 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rsi = rdi; bb = 137; continue; }
            137 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 139; continue; } bb = 138; continue; }
            138 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x39u64))) as u64; rdx = rbp.wrapping_sub(0x71); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x6fu64))) as u64; crate::simd_ops::wu64(rax, rdi); rax = ((r15 as u32)) as u32 as u64; bb = 149; continue; }
            139 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x6fu64))) as u64; crate::simd_ops::wu64(rax, rsi); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 147; continue; } bb = 140; continue; }
            140 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x77u64))) as u64; fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 145; continue; } bb = 141; continue; }
            141 => { fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 143; continue; } bb = 142; continue; }
            142 => { rax = (crate::simd_ops::ru64(r14)) as u64; rcx = rax.wrapping_add(0x48); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 144; continue; } bb = 143; continue; }
            143 => { rcx = r13.wrapping_add(0x18); bb = 144; continue; }
            144 => { xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x71u64))) as u128; rdx = (crate::simd_ops::ru64(rbx)) as u64; r8 = rbp.wrapping_sub(0x31); xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x61u64))) as u128; rdx = ((rdx).wrapping_add(0x48u64)) as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x31u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x51u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x21u64), (xmm1) as u128); crate::simd_ops::wu128(rbp.wrapping_sub(0x11u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 145; continue; }
            145 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64; fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 147; continue; } bb = 146; continue; }
            146 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P15, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 147; continue; }
            147 => { rax = ((r15 as u32)) as u32 as u64; bb = 149; continue; }
            148 => { rdx = rbp.wrapping_sub(0x71); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0x34; bb = 149; continue; }
            149 => { rsp = ((rsp).wrapping_add(0xa8u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14161a3b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141616430; const P1: u64 = 0x1415fa0e0; const P2: u64 = 0x14183acc8; const P3: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = rdx; rsi = rcx; rdx = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 22; continue; } bb = 2; continue; }
            2 => { rcx = ((rcx).wrapping_add(0x18u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r12); r12 = 0; fl_l = 0; fl_r = 0; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 4; continue; }
            4 => { rcx = rax.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; rdx = (((rdx as u32)).wrapping_sub(1)) as u32 as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x18u64)))) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = ((rdx).wrapping_sub(rax)) as u64; rcx = rax; rdx = ((((rdx as u32)) & (0xfffffff0u32))) as u32 as u64; r8 = ((rdx as u32)) as u32 as u64; rdx = rax.wrapping_add(0x10); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::lk_sub_u32(rdi.wrapping_add(0x20u64), (1) as u32)) as u32 as u64; rbp = (rbp & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 9; continue; }
            8 => { rbp = (rbp & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 11; continue; }
            11 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(0x20u64), ((r12 as u32)) as u32); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), r12); crate::simd_ops::wu32(rdi.wrapping_add(0x24u64), ((r12 as u32)) as u32); bb = 13; continue; }
            12 => { rbp = ((crate::simd_ops::ru8(rax.wrapping_add(0x10u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rax.wrapping_add(0x10u64), ((r12 as u8)) as u8); bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x10u64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 21; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 21; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu8(rdi.wrapping_add(0x10u64), ((r12 as u8)) as u8); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(0x20u64), ((r12 as u32)) as u32); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), r12); crate::simd_ops::wu32(rdi.wrapping_add(0x24u64), ((r12 as u32)) as u32); bb = 17; continue; }
            17 => { rdx = ((crate::simd_ops::ru8(r14)) as u32) as u32 as u64; rcx = rsi.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r9 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x20u64))) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub(1)) as u32 as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; r8 = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 20; continue; } bb = 19; continue; }
            19 => { r8 = ((r8).wrapping_sub(rax)) as u64; rax = (0x6666666666666667u64) as u64; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(4u32) as u64)) as u64; rcx = rdx; rcx = ((rcx).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rcx = r9; r8 = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(3u32)) as u32 as u64; rdx = r9.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::lk_sub_u32(rsi.wrapping_add(0x20u64), (1) as u32)) as u32 as u64; bb = 21; continue; }
            21 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = (((rbp as u8)) as u32) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 23; continue; }
            22 => { rax = ((crate::simd_ops::ru8(rsi.wrapping_add(0x10u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x10u64), (0x0u8) as u8); bb = 23; continue; }
            23 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14161a540(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1416164e0; const P1: u64 = 0x141616430; const P2: u64 = 0x14161a3b0; const P3: u64 = 0x1416198b0; const P4: u64 = 0x14183acc8; const P5: u64 = 0x141619790;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r15 = rdx; rsi = rcx; rdx = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(r15.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(r15.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 24; continue; } bb = 4; continue; }
            4 => { rcx = ((rcx).wrapping_add(0x18u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 5; continue; }
            5 => { rdx = ((crate::simd_ops::ru8(r15.wrapping_add(0x8u64))) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r14); fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(r15.wrapping_add(0x10u64))) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 8; continue; }
            8 => { rcx = rax.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 9; continue; }
            9 => { rdx = r15.wrapping_add(0x10); rcx = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r14 = (((rax as u8)) as u32) as u32 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 16; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x20u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 16; continue; } bb = 11; continue; }
            11 => { rcx = rbp; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = ((crate::simd_ops::ru8(r15.wrapping_add(0x8u64))) as u32) as u32 as u64; rcx = rdi.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r9 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; rbx = (0x6666666666666667u64) as u64; rcx = (((rcx as u32)).wrapping_sub(1)) as u32 as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; r8 = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { r8 = ((r8).wrapping_sub(rax)) as u64; rcx = r9; rax = rbx; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(4u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; r8 = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(3u32)) as u32 as u64; rdx = r9.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::lk_sub_u32(rdi.wrapping_add(0x20u64), (1) as u32)) as u32 as u64; bb = 17; continue; }
            15 => { r14 = ((crate::simd_ops::ru8(rdi.wrapping_add(0x10u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x10u64), (0x0u8) as u8); bb = 16; continue; }
            16 => { rbx = (0x6666666666666667u64) as u64; bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 23; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 23; continue; } bb = 19; continue; }
            19 => { rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(r15)) as u32 as u64; rcx = rsi.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r9 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 20; continue; }
            20 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x20u64))) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub(1)) as u32 as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; r8 = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 22; continue; } bb = 21; continue; }
            21 => { r8 = ((r8).wrapping_sub(rax)) as u64; rax = rbx; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(4u32) as u64)) as u64; rcx = rdx; rcx = ((rcx).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rcx = r9; r8 = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(3u32)) as u32 as u64; rdx = r9.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 22; continue; }
            22 => { fl_l = (crate::simd_ops::lk_sub_u32(rsi.wrapping_add(0x20u64), (1) as u32)) as u32 as u64; bb = 23; continue; }
            23 => { rax = (((r14 as u8)) as u32) as u32 as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            24 => { rax = ((crate::simd_ops::ru8(rsi.wrapping_add(0x10u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x10u64), (0x0u8) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14161a710(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141616590; const P1: u64 = 0x1416164e0; const P2: u64 = 0x14161a540; const P3: u64 = 0x141619b80; const P4: u64 = 0x14183acc8; const P5: u64 = 0x141619960;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r15 = rdx; rsi = rcx; rdx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(r15.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(r15.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(r15.wrapping_add(0x18u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(r15.wrapping_add(0x20u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru64(r15.wrapping_add(0x28u64))) as u64; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 28; continue; } bb = 6; continue; }
            6 => { rcx = ((rcx).wrapping_add(0x18u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru32(r15.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r14); fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(r15.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(r15.wrapping_add(0x18u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 12; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru8(r15.wrapping_add(0x20u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru64(r15.wrapping_add(0x28u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 12; continue; }
            12 => { rcx = rax.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r14 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 13; continue; }
            13 => { rdx = r15.wrapping_add(0x10); rcx = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbp = (((rax as u8)) as u32) as u32 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 20; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru32(r14.wrapping_add(0x20u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 20; continue; } bb = 15; continue; }
            15 => { rcx = r14; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(r15.wrapping_add(0x8u64))) as u32 as u64; rcx = rdi.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r9 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; rbx = (0x6666666666666667u64) as u64; rcx = (((rcx as u32)).wrapping_sub(1)) as u32 as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; r8 = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 18; continue; } bb = 17; continue; }
            17 => { r8 = ((r8).wrapping_sub(rax)) as u64; rcx = r9; rax = rbx; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(4u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; r8 = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(3u32)) as u32 as u64; rdx = r9.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::lk_sub_u32(rdi.wrapping_add(0x20u64), (1) as u32)) as u32 as u64; bb = 21; continue; }
            19 => { rbp = ((crate::simd_ops::ru8(rdi.wrapping_add(0x10u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x10u64), (0x0u8) as u8); bb = 20; continue; }
            20 => { rbx = (0x6666666666666667u64) as u64; bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 27; continue; } bb = 22; continue; }
            22 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 27; continue; } bb = 23; continue; }
            23 => { rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(r15)) as u64; rcx = rsi.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r9 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 24; continue; }
            24 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x20u64))) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub(1)) as u32 as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; r8 = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 26; continue; } bb = 25; continue; }
            25 => { r8 = ((r8).wrapping_sub(rax)) as u64; rax = rbx; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(4u32) as u64)) as u64; rcx = rdx; rcx = ((rcx).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rcx = r9; r8 = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(3u32)) as u32 as u64; rdx = r9.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            26 => { fl_l = (crate::simd_ops::lk_sub_u32(rsi.wrapping_add(0x20u64), (1) as u32)) as u32 as u64; bb = 27; continue; }
            27 => { rax = (((rbp as u8)) as u32) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            28 => { rax = ((crate::simd_ops::ru8(rsi.wrapping_add(0x10u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x10u64), (0x0u8) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14161a8f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x1415242e0; const P2: u64 = 0x1415bd160; const P3: u64 = 0x141834d30;
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
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rcx = rbx;
    crate::simd_ops::wu64(rbx.wrapping_add(P0), (0x0u64) as u64);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x28; rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rax = rbx;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14161a940(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xffffffffffffffda; const P1: u64 = 0x141524330; const P2: u64 = 0x1415bd140; const P3: u64 = 0x14268f778; const P4: u64 = 0x20; const P5: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdx = 0x28;
    rcx = ((rdx.wrapping_add(P0)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rbx = rax;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = P3; crate::simd_ops::wu64(rbx.wrapping_add(P4), (0x0u64) as u64); crate::simd_ops::wu64(rbx, rax); rax = rbx; crate::simd_ops::wu64(rbx.wrapping_add(P5), (0x0u64) as u64); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14161a9a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415bd1d0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rdx = 0x2;
    return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14161a9b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x5; const P1: u64 = 0x141524340; const P2: u64 = 0x2; const P3: u64 = 0x14183acbc; const P4: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rsi = rdx; r14 = rcx; rbx = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rax = ((rax).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rdx, rax); rdx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = ((rbx as u16)) as u16 as u64; fl_r = ((rbx as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { r15 = ((rbx.wrapping_mul(2).wrapping_add(P0)) as u32) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; r15 = ((((r15 as u32)) & (0xfffffffcu32))) as u32 as u64; r8 = ((r15.wrapping_add(rbx.wrapping_mul(4))) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = ((rbp.wrapping_add(0x34u64)) as u32) as u32 as u64; bb = 7; continue; }
            3 => { crate::simd_ops::wu16(rax, ((rbx as u16)) as u16); rcx = ((rax.wrapping_add(P2)) as u64) as u64; rdx = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); rdi = rbx; rbx = ((rbx).wrapping_add(rbx)) as u64; r8 = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_add_u64(rsi, (rbx) as u64); rbx = ((rdi.wrapping_mul(4)) as u64) as u64; rdx = (crate::simd_ops::ru64(rsi)) as u64; r8 = rbx; rcx = ((r15 as u32)) as u32 as u64; rcx = ((rcx).wrapping_add(rbp)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_add_u64(rsi, (rbx) as u64)) as u64; fl_r = fl_l; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(r14, rbp); bb = 6; continue; }
            4 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(r14, (0x0u64) as u64); bb = 6; continue; }
            6 => { rax = 0x1; bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14161aa80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14161a9b0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rdx = ((rdx).wrapping_add(0x4u64)) as u64;
    r8 = rsp.wrapping_add(0x40);
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdx);
    rcx = ((rcx).wrapping_add(0x18u64)) as u64;
    rdx = rsp.wrapping_add(0x38);
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14161aab0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x78; const P1: u64 = 0x14; const P2: u64 = 0x20; const P3: u64 = 0x1415355f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64; rdi = rcx; rbx = 0x1; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((rbx.wrapping_add(P1)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rcx = ((rcx).wrapping_add(0xf0u64)) as u64; rsi = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; rdx = ((rsi as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0x5; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; rax = 0x17; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbx = (if (((fl_l as u32)) == ((fl_r as u32))) { (rax as u32) } else { (rbx as u32) }) as u32 as u64; rax = ((rbx as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14161ab30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524330; const P1: u64 = 0x1415bf090; const P2: u64 = 0x1403cd2a0; const P3: u64 = 0xf8; const P4: u64 = 0x100; const P5: u64 = 0x14268f798; const P6: u64 = 0x142677588; const P7: u64 = 0x8; const P8: u64 = 0x142677ec8; const P9: u64 = 0x10; const P10: u64 = 0x142677ef0; const P11: u64 = 0x28; const P12: u64 = 0x14268f808;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = 0x108; rcx = 0x2; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; rcx = P2; crate::simd_ops::wu64(rbx.wrapping_add(P3), rax); crate::simd_ops::wu64(rbx.wrapping_add(P4), rax); rax = P5; crate::simd_ops::wu64(rbx, rax); rax = P6; crate::simd_ops::wu64(rbx.wrapping_add(P7), rax); rax = P8; crate::simd_ops::wu64(rbx.wrapping_add(P9), rax); rax = P10; crate::simd_ops::wu64(rbx.wrapping_add(P11), rax); rax = (crate::simd_ops::ru64(P12)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rcx = rbx; rax = crate::simd_ops::call_stk(rax, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 3; continue; }
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
pub unsafe extern "win64" fn fn_14161abe0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f00; const P1: u64 = 0x1401de410; const P2: u64 = 0x1415a2910; const P3: u64 = 0x14153be40;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu32(rsp.wrapping_add(0x18u64), ((r8 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rsi = rcx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x70u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rbx = (crate::simd_ops::ru32(rsi.wrapping_add(0x20u64))) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x70u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P0)) as u64; r8 = 0x1; r9 = ((rax as u32)) as u32 as u64; rdx = ((rbx as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = 0x5b; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { crate::simd_ops::lk_add_u64(rsp.wrapping_add(0x48u64), (0x4u64) as u64); r8 = rsp.wrapping_add(0x50); r9 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x48); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rdi = ((rax as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbp); rbp = (crate::simd_ops::ru32(rcx)) as u32 as u64; rcx = ((rcx).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rcx); fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 13; continue; } bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru32(rsi.wrapping_add(0x104u64))) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rdx = ((rbp as u32)) as u32 as u64; rcx = rsi.wrapping_add(0xf0); rdx = (((rdx as u32)).wrapping_sub((r8 as u32))) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rcx = 0x34; rcx = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (rdi as u32) } else { (rcx as u32) }) as u32 as u64; rdi = ((rcx as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 7; continue; }
            7 => { rbx = 0; fl_l = 0; fl_r = 0; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 13; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(rcx)) as u32 as u64; rcx = ((rcx).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rcx); rcx = rsi; crate::simd_ops::wu8(rsp.wrapping_add(0x44u64), (0x0u8) as u8); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xf; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 11; continue; }
            11 => { rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 9; continue; }
            13 => { rcx = ((rdi as u32)) as u32 as u64; bb = 14; continue; }
            14 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; bb = 15; continue; }
            15 => { rax = ((rcx as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14161ad20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14268fa10; const P1: u64 = 0x142677588; const P2: u64 = 0x8; const P3: u64 = 0x14268fc78; const P4: u64 = 0x10; const P5: u64 = 0x14268fca0; const P6: u64 = 0x28; const P7: u64 = 0x1415c77a0; const P8: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = P0;
    rbx = ((rdx as u32)) as u32 as u64;
    crate::simd_ops::wu64(rcx, rax);
    rdi = rcx;
    rax = P1;
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    rax = P3;
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rax);
    rax = P5;
    crate::simd_ops::wu64(rcx.wrapping_add(P6), rax);
    rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x1a0; rcx = rdi; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14161ad80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524330; const P1: u64 = 0x1415c7600; const P2: u64 = 0x14268fa10; const P3: u64 = 0x140697af0; const P4: u64 = 0x142677588; const P5: u64 = 0x8; const P6: u64 = 0x14268fc78; const P7: u64 = 0x10; const P8: u64 = 0x14268fca0; const P9: u64 = 0x28; const P10: u64 = 0x14268fa80; const P11: u64 = 0xd4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = 0x1a0; rcx = 0x2; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = P2; crate::simd_ops::wu64(rbx, rax); rcx = P3; rax = P4; crate::simd_ops::wu64(rbx.wrapping_add(P5), rax); rax = P6; crate::simd_ops::wu64(rbx.wrapping_add(P7), rax); rax = P8; crate::simd_ops::wu64(rbx.wrapping_add(P9), rax); rax = (crate::simd_ops::ru64(P10)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rcx = rbx; rax = crate::simd_ops::call_stk(rax, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xa; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 6; continue; }
            5 => { rax = (rax & !0xffu64) | (((0x80u8) as u8 as u64)); bb = 6; continue; }
            6 => { rcx = ((crate::simd_ops::ru8(rbx.wrapping_add(P11))) as u32) as u32 as u64; rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0x7fu8))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) | ((rax as u8)))) as u8 as u64)); rax = rbx; crate::simd_ops::wu8(rbx.wrapping_add(0xd4u64), ((rcx as u8)) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14161ae20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r9 = 0; fl_l = 0; fl_r = 0; r8 = 0x2; crate::simd_ops::wu64(rcx.wrapping_add(0x5800u64), r9); crate::simd_ops::wu32(rcx.wrapping_add(0x5808u64), ((r9 as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x5810u64), r9); bb = 1; continue; }
            1 => { rdx = ((r8.wrapping_sub(0x2u64)) as u32) as u32 as u64; rax = (((rdx) as i64).wrapping_mul((0xb0u64) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x5810u64))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rcx.wrapping_add(0x5810u64), rax); crate::simd_ops::wu64(rax, r9); bb = 4; continue; }
            3 => { crate::simd_ops::wu64(rax, rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x5810u64), rax); bb = 4; continue; }
            4 => { crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x5808u64), (1) as u32); rdx = ((r8.wrapping_sub(0x1u64)) as u32) as u32 as u64; rax = (((rdx) as i64).wrapping_mul((0xb0u64) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x5810u64))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rcx.wrapping_add(0x5810u64), rax); crate::simd_ops::wu64(rax, r9); bb = 7; continue; }
            6 => { crate::simd_ops::wu64(rax, rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x5810u64), rax); bb = 7; continue; }
            7 => { crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x5808u64), (1) as u32); rax = ((r8 as u32)) as u32 as u64; rdx = (((rax) as i64).wrapping_mul((0xb0u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x5810u64))) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rcx.wrapping_add(0x5810u64), rdx); crate::simd_ops::wu64(rdx, r9); bb = 10; continue; }
            9 => { crate::simd_ops::wu64(rdx, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x5810u64), rdx); bb = 10; continue; }
            10 => { crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x5808u64), (1) as u32); rdx = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = (((rdx) as i64).wrapping_mul((0xb0u64) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x5810u64))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rcx.wrapping_add(0x5810u64), rax); crate::simd_ops::wu64(rax, r9); bb = 13; continue; }
            12 => { crate::simd_ops::wu64(rax, rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x5810u64), rax); bb = 13; continue; }
            13 => { crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x5808u64), (1) as u32); rdx = ((r8.wrapping_add(0x2u64)) as u32) as u32 as u64; rax = (((rdx) as i64).wrapping_mul((0xb0u64) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x5810u64))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rcx.wrapping_add(0x5810u64), rax); crate::simd_ops::wu64(rax, r9); bb = 16; continue; }
            15 => { crate::simd_ops::wu64(rax, rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x5810u64), rax); bb = 16; continue; }
            16 => { crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x5808u64), (1) as u32); rdx = ((r8.wrapping_add(0x3u64)) as u32) as u32 as u64; rax = (((rdx) as i64).wrapping_mul((0xb0u64) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x5810u64))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { crate::simd_ops::wu64(rcx.wrapping_add(0x5810u64), rax); crate::simd_ops::wu64(rax, r9); bb = 19; continue; }
            18 => { crate::simd_ops::wu64(rax, rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x5810u64), rax); bb = 19; continue; }
            19 => { crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x5808u64), (1) as u32); rdx = ((r8.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = (((rdx) as i64).wrapping_mul((0xb0u64) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x5810u64))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { crate::simd_ops::wu64(rcx.wrapping_add(0x5810u64), rax); crate::simd_ops::wu64(rax, r9); bb = 22; continue; }
            21 => { crate::simd_ops::wu64(rax, rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x5810u64), rax); bb = 22; continue; }
            22 => { crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x5808u64), (1) as u32); rdx = ((r8.wrapping_add(0x5u64)) as u32) as u32 as u64; rax = (((rdx) as i64).wrapping_mul((0xb0u64) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x5810u64))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { crate::simd_ops::wu64(rcx.wrapping_add(0x5810u64), rax); crate::simd_ops::wu64(rax, r9); bb = 25; continue; }
            24 => { crate::simd_ops::wu64(rax, rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x5810u64), rax); bb = 25; continue; }
            25 => { crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x5808u64), (1) as u32); rdx = ((r8.wrapping_add(0x6u64)) as u32) as u32 as u64; rax = (((rdx) as i64).wrapping_mul((0xb0u64) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x5810u64))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 27; continue; } bb = 26; continue; }
            26 => { crate::simd_ops::wu64(rcx.wrapping_add(0x5810u64), rax); crate::simd_ops::wu64(rax, r9); bb = 28; continue; }
            27 => { crate::simd_ops::wu64(rax, rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x5810u64), rax); bb = 28; continue; }
            28 => { crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x5808u64), (1) as u32); rdx = ((r8.wrapping_add(0x7u64)) as u32) as u32 as u64; rax = (((rdx) as i64).wrapping_mul((0xb0u64) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x5810u64))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 30; continue; } bb = 29; continue; }
            29 => { crate::simd_ops::wu64(rcx.wrapping_add(0x5810u64), rax); crate::simd_ops::wu64(rax, r9); bb = 31; continue; }
            30 => { crate::simd_ops::wu64(rax, rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x5810u64), rax); bb = 31; continue; }
            31 => { crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x5808u64), (1) as u32); rdx = ((r8.wrapping_add(0x8u64)) as u32) as u32 as u64; rax = (((rdx) as i64).wrapping_mul((0xb0u64) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x5810u64))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 33; continue; } bb = 32; continue; }
            32 => { crate::simd_ops::wu64(rcx.wrapping_add(0x5810u64), rax); crate::simd_ops::wu64(rax, r9); bb = 34; continue; }
            33 => { crate::simd_ops::wu64(rax, rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x5810u64), rax); bb = 34; continue; }
            34 => { crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x5808u64), (1) as u32); rdx = ((r8.wrapping_add(0x9u64)) as u32) as u32 as u64; rax = (((rdx) as i64).wrapping_mul((0xb0u64) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x5810u64))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 36; continue; } bb = 35; continue; }
            35 => { crate::simd_ops::wu64(rcx.wrapping_add(0x5810u64), rax); crate::simd_ops::wu64(rax, r9); bb = 37; continue; }
            36 => { crate::simd_ops::wu64(rax, rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x5810u64), rax); bb = 37; continue; }
            37 => { crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x5808u64), (1) as u32); rdx = ((r8.wrapping_add(0xau64)) as u32) as u32 as u64; rax = (((rdx) as i64).wrapping_mul((0xb0u64) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x5810u64))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 39; continue; } bb = 38; continue; }
            38 => { crate::simd_ops::wu64(rcx.wrapping_add(0x5810u64), rax); crate::simd_ops::wu64(rax, r9); bb = 40; continue; }
            39 => { crate::simd_ops::wu64(rax, rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x5810u64), rax); bb = 40; continue; }
            40 => { crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x5808u64), (1) as u32); rdx = ((r8.wrapping_add(0xbu64)) as u32) as u32 as u64; rax = (((rdx) as i64).wrapping_mul((0xb0u64) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x5810u64))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 42; continue; } bb = 41; continue; }
            41 => { crate::simd_ops::wu64(rcx.wrapping_add(0x5810u64), rax); crate::simd_ops::wu64(rax, r9); bb = 43; continue; }
            42 => { crate::simd_ops::wu64(rax, rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x5810u64), rax); bb = 43; continue; }
            43 => { crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x5808u64), (1) as u32); rdx = ((r8.wrapping_add(0xcu64)) as u32) as u32 as u64; rax = (((rdx) as i64).wrapping_mul((0xb0u64) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x5810u64))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 45; continue; } bb = 44; continue; }
            44 => { crate::simd_ops::wu64(rcx.wrapping_add(0x5810u64), rax); crate::simd_ops::wu64(rax, r9); bb = 46; continue; }
            45 => { crate::simd_ops::wu64(rax, rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x5810u64), rax); bb = 46; continue; }
            46 => { crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x5808u64), (1) as u32); rdx = ((r8.wrapping_add(0xdu64)) as u32) as u32 as u64; rax = (((rdx) as i64).wrapping_mul((0xb0u64) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x5810u64))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 48; continue; } bb = 47; continue; }
            47 => { crate::simd_ops::wu64(rcx.wrapping_add(0x5810u64), rax); crate::simd_ops::wu64(rax, r9); bb = 49; continue; }
            48 => { crate::simd_ops::wu64(rax, rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x5810u64), rax); bb = 49; continue; }
            49 => { crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x5808u64), (1) as u32); r8 = (((r8 as u32)).wrapping_add(0x10u32)) as u32 as u64; rax = ((r8.wrapping_sub(0x2u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 1; continue; } bb = 50; continue; }
            50 => { rax = rcx; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14161b1c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x1415864b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    rbx = rcx;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    rdi = (crate::simd_ops::ru64(rdx)) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rax = rbx;
    fl_l = rdi; fl_r = rdi;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { fl_l = (crate::simd_ops::lk_add_u32(rdi, (1) as u32)) as u32 as u64; }
    crate::simd_ops::wu64(rbx, rdi);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14161b230(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14268fd30; const P1: u64 = 0x142677588; const P2: u64 = 0x8; const P3: u64 = 0x14268ffb0; const P4: u64 = 0x10; const P5: u64 = 0x14268ffd8; const P6: u64 = 0x28; const P7: u64 = 0x142690058; const P8: u64 = 0x118; const P9: u64 = 0x14161e050; const P10: u64 = 0x141621060; const P11: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = P0;
    rbx = ((rdx as u32)) as u32 as u64;
    crate::simd_ops::wu64(rcx, rax);
    rdi = rcx;
    rax = P1;
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    rax = P3;
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rax);
    rax = P5;
    crate::simd_ops::wu64(rcx.wrapping_add(P6), rax);
    rax = P7;
    crate::simd_ops::wu64(rcx.wrapping_add(P8), rax);
    rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp);
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x1a8; rcx = rdi; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14161b2b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x14; const P3: u64 = 0x1; const P4: u64 = 0x14161c2f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rdi = ((r8 as u32)) as u32 as u64; rsi = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; rbx = rdx; rcx = ((rax.wrapping_add(rsi.wrapping_mul(4))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = ((rax).wrapping_add(0x4u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rdx = 0; fl_l = 0; fl_r = 0; fl_l = rax; fl_r = rcx; rdx = (if (((fl_l as u64)) != ((fl_r as u64))) { rax } else { rdx }) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); rbp = ((rdx.wrapping_add(P3)) as u32) as u32 as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = ((rax as u32)) as u32 as u64; rcx = rbx; rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdx = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rbp as u32) } else { (rdx as u32) }) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; fl_l = rsi; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; rcx = ((rdx.wrapping_add(P3)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P1), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; r8 = ((rcx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu32(r8, ((rdi as u32)) as u32); rax = ((rbp as u32)) as u32 as u64; bb = 9; continue; }
            9 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            10 => { rax = 0x2; bb = 9; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = 0x39; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14161b370(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524330; const P1: u64 = 0x141620ef0; const P2: u64 = 0x14268fd30; const P3: u64 = 0x142677588; const P4: u64 = 0x14268ffb0; const P5: u64 = 0x14268ffd8; const P6: u64 = 0x142690058; const P7: u64 = 0x1416214f0; const P8: u64 = 0x141536a70; const P9: u64 = 0x1415bd1d0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = 0x1a8; rcx = 0x2; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { rcx = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rbx.wrapping_add(0x134u64), (0x2u8) as u8); rax = P2; crate::simd_ops::wu64(rbx.wrapping_add(0x120u64), rdi); crate::simd_ops::wu64(rbx, rax); rcx = rbx; rax = P3; crate::simd_ops::wu32(rbx.wrapping_add(0x128u64), ((rdi as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); rax = P4; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rax = P5; crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), rax); rax = P6; crate::simd_ops::wu64(rbx.wrapping_add(0x118u64), rax); crate::simd_ops::wu32(rbx.wrapping_add(0x130u64), ((rdi as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x138u64), ((rdi as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x140u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x148u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x150u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x158u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x160u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x168u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x170u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x178u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x180u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x188u64), rdi); crate::simd_ops::wu32(rbx.wrapping_add(0x190u64), ((rdi as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x198u64), rdi); crate::simd_ops::wu8(rbx.wrapping_add(0x1a0u64), ((rdi as u8)) as u8); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 2; continue; }
            2 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = rbx.wrapping_add(0x10); rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; rax = P8; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = ((rdi as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rax = crate::simd_ops::call_stk(rdx, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdi = 0; fl_l = 0; fl_r = 0; rax = ((rdi as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14161b4e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x188; const P1: u64 = 0x8; const P2: u64 = 0x30; const P3: u64 = 0x143930f78; const P4: u64 = 0x1415f34d0; const P5: u64 = 0x14161b590; const P6: u64 = 0x14159dc00;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbp = rcx; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rdi = rsi; bb = 2; continue; }
            2 => { rsi = (crate::simd_ops::ru64(rsi.wrapping_add(P1))) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; fl_l = rdi; fl_r = (crate::simd_ops::ru64(rbp.wrapping_add(0x188u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rbp.wrapping_add(0x188u64), rax); bb = 5; continue; }
            4 => { crate::simd_ops::wu64(0x8u64, rax); bb = 5; continue; }
            5 => { crate::simd_ops::lk_sub_u32(rbp.wrapping_add(0x180u64), (1) as u32); fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x190u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 6; continue; }
            6 => { r8 = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0x10u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(P3)) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rdx = rdi; rcx = rbp.wrapping_sub(0x10); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdi = rsi; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 9; continue; }
            9 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 10; continue; }
            10 => { rcx = rbp; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P6, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14161b590(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930fa0; const P1: u64 = 0x1415c53e0; const P2: u64 = 0x143930f90; const P3: u64 = 0x14159b3d0; const P4: u64 = 0x1415a1730; const P5: u64 = 0x1415af340; const P6: u64 = 0x1415242e0; const P7: u64 = 0x1415864b0; const P8: u64 = 0x143931dc8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; r8 = rdx; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x130u64))) as u64; rsi = rcx; rcx = (crate::simd_ops::ru64(P0)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x2cu64))) as u32 as u64; rcx = (crate::simd_ops::ru64(P2)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = 0x1; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x120u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x60u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x64u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = (rdx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 4; continue; }
            3 => { rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 4; continue; }
            4 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x60u64), ((rcx as u32)) as u32); if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 5; continue; }
            5 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 9; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x64u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x60u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rdi; rcx = 0x4; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { rdx = (crate::simd_ops::ru64(P8)) as u64; rcx = 0; fl_l = 0; fl_r = 0; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 14; continue; }
            14 => { rax = rdx.wrapping_add(0x5800); fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { fl_l = rbx; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = rdx; rdx = (crate::simd_ops::ru64(rax)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x5810u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { crate::simd_ops::wu64(rdx.wrapping_add(0x5810u64), rbx); crate::simd_ops::wu64(rbx, rax); bb = 20; continue; }
            19 => { crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x5810u64), rbx); bb = 20; continue; }
            20 => { crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x5808u64), (1) as u32); fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x5808u64))) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 25; continue; } bb = 21; continue; }
            21 => { fl_l = rdx; fl_r = (crate::simd_ops::ru64(P8)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x5800u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 23; continue; } bb = 22; continue; }
            22 => { crate::simd_ops::wu64(P8, rax); bb = 24; continue; }
            23 => { crate::simd_ops::wu64(rcx.wrapping_add(0x5800u64), rax); bb = 24; continue; }
            24 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 25; continue; }
            25 => { rcx = rsi.wrapping_add(0x10); rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 27; continue; } bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 27; continue; }
            27 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14161b7d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415af330; const P1: u64 = 0x143930f78; const P2: u64 = 0x1415f34d0; const P3: u64 = 0x14161b590; const P4: u64 = 0x1415af340; const P5: u64 = 0x1415242e0; const P6: u64 = 0x1415bdd20; const P7: u64 = 0x1415a0610;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x48u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); rdi = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r13); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); r15 = rdx; crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x24u64), (1) as u32); rcx = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 19; continue; } bb = 1; continue; }
            1 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 2; continue; }
            2 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 59; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x198u64))) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 59; continue; } bb = 4; continue; }
            4 => { r13 = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { fl_l = rcx; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0xa8u64))) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu32(rax.wrapping_add(0xa8u64), ((r13 as u32)) as u32); bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 10; continue; }
            10 => { bb = 59; continue; }
            11 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x198u64))) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 59; continue; } bb = 12; continue; }
            12 => { bb = 13; continue; }
            13 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { fl_l = rcx; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0xa8u64))) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu32(rax.wrapping_add(0xa8u64), (0x1u32) as u32); bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 13; continue; } bb = 18; continue; }
            18 => { bb = 59; continue; }
            19 => { rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x198u64))) as u64; r13 = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r12); r12 = (r12 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 58; continue; } bb = 20; continue; }
            20 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbp); bb = 21; continue; }
            21 => { rcx = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rcx); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 22; continue; }
            22 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(0x30u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 24; continue; } bb = 23; continue; }
            23 => { r13 = rsi; rbp = rdi.wrapping_add(0x1a0); rsi = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; bb = 42; continue; }
            24 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = rsi; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x198u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 26; continue; } bb = 25; continue; }
            25 => { crate::simd_ops::wu64(rdi.wrapping_add(0x198u64), rax); bb = 27; continue; }
            26 => { crate::simd_ops::wu64(r13.wrapping_add(0x8u64), rax); bb = 27; continue; }
            27 => { crate::simd_ops::lk_sub_u32(rdi.wrapping_add(0x190u64), (1) as u32); rsi = rax; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 36; continue; } bb = 28; continue; }
            28 => { fl_l = ((r12 as u8)) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 36; continue; } bb = 29; continue; }
            29 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x60u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 34; continue; } bb = 30; continue; }
            30 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x64u64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 34; continue; } bb = 31; continue; }
            31 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x60u64), ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 33; continue; } bb = 32; continue; }
            32 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x64u64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 35; continue; } bb = 33; continue; }
            33 => { rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r12 = (r12 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 40; continue; }
            34 => { fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x60u64), ((rax as u32)) as u32); bb = 35; continue; }
            35 => { r12 = (r12 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 40; continue; }
            36 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x1a0u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbp = rdi.wrapping_add(0x1a0); if (((fl_l as u8)) == ((fl_r as u8))) { bb = 41; continue; } bb = 37; continue; }
            37 => { rbp = rdi.wrapping_add(0x1a0); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 41; continue; } bb = 38; continue; }
            38 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; rbp = rdi.wrapping_add(0x1a0); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 41; continue; } bb = 39; continue; }
            39 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(P1)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 40; continue; }
            40 => { rbp = rdi.wrapping_add(0x1a0); bb = 41; continue; }
            41 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 42; continue; }
            42 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 21; continue; } bb = 43; continue; }
            43 => { fl_l = ((r12 as u8)) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 57; continue; } bb = 44; continue; }
            44 => { fl_l = (crate::simd_ops::ru8(rbp)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 46; continue; } bb = 45; continue; }
            45 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; r8 = rbx; rcx = (crate::simd_ops::ru64(P1)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 46; continue; }
            46 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x60u64))) as u32 as u64; rax = rbx.wrapping_add(0x64); fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 49; continue; } bb = 47; continue; }
            47 => { fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 49; continue; } bb = 48; continue; }
            48 => { rdx = (rdx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 50; continue; }
            49 => { rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 50; continue; }
            50 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x60u64), ((rcx as u32)) as u32); if (((fl_l as u32)) == ((fl_r as u32))) { bb = 54; continue; } bb = 51; continue; }
            51 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 55; continue; } bb = 52; continue; }
            52 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 55; continue; } bb = 53; continue; }
            53 => { fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 55; continue; } bb = 54; continue; }
            54 => { rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 55; continue; }
            55 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x60u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 57; continue; } bb = 56; continue; }
            56 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0x4; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 57; continue; }
            57 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; bb = 58; continue; }
            58 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 59; continue; }
            59 => { fl_l = (crate::simd_ops::ru8(r15.wrapping_add(0x1fu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 66; continue; } bb = 60; continue; }
            60 => { fl_l = (crate::simd_ops::ru8(r15.wrapping_add(0x1du64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 66; continue; } bb = 61; continue; }
            61 => { rcx = (crate::simd_ops::ru32(r15)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 64; continue; } bb = 62; continue; }
            62 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 76; continue; } bb = 63; continue; }
            63 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 66; continue; } bb = 64; continue; }
            64 => { rdx = 0; fl_l = 0; fl_r = 0; bb = 65; continue; }
            65 => { rcx = rdi.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 66; continue; }
            66 => { rsi = (crate::simd_ops::ru32(rdi.wrapping_add(0x100u64))) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 73; continue; } bb = 67; continue; }
            67 => { r14 = ((rsi as u32)) as u32 as u64; fl_l = ((r14).wrapping_shl(3u32)) as u64; r14 = ((fl_l as u64)) as u64; bb = 68; continue; }
            68 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0xf8u64))) as u64; r14 = r14.wrapping_sub(0x8); rsi = (((rsi as u32)).wrapping_sub(1)) as u32 as u64; fl_l = (crate::simd_ops::ru8(r15.wrapping_add(0x1cu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbx = (crate::simd_ops::ru64(r14.wrapping_add(rax))) as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 70; continue; } bb = 69; continue; }
            69 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x80u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 72; continue; } bb = 70; continue; }
            70 => { rdx = (crate::simd_ops::ru64(r15.wrapping_add(0x30u64))) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 72; continue; } bb = 71; continue; }
            71 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = r15; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x90u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 72; continue; }
            72 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 68; continue; } bb = 73; continue; }
            73 => { rcx = rdi.wrapping_add(0x10); rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 75; continue; } bb = 74; continue; }
            74 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rsp = ((rsp).wrapping_add(0x48u64)) as u64; return crate::simd_ops::tail_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
            75 => { rsp = ((rsp).wrapping_add(0x48u64)) as u64; return rax; }
            76 => { rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 65; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14161baf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415af330; const P1: u64 = 0x143930f78; const P2: u64 = 0x1415f34d0; const P3: u64 = 0x14161b590; const P4: u64 = 0x1415af340; const P5: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x38u64)) as u64; rsi = rcx; rcx = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 21; continue; } bb = 2; continue; }
            2 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 3; continue; }
            3 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 66; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; r15 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); rbx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rdi); rdi = (crate::simd_ops::ru64(rsi.wrapping_add(0x198u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r12); r12 = (r12 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), ((rax as u32)) as u32); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 65; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r13); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); bb = 6; continue; }
            6 => { r13 = rdi; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x30u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 31; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x2cu64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 31; continue; } bb = 10; continue; }
            10 => { r15 = rdi; rbp = rsi.wrapping_add(0x1a0); rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; bb = 49; continue; }
            11 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x198u64))) as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 66; continue; } bb = 12; continue; }
            12 => { r15 = 0; fl_l = 0; fl_r = 0; bb = 13; continue; }
            13 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { fl_l = rdx; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 19; continue; } bb = 15; continue; }
            15 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x2cu64))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 19; continue; } bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0xa8u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 19; continue; } bb = 18; continue; }
            18 => { crate::simd_ops::wu32(rax.wrapping_add(0xa8u64), ((r15 as u32)) as u32); bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 13; continue; } bb = 20; continue; }
            20 => { rsp = ((rsp).wrapping_add(0x38u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            21 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x198u64))) as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 66; continue; } bb = 22; continue; }
            22 => { bb = 23; continue; }
            23 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 24; continue; }
            24 => { fl_l = rdx; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 29; continue; } bb = 25; continue; }
            25 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 27; continue; } bb = 26; continue; }
            26 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x2cu64))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 29; continue; } bb = 27; continue; }
            27 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0xa8u64))) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 29; continue; } bb = 28; continue; }
            28 => { crate::simd_ops::wu32(rax.wrapping_add(0xa8u64), (0x1u32) as u32); bb = 29; continue; }
            29 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 23; continue; } bb = 30; continue; }
            30 => { rsp = ((rsp).wrapping_add(0x38u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            31 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(0x198u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 33; continue; } bb = 32; continue; }
            32 => { crate::simd_ops::wu64(rsi.wrapping_add(0x198u64), rax); bb = 34; continue; }
            33 => { crate::simd_ops::wu64(r15.wrapping_add(0x8u64), rax); bb = 34; continue; }
            34 => { crate::simd_ops::lk_sub_u32(rsi.wrapping_add(0x190u64), (1) as u32); rdi = rax; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 43; continue; } bb = 35; continue; }
            35 => { fl_l = ((r12 as u8)) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 43; continue; } bb = 36; continue; }
            36 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x60u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 41; continue; } bb = 37; continue; }
            37 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x64u64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 41; continue; } bb = 38; continue; }
            38 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x60u64), ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 40; continue; } bb = 39; continue; }
            39 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x64u64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 42; continue; } bb = 40; continue; }
            40 => { rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r12 = (r12 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 47; continue; }
            41 => { fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x60u64), ((rax as u32)) as u32); bb = 42; continue; }
            42 => { r12 = (r12 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 47; continue; }
            43 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x1a0u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbp = rsi.wrapping_add(0x1a0); if (((fl_l as u8)) == ((fl_r as u8))) { bb = 48; continue; } bb = 44; continue; }
            44 => { rbp = rsi.wrapping_add(0x1a0); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 48; continue; } bb = 45; continue; }
            45 => { r8 = (crate::simd_ops::ru64(r13.wrapping_add(0x30u64))) as u64; rbp = rsi.wrapping_add(0x1a0); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 48; continue; } bb = 46; continue; }
            46 => { rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x20u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(P1)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 47; continue; }
            47 => { rbp = rsi.wrapping_add(0x1a0); bb = 48; continue; }
            48 => { rdx = r13; rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x58u64))) as u32 as u64; bb = 49; continue; }
            49 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 50; continue; }
            50 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_l = ((r12 as u8)) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 64; continue; } bb = 51; continue; }
            51 => { fl_l = (crate::simd_ops::ru8(rbp)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 53; continue; } bb = 52; continue; }
            52 => { rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x20u64))) as u32 as u64; r8 = rbx; rcx = (crate::simd_ops::ru64(P1)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 53; continue; }
            53 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x60u64))) as u32 as u64; rax = rbx.wrapping_add(0x64); fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 56; continue; } bb = 54; continue; }
            54 => { fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 56; continue; } bb = 55; continue; }
            55 => { rdx = (rdx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 57; continue; }
            56 => { rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 57; continue; }
            57 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x60u64), ((rcx as u32)) as u32); if (((fl_l as u32)) == ((fl_r as u32))) { bb = 61; continue; } bb = 58; continue; }
            58 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 62; continue; } bb = 59; continue; }
            59 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 62; continue; } bb = 60; continue; }
            60 => { fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 62; continue; } bb = 61; continue; }
            61 => { rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 62; continue; }
            62 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x60u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 64; continue; } bb = 63; continue; }
            63 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0x4; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 64; continue; }
            64 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; bb = 65; continue; }
            65 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 66; continue; }
            66 => { rsp = ((rsp).wrapping_add(0x38u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14161bdf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14153eec0; const P1: u64 = 0x143930fa0; const P2: u64 = 0x1415c43f0; const P3: u64 = 0x143930f88; const P4: u64 = 0x1426452b0; const P5: u64 = 0x1426452a8; const P6: u64 = 0x142645560;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x20u64), r9); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0xc0u64)) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x164u64))) as u32 as u64; fl_r = 0x0; rsi = rcx; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 94; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x134u64))) as u8 as u64; fl_r = (0x3u8) as u8 as u64; crate::simd_ops::wu64(rax.wrapping_sub(0x10u64), rbx); crate::simd_ops::wu64(rax.wrapping_sub(0x18u64), rbp); crate::simd_ops::wu64(rax.wrapping_sub(0x20u64), rdi); crate::simd_ops::wu64(rax.wrapping_sub(0x28u64), r12); crate::simd_ops::wu64(rax.wrapping_sub(0x30u64), r13); crate::simd_ops::wu64(rax.wrapping_sub(0x38u64), r14); crate::simd_ops::wu64(rax.wrapping_sub(0x40u64), r15); if (((fl_l as u8)) != ((fl_r as u8))) { bb = 57; continue; } bb = 2; continue; }
            2 => { rbx = ((crate::simd_ops::ru8(rsp.wrapping_add(0xf0u64))) as u32) as u32 as u64; fl_l = ((rbx as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x130u64))) as u64; r8 = rax.wrapping_add(0x8); rdx = rbx; rcx = r9; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rax, rbx); bb = 5; continue; }
            5 => { rbx = ((crate::simd_ops::ru8(rsp.wrapping_add(0xf0u64))) as u32) as u32 as u64; bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x100u64))) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x108u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x130u64))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xf8u64))) as u64; rdx = rsp.wrapping_add(0x50); rcx = (crate::simd_ops::ru64(P1)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((r8 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), r8); crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), ((r8 as u32)) as u32); r8 = (crate::simd_ops::ru64(rsi.wrapping_add(0x130u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); crate::simd_ops::wu8(rsp.wrapping_add(0x68u64), (0xffu8) as u8); crate::simd_ops::wu8(rsp.wrapping_add(0x70u64), (0xffu8) as u8); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r8 = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { rdi = (crate::simd_ops::ru64(rsi.wrapping_add(0x140u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); rcx = rdi; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x138u64))) as u32 as u64; rax = rdi; bb = 14; continue; }
            12 => { rax = rcx.wrapping_add(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 17; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x138u64))) as u32 as u64; rax = rdi; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 33; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 16; continue; }
            16 => { rax = r8; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); bb = 17; continue; }
            17 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 93; continue; } bb = 18; continue; }
            18 => { bb = 19; continue; }
            19 => { r13 = rdi.wrapping_add(0x10); r12 = (((rbx as u8)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; fl_l = r13; fl_r = rax; rax = (crate::simd_ops::ru32(r13.wrapping_add(0x10u64))) as u32 as u64; r12 = (if (((fl_l as u64)) != ((fl_r as u64))) { (r8 as u32) } else { (r12 as u32) }) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 55; continue; } bb = 20; continue; }
            20 => { rdi = (crate::simd_ops::ru32(rsp.wrapping_add(0x108u64))) as u32 as u64; bb = 21; continue; }
            21 => { r15 = (crate::simd_ops::ru32(rbx)) as u32 as u64; rsi = (crate::simd_ops::ru64(P3)) as u64; bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru32(rsi)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 34; continue; } bb = 23; continue; }
            23 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rsi, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 22; continue; } bb = 24; continue; }
            24 => { r14 = rsi.wrapping_add(0x8); bb = 25; continue; }
            25 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x48u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 37; continue; } bb = 26; continue; }
            26 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r15 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 27; continue; }
            27 => { bb = 28; continue; }
            28 => { fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 36; continue; } bb = 29; continue; }
            29 => { rbp = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 28; continue; } bb = 30; continue; }
            30 => { r8 = 0; fl_l = 0; fl_r = 0; rbp = ((r8 as u32)) as u32 as u64; bb = 31; continue; }
            31 => { fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 38; continue; } bb = 32; continue; }
            32 => { fl_l = (crate::simd_ops::lk_add_u32(rbp.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 38; continue; }
            33 => { fl_l = ((rax).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); bb = 19; continue; }
            34 => { r14 = rsi.wrapping_add(0x8); rcx = r14; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsi)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 25; continue; } bb = 35; continue; }
            35 => { rcx = r14; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); bb = 22; continue; }
            36 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 31; continue; }
            37 => { r8 = 0; fl_l = 0; fl_r = 0; rbp = ((r8 as u32)) as u32 as u64; bb = 38; continue; }
            38 => { rax = (crate::simd_ops::ru32(rsi)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 46; continue; } bb = 39; continue; }
            39 => { bb = 40; continue; }
            40 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rsi, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 43; continue; } bb = 41; continue; }
            41 => { rax = (crate::simd_ops::ru32(rsi)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 40; continue; } bb = 42; continue; }
            42 => { bb = 47; continue; }
            43 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 49; continue; } bb = 44; continue; }
            44 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 49; continue; } bb = 45; continue; }
            45 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x38u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); bb = 48; continue; }
            46 => { r14 = rsi.wrapping_add(0x8); bb = 47; continue; }
            47 => { rcx = r14; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); bb = 48; continue; }
            48 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 49; continue; }
            49 => { fl_l = rbp; fl_r = rbp; rsi = rbp.wrapping_sub(0x10); rsi = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r8 } else { rsi }) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 53; continue; } bb = 50; continue; }
            50 => { rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x100u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0xe8u64))) as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rdi as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xf8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); rcx = rsi; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((r12 as u8)) as u8); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x68u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rsi.wrapping_add(0x10); rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 52; continue; } bb = 51; continue; }
            51 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 52; continue; }
            52 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 53; continue; }
            53 => { rcx = (crate::simd_ops::ru32(r13.wrapping_add(0x10u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x4u64)) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 54; continue; }
            54 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 55; continue; }
            55 => { rdi = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = ((crate::simd_ops::ru8(rsp.wrapping_add(0xf0u64))) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 19; continue; } bb = 56; continue; }
            56 => { bb = 93; continue; }
            57 => { rdi = (crate::simd_ops::ru64(rcx.wrapping_add(0x140u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 93; continue; } bb = 58; continue; }
            58 => { r12 = (crate::simd_ops::ru32(rsp.wrapping_add(0x108u64))) as u32 as u64; r13 = (crate::simd_ops::ru32(rsp.wrapping_add(0x100u64))) as u32 as u64; bb = 59; continue; }
            59 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 92; continue; } bb = 60; continue; }
            60 => { bb = 61; continue; }
            61 => { r15 = (crate::simd_ops::ru32(rbx)) as u32 as u64; rsi = (crate::simd_ops::ru64(P3)) as u64; bb = 62; continue; }
            62 => { rax = (crate::simd_ops::ru32(rsi)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 73; continue; } bb = 63; continue; }
            63 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rsi, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 62; continue; } bb = 64; continue; }
            64 => { r14 = rsi.wrapping_add(0x8); bb = 65; continue; }
            65 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x48u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 76; continue; } bb = 66; continue; }
            66 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r15 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 70; continue; } bb = 67; continue; }
            67 => { bb = 68; continue; }
            68 => { fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 75; continue; } bb = 69; continue; }
            69 => { rbp = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 68; continue; } bb = 70; continue; }
            70 => { r8 = 0; fl_l = 0; fl_r = 0; rbp = ((r8 as u32)) as u32 as u64; bb = 71; continue; }
            71 => { fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 77; continue; } bb = 72; continue; }
            72 => { fl_l = (crate::simd_ops::lk_add_u32(rbp.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 77; continue; }
            73 => { r14 = rsi.wrapping_add(0x8); rcx = r14; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsi)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 65; continue; } bb = 74; continue; }
            74 => { rcx = r14; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); bb = 62; continue; }
            75 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 71; continue; }
            76 => { r8 = 0; fl_l = 0; fl_r = 0; rbp = ((r8 as u32)) as u32 as u64; bb = 77; continue; }
            77 => { rax = (crate::simd_ops::ru32(rsi)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 85; continue; } bb = 78; continue; }
            78 => { bb = 79; continue; }
            79 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rsi, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 82; continue; } bb = 80; continue; }
            80 => { rax = (crate::simd_ops::ru32(rsi)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 79; continue; } bb = 81; continue; }
            81 => { bb = 86; continue; }
            82 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 88; continue; } bb = 83; continue; }
            83 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 88; continue; } bb = 84; continue; }
            84 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x38u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); bb = 87; continue; }
            85 => { r14 = rsi.wrapping_add(0x8); bb = 86; continue; }
            86 => { rcx = r14; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); bb = 87; continue; }
            87 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 88; continue; }
            88 => { fl_l = rbp; fl_r = rbp; rsi = rbp.wrapping_sub(0x10); rsi = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r8 } else { rsi }) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 91; continue; } bb = 89; continue; }
            89 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xf8u64))) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0xe8u64))) as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((r12 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r13 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); rcx = ((crate::simd_ops::ru8(rsp.wrapping_add(0xf0u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((rcx as u8)) as u8); rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x68u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rsi.wrapping_add(0x10); rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 91; continue; } bb = 90; continue; }
            90 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 91; continue; }
            91 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x4u64)) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 61; continue; } bb = 92; continue; }
            92 => { rdi = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 59; continue; } bb = 93; continue; }
            93 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; bb = 94; continue; }
            94 => { rsp = ((rsp).wrapping_add(0xc0u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14161c2f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524350; const P1: u64 = 0x141524320; const P2: u64 = 0x14183acbc; const P3: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(0x14u64))) as u32 as u64; rbx = rcx; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; rdx = ((rbp as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(2u32)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { fl_l = rdx; fl_r = (0x4u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdi = rcx; bb = 5; continue; }
            3 => { rcx = 0; fl_l = 0; fl_r = 0; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 4; continue; }
            4 => { r8 = rdx; r9 = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; rdx = rax; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdi = rax; bb = 5; continue; }
            5 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 6; continue; }
            6 => { bb = 21; continue; }
            7 => { r8 = 0x1; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 8; continue; }
            8 => { r8 = ((rsi.wrapping_mul(4)) as u32) as u32 as u64; rdx = rbx; rcx = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 21; continue; }
            9 => { fl_l = rdx; fl_r = (0x4u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdi = rbx; bb = 12; continue; }
            11 => { rcx = 0; fl_l = 0; fl_r = 0; r8 = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdi = rax; bb = 12; continue; }
            12 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 22; continue; }
            14 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 15; continue; }
            15 => { fl_l = rax; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 21; continue; } bb = 16; continue; }
            16 => { rdx = 0; fl_l = 0; fl_r = 0; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 17; continue; }
            17 => { bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(rdx.wrapping_mul(4)))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(rdx.wrapping_mul(4)), ((rcx as u32)) as u32); rdx = ((rdx).wrapping_add(1)) as u64; fl_l = rdx; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 18; continue; } bb = 19; continue; }
            19 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 21; continue; } bb = 20; continue; }
            20 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 21; continue; }
            21 => { crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rdi); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rbx.wrapping_add(0x14u64), ((rbp as u32)) as u32); bb = 22; continue; }
            22 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14161c400(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xd6; const P1: u64 = 0x1415a0660;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x2u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
    return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14161c420(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x28; const P2: u64 = 0x8; const P3: u64 = 0x1; const P4: u64 = 0x14161d720; const P5: u64 = 0x14161e400;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 1; continue; }
            1 => { r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(r9.wrapping_add(P2))) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { r9 = (crate::simd_ops::ru64(r9)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = ((r9.wrapping_add(P3)) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            5 => { rcx = ((rcx).wrapping_add(0xfffffffffffffee8u64)) as u64; rdx = ((r9.wrapping_add(P0)) as u64) as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; return crate::simd_ops::tail_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
            7 => { rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rax = 0x1; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14161c480(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1a0; const P2: u64 = 0x30; const P3: u64 = 0x143930f78; const P4: u64 = 0x20; const P5: u64 = 0x1415f3870; const P6: u64 = 0x1415f4a50; const P7: u64 = 0x1415f34d0; const P8: u64 = 0x14161c570;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm3 as u32)) as u32);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rbx = rcx;
    fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if (((fl_l as u8)) == ((fl_r as u8))) { r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x50u64))) as u32 as u64; rdx = rbx; r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
    r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64;
    rcx = (crate::simd_ops::ru64(P3)) as u64;
    fl_l = r8; fl_r = r8;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { r9 = (crate::simd_ops::ru32(rax.wrapping_add(P4))) as u32 as u64; rdx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; r9 = rsp.wrapping_add(0x30); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rcx = (crate::simd_ops::ru64(P3)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), (0x1u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x0u32) as u32); r8 = (crate::simd_ops::ru32(rax.wrapping_add(P4))) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); } else { rdx = (crate::simd_ops::ru32(rax.wrapping_add(P4))) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
    r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x50u64))) as u32 as u64;
    rdx = rbx;
    r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14161c510(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x198; const P1: u64 = 0x14161b590; const P2: u64 = 0x8; const P3: u64 = 0x190; const P4: u64 = 0x28; const P5: u64 = 0x1415bd8a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rcx; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdx = rax; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; fl_l = (crate::simd_ops::lk_sub_u32(rbx.wrapping_add(P3), (1) as u32)) as u32 as u64; crate::simd_ops::wu64(rbx.wrapping_add(P0), rax); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } bb = 4; continue; }
            4 => { rcx = ((rbx.wrapping_add(P4)) as u64) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P5, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14161c570(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14161de80; const P1: u64 = 0x14161d110;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r14); r10 = ((r8 as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r15); r11 = rdx; rbp = rcx; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { r10 = (crate::simd_ops::ru32(rcx.wrapping_add(0x138u64))) as u32 as u64; bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x140u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r12); rax = r8; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 5; continue; }
            5 => { r13 = 0; fl_l = 0; fl_r = 0; r10 = 0x10; r12 = ((r13.wrapping_add(0x8u64)) as u32) as u32 as u64; bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), r10); fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 13; continue; } bb = 7; continue; }
            7 => { rax = r8; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 10; continue; }
            10 => { bb = 13; continue; }
            11 => { r13 = rax.wrapping_add(0x10); r10 = rax.wrapping_add(0x20); r12 = rax.wrapping_add(0x18); bb = 6; continue; }
            12 => { rsi = rax.wrapping_add(0x10); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 20; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x138u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 19; continue; } bb = 14; continue; }
            14 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 18; continue; } bb = 16; continue; }
            16 => { r8 = (crate::simd_ops::ru64(r8)) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 15; continue; } bb = 17; continue; }
            17 => { bb = 19; continue; }
            18 => { rsi = r8.wrapping_add(0x10); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { rsi = 0; fl_l = 0; fl_r = 0; bb = 20; continue; }
            20 => { r14 = (crate::simd_ops::ru32(rdx.wrapping_add(0x2cu64))) as u32 as u64; r15 = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rdi); fl_l = r13; fl_r = r13; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 37; continue; } bb = 21; continue; }
            21 => { rbx = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru32(r10)) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 37; continue; } bb = 22; continue; }
            22 => { rdi = ((crate::simd_ops::ru8(rsp.wrapping_add(0x38u64))) as u32) as u32 as u64; bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x168u64))) as u64; rdx = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 24; continue; }
            24 => { bb = 25; continue; }
            25 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 52; continue; } bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 25; continue; } bb = 27; continue; }
            27 => { fl_l = ((((rdi as u8)) & (0xe0u8))) as u8 as u64; fl_r = fl_l; rdi = (rdi & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x0u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((rdi as u8)) as u8); bb = 28; continue; }
            28 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0xd6u64))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 34; continue; } bb = 29; continue; }
            29 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 34; continue; } bb = 30; continue; }
            30 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 34; continue; } bb = 31; continue; }
            31 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; r8 = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 34; continue; } bb = 32; continue; }
            32 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 35; continue; } bb = 33; continue; }
            33 => { rax = ((rax).wrapping_add(0x4u64)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 32; continue; } bb = 34; continue; }
            34 => { r9 = r15; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r14 as u32)) as u32); r8 = rsp.wrapping_add(0x30); rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; bb = 35; continue; }
            35 => { rcx = (crate::simd_ops::ru32(r10)) as u32 as u64; rbx = ((rbx).wrapping_add(0x4u64)) as u64; rax = (crate::simd_ops::ru64(r12)) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 23; continue; } bb = 36; continue; }
            36 => { r11 = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; bb = 37; continue; }
            37 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 51; continue; } bb = 38; continue; }
            38 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 51; continue; } bb = 39; continue; }
            39 => { rdi = ((crate::simd_ops::ru8(rsp.wrapping_add(0x38u64))) as u32) as u32 as u64; bb = 40; continue; }
            40 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x168u64))) as u64; rdx = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 43; continue; } bb = 41; continue; }
            41 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 54; continue; } bb = 42; continue; }
            42 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 41; continue; } bb = 43; continue; }
            43 => { fl_l = ((((rdi as u8)) & (0xe0u8))) as u8 as u64; fl_r = fl_l; rdi = (rdi & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); bb = 44; continue; }
            44 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 49; continue; } bb = 45; continue; }
            45 => { fl_l = r13; fl_r = r13; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 49; continue; } bb = 46; continue; }
            46 => { rax = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru32(r10)) as u32 as u64; r8 = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 49; continue; } bb = 47; continue; }
            47 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 50; continue; } bb = 48; continue; }
            48 => { rax = ((rax).wrapping_add(0x4u64)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 47; continue; } bb = 49; continue; }
            49 => { r8 = r11; rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; bb = 50; continue; }
            50 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x4u64)) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; r11 = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 40; continue; } bb = 51; continue; }
            51 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            52 => { fl_l = ((rax).wrapping_add(0xcu64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 27; continue; } bb = 53; continue; }
            53 => { xmm0 = ((crate::simd_ops::ru64(rax)) as u64) as u128; rdi = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rdi as u32)) as u32); bb = 28; continue; }
            54 => { fl_l = ((rax).wrapping_add(0xcu64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 43; continue; } bb = 55; continue; }
            55 => { rdi = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; bb = 44; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14161c810(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415945c0; const P1: u64 = 0x143930fa0; const P2: u64 = 0x1415c43f0; const P3: u64 = 0x14159cab0; const P4: u64 = 0x143931dc8; const P5: u64 = 0x141524330; const P6: u64 = 0x14161ae20; const P7: u64 = 0x14268fd20; const P8: u64 = 0x14161b1c0; const P9: u64 = 0x1415af330; const P10: u64 = 0x1415c4950; const P11: u64 = 0x143930f90; const P12: u64 = 0x141599ce0; const P13: u64 = 0x141620f50; const P14: u64 = 0x14161b590; const P15: u64 = 0x1415a1730; const P16: u64 = 0x143930f78; const P17: u64 = 0x1415f3870; const P18: u64 = 0x1415f4a50; const P19: u64 = 0x141594db0; const P20: u64 = 0x14158f5a0; const P21: u64 = 0x1415f9ee0; const P22: u64 = 0x143930f88; const P23: u64 = 0x1426452b0; const P24: u64 = 0x1426452a8; const P25: u64 = 0x142645560; const P26: u64 = 0x1415faca0; const P27: u64 = 0x141620e80; const P28: u64 = 0x141621270; const P29: u64 = 0x141620d10; const P30: u64 = 0x141620d50; const P31: u64 = 0x141621110; const P32: u64 = 0x141621010; const P33: u64 = 0x14152ede0; const P34: u64 = 0x1415864b0; const P35: u64 = 0x141621530; const P36: u64 = 0x141621390;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 129>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = (rsp) as u64; rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rbp = ((r11.wrapping_sub(0x308u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x3e8u64)) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x130u64))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; r14 = (0x2u32) as u32 as u64; rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; r13 = (rdx) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x320u64), ((r14 as u32)) as u32); r12 = (rcx) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x328u64), ((rax as u32)) as u32); if (((fl_l as u32)) == ((fl_r as u32))) { bb = 130; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x164u64))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 130; continue; } bb = 2; continue; }
            2 => { rcx = ((crate::simd_ops::ru8(rdx.wrapping_add(0x98u64))) as u32) as u32 as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), (rbx) as u64); crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), (rsi) as u64); rsi = ((crate::simd_ops::ru8(rdx.wrapping_add(0x99u64))) as u32) as u32 as u64; rax = ((rcx.wrapping_add(0x80u64)) as u32) as u32 as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), (rdi) as u64); crate::simd_ops::wu64(r11.wrapping_sub(0x40u64), (r15) as u64); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xcfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xb0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdi = ((crate::simd_ops::ru8(rdx.wrapping_add(0x9cu64))) as u32) as u32 as u64; bb = 6; continue; }
            5 => { rdi = (rdi & !0xffu64) | (((0xffu8) as u8 as u64)); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru32(rdx.wrapping_add(0x34u64))) as u32 as u64; rcx = (r13) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = (crate::simd_ops::ru64(r12.wrapping_add(0x130u64))) as u64; rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = ((rax as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rbx as u32)) as u32); rax = (crate::simd_ops::ru64(r13.wrapping_add(0x10u64))) as u64; r15 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru64(P1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (r15) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rax) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x58u64), ((rsi as u8)) as u8); crate::simd_ops::wu8(rsp.wrapping_add(0x60u64), ((rdi as u8)) as u8); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rsi = ((rax as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru8(r12.wrapping_add(0xd6u64))) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 34; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(r13.wrapping_add(0x98u64))) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 34; continue; } bb = 8; continue; }
            8 => { r8 = (crate::simd_ops::ru64(r12)) as u64; rdx = ((r15.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = (r12) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(r8.wrapping_add(0x118u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (r12) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { r8 = (crate::simd_ops::ru64(r12)) as u64; rdx = ((r15.wrapping_add(0x1u64)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(r8.wrapping_add(0x120u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 10; continue; }
            10 => { rax = ((r14 as u32)) as u32 as u64; bb = 11; continue; }
            11 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x3d0u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x3d8u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x3e0u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x3c8u64))) as u64; rsp = ((rsp).wrapping_add(0x3e8u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            12 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(P4)) as u64; rdi = (r15) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 13; continue; }
            13 => { bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x5810u64))) as u64 as u64; fl_r = (r15) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 20; continue; } bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x5800u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 16; continue; }
            16 => { rdx = (0x5818u32) as u32 as u64; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 17; continue; }
            17 => { rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(P4)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { crate::simd_ops::wu64(P4, (rbx) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x5800u64), (r15) as u64); bb = 20; continue; }
            19 => { crate::simd_ops::wu64(rbx.wrapping_add(0x5800u64), (rax) as u64); crate::simd_ops::wu64(P4, (rbx) as u64); bb = 20; continue; }
            20 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x5810u64))) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 40; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru64(rdi)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r15 } else { rax }) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x5810u64), (rax) as u64); fl_l = (crate::simd_ops::lk_sub_u32(rbx.wrapping_add(0x5808u64), (1) as u32)) as u32 as u64; bb = 22; continue; }
            22 => { fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 40; continue; } bb = 23; continue; }
            23 => { rax = ((P7) as u64) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x28u64), (r15) as u64); crate::simd_ops::wu64(rdi.wrapping_add(0x20u64), (r15) as u64); crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), (r15) as u64); crate::simd_ops::wu64(rdi.wrapping_add(0x30u64), (r15) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0x38u64), ((r15 as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0x40u64), ((r15 as u32)) as u32); crate::simd_ops::wu8(rdi.wrapping_add(0x48u64), (0xffu8) as u8); crate::simd_ops::wu8(rdi.wrapping_add(0x50u64), (0xffu8) as u8); crate::simd_ops::wu64(rdi.wrapping_add(0x58u64), (r15) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0xa4u64), ((r15 as u32)) as u32); crate::simd_ops::wu64(rdi, (rax) as u64); crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), (r12) as u64); crate::simd_ops::lk_add_u32(r12.wrapping_add(0x24u64), (1) as u32); rax = (crate::simd_ops::ru32(r13.wrapping_add(0x80u64))) as u32 as u64; rdx = ((r13.wrapping_add(0x20u64)) as u64) as u64; crate::simd_ops::wu32(rdi.wrapping_add(0xa8u64), ((rax as u32)) as u32); rcx = ((rdi.wrapping_add(0x18u64)) as u64) as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(rdi.wrapping_add(0x30u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; crate::simd_ops::wu128(rdi.wrapping_add(0x40u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; crate::simd_ops::wu128(rdi.wrapping_add(0x50u64), (xmm0) as u128); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(r13.wrapping_add(0x38u64))) as u128; crate::simd_ops::wu128(rdi.wrapping_add(0x60u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(r13.wrapping_add(0x48u64))) as u128; crate::simd_ops::wu128(rdi.wrapping_add(0x70u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(r13.wrapping_add(0x58u64))) as u128; crate::simd_ops::wu128(rdi.wrapping_add(0x80u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(r13.wrapping_add(0x68u64))) as u128; crate::simd_ops::wu128(rdi.wrapping_add(0x90u64), (xmm1) as u128); rax = (crate::simd_ops::ru32(r13.wrapping_add(0x78u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0xa0u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r13.wrapping_add(0x7cu64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0xa4u64), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x30u64))) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x60u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 28; continue; } bb = 24; continue; }
            24 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x64u64))) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 28; continue; } bb = 25; continue; }
            25 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x60u64), ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 27; continue; } bb = 26; continue; }
            26 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x64u64))) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 29; continue; } bb = 27; continue; }
            27 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 29; continue; }
            28 => { fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x60u64), ((rax as u32)) as u32); bb = 29; continue; }
            29 => { rdx = (crate::simd_ops::ru64(r12.wrapping_add(0x130u64))) as u64; r8 = (rdi) as u64; rcx = (crate::simd_ops::ru64(P1)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rbp.wrapping_add(0x320u64), ((rax as u32)) as u32); r14 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 39; continue; } bb = 30; continue; }
            30 => { rcx = (crate::simd_ops::ru64(r12.wrapping_add(0x198u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 32; continue; } bb = 31; continue; }
            31 => { crate::simd_ops::wu64(r12.wrapping_add(0x198u64), (rdi) as u64); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), (r15) as u64); bb = 33; continue; }
            32 => { crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), (rcx) as u64); crate::simd_ops::wu64(r12.wrapping_add(0x198u64), (rdi) as u64); bb = 33; continue; }
            33 => { crate::simd_ops::lk_add_u32(r12.wrapping_add(0x190u64), (1) as u32); rdx = (crate::simd_ops::ru32(r13.wrapping_add(0x34u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(P11)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 34; continue; }
            34 => { r8 = (r12) as u64; rcx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; rdx = (r13) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rbp.wrapping_add(0x318u64), ((r15 as u32)) as u32); fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 42; continue; } bb = 35; continue; }
            35 => { rax = (crate::simd_ops::ru64(r12.wrapping_add(0x140u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 42; continue; } bb = 36; continue; }
            36 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 41; continue; } bb = 37; continue; }
            37 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 36; continue; } bb = 38; continue; }
            38 => { bb = 42; continue; }
            39 => { rdx = (rdi) as u64; rcx = (r12) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P14) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 40; continue; }
            40 => { rax = (crate::simd_ops::ru64(r12)) as u64; rdx = (0x1u32) as u32 as u64; rcx = (r12) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x120u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (r12) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P15) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (0x2u32) as u32 as u64; bb = 11; continue; }
            41 => { rdi = ((rax.wrapping_add(0x10u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rdi) as u64); fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 50; continue; } bb = 42; continue; }
            42 => { rsi = (crate::simd_ops::ru32(r12.wrapping_add(0x138u64))) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 47; continue; } bb = 43; continue; }
            43 => { rax = (crate::simd_ops::ru64(r12.wrapping_add(0x140u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 47; continue; } bb = 44; continue; }
            44 => { bb = 45; continue; }
            45 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 49; continue; } bb = 46; continue; }
            46 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 45; continue; } bb = 47; continue; }
            47 => { rsi = (crate::simd_ops::ru32(rbp.wrapping_add(0x328u64))) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 122; continue; } bb = 48; continue; }
            48 => { r14 = ((rsi.wrapping_add(0x1u64)) as u32) as u32 as u64; bb = 128; continue; }
            49 => { rdi = ((rax.wrapping_add(0x10u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rdi) as u64); fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 47; continue; } bb = 50; continue; }
            50 => { fl_l = (crate::simd_ops::ru8(r12.wrapping_add(0x1a0u64))) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 55; continue; } bb = 51; continue; }
            51 => { r9 = (crate::simd_ops::ru32(r12.wrapping_add(0x20u64))) as u32 as u64; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; r8 = (crate::simd_ops::ru64(r13.wrapping_add(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(P16)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P17) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 53; continue; } bb = 52; continue; }
            52 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x24u64))) as u32 as u64; fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; bb = 54; continue; }
            53 => { crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rsi as u32)) as u32); rax = (0x1u32) as u32 as u64; bb = 54; continue; }
            54 => { r8 = (crate::simd_ops::ru32(r12.wrapping_add(0x20u64))) as u32 as u64; r9 = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rdx = (crate::simd_ops::ru64(r13.wrapping_add(0x10u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rcx = (crate::simd_ops::ru64(P16)) as u64; crate::simd_ops::wu8(rbp.wrapping_add(0x310u64), (if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((rax as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P18) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 56; continue; }
            55 => { crate::simd_ops::wu8(rbp.wrapping_add(0x310u64), ((r15 as u8)) as u8); bb = 56; continue; }
            56 => { rsi = (crate::simd_ops::ru32(rbp.wrapping_add(0x328u64))) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 59; continue; } bb = 57; continue; }
            57 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 59; continue; } bb = 58; continue; }
            58 => { r14 = ((rsi.wrapping_add(0x1u64)) as u32) as u32 as u64; bb = 128; continue; }
            59 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 61; continue; } bb = 60; continue; }
            60 => { rdx = (r12) as u64; rcx = (r13) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P19) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 61; continue; }
            61 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 121; continue; } bb = 62; continue; }
            62 => { rsi = ((crate::simd_ops::ru8(rsp.wrapping_add(0x78u64))) as u32) as u32 as u64; bb = 63; continue; }
            63 => { rax = (crate::simd_ops::ru32(r13)) as u32 as u64; r15 = ((r13.wrapping_add(0xb0u64)) as u64) as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x40u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x38u64), (rax) as u64); rax = (crate::simd_ops::ru64(r13.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rax) as u64); rax = (crate::simd_ops::ru64(r13.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), (rax) as u64); rax = (crate::simd_ops::ru32(r13.wrapping_add(0x34u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0xcu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(r13.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), (rax) as u64); rax = (crate::simd_ops::ru32(r13.wrapping_add(0x30u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x10u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(r13.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), (0x0u64) as u64); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 65; continue; } bb = 64; continue; }
            64 => { fl_l = (crate::simd_ops::lk_add_u32(rax, (1) as u32)) as u32 as u64; bb = 65; continue; }
            65 => { xmm0 = (crate::simd_ops::ru128(r13.wrapping_add(0x38u64))) as u128; crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), (rax) as u64); rcx = ((rbp.wrapping_add(0x70u64)) as u64) as u64; rax = (crate::simd_ops::ru32(r13.wrapping_add(0x80u64))) as u32 as u64; xmm1 = (crate::simd_ops::ru128(r13.wrapping_add(0x48u64))) as u128; crate::simd_ops::wu32(rbp.wrapping_add(0x40u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r13.wrapping_add(0x84u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x44u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(r13.wrapping_add(0x88u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x48u64), (rax) as u64); rax = (crate::simd_ops::ru32(r13.wrapping_add(0x90u64))) as u32 as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x8u64), (xmm0) as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x50u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r13.wrapping_add(0x94u64))) as u32 as u64; xmm0 = (crate::simd_ops::ru128(r13.wrapping_add(0x58u64))) as u128; crate::simd_ops::wu32(rbp.wrapping_add(0x54u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r13.wrapping_add(0xa0u64))) as u32 as u64; crate::simd_ops::wu128(rbp.wrapping_add(0x18u64), (xmm0) as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x60u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(r13.wrapping_add(0x78u64))) as u64) as u128; rax = (crate::simd_ops::ru64(r13.wrapping_add(0xa8u64))) as u64; crate::simd_ops::wu128(rbp.wrapping_add(0x8u64), (xmm1) as u128); crate::simd_ops::wu64(rbp.wrapping_add(0x68u64), (rax) as u64); xmm1 = (crate::simd_ops::ru128(r13.wrapping_add(0x68u64))) as u128; crate::simd_ops::wu64(rbp.wrapping_add(0x38u64), ((xmm0 as u64)) as u64); xmm0 = ((crate::simd_ops::ru64(r13.wrapping_add(0x98u64))) as u64) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x28u64), (xmm1) as u128); crate::simd_ops::wu64(rbp.wrapping_add(0x58u64), ((xmm0 as u64)) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P20) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = ((rbp.wrapping_add(0x70u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x128u64), (rax) as u64); rdx = (r15) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x130u64), (rax) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x138u64), (rax) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x140u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x148u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x290u64), (rax) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x298u64), (rax) as u64); rax = (crate::simd_ops::ru64(r15.wrapping_add(0x230u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x2a0u64), (rax) as u64); rax = (crate::simd_ops::ru64(r15.wrapping_add(0x238u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x2a8u64), (rax) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P21) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(r13.wrapping_add(0x2f0u64))) as u64; rcx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x2b8u64))) as u32) as u32 as u64; r15 = (crate::simd_ops::ru32(rbx)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x2b0u64), (rax) as u64); rax = (((rcx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) ^ (crate::simd_ops::ru8(r13.wrapping_add(0x2f8u64))))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xfu8))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) ^ ((rax as u8)))) as u8 as u64)); rax = (crate::simd_ops::ru64(r12.wrapping_add(0x168u64))) as u64; crate::simd_ops::wu8(rbp.wrapping_add(0x2b8u64), ((rcx as u8)) as u8); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 68; continue; } bb = 66; continue; }
            66 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 85; continue; } bb = 67; continue; }
            67 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 66; continue; } bb = 68; continue; }
            68 => { fl_l = ((((rsi as u8)) & (0xe0u8))) as u8 as u64; fl_r = fl_l; rsi = (rsi & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); bb = 69; continue; }
            69 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x310u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rsi as u32)) as u32); if (((fl_l as u8)) != ((fl_r as u8))) { bb = 71; continue; } bb = 70; continue; }
            70 => { fl_l = ((rsi as u8)) as u8 as u64; fl_r = (0x8u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 116; continue; } bb = 71; continue; }
            71 => { fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x50u64))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 73; continue; } bb = 72; continue; }
            72 => { fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x318u64))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 116; continue; } bb = 73; continue; }
            73 => { rdi = (crate::simd_ops::ru64(P22)) as u64; bb = 74; continue; }
            74 => { rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 87; continue; } bb = 75; continue; }
            75 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rdi, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 74; continue; } bb = 76; continue; }
            76 => { r14 = ((rdi.wrapping_add(0x8u64)) as u64) as u64; bb = 77; continue; }
            77 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x48u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 90; continue; } bb = 78; continue; }
            78 => { rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rax = ((r15 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 82; continue; } bb = 79; continue; }
            79 => { bb = 80; continue; }
            80 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 89; continue; } bb = 81; continue; }
            81 => { rsi = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 80; continue; } bb = 82; continue; }
            82 => { r15 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rsi = ((r15 as u32)) as u32 as u64; bb = 83; continue; }
            83 => { fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 91; continue; } bb = 84; continue; }
            84 => { fl_l = (crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 91; continue; }
            85 => { fl_l = ((rax).wrapping_add(0xcu64)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 68; continue; } bb = 86; continue; }
            86 => { rsi = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; bb = 69; continue; }
            87 => { r14 = ((rdi.wrapping_add(0x8u64)) as u64) as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P23)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 77; continue; } bb = 88; continue; }
            88 => { rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P24)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 74; continue; }
            89 => { r15 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 83; continue; }
            90 => { r15 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rsi = ((r15 as u32)) as u32 as u64; bb = 91; continue; }
            91 => { rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 98; continue; } bb = 92; continue; }
            92 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rdi, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 95; continue; } bb = 93; continue; }
            93 => { rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 92; continue; } bb = 94; continue; }
            94 => { bb = 99; continue; }
            95 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 100; continue; } bb = 96; continue; }
            96 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 100; continue; } bb = 97; continue; }
            97 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x38u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P25)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 100; continue; }
            98 => { r14 = ((rdi.wrapping_add(0x8u64)) as u64) as u64; bb = 99; continue; }
            99 => { rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P24)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 100; continue; }
            100 => { fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; rdi = ((rsi.wrapping_sub(0x10u64)) as u64) as u64; rdi = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r15 } else { rdi }) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 115; continue; } bb = 101; continue; }
            101 => { fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x328u64))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 103; continue; } bb = 102; continue; }
            102 => { rdx = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P26) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r14 = ((rax as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x320u64), ((rax as u32)) as u32); bb = 112; continue; }
            103 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x48u64))) as u64; rcx = ((rbp.wrapping_sub(0x80u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P27) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 105; continue; } bb = 104; continue; }
            104 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x48u64))) as u64; r8 = ((rbp.wrapping_sub(0x68u64)) as u64) as u64; rcx = (r12) as u64; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P28) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 109; continue; }
            105 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P29) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x68u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x68u64), (rax) as u64); fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 107; continue; } bb = 106; continue; }
            106 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P30) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x68u64))) as u64; bb = 107; continue; }
            107 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 110; continue; } bb = 108; continue; }
            108 => { r9 = ((rsp.wrapping_add(0x38u64)) as u64) as u64; rcx = (r12) as u64; r8 = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; rdx = ((rbp.wrapping_sub(0x80u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P31) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rbp.wrapping_add(0x320u64), ((rax as u32)) as u32); r14 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 111; continue; } bb = 109; continue; }
            109 => { rax = ((rbp.wrapping_sub(0x80u64)) as u64) as u64; rcx = (rdi) as u64; rdx = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x48u64), (rax) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P26) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r14 = ((rax as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x320u64), ((rax as u32)) as u32); bb = 111; continue; }
            110 => { r14 = (0x34u32) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x320u64), ((r14 as u32)) as u32); bb = 111; continue; }
            111 => { rcx = ((rbp.wrapping_sub(0x80u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P32) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 112; continue; }
            112 => { r15 = (crate::simd_ops::ru32(rbp.wrapping_add(0x318u64))) as u32 as u64; rcx = ((rdi.wrapping_add(0x10u64)) as u64) as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rax = ((r15.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = (if (((fl_l as u32)) != ((fl_r as u32))) { (r15 as u32) } else { (rax as u32) }) as u32 as u64; r15 = ((rax as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x318u64), ((rax as u32)) as u32); rax = (0xffffffffu32) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 114; continue; } bb = 113; continue; }
            113 => { rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rbp.wrapping_add(0x318u64), ((r15 as u32)) as u32); bb = 114; continue; }
            114 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 117; continue; }
            115 => { r14 = (crate::simd_ops::ru32(rbp.wrapping_add(0x320u64))) as u32 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 116; continue; }
            116 => { r15 = (crate::simd_ops::ru32(rbp.wrapping_add(0x318u64))) as u32 as u64; bb = 117; continue; }
            117 => { rcx = ((rbp.wrapping_add(0x70u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P33) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x20u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 119; continue; } bb = 118; continue; }
            118 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P34) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 119; continue; }
            119 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x4u64)) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rsi = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 63; continue; } bb = 120; continue; }
            120 => { rsi = (crate::simd_ops::ru32(rbp.wrapping_add(0x328u64))) as u32 as u64; bb = 121; continue; }
            121 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 128; continue; } bb = 122; continue; }
            122 => { fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 125; continue; } bb = 123; continue; }
            123 => { fl_l = (crate::simd_ops::ru8(r12.wrapping_add(0xd6u64))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 125; continue; } bb = 124; continue; }
            124 => { rdx = (r13) as u64; rcx = (r12) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P35) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 127; continue; }
            125 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 128; continue; } bb = 126; continue; }
            126 => { rdx = (rcx) as u64; rcx = (r12) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P36) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 127; continue; }
            127 => { r14 = ((rax as u32)) as u32 as u64; bb = 128; continue; }
            128 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 129; continue; }
            129 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P30) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 10; continue; }
            130 => { rax = ((r14 as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x3e8u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14161d110(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f88; const P1: u64 = 0x1426452b0; const P2: u64 = 0x1426452a8; const P3: u64 = 0x142645560; const P4: u64 = 0x14158f5a0; const P5: u64 = 0x1434268d0; const P6: u64 = 0x1415864b0; const P7: u64 = 0x1415945c0; const P8: u64 = 0x1415bfb60; const P9: u64 = 0x1415c0d60; const P10: u64 = 0x14152ede0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 113>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rax.wrapping_sub(0x288); rsp = ((rsp).wrapping_sub(0x360u64)) as u64; rbx = (crate::simd_ops::ru64(P0)) as u64; r13 = r8; r15 = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rsi); r12 = rcx; bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 12; continue; } bb = 2; continue; }
            2 => { r9 = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (r9 as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rsi = rbx.wrapping_add(0x8); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32 as u64; r14 = 0; fl_l = 0; fl_r = 0; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 5; continue; }
            5 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r15 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 9; continue; }
            9 => { rdi = r14; bb = 10; continue; }
            10 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 15; continue; }
            12 => { rsi = rbx.wrapping_add(0x8); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 4; continue; } bb = 13; continue; }
            13 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 1; continue; }
            14 => { rdi = r14; bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 22; continue; } bb = 16; continue; }
            16 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 19; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 16; continue; } bb = 18; continue; }
            18 => { bb = 23; continue; }
            19 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 24; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 24; continue; } bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 24; continue; }
            22 => { rsi = rbx.wrapping_add(0x8); bb = 23; continue; }
            23 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 24; continue; }
            24 => { fl_l = rdi; fl_r = rdi; rbx = rdi.wrapping_sub(0x10); rbx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r14 } else { rbx }) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 43; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(r12.wrapping_add(0x168u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), (0x4u32) as u32); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 26; continue; }
            26 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 44; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 26; continue; } bb = 28; continue; }
            28 => { rcx = ((r14 as u32)) as u32 as u64; bb = 29; continue; }
            29 => { xmm0 = (crate::simd_ops::ru128(r13.wrapping_add(0x60u64))) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rcx as u32)) as u32); rcx = rbp.wrapping_sub(0x10); xmm1 = (crate::simd_ops::ru128(r13.wrapping_add(0x70u64))) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r14); crate::simd_ops::wu128(rsp.wrapping_add(0x78u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (0x0u64) as u64); xmm0 = (crate::simd_ops::ru128(r13.wrapping_add(0x80u64))) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r14); crate::simd_ops::wu128(rbp.wrapping_sub(0x78u64), (xmm1) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r14); xmm1 = (crate::simd_ops::ru128(r13.wrapping_add(0x90u64))) as u128; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (0x0u64) as u64); crate::simd_ops::wu128(rbp.wrapping_sub(0x68u64), (xmm0) as u128); crate::simd_ops::wu16(rbp.wrapping_sub(0x28u64), (0xff00u16) as u16); xmm0 = ((crate::simd_ops::ru64(r13.wrapping_add(0xa0u64))) as u64) as u128; crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu128(rbp.wrapping_sub(0x58u64), (xmm1) as u128); crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), r14); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru8(rbp.wrapping_add(0xa7u64))) as u32) as u32 as u64; rdi = rbx.wrapping_add(0x10); crate::simd_ops::lk_and_u8(rbp.wrapping_add(0x234u64), (0xfeu8) as u8); rax = 0; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(r13.wrapping_add(0x18u64))) as u64; xmm0 = 0u128; crate::simd_ops::wu64(rbp.wrapping_add(0x50u64), rax); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0xfdu8))) as u8 as u64)); crate::simd_ops::wu64(rbp.wrapping_add(0x98u64), rax); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0xfau8))) as u8 as u64)); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0xa6u64))) as u32) as u32 as u64; xmm1 = 0u128; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x21u8))) as u8 as u64)); crate::simd_ops::wu8(rbp.wrapping_add(0xa7u64), ((rcx as u8)) as u8); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xdeu8))) as u8 as u64)); crate::simd_ops::wu8(rbp.wrapping_add(0xa6u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x238u64))) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xf3u8))) as u8 as u64)); crate::simd_ops::wu64(rbp.wrapping_add(0xa8u64), r14); rax = (rax & !0xffu64) | ((((((rax as u8)) | (0x3u8))) as u8 as u64)); crate::simd_ops::wu64(rbp.wrapping_add(0xb0u64), (0x0u64) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x238u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(r13.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); rax = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); rax = (crate::simd_ops::ru32(r13.wrapping_add(0x2cu64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x74u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(r13.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rax); rax = (crate::simd_ops::ru32(r13.wrapping_add(0x28u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x210u64), r14); crate::simd_ops::wu64(rbp.wrapping_add(0x218u64), (0x0u64) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x60u64), ((r14 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0xa0u64), ((r14 as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x6cu64), (0x0u64) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x74u64), (0x0u64) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x7cu64), ((r14 as u32)) as u32); crate::simd_ops::wu16(rbp.wrapping_add(0xa4u64), ((r14 as u16)) as u16); crate::simd_ops::wu32(rbp.wrapping_add(0x230u64), ((r14 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((r14 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); crate::simd_ops::wu128(rbp.wrapping_sub(0x10u64), (xmm0) as u128); crate::simd_ops::wu128(rbp, (xmm0) as u128); crate::simd_ops::wu128(rbp.wrapping_add(0x10u64), (xmm0) as u128); crate::simd_ops::wu128(rbp.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::wu128(rbp.wrapping_add(0x30u64), (xmm0) as u128); crate::simd_ops::wu128(rbp.wrapping_add(0x40u64), (xmm0) as u128); xmm0 = ((crate::simd_ops::ru32(P5)) as u32) as u128; crate::simd_ops::wu32(rbp.wrapping_add(0x68u64), ((xmm0 as u32)) as u32); xmm0 = 0u128; crate::simd_ops::wu32(rbp.wrapping_add(0x250u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0xc8u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu128(rbp.wrapping_add(0x220u64), (xmm0) as u128); crate::simd_ops::wu128(rbp.wrapping_add(0x240u64), (xmm1) as u128); crate::simd_ops::wu128(rbp.wrapping_add(0xb8u64), (xmm1) as u128); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 30; continue; }
            30 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 31; continue; }
            31 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 32; continue; }
            32 => { fl_l = (crate::simd_ops::lk_add_u32(rsi, (1) as u32)) as u32 as u64; bb = 33; continue; }
            33 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_sub(0x28u64))) as u8 as u64; fl_r = (0x90u8) as u8 as u64; rax = (crate::simd_ops::ru32(r13.wrapping_add(0xa8u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x40u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rsi); crate::simd_ops::wu32(rbp.wrapping_sub(0x3cu64), ((r14 as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x38u64), r14); crate::simd_ops::wu32(rbp.wrapping_sub(0x30u64), ((r14 as u32)) as u32); if (((fl_l as u8)) != ((fl_r as u8))) { bb = 37; continue; } bb = 34; continue; }
            34 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_sub(0x23u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) <= ((fl_r as u8))) { bb = 37; continue; } bb = 35; continue; }
            35 => { rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x238u64))) as u32) as u32 as u64; rcx = rsp.wrapping_add(0x40); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x1u8))) as u8 as u64)); crate::simd_ops::wu8(rbp.wrapping_add(0x2a8u64), ((rax as u8)) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = ((rax as u32)) as u32 as u64; r9 = rbp.wrapping_add(0x2a8); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x74u64))) as u32 as u64; rdx = rbp.wrapping_sub(0x28); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); rcx = rbx; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 39; continue; } bb = 36; continue; }
            36 => { rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x238u64))) as u32) as u32 as u64; rdx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x2a8u64))) as u32) as u32 as u64; rdx = (rdx & !0xffu64) | ((((((rdx as u8)) ^ ((rax as u8)))) as u8 as u64)); rdx = (rdx & !0xffu64) | ((((((rdx as u8)) & (0x1u8))) as u8 as u64)); fl_l = ((((rax as u8)) ^ ((rdx as u8)))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbp.wrapping_add(0x238u64), ((rax as u8)) as u8); bb = 37; continue; }
            37 => { rdx = rsp.wrapping_add(0x40); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 39; continue; } bb = 38; continue; }
            38 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = rsp.wrapping_add(0x40); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x250u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 39; continue; }
            39 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 41; continue; } bb = 40; continue; }
            40 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 41; continue; }
            41 => { rcx = rbp.wrapping_sub(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 43; continue; } bb = 42; continue; }
            42 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 43; continue; }
            43 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x390u64))) as u64; r11 = rsp.wrapping_add(0x360); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            44 => { fl_l = ((rax).wrapping_add(0xcu64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 28; continue; } bb = 45; continue; }
            45 => { rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as u32 as u64; bb = 29; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14161d500(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931100; const P1: u64 = 0x1415c5720; const P2: u64 = 0x14161d720; const P3: u64 = 0x1415c5750; const P4: u64 = 0x1415a36b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rdi = rcx; rbp = 0x1; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 10; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0xf8u64))) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x100u64))) as u32 as u64; rdx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 35; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x128u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbp = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x100u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0xf8u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 5; continue; }
            5 => { rax = ((rbp as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0xf8u64))) as u64; fl_l = rdi; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 35; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x130u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = ((rdi).wrapping_add(0x8u64)) as u64; fl_l = rdi; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { rax = ((rbp as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            10 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x128u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r15); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 33; continue; } bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0x130u64))) as u32 as u64; rbx = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x134u64))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; r8 = ((rbx as u32)) as u32 as u64; r8 = (r8 & !0xffu64) | (((if (((fl_l as u8)) != ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64)); rcx = ((rcx).wrapping_add(0x118u64)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r14 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 12; continue; }
            12 => { rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x140u64))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 13; continue; }
            13 => { bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(r14)) as u64; rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 18; continue; } bb = 16; continue; }
            16 => { rax = ((rax).wrapping_add(0x4u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 17; continue; }
            17 => { bb = 19; continue; }
            18 => { rdx = rsi.wrapping_add(0x10); rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbp = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 22; continue; } bb = 19; continue; }
            19 => { rsi = (crate::simd_ops::ru64(rsi)) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 20; continue; }
            20 => { crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x128u64), (1) as u32); rcx = rdi.wrapping_add(0x118); fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x134u64))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x130u64))) as u32 as u64; rbx = (rbx & !0xffu64) | (((if (((fl_l as u8)) != ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64)); r8 = ((rbx as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbp = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 34; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x130u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 34; continue; }
            22 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x140u64))) as u64; fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 34; continue; } bb = 23; continue; }
            23 => { bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru64(r14)) as u64; rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 30; continue; } bb = 25; continue; }
            25 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 28; continue; } bb = 26; continue; }
            26 => { rax = ((rax).wrapping_add(0x4u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 25; continue; } bb = 27; continue; }
            27 => { bb = 30; continue; }
            28 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 30; continue; } bb = 29; continue; }
            29 => { rcx = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x4u64)) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 29; continue; } bb = 30; continue; }
            30 => { rdi = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 24; continue; } bb = 31; continue; }
            31 => { bb = 34; continue; }
            32 => { rbp = 0x34; bb = 34; continue; }
            33 => { fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x128u64), ((rax as u32)) as u32); bb = 34; continue; }
            34 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 35; continue; }
            35 => { rax = ((rbp as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14161d720(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x1415a0b00; const P3: u64 = 0x1415a36b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rdi = rdx; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rsi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; rbx = ((rbx).wrapping_add(0x4u64)) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = ((rsi as u32)) as u32 as u64; bb = 5; continue; }
            5 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; fl_l = rdi; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru32(rdi)) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdi = ((rdi).wrapping_add(0x4u64)) as u64; fl_l = rdi; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { rax = ((rsi as u32)) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = 0x1; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14161d7c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415355f0; const P1: u64 = 0x14183acc8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru32(rdx.wrapping_add(0x20u64))) as u32 as u64; r9 = (r9 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; r8 = rdx; rbx = rcx; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x78u64))) as u64; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = r8; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0xf0); rdx = ((rdi as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x100u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0xf8u64))) as u64; rdx = (((rdx as u32)).wrapping_sub(1)) as u32 as u64; r8 = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { r8 = ((r8).wrapping_sub(rax)) as u64; rdx = rax.wrapping_add(0x8); r8 = ((((r8 as u32)) & (0xfffffff8u32))) as u32 as u64; rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::lk_sub_u32(rbx.wrapping_add(0x100u64), (1) as u32)) as u32 as u64; bb = 5; continue; }
            5 => { r9 = (r9 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 6; continue; }
            6 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x168u64))) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = r8; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rdx = rcx; rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 10; continue; }
            10 => { bb = 17; continue; }
            11 => { fl_l = rcx; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rbx.wrapping_add(0x168u64), rax); bb = 14; continue; }
            13 => { crate::simd_ops::wu64(rdx, rax); bb = 14; continue; }
            14 => { fl_l = rcx; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x170u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rbx.wrapping_add(0x170u64), rdx); bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x178u64))) as u64; crate::simd_ops::wu64(rcx, rax); fl_l = (crate::simd_ops::lk_sub_u32(rbx.wrapping_add(0x18cu64), (1) as u32)) as u32 as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x178u64), rcx); bb = 17; continue; }
            17 => { fl_l = ((r9 as u8)) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 20; continue; } bb = 18; continue; }
            18 => { rcx = rbx.wrapping_add(0x10); rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            20 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14161d8c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f00; const P1: u64 = 0x1401de410; const P2: u64 = 0x1415a2910; const P3: u64 = 0x143930fa0; const P4: u64 = 0x1415c53e0; const P5: u64 = 0x1415c4950; const P6: u64 = 0x14153be40; const P7: u64 = 0x141524330; const P8: u64 = 0x14161c2f0; const P9: u64 = 0x14161b2b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu32(rsp.wrapping_add(0x18u64), ((r8 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rbp = rsp; rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rsi = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x70u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsi; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rbx = (crate::simd_ops::ru32(rsi.wrapping_add(0x20u64))) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x70u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(P0)) as u64; r8 = 0x4; r9 = ((rax as u32)) as u32 as u64; rdx = ((rbx as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0x5b; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { crate::simd_ops::lk_add_u64(rbp.wrapping_add(0x28u64), (0x4u64) as u64); r8 = rbp.wrapping_add(0x30); r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r15); rdx = rbp.wrapping_add(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 84; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r14); rcx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); r14 = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = ((rax).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x20u64), ((r14 as u32)) as u32); r12 = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = ((rax).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; rcx = ((crate::simd_ops::ru8(rsi.wrapping_add(0x134u64))) as u32) as u32 as u64; r15 = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; r9 = rax.wrapping_add(0x1); rax = (rax & !0xffu64) | (((if (((fl_l as u8)) == ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), r9); rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_add(0x2u8)) as u8 as u64)); rcx = ((rcx).wrapping_shl(32u32)) as u64; crate::simd_ops::wu8(rbp.wrapping_add(0x24u64), ((rax as u8)) as u8); rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x20u64))) as u64; rax = rdi; rax = ((rax).wrapping_shr(32u32)) as u64; rdx = (((rax as u8)) as u32) as u32 as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x130u64))) as u32 as u64; rdx = ((rdx).wrapping_shl(32u32)) as u64; rcx = (((rcx) | (rax))) as u64; rdx = (((rdx) | (r14))) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x198u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x130u64))) as u64; r8 = rbx; rcx = (crate::simd_ops::ru64(P3)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(P3)) as u64; r8 = rbx; rdx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 8; continue; }
            8 => { r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; bb = 9; continue; }
            9 => { rax = ((crate::simd_ops::ru8(rsi.wrapping_add(0xd6u64))) as u32) as u32 as u64; fl_l = 0; fl_r = ((r15 as u8)) as u8 as u64; r15 = (r15 & !0xffu64) | (((0u8.wrapping_sub(fl_r as u8)) as u8 as u64)); crate::simd_ops::wu64(rsi.wrapping_add(0x130u64), rdi); rbx = 0x1; rcx = (rcx & !0xffu64) | (((if (((fl_l as u8)) < ((fl_r as u8))) { 0xffu8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu32(rsi.wrapping_add(0x138u64), ((r12 as u32)) as u32); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0x2u8))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xfdu8))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) | ((rax as u8)))) as u8 as u64)); crate::simd_ops::wu8(rsi.wrapping_add(0xd6u64), ((rcx as u8)) as u8); r14 = (crate::simd_ops::ru32(r9)) as u32 as u64; r9 = ((r9).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), r9); fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 20; continue; } bb = 10; continue; }
            10 => { rcx = rsi.wrapping_add(0xf0); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x14u64))) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 13; continue; } bb = 11; continue; }
            11 => { rdx = ((r14 as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_sub((rax as u32))) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; r15 = 0x34; r15 = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (rbx as u32) } else { (r15 as u32) }) as u32 as u64; rbx = ((r15 as u32)) as u32 as u64; fl_l = ((r15 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 83; continue; } bb = 12; continue; }
            12 => { r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; bb = 13; continue; }
            13 => { rdi = 0; fl_l = 0; fl_r = 0; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 20; continue; } bb = 14; continue; }
            14 => { bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru32(r9)) as u32 as u64; rcx = rsi; crate::simd_ops::wu32(rbp.wrapping_add(0x20u64), ((rax as u32)) as u32); r9 = ((r9).wrapping_add(0x4u64)) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu8(rbp.wrapping_add(0x24u64), (0x0u8) as u8); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), r9); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xf; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 19; continue; } bb = 17; continue; }
            17 => { rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 19; continue; } bb = 18; continue; }
            18 => { r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; bb = 15; continue; }
            19 => { r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; bb = 20; continue; }
            20 => { r15 = ((rbx as u32)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 83; continue; } bb = 21; continue; }
            21 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r13); r13 = (crate::simd_ops::ru32(r9)) as u32 as u64; r9 = ((r9).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), r9); fl_l = ((r13 as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 30; continue; } bb = 22; continue; }
            22 => { rdx = ((r13.wrapping_mul(4)) as u64) as u64; rcx = 0; fl_l = 0; fl_r = 0; rdx = ((rdx).wrapping_add(r13)) as u64; rdx = ((rdx).wrapping_shl(3u32)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsi.wrapping_add(0x158u64), rax); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 23; continue; }
            23 => { crate::simd_ops::wu32(rsi.wrapping_add(0x160u64), ((r13 as u32)) as u32); r8 = ((r14 as u32)) as u32 as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x150u64), rax); fl_l = ((r13 as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 28; continue; } bb = 24; continue; }
            24 => { rcx = rax.wrapping_add(0x20); bb = 25; continue; }
            25 => { rax = rcx.wrapping_sub(0x18); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 26; continue; }
            26 => { crate::simd_ops::wu64(rcx.wrapping_sub(0x8u64), r14); crate::simd_ops::wu64(rcx, r14); bb = 27; continue; }
            27 => { rax = rcx.wrapping_add(0x8); r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu64(rdx, rax); rcx = ((rcx).wrapping_add(0x28u64)) as u64; rdx = ((rdx).wrapping_add(0x28u64)) as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsi.wrapping_add(0x160u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 25; continue; } bb = 28; continue; }
            28 => { crate::simd_ops::wu64(rdx.wrapping_sub(0x28u64), r14); bb = 29; continue; }
            29 => { r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; bb = 31; continue; }
            30 => { r14 = 0; fl_l = 0; fl_r = 0; bb = 31; continue; }
            31 => { r12 = ((r14 as u32)) as u32 as u64; fl_l = ((r13 as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 57; continue; } bb = 32; continue; }
            32 => { bb = 34; continue; }
            33 => { r14 = 0; fl_l = 0; fl_r = 0; bb = 34; continue; }
            34 => { rbx = (crate::simd_ops::ru32(r9)) as u32 as u64; r9 = ((r9).wrapping_add(0x4u64)) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x140u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), r9); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 37; continue; } bb = 35; continue; }
            35 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 45; continue; } bb = 36; continue; }
            36 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 35; continue; } bb = 37; continue; }
            37 => { rdi = r14; bb = 38; continue; }
            38 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x150u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 43; continue; } bb = 39; continue; }
            39 => { rdx = ((rax.wrapping_add(0x28u64)) as u32) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 50; continue; } bb = 40; continue; }
            40 => { rcx = rax.wrapping_add(0x8); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 42; continue; } bb = 41; continue; }
            41 => { crate::simd_ops::wu64(rax.wrapping_add(0x18u64), r14); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), (0x0u64) as u64); bb = 42; continue; }
            42 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x150u64))) as u64; crate::simd_ops::wu64(rax, rcx); crate::simd_ops::wu64(rsi.wrapping_add(0x150u64), rax); bb = 43; continue; }
            43 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x148u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 47; continue; } bb = 44; continue; }
            44 => { crate::simd_ops::wu64(rsi.wrapping_add(0x140u64), rax); bb = 48; continue; }
            45 => { rdi = rax.wrapping_add(0x10); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 51; continue; } bb = 46; continue; }
            46 => { bb = 38; continue; }
            47 => { crate::simd_ops::wu64(rcx, rax); bb = 48; continue; }
            48 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x150u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x148u64), rcx); rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x150u64), rax); crate::simd_ops::wu64(rcx, r14); crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x164u64), (1) as u32); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x148u64))) as u64; fl_l = ((rax).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 50; continue; } bb = 49; continue; }
            49 => { crate::simd_ops::wu32(rax, ((rbx as u32)) as u32); rdi = rax.wrapping_add(0x8); bb = 50; continue; }
            50 => { r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 81; continue; } bb = 51; continue; }
            51 => { r14 = (crate::simd_ops::ru32(r9)) as u32 as u64; r9 = ((r9).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), r9); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 53; continue; } bb = 52; continue; }
            52 => { rdx = ((r14 as u32)) as u32 as u64; rcx = rdi; rdx = (((rdx as u32)).wrapping_sub((rax as u32))) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; bb = 53; continue; }
            53 => { rbx = 0; fl_l = 0; fl_r = 0; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 56; continue; } bb = 54; continue; }
            54 => { r8 = (crate::simd_ops::ru32(r9)) as u32 as u64; rdx = rdi; r9 = ((r9).wrapping_add(0x4u64)) as u64; rcx = rsi; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), r9); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 82; continue; } bb = 55; continue; }
            55 => { r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 54; continue; } bb = 56; continue; }
            56 => { r12 = (((r12 as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 33; continue; } bb = 57; continue; }
            57 => { r14 = (crate::simd_ops::ru32(r9)) as u32 as u64; r9 = ((r9).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), r9); fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 64; continue; } bb = 58; continue; }
            58 => { rdx = ((r14.wrapping_add(r14.wrapping_mul(2))) as u64) as u64; rcx = 0; fl_l = 0; fl_r = 0; rdx = ((rdx).wrapping_shl(3u32)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(0x180u64), rax); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 63; continue; } bb = 59; continue; }
            59 => { rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsi.wrapping_add(0x188u64), ((r14 as u32)) as u32); crate::simd_ops::wu64(rsi.wrapping_add(0x178u64), rax); fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 62; continue; } bb = 60; continue; }
            60 => { bb = 61; continue; }
            61 => { rax = rcx.wrapping_add(0x18); rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu64(rcx, rax); rcx = rax; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsi.wrapping_add(0x188u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 61; continue; } bb = 62; continue; }
            62 => { crate::simd_ops::wu64(rcx.wrapping_sub(0x18u64), (0x0u64) as u64); bb = 63; continue; }
            63 => { r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; bb = 64; continue; }
            64 => { rdi = 0; fl_l = 0; fl_r = 0; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 82; continue; } bb = 65; continue; }
            65 => { r12 = ((crate::simd_ops::ru8(rbp.wrapping_sub(0x28u64))) as u32) as u32 as u64; bb = 66; continue; }
            66 => { rbx = (crate::simd_ops::ru32(r9)) as u32 as u64; r9 = ((r9).wrapping_add(0x4u64)) as u64; rax = r9.wrapping_add(0x1); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), r9); r8 = rax.wrapping_add(0x1); rcx = ((crate::simd_ops::ru8(r9)) as u32) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0x3u8))) as u8 as u64)); rdx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rax = (((r12 as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xe0u8))) as u8 as u64)); rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_shl(3u32)) as u8 as u64)); rdx = (rdx & !0xffu64) | ((((((rdx as u8)) & (0x7u8))) as u8 as u64)); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), r8); r12 = (((rcx as u8)) as u32) as u32 as u64; r12 = (r12 & !0xffu64) | ((((((r12 as u8)) | ((rax as u8)))) as u8 as u64)); rax = (crate::simd_ops::ru32(r8)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x30u64), ((rax as u32)) as u32); r12 = (r12 & !0xffu64) | ((((((r12 as u8)) | ((rdx as u8)))) as u8 as u64)); rdx = r8.wrapping_add(0x4); crate::simd_ops::wu8(rbp.wrapping_sub(0x28u64), ((r12 as u8)) as u8); rax = rdx.wrapping_add(0x4); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rdx); rcx = (crate::simd_ops::ru32(rdx)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); rax = (((r12 as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shr(3u32)) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x1u8))) as u8 as u64)); crate::simd_ops::wu32(rbp.wrapping_sub(0x2cu64), ((rcx as u32)) as u32); crate::simd_ops::lk_or_u8(rsi.wrapping_add(0x1a0u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x168u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 69; continue; } bb = 67; continue; }
            67 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 74; continue; } bb = 68; continue; }
            68 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 67; continue; } bb = 69; continue; }
            69 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x178u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 72; continue; } bb = 70; continue; }
            70 => { rdx = ((rax.wrapping_add(0x18u64)) as u32) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 81; continue; } bb = 71; continue; }
            71 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x178u64))) as u64; crate::simd_ops::wu64(rax, rcx); crate::simd_ops::wu64(rsi.wrapping_add(0x178u64), rax); bb = 72; continue; }
            72 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x170u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 76; continue; } bb = 73; continue; }
            73 => { crate::simd_ops::wu64(rsi.wrapping_add(0x168u64), rax); bb = 77; continue; }
            74 => { rcx = rax.wrapping_add(0xc); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 69; continue; } bb = 75; continue; }
            75 => { xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64))) as u64) as u128; rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x28u64))) as u32 as u64; crate::simd_ops::wu64(rcx, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((rax as u32)) as u32); bb = 79; continue; }
            76 => { crate::simd_ops::wu64(rcx, rax); bb = 77; continue; }
            77 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x178u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x170u64), rcx); rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x178u64), rax); crate::simd_ops::wu64(rcx, (0x0u64) as u64); crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x18cu64), (1) as u32); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x170u64))) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 81; continue; } bb = 78; continue; }
            78 => { xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64))) as u64) as u128; rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x28u64))) as u32 as u64; crate::simd_ops::wu32(rcx, ((rbx as u32)) as u32); fl_l = ((rcx).wrapping_add(0x4u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rcx, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((rax as u32)) as u32); if (((fl_l as u64)) == 0) { bb = 81; continue; } bb = 79; continue; }
            79 => { rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 82; continue; } bb = 80; continue; }
            80 => { r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; bb = 66; continue; }
            81 => { r15 = 0x34; bb = 82; continue; }
            82 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 83; continue; }
            83 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 84; continue; }
            84 => { rax = ((r15 as u32)) as u32 as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14161de80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f88; const P1: u64 = 0x1426452b0; const P2: u64 = 0x1426452a8; const P3: u64 = 0x142645560; const P4: u64 = 0x143930f08; const P5: u64 = 0x1415ab5b0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rbx = (crate::simd_ops::ru64(P0)) as u64; r15 = r9; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rdi); r12 = r8; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), r14); r13 = rcx; r14 = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rsi); bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 11; continue; } bb = 2; continue; }
            2 => { r10 = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (r10 as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rsi = rbx.wrapping_add(0x8); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32 as u64; rbp = 0; fl_l = 0; fl_r = 0; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 13; continue; } bb = 5; continue; }
            5 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r14 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 8; continue; }
            8 => { rdi = rbp; bb = 9; continue; }
            9 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 14; continue; }
            11 => { rsi = rbx.wrapping_add(0x8); rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 4; continue; } bb = 12; continue; }
            12 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            13 => { rdi = rbp; bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 22; continue; } bb = 15; continue; }
            15 => { bb = 16; continue; }
            16 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 19; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 16; continue; } bb = 18; continue; }
            18 => { bb = 23; continue; }
            19 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 24; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 24; continue; } bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 24; continue; }
            22 => { rsi = rbx.wrapping_add(0x8); bb = 23; continue; }
            23 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 24; continue; }
            24 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rbx = rdi.wrapping_sub(0x10); fl_l = rdi; fl_r = rdi; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rbx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbp } else { rbx }) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 25; continue; }
            25 => { rcx = (crate::simd_ops::ru64(P4)) as u64; r9 = 0; fl_l = 0; fl_r = 0; r8 = r15; rdx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = ((crate::simd_ops::ru8(r12.wrapping_add(0x8u64))) as u32) as u32 as u64; rcx = rbx; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 27; continue; } bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xa0u64))) as u32 as u64; rdx = rsp.wrapping_add(0x20); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r12)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x3fu64), (0x0u16) as u16); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbp); crate::simd_ops::wu16(rsp.wrapping_add(0x3cu64), (0x0u16) as u16); crate::simd_ops::wu8(rsp.wrapping_add(0x3eu64), (0x0u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x4u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rbp as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r15); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x80u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; r8 = r15; rcx = (crate::simd_ops::ru64(P4)) as u64; rdx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 28; continue; }
            27 => { rax = (crate::simd_ops::ru64(rbx)) as u64; r9 = 0; fl_l = 0; fl_r = 0; r8 = r13; rdx = r15; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0xa0u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 28; continue; }
            28 => { rcx = rbx.wrapping_add(0x10); rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 30; continue; } bb = 29; continue; }
            29 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 30; continue; }
            30 => { rsp = ((rsp).wrapping_add(0x50u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14161e050(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415242e0; const P1: u64 = 0x141621970;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x140u64))) as u64; rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rdi = rcx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rsi = rbx.wrapping_add(0x10); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu32(rsi.wrapping_add(0x10u64), ((rbp as u32)) as u32); fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rbp); crate::simd_ops::wu32(rsi.wrapping_add(0x14u64), ((rbp as u32)) as u32); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x140u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x140u64), rax); fl_l = rcx; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x148u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rdi.wrapping_add(0x148u64), rbp); bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x150u64))) as u64; crate::simd_ops::wu64(rcx, rax); fl_l = (crate::simd_ops::lk_sub_u32(rdi.wrapping_add(0x164u64), (1) as u32)) as u32 as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x150u64), rcx); bb = 8; continue; }
            12 => { crate::simd_ops::wu64(rdi.wrapping_add(0x198u64), rbp); crate::simd_ops::wu32(rdi.wrapping_add(0x190u64), ((rbp as u32)) as u32); bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x168u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x168u64), rax); fl_l = rcx; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x170u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rdi.wrapping_add(0x170u64), rbp); bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x178u64))) as u64; crate::simd_ops::wu64(rcx, rax); fl_l = (crate::simd_ops::lk_sub_u32(rdi.wrapping_add(0x18cu64), (1) as u32)) as u32 as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x178u64), rcx); bb = 13; continue; }
            17 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x178u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 18; continue; }
            18 => { bb = 19; continue; }
            19 => { r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x180u64))) as u64; rbx = rdi.wrapping_add(0x188); rsi = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 21; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((r8.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 22; continue; } bb = 21; continue; }
            21 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 22; continue; }
            22 => { rdx = rsi; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 19; continue; } bb = 23; continue; }
            23 => { bb = 25; continue; }
            24 => { rbx = rdi.wrapping_add(0x188); bb = 25; continue; }
            25 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x180u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 26; continue; }
            26 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(0x180u64), rbp); bb = 27; continue; }
            27 => { crate::simd_ops::wu32(rbx, ((rbp as u32)) as u32); crate::simd_ops::wu64(rdi.wrapping_add(0x168u64), rbp); crate::simd_ops::wu64(rdi.wrapping_add(0x170u64), rbp); crate::simd_ops::wu64(rdi.wrapping_add(0x178u64), rbp); bb = 28; continue; }
            28 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x140u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 29; continue; }
            29 => { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x140u64), rax); fl_l = rcx; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x148u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 31; continue; } bb = 30; continue; }
            30 => { crate::simd_ops::wu64(rdi.wrapping_add(0x148u64), rbp); bb = 31; continue; }
            31 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x150u64))) as u64; crate::simd_ops::wu64(rcx, rax); fl_l = (crate::simd_ops::lk_sub_u32(rdi.wrapping_add(0x164u64), (1) as u32)) as u32 as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x150u64), rcx); bb = 28; continue; }
            32 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x150u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 39; continue; } bb = 33; continue; }
            33 => { bb = 34; continue; }
            34 => { r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x158u64))) as u64; rbx = rdi.wrapping_add(0x160); rsi = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 36; continue; } bb = 35; continue; }
            35 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((r8.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 37; continue; } bb = 36; continue; }
            36 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 37; continue; }
            37 => { rdx = rsi; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 34; continue; } bb = 38; continue; }
            38 => { bb = 40; continue; }
            39 => { rbx = rdi.wrapping_add(0x160); bb = 40; continue; }
            40 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x158u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 42; continue; } bb = 41; continue; }
            41 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(0x158u64), rbp); bb = 42; continue; }
            42 => { rcx = rdi; crate::simd_ops::wu32(rbx, ((rbp as u32)) as u32); crate::simd_ops::wu64(rdi.wrapping_add(0x140u64), rbp); crate::simd_ops::wu64(rdi.wrapping_add(0x148u64), rbp); crate::simd_ops::wu64(rdi.wrapping_add(0x150u64), rbp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14161e2a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931100; const P1: u64 = 0x1415c5720; const P2: u64 = 0x1415a36b0; const P3: u64 = 0x1415c57b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x28u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rsi = rcx; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0xf8u64))) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x100u64))) as u32 as u64; rdx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x130u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x100u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0xf8u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x128u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 19; continue; } bb = 6; continue; }
            6 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x128u64), ((rax as u32)) as u32); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 19; continue; } bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0x130u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); rbp = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x134u64))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; r8 = ((rbp as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); r8 = (r8 & !0xffu64) | (((if (((fl_l as u8)) != ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); rcx = ((rcx).wrapping_add(0x118u64)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r14 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); rdi = (crate::simd_ops::ru64(rsi.wrapping_add(0x140u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 9; continue; }
            9 => { bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(r14)) as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rax = ((rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 12; continue; }
            12 => { rcx = ((rcx).wrapping_add(0x4u64)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 13; continue; }
            13 => { bb = 16; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x4u64)) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 16; continue; }
            16 => { rdi = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 17; continue; }
            17 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x134u64))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; rcx = rsi.wrapping_add(0x118); rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x130u64))) as u32 as u64; rbp = (rbp & !0xffu64) | (((if (((fl_l as u8)) != ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64)); r8 = ((rbp as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 19; continue; }
            19 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14161e400(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x1415a36b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rdi = rdx;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    rcx = ((rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
    fl_l = rbx; fl_r = rcx;
    if (((fl_l as u64)) == ((fl_r as u64))) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    loop { rcx = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; rbx = ((rbx).wrapping_add(0x4u64)) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; fl_l = rbx; fl_r = rcx; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14161e450(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r9 = 0; fl_l = 0; fl_r = 0; r8 = 0x2; crate::simd_ops::wu64(rcx.wrapping_add(0x4400u64), r9); crate::simd_ops::wu32(rcx.wrapping_add(0x4408u64), ((r9 as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x4410u64), r9); bb = 1; continue; }
            1 => { rdx = ((r8.wrapping_sub(0x2u64)) as u32) as u32 as u64; rax = (((rdx) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x4410u64))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rcx.wrapping_add(0x4410u64), rax); crate::simd_ops::wu64(rax, r9); bb = 4; continue; }
            3 => { crate::simd_ops::wu64(rax, rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x4410u64), rax); bb = 4; continue; }
            4 => { crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x4408u64), (1) as u32); rdx = ((r8.wrapping_sub(0x1u64)) as u32) as u32 as u64; rax = (((rdx) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x4410u64))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rcx.wrapping_add(0x4410u64), rax); crate::simd_ops::wu64(rax, r9); bb = 7; continue; }
            6 => { crate::simd_ops::wu64(rax, rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x4410u64), rax); bb = 7; continue; }
            7 => { crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x4408u64), (1) as u32); rax = ((r8 as u32)) as u32 as u64; rdx = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x4410u64))) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rcx.wrapping_add(0x4410u64), rdx); crate::simd_ops::wu64(rdx, r9); bb = 10; continue; }
            9 => { crate::simd_ops::wu64(rdx, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x4410u64), rdx); bb = 10; continue; }
            10 => { crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x4408u64), (1) as u32); rdx = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = (((rdx) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x4410u64))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rcx.wrapping_add(0x4410u64), rax); crate::simd_ops::wu64(rax, r9); bb = 13; continue; }
            12 => { crate::simd_ops::wu64(rax, rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x4410u64), rax); bb = 13; continue; }
            13 => { crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x4408u64), (1) as u32); rdx = ((r8.wrapping_add(0x2u64)) as u32) as u32 as u64; rax = (((rdx) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x4410u64))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rcx.wrapping_add(0x4410u64), rax); crate::simd_ops::wu64(rax, r9); bb = 16; continue; }
            15 => { crate::simd_ops::wu64(rax, rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x4410u64), rax); bb = 16; continue; }
            16 => { crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x4408u64), (1) as u32); rdx = ((r8.wrapping_add(0x3u64)) as u32) as u32 as u64; rax = (((rdx) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x4410u64))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { crate::simd_ops::wu64(rcx.wrapping_add(0x4410u64), rax); crate::simd_ops::wu64(rax, r9); bb = 19; continue; }
            18 => { crate::simd_ops::wu64(rax, rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x4410u64), rax); bb = 19; continue; }
            19 => { crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x4408u64), (1) as u32); rdx = ((r8.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = (((rdx) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x4410u64))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { crate::simd_ops::wu64(rcx.wrapping_add(0x4410u64), rax); crate::simd_ops::wu64(rax, r9); bb = 22; continue; }
            21 => { crate::simd_ops::wu64(rax, rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x4410u64), rax); bb = 22; continue; }
            22 => { crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x4408u64), (1) as u32); rdx = ((r8.wrapping_add(0x5u64)) as u32) as u32 as u64; rax = (((rdx) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x4410u64))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { crate::simd_ops::wu64(rcx.wrapping_add(0x4410u64), rax); crate::simd_ops::wu64(rax, r9); bb = 25; continue; }
            24 => { crate::simd_ops::wu64(rax, rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x4410u64), rax); bb = 25; continue; }
            25 => { crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x4408u64), (1) as u32); rdx = ((r8.wrapping_add(0x6u64)) as u32) as u32 as u64; rax = (((rdx) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x4410u64))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 27; continue; } bb = 26; continue; }
            26 => { crate::simd_ops::wu64(rcx.wrapping_add(0x4410u64), rax); crate::simd_ops::wu64(rax, r9); bb = 28; continue; }
            27 => { crate::simd_ops::wu64(rax, rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x4410u64), rax); bb = 28; continue; }
            28 => { crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x4408u64), (1) as u32); rdx = ((r8.wrapping_add(0x7u64)) as u32) as u32 as u64; rax = (((rdx) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x4410u64))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 30; continue; } bb = 29; continue; }
            29 => { crate::simd_ops::wu64(rcx.wrapping_add(0x4410u64), rax); crate::simd_ops::wu64(rax, r9); bb = 31; continue; }
            30 => { crate::simd_ops::wu64(rax, rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x4410u64), rax); bb = 31; continue; }
            31 => { crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x4408u64), (1) as u32); rdx = ((r8.wrapping_add(0x8u64)) as u32) as u32 as u64; rax = (((rdx) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x4410u64))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 33; continue; } bb = 32; continue; }
            32 => { crate::simd_ops::wu64(rcx.wrapping_add(0x4410u64), rax); crate::simd_ops::wu64(rax, r9); bb = 34; continue; }
            33 => { crate::simd_ops::wu64(rax, rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x4410u64), rax); bb = 34; continue; }
            34 => { crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x4408u64), (1) as u32); rdx = ((r8.wrapping_add(0x9u64)) as u32) as u32 as u64; rax = (((rdx) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x4410u64))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 36; continue; } bb = 35; continue; }
            35 => { crate::simd_ops::wu64(rcx.wrapping_add(0x4410u64), rax); crate::simd_ops::wu64(rax, r9); bb = 37; continue; }
            36 => { crate::simd_ops::wu64(rax, rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x4410u64), rax); bb = 37; continue; }
            37 => { crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x4408u64), (1) as u32); rdx = ((r8.wrapping_add(0xau64)) as u32) as u32 as u64; rax = (((rdx) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x4410u64))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 39; continue; } bb = 38; continue; }
            38 => { crate::simd_ops::wu64(rcx.wrapping_add(0x4410u64), rax); crate::simd_ops::wu64(rax, r9); bb = 40; continue; }
            39 => { crate::simd_ops::wu64(rax, rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x4410u64), rax); bb = 40; continue; }
            40 => { crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x4408u64), (1) as u32); rdx = ((r8.wrapping_add(0xbu64)) as u32) as u32 as u64; rax = (((rdx) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x4410u64))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 42; continue; } bb = 41; continue; }
            41 => { crate::simd_ops::wu64(rcx.wrapping_add(0x4410u64), rax); crate::simd_ops::wu64(rax, r9); bb = 43; continue; }
            42 => { crate::simd_ops::wu64(rax, rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x4410u64), rax); bb = 43; continue; }
            43 => { crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x4408u64), (1) as u32); rdx = ((r8.wrapping_add(0xcu64)) as u32) as u32 as u64; rax = (((rdx) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x4410u64))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 45; continue; } bb = 44; continue; }
            44 => { crate::simd_ops::wu64(rcx.wrapping_add(0x4410u64), rax); crate::simd_ops::wu64(rax, r9); bb = 46; continue; }
            45 => { crate::simd_ops::wu64(rax, rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x4410u64), rax); bb = 46; continue; }
            46 => { crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x4408u64), (1) as u32); rdx = ((r8.wrapping_add(0xdu64)) as u32) as u32 as u64; rax = (((rdx) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x4410u64))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 48; continue; } bb = 47; continue; }
            47 => { crate::simd_ops::wu64(rcx.wrapping_add(0x4410u64), rax); crate::simd_ops::wu64(rax, r9); bb = 49; continue; }
            48 => { crate::simd_ops::wu64(rax, rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x4410u64), rax); bb = 49; continue; }
            49 => { crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x4408u64), (1) as u32); r8 = (((r8 as u32)).wrapping_add(0x10u32)) as u32 as u64; rax = ((r8.wrapping_sub(0x2u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 1; continue; } bb = 50; continue; }
            50 => { rax = rcx; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14161e800(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x118; const P1: u64 = 0x1426900d0; const P2: u64 = 0x142677588; const P3: u64 = 0x8; const P4: u64 = 0x142690350; const P5: u64 = 0x10; const P6: u64 = 0x142690378; const P7: u64 = 0x28; const P8: u64 = 0x120; const P9: u64 = 0x141623960; const P10: u64 = 0x14; const P11: u64 = 0x18; const P12: u64 = 0x1415242e0; const P13: u64 = 0x141621970; const P14: u64 = 0x141621060; const P15: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = P1; crate::simd_ops::wu64(rcx, rax); rbp = ((rdx as u32)) as u32 as u64; rax = P2; rsi = rcx; crate::simd_ops::wu64(rcx.wrapping_add(P3), rax); rax = P4; crate::simd_ops::wu64(rcx.wrapping_add(P5), rax); rax = P6; crate::simd_ops::wu64(rcx.wrapping_add(P7), rax); rax = (crate::simd_ops::ru32(rcx.wrapping_add(P8))) as u32 as u64; rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(P10), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P11))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x118u64))) as u64; rbx = 0; fl_l = 0; fl_r = 0; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsi.wrapping_add(0x120u64), ((rbx as u32)) as u32); rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi.wrapping_add(0x118u64), rbx); crate::simd_ops::wu32(rsi.wrapping_add(0x124u64), ((rbx as u32)) as u32); bb = 7; continue; }
            7 => { rcx = rsi; crate::simd_ops::wu64(rsi.wrapping_add(0x130u64), rbx); rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rax = crate::simd_ops::call_stk(P14, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rbp as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = 0x140; rcx = rsi; rax = crate::simd_ops::call_stk(P15, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = rsi; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14161e900(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141535120; const P1: u64 = 0x120; const P2: u64 = 0x118; const P3: u64 = 0x141623b50;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rsi = rdx;
    rdi = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rbp = ((rax as u32)) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1;
    if (((fl_l as u32)) != ((fl_r as u32))) { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx);
    rbx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64;
    rdi = ((rbx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64;
    fl_l = rbx; fl_r = rdi;
    if (((fl_l as u64)) == ((fl_r as u64))) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    loop { rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = ((rsi as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rax = ((rbp as u32)) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14161e970(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x270; const P1: u64 = 0x20; const P2: u64 = 0xf0; const P3: u64 = 0x14153b360; const P4: u64 = 0x2; const P5: u64 = 0x24; const P6: u64 = 0x118; const P7: u64 = 0x120; const P8: u64 = 0x30; const P9: u64 = 0x38; const P10: u64 = 0x8; const P11: u64 = 0x141623b50;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rsi = rdx; rdi = rcx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbp = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru32(rsi.wrapping_add(P1))) as u32 as u64; rcx = ((rdi.wrapping_add(P2)) as u64) as u64; r8 = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rbp = ((rax.wrapping_add(P4)) as u32) as u32 as u64; bb = 4; continue; }
            3 => { crate::simd_ops::wu64(rax, rsi); rdx = rdi; rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(P5), (1) as u32)) as u32 as u64; bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(P6))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(P7))) as u32 as u64; rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 5; continue; }
            5 => { r8 = (crate::simd_ops::ru64(rbx)) as u64; rdx = (crate::simd_ops::ru32(rsi.wrapping_add(P1))) as u32 as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(P8))) as u64; rcx = (crate::simd_ops::ru32(r8.wrapping_add(P9))) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 6; continue; }
            6 => { bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rax = ((rax).wrapping_add(0x10u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 9; continue; }
            9 => { bb = 12; continue; }
            10 => { rdx = (crate::simd_ops::ru64(rax.wrapping_add(P10))) as u64; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(P10))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru32(rax)) as u32 as u64; rcx = r8; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = ((rbp as u32)) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14161ea50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524330; const P1: u64 = 0x141620ef0; const P2: u64 = 0x1426900d0; const P3: u64 = 0x138; const P4: u64 = 0x142677588; const P5: u64 = 0x118; const P6: u64 = 0x8; const P7: u64 = 0x142690350; const P8: u64 = 0x10; const P9: u64 = 0x142690378; const P10: u64 = 0x28; const P11: u64 = 0x120; const P12: u64 = 0x1416214f0; const P13: u64 = 0x141536a70; const P14: u64 = 0x1415bd1d0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = 0x140; rcx = 0x2; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { rcx = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = P2; crate::simd_ops::wu8(rbx.wrapping_add(P3), (0x0u8) as u8); crate::simd_ops::wu64(rbx, rax); rcx = rbx; rax = P4; crate::simd_ops::wu64(rbx.wrapping_add(P5), (0x0u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(P6), rax); rax = P7; crate::simd_ops::wu64(rbx.wrapping_add(P8), rax); rax = P9; crate::simd_ops::wu64(rbx.wrapping_add(P10), rax); crate::simd_ops::wu64(rbx.wrapping_add(P11), (0x0u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x130u64), (0x0u64) as u64); rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 2; continue; }
            2 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = rbx.wrapping_add(0x10); rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; rax = P13; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P14, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rax = crate::simd_ops::call_stk(rdx, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            8 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14161eb40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415a1730; const P1: u64 = 0x141620a50; const P2: u64 = 0x1415c20b0; const P3: u64 = 0x1415864b0; const P4: u64 = 0x143931dd8; const P5: u64 = 0x1415242e0; const P6: u64 = 0x14159d630;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x70u64))) as u64; rbp = (((rdx as u16)) as u32) as u32 as u64; rsi = rcx; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0xd4u64))) as u16 as u64; fl_r = (0x1u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 27; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x130u64))) as u64; r14 = 0; fl_l = 0; fl_r = 0; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); bb = 4; continue; }
            4 => { rdi = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(0x130u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsi.wrapping_add(0x130u64), rdi); bb = 7; continue; }
            6 => { crate::simd_ops::wu64(r14, rdi); bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x138u64))) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = 0x1; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x120u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x60); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(P4)) as u64; rcx = r14; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 12; continue; }
            12 => { bb = 13; continue; }
            13 => { rax = rdx.wrapping_add(0x4400); fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { fl_l = rbx; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = rdx; rdx = (crate::simd_ops::ru64(rax)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 13; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x4410u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { crate::simd_ops::wu64(rdx.wrapping_add(0x4410u64), rbx); crate::simd_ops::wu64(rbx, r14); bb = 19; continue; }
            18 => { crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x4410u64), rbx); bb = 19; continue; }
            19 => { crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x4408u64), (1) as u32); fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x4408u64))) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 20; continue; }
            20 => { fl_l = rdx; fl_r = (crate::simd_ops::ru64(P4)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x4400u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 22; continue; } bb = 21; continue; }
            21 => { crate::simd_ops::wu64(P4, rax); bb = 23; continue; }
            22 => { crate::simd_ops::wu64(rcx.wrapping_add(0x4400u64), rax); bb = 23; continue; }
            23 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 24; continue; }
            24 => { rbx = rdi; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 25; continue; }
            25 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 26; continue; }
            26 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 27; continue; }
            27 => { rdx = (((rbp as u16)) as u32) as u32 as u64; rcx = rsi; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P6, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14161ecb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141623270; const P1: u64 = 0x14161eeb0; const P2: u64 = 0x1415bf7c0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x78u64)) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x70u64))) as u64; fl_r = (0x0u64) as u64; r14 = rdx; r15 = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 28; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 27; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 27; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rax.wrapping_sub(0x18u64), rdi); rdi = (crate::simd_ops::ru64(rcx.wrapping_add(0x130u64))) as u64; crate::simd_ops::wu64(rax.wrapping_sub(0x68u64), rdi); crate::simd_ops::wu64(rax.wrapping_sub(0x60u64), (0x0u64) as u64); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); bb = 5; continue; }
            5 => { rsi = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; xmm0 = (crate::simd_ops::ru128(rsi.wrapping_add(0x20u64))) as u128; xmm1 = (crate::simd_ops::ru128(rsi.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rsi.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm1) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm0) as u128); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 20; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 20; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x50u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 20; continue; } bb = 11; continue; }
            11 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x28u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 13; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x58u64))) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 20; continue; } bb = 13; continue; }
            13 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x30u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 15; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x60u64))) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 20; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x38u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 20; continue; } bb = 17; continue; }
            17 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x78u64))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x80u64))) as u32 as u64; rbp = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; r8 = rbp; rdx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x80u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x78u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 18; continue; } bb = 19; continue; }
            19 => { fl_l = (crate::simd_ops::lk_sub_u32(rdi.wrapping_add(0x70u64), (1) as u32)) as u32 as u64; bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x138u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 23; continue; } bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x70u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 23; continue; } bb = 22; continue; }
            22 => { xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; r8 = rsp.wrapping_add(0x20); rdx = rsp.wrapping_add(0x30); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rdi = ((xmm0 as u64)) as u64; bb = 24; continue; }
            23 => { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdi); rdi = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); bb = 24; continue; }
            24 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 25; continue; }
            25 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(r15)) as u64; rdx = 0x1; rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x120u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; bb = 27; continue; }
            27 => { rdx = r14; rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 28; continue; }
            28 => { rsp = ((rsp).wrapping_add(0x78u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14161ee60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x60; const P1: u64 = 0xd0; const P2: u64 = 0xd4; const P3: u64 = 0x1416208e0; const P4: u64 = 0x14159dc00;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rcx = ((rcx).wrapping_add(0xfffffffffffffff0u64)) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(P1))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if ((((fl_l as u16)) as i16) > (((fl_r as u16)) as i16)) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(P2))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if ((((fl_l as u16)) as i16) <= (((fl_r as u16)) as i16)) { bb = 4; continue; } bb = 3; continue; }
            3 => { r8 = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rcx = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14161eeb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141620a50; const P1: u64 = 0x1415c20b0; const P2: u64 = 0x1415864b0; const P3: u64 = 0x143931dd8; const P4: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(r8)) as u64; rdi = r8; rsi = rdx; r9 = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rbx; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x130u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rcx.wrapping_add(0x130u64), rax); bb = 3; continue; }
            2 => { crate::simd_ops::wu64(r9, rax); bb = 3; continue; }
            3 => { crate::simd_ops::wu64(r8, rax); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x8u64), r9); fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x138u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = 0x1; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x120u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx.wrapping_add(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(P3)) as u64; rcx = 0; fl_l = 0; fl_r = 0; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { rax = rdx.wrapping_add(0x4400); fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = rbx; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = rdx; rdx = (crate::simd_ops::ru64(rax)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x4410u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rdx.wrapping_add(0x4410u64), rbx); crate::simd_ops::wu64(rbx, rax); bb = 14; continue; }
            13 => { crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x4410u64), rbx); bb = 14; continue; }
            14 => { crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x4408u64), (1) as u32); fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x4408u64))) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 19; continue; } bb = 15; continue; }
            15 => { fl_l = rdx; fl_r = (crate::simd_ops::ru64(P3)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x4400u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu64(P3, rax); bb = 18; continue; }
            17 => { crate::simd_ops::wu64(rcx.wrapping_add(0x4400u64), rax); bb = 18; continue; }
            18 => { rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 19; continue; }
            19 => { xmm0 = (crate::simd_ops::ru128(rdi)) as u128; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = rsi; crate::simd_ops::wu128(rsi, (xmm0) as u128); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14161efd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x130; const P1: u64 = 0x14161eeb0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x48u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r9 = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x0u64) as u64); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } bb = 3; continue; }
            3 => { rsp = ((rsp).wrapping_add(0x48u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            4 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; r8 = rsp.wrapping_add(0x20); rdx = rsp.wrapping_add(0x30); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rcx = r9; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { rsp = ((rsp).wrapping_add(0x48u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14161f110(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1416208e0; const P1: u64 = 0x1415bdd20; const P2: u64 = 0x1415a0610;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rbp = rcx; rdx = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 20; continue; } bb = 1; continue; }
            1 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 2; continue; }
            2 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 21; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x130u64))) as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 4; continue; }
            4 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { fl_l = r8; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 7; continue; }
            7 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x54u64))) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu32(rax.wrapping_add(0x58u64), ((rcx as u32)) as u32); bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 11; continue; }
            11 => { bb = 21; continue; }
            12 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x130u64))) as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 13; continue; }
            13 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { fl_l = rdx; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 18; continue; } bb = 15; continue; }
            15 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x54u64))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 18; continue; } bb = 17; continue; }
            17 => { crate::simd_ops::wu32(rax.wrapping_add(0x58u64), (0x1u32) as u32); bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 13; continue; } bb = 19; continue; }
            19 => { bb = 21; continue; }
            20 => { r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x1fu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 28; continue; } bb = 22; continue; }
            22 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x1du64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 28; continue; } bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 26; continue; } bb = 24; continue; }
            24 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 37; continue; } bb = 25; continue; }
            25 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 28; continue; } bb = 26; continue; }
            26 => { rdx = 0; fl_l = 0; fl_r = 0; bb = 27; continue; }
            27 => { rcx = rbp.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 28; continue; }
            28 => { rsi = (crate::simd_ops::ru32(rbp.wrapping_add(0x100u64))) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 36; continue; } bb = 29; continue; }
            29 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r14); r14 = ((rsi as u32)) as u32 as u64; fl_l = ((r14).wrapping_shl(3u32)) as u64; r14 = ((fl_l as u64)) as u64; bb = 30; continue; }
            30 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0xf8u64))) as u64; r14 = r14.wrapping_sub(0x8); rsi = (((rsi as u32)).wrapping_sub(1)) as u32 as u64; fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x1cu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rdi = (crate::simd_ops::ru64(r14.wrapping_add(rax))) as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 32; continue; } bb = 31; continue; }
            31 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x80u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 34; continue; } bb = 32; continue; }
            32 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 34; continue; } bb = 33; continue; }
            33 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x90u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 34; continue; }
            34 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 30; continue; } bb = 35; continue; }
            35 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 36; continue; }
            36 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            37 => { rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 27; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14161f270(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x1416208e0; const P3: u64 = 0x130; const P4: u64 = 0x54; const P5: u64 = 0x58;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r8 = (crate::simd_ops::ru32(rdx)) as u32 as u64; r9 = rcx; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x1; r8 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 15; continue; } bb = 2; continue; }
            2 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x1; r8 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { return rax; }
            6 => { rax = (crate::simd_ops::ru64(r9.wrapping_add(P3))) as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 7; continue; }
            7 => { rdx = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = r8; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 10; continue; }
            10 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P4))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu32(rax.wrapping_add(P5), ((rdx as u32)) as u32); bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 14; continue; }
            14 => { return rax; }
            15 => { rax = (crate::simd_ops::ru64(r9.wrapping_add(P3))) as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 16; continue; }
            16 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { fl_l = r8; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 18; continue; }
            18 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P4))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 21; continue; } bb = 20; continue; }
            20 => { crate::simd_ops::wu32(rax.wrapping_add(P5), (0x1u32) as u32); bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 16; continue; } bb = 22; continue; }
            22 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14161f380(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415a0190; const P1: u64 = 0x14159cab0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14161f3b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14159dd20; const P1: u64 = 0x1416221c0; const P2: u64 = 0x1415c0ff0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((r9 as u8)) as u8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; r13 = (((r8 as u8)) as u32) as u32 as u64; r14 = rdx; r12 = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x2; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r15); r15 = 0x1; fl_l = ((r13 as u8)) as u8 as u64; fl_r = ((r13 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 25; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 25; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 25; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(r12)) as u64; rdx = (((r15 as u16)) as u32) as u32 as u64; rcx = r12; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x118u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(r12.wrapping_add(0x130u64))) as u64; rcx = 0x2; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; r15 = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (rcx as u32) } else { (r15 as u32) }) as u32 as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rdi); bb = 7; continue; }
            7 => { rsi = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; xmm0 = (crate::simd_ops::ru128(rsi.wrapping_add(0x20u64))) as u128; xmm1 = (crate::simd_ops::ru128(rsi.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rsi.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm1) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 22; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 22; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 22; continue; } bb = 13; continue; }
            13 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x28u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 15; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x38u64))) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 22; continue; } bb = 15; continue; }
            15 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x30u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 17; continue; } bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x40u64))) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 22; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 22; continue; } bb = 19; continue; }
            19 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x78u64))) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x80u64))) as u32 as u64; rbp = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; rcx = ((rdi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 21; continue; } bb = 20; continue; }
            20 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; r8 = rbp; rdx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x80u64))) as u32 as u64; rdi = ((rdi).wrapping_add(0x8u64)) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x78u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 20; continue; } bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x70u64), (1) as u32)) as u32 as u64; bb = 22; continue; }
            22 => { rbx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 23; continue; }
            23 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; bb = 24; continue; }
            24 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; bb = 25; continue; }
            25 => { r9 = ((crate::simd_ops::ru8(rsp.wrapping_add(0x98u64))) as u32) as u32 as u64; r8 = (((r13 as u8)) as u32) as u32 as u64; rdx = r14; rcx = r12; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((r15 as u32)) as u32 as u64; fl_r = 0x1; rax = (if (((fl_l as u32)) != ((fl_r as u32))) { (r15 as u32) } else { (rax as u32) }) as u32 as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14161f580(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415af330; const P1: u64 = 0x1415864b0; const P2: u64 = 0x143930f90; const P3: u64 = 0x141599ce0; const P4: u64 = 0x1415945c0; const P5: u64 = 0x143930f70; const P6: u64 = 0x1415d9620; const P7: u64 = 0x1415d9820; const P8: u64 = 0x141524330; const P9: u64 = 0x141621be0; const P10: u64 = 0x14153e940; const P11: u64 = 0x141621e80; const P12: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 29>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x37); rsp = ((rsp).wrapping_sub(0xc0u64)) as u64; r15 = r8; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rdx); rsi = rcx; crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x24u64), (1) as u32); xmm0 = ((crate::simd_ops::ru64(r8.wrapping_add(0x98u64))) as u64) as u128; crate::simd_ops::wu64(rbp.wrapping_sub(0x21u64), ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x21u64))) as u64; r8 = rcx; r8 = ((r8).wrapping_shr(8u32)) as u64; rax = ((rcx.wrapping_add(0x80u64)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xcfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xb0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = rcx; fl_l = ((rdx).wrapping_shr(32u32)) as u64; rdx = ((fl_l as u64)) as u64; bb = 4; continue; }
            3 => { rdx = (rdx & !0xffu64) | (((0xffu8) as u8 as u64)); bb = 4; continue; }
            4 => { rdi = 0; fl_l = 0; fl_r = 0; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; bb = 8; continue; }
            7 => { rcx = ((rdi as u32)) as u32 as u64; bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x7u64), rax); rax = (crate::simd_ops::ru32(r15.wrapping_add(0x34u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0xfu64), ((rax as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_add(0x7u64))) as u128; crate::simd_ops::wu32(rbp.wrapping_add(0x17u64), ((rcx as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x1fu64), ((r8 as u8)) as u8); xmm1 = (crate::simd_ops::ru128(rbp.wrapping_add(0x17u64))) as u128; crate::simd_ops::wu64(rbp.wrapping_add(0x2fu64), rdi); crate::simd_ops::wu128(rsi.wrapping_add(0x10u64), (xmm0) as u128); crate::simd_ops::wu8(rbp.wrapping_add(0x27u64), ((rdx as u8)) as u8); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_add(0x27u64))) as u128; crate::simd_ops::wu128(rsi.wrapping_add(0x20u64), (xmm1) as u128); crate::simd_ops::wu128(rsi.wrapping_add(0x30u64), (xmm0) as u128); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x60u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 13; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x64u64))) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 13; continue; } bb = 10; continue; }
            10 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x60u64), ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x64u64))) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 14; continue; } bb = 12; continue; }
            12 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 14; continue; }
            13 => { fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x60u64), ((rax as u32)) as u32); bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru32(r15.wrapping_add(0x34u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x54u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(r15.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x48u64), rax); rax = (crate::simd_ops::ru32(r15.wrapping_add(0x30u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x50u64), ((rax as u32)) as u32); rbx = (crate::simd_ops::ru64(r15.wrapping_add(0x20u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 16; continue; }
            16 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::lk_add_u32(rbx, (1) as u32)) as u32 as u64; bb = 18; continue; }
            18 => { crate::simd_ops::wu64(rsi.wrapping_add(0x40u64), rbx); rax = (crate::simd_ops::ru32(r15.wrapping_add(0x80u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x58u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x138u64))) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x54u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(P2)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 20; continue; }
            20 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(0x118u64))) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x120u64))) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 37; continue; } bb = 21; continue; }
            21 => { crate::simd_ops::wu64(rsp.wrapping_add(0xf8u64), r13); r13 = rsi.wrapping_add(0x60); crate::simd_ops::wu64(rsp.wrapping_add(0xf0u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x100u64), r14); bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), rdx); crate::simd_ops::wu32(rbp.wrapping_sub(0x1u64), ((rdi as u32)) as u32); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { fl_l = ((rax).wrapping_add(0x18u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), rax); bb = 25; continue; }
            24 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), rdi); bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru32(r15.wrapping_add(0xa0u64))) as u32 as u64; rcx = r15; xmm0 = ((crate::simd_ops::ru64(r15.wrapping_add(0x98u64))) as u64) as u128; crate::simd_ops::wu32(rbp.wrapping_sub(0x49u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(r15.wrapping_add(0xa8u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), rax); crate::simd_ops::wu64(rbp.wrapping_sub(0x59u64), rdi); crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), (0x0u64) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x31u64), ((rdi as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x2du64), (0x1u32) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), rdi); crate::simd_ops::wu64(rbp.wrapping_sub(0x51u64), ((xmm0 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rbx)) as u64; r9 = rbp.wrapping_sub(0x59); rcx = (crate::simd_ops::ru64(P5)) as u64; rdx = rbp.wrapping_sub(0x11); crate::simd_ops::wu32(rbp.wrapping_sub(0x39u64), ((rax as u32)) as u32); r8 = ((r8).wrapping_add(0x20u64)) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x59u64), rax); rax = (crate::simd_ops::ru32(r15.wrapping_add(0x84u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x31u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r15.wrapping_add(0x34u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x35u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(r15.wrapping_add(0x2f8u64))) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; rax = ((((rax as u32)) & (0x1u32))) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), rdi); crate::simd_ops::wu32(rbp.wrapping_sub(0x2du64), ((rax as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r13); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x4cu64))) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 27; continue; } bb = 26; continue; }
            26 => { r8 = (crate::simd_ops::ru32(rax.wrapping_add(0x48u64))) as u32 as u64; r9 = rbp.wrapping_sub(0x59); rcx = (crate::simd_ops::ru64(P5)) as u64; rdx = rbp.wrapping_sub(0x11); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r13); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 27; continue; }
            27 => { rdi = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0x68; rcx = ((rdx.wrapping_sub(0x58u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 35; continue; } bb = 28; continue; }
            28 => { r8 = rdi; rdx = rsi; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 35; continue; } bb = 29; continue; }
            29 => { r12 = (crate::simd_ops::ru32(rsi.wrapping_add(0x80u64))) as u32 as u64; fl_l = ((r12 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsi.wrapping_add(0x84u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 32; continue; } bb = 30; continue; }
            30 => { rcx = rsi.wrapping_add(0x78); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 34; continue; } bb = 31; continue; }
            31 => { fl_l = ((r12 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsi.wrapping_add(0x84u64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 34; continue; } bb = 32; continue; }
            32 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x80u64))) as u32 as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x80u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x78u64))) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 34; continue; } bb = 33; continue; }
            33 => { crate::simd_ops::wu64(rdx, r14); bb = 35; continue; }
            34 => { rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = r14; rcx = 0x10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 35; continue; }
            35 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rdi = 0x0; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(0x120u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x118u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 22; continue; } bb = 36; continue; }
            36 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x100u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0xf8u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; bb = 37; continue; }
            37 => { rsp = ((rsp).wrapping_add(0xc0u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14161f8a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(0x130u64))) as u64; r14 = r9; rsi = r8; rbp = rdx; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x78u64))) as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x80u64))) as u32 as u64; r8 = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x50u64))) as u64; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = ((rax).wrapping_add(0x8u64)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 5; continue; }
            5 => { bb = 17; continue; }
            6 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x58u64))) as u64; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(0x60u64))) as u32 as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rcx = ((rcx).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru64(rbx)) as u64; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rbx = ((rbx).wrapping_add(0x20u64)) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 11; continue; }
            11 => { bb = 17; continue; }
            12 => { fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 13; continue; }
            13 => { rcx = r14.wrapping_add(0x10); rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x24u64), (1) as u32)) as u32 as u64; bb = 17; continue; }
            17 => { rdi = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 18; continue; }
            18 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 19; continue; }
            19 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14161f990(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930fa0; const P1: u64 = 0x1415c3fb0; const P2: u64 = 0x141622db0; const P3: u64 = 0x141622350; const P4: u64 = 0x14158f5a0; const P5: u64 = 0x1415f9ee0; const P6: u64 = 0x1415945c0; const P7: u64 = 0x1415bfb60; const P8: u64 = 0x1415c0d60; const P9: u64 = 0x14152ede0; const P10: u64 = 0x1415864b0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 111>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = r11.wrapping_sub(0x278); rsp = ((rsp).wrapping_sub(0x360u64)) as u64; r14 = (crate::simd_ops::ru64(rcx.wrapping_add(0xf8u64))) as u64; r15 = r8; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x100u64))) as u32 as u64; rdx = rcx; r9 = ((r14.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = r14; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 31; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(r11.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), rsi); rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), rdi); crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), r12); crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), r13); bb = 2; continue; }
            2 => { rdi = (crate::simd_ops::ru64(rdx.wrapping_add(0x118u64))) as u64; r12 = ((rsi as u32)) as u32 as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x120u64))) as u32 as u64; r13 = (crate::simd_ops::ru64(r14)) as u64; rsi = ((rdi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x48u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x38u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(r13.wrapping_add(0x20u64))) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 8; continue; }
            8 => { bb = 11; continue; }
            9 => { rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x48u64))) as u64; r8 = r15.wrapping_add(0x10); rcx = (crate::simd_ops::ru64(P0)) as u64; r9 = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (0x0u64) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm1 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32) as u128; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { fl_l = (((r12 as u32)).wrapping_add(1)) as u32 as u64; r12 = ((fl_l as u32)) as u32 as u64; bb = 11; continue; }
            11 => { rdi = ((rdi).wrapping_add(0x8u64)) as u64; fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(r15.wrapping_add(0x78u64))) as u64; rax = (crate::simd_ops::ru32(r15.wrapping_add(0x80u64))) as u32 as u64; rdi = (crate::simd_ops::ru32(r13.wrapping_add(0x20u64))) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 13; continue; }
            13 => { bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = ((rdi as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 17; continue; } bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru32(r15.wrapping_add(0x80u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x78u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 16; continue; }
            16 => { bb = 18; continue; }
            17 => { crate::simd_ops::wu32(rax.wrapping_add(0x8u64), ((r12 as u32)) as u32); fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 28; continue; } bb = 18; continue; }
            18 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x288u64))) as u64; rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rsi); rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x74u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rax); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { fl_l = (crate::simd_ops::lk_add_u32(rax, (1) as u32)) as u32 as u64; bb = 20; continue; }
            20 => { xmm0 = (crate::simd_ops::ru128(rdi.wrapping_add(0x38u64))) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); rcx = rbp.wrapping_sub(0x10); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x80u64))) as u32 as u64; xmm1 = (crate::simd_ops::ru128(rdi.wrapping_add(0x48u64))) as u128; crate::simd_ops::wu32(rbp.wrapping_sub(0x40u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x84u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x3cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x88u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x38u64), rax); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x90u64))) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x78u64), (xmm0) as u128); crate::simd_ops::wu32(rbp.wrapping_sub(0x30u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x94u64))) as u32 as u64; xmm0 = (crate::simd_ops::ru128(rdi.wrapping_add(0x58u64))) as u128; crate::simd_ops::wu32(rbp.wrapping_sub(0x2cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0xa0u64))) as u32 as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x68u64), (xmm0) as u128); crate::simd_ops::wu32(rbp.wrapping_sub(0x20u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdi.wrapping_add(0x78u64))) as u64) as u128; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0xa8u64))) as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x78u64), (xmm1) as u128); crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), rax); xmm1 = (crate::simd_ops::ru128(rdi.wrapping_add(0x68u64))) as u128; crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), ((xmm0 as u64)) as u64); xmm0 = ((crate::simd_ops::ru64(rdi.wrapping_add(0x98u64))) as u64) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x58u64), (xmm1) as u128); crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), ((xmm0 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x2e0u64))) as u64; rdx = rdi.wrapping_add(0xb0); crate::simd_ops::wu64(rbp.wrapping_add(0x220u64), rax); rcx = rbp.wrapping_sub(0x10); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x2e8u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x228u64), rax); crate::simd_ops::wu64(rbp.wrapping_add(0xa8u64), rsi); crate::simd_ops::wu64(rbp.wrapping_add(0xb0u64), rsi); crate::simd_ops::wu64(rbp.wrapping_add(0xb8u64), rsi); crate::simd_ops::wu64(rbp.wrapping_add(0xc0u64), rsi); crate::simd_ops::wu32(rbp.wrapping_add(0xc8u64), ((rsi as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x210u64), rsi); crate::simd_ops::wu64(rbp.wrapping_add(0x218u64), rsi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x238u64))) as u32) as u32 as u64; rcx = (rcx & !0xffu64) | ((((((rcx as u8)) ^ (crate::simd_ops::ru8(rdi.wrapping_add(0x2f8u64))))) as u8 as u64)); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x2f0u64))) as u64; rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0xfu8))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) ^ (crate::simd_ops::ru8(rbp.wrapping_add(0x238u64))))) as u8 as u64)); crate::simd_ops::wu64(rbp.wrapping_add(0x230u64), rax); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x280u64))) as u64; rax = ((rax).wrapping_add(0x10u64)) as u64; crate::simd_ops::wu8(rbp.wrapping_add(0x238u64), ((rcx as u8)) as u8); fl_l = (crate::simd_ops::ru8(rbp.wrapping_sub(0x28u64))) as u8 as u64; fl_r = (0x90u8) as u8 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rsi as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x38u64), rsi); crate::simd_ops::wu32(rbp.wrapping_sub(0x30u64), ((rsi as u32)) as u32); if (((fl_l as u8)) != ((fl_r as u8))) { bb = 24; continue; } bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_sub(0x23u64))) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if (((fl_l as u8)) <= ((fl_r as u8))) { bb = 24; continue; } bb = 22; continue; }
            22 => { rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0x1u8))) as u8 as u64)); crate::simd_ops::wu8(rbp.wrapping_add(0x298u64), ((rcx as u8)) as u8); rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = ((rax as u32)) as u32 as u64; r9 = rbp.wrapping_add(0x298); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x74u64))) as u32 as u64; rdx = rbp.wrapping_sub(0x28); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); rcx = r13; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 26; continue; } bb = 23; continue; }
            23 => { rdx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x238u64))) as u32) as u32 as u64; rax = (((rdx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) ^ (crate::simd_ops::ru8(rbp.wrapping_add(0x298u64))))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x1u8))) as u8 as u64)); fl_l = ((((rdx as u8)) ^ ((rax as u8)))) as u8 as u64; fl_r = fl_l; rdx = (rdx & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbp.wrapping_add(0x238u64), ((rdx as u8)) as u8); bb = 24; continue; }
            24 => { rdx = rsp.wrapping_add(0x40); rcx = r13; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 26; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(r13)) as u64; rdx = rsp.wrapping_add(0x40); rcx = r13; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x250u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 26; continue; }
            26 => { rcx = rbp.wrapping_sub(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 27; continue; }
            27 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 29; continue; }
            28 => { rsi = 0; fl_l = 0; fl_r = 0; bb = 29; continue; }
            29 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x280u64))) as u64; r14 = ((r14).wrapping_add(0x8u64)) as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(0x100u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0xf8u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = r14; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 30; continue; }
            30 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x340u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x348u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x350u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x358u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x390u64))) as u64; bb = 31; continue; }
            31 => { rax = 0x1; rsp = ((rsp).wrapping_add(0x360u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14161fd60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141594db0; const P1: u64 = 0x143931dd8; const P2: u64 = 0x141524330; const P3: u64 = 0x14161e450; const P4: u64 = 0x1411ffc90; const P5: u64 = 0x14161f580; const P6: u64 = 0x14161efd0; const P7: u64 = 0x14161f990; const P8: u64 = 0x14161ff50;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = rdx; rsi = rcx; rdx = rcx; rcx = r14; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(P1)) as u64; rbp = 0; fl_l = 0; fl_r = 0; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x4410u64))) as u64; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x4400u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rdx = 0x4418; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 5; continue; }
            5 => { rcx = rax; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(P1)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(P1, rbx); crate::simd_ops::wu64(rbx.wrapping_add(0x4400u64), rbp); bb = 8; continue; }
            7 => { crate::simd_ops::wu64(rbx.wrapping_add(0x4400u64), rax); crate::simd_ops::wu64(P1, rbx); bb = 8; continue; }
            8 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x4410u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rax; fl_r = rax; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbp } else { rax }) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x4410u64), rax); crate::simd_ops::lk_sub_u32(rbx.wrapping_add(0x4408u64), (1) as u32); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rdi, rbp); rcx = rdi.wrapping_add(0x60); crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), rbp); crate::simd_ops::wu32(rdi.wrapping_add(0x18u64), ((rbp as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0x20u64), ((rbp as u32)) as u32); crate::simd_ops::wu8(rdi.wrapping_add(0x28u64), (0xffu8) as u8); crate::simd_ops::wu8(rdi.wrapping_add(0x30u64), (0xffu8) as u8); crate::simd_ops::wu64(rdi.wrapping_add(0x38u64), rbp); crate::simd_ops::wu64(rdi.wrapping_add(0x50u64), rbp); crate::simd_ops::wu64(rdi.wrapping_add(0x48u64), rbp); crate::simd_ops::wu64(rdi.wrapping_add(0x40u64), rbp); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rdi.wrapping_add(0x70u64), ((rbp as u32)) as u32); crate::simd_ops::wu64(rdi.wrapping_add(0x78u64), rbp); crate::simd_ops::wu64(rdi.wrapping_add(0x80u64), rbp); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x130u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rsi.wrapping_add(0x130u64), rdi); crate::simd_ops::wu64(rdi, rbp); bb = 13; continue; }
            12 => { crate::simd_ops::wu64(rdi, rax); crate::simd_ops::wu64(rsi.wrapping_add(0x130u64), rdi); bb = 13; continue; }
            13 => { r8 = r14; rdx = rsi; rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x138u64))) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 19; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = 0x1; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x118u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 18; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = 0x1; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x120u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rdi; rcx = rsi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            16 => { rax = 0x2; bb = 17; continue; }
            17 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            18 => { r8 = rdi; rdx = r14; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            19 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x70u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { rbx = ((crate::simd_ops::ru16(rax.wrapping_add(0xd4u64))) as u32) as u32 as u64; bb = 22; continue; }
            21 => { rbx = ((rbp as u32)) as u32 as u64; bb = 22; continue; }
            22 => { rdx = r14; rcx = rsi; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x70u64))) as u64; r14 = ((rax as u32)) as u32 as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { rbp = ((crate::simd_ops::ru16(rcx.wrapping_add(0xd4u64))) as u32) as u32 as u64; bb = 24; continue; }
            24 => { fl_l = ((rbp as u16)) as u16 as u64; fl_r = ((rbx as u16)) as u16 as u64; if ((((fl_l as u16)) as i16) <= (((fl_r as u16)) as i16)) { bb = 26; continue; } bb = 25; continue; }
            25 => { rax = 0x1; bb = 17; continue; }
            26 => { rdx = rdi; rcx = rsi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rax = ((r14 as u32)) as u32 as u64; bb = 17; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14161ff50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141620f50; const P1: u64 = 0x14158f5a0; const P2: u64 = 0x1415f9ee0; const P3: u64 = 0x1415945c0; const P4: u64 = 0x1415bfb60; const P5: u64 = 0x1415c0d60; const P6: u64 = 0x141620e80; const P7: u64 = 0x141621270; const P8: u64 = 0x141620d10; const P9: u64 = 0x141620d50; const P10: u64 = 0x141621110; const P11: u64 = 0x141621010; const P12: u64 = 0x14152ede0; const P13: u64 = 0x1415864b0; const P14: u64 = 0x141621530; const P15: u64 = 0x141621390;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 119>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rbp = rax.wrapping_sub(0x2b8); rsp = ((rsp).wrapping_sub(0x3a8u64)) as u64; crate::simd_ops::wu64(rax.wrapping_sub(0x18u64), rbx); r8 = rcx; crate::simd_ops::wu64(rax.wrapping_sub(0x20u64), rsi); rdi = 0x1; crate::simd_ops::wu64(rax.wrapping_sub(0x28u64), r12); rsi = rdx; crate::simd_ops::wu64(rax.wrapping_sub(0x30u64), r13); crate::simd_ops::wu64(rax.wrapping_sub(0x40u64), r15); r15 = rcx; rcx = rbp.wrapping_add(0x2d8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rsi)) as u32 as u64; r12 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbp.wrapping_add(0x2c8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(r15.wrapping_add(0x70u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((crate::simd_ops::ru16(rax.wrapping_add(0xd4u64))) as u32) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x2d0u64), ((rcx as u32)) as u32); bb = 3; continue; }
            2 => { crate::simd_ops::wu32(rbp.wrapping_add(0x2d0u64), ((r12 as u32)) as u32); bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(r15.wrapping_add(0xf8u64))) as u64; r13 = (crate::simd_ops::ru32(r15.wrapping_add(0x100u64))) as u32 as u64; rax = ((rbx.wrapping_add(r13.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 36; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x380u64), r14); bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rsi)) as u32 as u64; rdi = rsi.wrapping_add(0xb0); r14 = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rax); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rax); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x78u64), rax); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x34u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x5cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x68u64), rax); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x30u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x60u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), r12); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::lk_add_u32(rax, (1) as u32)) as u32 as u64; bb = 7; continue; }
            7 => { xmm0 = (crate::simd_ops::ru128(rsi.wrapping_add(0x38u64))) as u128; crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), rax); rcx = rbp.wrapping_add(0x20); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x80u64))) as u32 as u64; xmm1 = (crate::simd_ops::ru128(rsi.wrapping_add(0x48u64))) as u128; crate::simd_ops::wu32(rbp.wrapping_sub(0x10u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x84u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0xcu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x88u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x8u64), rax); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x90u64))) as u32 as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x58u64), (xmm0) as u128); crate::simd_ops::wu32(rbp, ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x94u64))) as u32 as u64; xmm0 = (crate::simd_ops::ru128(rsi.wrapping_add(0x58u64))) as u128; crate::simd_ops::wu32(rbp.wrapping_add(0x4u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0xa0u64))) as u32 as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x38u64), (xmm0) as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rsi.wrapping_add(0x78u64))) as u64) as u128; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0xa8u64))) as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x48u64), (xmm1) as u128); crate::simd_ops::wu64(rbp.wrapping_add(0x18u64), rax); xmm1 = (crate::simd_ops::ru128(rsi.wrapping_add(0x68u64))) as u128; crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), ((xmm0 as u64)) as u64); xmm0 = ((crate::simd_ops::ru64(rsi.wrapping_add(0x98u64))) as u64) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x28u64), (xmm1) as u128); crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), ((xmm0 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x230u64))) as u64; rcx = rbp.wrapping_add(0x20); crate::simd_ops::wu64(rbp.wrapping_add(0x250u64), rax); rdx = rdi; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x238u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x258u64), rax); crate::simd_ops::wu64(rbp.wrapping_add(0xd8u64), r12); crate::simd_ops::wu64(rbp.wrapping_add(0xe0u64), r12); crate::simd_ops::wu64(rbp.wrapping_add(0xe8u64), r12); crate::simd_ops::wu64(rbp.wrapping_add(0xf0u64), r12); crate::simd_ops::wu32(rbp.wrapping_add(0xf8u64), ((r12 as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x240u64), r12); crate::simd_ops::wu64(rbp.wrapping_add(0x248u64), r12); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x268u64))) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x2f0u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x260u64), rax); rax = (((rcx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) ^ (crate::simd_ops::ru8(rsi.wrapping_add(0x2f8u64))))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xfu8))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) ^ ((rax as u8)))) as u8 as u64)); crate::simd_ops::wu8(rbp.wrapping_add(0x268u64), ((rcx as u8)) as u8); fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x2c8u64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 16; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0x90u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 12; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0xdu64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) <= ((fl_r as u8))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0x1u8))) as u8 as u64)); crate::simd_ops::wu8(rbp.wrapping_add(0x2c0u64), ((rcx as u8)) as u8); rcx = rsp.wrapping_add(0x70); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = ((rax as u32)) as u32 as u64; r9 = rbp.wrapping_add(0x2c0); rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x5cu64))) as u32 as u64; rdx = rbp.wrapping_add(0x8); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); rcx = r14; rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x80u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 32; continue; } bb = 11; continue; }
            11 => { rcx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x268u64))) as u32) as u32 as u64; rax = (((rcx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) ^ (crate::simd_ops::ru8(rbp.wrapping_add(0x2c0u64))))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) & ((rdi as u8)))) as u8 as u64)); fl_l = ((((rcx as u8)) ^ ((rax as u8)))) as u8 as u64; fl_r = fl_l; rcx = (rcx & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbp.wrapping_add(0x268u64), ((rcx as u8)) as u8); bb = 12; continue; }
            12 => { rdx = rsp.wrapping_add(0x70); rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rdi = ((rax.wrapping_sub(0x2u64)) as u32) as u32 as u64; bb = 32; continue; }
            14 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 32; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(r14)) as u64; rdx = rsp.wrapping_add(0x70); rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x250u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rax as u32)) as u32 as u64; bb = 32; continue; }
            16 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x8u64))) as u64; rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((r13 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 18; continue; } bb = 17; continue; }
            17 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x8u64))) as u64; r8 = rsp.wrapping_add(0x48); rcx = r15; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 22; continue; }
            18 => { crate::simd_ops::wu32(rbp, ((r12 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 20; continue; }
            20 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 21; continue; }
            21 => { r9 = rbp.wrapping_add(0x2d8); rcx = r15; r8 = rsp.wrapping_add(0x70); rdx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 31; continue; } bb = 22; continue; }
            22 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0x90u8) as u8 as u64; rax = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rbp.wrapping_sub(0x8u64), rax); if (((fl_l as u8)) != ((fl_r as u8))) { bb = 26; continue; } bb = 23; continue; }
            23 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0xdu64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) <= ((fl_r as u8))) { bb = 26; continue; } bb = 24; continue; }
            24 => { rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x268u64))) as u32) as u32 as u64; rcx = rsp.wrapping_add(0x70); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x1u8))) as u8 as u64)); crate::simd_ops::wu8(rbp.wrapping_add(0x2c0u64), ((rax as u8)) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = ((rax as u32)) as u32 as u64; r9 = rbp.wrapping_add(0x2c0); rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x5cu64))) as u32 as u64; rdx = rbp.wrapping_add(0x8); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); rcx = r14; rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x80u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 31; continue; } bb = 25; continue; }
            25 => { rcx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x268u64))) as u32) as u32 as u64; rax = (((rcx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) ^ (crate::simd_ops::ru8(rbp.wrapping_add(0x2c0u64))))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) & ((rdi as u8)))) as u8 as u64)); fl_l = ((((rcx as u8)) ^ ((rax as u8)))) as u8 as u64; fl_r = fl_l; rcx = (rcx & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbp.wrapping_add(0x268u64), ((rcx as u8)) as u8); bb = 26; continue; }
            26 => { rdx = rsp.wrapping_add(0x70); rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 28; continue; } bb = 27; continue; }
            27 => { rdi = ((rax.wrapping_sub(0x2u64)) as u32) as u32 as u64; bb = 31; continue; }
            28 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 31; continue; } bb = 29; continue; }
            29 => { rax = (crate::simd_ops::ru64(r14)) as u64; rdx = rsp.wrapping_add(0x70); rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x250u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rax as u32)) as u32 as u64; bb = 31; continue; }
            30 => { rdi = 0x34; bb = 31; continue; }
            31 => { rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 32; continue; }
            32 => { rcx = rbp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x70u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 34; continue; } bb = 33; continue; }
            33 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 34; continue; }
            34 => { rcx = (crate::simd_ops::ru32(r15.wrapping_add(0x100u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0xf8u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 35; continue; }
            35 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x380u64))) as u64; bb = 36; continue; }
            36 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x388u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x3a0u64))) as u64; fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x2c8u64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 44; continue; } bb = 37; continue; }
            37 => { rax = (crate::simd_ops::ru64(r15.wrapping_add(0x70u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 39; continue; } bb = 38; continue; }
            38 => { r12 = ((crate::simd_ops::ru16(rax.wrapping_add(0xd4u64))) as u32) as u32 as u64; bb = 39; continue; }
            39 => { fl_l = ((r12 as u16)) as u16 as u64; fl_r = (crate::simd_ops::ru16(rbp.wrapping_add(0x2d0u64))) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 41; continue; } bb = 40; continue; }
            40 => { rdx = rsi; rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 43; continue; }
            41 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x2d8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 45; continue; } bb = 42; continue; }
            42 => { rdx = rcx; rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P15, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 43; continue; }
            43 => { rdi = ((rax as u32)) as u32 as u64; bb = 44; continue; }
            44 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x2d8u64))) as u64; bb = 45; continue; }
            45 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x378u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x390u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x398u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 47; continue; } bb = 46; continue; }
            46 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 47; continue; }
            47 => { rax = ((rdi as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x3a8u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416203f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x118; const P1: u64 = 0x120; const P2: u64 = 0x20; const P3: u64 = 0x141623ad0; const P4: u64 = 0x78; const P5: u64 = 0xf0; const P6: u64 = 0x1415355f0; const P7: u64 = 0x100; const P8: u64 = 0xf8; const P9: u64 = 0x8; const P10: u64 = 0x14183acc8; const P11: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r14 = rdx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; rsi = rcx; rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru32(r14.wrapping_add(P2))) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(r14.wrapping_add(P4))) as u64; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(r14)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = r14; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(r14.wrapping_add(P2))) as u32 as u64; rcx = ((rsi.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru32(rsi.wrapping_add(P7))) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P8))) as u64; rdx = (((rdx as u32)).wrapping_sub(1)) as u32 as u64; r8 = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { r8 = ((r8).wrapping_sub(rax)) as u64; rdx = ((rax.wrapping_add(P9)) as u64) as u64; r8 = ((((r8 as u32)) & (0xfffffff8u32))) as u32 as u64; rcx = rax; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::lk_sub_u32(rsi.wrapping_add(P7), (1) as u32)) as u32 as u64; bb = 7; continue; }
            7 => { rcx = ((rsi.wrapping_add(P11)) as u64) as u64; rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416204c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f00; const P1: u64 = 0x1401de410; const P2: u64 = 0x1415a2910; const P3: u64 = 0x14153be40; const P4: u64 = 0x143930f88; const P5: u64 = 0x1426452b0; const P6: u64 = 0x1426452a8; const P7: u64 = 0x142645560; const P8: u64 = 0x141534be0; const P9: u64 = 0x1416222e0; const P10: u64 = 0x141534ef0; const P11: u64 = 0x141623960; const P12: u64 = 0x141623820; const P13: u64 = 0x14153e940;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu32(rsp.wrapping_add(0x18u64), ((r8 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r14 = rcx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x70u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = r14; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x5; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(r14)) as u64; rbx = (crate::simd_ops::ru32(r14.wrapping_add(0x20u64))) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x70u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P0)) as u64; r8 = 0x5; r9 = ((rax as u32)) as u32 as u64; rdx = ((rbx as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = 0x5b; rsp = ((rsp).wrapping_add(0x68u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { crate::simd_ops::lk_add_u64(rsp.wrapping_add(0x88u64), (0x4u64) as u64); r8 = rsp.wrapping_add(0x90); r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r15); rdx = rsp.wrapping_add(0x88); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r15 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 68; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rbx = ((rax as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); rdi = (crate::simd_ops::ru32(rcx)) as u32 as u64; rcx = ((rcx).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rcx); fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru32(r14.wrapping_add(0x104u64))) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rdx = ((rdi as u32)) as u32 as u64; rcx = r14.wrapping_add(0xf0); rdx = (((rdx as u32)).wrapping_sub((r8 as u32))) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; r15 = 0x34; r15 = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (rbx as u32) } else { (r15 as u32) }) as u32 as u64; rax = ((r15 as u32)) as u32 as u64; fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 67; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; bb = 7; continue; }
            7 => { rbx = 0; fl_l = 0; fl_r = 0; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(rcx)) as u32 as u64; rcx = ((rcx).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x80u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rcx); rcx = r14; crate::simd_ops::wu8(rsp.wrapping_add(0x84u64), (0x0u8) as u8); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xf; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 11; continue; }
            11 => { rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; bb = 9; continue; }
            13 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; bb = 14; continue; }
            14 => { r15 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 67; continue; } bb = 15; continue; }
            15 => { rdx = (crate::simd_ops::ru32(rcx)) as u32 as u64; rcx = ((rcx).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r12); r12 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x98u64), ((rdx as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rcx); fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 66; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r13); r13 = ((rax.wrapping_sub(0x2u64)) as u32) as u32 as u64; bb = 17; continue; }
            17 => { rbp = (crate::simd_ops::ru32(rcx)) as u32 as u64; rdi = (crate::simd_ops::ru64(P4)) as u64; bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x1b8u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 29; continue; } bb = 19; continue; }
            19 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rdi.wrapping_add(0x1b8u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 18; continue; } bb = 20; continue; }
            20 => { rsi = rdi.wrapping_add(0x1c0); bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x200u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 31; continue; } bb = 22; continue; }
            22 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x1f8u64))) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbp as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rbx = (crate::simd_ops::ru64(rbx.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 23; continue; }
            23 => { bb = 24; continue; }
            24 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 27; continue; } bb = 25; continue; }
            25 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 24; continue; } bb = 26; continue; }
            26 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 27; continue; }
            27 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 28; continue; }
            28 => { fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 32; continue; }
            29 => { rsi = rdi.wrapping_add(0x1c0); rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x1b8u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 21; continue; } bb = 30; continue; }
            30 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); bb = 18; continue; }
            31 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 32; continue; }
            32 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x1b8u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 40; continue; } bb = 33; continue; }
            33 => { bb = 34; continue; }
            34 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rdi.wrapping_add(0x1b8u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 37; continue; } bb = 35; continue; }
            35 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x1b8u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 34; continue; } bb = 36; continue; }
            36 => { bb = 41; continue; }
            37 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 42; continue; } bb = 38; continue; }
            38 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x1e8u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 42; continue; } bb = 39; continue; }
            39 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x1f0u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); bb = 42; continue; }
            40 => { rsi = rdi.wrapping_add(0x1c0); bb = 41; continue; }
            41 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); bb = 42; continue; }
            42 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 54; continue; } bb = 43; continue; }
            43 => { rdi = (crate::simd_ops::ru64(P4)) as u64; rcx = rsp.wrapping_add(0x28); rdx = rdi.wrapping_add(0x1b8); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 48; continue; } bb = 44; continue; }
            44 => { rcx = rdi.wrapping_add(0x1f8); crate::simd_ops::wu32(rax.wrapping_add(0x10u64), ((rbp as u32)) as u32); r8 = rsp.wrapping_add(0x80); rdx = rax; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 48; continue; } bb = 45; continue; }
            45 => { crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), (0x0u32) as u32); rax = ((r13 as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 47; continue; } bb = 46; continue; }
            46 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 47; continue; }
            47 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 48; continue; }
            48 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_l = (crate::simd_ops::lk_add_u32(rax, (1) as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; if (((fl_l as u32)) != 0) { bb = 50; continue; } bb = 49; continue; }
            49 => { crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), (0x0u32) as u32); bb = 50; continue; }
            50 => { rcx = ((rcx).wrapping_add(0x8u64)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 64; continue; } bb = 51; continue; }
            51 => { rdx = r14; rcx = rbx; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); r8 = rsp.wrapping_add(0x90); rcx = rbx; rdx = rsp.wrapping_add(0x88); rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); r15 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 55; continue; } bb = 52; continue; }
            52 => { let xa_src: u32 = (r13 as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; r13 = (xa_old) as u32 as u64; fl_l = ((r13 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 65; continue; } bb = 53; continue; }
            53 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 65; continue; }
            54 => { rdx = r14; rcx = rbx; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); bb = 55; continue; }
            55 => { rdi = r14.wrapping_add(0x118); rsi = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 58; continue; } bb = 56; continue; }
            56 => { rcx = rdi; rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 60; continue; } bb = 57; continue; }
            57 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 60; continue; } bb = 58; continue; }
            58 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 60; continue; } bb = 59; continue; }
            59 => { crate::simd_ops::wu64(rdx, rbx); bb = 62; continue; }
            60 => { r15 = 0x2; rax = ((r13 as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 62; continue; } bb = 61; continue; }
            61 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 62; continue; }
            62 => { r12 = (((r12 as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((r12 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsp.wrapping_add(0x98u64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 65; continue; } bb = 63; continue; }
            63 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; bb = 17; continue; }
            64 => { r15 = 0x34; bb = 65; continue; }
            65 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 66; continue; }
            66 => { fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = (rax & !0xffu64) | (((if (((fl_l as u8)) != ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu8(r14.wrapping_add(0x138u64), ((rax as u8)) as u8); bb = 67; continue; }
            67 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 68; continue; }
            68 => { rax = ((r15 as u32)) as u32 as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x68u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416208e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141620a50; const P1: u64 = 0x1415c20b0; const P2: u64 = 0x1415864b0; const P3: u64 = 0x143931dd8; const P4: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x130u64))) as u64; r12 = 0; fl_l = 0; fl_r = 0; rbp = ((r8 as u32)) as u32 as u64; r14 = rdx; rsi = rcx; r15 = ((r12 as u32)) as u32 as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); bb = 2; continue; }
            2 => { fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = r14; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x54u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { r15 = rbx; rbx = (crate::simd_ops::ru64(rbx)) as u64; bb = 27; continue; }
            7 => { rdi = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(0x130u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rsi.wrapping_add(0x130u64), rdi); bb = 10; continue; }
            9 => { crate::simd_ops::wu64(r15, rdi); bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x138u64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = 0x1; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x120u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x60); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { rdx = (crate::simd_ops::ru64(P3)) as u64; rcx = r12; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 15; continue; }
            15 => { rax = rdx.wrapping_add(0x4400); fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 17; continue; } bb = 16; continue; }
            16 => { fl_l = rbx; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 18; continue; } bb = 17; continue; }
            17 => { rcx = rdx; rdx = (crate::simd_ops::ru64(rax)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 15; continue; } bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x4410u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::wu64(rdx.wrapping_add(0x4410u64), rbx); crate::simd_ops::wu64(rbx, r12); bb = 21; continue; }
            20 => { crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x4410u64), rbx); bb = 21; continue; }
            21 => { crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x4408u64), (1) as u32); fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x4408u64))) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 26; continue; } bb = 22; continue; }
            22 => { fl_l = rdx; fl_r = (crate::simd_ops::ru64(P3)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x4400u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 24; continue; } bb = 23; continue; }
            23 => { crate::simd_ops::wu64(P3, rax); bb = 25; continue; }
            24 => { crate::simd_ops::wu64(rcx.wrapping_add(0x4400u64), rax); bb = 25; continue; }
            25 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            26 => { rbx = rdi; bb = 27; continue; }
            27 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 28; continue; }
            28 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 29; continue; }
            29 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141620a50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141621e80; const P1: u64 = 0x1415242e0; const P2: u64 = 0x143930f90; const P3: u64 = 0x14159b3d0; const P4: u64 = 0x1415c20b0; const P5: u64 = 0x1415af340;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x78u64))) as u64; rdi = rcx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x80u64))) as u32 as u64; rdx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rsi = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = rsi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = rsi; rcx = 0x10; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x78u64))) as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x80u64))) as u32 as u64; rcx = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 4; continue; }
            4 => { rbx = rdx; bb = 5; continue; }
            5 => { rsi = 0; fl_l = 0; fl_r = 0; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = rbx; crate::simd_ops::wu32(rdi.wrapping_add(0x80u64), ((rsi as u32)) as u32); rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(0x78u64), rsi); crate::simd_ops::wu32(rdi.wrapping_add(0x84u64), ((rsi as u32)) as u32); bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x138u64))) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x54u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(P2)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rax = 0xffffffff; rcx = ((rcx).wrapping_add(0x10u64)) as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rsi); bb = 13; continue; }
            13 => { rcx = rdi.wrapping_add(0x60); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x60u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 17; continue; } bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x64u64))) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rdx = (rdx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 18; continue; }
            17 => { rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 18; continue; }
            18 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x60u64), ((rcx as u32)) as u32); if (((fl_l as u32)) == ((fl_r as u32))) { bb = 22; continue; } bb = 19; continue; }
            19 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 23; continue; } bb = 20; continue; }
            20 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 23; continue; } bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x64u64))) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 23; continue; }
            23 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x60u64))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 25; continue; } bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0x4; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 25; continue; }
            25 => { crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), rsi); bb = 26; continue; }
            26 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141620b90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x12; const P1: u64 = 0x10; const P2: u64 = 0x8; const P3: u64 = 0x18; const P4: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx, rax);
    fl_l = (crate::simd_ops::lk_and_u8(rcx.wrapping_add(P0), (0xfcu8) as u8)) as u8 as u64; fl_r = fl_l;
    crate::simd_ops::wu16(rcx.wrapping_add(P1), ((rax as u16)) as u16);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rax);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141620bb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18; const P2: u64 = 0x14;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P0), (0x0u64) as u64); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rdi = 0xffffffff; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rcx = ((rcx).wrapping_add(0x10u64)) as u64; rax = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(P2), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbx.wrapping_add(P1), (0x0u64) as u64); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { rcx = ((rcx).wrapping_add(0x10u64)) as u64; let xa_src: u32 = (rdi as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(P2), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rdi = (xa_old) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rbx, (0x0u64) as u64); bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141620c30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415fab60;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rdx; rdi = rcx; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(0x24u64), (1) as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; rbp = 0xffffffff; crate::simd_ops::wu64(rdi, rax); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { rcx = ((rcx).wrapping_add(0x10u64)) as u64; rax = ((rbp as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rbx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rdx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            8 => { rax = 0; fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rax); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(0x24u64), (1) as u32)) as u32 as u64; bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), rax); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 12; continue; }
            12 => { rcx = ((rcx).wrapping_add(0x10u64)) as u64; let xa_src: u32 = (rbp as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rbp = (xa_old) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x20u64), rax); bb = 15; continue; }
            15 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = rdi; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141620d10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524330; const P1: u64 = 0x8; const P2: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rdx = 0x18;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { crate::simd_ops::wu64(rax, rcx); crate::simd_ops::wu64(rax.wrapping_add(P1), rcx); crate::simd_ops::wu32(rax.wrapping_add(P2), (0x1u32) as u32); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rax = rcx;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141620d50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141621390; const P1: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x1; crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); rsi = rcx; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); rdi = (crate::simd_ops::ru64(rcx)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; rcx = ((rdi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rdi = ((rdi).wrapping_add(0x28u64)) as u64; r8 = (crate::simd_ops::ru64(rsi)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; rcx = ((r8.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 7; continue; }
            7 => { rdi = r8; bb = 8; continue; }
            8 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rcx = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbp); rbx = ((rdi.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rbp = 0; fl_l = 0; fl_r = 0; fl_l = rdi; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 22; continue; } bb = 10; continue; }
            10 => { fl_l = ((rdi).wrapping_add(0x18u64)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_sub(0x10u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_sub(0x10u64), rbp); bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 14; continue; }
            14 => { rcx = ((rcx).wrapping_add(0x10u64)) as u64; rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            16 => { crate::simd_ops::wu64(rdi, rbp); bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_sub(0x18u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 18; continue; }
            18 => { rcx = ((rcx).wrapping_add(0x10u64)) as u64; rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            20 => { crate::simd_ops::wu64(rdi.wrapping_sub(0x18u64), rbp); bb = 21; continue; }
            21 => { rdi = ((rdi).wrapping_add(0x28u64)) as u64; rax = rdi.wrapping_sub(0x18); fl_l = rax; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 22; continue; }
            22 => { rdx = (crate::simd_ops::ru64(rsi)) as u64; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rbp as u32)) as u32); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rsi, rbp); crate::simd_ops::wu32(rsi.wrapping_add(0xcu64), ((rbp as u32)) as u32); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 23; continue; }
            23 => { rdx = rsi; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 24; continue; }
            24 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141620e80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x28; const P5: u64 = 0x30; const P6: u64 = 0x38; const P7: u64 = 0x143930fa8; const P8: u64 = 0x1415f8000;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rbx = rcx; crate::simd_ops::wu64(rcx, rax); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P0), rax); r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P1), r8); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx.wrapping_add(P2), rcx); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P3), ((rax as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(P4), rcx); crate::simd_ops::wu64(rbx.wrapping_add(P5), rcx); crate::simd_ops::wu64(rbx.wrapping_add(P6), rcx); rcx = (crate::simd_ops::ru64(P7)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(r8)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P6), rcx); bb = 5; continue; }
            5 => { rax = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141620f50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x88; const P1: u64 = 0x18; const P2: u64 = 0x141620d00; const P3: u64 = 0x141620d50; const P4: u64 = 0x141621850; const P5: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; crate::simd_ops::wu64(rcx, (0x0u64) as u64); rbp = r8; fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = 0x0; rbx = rcx; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); rdi = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 2; continue; }
            2 => { rcx = rdi; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rbx, rdi); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rsi = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbp.wrapping_add(0x108); rdx = rsi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P5))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(P5), (1) as u32)) as u32 as u64; bb = 8; continue; }
            8 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = rbx; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            9 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            10 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141621010(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x143930fa8; const P2: u64 = 0x1415f8290; const P3: u64 = 0x28; const P4: u64 = 0x1415c2070; const P5: u64 = 0x18; const P6: u64 = 0x141620d50;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rbx = rcx;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rcx = ((rbx.wrapping_add(P3)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P5), (0x0u64) as u64); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416210b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426903f8; const P1: u64 = 0x142677588; const P2: u64 = 0x8; const P3: u64 = 0x142690678; const P4: u64 = 0x10; const P5: u64 = 0x1426906a0; const P6: u64 = 0x28; const P7: u64 = 0x141649250; const P8: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = P0;
    rbx = ((rdx as u32)) as u32 as u64;
    crate::simd_ops::wu64(rcx, rax);
    rdi = rcx;
    rax = P1;
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    rax = P3;
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rax);
    rax = P5;
    crate::simd_ops::wu64(rcx.wrapping_add(P6), rax);
    rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x118; rcx = rdi; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}
