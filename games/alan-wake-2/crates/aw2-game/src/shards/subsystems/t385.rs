#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn fn_142474760(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426d01e0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x10u64)) as u64; rsi = (crate::simd_ops::ru64(rdx)) as u64; rbx = 0; fl_l = 0; fl_r = 0; r14 = rcx; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 1; continue; }
            1 => { xmm0 = (crate::simd_ops::ru128(r8)) as u128; r10 = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; r15 = P0; crate::simd_ops::wu128(rsp, (xmm0) as u128); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; fl_l = ((r10).wrapping_add(0x20u64)) as u64; fl_r = fl_l; r10 = ((fl_l as u64)) as u64; rbp = (crate::simd_ops::ru64(rsp)) as u64; bb = 2; continue; }
            2 => { rcx = ((crate::simd_ops::ru8(r10.wrapping_sub(0x5u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 5; continue; }
            4 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(r10.wrapping_sub(0x8u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rax = r10; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(r10)) as u64; bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru32(r10.wrapping_sub(0x4u64))) as u32 as u64; fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 8; continue; }
            8 => { rdx = rdi; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 9; continue; }
            9 => { r11 = rbp; fl_l = r11; fl_r = rax; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 10; continue; }
            10 => { rcx = ((crate::simd_ops::ru8(r11.wrapping_add(rax))) as u32) as u32 as u64; rdx = ((rdx).wrapping_sub(1)) as u64; r8 = ((crate::simd_ops::ru8(rcx.wrapping_add(r15))) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rcx.wrapping_add(r15))) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 14; continue; } bb = 11; continue; }
            11 => { if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 14; continue; } bb = 12; continue; }
            12 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu32(r14, ((rbx as u32)) as u32); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 16; continue; }
            14 => { rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; r10 = ((r10).wrapping_add(0x64u64)) as u64; rax = ((((rbx as u32)) as i32 as i64) as u64) as u64; fl_l = rax; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu32(r14, (0x0u32) as u32); rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x10u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424748c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x60;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (((crate::simd_ops::ru32(rcx.wrapping_add(P0))) as i32 as i64) as u64) as u64;
    rax = (((rax) as i64).wrapping_mul((0x64u64) as i64) as u64) as u64;
    fl_l = ((rax).wrapping_add(crate::simd_ops::ru64(rcx))) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    rax = ((crate::simd_ops::ru8(rax.wrapping_add(P1))) as u32) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142474920(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426d01e0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); r8 = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 21; continue; } bb = 1; continue; }
            1 => { rsi = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; r11 = 0; fl_l = 0; fl_r = 0; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 21; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; rdi = rdx.wrapping_add(0x8); r14 = (crate::simd_ops::ru64(rcx)) as u64; r12 = P0; rbx = ((crate::simd_ops::ru8(rdx.wrapping_add(0x3u64))) as u32) as u32 as u64; fl_l = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; rcx = ((r11 as u32)) as u32 as u64; r15 = ((r8 as u32)) as u32 as u64; rbp = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 3; continue; }
            3 => { rax = (((rcx) as i64).wrapping_mul((0x64u64) as i64) as u64) as u64; rax = ((rax).wrapping_add(r14)) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = ((rbx.wrapping_mul(8)) as u32) as u32 as u64; bb = 6; continue; }
            5 => { rcx = ((rbx as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 6; continue; }
            6 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rcx = rdi; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; bb = 8; continue; }
            8 => { r8 = ((crate::simd_ops::ru8(rax.wrapping_add(0x1bu64))) as u32) as u32 as u64; r10 = rax.wrapping_add(0x20); fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { r8 = ((r8.wrapping_mul(8)) as u32) as u32 as u64; bb = 11; continue; }
            10 => { r8 = (((r8 as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x7c00; r8 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru32(rax.wrapping_add(0x18u64))) as u32 as u64; rdx = ((((rdx as u32)) & (0xffffffu32))) as u32 as u64; rdx = (((rdx as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { r10 = (crate::simd_ops::ru64(r10)) as u64; bb = 13; continue; }
            13 => { rdx = (crate::simd_ops::ru32(rax.wrapping_add(0x1cu64))) as u32 as u64; fl_l = rdx; fl_r = r15; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 20; continue; } bb = 14; continue; }
            14 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 15; continue; }
            15 => { fl_l = r10; fl_r = rcx; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 16; continue; }
            16 => { rax = ((crate::simd_ops::ru8(r10.wrapping_add(rcx))) as u32) as u32 as u64; rdx = ((rdx).wrapping_sub(1)) as u64; r8 = ((crate::simd_ops::ru8(rax.wrapping_add(r12))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rax.wrapping_add(r12))) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 20; continue; } bb = 17; continue; }
            17 => { if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 20; continue; } bb = 18; continue; }
            18 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 16; continue; } bb = 19; continue; }
            19 => { rax = ((r11 as u32)) as u32 as u64; bb = 22; continue; }
            20 => { r11 = (((r11 as u32)).wrapping_add(1)) as u32 as u64; rcx = ((((r11 as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 21; continue; }
            21 => { rax = 0xffffffff; bb = 22; continue; }
            22 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142474a60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x62; const P2: u64 = 0x48; const P3: u64 = 0x1401e6ff0; const P4: u64 = 0x1413bb090;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = 0; fl_l = 0; fl_r = 0; r14 = rdx; r15 = rcx; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 10; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); r8 = ((rbp as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(r15)) as u64; r8 = (((r8) as i64).wrapping_mul((0x64u64) as i64) as u64) as u64; fl_l = (crate::simd_ops::ru8(r8.wrapping_add(rax).wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(r14)) as u64; rsi = ((r8.wrapping_add(rax)) as u64) as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(P0))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdi = ((rbx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 4; continue; }
            4 => { bb = 5; continue; }
            5 => { rdx = ((rsi.wrapping_add(P2)) as u64) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { rbx = ((rbx).wrapping_add(0x18u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 7; continue; }
            7 => { rdx = ((rsi.wrapping_add(P2)) as u64) as u64; rcx = r14; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru32(r15.wrapping_add(P0))) as u32 as u64; rbp = (((rbp as u32)).wrapping_add(1)) as u32 as u64; r8 = ((((rbp as u32)) as i32 as i64) as u64) as u64; fl_l = r8; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 9; continue; }
            9 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 10; continue; }
            10 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142474b20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x62; const P2: u64 = 0x18; const P3: u64 = 0x1413bb090;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = 0; fl_l = 0; fl_r = 0; rsi = rdx; rdi = rcx; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 5; continue; } bb = 1; continue; }
            1 => { r8 = ((rbx as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdi)) as u64; r8 = (((r8) as i64).wrapping_mul((0x64u64) as i64) as u64) as u64; fl_l = (crate::simd_ops::ru8(r8.wrapping_add(rax).wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = ((rax.wrapping_add(P2)) as u64) as u64; rcx = rsi; rdx = ((rdx).wrapping_add(r8)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; r8 = ((((rbx as u32)) as i32 as i64) as u64) as u64; fl_l = r8; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142474b90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142409b50; const P1: u64 = 0x1426d01e0; const P2: u64 = 0x14240ab10;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rbx = rdx; r14 = rcx; rax = (((crate::simd_ops::ru32(rcx.wrapping_add(0x14u64))) as i32 as i64) as u64) as u64; r8 = (((rax) as i64).wrapping_mul((0x64u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = ((rax).wrapping_add(0x18u64)) as u64; rax = ((rax).wrapping_add(r8)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x2000002u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x0u8) as u8); rcx = rsp.wrapping_add(0x30); fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 1; continue; }
            1 => { r9 = (crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as u32 as u64; rdx = rax.wrapping_add(0x8); rcx = ((crate::simd_ops::ru8(rax.wrapping_add(0x3u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 4; continue; }
            3 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r9); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            7 => { rsi = rbx.wrapping_add(0x8); rcx = ((crate::simd_ops::ru8(rbx.wrapping_add(0x3u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 10; continue; }
            9 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rsi = (crate::simd_ops::ru64(rsi)) as u64; bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32 as u64; rbp = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; r10 = (crate::simd_ops::ru64(r14)) as u64; rdi = 0; fl_l = 0; fl_r = 0; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 13; continue; }
            13 => { fl_l = ((r10).wrapping_add(0x20u64)) as u64; fl_r = fl_l; r10 = ((fl_l as u64)) as u64; r15 = P1; bb = 14; continue; }
            14 => { rcx = ((crate::simd_ops::ru8(r10.wrapping_sub(0x5u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 17; continue; }
            16 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(r10.wrapping_sub(0x8u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rax = r10; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(r10)) as u64; bb = 19; continue; }
            19 => { rcx = (crate::simd_ops::ru32(r10.wrapping_sub(0x4u64))) as u32 as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 26; continue; } bb = 20; continue; }
            20 => { rdx = rbx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 21; continue; }
            21 => { r11 = rsi; fl_l = r11; fl_r = rax; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 22; continue; }
            22 => { rdx = ((rdx).wrapping_sub(1)) as u64; rcx = ((crate::simd_ops::ru8(rax.wrapping_add(r11))) as u32) as u32 as u64; r8 = ((crate::simd_ops::ru8(rcx.wrapping_add(r15))) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rcx.wrapping_add(r15))) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 26; continue; } bb = 23; continue; }
            23 => { if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 26; continue; } bb = 24; continue; }
            24 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 22; continue; } bb = 25; continue; }
            25 => { rbx = (rbx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 28; continue; }
            26 => { rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; r10 = ((r10).wrapping_add(0x64u64)) as u64; rax = ((((rdi as u32)) as i32 as i64) as u64) as u64; fl_l = rax; fl_r = rbp; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 14; continue; } bb = 27; continue; }
            27 => { rbx = (rbx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rdi = 0; fl_l = 0; fl_r = 0; bb = 28; continue; }
            28 => { crate::simd_ops::wu32(r14.wrapping_add(0x14u64), ((rdi as u32)) as u32); rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (((rbx as u8)) as u32) as u32 as u64; r11 = rsp.wrapping_add(0x50); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142474d50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142409b50; const P1: u64 = 0x1426d01e0; const P2: u64 = 0x14240ab10;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rbx = rdx; r14 = rcx; rax = (((crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as i32 as i64) as u64) as u64; r8 = (((rax) as i64).wrapping_mul((0x64u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = ((rax).wrapping_add(0x18u64)) as u64; rax = ((rax).wrapping_add(r8)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x2000002u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x0u8) as u8); rcx = rsp.wrapping_add(0x30); fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 1; continue; }
            1 => { r9 = (crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as u32 as u64; rdx = rax.wrapping_add(0x8); rcx = ((crate::simd_ops::ru8(rax.wrapping_add(0x3u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 4; continue; }
            3 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r9); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            7 => { rsi = rbx.wrapping_add(0x8); rcx = ((crate::simd_ops::ru8(rbx.wrapping_add(0x3u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 10; continue; }
            9 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rsi = (crate::simd_ops::ru64(rsi)) as u64; bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32 as u64; rbp = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; r10 = (crate::simd_ops::ru64(r14)) as u64; rdi = 0; fl_l = 0; fl_r = 0; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 13; continue; }
            13 => { fl_l = ((r10).wrapping_add(0x20u64)) as u64; fl_r = fl_l; r10 = ((fl_l as u64)) as u64; r15 = P1; bb = 14; continue; }
            14 => { rcx = ((crate::simd_ops::ru8(r10.wrapping_sub(0x5u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 17; continue; }
            16 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(r10.wrapping_sub(0x8u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rax = r10; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(r10)) as u64; bb = 19; continue; }
            19 => { rcx = (crate::simd_ops::ru32(r10.wrapping_sub(0x4u64))) as u32 as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 26; continue; } bb = 20; continue; }
            20 => { rdx = rbx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 21; continue; }
            21 => { r11 = rsi; fl_l = r11; fl_r = rax; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 22; continue; }
            22 => { rdx = ((rdx).wrapping_sub(1)) as u64; rcx = ((crate::simd_ops::ru8(rax.wrapping_add(r11))) as u32) as u32 as u64; r8 = ((crate::simd_ops::ru8(rcx.wrapping_add(r15))) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rcx.wrapping_add(r15))) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 26; continue; } bb = 23; continue; }
            23 => { if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 26; continue; } bb = 24; continue; }
            24 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 22; continue; } bb = 25; continue; }
            25 => { rbx = (rbx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 28; continue; }
            26 => { rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; r10 = ((r10).wrapping_add(0x64u64)) as u64; rax = ((((rdi as u32)) as i32 as i64) as u64) as u64; fl_l = rax; fl_r = rbp; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 14; continue; } bb = 27; continue; }
            27 => { rbx = (rbx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rdi = 0; fl_l = 0; fl_r = 0; bb = 28; continue; }
            28 => { crate::simd_ops::wu32(r14.wrapping_add(0x10u64), ((rdi as u32)) as u32); rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (((rbx as u8)) as u32) as u32 as u64; r11 = rsp.wrapping_add(0x50); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142474f10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1424745f0; const P1: u64 = 0x142471850; const P2: u64 = 0x1424704d0; const P3: u64 = 0x142473ba0;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rcx = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142474fb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1424b5a90; const P1: u64 = 0x14340ed98; const P2: u64 = 0x142495870; const P3: u64 = 0x142499920; const P4: u64 = 0x14020c080; const P5: u64 = 0x14183acbc; const P6: u64 = 0x142442de0; const P7: u64 = 0x14240ab10;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 57>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0xa0); rsp = ((rsp).wrapping_sub(0x1a0u64)) as u64; r15 = r8; r14 = rdx; rdi = rcx; rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((rsi as u32)) as u32); fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x18u64))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 42; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru32(rcx.wrapping_add(0x20u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 42; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x24u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 42; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu16(rsp.wrapping_add(0x20u64), ((rsi as u16)) as u16); crate::simd_ops::wu8(rsp.wrapping_add(0x22u64), (0x2u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x23u64), ((rsi as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x27u64), ((rsi as u8)) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rsi as u32)) as u32); crate::simd_ops::wu16(rsp.wrapping_add(0x2cu64), ((rcx as u16)) as u16); crate::simd_ops::wu16(rsp.wrapping_add(0x2eu64), ((rax as u16)) as u16); crate::simd_ops::wu16(rsp.wrapping_add(0x30u64), (0x2020u16) as u16); rax = ((((rax as u32)) as i32).wrapping_mul(((rcx as u32)) as i32) as u32) as u32 as u64; rax = ((rax.wrapping_mul(4).wrapping_add(0x12u64)) as u32) as u32 as u64; crate::simd_ops::wu32(r8, ((rax as u32)) as u32); rdx = ((((rax as u32)) as i32 as i64) as u64) as u64; rcx = rsp.wrapping_add(0x60); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = rsp.wrapping_add(0x60); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rax = P1; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x4cu64), ((rsi as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x48u64), ((rsi as u8)) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r8 = ((rsi.wrapping_add(0x1u64)) as u32) as u32 as u64; rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x60); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_add(0x118u64))) as u64; r12 = P2; rdx = rsp.wrapping_add(0x21); fl_l = r8; fl_r = r12; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = ((rsi.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            5 => { rcx = rsp.wrapping_add(0x38); rax = crate::simd_ops::call_stk(r8, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_add(0x118u64))) as u64; rdx = rsp.wrapping_add(0x22); fl_l = r8; fl_r = r12; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = 0x1; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            8 => { rcx = rsp.wrapping_add(0x38); rax = crate::simd_ops::call_stk(r8, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_add(0xf8u64))) as u64; rbx = P3; rdx = rsp.wrapping_add(0x23); rcx = rsp.wrapping_add(0x38); fl_l = r8; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            11 => { rax = crate::simd_ops::call_stk(r8, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_add(0xf8u64))) as u64; rdx = rsp.wrapping_add(0x25); rcx = rsp.wrapping_add(0x38); fl_l = r8; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            14 => { rax = crate::simd_ops::call_stk(r8, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_add(0x118u64))) as u64; rdx = rsp.wrapping_add(0x27); fl_l = r8; fl_r = r12; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = 0x1; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 18; continue; }
            17 => { rcx = rsp.wrapping_add(0x38); rax = crate::simd_ops::call_stk(r8, rax, rcx, rdx, r8, r9, rsp); bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_add(0xf8u64))) as u64; rdx = rsp.wrapping_add(0x28); rcx = rsp.wrapping_add(0x38); fl_l = r8; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 21; continue; }
            20 => { rax = crate::simd_ops::call_stk(r8, rax, rcx, rdx, r8, r9, rsp); bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_add(0xf8u64))) as u64; rdx = rsp.wrapping_add(0x2a); rcx = rsp.wrapping_add(0x38); fl_l = r8; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 24; continue; }
            23 => { rax = crate::simd_ops::call_stk(r8, rax, rcx, rdx, r8, r9, rsp); bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_add(0xf8u64))) as u64; rdx = rsp.wrapping_add(0x2c); rcx = rsp.wrapping_add(0x38); fl_l = r8; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 26; continue; } bb = 25; continue; }
            25 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 27; continue; }
            26 => { rax = crate::simd_ops::call_stk(r8, rax, rcx, rdx, r8, r9, rsp); bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_add(0xf8u64))) as u64; rdx = rsp.wrapping_add(0x2e); rcx = rsp.wrapping_add(0x38); fl_l = r8; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 29; continue; } bb = 28; continue; }
            28 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 30; continue; }
            29 => { rax = crate::simd_ops::call_stk(r8, rax, rcx, rdx, r8, r9, rsp); bb = 30; continue; }
            30 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_add(0x118u64))) as u64; rdx = rsp.wrapping_add(0x30); fl_l = r8; fl_r = r12; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 32; continue; } bb = 31; continue; }
            31 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = 0x1; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 33; continue; }
            32 => { rcx = rsp.wrapping_add(0x38); rax = crate::simd_ops::call_stk(r8, rax, rcx, rdx, r8, r9, rsp); bb = 33; continue; }
            33 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r9 = (crate::simd_ops::ru64(rax.wrapping_add(0x118u64))) as u64; rdx = rsp.wrapping_add(0x31); fl_l = r9; fl_r = r12; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 35; continue; } bb = 34; continue; }
            34 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = 0x1; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 36; continue; }
            35 => { rcx = rsp.wrapping_add(0x38); rax = crate::simd_ops::call_stk(r9, rax, rcx, rdx, r8, r9, rsp); bb = 36; continue; }
            36 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul((crate::simd_ops::ru32(rdi.wrapping_add(0x24u64))) as i32) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 39; continue; } bb = 37; continue; }
            37 => { bb = 38; continue; }
            38 => { rax = ((((rsi as u32)) as i32 as i64) as u64) as u64; rbx = ((rax.wrapping_mul(4)) as u64) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rdx = ((rdx).wrapping_add(rbx)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rcx = rsp.wrapping_add(0x38); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x110u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rdx = ((rdx).wrapping_add(1)) as u64; rdx = ((rdx).wrapping_add(rbx)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rcx = rsp.wrapping_add(0x38); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x110u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rdx = ((rdx).wrapping_add(0x2u64)) as u64; rdx = ((rdx).wrapping_add(rbx)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rcx = rsp.wrapping_add(0x38); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x110u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rdx = ((rdx).wrapping_add(0x3u64)) as u64; rdx = ((rdx).wrapping_add(rbx)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rcx = rsp.wrapping_add(0x38); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x110u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul((crate::simd_ops::ru32(rdi.wrapping_add(0x24u64))) as i32) as u32) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 38; continue; } bb = 39; continue; }
            39 => { r9 = (((crate::simd_ops::ru32(r15)) as i32 as i64) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rbx = rax; r8 = (((crate::simd_ops::ru32(r15)) as i32 as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x78u64))) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(r14, rbx); crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), (0x7u32) as u32); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x78u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 41; continue; } bb = 40; continue; }
            40 => { rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 41; continue; }
            41 => { rcx = rsp.wrapping_add(0x68); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 43; continue; }
            42 => { crate::simd_ops::wu64(rdx, rsi); bb = 43; continue; }
            43 => { rax = r14; r11 = rsp.wrapping_add(0x1a0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142475360(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x24; const P2: u64 = 0x10; const P3: u64 = 0x110;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rbp = 0; fl_l = 0; fl_r = 0; rax = ((((rax as u32)) as i32).wrapping_mul((crate::simd_ops::ru32(rcx.wrapping_add(P1))) as i32) as u32) as u32 as u64; rsi = rdx; rbx = rcx; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rcx = rsi; r8 = (crate::simd_ops::ru64(rsi)) as u64; rdx = ((rdx).wrapping_add(0x2u64)) as u64; rax = ((((rbp as u32)) as i32 as i64) as u64) as u64; rdi = ((rax.wrapping_mul(4)) as u64) as u64; rdx = ((rdx).wrapping_add(rdi)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r8.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rcx = rsi; r8 = (crate::simd_ops::ru64(rsi)) as u64; rdx = ((rdx).wrapping_add(1)) as u64; rdx = ((rdx).wrapping_add(rdi)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r8.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rcx = rsi; r8 = (crate::simd_ops::ru64(rsi)) as u64; rdx = ((rdx).wrapping_add(rdi)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r8.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rcx = rsi; r8 = (crate::simd_ops::ru64(rsi)) as u64; rdx = ((rdx).wrapping_add(0x3u64)) as u64; rdx = ((rdx).wrapping_add(rdi)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r8.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; rbp = (((rbp as u32)).wrapping_add(1)) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul((crate::simd_ops::ru32(rbx.wrapping_add(P1))) as i32) as u32) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142475420(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x24; const P2: u64 = 0x143426658; const P3: u64 = 0x143426810; const P4: u64 = 0x14342657c; const P5: u64 = 0x10; const P6: u64 = 0x2; const P7: u64 = 0x1; const P8: u64 = 0x110;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); crate::simd_ops::wu8(rsp.wrapping_add(0x18u64), ((r8 as u8)) as u8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rbp = 0; fl_l = 0; fl_r = 0; rax = ((((rax as u32)) as i32).wrapping_mul((crate::simd_ops::ru32(rcx.wrapping_add(P1))) as i32) as u32) as u32 as u64; rsi = rdx; rdi = rcx; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm6) as u128); xmm6 = ((crate::simd_ops::ru32(P2)) as u32) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm7) as u128); xmm7 = ((crate::simd_ops::ru32(P3)) as u32) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm8) as u128); xmm8 = ((crate::simd_ops::ru32(P4)) as u32) as u128; bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64; rdx = rsp.wrapping_add(0x70); xmm0 = 0u128; xmm1 = 0u128; rbx = ((((rbp as u32)) as i32 as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(rbx.wrapping_mul(4)).wrapping_add(P6))) as u32) as u32 as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128); rax = ((crate::simd_ops::ru8(rcx.wrapping_add(rbx.wrapping_mul(4)).wrapping_add(P7))) as u32) as u32 as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128); rax = ((crate::simd_ops::ru8(rcx.wrapping_add(rbx.wrapping_mul(4)))) as u32) as u32 as u64; rcx = rsi; xmm2 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm0 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm3 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm1 = 0u128; xmm1 = (xmm1 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128); xmm0 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm2 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm2 as u32)))) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x70u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rsi)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P8))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64; rcx = rsi; r8 = (crate::simd_ops::ru64(rsi)) as u64; rdx = ((rdx).wrapping_add(0x3u64)) as u64; rdx = ((rdx.wrapping_add(rbx.wrapping_mul(4))) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(r8.wrapping_add(P8))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; rbp = (((rbp as u32)).wrapping_add(1)) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul((crate::simd_ops::ru32(rdi.wrapping_add(P0))) as i32) as u32) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 3; continue; }
            3 => { xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; bb = 4; continue; }
            4 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142475560(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14020c080; const P1: u64 = 0x142442de0; const P2: u64 = 0x8; const P3: u64 = 0x14183acbc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = ((((r8 as u32)) as i32 as i64) as u64) as u64; rbp = rdx; rbx = rcx; r9 = rdi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x20); fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rbx, rax); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            3 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { crate::simd_ops::wu32(rbx.wrapping_add(P2), ((rdi as u32)) as u32); r8 = rdi; rdx = rbp; rcx = (crate::simd_ops::ru64(rbx)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424755e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14020c080; const P1: u64 = 0x142442de0; const P2: u64 = 0x8; const P3: u64 = 0x14183acbc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = ((((r9 as u32)) as i32 as i64) as u64) as u64; rbp = r8; rbx = rdx; r9 = rdi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x20); fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rbx, rax); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            3 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { crate::simd_ops::wu32(rbx.wrapping_add(P2), ((rdi as u32)) as u32); r8 = rdi; rdx = rbp; rcx = (crate::simd_ops::ru64(rbx)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142475680(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1424b5a90; const P1: u64 = 0x14340ed98; const P2: u64 = 0x142495830; const P3: u64 = 0x14240da50; const P4: u64 = 0x14020c080; const P5: u64 = 0x14183acbc; const P6: u64 = 0x142646558; const P7: u64 = 0x142442de0; const P8: u64 = 0x14240ab10;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 59>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((r9 as u8)) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0xa0); rsp = ((rsp).wrapping_sub(0x1a0u64)) as u64; r13 = r8; r12 = rdx; rbx = rcx; r15 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((r15 as u32)) as u32); fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x18u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 24; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru32(rcx.wrapping_add(0x20u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 24; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x24u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 24; continue; } bb = 3; continue; }
            3 => { rax = ((((rax as u32)) as i32).wrapping_mul(((rcx as u32)) as i32) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_shl(2u32)) as u32 as u64; rdx = ((((rax as u32)) as i32 as i64) as u64) as u64; rcx = rsp.wrapping_add(0x60); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x60); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx); r10 = P1; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r10); crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), ((r15 as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x40u64), ((r15 as u8)) as u8); rsi = ((r15 as u32)) as u32 as u64; r9 = (crate::simd_ops::ru32(rbx.wrapping_add(0x24u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32 as u64; rax = ((r9 as u32)) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul(((r8 as u32)) as i32) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 19; continue; } bb = 4; continue; }
            4 => { r14 = P2; bb = 5; continue; }
            5 => { rdi = ((((rsi as u32)) as i32 as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdx = ((rdx).wrapping_add(0x2u64)) as u64; rdx = ((rdx.wrapping_add(rdi.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(r10.wrapping_add(0x110u64))) as u64; fl_l = rax; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = 0x1; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            7 => { rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(rax, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdx = ((rdx).wrapping_add(1)) as u64; rdx = ((rdx.wrapping_add(rdi.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_add(0x110u64))) as u64; fl_l = r8; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = 0x1; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            10 => { rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(r8, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdx = ((rax.wrapping_add(rdi.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_add(0x110u64))) as u64; fl_l = r8; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = 0x1; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            13 => { rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(r8, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdx = ((rdx).wrapping_add(0x3u64)) as u64; rdx = ((rdx.wrapping_add(rdi.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_add(0x110u64))) as u64; fl_l = r8; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = 0x1; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            16 => { rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(r8, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            17 => { rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; r9 = (crate::simd_ops::ru32(rbx.wrapping_add(0x24u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32 as u64; rax = ((r9 as u32)) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul(((r8 as u32)) as i32) as u32) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 19; continue; } bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 5; continue; }
            19 => { rsi = r15; rdi = ((r15 as u32)) as u32 as u64; rdx = ((r8.wrapping_mul(4)) as u32) as u32 as u64; rax = rbp.wrapping_add(0xf8); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x4u32) as u32); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x78u64))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r14 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { rdi = (((crate::simd_ops::ru32(rbp.wrapping_add(0xf8u64))) as i32 as i64) as u64) as u64; r9 = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rsi = rax; r15 = 0x6; r8 = rdi; rdx = r14; rcx = rax; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = r14; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); bb = 21; continue; }
            21 => { crate::simd_ops::wu32(r13, ((rdi as u32)) as u32); crate::simd_ops::wu64(r12, rsi); r15 = ((((r15 as u32)) | (0x1u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((r15 as u32)) as u32); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x78u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 23; continue; }
            23 => { rcx = rsp.wrapping_add(0x68); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 25; continue; }
            24 => { crate::simd_ops::wu64(rdx, r15); bb = 25; continue; }
            25 => { rax = r12; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x1e0u64))) as u64; rsp = ((rsp).wrapping_add(0x1a0u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424759a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143871588; const P1: u64 = 0x1401e42d0; const P2: u64 = 0x1438715a0; const P3: u64 = 0x143426fac; const P4: u64 = 0x143426dd8; const P5: u64 = 0x1423f0a10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbx = rdx; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 1; continue; }
            1 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm5 = (xmm0) as u128; xmm3 = 0u128; fl_l = rbx; fl_r = rbx; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { xmm3 = (xmm3 & !0xffff_ffff_ffff_ffffu128) | ((((rbx) as i64) as f64).to_bits() as u128); bb = 4; continue; }
            3 => { rax = rbx; rbx = ((((rbx as u32)) & (0x1u32))) as u32 as u64; rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = (((rax) | (rbx))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; xmm3 = (xmm3 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm3 as u64) + f64::from_bits(xmm3 as u64)).to_bits() as u128); bb = 4; continue; }
            4 => { xmm0 = ((crate::simd_ops::ru32(P2)) as u32) as u128; xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P3))).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | (((f32::from_bits(xmm1 as u32)) as f64).to_bits() as u128); xmm4 = (xmm3 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm3 as u64) / f64::from_bits(xmm2 as u64)).to_bits() as u128); fl_l = ((xmm4 as u64)) as u64; fl_r = ((xmm5 as u64)) as u64; if !(f64::from_bits(fl_l) > f64::from_bits(fl_r)) { bb = 6; continue; } bb = 5; continue; }
            5 => { xmm0 = (xmm4 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm4 as u64) - f64::from_bits(xmm5 as u64)).to_bits() as u128); xmm1 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) * f64::from_bits(crate::simd_ops::ru64(P4))).to_bits() as u128); xmm1 = crate::simd_ops::vroundsd(xmm1, (xmm1 as u64), 0xcu8); rax = (crate::simd_ops::cvt_f64_i32((f64::from_bits(xmm1 as u64)).round_ties_even())) as u32 as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            6 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142475a40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = (crate::simd_ops::ru32(rax)) as u32 as u64;
    fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64;
    rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142475a60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426455a8; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x1426455b8; const P4: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = ((rcx).wrapping_add(0x8u64)) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rbx)) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { rcx = ((rbx.wrapping_add(P1)) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); }
    loop { r9 = 0; fl_l = 0; fl_r = 0; rdx = ((rbx.wrapping_add(P1)) as u64) as u64; r8 = 0xffffffff; rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if !(((((fl_l as u32)) & ((fl_r as u32))) != 0)) { break; } }
    rcx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142475ae0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426455a8; const P1: u64 = 0x10; const P2: u64 = 0x1426455c0; const P3: u64 = 0x8; const P4: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rcx;
    rcx = ((rcx).wrapping_add(0x8u64)) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = 0xffffffff;
    let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64;
    rax = (xa_old) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rcx = ((rdi.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); }
    rcx = ((rdi.wrapping_add(P3)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142475b30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(rdx.wrapping_add(P0), ((rax as u8)) as u8);
    rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142475b40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = 0x1;
    crate::simd_ops::wu64(rcx, rdx);
    let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(rdx.wrapping_add(P0), ((rax as u8)) as u8);
    rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64));
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142475be0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1431edc44; const P1: u64 = 0x142498620;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    r8 = rdx;
    rdx = P0;
    return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142475c10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143871620; const P1: u64 = 0x1413bb930; const P2: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x142475c10, funcinfo_rva: 0x37e8bd4, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x28, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (rcx) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 1; continue; }
            1 => { fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x1u32) as u32); r9 = ((rsp.wrapping_add(0x38u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142475c44); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rax, (rbx) as u64); rbx = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rdi, (rdx) as u64); fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142475c62); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142475c67); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142475c7f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142475c84); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            6 => { fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rcx, (0x0u64) as u64); rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142475ca9); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142475cae); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247bca0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a08320; const P1: u64 = 0x1426cdcb0; const P2: u64 = 0x1401df850; const P3: u64 = 0x1401e06c0; const P4: u64 = 0x14240ab10; const P5: u64 = 0x14340cc20; const P6: u64 = 0x1423fdfc0; const P7: u64 = 0x1401feaf0; const P8: u64 = 0x143079330; const P9: u64 = 0x141461fb0; const P10: u64 = 0x1431edc44; const P11: u64 = 0x142498620; const P12: u64 = 0x143231690; const P13: u64 = 0x143a082f0; const P14: u64 = 0x14247e740; const P15: u64 = 0x1401df790; const P16: u64 = 0x143871588; const P17: u64 = 0x1401e42d0; const P18: u64 = 0x1438715a0; const P19: u64 = 0x143426fac; const P20: u64 = 0x143426dd8; const P21: u64 = 0x1423f0a10; const P22: u64 = 0x14240a730; const P23: u64 = 0x14247a740; const P24: u64 = 0x14101f7d0; const P25: u64 = 0x1413bbda0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 105>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x14247bca0, funcinfo_rva: 0x37e983c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x348, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rsp.wrapping_sub(0x208u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x308u64)) as u64; r15 = (rdx) as u64; r12 = (rcx) as u64; rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbp.wrapping_add(0x250u64), ((rbx as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x258u64), ((rbx as u32)) as u32); rsi = (crate::simd_ops::ru64(P0)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x268u64), (rsi) as u64); rdi = ((rdx.wrapping_add(0x8u64)) as u64) as u64; rdx = ((crate::simd_ops::ru8(rdx.wrapping_add(0x3u64))) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = ((rdx.wrapping_mul(8)) as u32) as u32 as u64; bb = 3; continue; }
            2 => { rdx = (((rdx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(r15)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rcx = (rdi) as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(r15.wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rcx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rax) as u64); rax = ((P1) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (rbx) as u64); rdx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x80u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247bd3d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x80u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x80u64), (xmm1) as u128); r8 = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rdx = ((rbp.wrapping_sub(0x80u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247bd65); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rsi)) as u64; r9 = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; rdx = ((crate::simd_ops::ru8(rbp.wrapping_sub(0x3du64))) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = ((rdx.wrapping_mul(8)) as u32) as u32 as u64; bb = 8; continue; }
            7 => { rdx = (((rdx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x40u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; rcx = ((rbp.wrapping_sub(0x38u64)) as u64) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rcx = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rbp.wrapping_sub(0x38u64)) } else { rcx }) as u64; rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x3cu64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rcx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rax) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x80u64), (xmm0) as u128); r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rbp.wrapping_sub(0x80u64)) as u64) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247bdc8); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((r9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r13 = (rax) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x260u64), (rax) as u64); rcx = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247bdd9); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r14 = ((r15.wrapping_add(0x18u64)) as u64) as u64; rcx = (crate::simd_ops::ru64(rsi)) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x90u64))) as u64; rdx = (r13) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247bdf2); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((r8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(r14.wrapping_add(0x30u64))) as u128; ymm0_hi = 0; rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x40u64))) as u64; rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x48u64))) as u64; rax = ((rax).wrapping_sub(rdx)) as u64; rcx = ((rcx).wrapping_sub(rdx)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rcx }) as u64; fl_l = (crate::simd_ops::ru64(r14.wrapping_add(0x38u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = ((xmm0 as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rcx }) as u64; bb = 10; continue; }
            10 => { crate::simd_ops::wu64(r14.wrapping_add(0x40u64), (rdx) as u64); rax = ((rcx.wrapping_add(rdx)) as u64) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x48u64), (rax) as u64); r8 = (crate::simd_ops::ru64(r14.wrapping_add(0x40u64))) as u64; fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rsi)) as u64; r9 = (0x1u32) as u32 as u64; rdx = (r13) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247be47); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x80u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 12; continue; }
            12 => { rcx = ((crate::simd_ops::ru8(r15.wrapping_add(0x3u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 15; continue; }
            14 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru32(r15)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rdi = (crate::simd_ops::ru64(rdi)) as u64; bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(r15.wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rdi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rax) as u64); r12 = (crate::simd_ops::ru64(r12.wrapping_add(0x138u64))) as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x80u64), (xmm0) as u128); crate::simd_ops::wu64(rbp.wrapping_add(0x258u64), (r12) as u64); fl_l = (crate::simd_ops::ru64(r12)) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 21; continue; } bb = 18; continue; }
            18 => { rsi = (crate::simd_ops::ru32(r14.wrapping_add(0xcu64))) as u32 as u64; rdi = (crate::simd_ops::ru64(r14.wrapping_add(0x48u64))) as u64; rdi = ((rdi).wrapping_sub(crate::simd_ops::ru64(r14.wrapping_add(0x40u64)))) as u64; rax = ((P5) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (0xcu64) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247bed2); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r13 = (rax) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), (rdi) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x70u64), ((rsi as u32)) as u32); rax = ((rbp.wrapping_sub(0x80u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); rax = ((P7) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rax) as u64); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x10u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x70u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0xf14u64) as u64); rax = ((rsp.wrapping_add(0x40u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rax) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x80u64), (rbx) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x88u64), (0x7ffu64) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x68u64), (r13) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x70u64), (rbx) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x78u64), (0x7ffu64) as u64); rax = ((P8) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x60u64), (rax) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x90u64), (r13) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x70u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x10u64), (xmm1) as u128); r9 = (rbx) as u64; r8 = ((rbp.wrapping_sub(0x70u64)) as u64) as u64; rdx = ((rbp.wrapping_sub(0x10u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247bf84); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x88u64))) as u64; rdx = (rcx) as u64; rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(rbp.wrapping_add(0x80u64)))) as u64; r9 = (rbx) as u64; fl_l = (rcx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbp.wrapping_add(0x80u64))) as u64 as u64; r9 = (if (((fl_l as u64)) > ((fl_r as u64))) { rdx } else { r9 }) as u64; fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x70u64))) as u64 as u64; fl_r = (r9) as u64 as u64; r9 = (if (((fl_l as u64)) < ((fl_r as u64))) { crate::simd_ops::ru64(rbp.wrapping_add(0x70u64)) } else { r9 }) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x90u64))) as u64; fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x68u64))) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 20; continue; } bb = 19; continue; }
            19 => { fl_l = ((rax).wrapping_add(r9)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 20; continue; }
            20 => { crate::simd_ops::wu8(rax, (0x0u8) as u8); r8 = (r13) as u64; rdx = ((P10) as u64) as u64; rcx = (crate::simd_ops::ru64(r12)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247bfd2); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(r12)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { r8 = ((P12) as u64) as u64; rdx = ((P10) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247bfef); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 23; continue; }
            23 => { fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 25; continue; } bb = 24; continue; }
            24 => { rcx = (crate::simd_ops::ru64(P13)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247c003); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P14) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 25; continue; }
            25 => { fl_l = (crate::simd_ops::ru64(r14.wrapping_add(0x38u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 27; continue; } bb = 26; continue; }
            26 => { xmm0 = (crate::simd_ops::ru128(r14.wrapping_add(0x30u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (rbx) as u64); r13 = (0x10u32) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x60u64), ((r13 as u16)) as u16); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (rbx) as u64); r12 = (crate::simd_ops::ru64(r14.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (r12) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x250u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x258u64), (rbx) as u64); rsi = (rbx) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (rbx) as u64); bb = 28; continue; }
            27 => { rsi = (crate::simd_ops::ru64(r14.wrapping_add(0x48u64))) as u64; rsi = ((rsi).wrapping_sub(crate::simd_ops::ru64(r14.wrapping_add(0x40u64)))) as u64; rdi = (crate::simd_ops::ru64(r14.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (rdi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (rbx) as u64); r13 = (0x10u32) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x60u64), ((r13 as u16)) as u16); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (rbx) as u64); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x250u64))) as u32 as u64; rax = ((((rax as u32)) | (0x4u32))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x258u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdi)) as u64; r8 = ((r13 as u32)) as u32 as u64; rdx = (rsi) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247c0ac); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r12 = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (rsi) as u64); crate::simd_ops::wu16(rsp.wrapping_add(0x60u64), ((r13 as u16)) as u16); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (rdi) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x250u64), (rsi) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x258u64), (rax) as u64); bb = 28; continue; }
            28 => { xmm0 = 0u128; ymm0_hi = 0; rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let y_addr: u64 = rbp.wrapping_sub(0x70u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), (rax) as u64); rcx = ((rbp.wrapping_sub(0x70u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247c0f5); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P15) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x268u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r9 = (crate::simd_ops::ru64(r14.wrapping_add(0x48u64))) as u64; r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(r14.wrapping_add(0x40u64)))) as u64; r8 = (r12) as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x260u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247c116); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x98u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (crate::simd_ops::ru64(r14.wrapping_add(0x48u64))) as u64; rdi = ((rdi).wrapping_sub(crate::simd_ops::ru64(r14.wrapping_add(0x40u64)))) as u64; fl_l = (crate::simd_ops::ru8(P16)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 34; continue; } bb = 29; continue; }
            29 => { rcx = ((rbp.wrapping_sub(0x70u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247c131); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P17) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm3 = (xmm0) as u128; xmm1 = ((crate::simd_ops::ru32(P18)) as u32) as u128; xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(core::ptr::read_unaligned((P19) as *const u32))).to_bits() as u128); xmm0 = (xmm2 & !0xffff_ffff_ffff_ffffu128) | (((f32::from_bits(xmm2 as u32)) as f64).to_bits() as u128); ymm0_hi = 0; xmm1 = 0u128; fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { bb = 31; continue; } bb = 30; continue; }
            30 => { xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((((rdi) as i64) as f64).to_bits() as u128); bb = 32; continue; }
            31 => { rax = (rdi) as u64; rax = ((rax).wrapping_shr(1u32)) as u64; rdi = ((((rdi as u32)) & (0x1u32))) as u32 as u64; fl_l = (((rax) | (rdi))) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) + f64::from_bits(xmm1 as u64)).to_bits() as u128); bb = 32; continue; }
            32 => { xmm0 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) / f64::from_bits(xmm0 as u64)).to_bits() as u128); ymm0_hi = 0; fl_l = ((xmm0 as u64)) as u64; fl_r = ((xmm3 as u64)) as u64; if !(f64::from_bits(fl_l) > f64::from_bits(fl_r)) { bb = 34; continue; } bb = 33; continue; }
            33 => { xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) - f64::from_bits(xmm3 as u64)).to_bits() as u128); ymm0_hi = 0; xmm1 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) * f64::from_bits(core::ptr::read_unaligned((P20) as *const u64))).to_bits() as u128); xmm3 = crate::simd_ops::vroundsd(xmm1, (xmm1 as u64), 0xcu8); rax = (crate::simd_ops::cvt_f64_i32((f64::from_bits(xmm3 as u64)).round_ties_even())) as u32 as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247c196); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P21) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 34; continue; }
            34 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x268u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x260u64))) as u64; rdx = ((rbp.wrapping_add(0x268u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247c1b3); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0xb8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x260u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247c1cd); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x260u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rbx) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), (r12) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rbx) as u64); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x250u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x38u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (rbx) as u64); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x258u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (rbx) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), (rsi) as u64); crate::simd_ops::wu16(rsp.wrapping_add(0x60u64), ((r13 as u16)) as u16); crate::simd_ops::wu16(rbp.wrapping_sub(0x20u64), ((r13 as u16)) as u16); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (rbx) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), (rax) as u64); rax = ((rbp.wrapping_sub(0x70u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x250u64), (rax) as u64); rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x70u64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000002u32))) as u32 as u64; rax = ((((rax as u32)) | (0x2u32))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x70u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_sub(0x6du64), (0x2u8) as u8); crate::simd_ops::wu32(rbp.wrapping_sub(0x6cu64), ((rbx as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_sub(0x68u64), (0x0u8) as u8); rdx = (r15) as u64; rcx = ((rbp.wrapping_sub(0x70u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247c251); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P22) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r14) as u64); r9 = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; r8 = (rdi) as u64; rdx = ((rbp.wrapping_sub(0x70u64)) as u64) as u64; rcx = ((rbp) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247c26b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P23) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((rbp) as u64) as u64; rcx = ((rbp.wrapping_add(0x1a0u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247c27c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P24) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (rax) as u64; rcx = ((r14.wrapping_add(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247c288); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P25) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 36; continue; } bb = 35; continue; }
            35 => { r9 = ((crate::simd_ops::ru16(rbp.wrapping_add(0x40u64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x48u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247c2a7); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 36; continue; }
            36 => { rcx = ((rbp) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247c2af); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rsp = ((rsp).wrapping_add(0x308u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247c370(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x3; const P1: u64 = 0x4; const P2: u64 = 0x8; const P3: u64 = 0x14240a730; const P4: u64 = 0x18; const P5: u64 = 0x38; const P6: u64 = 0x58; const P7: u64 = 0x68;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rdi = rcx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx);
    crate::simd_ops::lk_and_u32(rcx, (0xff000002u32) as u32);
    crate::simd_ops::lk_or_u32(rcx, (0x2u32) as u32);
    crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x2u8) as u8);
    crate::simd_ops::wu32(rcx.wrapping_add(P1), (0x0u32) as u32);
    crate::simd_ops::wu8(rcx.wrapping_add(P2), (0x0u8) as u8);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    let y_addr: u64 = rbx.wrapping_add(P4); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    let y_addr: u64 = rdi.wrapping_add(P4); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    let y_addr: u64 = rbx.wrapping_add(P5); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    let y_addr: u64 = rdi.wrapping_add(P5); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi);
    xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(P6))) as u128; ymm0_hi = 0;
    crate::simd_ops::wu128(rdi.wrapping_add(P6), (xmm0) as u128);
    xmm1 = ((crate::simd_ops::ru64(rbx.wrapping_add(P7))) as u64) as u128; ymm1_hi = 0;
    crate::simd_ops::wu64(rdi.wrapping_add(P7), ((xmm1 as u64)) as u64);
    rax = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247c430(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14240a730; const P1: u64 = 0x18; const P2: u64 = 0x38; const P3: u64 = 0x58; const P4: u64 = 0x68;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rdi = rcx;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    let y_addr: u64 = rbx.wrapping_add(P1); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    let y_addr: u64 = rdi.wrapping_add(P1); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    let y_addr: u64 = rbx.wrapping_add(P2); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    let y_addr: u64 = rdi.wrapping_add(P2); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi);
    xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(P3))) as u128; ymm0_hi = 0;
    crate::simd_ops::wu128(rdi.wrapping_add(P3), (xmm0) as u128);
    xmm1 = ((crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64) as u128; ymm1_hi = 0;
    crate::simd_ops::wu64(rdi.wrapping_add(P4), ((xmm1 as u64)) as u64);
    rax = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247c510(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14247c370; const P1: u64 = 0x14240ab10; const P2: u64 = 0x1401e2380; const P3: u64 = 0x14101f7d0; const P4: u64 = 0x1413bbda0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 49>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x60); rsp = ((rsp).wrapping_sub(0x160u64)) as u64; r14 = rdx; rdi = rcx; r15 = 0; fl_l = 0; fl_r = 0; rsi = ((r15 as u32)) as u32 as u64; rbx = rcx.wrapping_add(0x54); r12 = ((r15.wrapping_add(0x10u64)) as u32) as u32 as u64; bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_sub(0x4u64))) as u32 as u64; rax = ((rax).wrapping_add(rcx)) as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; let d = (rcx) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rdx = (((rdx) as i64).wrapping_mul((0x70u64) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_sub(0xcu64)))) as u64; rcx = rbp.wrapping_sub(0x70); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = ((rax).wrapping_sub(1)) as u64; rax = ((rax).wrapping_add(rcx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; let d = (rcx) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } crate::simd_ops::wu32(rbx, ((rdx as u32)) as u32); rax = ((r8.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), ((rax as u32)) as u32); rax = ((rdx as u32)) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x70u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_sub(0xcu64)))) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { crate::simd_ops::lk_sub_u64(rdi.wrapping_add(0xa0u64), (1) as u64); rax = ((crate::simd_ops::ru8(rbp.wrapping_sub(0x50u64))) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shr(1u32)) as u8 as u64)); rdx = rbp.wrapping_sub(0x70); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 6; continue; }
            6 => { rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r15); crate::simd_ops::wu16(rsp.wrapping_add(0x70u64), ((r12 as u16)) as u16); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), r15); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x20u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x28u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x80u64), ((r15 as u32)) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x78u64), rax); rdx = rsp.wrapping_add(0x30); rcx = rbp; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = rax; rcx = rbp.wrapping_sub(0x40); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { r9 = ((crate::simd_ops::ru16(rsp.wrapping_add(0x70u64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rbp.wrapping_sub(0x70); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 3; continue; } bb = 9; continue; }
            9 => { rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; rbx = ((rbx).wrapping_add(0x18u64)) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 1; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu8(r14.wrapping_add(0x70u64), (0x0u8) as u8); bb = 11; continue; }
            11 => { rax = r14; r11 = rsp.wrapping_add(0x160); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            12 => { rcx = r14; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(r14.wrapping_add(0x70u64), (0x1u8) as u8); rcx = rbp.wrapping_sub(0x70); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247c6b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645790; const P1: u64 = 0x1426455a8; const P2: u64 = 0x1413bf790; const P3: u64 = 0x14240a730; const P4: u64 = 0x14240ab10; const P5: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rdx; rbp = rcx; rbx = rcx.wrapping_add(0x38); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rax as u32)) as u32 as u64; r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rdi as u32)) as u32); bb = 2; continue; }
            2 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(0xcu64), (1) as u32); rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0x30u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0xcu64))) as u32 as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x30u64))) as u32 as u64; r14 = (((rax) as i64).wrapping_mul((0x70u64) as i64) as u64) as u64; r14 = ((r14).wrapping_add(rbx)) as u64; r15 = 0; fl_l = 0; fl_r = 0; fl_l = rbx; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 5; continue; }
            5 => { bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0xcu64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x70u64) as i64) as u64) as u64; rax = (((rdx) as i64).wrapping_mul((0x70u64) as i64) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x70u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rsi))) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rcx.wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rcx.wrapping_add(0x40u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(0x60u64), (xmm0) as u128); crate::simd_ops::wu64(rcx, (0x2u64) as u64); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x2u8) as u8); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), r15); crate::simd_ops::wu8(rcx.wrapping_add(0x20u64), (0x0u8) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x24u64), (0x1u64) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), r15); crate::simd_ops::wu64(rcx.wrapping_add(0x38u64), r15); crate::simd_ops::wu64(rcx.wrapping_add(0x40u64), r15); crate::simd_ops::wu64(rcx.wrapping_add(0x48u64), r15); crate::simd_ops::wu64(rcx.wrapping_add(0x50u64), r15); crate::simd_ops::wu64(rcx.wrapping_add(0x58u64), r15); crate::simd_ops::wu64(rcx.wrapping_add(0x60u64), r15); crate::simd_ops::wu64(rcx.wrapping_add(0x68u64), r15); crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x8u64), (1) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rdi = (((rax) as i64).wrapping_mul((0x70u64) as i64) as u64) as u64; rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(rsi))) as u64; rdx = rbx; rcx = rdi.wrapping_sub(0x70); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); let y_addr: u64 = rbx.wrapping_add(0x18u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rdi.wrapping_sub(0x58u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rbx.wrapping_add(0x38u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rdi.wrapping_sub(0x38u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(0x58u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rdi.wrapping_sub(0x18u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x68u64))) as u64) as u128; ymm1_hi = 0; crate::simd_ops::wu64(rdi.wrapping_sub(0x8u64), ((xmm1 as u64)) as u64); rbx = ((rbx).wrapping_add(0x70u64)) as u64; fl_l = rbx; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x30u64))) as u32 as u64; rdi = (((rax) as i64).wrapping_mul((0x70u64) as i64) as u64) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x70u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu32(rbp.wrapping_add(0x30u64), ((r15 as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(0xcu64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = 0xffffffff; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rax.wrapping_add(0x8u64), ((rcx as u32)) as u32); rcx = (tmp_xchg) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247c8b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14247c6b0; const P1: u64 = 0x1413bf990; const P2: u64 = 0x142645790; const P3: u64 = 0x1426455a8; const P4: u64 = 0x14247b250; const P5: u64 = 0x14247c370; const P6: u64 = 0x14247c510; const P7: u64 = 0x142645598; const P8: u64 = 0x14240ab10; const P9: u64 = 0x14241c190;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r15 = rdx; r14 = rcx; rax = 0; fl_l = 0; fl_r = 0; xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x28u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rdx = rsp.wrapping_add(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x70u64) as i64) as u64) as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rcx = ((rcx).wrapping_add(r8)) as u64; rdx = rcx; rdx = ((rdx).wrapping_sub(r8)) as u64; rax = (0x4924924924924925u64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rdx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(5u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), r8); r9 = ((crate::simd_ops::ru8(rsp.wrapping_add(0x78u64))) as u32) as u32 as u64; r8 = rdx; rdx = rsp.wrapping_add(0x70); rcx = rsp.wrapping_add(0x80); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = r14.wrapping_add(0x90); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rax as u32)) as u32 as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rdi as u32)) as u32); bb = 2; continue; }
            2 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(0xcu64), (1) as u32); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rbx = rbp; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rsi = (((rax) as i64).wrapping_mul((0x70u64) as i64) as u64) as u64; rsi = ((rsi).wrapping_add(rbp)) as u64; fl_l = rbp; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x24u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 4; continue; }
            4 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x13; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 5; continue; }
            5 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1f; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 6; continue; }
            6 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x31; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x384; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rax = 0x48; bb = 11; continue; }
            9 => { rax = 0x60; bb = 11; continue; }
            10 => { rax = 0x78; bb = 11; continue; }
            11 => { rdi = ((rax.wrapping_add(r14)) as u64) as u64; rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rax = ((rax).wrapping_sub(1)) as u64; rax = ((rax).wrapping_add(rcx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; let d = (rcx) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rdx as u32)) as u32); rax = ((rdx as u32)) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x70u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; rdx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x10u64), (1) as u32); rbx = ((rbx).wrapping_add(0x70u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 14; continue; }
            14 => { rdx = r15; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(0xcu64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = 0xffffffff; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rax.wrapping_add(0x8u64), ((rcx as u32)) as u32); rcx = (tmp_xchg) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; bb = 16; continue; }
            16 => { fl_l = rbp; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 17; continue; }
            17 => { rbx = rbp; bb = 18; continue; }
            18 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x70u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 18; continue; } bb = 19; continue; }
            19 => { fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 21; continue; }
            21 => { rax = r15; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247ca80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x1c; const P2: u64 = 0x3; const P3: u64 = 0x8; const P4: u64 = 0x4; const P5: u64 = 0x1401f5ea0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x88u64)) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { r8 = ((r8).wrapping_shr(32u32)) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 2; continue; }
            2 => { r8 = ((crate::simd_ops::ru8(rcx.wrapping_add(P2))) as u32) as u32 as u64; r9 = ((rcx.wrapping_add(P3)) as u64) as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { r8 = ((r8.wrapping_mul(8)) as u32) as u32 as u64; bb = 5; continue; }
            4 => { r8 = (((r8 as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x7c00; r8 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rcx)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { r9 = (crate::simd_ops::ru64(r9)) as u64; bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(P4))) as u32 as u64; r8 = rsp.wrapping_add(0x30); xmm0 = (crate::simd_ops::ru128(rdx)) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rdx = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); rcx = rsp.wrapping_add(0x40); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm1) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x88u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            10 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x88u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247cb40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xa8; const P1: u64 = 0x142407c80; const P2: u64 = 0x1413bee30; const P3: u64 = 0x14240ab10;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rdx;
    rbx = ((rcx.wrapping_add(P0)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rax as u32)) as u32 as u64;
    rcx = ((rcx).wrapping_shl(4u32)) as u64;
    rcx = ((rcx).wrapping_add(rbx)) as u64;
    rdx = rdi;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rcx = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247cbb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x30; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x28; const P5: u64 = 0x8; const P6: u64 = 0x20; const P7: u64 = 0x48; const P8: u64 = 0x40; const P9: u64 = 0x50; const P10: u64 = 0x142496730;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rbx = rcx;
    fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_r = rax;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P1))) as u128; crate::simd_ops::wu64(rcx.wrapping_add(P2), rax); crate::simd_ops::wu64(rcx.wrapping_add(P3), rax); crate::simd_ops::wu64(rcx.wrapping_add(P4), rax); rax = rcx; crate::simd_ops::wu64(rcx, (((xmm0 >> 64) as u64)) as u64); crate::simd_ops::wu64(rcx.wrapping_add(P5), ((xmm0 as u64)) as u64); crate::simd_ops::wu16(rcx.wrapping_add(P6), (0x10u16) as u16); rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P7))) as u64;
    r9 = 0x10;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx.wrapping_add(P8)))) as u64;
    rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P9))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247cc20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    r11 = (crate::simd_ops::ru64(rdx)) as u64;
    r10 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r9 = ((r9).wrapping_sub(r11)) as u64;
    r10 = ((r10).wrapping_sub(r11)) as u64;
    fl_l = r9; fl_r = r10;
    r10 = (if (((fl_l as u64)) < ((fl_r as u64))) { r9 } else { r10 }) as u64;
    fl_l = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { fl_l = (crate::simd_ops::ru64(r8)) as u64; fl_r = r10; r10 = (if (((fl_l as u64)) < ((fl_r as u64))) { crate::simd_ops::ru64(r8) } else { r10 }) as u64; }
    rax = ((r10.wrapping_add(r11)) as u64) as u64;
    crate::simd_ops::wu64(rcx, r11);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247cc60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14340cc20; const P1: u64 = 0x1423fdfc0; const P2: u64 = 0x1401feaf0; const P3: u64 = 0x143079330; const P4: u64 = 0x141461fb0; const P5: u64 = 0x1431edc44; const P6: u64 = 0x142498620; const P7: u64 = 0x143231690;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 61>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x14247cc60, funcinfo_rva: 0x37e9a6c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x1e8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rdi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r9) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rsp.wrapping_sub(0xd0u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x1d0u64)) as u64; r15 = (rdx) as u64; rsi = (rcx) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x108u64), (rcx) as u64); fl_l = (crate::simd_ops::ru64(rcx)) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdi = (crate::simd_ops::ru32(r8.wrapping_add(0xcu64))) as u32 as u64; rbx = (crate::simd_ops::ru64(r8.wrapping_add(0x48u64))) as u64; rbx = ((rbx).wrapping_sub(crate::simd_ops::ru64(r8.wrapping_add(0x40u64)))) as u64; rax = ((P0) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (0xcu64) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247ccc0); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r14 = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rbx) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rdi as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r15) as u64); rax = ((P2) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x80u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (0xf14u64) as u64); rax = ((rsp.wrapping_add(0x60u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), (rbx) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), (0x7ffu64) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x68u64), (r14) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), (rbx) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x58u64), (0x7ffu64) as u64); rax = ((P3) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), (rax) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), (r14) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm1) as u128); r9 = ((rbx as u32)) as u32 as u64; r8 = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x70u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247cd6e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x48u64))) as u64; rdx = (rcx) as u64; rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(rbp.wrapping_sub(0x50u64)))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbp.wrapping_sub(0x50u64))) as u64 as u64; rbx = (if (((fl_l as u64)) > ((fl_r as u64))) { rdx } else { rbx }) as u64; fl_l = (crate::simd_ops::ru64(rbp.wrapping_sub(0x60u64))) as u64 as u64; fl_r = (rbx) as u64 as u64; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { crate::simd_ops::ru64(rbp.wrapping_sub(0x60u64)) } else { rbx }) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64; fl_l = (crate::simd_ops::ru64(rbp.wrapping_sub(0x68u64))) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = ((rax).wrapping_add(rbx)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 3; continue; }
            3 => { crate::simd_ops::wu8(rax, (0x0u8) as u8); r8 = (r14) as u64; rdx = ((P5) as u64) as u64; rcx = (crate::simd_ops::ru64(rsi)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247cdac); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rsi)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { r8 = ((P7) as u64) as u64; rdx = ((P5) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247cdc8); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 6; continue; }
            6 => { r11 = ((rsp.wrapping_add(0x1d0u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247ce00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426cdcb0; const P1: u64 = 0x1401df850; const P2: u64 = 0x1401e06c0; const P3: u64 = 0x20; const P4: u64 = 0x14240ab10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x14247ce00, funcinfo_rva: 0x37e9aa8, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x68, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rbx = (rdx) as u64; rdi = (rcx) as u64; rax = ((P0) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x0u64) as u64); rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247ce2f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rbx)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm1) as u128); r8 = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247ce59); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rdi)) as u64; r9 = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; rdx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x43u64))) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = ((rdx.wrapping_mul(8)) as u32) as u32 as u64; bb = 3; continue; }
            2 => { rdx = (((rdx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; rcx = ((rsp.wrapping_add(0x48u64)) as u64) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rcx = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0x48u64)) } else { rcx }) as u64; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x44u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rcx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247cec3); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((r9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (rax) as u64; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247cece); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247cef0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 1; continue; }
            1 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x13; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 2; continue; }
            2 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1f; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x31; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x384; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = 0; fl_l = 0; fl_r = 0; return rax; }
            6 => { rax = 0x1; return rax; }
            7 => { rax = 0x2; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247cf30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14340ccb8; const P1: u64 = 0x10; const P2: u64 = 0x8; const P3: u64 = 0x1426455a8; const P4: u64 = 0x142645598; const P5: u64 = 0x142645590; const P6: u64 = 0x18; const P7: u64 = 0x142442de0; const P8: u64 = 0x1413bed10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rcx; rax = P0; crate::simd_ops::wu64(rcx, rax); rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rax = 0x1; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(rdx.wrapping_add(P2), ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64)); rdi = ((rdx.wrapping_add(P1)) as u64) as u64; rcx = ((rdi.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_add_u32(rdi.wrapping_add(P1), (1) as u32); rcx = ((rdi.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsi.wrapping_add(P6))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rcx = ((rsi.wrapping_add(P1)) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P8, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247cfd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14340ccb8; const P1: u64 = 0x1413bf2f0; const P2: u64 = 0x1413bf440; const P3: u64 = 0x1425371a0; const P4: u64 = 0x142533200; const P5: u64 = 0x14340cc58; const P6: u64 = 0x142645748; const P7: u64 = 0x1423f0c20; const P8: u64 = 0x142442de0; const P9: u64 = 0x1413bf530; const P10: u64 = 0x14340cc98; const P11: u64 = 0x143406c90; const P12: u64 = 0x1423f0780; const P13: u64 = 0x14136eec0; const P14: u64 = 0x14247d253;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x14247cfd0, funcinfo_rva: 0x37e9b04, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x48, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = ((r8 as u32)) as u32 as u64; rbp = (rdx) as u64; rsi = (rcx) as u64; rax = ((P0) as u64) as u64; crate::simd_ops::wu64(rcx, (rax) as u64); rax = (((r8 as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x1u8))) as u8 as u64)); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), ((rax as u8)) as u8); r8 = ((rdx.wrapping_add(0x18u64)) as u64) as u64; rcx = ((rcx).wrapping_add(0x10u64)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247d005); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rsi.wrapping_add(0x18u64), (0x0u64) as u64); xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rsi.wrapping_add(0x20u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); let y_addr: u64 = rsi.wrapping_add(0x40u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); r8 = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; rbx = (((rbx as u32)).wrapping_shr(1u32)) as u32 as u64; r9 = (rbp) as u64; rcx = ((rsp.wrapping_add(0x58u64)) as u64) as u64; fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 1; continue; }
            1 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247d03b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rax, (0x0u64) as u64); rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x18u64), (rcx) as u64); fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247d063); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r10 = ((rbx.wrapping_sub(0x1u64)) as u64) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64 as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = (rbx) as u64; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = ((r10.wrapping_add(0x108u64)) as u64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rbx, (rax) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), (rbx) as u64); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 9; continue; } bb = 5; continue; }
            5 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247d0c0); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 9; continue; }
            6 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 8; continue; }
            7 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { rdx = (rcx) as u64; r9 = (rbx) as u64; rcx = (r10) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247d0d8); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 10; continue; }
            10 => { rax = ((P5) as u64) as u64; crate::simd_ops::wu64(rbx, (rax) as u64); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x108u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rdx = (0xffffffffu32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247d107); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 27; continue; } bb = 12; continue; }
            12 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247d118); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247d121); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 25; continue; }
            13 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247d12c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rax, (0x0u64) as u64); rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x18u64), (rcx) as u64); fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247d154); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r10 = ((rbx.wrapping_sub(0x1u64)) as u64) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64 as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = (rbx) as u64; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = ((r10.wrapping_add(0x108u64)) as u64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 19; continue; } bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 18; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rbx, (rax) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), (rbx) as u64); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 21; continue; } bb = 17; continue; }
            17 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247d1b1); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 21; continue; }
            18 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 20; continue; }
            19 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 20; continue; }
            20 => { rdx = (rcx) as u64; r9 = (rbx) as u64; rcx = (r10) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247d1c9); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 21; continue; }
            21 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 22; continue; }
            22 => { rax = ((P10) as u64) as u64; crate::simd_ops::wu64(rbx, (rax) as u64); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x108u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { rdx = (0xffffffffu32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247d1f4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 26; continue; } bb = 24; continue; }
            24 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247d201); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247d20a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 25; continue; }
            25 => { rax = (rsi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            26 => { rdx = ((P11) as u64) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247d22c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247d234); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 27; continue; }
            27 => { rdx = ((P11) as u64) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247d246); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247d24e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P14, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247d310(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::lk_add_u64(rcx.wrapping_add(P0), (0x8u64) as u64)) as u64; fl_r = fl_l;
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247d360(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x78;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru64(rax)) as u64;
    rax = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247d380(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x140200070;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::lk_add_u64(rcx.wrapping_add(P1), (0x8u64) as u64); rax = rcx; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            2 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14247d39c, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247d3b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x140200070;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rdx)) as u64; rax = (rax & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            3 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14247d3d4, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247d430(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x78; const P3: u64 = 0x140200070;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            2 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14247d44f, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247d480(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141834d64; const P1: u64 = 0x14340cd50; const P2: u64 = 0x8; const P3: u64 = 0x10; const P4: u64 = 0x18;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = 0x20;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = P1;
    crate::simd_ops::wu64(rax, rcx);
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64;
    crate::simd_ops::wu64(rax.wrapping_add(P2), rdx);
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64;
    crate::simd_ops::wu64(rax.wrapping_add(P3), rdx);
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64;
    crate::simd_ops::wu64(rax.wrapping_add(P4), rcx);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247d4e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x18; const P2: u64 = 0x8; const P3: u64 = 0x1401dffd0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdx = rsp.wrapping_add(0x20);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247d5b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = ((rax).wrapping_add(0x200u64)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::wu8(rdx.wrapping_add(P0), (0x1u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247d5d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rdx, rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rax);
    rax = rdx;
    crate::simd_ops::wu8(rdx.wrapping_add(P1), (0x0u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247d5f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = ((rax).wrapping_add(0x200u64)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247d620(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu64(rcx, rdx);
    rax = (crate::simd_ops::ru64(r8)) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247d640(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x1401dffd0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdx = rsp.wrapping_add(0x20);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247d6c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x1c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rbx = rcx; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; rax = (crate::simd_ops::ru64(rax)) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rbx.wrapping_add(P1), rdx); crate::simd_ops::wu8(rbx.wrapping_add(P2), (0x1u8) as u8); r8 = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru64(rdx)) as u64; rdx = rsp.wrapping_add(0x20); xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(P0))) as u128; r9 = ((crate::simd_ops::ru8(r8.wrapping_add(P3))) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = (crate::simd_ops::ru32(r8.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rdx = ((rdx).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P0), rdx); rax = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; rax = (crate::simd_ops::ru64(rax)) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247d780(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(rcx, rax);
    rax = rcx;
    xmm0 = (crate::simd_ops::ru128(r8)) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P0), (xmm0) as u128);
    xmm1 = ((crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64) as u128;
    crate::simd_ops::wu64(rcx.wrapping_add(P2), ((xmm1 as u64)) as u64);
    crate::simd_ops::wu8(rcx.wrapping_add(P3), (0x1u8) as u8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247d7b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18; const P2: u64 = 0x14247d6c0;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    crate::simd_ops::wu64(rcx, rdx);
    rbx = rcx;
    rax = (crate::simd_ops::ru64(r8)) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    crate::simd_ops::wu8(rcx.wrapping_add(P1), (0x0u8) as u8);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247d850(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x9;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu64(rcx, rdx);
    rax = rcx;
    crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x1u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247d870(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x78; const P1: u64 = 0x48; const P2: u64 = 0x60; const P3: u64 = 0x58; const P4: u64 = 0x50; const P5: u64 = 0x10; const P6: u64 = 0x18; const P7: u64 = 0x14240ab10; const P8: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = ((rdx as u32)) as u32 as u64; rbx = rcx; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; r9 = ((crate::simd_ops::ru16(rbx.wrapping_add(P3))) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rcx = ((rbx.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { fl_l = ((rsi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = 0x80; rcx = rbx; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247d8e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x60; const P2: u64 = 0x30; const P3: u64 = 0x48; const P4: u64 = 0x40; const P5: u64 = 0x38; const P6: u64 = 0x10; const P7: u64 = 0x14240ab10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = ((rcx.wrapping_add(P0)) as u64) as u64;
    fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_l = rdx; fl_r = rdx; if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; r9 = ((crate::simd_ops::ru16(rbx.wrapping_add(P4))) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); } rcx = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P7, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247d930(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14340cd38; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x78;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rcx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    crate::simd_ops::wu8(rcx.wrapping_add(P3), ((rax as u8)) as u8);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247d980(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1426455a8; const P2: u64 = 0x78; const P3: u64 = 0x10; const P4: u64 = 0x1426455b8; const P5: u64 = 0x18; const P6: u64 = 0x14101f7d0; const P7: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbp = rdx; rbx = rcx; rdi = ((rcx.wrapping_add(P0)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdi); rcx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x1u8) as u8); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { r9 = 0; fl_l = 0; fl_r = 0; r8 = 0xffffffff; rdx = rdi; rcx = ((rbx.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rdx = ((rbx.wrapping_add(P5)) as u64) as u64; rcx = rbp; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); rax = rbp; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247da10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1426455a8; const P2: u64 = 0x18; const P3: u64 = 0x60; const P4: u64 = 0x14101f7d0; const P5: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rdi = rdx;
    rbx = rcx;
    rsi = ((rcx.wrapping_add(P0)) as u64) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi);
    rcx = rsi;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x1u8) as u8);
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P3), (0x0u8) as u8);
    fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P3))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rdi.wrapping_add(P3), (0x1u8) as u8); }
    rcx = rsi;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247da80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1426455a8; const P2: u64 = 0x18; const P3: u64 = 0x78; const P4: u64 = 0x14240a730; const P5: u64 = 0x30; const P6: u64 = 0x20; const P7: u64 = 0x38; const P8: u64 = 0x1424968a0; const P9: u64 = 0x50; const P10: u64 = 0x68; const P11: u64 = 0x58; const P12: u64 = 0x14101f7d0; const P13: u64 = 0x1426455c0; const P14: u64 = 0x142645598; const P15: u64 = 0x14240ab10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = rdx; rsi = rcx; rbp = ((rcx.wrapping_add(P0)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x1u8) as u8); rdx = rbx; rcx = ((rsi.wrapping_add(P2)) as u64) as u64; fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(P3))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(P5), rax); rdx = ((rbx.wrapping_add(P6)) as u64) as u64; rcx = ((rsi.wrapping_add(P7)) as u64) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P9))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(P10), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P11))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x70u64), rax); bb = 3; continue; }
            2 => { rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rsi.wrapping_add(0x78u64), (0x1u8) as u8); bb = 3; continue; }
            3 => { rcx = rsi.wrapping_add(0x10); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P13), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P14), rax), rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x48u64))) as u64; r9 = ((crate::simd_ops::ru16(rbx.wrapping_add(0x40u64))) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rcx = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P15, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247db50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x508; const P1: u64 = 0x4e8; const P2: u64 = 0x4; const P3: u64 = 0x4f0; const P4: u64 = 0x8; const P5: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rsi = 0; fl_l = 0; fl_r = 0; r15 = r9; crate::simd_ops::wu64(rdx, rsi); r12 = r8; r14 = rdx; rbp = rcx; rax = 0x2; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(P0), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rcx.wrapping_add(P3))) as u32 as u64; rbx = ((rdi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rdi; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rdx = rsp.wrapping_add(0x20); xmm0 = (crate::simd_ops::ru128(r12)) as u128; r9 = ((crate::simd_ops::ru8(r15.wrapping_add(P4))) as u32) as u32 as u64; r8 = (crate::simd_ops::ru32(r15.wrapping_add(P5))) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rsi = ((rsi).wrapping_add(1)) as u64; rdi = ((rdi).wrapping_add(0x8u64)) as u64; fl_l = rdi; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { bb = 6; continue; }
            5 => { crate::simd_ops::wu32(r14, ((rsi as u32)) as u32); crate::simd_ops::lk_add_u64(rbp.wrapping_add(0x2c8u64), (1) as u64); rax = (crate::simd_ops::ru64(r14)) as u64; rdx = rsp.wrapping_add(0x30); xmm0 = (crate::simd_ops::ru128(r12)) as u128; crate::simd_ops::wu64(r15, rax); r8 = r15; rcx = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = r14; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247dc40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4d8; const P1: u64 = 0x8; const P2: u64 = 0x1426455a8; const P3: u64 = 0x10; const P4: u64 = 0x1426455c0; const P5: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rcx = ((rdi.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = 0xffffffff;
    let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64;
    rax = (xa_old) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rcx = ((rdi.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); }
    rcx = ((rdi.wrapping_add(P1)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247dca0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    rdx = rsp.wrapping_add(0x20);
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247dcf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x78;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247dd00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x200; const P1: u64 = 0x78; const P2: u64 = 0x140200070;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x14247dd00, funcinfo_rva: 0x37e9ce4, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x68, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x68u64)) as u64; rcx = ((rcx).wrapping_add(0x2d8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rcx) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x29u64), (0x1u8) as u8); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rax = ((rsp.wrapping_add(0x20u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (rcx) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x58u64), (0x0u8) as u8); r9 = ((rcx.wrapping_add(P0)) as u64) as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x48u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), ((xmm1 as u64)) as u64); rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x40u64))) as u32) as u32 as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 1; continue; }
            1 => { fl_l = (rcx) as u64 as u64; fl_r = (r9) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rax = ((((rax as u8)) as i8 as i64) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64; r8 = (((r8 as u32)).wrapping_add((rcx as u32))) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rcx = ((rcx).wrapping_add(0x8u64)) as u64; rdx = (rcx) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rcx) as u64); rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x40u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { bb = 1; continue; }
            5 => { rax = ((r8 as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x68u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247dd9d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 7; continue; }
            7 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247dda3); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247ddc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14; const P1: u64 = 0x10; const P2: u64 = 0x8;
    let (mut rax, mut r10) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    r10 = (crate::simd_ops::ru64(rdx)) as u64;
    rdx = rsp.wrapping_add(0x20);
    xmm0 = (crate::simd_ops::ru128(rcx)) as u128;
    r9 = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    rcx = r10;
    rax = (crate::simd_ops::ru64(r10)) as u64;
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247de00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4d8; const P1: u64 = 0x8; const P2: u64 = 0x1426455a8; const P3: u64 = 0x10; const P4: u64 = 0x1426455c0; const P5: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rdi = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64;
    rcx = ((rdi.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = 0xffffffff;
    let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64;
    rax = (xa_old) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rcx = ((rdi.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); }
    rcx = ((rdi.wrapping_add(P1)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247de60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x2a8; const P1: u64 = 0x1413c1470;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = rdx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r8);
    rdx = ((rcx.wrapping_add(P0)) as u64) as u64;
    rcx = rbx;
    r8 = rsp.wrapping_add(0x48);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247dff0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp; rsp = ((rsp).wrapping_sub(0x70u64)) as u64; r14 = 0x3e8; r15 = rdx; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 6; continue; } bb = 1; continue; }
            1 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 3; continue; } bb = 2; continue; }
            2 => { r14 = 0x64; bb = 6; continue; }
            3 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x33; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 5; continue; } bb = 4; continue; }
            4 => { r14 = 0x33; bb = 6; continue; }
            5 => { rax = 0x14; r14 = 0x1; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; r14 = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { (rax as u32) } else { (r14 as u32) }) as u32 as u64; bb = 6; continue; }
            6 => { xmm1 = (crate::simd_ops::ru128(rdx)) as u128; rcx = ((rcx).wrapping_add(0x4e8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rsi); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), rcx); crate::simd_ops::wu32(rbp.wrapping_sub(0x40u64), ((rax as u32)) as u32); rdi = rcx.wrapping_add(0x8); crate::simd_ops::wu8(rbp.wrapping_sub(0x3cu64), ((rax as u8)) as u8); rbx = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x28u64), (xmm1) as u128); crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), ((xmm0 as u64)) as u64); rdx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = rbp.wrapping_sub(0x50); r9 = ((crate::simd_ops::ru8(rbp.wrapping_sub(0x14u64))) as u32) as u32 as u64; rsi = rbx; r8 = (crate::simd_ops::ru32(rbp.wrapping_sub(0x18u64))) as u32 as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x50u64), (xmm1) as u128); rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64))) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru32(rdi)) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 10; continue; }
            10 => { xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x28u64))) as u128; bb = 8; continue; }
            11 => { rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x20u64))) as u64; bb = 12; continue; }
            12 => { rdx = (crate::simd_ops::ru32(rdi)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 13; continue; }
            13 => { bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru64(rsi)) as u64; rdx = rbp.wrapping_sub(0x50); xmm0 = (crate::simd_ops::ru128(r15)) as u128; r8 = ((r14 as u32)) as u32 as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x50u64), (xmm0) as u128); rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 22; continue; } bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64))) as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rdx = (crate::simd_ops::ru32(rdi)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 16; continue; }
            16 => { bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = rbp.wrapping_sub(0x50); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x28u64))) as u128; r9 = ((crate::simd_ops::ru8(rbp.wrapping_sub(0x14u64))) as u32) as u32 as u64; rsi = rbx; r8 = (crate::simd_ops::ru32(rbp.wrapping_sub(0x18u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x50u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64))) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { rdx = (crate::simd_ops::ru32(rdi)) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 19; continue; }
            19 => { rdx = (crate::simd_ops::ru32(rdi)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 20; continue; }
            20 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 21; continue; }
            21 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; r11 = rsp.wrapping_add(0x70); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            22 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 21; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247e1b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((rdx as u32)) as u32 as u64;
    rax = ((rax).wrapping_shl(5u32)) as u64;
    rax = ((rax).wrapping_add(0x28u64)) as u64;
    fl_l = ((rax).wrapping_add(rcx)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247e280(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142407c80; const P1: u64 = 0x143a08320; const P2: u64 = 0x14340cd38; const P3: u64 = 0x143c4aa00; const P4: u64 = 0x1426455a8; const P5: u64 = 0x14101f7d0; const P6: u64 = 0x142645598; const P7: u64 = 0x142645868; const P8: u64 = 0x14240ab10; const P9: u64 = 0x1426455b8;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 69>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x100); rsp = ((rsp).wrapping_sub(0x200u64)) as u64; rbx = r9; r12 = r8; r14 = rdx; r15 = rcx; rsi = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x120u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm1 = (crate::simd_ops::ru128(rbx)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0xb0u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0xc0u64), (xmm0) as u128); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0xe8u64), rax); rcx = P2; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rsi); crate::simd_ops::wu8(rbp.wrapping_sub(0x38u64), ((rsi as u8)) as u8); crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), rsi); r13 = r14.wrapping_add(0x2c8); rcx = rsp.wrapping_add(0x50); crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), rcx); crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), r13); crate::simd_ops::wu128(rbp, (xmm0) as u128); crate::simd_ops::wu64(rbp.wrapping_sub(0x8u64), rsi); crate::simd_ops::wu8(rbp.wrapping_sub(0x28u64), (0x1u8) as u8); crate::simd_ops::wu64(rbp.wrapping_sub(0x24u64), (0x3e8u64) as u64); crate::simd_ops::wu128(rbp.wrapping_add(0x10u64), (xmm1) as u128); crate::simd_ops::wu64(rbp.wrapping_add(0x20u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x138u64), ((rsi as u32)) as u32); rax = 0x2; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(r14.wrapping_add(0x508u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x13cu64), ((rax as u32)) as u32); rdi = (crate::simd_ops::ru64(r14.wrapping_add(0x4e8u64))) as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(0x4f0u64))) as u32 as u64; rbx = ((rdi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rdi; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; xmm0 = (crate::simd_ops::ru128(r12)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x30u64), (xmm0) as u128); r9 = ((crate::simd_ops::ru8(rbp.wrapping_sub(0x28u64))) as u32) as u32 as u64; r8 = (crate::simd_ops::ru32(rbp.wrapping_sub(0x20u64))) as u32 as u64; rdx = rbp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rsi = ((rsi).wrapping_add(1)) as u64; rdi = ((rdi).wrapping_add(0x8u64)) as u64; fl_l = rdi; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { bb = 6; continue; }
            5 => { crate::simd_ops::wu32(rbp.wrapping_add(0x138u64), ((rsi as u32)) as u32); crate::simd_ops::lk_add_u64(r13, (1) as u64); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x138u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), rax); rcx = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; xmm0 = (crate::simd_ops::ru128(r12)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); r8 = rbp.wrapping_sub(0x30); rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((rax as u32)) as u32 as u64; rcx = P3; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 8; continue; }
            8 => { rsi = 0; fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { rax = rsp.wrapping_add(0x58); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rcx = rsp.wrapping_add(0x58); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(rsp.wrapping_add(0x40u64), (0x1u8) as u8); crate::simd_ops::wu8(rbp.wrapping_add(0xa0u64), (0x0u8) as u8); fl_l = (crate::simd_ops::ru8(rbp.wrapping_sub(0x38u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdx = rsp.wrapping_add(0x68); rcx = rbp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(rbp.wrapping_add(0xa0u64), (0x1u8) as u8); bb = 11; continue; }
            11 => { rcx = rsp.wrapping_add(0x58); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0xa0u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 16; continue; } bb = 12; continue; }
            12 => { xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); rbx = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru32(r14.wrapping_add(0x10u64))) as u32 as u64; rdi = ((rdi).wrapping_shl(4u32)) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 13; continue; }
            13 => { bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(rax, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            16 => { rdx = rbp.wrapping_add(0x40); rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0xa0u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 20; continue; } bb = 17; continue; }
            17 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x70u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { r9 = ((crate::simd_ops::ru16(rbp.wrapping_add(0x80u64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x88u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x78u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 19; continue; }
            19 => { rcx = rbp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_sub(0x38u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 31; continue; } bb = 21; continue; }
            21 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x68u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { r9 = ((crate::simd_ops::ru16(rbp.wrapping_sub(0x58u64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x50u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x60u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 23; continue; }
            23 => { bb = 30; continue; }
            24 => { rax = rsp.wrapping_add(0x58); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rcx = rsp.wrapping_add(0x58); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(rsp.wrapping_add(0x40u64), (0x1u8) as u8); fl_l = (crate::simd_ops::ru8(rbp.wrapping_sub(0x38u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 27; continue; } bb = 25; continue; }
            25 => { bb = 26; continue; }
            26 => { r9 = 0; fl_l = 0; fl_r = 0; r8 = 0xffffffff; rdx = rsp.wrapping_add(0x58); rcx = rsp.wrapping_add(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rbp.wrapping_sub(0x38u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 26; continue; } bb = 27; continue; }
            27 => { rdx = rsp.wrapping_add(0x68); rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x58); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rbp.wrapping_sub(0x38u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 31; continue; } bb = 28; continue; }
            28 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x68u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 29; continue; }
            29 => { r9 = ((crate::simd_ops::ru16(rbp.wrapping_sub(0x58u64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x50u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x60u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 30; continue; }
            30 => { rcx = rsp.wrapping_add(0x68); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 31; continue; }
            31 => { rax = r15; r11 = rsp.wrapping_add(0x200); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x48u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247e5f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e5120; const P1: u64 = 0x14243b450; const P2: u64 = 0x142407c80; const P3: u64 = 0x14247dea0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x90u64)) as u64; rsi = r9; r13 = r8; rbp = rdx; r15 = rcx; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(0x30u64))) as u64; r12 = ((rcx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rbx = (crate::simd_ops::ru64(r12.wrapping_add(0x2d8u64))) as u64; rdx = r9; rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; crate::simd_ops::wu64(rsp.wrapping_add(0xe0u64), rax); rcx = rbx.wrapping_add(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = rdi; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 3; continue; }
            2 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x38u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = rdi; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi.wrapping_add(0x38u64), (0x0u64) as u64); bb = 5; continue; }
            5 => { crate::simd_ops::wu64(r14.wrapping_add(0x28u64), rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((rax as u32)) as u32 as u64; crate::simd_ops::wu64(r14.wrapping_add(0x30u64), rax); let y_addr: u64 = r14; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp.wrapping_add(0x40u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = r14.wrapping_add(0x20u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp.wrapping_add(0x60u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); xmm0 = ((crate::simd_ops::ru64(r14.wrapping_add(0x40u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), ((xmm0 as u64)) as u64); xmm1 = (crate::simd_ops::ru128(r13)) as u128; ymm1_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm1) as u128); rax = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = (crate::simd_ops::ru64(r12.wrapping_add(0x2d8u64))) as u64; r8 = rsp.wrapping_add(0x30); rdx = rbp; rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x38u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = rsi; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(0x38u64), (0x0u64) as u64); bb = 7; continue; }
            7 => { rax = rbp; r11 = rsp.wrapping_add(0x90); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247e740(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4d8; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x142645868;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rsi = rcx; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rbp = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbp); rbx = (crate::simd_ops::ru64(rsi.wrapping_add(P1))) as u64; rdi = (crate::simd_ops::ru32(rsi.wrapping_add(P2))) as u32 as u64; rdi = ((rdi).wrapping_shl(4u32)) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(rax, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbp); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64; rcx = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) > 0) { bb = 2; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247e800(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x2c8; const P1: u64 = 0x2d0; const P2: u64 = 0x2d8; const P3: u64 = 0x200; const P4: u64 = 0x78; const P5: u64 = 0x140200070;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x14247e800, funcinfo_rva: 0x37e9e68, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x68, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x68u64)) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 1; continue; }
            1 => { rax = ((rcx.wrapping_add(P2)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x29u64), (0x1u8) as u8); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rcx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (rax) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x58u64), (0x0u8) as u8); r9 = ((rax.wrapping_add(P3)) as u64) as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x48u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), ((xmm1 as u64)) as u64); rcx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x40u64))) as u32) as u32 as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 2; continue; }
            2 => { fl_l = (rax) as u64 as u64; fl_r = (r9) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 9; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(P4))) as u32 as u64; r8 = (((r8 as u32)).wrapping_add((rcx as u32))) as u32 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = ((rax).wrapping_add(0x8u64)) as u64; rdx = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rax) as u64); rcx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x40u64))) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 10; continue; } bb = 5; continue; }
            5 => { bb = 2; continue; }
            6 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) > 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x68u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            8 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x68u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            9 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247e8c2); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 10; continue; }
            10 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247e8c8); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247e8e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142407c80; const P1: u64 = 0x2d8; const P2: u64 = 0x142475db0;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = ((rdx as u32)) as u32 as u64;
    rdi = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rax as u32)) as u32 as u64;
    rdx = ((rbx as u32)) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P1))) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247e920(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a082f0; const P1: u64 = 0x1413c11c0; const P2: u64 = 0x4e8; const P3: u64 = 0x1401e3e80; const P4: u64 = 0x4e0; const P5: u64 = 0x142442de0; const P6: u64 = 0x4d8; const P7: u64 = 0x2d8; const P8: u64 = 0x1413c1140; const P9: u64 = 0x38; const P10: u64 = 0x141834ee8; const P11: u64 = 0x2b8; const P12: u64 = 0x1401e3450; const P13: u64 = 0x1401de830; const P14: u64 = 0x1413c1020;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; crate::simd_ops::wu64(P0, (0x0u64) as u64); rcx = ((rcx).wrapping_add(0x4f8u64)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rsi = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P6))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rcx = ((rdi.wrapping_add(P7)) as u64) as u64; r9 = P8; rdx = 0x8; r8 = ((rdx.wrapping_add(P9)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P11)) as u64) as u64; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x2a8); rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P14, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247e9f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e8430; const P1: u64 = 0x1401f9980; const P2: u64 = 0x141834fc0; const P3: u64 = 0x1413c1140; const P4: u64 = 0x140200580; const P5: u64 = 0x1413c12a0; const P6: u64 = 0x1411cddb0; const P7: u64 = 0x143a082f0; const P8: u64 = 0x1413c1340; const P9: u64 = 0x1413bbae0; const P10: u64 = 0x14243b540; const P11: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r12 = rcx; crate::simd_ops::wu64(rcx, rdx); xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rcx.wrapping_add(0x8u64), (xmm0) as u128); r13 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), r13); crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), r13); crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), r13); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), r13); rcx = ((rcx).wrapping_add(0x28u64)) as u64; rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = P1; rdx = ((r13.wrapping_add(0x20u64)) as u32) as u32 as u64; r8 = ((r13.wrapping_add(0x13u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(r12.wrapping_add(0x288u64), r13); crate::simd_ops::wu64(r12.wrapping_add(0x290u64), r13); crate::simd_ops::wu64(r12.wrapping_add(0x298u64), r13); crate::simd_ops::wu64(r12.wrapping_add(0x2a0u64), r13); xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(r12.wrapping_add(0x2a8u64), (xmm0) as u128); crate::simd_ops::wu64(r12.wrapping_add(0x2a8u64), r13); crate::simd_ops::wu64(r12.wrapping_add(0x2b0u64), r13); crate::simd_ops::wu64(r12.wrapping_add(0x2b8u64), r13); crate::simd_ops::wu64(r12.wrapping_add(0x2c0u64), r13); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r12.wrapping_add(0x2c8u64), rax); crate::simd_ops::wu64(r12.wrapping_add(0x2d0u64), rax); rdi = r12.wrapping_add(0x2d8); rax = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = P4; rdx = ((r13.wrapping_add(0x8u64)) as u32) as u32 as u64; r8 = ((r13.wrapping_add(0x40u64)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = r12.wrapping_add(0x4d8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = r12.wrapping_add(0x4e0); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = r12.wrapping_add(0x4e8u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(r12.wrapping_add(0x4e8u64), r13); crate::simd_ops::wu64(r12.wrapping_add(0x4f0u64), r13); crate::simd_ops::wu64(r12.wrapping_add(0x4f8u64), r13); crate::simd_ops::wu64(r12.wrapping_add(0x500u64), r13); crate::simd_ops::wu32(r12.wrapping_add(0x508u64), (0x1u32) as u32); crate::simd_ops::wu64(P7, r12); r14 = ((r13 as u32)) as u32 as u64; rbx = rdi.wrapping_add(0x200); fl_l = rdi; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); r9 = rsp.wrapping_add(0x30); r8 = r12.wrapping_add(0x8); rcx = rsp.wrapping_add(0x88); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rax, r13); rbp = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rdi, rcx); fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rbp.wrapping_add(0x80); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x80u64), r13); crate::simd_ops::wu64(rbp.wrapping_add(0x88u64), r13); crate::simd_ops::wu64(rbp.wrapping_add(0x90u64), r13); rcx = rbp.wrapping_add(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x20u64), r13); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), r13); crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), r13); rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            4 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = rbp.wrapping_add(0x80); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x80u64), r13); crate::simd_ops::wu64(rbp.wrapping_add(0x88u64), r13); crate::simd_ops::wu64(rbp.wrapping_add(0x90u64), r13); rcx = rbp.wrapping_add(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x20u64), r13); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), r13); crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), r13); rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { r14 = ((r14).wrapping_add(1)) as u64; rdi = ((rdi).wrapping_add(0x8u64)) as u64; fl_l = rdi; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 7; continue; }
            7 => { rax = r12; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247ec60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143354c70; const P1: u64 = 0x143a08320; const P2: u64 = 0x14247e280; const P3: u64 = 0x14240ab10; const P4: u64 = 0x1413d2900; const P5: u64 = 0x142409b50; const P6: u64 = 0x1413b6030; const P7: u64 = 0x142448b40;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x1010); rax = 0x1110; rsp = ((rsp).wrapping_sub(rax)) as u64; rbx = r8; rdi = rdx; rsi = rcx; r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r14 as u32)) as u32); xmm0 = (crate::simd_ops::ru128(P0)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm0) as u128); crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), r14); crate::simd_ops::wu64(rbp.wrapping_sub(0x78u64), r14); rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x120u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), rax); let y_addr: u64 = rsp.wrapping_add(0x70u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp.wrapping_add(0x40u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm1 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x70u64))) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), ((xmm1 as u64)) as u64); xmm0 = (crate::simd_ops::ru128(rbx)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); r9 = rsp.wrapping_add(0x40); r8 = rsp.wrapping_add(0x30); rdx = rdi; rcx = rbp.wrapping_sub(0x68); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x20u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r15); rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x28u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbx); rdi = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x30u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); r12 = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x38u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r12); r13 = ((crate::simd_ops::ru16(rax.wrapping_add(0x40u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rax.wrapping_add(0x40u64), (0x10u16) as u16); crate::simd_ops::wu16(rsp.wrapping_add(0x60u64), ((r13 as u16)) as u16); r14 = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x48u64), (0x0u64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r14); rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x38u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { r9 = ((crate::simd_ops::ru16(rbp.wrapping_sub(0x28u64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { rcx = rbp.wrapping_sub(0x68); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::lk_and_u32(rsi.wrapping_add(0x8u64), (0xff000020u32) as u32); crate::simd_ops::lk_or_u32(rsi.wrapping_add(0x8u64), (0x20u32) as u32); crate::simd_ops::wu8(rsi.wrapping_add(0xbu64), (0x20u8) as u8); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsi.wrapping_add(0xcu64), ((rax as u32)) as u32); crate::simd_ops::wu8(rsi.wrapping_add(0x10u64), ((rax as u8)) as u8); crate::simd_ops::wu8(rsi.wrapping_add(0x110u64), ((rax as u8)) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); crate::simd_ops::wu32(rbp, (0x8c000200u32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x4u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x8u64), ((rax as u8)) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbx); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); r9 = rbx; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x30); rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(rbp)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(24u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 5; continue; }
            4 => { rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7c00; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 5; continue; }
            5 => { rcx = ((((rcx as u32)) & (0xffffffu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(3u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsi)) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = rbp.wrapping_add(0x8); bb = 8; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { r8 = (0xffffffffffffffffu64) as u64; bb = 9; continue; }
            9 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 10; continue; }
            10 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 14; continue; }
            11 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (0x0u64) as u64); rax = rbx; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_r = (0xfu64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rbx)) as u64; bb = 13; continue; }
            13 => { crate::simd_ops::wu8(rax, (0x0u8) as u8); bb = 14; continue; }
            14 => { rax = rbx.wrapping_add(0x20); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (0x0u64) as u64); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; fl_r = (0xfu64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rax)) as u64; bb = 16; continue; }
            16 => { crate::simd_ops::wu8(rax, (0x0u8) as u8); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { r9 = r13; rax = (crate::simd_ops::ru64(r14)) as u64; r8 = r12; rdx = rdi; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 18; continue; }
            18 => { rax = rsi; r11 = rsp.wrapping_add(0x1110); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x48u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247eee0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x142646438; const P3: u64 = 0x142646430; const P4: u64 = 0x1401de830;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = rcx; rsi = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = ((rsi as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(rcx.wrapping_mul(8)))) as u64; rdx = 0x2; rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(P1))) as u32 as u64; rcx = ((((rsi as u32)) as i32 as i64) as u64) as u64; rbx = (crate::simd_ops::ru64(r14.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rcx = ((r14.wrapping_add(P0)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(r14)) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = 0x2; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(r14)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(r14, (0xffffffffffffffffu64) as u64); bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247eff0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1413c15f0; const P2: u64 = 0x8; const P3: u64 = 0x14247f540; const P4: u64 = 0x14240ab10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rcx;
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rdx = ((rdi.wrapping_add(P0)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rdx = ((rdi.wrapping_add(P0)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rcx = ((rdi.wrapping_add(P0)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P4, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247f060(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xe; const P2: u64 = 0x142482190; const P3: u64 = 0x1413c2880; const P4: u64 = 0x14340cdc0; const P5: u64 = 0x142482220; const P6: u64 = 0x1413c28f0; const P7: u64 = 0x14247f0bc;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x14247f060, funcinfo_rva: 0x37ea00c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x48, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x48u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = ((rax).wrapping_add(0x10u64)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = (crate::simd_ops::ru16(rax.wrapping_add(P1))) as u16 as u64; fl_r = (0x4u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 4; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rdx, (r9) as u64); crate::simd_ops::wu64(rdx.wrapping_add(P0), (rax) as u64); rax = (rdx) as u64; rsp = ((rsp).wrapping_add(0x48u64)) as u64; return rax; }
            3 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247f090); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247f099); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 4; continue; }
            4 => { r8 = ((P4) as u64) as u64; rdx = (r9) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247f0ae); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247f0b7); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P7, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247f0d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1e;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = (crate::simd_ops::ru16(rax.wrapping_add(P0))) as u16 as u64; fl_r = (0x0u16) as u16 as u64;
    rax = (rax & !0xffu64) | (((if (((fl_l as u16)) != ((fl_r as u16))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247f0f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::lk_add_u64(rcx, (0x20u64) as u64)) as u64; fl_r = fl_l;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247f100(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xe; const P2: u64 = 0x10; const P3: u64 = 0x142482190; const P4: u64 = 0x1413c2880; const P5: u64 = 0x14340cdb8; const P6: u64 = 0x142482220; const P7: u64 = 0x1413c28f0; const P8: u64 = 0x14247f162;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x14247f100, funcinfo_rva: 0x37ea03c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x48, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x48u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = ((rax).wrapping_add(0x10u64)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = (crate::simd_ops::ru16(rax.wrapping_add(P1))) as u16 as u64; fl_r = (0x3u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 4; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rdx, (r9) as u64); crate::simd_ops::wu64(rdx.wrapping_add(P0), (rax) as u64); rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdx.wrapping_add(P2), (rax) as u64); rax = (rdx) as u64; rsp = ((rsp).wrapping_add(0x48u64)) as u64; return rax; }
            3 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247f136); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247f13f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 4; continue; }
            4 => { r8 = ((P5) as u64) as u64; rdx = (r9) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247f154); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247f15d); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P8, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247f170(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14340cda8; const P2: u64 = 0x1413c2cc0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r9 = rcx;
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    if !((((fl_l as u64)) != 0)) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
    r9 = (crate::simd_ops::ru64(r9.wrapping_add(P0))) as u64;
    r8 = P1;
    return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247f1e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14247f340; const P2: u64 = 0x1e; const P3: u64 = 0xe; const P4: u64 = 0x142482340; const P5: u64 = 0x1413415d0; const P6: u64 = 0x1424823d0; const P7: u64 = 0x1401eb1d0; const P8: u64 = 0x14247f26f;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x14247f1e0, funcinfo_rva: 0x37ea074, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x48, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rsi = (r8) as u64; rbx = (rdx) as u64; rdi = (rcx) as u64; rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdx.wrapping_add(P0), (rax) as u64); crate::simd_ops::wu64(rdx, (rax) as u64); rdx = (r8) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247f204); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(P2))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 6; continue; } bb = 2; continue; }
            2 => { fl_l = ((rax).wrapping_add(0x10u64)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = (crate::simd_ops::ru16(rax.wrapping_add(P3))) as u16 as u64; fl_r = (0x4u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rbx.wrapping_add(P0), (rax) as u64); crate::simd_ops::wu64(rbx, (rdx) as u64); rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            5 => { r8 = (rsi) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247f247); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247f250); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 6; continue; }
            6 => { r8 = (rsi) as u64; rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247f261); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247f26a); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P8, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247f280(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14247f340; const P1: u64 = 0x1e; const P2: u64 = 0xe; const P3: u64 = 0x8; const P4: u64 = 0x142482340; const P5: u64 = 0x1413415d0; const P6: u64 = 0x14247f2fa;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x14247f280, funcinfo_rva: 0x37ea0ac, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x48, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbx = (r8) as u64; rsi = (rdx) as u64; rdi = (rcx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247f298); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(P1))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = ((rax).wrapping_add(0x10u64)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = (crate::simd_ops::ru16(rax.wrapping_add(P2))) as u16 as u64; fl_r = (0x4u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 6; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rbx.wrapping_add(P3), (rax) as u64); crate::simd_ops::wu64(rbx, (rdx) as u64); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            6 => { r8 = (rsi) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247f2ec); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247f2f5); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P6, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247f620(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x14247f7c0; const P3: u64 = 0x1424823d0; const P4: u64 = 0x1401eb1d0; const P5: u64 = 0x14247f680;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x14247f620, funcinfo_rva: 0x37ea110, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x48, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = (r8) as u64; rbx = (rdx) as u64; rsi = (rcx) as u64; rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdx, (rax) as u64); crate::simd_ops::wu64(rdx.wrapping_add(P0), (rax) as u64); crate::simd_ops::wu64(rdx.wrapping_add(P1), (rax) as u64); r8 = (rdx) as u64; rdx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247f64b); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            2 => { r8 = (rdi) as u64; rdx = (crate::simd_ops::ru64(rsi)) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247f672); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247f67b); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P5, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247f7c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14247f340; const P1: u64 = 0x1e; const P2: u64 = 0xe; const P3: u64 = 0x8; const P4: u64 = 0x142482340; const P5: u64 = 0x1413415d0; const P6: u64 = 0x14247f83a;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x14247f7c0, funcinfo_rva: 0x37ea17c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x48, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbx = (r8) as u64; rsi = (rdx) as u64; rdi = (rcx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247f7d8); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(P1))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = ((rax).wrapping_add(0x10u64)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = (crate::simd_ops::ru16(rax.wrapping_add(P2))) as u16 as u64; fl_r = (0x3u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 6; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rbx.wrapping_add(P3), (rax) as u64); crate::simd_ops::wu64(rbx, (rdx) as u64); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            6 => { r8 = (rsi) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247f82c); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247f835); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P6, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247fd70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1413c3800; const P1: u64 = 0x14183acbc; const P2: u64 = 0x1413c3460; const P3: u64 = 0x14247f340; const P4: u64 = 0x142482340; const P5: u64 = 0x1413415d0; const P6: u64 = 0x1424823d0; const P7: u64 = 0x1401eb1d0; const P8: u64 = 0x14247fee0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x14247fd70, funcinfo_rva: 0x37ea26c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x88, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rdi = (r8) as u64; rsi = (rdx) as u64; r14 = (rcx) as u64; r12 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r15 = (crate::simd_ops::ru64(rax)) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); rax = (0x4u32) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x3eu64), ((rax as u16)) as u16); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); r13 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu16(rsp.wrapping_add(0x2eu64), ((r13 as u16)) as u16); rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; rax = (r8) as u64; rbx = ((r8 as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru8(r8)) as u8 as u64; fl_r = ((r13 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((rax).wrapping_add(1)) as u64; rbx = (rax) as u64; fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rbx = (((rbx as u32)).wrapping_sub((rdi as u32))) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = (0xdu32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (0x1c05u32) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x2eu64), ((rax as u16)) as u16); rax = (0xdu32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; rax = (rax & !0xffu64) | ((((fl_l as u8).wrapping_sub(fl_r as u8)) as u8 as u64)); crate::simd_ops::wu8(rsp.wrapping_add(0x2du64), ((rax as u8)) as u8); rbp = ((rsp.wrapping_add(0x20u64)) as u64) as u64; bb = 5; continue; }
            4 => { rax = (0xc05u32) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x2eu64), ((rax as u16)) as u16); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rbx as u32)) as u32); rdx = ((rbx.wrapping_add(0x1u64)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247fe10); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbp = (rax) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = (0xffff000000000000u64) as u64; rdx = (((rdx) & (rax))) as u64; fl_l = (((rdx) | (rbp))) as u64 as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rdx) as u64); bb = 5; continue; }
            5 => { r8 = (rbx) as u64; rdx = (rdi) as u64; rcx = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247fe3b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu8(rbx.wrapping_add(rbp), (0x0u8) as u8); r9 = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; r8 = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = (r12) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247fe55); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), (r13) as u64); crate::simd_ops::wu64(rsi, (r13) as u64); rdx = (rdi) as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247fe67); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0x1eu64))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 11; continue; } bb = 7; continue; }
            7 => { fl_l = ((rax).wrapping_add(0x10u64)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(r14)) as u64; fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0xeu64))) as u16 as u64; fl_r = (0x4u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), (rax) as u64); crate::simd_ops::wu64(rsi, (rdx) as u64); rax = (rsi) as u64; r11 = ((rsp.wrapping_add(0x60u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            10 => { r8 = (rdi) as u64; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247feb8); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247fec1); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 11; continue; }
            11 => { r8 = (rdi) as u64; rdx = (crate::simd_ops::ru64(r14)) as u64; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247fed2); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247fedb); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P8, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14247fef0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1413c3800; const P1: u64 = 0x14183acbc; const P2: u64 = 0x1413c3460; const P3: u64 = 0x14247f620;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x14247fef0, funcinfo_rva: 0x37ea2c0, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x58, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rsi = (r8) as u64; rbp = (rdx) as u64; r14 = (rcx) as u64; r12 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r15 = (crate::simd_ops::ru64(rax)) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); rax = (0x3u32) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x3eu64), ((rax as u16)) as u16); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu16(rsp.wrapping_add(0x2eu64), ((rax as u16)) as u16); rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; rax = (r8) as u64; rbx = ((r8 as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru8(r8)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((rax).wrapping_add(1)) as u64; rbx = (rax) as u64; fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rbx = (((rbx as u32)).wrapping_sub((rsi as u32))) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = (0xdu32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (0x1c05u32) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x2eu64), ((rax as u16)) as u16); rax = (0xdu32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; rax = (rax & !0xffu64) | ((((fl_l as u8).wrapping_sub(fl_r as u8)) as u8 as u64)); crate::simd_ops::wu8(rsp.wrapping_add(0x2du64), ((rax as u8)) as u8); rdi = ((rsp.wrapping_add(0x20u64)) as u64) as u64; bb = 5; continue; }
            4 => { rax = (0xc05u32) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x2eu64), ((rax as u16)) as u16); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rbx as u32)) as u32); rdx = ((rbx.wrapping_add(0x1u64)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247ff91); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (rax) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = (0xffff000000000000u64) as u64; rdx = (((rdx) & (rax))) as u64; fl_l = (((rdx) | (rdi))) as u64 as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rdx) as u64); bb = 5; continue; }
            5 => { r8 = (rbx) as u64; rdx = (rsi) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247ffbc); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu8(rbx.wrapping_add(rdi), (0x0u8) as u8); r9 = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; r8 = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = (r12) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247ffd6); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = (rsi) as u64; rdx = (rbp) as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14247ffe4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (rbp) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142480720(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14247f340; const P1: u64 = 0x1e; const P2: u64 = 0xe; const P3: u64 = 0x8; const P4: u64 = 0x142482340; const P5: u64 = 0x1413415d0; const P6: u64 = 0x14248079d;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x142480720, funcinfo_rva: 0x37ea45c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x48, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbx = (r8) as u64; rsi = (rdx) as u64; rdi = (rcx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142480738); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(P1))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = ((rax).wrapping_add(0x10u64)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(P2))) as u16 as u64; fl_r = (0x3u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 6; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rbx.wrapping_add(P3), (rax) as u64); rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rbx, (rax) as u64); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            6 => { r8 = (rsi) as u64; rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14248078f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142480798); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P6, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142480b40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x142481120; const P2: u64 = 0x1424823d0; const P3: u64 = 0x1401eb1d0; const P4: u64 = 0x142480b9c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x142480b40, funcinfo_rva: 0x37ea50c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x48, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = (r8) as u64; rbx = (rdx) as u64; rsi = (rcx) as u64; rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdx.wrapping_add(P0), (rax) as u64); crate::simd_ops::wu64(rdx, (rax) as u64); r8 = (rdx) as u64; rdx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142480b67); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            2 => { r8 = (rdi) as u64; rdx = (crate::simd_ops::ru64(rsi)) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142480b8e); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142480b97); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P4, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142480bb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14247f340; const P1: u64 = 0x1e; const P2: u64 = 0xe; const P3: u64 = 0x8; const P4: u64 = 0x10; const P5: u64 = 0x1424823d0; const P6: u64 = 0x1401eb1d0; const P7: u64 = 0x142482340; const P8: u64 = 0x1413415d0; const P9: u64 = 0x142480c50;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x142480bb0, funcinfo_rva: 0x37ea53c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x48, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rsi = (r8) as u64; rbx = (rdx) as u64; rdi = (rcx) as u64; rdx = (r8) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142480bcb); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(P1))) as u16 as u64; fl_r = ((r8 as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = ((rax).wrapping_add(0x10u64)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = (crate::simd_ops::ru16(rax.wrapping_add(P2))) as u16 as u64; fl_r = (0x3u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 9; continue; } bb = 4; continue; }
            4 => { rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 6; continue; }
            5 => { rdx = (rdx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rax = (r8) as u64; rcx = (r8) as u64; bb = 6; continue; }
            6 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rbx, (rcx) as u64); crate::simd_ops::wu64(rbx.wrapping_add(P3), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(P4), (r8) as u64); rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            8 => { r8 = (rsi) as u64; rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142480c28); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142480c31); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 9; continue; }
            9 => { r8 = (rsi) as u64; rdx = (rcx) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142480c42); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142480c4b); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P9, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142480c60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x142482060; const P2: u64 = 0x1413c38d0; const P3: u64 = 0x142480cb6;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x142480c60, funcinfo_rva: 0x37ea56c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x48, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x48u64)) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru32(r9)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 1; continue; }
            1 => { r9 = (crate::simd_ops::ru64(r9.wrapping_add(P0))) as u64; rax = (0xffffffffffffu64) as u64; r9 = (((r9) & (rax))) as u64; rax = ((((r8 as u32)) as i32 as i64) as u64) as u64; rax = ((rax).wrapping_shl(5u32)) as u64; r9 = ((r9).wrapping_add(rax)) as u64; crate::simd_ops::wu64(rdx, (r9) as u64); rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P0), (rax) as u64); rax = (rdx) as u64; rsp = ((rsp).wrapping_add(0x48u64)) as u64; return rax; }
            2 => { r9 = ((rax as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(rcx)) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142480ca8); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142480cb1); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P3, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142481120(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14247f340; const P1: u64 = 0x1e; const P2: u64 = 0xe; const P3: u64 = 0x8; const P4: u64 = 0x142482340; const P5: u64 = 0x1413415d0; const P6: u64 = 0x14248119d;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x142481120, funcinfo_rva: 0x37ea678, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x48, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbx = (r8) as u64; rsi = (rdx) as u64; rdi = (rcx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142481138); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(P1))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = ((rax).wrapping_add(0x10u64)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(P2))) as u16 as u64; fl_r = (0x4u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 6; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rbx.wrapping_add(P3), (rax) as u64); rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rbx, (rax) as u64); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            6 => { r8 = (rsi) as u64; rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14248118f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142481198); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P6, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142481510(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142481560; const P1: u64 = 0x1424817c0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    xmm0 = 0u128;
    rax = 0;
    fl_l = 0; fl_r = 0;
    r8 = rsp.wrapping_add(0x20);
    rcx = ((rcx).wrapping_add(0x1d8u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128);
    rbx = rdx;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) != 0)) { rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rdx = rbx;
    rcx = rsp.wrapping_add(0x20);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142481560(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142480720; const P1: u64 = 0x143871780; const P2: u64 = 0x142645790; const P3: u64 = 0x143871788; const P4: u64 = 0x1426455a8; const P5: u64 = 0x14387178c; const P6: u64 = 0x1424b06b0; const P7: u64 = 0x1424b14c0; const P8: u64 = 0x1401dec00; const P9: u64 = 0x14183acc8; const P10: u64 = 0x1425371a0; const P11: u64 = 0x142533200; const P12: u64 = 0x142645598; const P13: u64 = 0x1401de830;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x142481560, funcinfo_rva: 0x37ea740, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x58, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r12 = (r8) as u64; rbx = (rdx) as u64; r13 = (rcx) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = (rdx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14248158b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (rax) as u64; r8 = (r12) as u64; rcx = (r13) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142481596); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 28; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 28; continue; }
            4 => { xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); rbp = ((P1) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (rbp) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424815d6); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = ((rax as u32)) as u32 as u64; rcx = (crate::simd_ops::ru32(P3)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424815eb); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(P3, ((rdi as u32)) as u32); bb = 6; continue; }
            6 => { crate::simd_ops::lk_add_u32(P5, (1) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424815fd); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142481608); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rax as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14248160d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rsi = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; rdi = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 7; continue; }
            7 => { rbp = (crate::simd_ops::ru32(rsp.wrapping_add(0x2cu64))) as u32 as u64; bb = 8; continue; }
            8 => { rax = ((rdi as u32)) as u32 as u64; r14 = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = ((rbp as u32)) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = (r14) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 18; continue; } bb = 9; continue; }
            9 => { r14 = ((r14).wrapping_sub(rbx)) as u64; rdi = (r14) as u64; rdi = ((((rdi) as i64).wrapping_shr(3u32) as u64)) as u64; rax = ((rdi.wrapping_add(0x1u64)) as u64) as u64; rcx = ((rcx).wrapping_sub(rbx)) as u64; rcx = ((((rcx) as i64).wrapping_shr(3u32) as u64)) as u64; rbp = (rcx) as u64; rbp = ((rbp).wrapping_shr(1u32)) as u64; rbp = ((rbp).wrapping_add(rcx)) as u64; fl_l = (rbp) as u64 as u64; fl_r = (rax) as u64 as u64; rbp = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rbp }) as u64; rdx = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14248166d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r15 = (rax) as u64; r8 = (r14) as u64; rdx = (rbx) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14248167e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 10; continue; }
            10 => { rbx = ((rbx).wrapping_sub(1)) as u64; rbx = (((rbx) & (0xfffffffffe000000u64))) as u64; rcx = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x100u64))) as u64 as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = (r9) as u64; rax = ((rax).wrapping_sub(rbx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(0x108u64)) as u64; rdx = ((rdx).wrapping_add(rbx)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 15; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 14; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, (rax) as u64); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), (r9) as u64); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 17; continue; } bb = 13; continue; }
            13 => { rcx = (rdx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424816ea); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 17; continue; }
            14 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 16; continue; }
            15 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 16; continue; }
            16 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424816fc); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 17; continue; }
            17 => { rbx = (r15) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), ((rbp as u32)) as u32); bb = 18; continue; }
            18 => { rax = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu64(rbx.wrapping_add(rax.wrapping_mul(8)), (rsi) as u64); rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rdi as u32)) as u32); rsi = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 19; continue; }
            19 => { rbp = ((P1) as u64) as u64; bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::ru32(P5)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; crate::simd_ops::wu32(P5, ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 22; continue; } bb = 21; continue; }
            21 => { rax = (0xffffffffu32) as u32 as u64; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(P3, ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; rcx = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142481744); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P12)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 22; continue; }
            22 => { rax = ((rdi as u32)) as u32 as u64; rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 26; continue; } bb = 23; continue; }
            23 => { bb = 24; continue; }
            24 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142481766); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (rax) as u64; r8 = (r12) as u64; rcx = (r13) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142481771); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 29; continue; } bb = 25; continue; }
            25 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 24; continue; } bb = 26; continue; }
            26 => { rbx = (rbx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 27; continue; }
            27 => { rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14248178a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (((rbx as u8)) as u32) as u32 as u64; bb = 28; continue; }
            28 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            29 => { rbx = (rbx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 27; continue; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424817c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14240ab10; const P1: u64 = 0x1425371a0; const P2: u64 = 0x142533200; const P3: u64 = 0x143c16668; const P4: u64 = 0x1424b0060;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 139>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x440u64)) as u64; rsi = rdx; r15 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x470u64), r15); xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r15); crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), ((r15 as u8)) as u8); rax = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); let y_addr: u64 = rsp.wrapping_add(0x40u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r15); crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((r15 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x74u64), (0xffffffffu32) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x78u64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000010u32))) as u32 as u64; rax = ((((rax as u32)) | (0x10u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x78u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x7bu64), (0x10u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x7cu64), ((r15 as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x80u64), ((r15 as u8)) as u8); rax = (crate::simd_ops::ru64(rdx)) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; rcx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(rbx, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x470u64), rax); rax = (crate::simd_ops::ru64(rsi)) as u64; r9 = 0; fl_l = 0; fl_r = 0; r8 = rsp.wrapping_add(0x20); rdx = r14; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x170u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x78); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x58u64))) as u32 as u64; rdi = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x88u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 3; continue; }
            3 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 4; continue; }
            4 => { rbx = ((rbx).wrapping_sub(1)) as u64; rbx = (((rbx) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(rbx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(0x108u64)) as u64; rdx = ((rdx).wrapping_add(rbx)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { rcx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            8 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 10; continue; }
            9 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            10 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r15); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; rdi = (((rax) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 12; continue; }
            12 => { bb = 13; continue; }
            13 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x108u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 15; continue; }
            15 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 16; continue; }
            16 => { rbx = ((rbx).wrapping_sub(1)) as u64; rbx = (((rbx) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(rbx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(0x108u64)) as u64; rdx = ((rdx).wrapping_add(rbx)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 21; continue; } bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 20; continue; } bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 23; continue; } bb = 19; continue; }
            19 => { rcx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 23; continue; }
            20 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 22; continue; }
            21 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 22; continue; }
            22 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 23; continue; }
            23 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r15); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; rdi = (((rax) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 27; continue; } bb = 24; continue; }
            24 => { bb = 25; continue; }
            25 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x108u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 25; continue; } bb = 26; continue; }
            26 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 27; continue; }
            27 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 35; continue; } bb = 28; continue; }
            28 => { rbx = ((rbx).wrapping_sub(1)) as u64; rbx = (((rbx) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(rbx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(0x108u64)) as u64; rdx = ((rdx).wrapping_add(rbx)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 33; continue; } bb = 29; continue; }
            29 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 32; continue; } bb = 30; continue; }
            30 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 35; continue; } bb = 31; continue; }
            31 => { rcx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 35; continue; }
            32 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 34; continue; }
            33 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 34; continue; }
            34 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 35; continue; }
            35 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r15); r8 = P3; rdx = rsi; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 37; continue; }
            36 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x468u64))) as u64; bb = 37; continue; }
            37 => { r11 = rsp.wrapping_add(0x440); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142481b20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142483b00; const P1: u64 = 0x1424833b0; const P2: u64 = 0x14129d530; const P3: u64 = 0x14240a730; const P4: u64 = 0x14240ab10; const P5: u64 = 0x142442de0; const P6: u64 = 0x1413c35d0; const P7: u64 = 0x141834d30; const P8: u64 = 0x1425371a0; const P9: u64 = 0x142533200; const P10: u64 = 0x142483ab0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 77>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x142481b20, funcinfo_rva: 0x37ea828, handler: 1, shape: 2, frame_reg: 0, establisher_off: 0x268, conts: &[0x142481d08] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x250u64)) as u64; rbx = (r8) as u64; rdi = (rdx) as u64; rsi = (rcx) as u64; r14 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r14 as u32)) as u32); xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142481b55); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rbx)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); r8 = (crate::simd_ops::ru64(rdi)) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142481b71); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x142481b71, (P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x142481d08 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 15; continue; } _ => l.unknown(0x142481b20) } }; rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; crate::simd_ops::wu128(rsi, (xmm0) as u128); rbx = ((rsi.wrapping_add(0x10u64)) as u64) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (r14) as u64); crate::simd_ops::wu64(rbx, (rax) as u64); rcx = ((rbx.wrapping_add(0x8u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x68u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142481b9b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rbx.wrapping_add(0x140u64)) as u64) as u64; crate::simd_ops::lk_and_u32(rcx, (0xff000010u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x10u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x10u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((r14 as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = ((rsp.wrapping_add(0x1a0u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142481bc4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x1c8u64), (rbx) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x1d0u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x1d8u64), (r14) as u64); rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x1e0u64), (rbx) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x1e8u64), (rax) as u64); crate::simd_ops::wu8(rsi.wrapping_add(0x200u64), (0x1u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); rcx = ((rsp.wrapping_add(0x1a0u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142481c09); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x180u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142481c1b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 2; continue; }
            2 => { rcx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142481c26); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 3; continue; }
            3 => { rcx = ((rbx.wrapping_add(0x8088u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142481c40); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rbx.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142481c49); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142481c5a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (0x8028u32) as u32 as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142481c67); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 5; continue; }
            5 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142481c78); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (0x8028u32) as u32 as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142481c85); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 7; continue; }
            7 => { r10 = ((rbx.wrapping_sub(0x1u64)) as u64) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64 as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = (rbx) as u64; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = ((r10.wrapping_add(0x108u64)) as u64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rbx, (rax) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), (rbx) as u64); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 14; continue; } bb = 10; continue; }
            10 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142481ce5); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 14; continue; }
            11 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 13; continue; }
            12 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 13; continue; }
            13 => { rdx = (rcx) as u64; r9 = (rbx) as u64; rcx = (r10) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142481cfd); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 14; continue; }
            14 => { rax = (rsi) as u64; bb = 16; continue; }
            15 => { rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142481d0d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x270u64))) as u64; bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x280u64))) as u64; rsp = ((rsp).wrapping_add(0x250u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142481d40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x142408f50; const P2: u64 = 0x3; const P3: u64 = 0x4; const P4: u64 = 0x1401df850; const P5: u64 = 0x1401e01e0; const P6: u64 = 0x143079440; const P7: u64 = 0x14240a590; const P8: u64 = 0x14240a630;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = rdx; rbx = rcx; rcx = (crate::simd_ops::ru32(rcx)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; rdx = ((rdx).wrapping_add(0x5u64)) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(rbx)) as u32 as u64; bb = 2; continue; }
            2 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u32) as u32 as u64; rdx = ((rbx.wrapping_add(P0)) as u64) as u64; crate::simd_ops::wu32(rbx.wrapping_add(P3), (0x0u32) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 5; continue; }
            4 => { rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7c00; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 5; continue; }
            5 => { rcx = ((((rcx as u32)) & (0xffffffu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 7; continue; }
            7 => { xmm0 = (crate::simd_ops::ru128(rdi)) as u128; crate::simd_ops::wu8(rdx, (0x0u8) as u8); rcx = rsp.wrapping_add(0x30); rdx = rsp.wrapping_add(0x20); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rsp.wrapping_add(0x20); rcx = rbx; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = P6; rax = (0xffffffffffffffffu64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); bb = 8; continue; }
            8 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rax))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 9; continue; }
            9 => { rdx = (rdx & !0xffu64) | (((0x2eu8) as u8 as u64)); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rdx = rsp.wrapping_add(0x20); rcx = rbx; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142481e40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142481560; const P1: u64 = 0x1424817c0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = r8; rbx = rdx; xmm0 = 0u128; rsi = 0; fl_l = 0; fl_r = 0; r8 = rsp.wrapping_add(0x28); rdx = rdi; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); rcx = ((rcx).wrapping_add(0x1d8u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x28u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rdx = rdi; rcx = rsp.wrapping_add(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rbx, rax); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { crate::simd_ops::wu64(rbx, rsi); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142482860(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x28; const P6: u64 = 0x3c; const P7: u64 = 0x44; const P8: u64 = 0x38; const P9: u64 = 0x50; const P10: u64 = 0x54; const P11: u64 = 0x58;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), (0x200u64) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), (rax) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), (rax) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P3), (rax) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P4), (rax) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P5), (rax) as u64);
    crate::simd_ops::wu8(rcx.wrapping_add(P6), ((rax as u8)) as u8);
    crate::simd_ops::wu64(rcx.wrapping_add(P7), (0x1u64) as u64);
    crate::simd_ops::wu64(rcx, (rdx) as u64);
    crate::simd_ops::wu32(rcx.wrapping_add(P8), (0x144u32) as u32);
    crate::simd_ops::lk_and_u32(rcx.wrapping_add(P9), (0xff000020u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rcx.wrapping_add(P9), (0x20u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rcx.wrapping_add(P10), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rcx.wrapping_add(P11), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rcx.wrapping_add(0x53u64), (0x20u8) as u8);
    crate::simd_ops::wu64(rcx.wrapping_add(0x158u64), (rax) as u64);
    rax = (rcx) as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(0x40u64), (0x9u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142482890(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x205061; const P1: u64 = 0x50; const P2: u64 = 0x54; const P3: u64 = 0x58; const P4: u64 = 0x53; const P5: u64 = 0x158; const P6: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    crate::simd_ops::lk_add_u32(rax, ((rax as u32)) as u32);
    crate::simd_ops::lk_add_u8(rcx.wrapping_add(P0), ((rax as u8)) as u8);
    rbx = (rbx & !0xff00u64) | ((((((rbx >> 8) as u8)).wrapping_add(((rbx >> 8) as u8))) as u8 as u64) << 8);
    fl_l = (crate::simd_ops::lk_or_u32(rcx.wrapping_add(P1), (0x20u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rcx.wrapping_add(P3), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rcx.wrapping_add(P4), (0x20u8) as u8);
    crate::simd_ops::wu64(rcx.wrapping_add(P5), rax);
    rax = rcx;
    crate::simd_ops::wu8(rcx.wrapping_add(P6), (0x9u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142482b20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14340ecd8; const P1: u64 = 0x142497fd0; const P2: u64 = 0x142409b50; const P3: u64 = 0x1424b48d0; const P4: u64 = 0x142498890; const P5: u64 = 0x142498af0; const P6: u64 = 0x141833e90; const P7: u64 = 0x1425371a0; const P8: u64 = 0x142533200; const P9: u64 = 0x14240ab10;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 89>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x142482b20, funcinfo_rva: 0x37eaa70, handler: 1, shape: 2, frame_reg: 0, establisher_off: 0x2c8, conts: &[0x142482e31] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rdi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x2c0u64)) as u64; r14 = (rdx) as u64; rdi = (rcx) as u64; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000020u32))) as u32 as u64; rax = ((((rax as u32)) | (0x20u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x33u64), (0x20u8) as u8); r15 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((r15 as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((r15 as u8)) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rcx) as u64); rbx = (0xffffffffffffffffu64) as u64; rax = (rbx) as u64; bb = 1; continue; }
            1 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rax))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 1; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); rax = ((P0) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x170u64), (rax) as u64); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x178u64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000020u32))) as u32 as u64; rax = ((((rax as u32)) | (0x20u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x178u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x17bu64), (0x20u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x17cu64), ((r15 as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x180u64), (0x0u8) as u8); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x170u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142482bdb); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x142482bdb, (P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x142482e31 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 25; continue; } _ => l.unknown(0x142482b20) } }; rax = c_rax; xmm0 = c_x0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x290u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((r15 as u32)) as u32); rdx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x33u64))) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = ((rdx.wrapping_mul(8)) as u32) as u32 as u64; bb = 5; continue; }
            4 => { rdx = (((rdx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rcx = ((((rcx as u32)) & (0xffffffu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(3u32)) as u32 as u64; rax = ((rsp.wrapping_add(0x38u64)) as u64) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rax = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0x38u64)) } else { rax }) as u64; crate::simd_ops::wu8(rax, (0x0u8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x140u64), (r15) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x148u64), (r15) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x150u64), (r15) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x158u64), (0x0u8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x160u64), (r15) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x138u64), ((r15 as u32)) as u32); fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((r15 as u32)) as u32); rdx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x33u64))) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = ((rdx.wrapping_mul(8)) as u32) as u32 as u64; bb = 9; continue; }
            8 => { rdx = (((rdx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rcx = ((((rcx as u32)) & (0xffffffu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(3u32)) as u32 as u64; rax = ((rsp.wrapping_add(0x38u64)) as u64) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rax = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0x38u64)) } else { rax }) as u64; crate::simd_ops::wu8(rax, (0x0u8) as u8); bb = 12; continue; }
            10 => { rbx = ((rbx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(rbx))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 10; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rbx) as u64); r9 = (rbx) as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142482cc5); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x142482cc5, (P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x142482e31 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 25; continue; } _ => l.unknown(0x142482b20) } }; rax = c_rax; xmm0 = c_x0; bb = 12; continue; }
            12 => { rdx = (rsi) as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142482cd2); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x142482cd2, (P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x142482e31 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 25; continue; } _ => l.unknown(0x142482b20) } }; rax = c_rax; xmm0 = c_x0; r8 = (rsi) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x140u64))) as u64; rcx = ((rsp.wrapping_add(0x170u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142482cea); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x142482cea, (P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x142482e31 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 25; continue; } _ => l.unknown(0x142482b20) } }; rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rsp.wrapping_add(0x160u64), (rsi) as u64); rcx = ((rsp.wrapping_add(0x170u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142482cff); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x142482cff, (P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x142482e31 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 25; continue; } _ => l.unknown(0x142482b20) } }; rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x140u64))) as u64; rdx = ((r14.wrapping_add(rcx)) as u64) as u64; r8 = (r8 & !0xffu64) | (((0xau8) as u8 as u64)); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142482d14); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x142482d14, (P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x142482e31 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 25; continue; } _ => l.unknown(0x142482b20) } }; rax = c_rax; xmm0 = c_x0; rbx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x140u64))) as u64; fl_l = (r10) as u64 as u64; fl_r = (r10) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x158u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 21; continue; } bb = 14; continue; }
            14 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64 as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x140u64))) as u64; rax = (r9) as u64; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((r10.wrapping_add(0x108u64)) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 19; continue; } bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 18; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, (rax) as u64); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), (r9) as u64); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 21; continue; } bb = 17; continue; }
            17 => { rcx = (rdx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142482d9c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x142482d9c, (P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x142482e31 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 25; continue; } _ => l.unknown(0x142482b20) } }; rax = c_rax; xmm0 = c_x0; bb = 21; continue; }
            18 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 20; continue; }
            19 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 20; continue; }
            20 => { rcx = (r10) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142482dae); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x142482dae, (P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x142482e31 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 25; continue; } _ => l.unknown(0x142482b20) } }; rax = c_rax; xmm0 = c_x0; bb = 21; continue; }
            21 => { crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((r15 as u32)) as u32); rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x33u64))) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 24; continue; }
            23 => { rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 24; continue; }
            24 => { rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rdx = ((((rdx as u32)) & (0xffffffu32))) as u32 as u64; rdx = (((rdx as u32)).wrapping_shl(3u32)) as u32 as u64; rcx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rcx = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0x38u64)) } else { rcx }) as u64; crate::simd_ops::wu8(rcx, (0x0u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x138u64), ((r15 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x140u64), (r15) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x148u64), (r15) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x150u64), (r15) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x158u64), (0x0u8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x160u64), (r15) as u64); rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142482e27); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x142482e27, (P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x142482e31 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 25; continue; } _ => l.unknown(0x142482b20) } }; rax = c_rax; xmm0 = c_x0; rax = ((rbx as u32)) as u32 as u64; bb = 26; continue; }
            25 => { rax = (0xffffffffffffffffu64) as u64; bb = 26; continue; }
            26 => { r11 = ((rsp.wrapping_add(0x2c0u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; r14 = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142482e60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xe; const P1: u64 = 0x14340e058; const P2: u64 = 0x142481ec0; const P3: u64 = 0x1413c65c0; const P4: u64 = 0x142482e9f;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x142482e60, funcinfo_rva: 0x37eaad0, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x48, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x48u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((crate::simd_ops::ru16(rax.wrapping_add(P0))) as u32) as u32 as u64; fl_l = ((rdx as u16)) as u16 as u64; fl_r = (0x3u16) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = ((rdx as u16)) as u16 as u64; fl_r = (0x4u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rsp = ((rsp).wrapping_add(0x48u64)) as u64; return rax; }
            3 => { r9 = (0xffffffffu32) as u32 as u64; r8 = ((P1) as u64) as u64; rdx = (rcx) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142482e91); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142482e9a); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P4, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142482ec0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140000000; const P1: u64 = 0x14340e1a8; const P2: u64 = 0x14340e0f0; const P3: u64 = 0x14340e110; const P4: u64 = 0x14340e128; const P5: u64 = 0x14340e140; const P6: u64 = 0x14340e228; const P7: u64 = 0x14340e250; const P8: u64 = 0x14340e278; const P9: u64 = 0x14340e2a0; const P10: u64 = 0x14340e1c0; const P11: u64 = 0x14340e1d8; const P12: u64 = 0x14340e1f0; const P13: u64 = 0x14340e210; const P14: u64 = 0x14340e078; const P15: u64 = 0x14340e098; const P16: u64 = 0x14340e0b8; const P17: u64 = 0x14340e0d8; const P18: u64 = 0x1426cdcb0;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x50u64))) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub(1)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x10u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 19; continue; } bb = 1; continue; }
            1 => { rax = ((((rcx as u32)) as i32 as i64) as u64) as u64; rdx = ((P0) as u64) as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x2482f78u64))) as u32 as u64; fl_l = ((rcx).wrapping_add(rdx)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; j_tgt = (rcx) as u64; bb = 0xffff_fffe; continue; }
            2 => { rax = ((P1) as u64) as u64; return rax; }
            3 => { rax = ((P2) as u64) as u64; return rax; }
            4 => { rax = ((P3) as u64) as u64; return rax; }
            5 => { rax = ((P4) as u64) as u64; return rax; }
            6 => { rax = ((P5) as u64) as u64; return rax; }
            7 => { rax = ((P6) as u64) as u64; return rax; }
            8 => { rax = ((P7) as u64) as u64; return rax; }
            9 => { rax = ((P8) as u64) as u64; return rax; }
            10 => { rax = ((P9) as u64) as u64; return rax; }
            11 => { rax = ((P10) as u64) as u64; return rax; }
            12 => { rax = ((P11) as u64) as u64; return rax; }
            13 => { rax = ((P12) as u64) as u64; return rax; }
            14 => { rax = ((P13) as u64) as u64; return rax; }
            15 => { rax = ((P14) as u64) as u64; return rax; }
            16 => { rax = ((P15) as u64) as u64; return rax; }
            17 => { rax = ((P16) as u64) as u64; return rax; }
            18 => { rax = ((P17) as u64) as u64; return rax; }
            19 => { rax = ((P18) as u64) as u64; return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x142482ec0u64) { 0x11 => { bb = 1; continue; } 0x27 => { bb = 2; continue; } 0x2f => { bb = 3; continue; } 0x37 => { bb = 4; continue; } 0x3f => { bb = 5; continue; } 0x47 => { bb = 6; continue; } 0x4f => { bb = 7; continue; } 0x57 => { bb = 8; continue; } 0x5f => { bb = 9; continue; } 0x67 => { bb = 10; continue; } 0x6f => { bb = 11; continue; } 0x77 => { bb = 12; continue; } 0x7f => { bb = 13; continue; } 0x87 => { bb = 14; continue; } 0x8f => { bb = 15; continue; } 0x97 => { bb = 16; continue; } 0x9f => { bb = 17; continue; } 0xa7 => { bb = 18; continue; } 0xaf => { bb = 19; continue; } _ => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(j_tgt, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); } } }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142482fd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140000000; const P1: u64 = 0x14340e1a8; const P2: u64 = 0x14340e0f0; const P3: u64 = 0x14340e110; const P4: u64 = 0x14340e128; const P5: u64 = 0x14340e140; const P6: u64 = 0x14340e228; const P7: u64 = 0x14340e250; const P8: u64 = 0x14340e278; const P9: u64 = 0x14340e2a0; const P10: u64 = 0x14340e1c0; const P11: u64 = 0x14340e1d8; const P12: u64 = 0x14340e1f0; const P13: u64 = 0x14340e210; const P14: u64 = 0x14340e078; const P15: u64 = 0x14340e098; const P16: u64 = 0x14340e0b8; const P17: u64 = 0x14340e0d8; const P18: u64 = 0x1426cdcb0;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (((rcx as u32)).wrapping_sub(1)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x10u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 19; continue; } bb = 1; continue; }
            1 => { rax = ((((rcx as u32)) as i32 as i64) as u64) as u64; rdx = ((P0) as u64) as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x2483084u64))) as u32 as u64; fl_l = ((rcx).wrapping_add(rdx)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; j_tgt = (rcx) as u64; bb = 0xffff_fffe; continue; }
            2 => { rax = ((P1) as u64) as u64; return rax; }
            3 => { rax = ((P2) as u64) as u64; return rax; }
            4 => { rax = ((P3) as u64) as u64; return rax; }
            5 => { rax = ((P4) as u64) as u64; return rax; }
            6 => { rax = ((P5) as u64) as u64; return rax; }
            7 => { rax = ((P6) as u64) as u64; return rax; }
            8 => { rax = ((P7) as u64) as u64; return rax; }
            9 => { rax = ((P8) as u64) as u64; return rax; }
            10 => { rax = ((P9) as u64) as u64; return rax; }
            11 => { rax = ((P10) as u64) as u64; return rax; }
            12 => { rax = ((P11) as u64) as u64; return rax; }
            13 => { rax = ((P12) as u64) as u64; return rax; }
            14 => { rax = ((P13) as u64) as u64; return rax; }
            15 => { rax = ((P14) as u64) as u64; return rax; }
            16 => { rax = ((P15) as u64) as u64; return rax; }
            17 => { rax = ((P16) as u64) as u64; return rax; }
            18 => { rax = ((P17) as u64) as u64; return rax; }
            19 => { rax = ((P18) as u64) as u64; return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x142482fd0u64) { 0xb => { bb = 1; continue; } 0x21 => { bb = 2; continue; } 0x29 => { bb = 3; continue; } 0x31 => { bb = 4; continue; } 0x39 => { bb = 5; continue; } 0x41 => { bb = 6; continue; } 0x49 => { bb = 7; continue; } 0x51 => { bb = 8; continue; } 0x59 => { bb = 9; continue; } 0x61 => { bb = 10; continue; } 0x69 => { bb = 11; continue; } 0x71 => { bb = 12; continue; } 0x79 => { bb = 13; continue; } 0x81 => { bb = 14; continue; } 0x89 => { bb = 15; continue; } 0x91 => { bb = 16; continue; } 0x99 => { bb = 17; continue; } 0xa1 => { bb = 18; continue; } 0xa9 => { bb = 19; continue; } _ => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(j_tgt, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); } } }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424830d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14240ab10; const P1: u64 = 0x10; const P2: u64 = 0x1413c35d0; const P3: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = ((rcx).wrapping_add(0x40u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    fl_l = rbx; fl_r = rbx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = 0x8028; rcx = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142483120(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14240ab10; const P1: u64 = 0x10; const P2: u64 = 0x1413c35d0; const P3: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = ((rcx).wrapping_add(0x50u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    fl_l = rbx; fl_r = rbx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = 0x8028; rcx = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142483170(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426cdcb0; const P1: u64 = 0x1402036f0; const P2: u64 = 0x1413c6f60; const P3: u64 = 0x14240ab10; const P4: u64 = 0x1413c35d0; const P5: u64 = 0x141834d30; const P6: u64 = 0x142482900; const P7: u64 = 0x1424828c0; const P8: u64 = 0x142481f60; const P9: u64 = 0x1413c67a0; const P10: u64 = 0x1424832a1;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 53>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x142483170, funcinfo_rva: 0x37eab18, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x1a8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rbp = ((rsp.wrapping_sub(0xa0u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x1a0u64)) as u64; rbx = (rdx) as u64; rsi = (rcx) as u64; rdi = (crate::simd_ops::ru64(rcx)) as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((P0) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424831a5); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbp.wrapping_add(0xb0u64), (rbx) as u64); rax = ((rbp.wrapping_add(0xb0u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rcx) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); xmm1 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm1) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (0x200u64) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), (0x144u32) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x5cu64), ((rcx as u8)) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x64u64), (0x1u64) as u64); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x70u64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000020u32))) as u32 as u64; rax = ((((rax as u32)) | (0x20u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x73u64), (0x20u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x74u64), ((rcx as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x78u64), ((rcx as u8)) as u8); crate::simd_ops::wu64(rbp.wrapping_add(0x78u64), (rcx) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x60u64), (0x9u8) as u8); rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142483224); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rcx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142483232); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142483244); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (0x8028u32) as u32 as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142483251); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 3; continue; }
            3 => { r11 = ((rsp.wrapping_add(0x1a0u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = (r11) as u64; rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            4 => { rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142483274); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = (rax) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142483281); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = (rax) as u64; rdx = (rsi) as u64; rcx = ((rbp.wrapping_add(0x80u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142483293); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14248329c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P10, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424832b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141833e90;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    r8 = (r8 & !0xffu64) | (((0xau8) as u8 as u64));
    rdx = ((rdx).wrapping_add(rcx)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = (((rax as u32)).wrapping_add(1)) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424832d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14340e2dc; const P1: u64 = 0x1432127b4; const P2: u64 = 0x143128210; const P3: u64 = 0x143128208; const P4: u64 = 0x14340cdc0; const P5: u64 = 0x14340e2e4; const P6: u64 = 0x14340e2ec; const P7: u64 = 0x14340e2f4; const P8: u64 = 0x14340e2bc; const P9: u64 = 0x14340e2c4; const P10: u64 = 0x14340e2cc; const P11: u64 = 0x14340e2d4; const P12: u64 = 0x1432316c0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rdx = ((crate::simd_ops::ru16(rcx.wrapping_add(0xeu64))) as u32) as u32 as u64; fl_l = ((rdx as u16)) as u16 as u64; fl_r = ((rdx as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = P0; return rax; }
            2 => { rax = (((rdx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shr(3u32)) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = P1; return rax; }
            4 => { fl_l = ((rdx as u16)) as u16 as u64; fl_r = (0x9u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = P2; return rax; }
            6 => { fl_l = ((rdx as u16)) as u16 as u64; fl_r = (0xau16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = P3; return rax; }
            8 => { fl_l = ((rdx as u16)) as u16 as u64; fl_r = (0x4u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = P4; return rax; }
            10 => { rax = (((rdx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shr(5u32)) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = P5; return rax; }
            12 => { rax = (((rdx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shr(6u32)) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = P6; return rax; }
            14 => { rax = (((rdx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shr(7u32)) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rax = P7; return rax; }
            16 => { rax = (((rdx as u16)) as u32) as u32 as u64; rax = (rax & !0xffffu64) | (((((rax as u16)).wrapping_shr(8u32)) as u16 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { rax = P8; return rax; }
            18 => { rax = (((rdx as u16)) as u32) as u32 as u64; rax = (rax & !0xffffu64) | (((((rax as u16)).wrapping_shr(9u32)) as u16 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { rax = P9; return rax; }
            20 => { rax = (((rdx as u16)) as u32) as u32 as u64; rax = (rax & !0xffffu64) | (((((rax as u16)).wrapping_shr(10u32)) as u16 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { rax = P10; return rax; }
            22 => { rdx = (rdx & !0xffu64) | (((((rdx as u8)).wrapping_shr(4u32)) as u8 as u64)); rcx = P11; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; rax = P12; rax = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { rcx } else { rax }) as u64; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424833b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1413c6630; const P1: u64 = 0x50; const P2: u64 = 0xe; const P3: u64 = 0x14340e058; const P4: u64 = 0x142481ec0; const P5: u64 = 0x1413c65c0; const P6: u64 = 0x14340e158; const P7: u64 = 0x14340e178; const P8: u64 = 0x142483471;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1424833b0, funcinfo_rva: 0x37eab50, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x68, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rbx = (rcx) as u64; fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rdx) as u64); rax = ((rdx.wrapping_add(r8)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rdx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (0x0u64) as u64); rdx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424833e2); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((crate::simd_ops::ru16(rax.wrapping_add(P2))) as u32) as u32 as u64; fl_l = ((rcx as u16)) as u16 as u64; fl_r = (0x3u16) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = ((rcx as u16)) as u16 as u64; fl_r = (0x4u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rsp = ((rsp).wrapping_add(0x60u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            5 => { r9 = (0xffffffffu32) as u32 as u64; r8 = ((P3) as u64) as u64; rdx = (rbx) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14248341b); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142483424); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 6; continue; }
            6 => { r9 = (0xffffffffu32) as u32 as u64; r8 = ((P6) as u64) as u64; rdx = (rbx) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14248343f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142483448); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 7; continue; }
            7 => { r9 = (0xffffffffu32) as u32 as u64; r8 = ((P7) as u64) as u64; rdx = (rbx) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142483463); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14248346c); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P8, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142483480(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14129d530; const P1: u64 = 0x140; const P2: u64 = 0x3; const P3: u64 = 0x4; const P4: u64 = 0x8; const P5: u64 = 0x14240a730; const P6: u64 = 0x1c8; const P7: u64 = 0x1d0; const P8: u64 = 0x1d8; const P9: u64 = 0x1e0; const P10: u64 = 0x1e8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdi = rcx; rax = (crate::simd_ops::ru64(rdx)) as u64; rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdx, rsi); crate::simd_ops::wu64(rcx, rax); rdx = ((rdx).wrapping_add(0x8u64)) as u64; rcx = ((rcx).wrapping_add(0x8u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P1)) as u64) as u64; crate::simd_ops::lk_and_u32(rcx, (0xff000010u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x10u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(P2), (0x10u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(P3), ((rsi as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(P4), ((rsi as u8)) as u8); rdx = ((rbx.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P6), rdi); crate::simd_ops::wu64(rdi.wrapping_add(P7), rax); crate::simd_ops::wu64(rdi.wrapping_add(P8), rsi); rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P9), rdi); crate::simd_ops::wu64(rdi.wrapping_add(P10), rax); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142483530(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142409b50; const P1: u64 = 0x1413c6f60; const P2: u64 = 0x14240ab10; const P3: u64 = 0x1413c35d0; const P4: u64 = 0x141834d30; const P5: u64 = 0x14340ecd8; const P6: u64 = 0x142497fd0; const P7: u64 = 0x142498740; const P8: u64 = 0x142498af0; const P9: u64 = 0x1426467f0; const P10: u64 = 0x142483700; const P11: u64 = 0x142482900; const P12: u64 = 0x1424828c0; const P13: u64 = 0x142481f60; const P14: u64 = 0x1413c67a0; const P15: u64 = 0x14248374f;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 99>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x142483530, funcinfo_rva: 0x37eabb0, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x318, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rbp = ((rsp.wrapping_sub(0x210u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x310u64)) as u64; rdi = (rdx) as u64; rbx = (rcx) as u64; rcx = ((rcx).wrapping_add(0x140u64)) as u64; xmm0 = (crate::simd_ops::ru128(rdx)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); r9 = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142483572); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (0xfu64) as u64); rsi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rsi) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((rsi as u8)) as u8); rax = ((rsp.wrapping_add(0x20u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x220u64), (rax) as u64); rax = ((rbp.wrapping_add(0x220u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (rsi) as u64); crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm0) as u128); xmm1 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x80u64), (xmm1) as u128); crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), (0x200u64) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x68u64), (0x144u32) as u32); crate::simd_ops::wu8(rbp.wrapping_sub(0x64u64), ((rsi as u8)) as u8); crate::simd_ops::wu64(rbp.wrapping_sub(0x5cu64), (0x1u64) as u64); rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x50u64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000020u32))) as u32 as u64; rax = ((((rax as u32)) | (0x20u32))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x50u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_sub(0x4du64), (0x20u8) as u8); crate::simd_ops::wu32(rbp.wrapping_sub(0x4cu64), ((rsi as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_sub(0x48u64), ((rsi as u8)) as u8); crate::simd_ops::wu64(rbp.wrapping_add(0xb8u64), (rsi) as u64); crate::simd_ops::wu8(rbp.wrapping_sub(0x60u64), (0x9u8) as u8); rdx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142483609); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { rcx = ((rbp.wrapping_sub(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14248361a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14248362c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (0x8028u32) as u32 as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142483639); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 3; continue; }
            3 => { rax = ((P5) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0xc0u64), (rax) as u64); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0xc8u64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000020u32))) as u32 as u64; rax = ((((rax as u32)) | (0x20u32))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0xc8u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0xcbu64), (0x20u8) as u8); crate::simd_ops::wu32(rbp.wrapping_add(0xccu64), ((rsi as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0xd0u64), (0x0u8) as u8); xmm0 = (crate::simd_ops::ru128(rdi)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); r8 = (0x1u32) as u32 as u64; rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0xc0u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142483690); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64 as u64; fl_r = (0xfu64) as u64 as u64; rdx = (if (((fl_l as u64)) > ((fl_r as u64))) { crate::simd_ops::ru64(rsp.wrapping_add(0x20u64)) } else { rdx }) as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = ((rbp.wrapping_add(0xc0u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424836b3); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rbp.wrapping_add(0xc0u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424836c0); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0xfu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 8; continue; } bb = 4; continue; }
            4 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = (rcx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424836fa); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P9)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P10, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142483701); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 8; continue; }
            8 => { r11 = ((rsp.wrapping_add(0x310u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = (r11) as u64; rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            9 => { rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142483724); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = (rax) as u64; rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142483731); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = (rax) as u64; rdx = (rbx) as u64; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142483741); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14248374a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P14) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P15, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142483760(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426cdcb0; const P1: u64 = 0x142409b50; const P2: u64 = 0x1413c6a80; const P3: u64 = 0x14240ab10; const P4: u64 = 0x1413c35d0; const P5: u64 = 0x141834d30; const P6: u64 = 0x142482780; const P7: u64 = 0x142482740; const P8: u64 = 0x142481f60; const P9: u64 = 0x1413c67a0; const P10: u64 = 0x14248389d;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 55>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x142483760, funcinfo_rva: 0x37eac0c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x1b8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rbp = ((rsp.wrapping_sub(0xa0u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x1a0u64)) as u64; rbx = (rdx) as u64; rsi = (rcx) as u64; rdi = (crate::simd_ops::ru64(rcx)) as u64; rax = ((P0) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); r14 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (r14) as u64); r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424837a8); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbp.wrapping_add(0xc0u64), (rbx) as u64); rax = ((rbp.wrapping_add(0xc0u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (r14) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); xmm1 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm1) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (0x200u64) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x78u64), (0x144u32) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x7cu64), ((r14 as u8)) as u8); crate::simd_ops::wu64(rbp.wrapping_sub(0x7cu64), (0x1u64) as u64); rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x70u64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000020u32))) as u32 as u64; rax = ((((rax as u32)) | (0x20u32))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x70u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_sub(0x6du64), (0x20u8) as u8); crate::simd_ops::wu32(rbp.wrapping_sub(0x6cu64), ((r14 as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_sub(0x68u64), ((r14 as u8)) as u8); crate::simd_ops::wu64(rbp.wrapping_add(0x98u64), (r14) as u64); crate::simd_ops::wu8(rbp.wrapping_sub(0x80u64), (0x9u8) as u8); rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142483824); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rcx = ((rbp.wrapping_sub(0x70u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142483831); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142483843); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (0x8028u32) as u32 as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142483850); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 3; continue; }
            3 => { r11 = ((rsp.wrapping_add(0x1a0u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = (r11) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            4 => { rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142483872); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = (rax) as u64; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14248387f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = (rax) as u64; rdx = (rsi) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14248388f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142483898); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P10, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424838b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142409b50; const P1: u64 = 0x1413c6470; const P2: u64 = 0x14340e058; const P3: u64 = 0x142481ec0; const P4: u64 = 0x1413c65c0; const P5: u64 = 0x142482eb0; const P6: u64 = 0x1401df680; const P7: u64 = 0x142482b20; const P8: u64 = 0x142482ec0; const P9: u64 = 0x1424839e5;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1424838b0, funcinfo_rva: 0x37eac44, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x48, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rsi = (rdx) as u64; rdi = (rcx) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = (rdx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424838cb); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = ((rdi.wrapping_add(0x140u64)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), ((rax as u32)) as u32); rcx = ((rbx.wrapping_add(0x8u64)) as u64) as u64; rdx = ((crate::simd_ops::ru8(rbx.wrapping_add(0x3u64))) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = ((rdx.wrapping_mul(8)) as u32) as u32 as u64; bb = 4; continue; }
            3 => { rdx = (((rdx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; bb = 6; continue; }
            6 => { crate::simd_ops::wu8(rcx, (0x0u8) as u8); bb = 10; continue; }
            7 => { r9 = (0xffffffffffffffffu64) as u64; bb = 8; continue; }
            8 => { r9 = ((r9).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r9))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (r9) as u64); r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14248393f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (rsi) as u64); rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = (crate::simd_ops::ru64(rdi)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142483951); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x50u64))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((crate::simd_ops::ru16(rax.wrapping_add(0xeu64))) as u32) as u32 as u64; fl_l = ((rcx as u16)) as u16 as u64; fl_r = (0x3u16) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 13; continue; } bb = 12; continue; }
            12 => { fl_l = ((rcx as u16)) as u16 as u64; fl_r = (0x4u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            14 => { r9 = (0xffffffffu32) as u32 as u64; r8 = ((P2) as u64) as u64; rdx = (rdi) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142483994); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14248399d); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 15; continue; }
            15 => { rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424839a6); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = (rax) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424839b1); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (r8) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424839bc); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r9 = ((rax as u32)) as u32 as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424839c7); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = (rax) as u64; rdx = (rdi) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424839d7); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424839e0); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P9, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142483a00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x142409b50; const P2: u64 = 0x1424b5b70; const P3: u64 = 0x1412aaf30; const P4: u64 = 0x142442de0; const P5: u64 = 0x14240ab10; const P6: u64 = 0x138; const P7: u64 = 0x120; const P8: u64 = 0x1424833b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 47>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x142483a00, funcinfo_rva: 0x37eac78, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x178, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x170u64)) as u64; rbx = (rdx) as u64; rdi = (rcx) as u64; rcx = ((rcx).wrapping_add(0x140u64)) as u64; xmm0 = (crate::simd_ops::ru128(rdx)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); r9 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142483a30); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rbx)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142483a49); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rdi.wrapping_add(P0)) as u64) as u64; rdx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142483a56); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x148u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142483a69); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 2; continue; }
            2 => { rcx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142483a74); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = (crate::simd_ops::ru64(rdi.wrapping_add(P6))) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P7))) as u64; rcx = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x180u64))) as u64; rsp = ((rsp).wrapping_add(0x170u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); drop(_eh); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142483ab0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14240ab10; const P1: u64 = 0x120; const P2: u64 = 0x142442de0; const P3: u64 = 0x10; const P4: u64 = 0x1413c6280;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = ((rcx).wrapping_add(0x140u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rcx = ((rbx.wrapping_add(P3)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P4, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142483b00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1413c6300; const P1: u64 = 0x1433e7ad0; const P2: u64 = 0x8; const P3: u64 = 0x10; const P4: u64 = 0x13; const P5: u64 = 0x14; const P6: u64 = 0x18; const P7: u64 = 0x118; const P8: u64 = 0x120; const P9: u64 = 0x128; const P10: u64 = 0x130; const P11: u64 = 0x138; const P12: u64 = 0x1401de410;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = P1; crate::simd_ops::wu64(rdi.wrapping_add(P2), rax); crate::simd_ops::lk_and_u32(rdi.wrapping_add(P3), (0xff000020u32) as u32); crate::simd_ops::lk_or_u32(rdi.wrapping_add(P3), (0x20u32) as u32); crate::simd_ops::wu8(rdi.wrapping_add(P4), (0x20u8) as u8); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rcx as u32)) as u32); crate::simd_ops::wu8(rdi.wrapping_add(P6), ((rcx as u8)) as u8); crate::simd_ops::wu32(rdi.wrapping_add(P7), ((rcx as u32)) as u32); crate::simd_ops::wu64(rdi.wrapping_add(P8), rcx); crate::simd_ops::wu64(rdi.wrapping_add(P9), rcx); crate::simd_ops::wu64(rdi.wrapping_add(P10), rcx); crate::simd_ops::wu64(rdi.wrapping_add(P11), rcx); crate::simd_ops::lk_and_u32(rdi.wrapping_add(0x140u64), (0xff000010u32) as u32); crate::simd_ops::lk_or_u32(rdi.wrapping_add(0x140u64), (0x10u32) as u32); crate::simd_ops::wu8(rdi.wrapping_add(0x143u64), (0x10u8) as u8); crate::simd_ops::wu32(rdi.wrapping_add(0x144u64), ((rcx as u32)) as u32); crate::simd_ops::wu8(rdi.wrapping_add(0x148u64), ((rcx as u8)) as u8); rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x1c8u64), rdi); crate::simd_ops::wu64(rdi.wrapping_add(0x1d0u64), rax); crate::simd_ops::wu64(rdi.wrapping_add(0x1d8u64), rcx); rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x1e0u64), rdi); crate::simd_ops::wu64(rdi.wrapping_add(0x1e8u64), rax); rbx = (crate::simd_ops::ru64(rdi)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = 0u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); crate::simd_ops::wu16(rbx.wrapping_add(0xeu64), (0x3u16) as u16); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142483c20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xe; const P2: u64 = 0x1423ffd20; const P3: u64 = 0x14330f110; const P4: u64 = 0x142482100; const P5: u64 = 0x1413c2960; const P6: u64 = 0x142482340; const P7: u64 = 0x1413415d0; const P8: u64 = 0x142483ce0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x142483c20, funcinfo_rva: 0x37eacd4, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x68, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r9 = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; r10 = (0xffffffffffffu64) as u64; r9 = (((r9) & (r10))) as u64; rax = ((rdx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; r9 = ((r9).wrapping_add(rax)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; rcx = ((crate::simd_ops::ru16(r9.wrapping_add(P1))) as u32) as u32 as u64; rax = (((rcx as u16)) as u32) as u32 as u64; rax = (rax & !0xffffu64) | (((((rax as u16)).wrapping_shr(10u32)) as u16 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { fl_l = ((((rcx as u16)) >> (12u32)) & 1) as u64; if ((fl_l as u16) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { r9 = (crate::simd_ops::ru64(r9.wrapping_add(P0))) as u64; fl_l = (((r9) & (r10))) as u64 as u64; fl_r = fl_l; r9 = ((fl_l as u64)) as u64; bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r9) as u64); rax = (0xffffffffffffffffu64) as u64; bb = 4; continue; }
            4 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r9.wrapping_add(rax))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rdx = (r8) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142483c94); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x60u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { r8 = ((P3) as u64) as u64; rdx = (rbx) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142483cb4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142483cbd); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 8; continue; }
            8 => { r8 = ((P3) as u64) as u64; rdx = (rbx) as u64; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142483cd2); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142483cdb); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P8, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142483d80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x4; const P2: u64 = 0x10; const P3: u64 = 0x1; const P4: u64 = 0x1413ca700;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x142483d80, funcinfo_rva: 0x37ead0c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x38, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rsi = (rdx) as u64; rdi = (rcx) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rax = (0x4u32) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x2eu64), ((rax as u16)) as u16); rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = ((rax.wrapping_add(P2)) as u32) as u32 as u64; bb = 4; continue; }
            3 => { rdx = ((rax.wrapping_add(P3)) as u32) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru64(r8.wrapping_add(P2))) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142483dd1); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; r8 = (0xffffffffffffu64) as u64; rdx = (((rdx) & (r8))) as u64; rcx = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = ((rcx.wrapping_add(P3)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu16(rsp.wrapping_add(0x2eu64), ((rax as u16)) as u16); rcx = (crate::simd_ops::ru32(rbx)) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub(1)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rax = (((rax) & (r8))) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rsi, (rax) as u64); crate::simd_ops::wu64(rsi.wrapping_add(P0), (rcx) as u64); rax = (rsi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142483f50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x4; const P2: u64 = 0x10; const P3: u64 = 0x1; const P4: u64 = 0x1413ca700;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x142483f50, funcinfo_rva: 0x37ead30, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x38, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = (crate::simd_ops::ru64(rax)) as u64; rcx = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rcx) as u64); rcx = (((rcx as u32)).wrapping_shr(26u32)) as u32 as u64; rcx = (rcx & !0xffffu64) | (((!((rcx as u16))) as u16 as u64)); rcx = (rcx & !0xffffu64) | ((((((rcx as u16)) & (0x20u16))) as u16 as u64)); rax = (0x1d6u32) as u32 as u64; rcx = (rcx & !0xffffu64) | ((((((rcx as u16)) | ((rax as u16)))) as u16 as u64)); crate::simd_ops::wu16(rsp.wrapping_add(0x2eu64), ((rcx as u16)) as u16); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = ((rax.wrapping_add(P2)) as u32) as u32 as u64; bb = 4; continue; }
            3 => { rdx = ((rax.wrapping_add(P3)) as u32) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru64(r8.wrapping_add(P2))) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142483fa7); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rax = (0xffffffffffffu64) as u64; rdx = (((rdx) & (rax))) as u64; rcx = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = ((rcx.wrapping_add(P3)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142483ff0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x4; const P2: u64 = 0x10; const P3: u64 = 0x1; const P4: u64 = 0x1413ca700;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x142483ff0, funcinfo_rva: 0x37ead54, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x38, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = (crate::simd_ops::ru64(rax)) as u64; rcx = (((crate::simd_ops::ru8(rdx)) as i8 as i64) as u64) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rcx) as u64); rax = (((rcx as u16)) as u32) as u32 as u64; rax = (rax & !0xffffu64) | (((((((rax as u16)) as i16).wrapping_shr(15u32) as u16)) as u16 as u64)); rcx = (0xfec0u32) as u32 as u64; rax = (rax & !0xffffu64) | ((((((rax as u16)) & ((rcx as u16)))) as u16 as u64)); rcx = (0x1f6u32) as u32 as u64; rax = (rax & !0xffffu64) | (((((rax as u16)).wrapping_add((rcx as u16))) as u16 as u64)); crate::simd_ops::wu16(rsp.wrapping_add(0x2eu64), ((rax as u16)) as u16); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = ((rax.wrapping_add(P2)) as u32) as u32 as u64; bb = 4; continue; }
            3 => { rdx = ((rax.wrapping_add(P3)) as u32) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru64(r8.wrapping_add(P2))) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14248404d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rax = (0xffffffffffffu64) as u64; rdx = (((rdx) & (rax))) as u64; rcx = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = ((rcx.wrapping_add(P3)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424841f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x4; const P2: u64 = 0x10; const P3: u64 = 0x1; const P4: u64 = 0x1413ca700;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1424841f0, funcinfo_rva: 0x37ead78, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x38, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rdx)) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rcx) as u64); rdx = (0x116u32) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x2eu64), ((rdx as u16)) as u16); rax = (0x196u32) as u32 as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu16(rsp.wrapping_add(0x2eu64), ((rax as u16)) as u16); bb = 2; continue; }
            2 => { rax = (rax & !0xffffu64) | (((if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { (rdx as u16) } else { (rax as u16) }) as u16 as u64)); rdx = (0xffffffff00000000u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (0x1d6u32) as u32 as u64; rdx = (0x156u32) as u32 as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; rax = (rax & !0xffffu64) | (((if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { (rdx as u16) } else { (rax as u16) }) as u16 as u64)); crate::simd_ops::wu16(rsp.wrapping_add(0x2eu64), ((rax as u16)) as u16); bb = 4; continue; }
            4 => { fl_l = (rcx) as u64 as u64; fl_r = (0xffffffff80000000u64) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = ((((rax as u16)) | (0x20u16))) as u16 as u64; fl_r = fl_l; rax = (rax & !0xffffu64) | ((((fl_l as u16)) as u16 as u64)); crate::simd_ops::wu16(rsp.wrapping_add(0x2eu64), ((rax as u16)) as u16); bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 11; continue; } bb = 7; continue; }
            7 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = ((rax.wrapping_add(P2)) as u32) as u32 as u64; bb = 10; continue; }
            9 => { rdx = ((rax.wrapping_add(P3)) as u32) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; bb = 10; continue; }
            10 => { r8 = (crate::simd_ops::ru64(r8.wrapping_add(P2))) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142484284); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rax = (0xffffffffffffu64) as u64; rdx = (((rdx) & (rax))) as u64; rcx = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = ((rcx.wrapping_add(P3)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424842d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x4; const P2: u64 = 0x10; const P3: u64 = 0x1; const P4: u64 = 0x1413ca700;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1424842d0, funcinfo_rva: 0x37ead9c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x38, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = (crate::simd_ops::ru64(rax)) as u64; rcx = (((crate::simd_ops::ru32(rdx)) as i32 as i64) as u64) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rcx) as u64); rax = (0xb6u32) as u32 as u64; rdx = (0x1f6u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rax = (rax & !0xffffu64) | (((if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { (rdx as u16) } else { (rax as u16) }) as u16 as u64)); crate::simd_ops::wu16(rsp.wrapping_add(0x2eu64), ((rax as u16)) as u16); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = ((rax.wrapping_add(P2)) as u32) as u32 as u64; bb = 4; continue; }
            3 => { rdx = ((rax.wrapping_add(P3)) as u32) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru64(r8.wrapping_add(P2))) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142484325); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rax = (0xffffffffffffu64) as u64; rdx = (((rdx) & (rax))) as u64; rcx = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = ((rcx.wrapping_add(P3)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424845b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x4; const P2: u64 = 0x10; const P3: u64 = 0x1; const P4: u64 = 0x1413ca700;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1424845b0, funcinfo_rva: 0x37eadec, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x38, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rsi = (rdx) as u64; rdi = (rcx) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rax = (0x3u32) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x2eu64), ((rax as u16)) as u16); rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = ((rax.wrapping_add(P2)) as u32) as u32 as u64; bb = 4; continue; }
            3 => { rdx = ((rax.wrapping_add(P3)) as u32) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru64(r8.wrapping_add(P2))) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142484601); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; r8 = (0xffffffffffffu64) as u64; rdx = (((rdx) & (r8))) as u64; rcx = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = ((rcx.wrapping_add(P3)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu16(rsp.wrapping_add(0x2eu64), ((rdx as u16)) as u16); rcx = (crate::simd_ops::ru32(rbx)) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub(1)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rax = (((rax) & (r8))) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rsi, (rax) as u64); crate::simd_ops::wu64(rsi.wrapping_add(P0), (rcx) as u64); crate::simd_ops::wu64(rsi.wrapping_add(P2), (rdx) as u64); rax = (rsi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142484700(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x4; const P2: u64 = 0x10; const P3: u64 = 0x1; const P4: u64 = 0x1413ca700;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x142484700, funcinfo_rva: 0x37eae34, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x38, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = (crate::simd_ops::ru64(rax)) as u64; rcx = ((crate::simd_ops::ru16(rdx)) as u32) as u32 as u64; xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rcx) as u64); rcx = (((rcx as u32)).wrapping_shr(26u32)) as u32 as u64; rcx = (rcx & !0xffffu64) | (((!((rcx as u16))) as u16 as u64)); rcx = (rcx & !0xffffu64) | ((((((rcx as u16)) & (0x20u16))) as u16 as u64)); rax = (0x1d6u32) as u32 as u64; rcx = (rcx & !0xffffu64) | ((((((rcx as u16)) | ((rax as u16)))) as u16 as u64)); crate::simd_ops::wu16(rsp.wrapping_add(0x2eu64), ((rcx as u16)) as u16); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = ((rax.wrapping_add(P2)) as u32) as u32 as u64; bb = 4; continue; }
            3 => { rdx = ((rax.wrapping_add(P3)) as u32) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru64(r8.wrapping_add(P2))) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142484757); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rax = (0xffffffffffffu64) as u64; rdx = (((rdx) & (rax))) as u64; rcx = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = ((rcx.wrapping_add(P3)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424847a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1413c3800; const P1: u64 = 0x14183acbc; const P2: u64 = 0x1413ca700;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1424847a0, funcinfo_rva: 0x37eae68, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x38, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rbp = (crate::simd_ops::ru64(rax)) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu16(rsp.wrapping_add(0x2eu64), ((rax as u16)) as u16); r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; rbx = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; r14 = ((rdx.wrapping_add(0x8u64)) as u64) as u64; rcx = ((crate::simd_ops::ru8(rdx.wrapping_add(0x3u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 3; continue; }
            2 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { r14 = (crate::simd_ops::ru64(r14)) as u64; bb = 5; continue; }
            5 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = (0xdu32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (0x1c05u32) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x2eu64), ((rax as u16)) as u16); rax = (0xdu32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; rax = (rax & !0xffu64) | ((((fl_l as u8).wrapping_sub(fl_r as u8)) as u8 as u64)); crate::simd_ops::wu8(rsp.wrapping_add(0x2du64), ((rax as u8)) as u8); rsi = ((rsp.wrapping_add(0x20u64)) as u64) as u64; bb = 8; continue; }
            7 => { rax = (0xc05u32) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x2eu64), ((rax as u16)) as u16); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rbx as u32)) as u32); rdx = ((rbx.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = (r8) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142484844); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rsi = (rax) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = (0xffff000000000000u64) as u64; rcx = (((rcx) & (rax))) as u64; fl_l = (((rcx) | (rsi))) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rcx) as u64); bb = 8; continue; }
            8 => { r8 = (rbx) as u64; rdx = (r14) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14248486f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu8(rbx.wrapping_add(rsi), (0x0u8) as u8); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x4u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 13; continue; } bb = 9; continue; }
            9 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdx = ((rax.wrapping_add(0x10u64)) as u32) as u32 as u64; bb = 12; continue; }
            11 => { rdx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; bb = 12; continue; }
            12 => { r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142484896); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 13; continue; }
            13 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rax = (0xffffffffffffu64) as u64; rdx = (((rdx) & (rax))) as u64; rcx = (crate::simd_ops::ru32(rdi)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi, ((rax as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424848f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x4; const P2: u64 = 0x10; const P3: u64 = 0x1; const P4: u64 = 0x1413ca700;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1424848f0, funcinfo_rva: 0x37eae90, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x38, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru32(rdx)) as u32 as u64; xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rcx) as u64); rcx = (((rcx as u32)).wrapping_shr(26u32)) as u32 as u64; rcx = (rcx & !0xffffu64) | (((!((rcx as u16))) as u16 as u64)); rcx = (rcx & !0xffffu64) | ((((((rcx as u16)) & (0x20u16))) as u16 as u64)); rax = (0x1d6u32) as u32 as u64; rcx = (rcx & !0xffffu64) | ((((((rcx as u16)) | ((rax as u16)))) as u16 as u64)); crate::simd_ops::wu16(rsp.wrapping_add(0x2eu64), ((rcx as u16)) as u16); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = ((rax.wrapping_add(P2)) as u32) as u32 as u64; bb = 4; continue; }
            3 => { rdx = ((rax.wrapping_add(P3)) as u32) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru64(r8.wrapping_add(P2))) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142484946); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rax = (0xffffffffffffu64) as u64; rdx = (((rdx) & (rax))) as u64; rcx = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = ((rcx.wrapping_add(P3)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142484990(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x4; const P2: u64 = 0x10; const P3: u64 = 0x1; const P4: u64 = 0x1413ca700;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x142484990, funcinfo_rva: 0x37eaeb4, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x38, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru32(rdx)) as u32) as u128; xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | (((f32::from_bits(xmm1 as u32)) as f64).to_bits() as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), ((xmm1 as u64)) as u64); rax = (0x216u32) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x2eu64), ((rax as u16)) as u16); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = ((rax.wrapping_add(P2)) as u32) as u32 as u64; bb = 4; continue; }
            3 => { rdx = ((rax.wrapping_add(P3)) as u32) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424849e0); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rax = (0xffffffffffffu64) as u64; rdx = (((rdx) & (rax))) as u64; rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rdx.wrapping_add(rcx.wrapping_mul(8)), (xmm0) as u128); rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142484a20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x4; const P2: u64 = 0x10; const P3: u64 = 0x1; const P4: u64 = 0x1413ca700;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x142484a20, funcinfo_rva: 0x37eaed8, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x38, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru8(rdx)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rax = (rax & !0xffu64) | (((if (((fl_l as u8)) != ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = (rax & !0xffffu64) | (((((rax as u16)).wrapping_add(0x9u16)) as u16 as u64)); crate::simd_ops::wu16(rsp.wrapping_add(0x2eu64), ((rax as u16)) as u16); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = ((rax.wrapping_add(P2)) as u32) as u32 as u64; bb = 4; continue; }
            3 => { rdx = ((rax.wrapping_add(P3)) as u32) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142484a68); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rax = (0xffffffffffffu64) as u64; rdx = (((rdx) & (rax))) as u64; rcx = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = ((rcx.wrapping_add(P3)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142484ab0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0xe; const P3: u64 = 0x14330f110; const P4: u64 = 0x1424823d0; const P5: u64 = 0x1401eb1d0; const P6: u64 = 0x142484b17;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x142484ab0, funcinfo_rva: 0x37eaefc, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x48, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x48u64)) as u64; rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdx, (rax) as u64); crate::simd_ops::wu64(rdx.wrapping_add(P0), (rax) as u64); crate::simd_ops::wu64(rdx.wrapping_add(P1), (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r9 = ((r8 as u32)) as u32 as u64; r9 = ((r9).wrapping_shl(4u32)) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; rax = (0xffffffffffffu64) as u64; r8 = (((r8) & (rax))) as u64; r9 = ((r9).wrapping_add(r8)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = (crate::simd_ops::ru16(r9.wrapping_add(P2))) as u16 as u64; fl_r = (0x3u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rdx.wrapping_add(P0), (r9) as u64); crate::simd_ops::wu64(rdx, (rax) as u64); rax = (rdx) as u64; rsp = ((rsp).wrapping_add(0x48u64)) as u64; return rax; }
            2 => { r8 = ((P3) as u64) as u64; rdx = (rax) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142484b09); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142484b12); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P6, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142484b20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xe;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    r9 = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64;
    rax = (0xffffffffffffu64) as u64;
    r9 = (((r9) & (rax))) as u64;
    rax = ((rdx as u32)) as u32 as u64;
    rax = ((rax).wrapping_shl(4u32)) as u64;
    r9 = ((r9).wrapping_add(rax)) as u64;
    fl_l = (crate::simd_ops::ru16(r9.wrapping_add(P1))) as u16 as u64; fl_r = (0x4u16) as u16 as u64;
    if !((((fl_l as u16)) != ((fl_r as u16)))) { crate::simd_ops::wu64(r8.wrapping_add(P0), r9); rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(r8, rax); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142484b60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xe;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    r9 = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64;
    rax = (0xffffffffffffu64) as u64;
    r9 = (((r9) & (rax))) as u64;
    rax = ((rdx as u32)) as u32 as u64;
    rax = ((rax).wrapping_shl(4u32)) as u64;
    r9 = ((r9).wrapping_add(rax)) as u64;
    fl_l = (crate::simd_ops::ru16(r9.wrapping_add(P1))) as u16 as u64; fl_r = (0x3u16) as u16 as u64;
    if !((((fl_l as u16)) != ((fl_r as u16)))) { crate::simd_ops::wu64(r8.wrapping_add(P0), r9); rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(r8, rax); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142484de0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1424ba100; const P1: u64 = 0x141378730; const P2: u64 = 0x1413cbcb0; const P3: u64 = 0x141834ee8; const P4: u64 = 0x142442de0; const P5: u64 = 0x1424076e0; const P6: u64 = 0x1425371a0; const P7: u64 = 0x142533200;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x1000u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x1000u64), (0x0u64) as u64); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (0x0u64) as u64); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rdi.wrapping_add(0x40); r9 = P2; rdx = 0x40; r8 = ((rdx as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), (0x0u64) as u64); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 9; continue; }
            9 => { rcx = rdi.wrapping_add(0x40); r9 = P2; rdx = 0x40; r8 = ((rdx as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r10 = rdi.wrapping_sub(0x1); r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = rdi; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = r10.wrapping_add(0x108); rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 14; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 13; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rdi, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), rdi); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 16; continue; } bb = 12; continue; }
            12 => { rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            13 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 15; continue; }
            14 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 15; continue; }
            15 => { rdx = rcx; r9 = rdi; rcx = r10; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 17; continue; }
            17 => { rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); r10 = rbx.wrapping_sub(0x1); r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = rbx; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = r10.wrapping_add(0x108); rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 22; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 21; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), rbx); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 24; continue; } bb = 20; continue; }
            20 => { rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 24; continue; }
            21 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 23; continue; }
            22 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 23; continue; }
            23 => { rdx = rcx; r9 = rbx; rcx = r10; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 24; continue; }
            24 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142484fd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a082f8; const P1: u64 = 0x143c1bfb4; const P2: u64 = 0x143c1bfa8; const P3: u64 = 0x1413d4870; const P4: u64 = 0x143c1bfb0; const P5: u64 = 0x1413cb530; const P6: u64 = 0x1424076e0; const P7: u64 = 0x1425371a0; const P8: u64 = 0x142533200; const P9: u64 = 0x142442de0; const P10: u64 = 0x1413cb6d0; const P11: u64 = 0x1413cbcb0; const P12: u64 = 0x141834ee8; const P13: u64 = 0x141378490; const P14: u64 = 0x141378730; const P15: u64 = 0x1434077d8; const P16: u64 = 0x1423fe030; const P17: u64 = 0x141378580; const P18: u64 = 0x1423f0c20; const P19: u64 = 0x1423f0b30; const P20: u64 = 0x142645868; const P21: u64 = 0x1413cb850;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x142484fd0, funcinfo_rva: 0x37eafa0, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x98, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rsi = (rdx) as u64; r15 = (rcx) as u64; rbx = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rcx, (rbx) as u64); r12 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), (r12) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), (r12) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), (r12) as u64); crate::simd_ops::wu64(P0, (rcx) as u64); r11 = (crate::simd_ops::ru32(P1)) as u32 as u64; rcx = ((r11.wrapping_add(r11.wrapping_mul(2))) as u64) as u64; r8 = (crate::simd_ops::ru64(P2)) as u64; rdx = ((r8.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_sub(r8)) as u64; r14 = (0x2aaaaaaaaaaaaaabu64) as u64; rax = (r14) as u64; let prod = (rax as i64 as i128).wrapping_mul((rdx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(1u32) as u64)) as u64; rax = (rdx) as u64; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rbp = (0x1u32) as u32 as u64; fl_l = (rdx) as u64 as u64; fl_r = (rbp) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = ((rbp as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142485047); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r11 = (crate::simd_ops::ru32(P1)) as u32 as u64; r8 = (crate::simd_ops::ru64(P2)) as u64; bb = 2; continue; }
            2 => { rdi = ((rbx.wrapping_sub(0x1u64)) as u32) as u32 as u64; r10 = (crate::simd_ops::ru32(P4)) as u32 as u64; rdx = ((r10.wrapping_add(r10.wrapping_mul(2))) as u64) as u64; r9 = ((r8.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; rcx = ((r11 as u32)) as u32 as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = ((r8.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; fl_l = (r9) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { r9 = ((r9).wrapping_sub(r8)) as u64; rax = (r14) as u64; let prod = (rax as i64 as i128).wrapping_mul((r9) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(1u32) as u64)) as u64; rax = (rdx) as u64; rax = ((rax).wrapping_shr(63u32)) as u64; rax = ((rax).wrapping_add(1)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142485095); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r10 = (crate::simd_ops::ru32(P4)) as u32 as u64; r8 = (crate::simd_ops::ru64(P2)) as u64; bb = 4; continue; }
            4 => { rax = ((r10 as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu32(r8.wrapping_add(rcx.wrapping_mul(4)), ((rbx as u32)) as u32); crate::simd_ops::wu32(r8.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x4u64), ((r12 as u32)) as u32); crate::simd_ops::wu32(r8.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x8u64), ((rdi as u32)) as u32); crate::simd_ops::lk_add_u32(P4, (1) as u32); rcx = ((rsp.wrapping_add(0xb0u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424850cb); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rax, (r12) as u64); rbx = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r15.wrapping_add(0x8u64), (rcx) as u64); fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 5; continue; }
            5 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424850ea); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r10 = ((rbx.wrapping_sub(0x1u64)) as u64) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64 as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = (rbx) as u64; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = ((r10.wrapping_add(0x108u64)) as u64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rbx, (rax) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), (rbx) as u64); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14248514a); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 12; continue; }
            9 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 11; continue; }
            10 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 11; continue; }
            11 => { rdx = (rcx) as u64; r9 = (rbx) as u64; rcx = (r10) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142485162); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142485178); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142485180); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 14; continue; }
            14 => { r9 = ((rsi.wrapping_add(0x8u64)) as u64) as u64; r8 = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; rcx = ((rsp.wrapping_add(0xb8u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142485196); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rax, (r12) as u64); rbx = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(r15.wrapping_add(0x10u64), (rcx) as u64); fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 15; continue; }
            15 => { rcx = ((rbx.wrapping_add(0x40u64)) as u64) as u64; r9 = ((P11) as u64) as u64; rdx = (0x40u32) as u32 as u64; r8 = ((rdx as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424851c5); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r10 = ((rbx.wrapping_sub(0x1u64)) as u64) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64 as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = (rbx) as u64; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = ((r10.wrapping_add(0x108u64)) as u64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 20; continue; } bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 19; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rbx, (rax) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), (rbx) as u64); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 22; continue; } bb = 18; continue; }
            18 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142485226); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 22; continue; }
            19 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 21; continue; }
            20 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 21; continue; }
            21 => { rdx = (rcx) as u64; r9 = (rbx) as u64; rcx = (r10) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14248523e); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 22; continue; }
            22 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { rcx = ((rbx.wrapping_add(0x40u64)) as u64) as u64; r9 = ((P11) as u64) as u64; rdx = (0x40u32) as u32 as u64; r8 = ((rdx as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142485264); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14248526d); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 24; continue; }
            24 => { rsi = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(r15)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); rax = ((rsp.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); r9 = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; r8 = (rsi) as u64; rcx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142485295); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rax, (r12) as u64); rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x1000u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x1000u64), (rcx) as u64); fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 25; continue; }
            25 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424852b3); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P14) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 26; continue; }
            26 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 27; continue; }
            27 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424852c2); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P14) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 28; continue; }
            28 => { rsi = (crate::simd_ops::ru64(rsi.wrapping_add(0x1000u64))) as u64; rdi = (r12) as u64; fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 38; continue; } bb = 29; continue; }
            29 => { bb = 30; continue; }
            30 => { rbx = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xa8u64), ((rbx as u32)) as u32); rdx = ((rdi as u32)) as u32 as u64; rcx = ((P15) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424852f2); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P16) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rax) as u64); rax = ((rsp.wrapping_add(0x40u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); rax = ((rsp.wrapping_add(0xa8u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); r9 = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; r8 = (crate::simd_ops::ru64(rsi)) as u64; rcx = ((rsp.wrapping_add(0x48u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14248531f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P17) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r14 = ((rbx.wrapping_mul(8)) as u64) as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rax, (r12) as u64); rbx = (crate::simd_ops::ru64(r14.wrapping_add(rcx))) as u64; crate::simd_ops::wu64(r14.wrapping_add(rcx), (rdx) as u64); fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 31; continue; }
            31 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142485346); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P18) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14248534e); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 32; continue; }
            32 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 34; continue; } bb = 33; continue; }
            33 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142485361); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P18) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142485369); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 34; continue; }
            34 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; rbx = (crate::simd_ops::ru64(r14.wrapping_add(rax))) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14248537a); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P19) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 37; continue; } bb = 35; continue; }
            35 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x131u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 37; continue; } bb = 36; continue; }
            36 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14248538f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P20)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 35; continue; }
            37 => { rdi = ((rdi).wrapping_add(1)) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; fl_l = (rdi) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 30; continue; } bb = 38; continue; }
            38 => { rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424853ab); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P21) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rax, (r12) as u64); rbx = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(r15.wrapping_add(0x18u64), (rcx) as u64); fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 46; continue; } bb = 39; continue; }
            39 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424853cf); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r10 = ((rbx.wrapping_sub(0x1u64)) as u64) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64 as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = (rbx) as u64; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = ((r10.wrapping_add(0x108u64)) as u64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 44; continue; } bb = 40; continue; }
            40 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 43; continue; } bb = 41; continue; }
            41 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rbx, (rax) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), (rbx) as u64); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 46; continue; } bb = 42; continue; }
            42 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14248542c); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 46; continue; }
            43 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 45; continue; }
            44 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 45; continue; }
            45 => { rdx = (rcx) as u64; r9 = (rbx) as u64; rcx = (r10) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142485444); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 46; continue; }
            46 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 48; continue; } bb = 47; continue; }
            47 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14248545c); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142485461); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 48; continue; }
            48 => { rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rcx.wrapping_add(0x41e8u64), ((rbp as u32)) as u32); rbp = (tmp_xchg) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142485471); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P19) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (r15) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142485490(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x142406a80;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    r8 = 0xffffffff;
    return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424859d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1413cb440; const P1: u64 = 0x143a082f8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    r8 = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(P1, rax);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424859f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x28; const P2: u64 = 0x142645720; const P3: u64 = 0x143841718; const P4: u64 = 0x10; const P5: u64 = 0x18; const P6: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x50u64)) as u64;
    crate::simd_ops::wu64(rcx, (0x0u64) as u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    xmm0 = 0u128; ymm0_hi = 0;
    let y_addr: u64 = rcx.wrapping_add(P0); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    rbx = rcx;
    rcx = rsp.wrapping_add(0x20);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rcx = (crate::simd_ops::ru32(P3)) as u32 as u64;
    rax = 0x1;
    fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64;
    crate::simd_ops::wu64(rbx.wrapping_add(P0), (0x400u64) as u64);
    rcx = (if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { crate::simd_ops::ru32(rsp.wrapping_add(0x40u64)) } else { (rcx as u32) }) as u32 as u64;
    rcx = (((rcx as u32)).wrapping_add(0xfffffffeu32)) as u32 as u64;
    crate::simd_ops::wu64(rbx.wrapping_add(P4), (0x100u64) as u64);
    fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    crate::simd_ops::wu64(rbx.wrapping_add(P5), (0x400u64) as u64);
    crate::simd_ops::wu64(rbx.wrapping_add(P6), (0x80u64) as u64);
    rax = (if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { (rcx as u32) } else { (rax as u32) }) as u32 as u64;
    crate::simd_ops::wu64(rbx.wrapping_add(P1), (0x4u64) as u64);
    rax = (rax as u32 as i32 as i64) as u64;
    crate::simd_ops::wu64(rbx, rax);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x50u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424860e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1413cbcb0; const P1: u64 = 0x141834ee8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = ((rcx).wrapping_add(0x40u64)) as u64;
    r9 = P0;
    rdx = 0x40;
    r8 = ((rdx as u32)) as u32 as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142486110(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1413cbcb0; const P1: u64 = 0x1413cbc90; const P2: u64 = 0x141834fc0; const P3: u64 = 0x1413cbd10; const P4: u64 = 0x1413cbc70; const P5: u64 = 0x141834ee8; const P6: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x38u64)) as u64; r14 = rcx; crate::simd_ops::wu64(rcx, rdx); rdi = rcx.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rdi); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = P1; rdx = 0x40; r8 = ((rdx as u32)) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r15 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r14.wrapping_add(0x1040u64), r15); crate::simd_ops::wu64(r14.wrapping_add(0x1048u64), r15); crate::simd_ops::wu32(r14.wrapping_add(0x1050u64), ((r15 as u32)) as u32); rbp = ((r15 as u32)) as u32 as u64; rbx = rdi.wrapping_add(0x1000); fl_l = rdi; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rsp.wrapping_add(0x78u64), ((rbp as u32)) as u32); r9 = rsp.wrapping_add(0x80); r8 = rsp.wrapping_add(0x78); rcx = rsp.wrapping_add(0x88); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rax, r15); rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rdx); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { r9 = P4; rdx = 0x40; r8 = ((rdx.wrapping_sub(0x3bu64)) as u32) as u32 as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { r9 = P4; rdx = 0x40; r8 = ((rdx.wrapping_sub(0x3bu64)) as u32) as u32 as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rbp = ((rbp).wrapping_add(1)) as u64; rdi = ((rdi).wrapping_add(0x40u64)) as u64; fl_l = rdi; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 6; continue; }
            6 => { rax = r14; rsp = ((rsp).wrapping_add(0x38u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142486530(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1008; const P1: u64 = 0x142645580;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    r8 = 0;
    fl_l = 0; fl_r = 0;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    return crate::simd_ops::call_fwd(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142486550(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142407c80; const P1: u64 = 0x48; const P2: u64 = 0x142485aa0;
    let (mut rax, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (((r8 as u16)) as u32) as u32 as u64;
    rsi = rdx;
    rdi = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    r9 = ((rax as u32)) as u32 as u64;
    rdx = rsi;
    r9 = ((r9).wrapping_shl(6u32)) as u64;
    rcx = (((rbx as u8)) as u32) as u32 as u64;
    rcx = ((rcx).wrapping_shl(6u32)) as u64;
    rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r9.wrapping_add(rdi).wrapping_add(P1)))) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rsi;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424865d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143428ae0; const P1: u64 = 0x14340b588; const P2: u64 = 0x143c4a9b0; const P3: u64 = 0x143c4aca0; const P4: u64 = 0x143c4a9d8; const P5: u64 = 0x143c4aca8; const P6: u64 = 0x143c4a9b8; const P7: u64 = 0x143c4a9e0; const P8: u64 = 0x143a392e8; const P9: u64 = 0x14139db10; const P10: u64 = 0x143a082b0;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    let y_addr: u64 = P0; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    rax = P1;
    crate::simd_ops::wu8(P2, (0x0u8) as u8);
    crate::simd_ops::wu64(P3, rax);
    r8 = rsp.wrapping_add(0x40);
    rax = P2;
    crate::simd_ops::wu8(P4, (0x0u8) as u8);
    crate::simd_ops::wu64(P5, rax);
    rbx = rcx;
    let y_addr: u64 = P6; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    let y_addr: u64 = P7; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    rax = P3;
    let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(P8, rax);
    rax = (tmp_xchg) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (0x1fffc0u64) as u64);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    crate::simd_ops::wu64(P10, rax);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142486680(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a082e8; const P1: u64 = 0x1413bafb0; const P2: u64 = 0x142442de0; const P3: u64 = 0x143a083f8; const P4: u64 = 0x14240ab10; const P5: u64 = 0x143a08400; const P6: u64 = 0x143a082a8; const P7: u64 = 0x14139d6b0; const P8: u64 = 0x143a08328; const P9: u64 = 0x143a39258; const P10: u64 = 0x143a00250; const P11: u64 = 0x141834d30; const P12: u64 = 0x1426463e0; const P13: u64 = 0x143a00280; const P14: u64 = 0x143a083e0; const P15: u64 = 0x1401e5380; const P16: u64 = 0x14028cc60; const P17: u64 = 0x143a083f0; const P18: u64 = 0x1401e66f0; const P19: u64 = 0x14241bd10; const P20: u64 = 0x143a392e8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rcx; rbx = (crate::simd_ops::ru64(P0)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(P0, rdi); rbx = (crate::simd_ops::ru64(P3)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rbx.wrapping_add(0x1368); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x12e0); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x1258); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x11d0); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x1148); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x10c0); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x1038); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0xfb0); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0xf28); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0xea0); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0xe18); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0xd90); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0xd08); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0xc80); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0xb78); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0xa70); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x968); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x860); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x758); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x6d0); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x648); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x5c0); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x538); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x4b0); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x428); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x3a0); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x298); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x190); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x108); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { crate::simd_ops::wu64(P3, rdi); rbx = (crate::simd_ops::ru64(P5)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = rbx.wrapping_add(0x268); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x1e0); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x158); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x110); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x88); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { crate::simd_ops::wu64(P5, rdi); rbx = (crate::simd_ops::ru64(P6)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { crate::simd_ops::wu64(P6, rdi); rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(P8)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(P9)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu64(P10, rdi); rdx = 0x1; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(P13)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            16 => { crate::simd_ops::wu64(P13, rdi); rbx = (crate::simd_ops::ru64(P14)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { rcx = rbx.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P15, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x8); rax = crate::simd_ops::call_stk(P16, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 18; continue; }
            18 => { crate::simd_ops::wu64(P14, rdi); rsi = (crate::simd_ops::ru64(P17)) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 19; continue; }
            19 => { rcx = rsi.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P15, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rax, rdi); rdx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { rbx = (crate::simd_ops::ru64(rdx)) as u64; rax = crate::simd_ops::call_stk(P18, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 20; continue; } bb = 21; continue; }
            21 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rax = crate::simd_ops::call_stk(P18, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 22; continue; }
            22 => { crate::simd_ops::wu64(P17, rdi); rax = crate::simd_ops::call_stk(P19, rax, rcx, rdx, r8, r9, rsp); let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(P20, rdi); rdi = (tmp_xchg) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424869d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143c16860; const P1: u64 = 0x143c4a9b0; const P2: u64 = 0x143c4a9b8; const P3: u64 = 0x143c4a9c0; const P4: u64 = 0x143c4a9c8; const P5: u64 = 0x143c4a9d0; const P6: u64 = 0x143c4a9d8; const P7: u64 = 0x143c4a9e0; const P8: u64 = 0x143c4a9e8; const P9: u64 = 0x143c4a9f0; const P10: u64 = 0x143c4a9f8; const P11: u64 = 0x14340b588; const P12: u64 = 0x143c4aca0; const P13: u64 = 0x143c4aca8; const P14: u64 = 0x143a392e8; const P15: u64 = 0x14139db10; const P16: u64 = 0x143a082b0; const P17: u64 = 0x142646a40; const P18: u64 = 0x142646a80; const P19: u64 = 0x143a083e0; const P20: u64 = 0x1413d8950; const P21: u64 = 0x1413d8a30; const P22: u64 = 0x143a083f0; const P23: u64 = 0x141376cb0; const P24: u64 = 0x143a00280; const P25: u64 = 0x1426463d8; const P26: u64 = 0x1426463e0; const P27: u64 = 0x141834d64; const P28: u64 = 0x143a00250; const P29: u64 = 0x143a39258; const P30: u64 = 0x143a08328; const P31: u64 = 0x1413d23b0; const P32: u64 = 0x1413cbef0; const P33: u64 = 0x143c16868; const P34: u64 = 0x143a082a8; const P35: u64 = 0x14139d730; const P36: u64 = 0x1413cbe60; const P37: u64 = 0x143a08400; const P38: u64 = 0x143a083f8; const P39: u64 = 0x1413d9350; const P40: u64 = 0x1413bacb0; const P41: u64 = 0x143a082e8;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 59>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1424869d0, funcinfo_rva: 0x37eb2b4, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x1d8, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = (rsp) as u64; crate::simd_ops::wu64(r11.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x1d0u64)) as u64; rbx = ((P0) as u64) as u64; crate::simd_ops::wu64(r11.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu8(P1, (0x0u8) as u8); rdx = (0xcacacacacacacacau64) as u64; crate::simd_ops::wu64(P2, (rdx) as u64); rax = (0xdadadadadadadadau64) as u64; crate::simd_ops::wu64(P3, (rax) as u64); rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(P4, (rcx) as u64); crate::simd_ops::wu64(P5, (rcx) as u64); crate::simd_ops::wu8(P6, ((rcx as u8)) as u8); crate::simd_ops::wu64(P7, (rdx) as u64); crate::simd_ops::wu64(P8, (rax) as u64); crate::simd_ops::wu64(P9, (rcx) as u64); crate::simd_ops::wu64(P10, (rcx) as u64); rax = ((P11) as u64) as u64; crate::simd_ops::wu64(P12, (rax) as u64); rax = ((P1) as u64) as u64; crate::simd_ops::wu64(P13, (rax) as u64); rax = ((P12) as u64) as u64; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(P14, (rax) as u64); rax = (tmp_xchg) as u64; crate::simd_ops::wu64(r11.wrapping_add(0x8u64), (0x1fffc0u64) as u64); r8 = ((r11.wrapping_add(0x8u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142486a7b); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P15) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(P16, (rax) as u64); rax = ((crate::exe::tls_pointer() as u64)) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; rax = (0x4b460u32) as u32 as u64; rax = ((rax).wrapping_add(rcx)) as u64; rdx = (0x463d8u32) as u32 as u64; crate::simd_ops::wu64(rdx.wrapping_add(rcx), (rax) as u64); rax = (0x83460u32) as u32 as u64; rax = ((rax).wrapping_add(rcx)) as u64; rdx = (0x463d0u32) as u32 as u64; crate::simd_ops::wu64(rdx.wrapping_add(rcx), (rax) as u64); rax = (0x6312u32) as u32 as u64; crate::simd_ops::wu8(rax.wrapping_add(rcx), (0x1u8) as u8); rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142486ac0); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P17)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = ((rax as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142486ac8); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P18)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (crate::simd_ops::ru64(P19)) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142486ad8); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P20) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(P19, (rax) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142486ae4); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P21) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(P22, (rax) as u64); bb = 2; continue; }
            2 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142486af0); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P23) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(P24, (rax) as u64); rcx = (0x2u32) as u32 as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142486b06); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P25)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru16(rsp.wrapping_add(0x30u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (rax & !0xffffu64) | (((((rax as u16)).wrapping_shr(8u32)) as u16 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142486b21); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P26)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 6; continue; }
            6 => { rcx = (0x1u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142486b2c); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P27) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rsp.wrapping_add(0x1e0u64), (rax) as u64); crate::simd_ops::wu64(P28, (rax) as u64); crate::simd_ops::wu64(P29, (rax) as u64); fl_l = (crate::simd_ops::ru64(P30)) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142486b51); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P31) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(P30, (rax) as u64); bb = 8; continue; }
            8 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142486b5d); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P32) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(P33, (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x1e0u64), (0x40000u64) as u64); fl_l = (crate::simd_ops::ru64(P34)) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { r8 = ((rsp.wrapping_add(0x1e0u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142486b87); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P35) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(P34, (rax) as u64); bb = 10; continue; }
            10 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142486b93); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P36) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(P37, (rax) as u64); fl_l = (crate::simd_ops::ru64(P38)) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142486ba9); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P39) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(P38, (rax) as u64); bb = 12; continue; }
            12 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142486bb5); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P40) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(P41, (rax) as u64); rax = (rbx) as u64; rsp = ((rsp).wrapping_add(0x1d0u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142486bf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x1426463f8; const P2: u64 = 0x2; const P3: u64 = 0x8; const P4: u64 = 0x142409b50; const P5: u64 = 0x1438795f0; const P6: u64 = 0x14240ab10;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rdi = r9; rbx = rdx; rsi = rcx; crate::simd_ops::wu32(rcx.wrapping_add(P0), (0x0u32) as u32); crate::simd_ops::wu16(rcx, (0x2u16) as u16); rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu16(rsi.wrapping_add(P2), ((rax as u16)) as u16); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsi.wrapping_add(P3), rax); r9 = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x8000008u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x0u8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r9); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0xeu32) as u32); rax = P5; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rdi, (xmm0) as u128); rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rsi; r11 = rsp.wrapping_add(0x80); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142486cb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x1426463f8; const P2: u64 = 0x2; const P3: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), (0x0u32) as u32);
    crate::simd_ops::wu16(rcx, (0x2u16) as u16);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu16(rbx.wrapping_add(P2), ((rax as u16)) as u16);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rbx.wrapping_add(P3), rax);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142486cf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x1426463f8; const P2: u64 = 0x2; const P3: u64 = 0x8; const P4: u64 = 0x142409b50; const P5: u64 = 0x14240ab10; const P6: u64 = 0x1413cc020; const P7: u64 = 0x1413cc060; const P8: u64 = 0x142486d70;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut xmm0: u128 = 0; let mut xmm1: u128 = 0; let mut xmm2: u128 = 0; let mut xmm3: u128 = 0; let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x142486cf0, funcinfo_rva: 0x37eb318, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x98, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x90u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x80u64), (xmm6) as u128); rbx = (rcx) as u64; xmm6 = (crate::simd_ops::ru128(rdx)) as u128; crate::simd_ops::wu32(rcx.wrapping_add(P0), (0x0u32) as u32); crate::simd_ops::wu16(rcx, (0x2u16) as u16); rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142486d17); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu16(rbx.wrapping_add(P2), ((rax as u16)) as u16); rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx.wrapping_add(P3), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x8000008u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((rax as u8)) as u8); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); r9 = (((xmm6 >> 64) as u64)) as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142486d4d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142486d58); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142486d62); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142486d6b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P8, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142486d90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x1426463f8; const P2: u64 = 0x2; const P3: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), (0x0u32) as u32);
    crate::simd_ops::wu16(rcx, (0x2u16) as u16);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu16(rbx.wrapping_add(P2), ((rax as u16)) as u16);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rbx.wrapping_add(P3), rax);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142486dc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x8; const P2: u64 = 0x24; const P3: u64 = 0x14; const P4: u64 = 0x20; const P5: u64 = 0x10; const P6: u64 = 0x142487ad0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    r9 = ((rcx.wrapping_add(P1)) as u64) as u64;
    rax = (crate::simd_ops::ru32(r9.wrapping_add(P2))) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rax as u32)) as u32);
    rcx = ((rcx).wrapping_add(0x10u64)) as u64;
    r8 = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    rax = (crate::simd_ops::ru32(r9.wrapping_add(P4))) as u32 as u64;
    rdx = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32);
    rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142486e00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x24; const P4: u64 = 0x20; const P5: u64 = 0x14; const P6: u64 = 0x142487ad0; const P7: u64 = 0x14340e528; const P8: u64 = 0x1401de830;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rbx = ((rcx.wrapping_add(P1)) as u64) as u64;
    rcx = ((rdx.wrapping_add(P2)) as u64) as u64;
    rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64;
    r9 = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rdx as u32)) as u32);
    crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r9 as u32)) as u32);
    r9 = rbx;
    r8 = (crate::simd_ops::ru32(rax.wrapping_add(P5))) as u32 as u64;
    rdx = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64;
    rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp);
    rax = P7;
    crate::simd_ops::wu64(rbx, rax);
    fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rbx.wrapping_add(P1), (0x0u8) as u8); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx.wrapping_add(P4), rax); }
    rcx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142486e80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x24; const P1: u64 = 0x20; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x142487ad0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rax = r8;
    r9 = rdx;
    r8 = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    rcx = ((rcx).wrapping_add(0x10u64)) as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((r8 as u32)) as u32);
    r8 = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    rdx = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r8 as u32)) as u32);
    r8 = (crate::simd_ops::ru32(rax.wrapping_add(P3))) as u32 as u64;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424873b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14340e528; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rax);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), (0x1u8) as u8);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142487510(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1433fa69c; const P1: u64 = 0x1427e54e0; const P2: u64 = 0x14340e544; const P3: u64 = 0x14340e548; const P4: u64 = 0x1433fa6a0; const P5: u64 = 0x14340e530; const P6: u64 = 0x1413cc0d0; const P7: u64 = 0x1413cc1a0; const P8: u64 = 0x142487692;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x142487510, funcinfo_rva: 0x37eb3d4, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x38, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = (0xffffffffffffffffu64) as u64; bb = 1; continue; }
            1 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rax))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 1; continue; } bb = 2; continue; }
            2 => { r10 = ((rcx.wrapping_sub(0x3u64)) as u64) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r8 = ((P0) as u64) as u64; r9 = (r10) as u64; fl_l = (r9) as u64 as u64; fl_r = (r8) as u64 as u64; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rbx = ((P1) as u64) as u64; bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru8(r9.wrapping_add(r8))) as u32) as u32 as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(rbx))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; r8 = ((r8.wrapping_add(0x1u64)) as u64) as u64; rax = ((crate::simd_ops::ru8(rax.wrapping_add(rbx))) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 5; continue; }
            5 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            7 => { r8 = ((P2) as u64) as u64; r9 = (r10) as u64; fl_l = (r9) as u64 as u64; fl_r = (r8) as u64 as u64; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 8; continue; }
            8 => { rax = ((crate::simd_ops::ru8(r9.wrapping_add(r8))) as u32) as u32 as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(rbx))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; r8 = ((r8.wrapping_add(0x1u64)) as u64) as u64; rax = ((crate::simd_ops::ru8(rax.wrapping_add(rbx))) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 10; continue; }
            10 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (0x2u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            12 => { r8 = ((P3) as u64) as u64; r9 = (r10) as u64; fl_l = (r9) as u64 as u64; fl_r = (r8) as u64 as u64; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 13; continue; }
            13 => { rax = ((crate::simd_ops::ru8(r9.wrapping_add(r8))) as u32) as u32 as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(rbx))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; r8 = ((r8.wrapping_add(0x1u64)) as u64) as u64; rax = ((crate::simd_ops::ru8(rax.wrapping_add(rbx))) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 15; continue; }
            15 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = (0x3u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            17 => { r8 = ((P4) as u64) as u64; r9 = (r10) as u64; fl_l = (r9) as u64 as u64; fl_r = (r8) as u64 as u64; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 18; continue; }
            18 => { rax = ((crate::simd_ops::ru8(r9.wrapping_add(r8))) as u32) as u32 as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(rbx))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; r8 = ((r8.wrapping_add(0x1u64)) as u64) as u64; rax = ((crate::simd_ops::ru8(rax.wrapping_add(rbx))) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 18; continue; } bb = 20; continue; }
            20 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 22; continue; } bb = 21; continue; }
            21 => { rax = (0x4u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            22 => { r8 = ((P5) as u64) as u64; fl_l = (r10) as u64 as u64; fl_r = (r8) as u64 as u64; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 23; continue; }
            23 => { rax = ((crate::simd_ops::ru8(r10.wrapping_add(r8))) as u32) as u32 as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(rbx))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; r8 = ((r8.wrapping_add(0x1u64)) as u64) as u64; rax = ((crate::simd_ops::ru8(rax.wrapping_add(rbx))) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 25; continue; } bb = 24; continue; }
            24 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 23; continue; } bb = 25; continue; }
            25 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 27; continue; } bb = 26; continue; }
            26 => { rax = (0x5u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            27 => { rdx = (rcx) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142487684); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14248768d); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P8, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424876a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142475680; const P1: u64 = 0x142474fb0; const P2: u64 = 0x14340e550; const P3: u64 = 0x1413cc0d0; const P4: u64 = 0x1413cc1a0; const P5: u64 = 0x1424876f5;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1424876a0, funcinfo_rva: 0x37eb3fc, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x58, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rbx = (rdx) as u64; rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(r9, ((rax as u32)) as u32); fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = (0x4u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { r8 = (r9) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424876bc); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (rbx) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            3 => { r8 = (r9) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424876cd); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (rbx) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rdx = ((P2) as u64) as u64; rcx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424876e7); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424876f0); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P5, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142487700(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x0u8) as u8);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142487720(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1401de830; const P2: u64 = 0x20; const P3: u64 = 0x18; const P4: u64 = 0x24; const P5: u64 = 0x10; const P6: u64 = 0x1413bb8a0; const P7: u64 = 0x14183acbc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rbx = rcx; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = ((rcx).wrapping_add(0x10u64)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx.wrapping_add(P2), rax); bb = 3; continue; }
            3 => { crate::simd_ops::wu8(rbx.wrapping_add(P0), (0x1u8) as u8); fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(P4))) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul((crate::simd_ops::ru32(rdi.wrapping_add(P2))) as i32) as u32) as u32 as u64; rdx = ((((rax as u32)) as i32 as i64) as u64) as u64; rcx = ((rbx.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); r8 = (((crate::simd_ops::ru32(rdi.wrapping_add(P4))) as i32 as i64) as u64) as u64; rax = (((crate::simd_ops::ru32(rdi.wrapping_add(P2))) as i32 as i64) as u64) as u64; r8 = (((r8) as i64).wrapping_mul((rax) as i64) as u64) as u64; r8 = ((r8).wrapping_shl(2u32)) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P2), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P4), ((rax as u32)) as u32); bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 6; continue; }
            6 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424877b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu32(rdx, (0xff000000u32) as u32);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424877c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1401de830; const P2: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rcx = ((rcx).wrapping_add(0x10u64)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx.wrapping_add(P2), rax); crate::simd_ops::wu8(rbx.wrapping_add(P0), (0x1u8) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x1u8) as u8);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142487800(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1401de830; const P2: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rcx = ((rcx).wrapping_add(0x10u64)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rbx.wrapping_add(P0), (0x0u8) as u8); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx.wrapping_add(P2), rax); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142487850(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14340e528; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rcx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rax);
    crate::simd_ops::wu8(rcx.wrapping_add(P4), (0x1u8) as u8);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142487880(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142487510; const P1: u64 = 0x142475680; const P2: u64 = 0x142474fb0; const P3: u64 = 0x14340ecd8; const P4: u64 = 0x142497fd0; const P5: u64 = 0x142498740; const P6: u64 = 0x142497c10; const P7: u64 = 0x143a08320; const P8: u64 = 0x142498af0; const P9: u64 = 0x142442de0; const P10: u64 = 0x14340e550; const P11: u64 = 0x1413cc0d0; const P12: u64 = 0x1413cc1a0; const P13: u64 = 0x14183acc2; const P14: u64 = 0x1423f3530; const P15: u64 = 0x1401df680; const P16: u64 = 0x1413d0070; const P17: u64 = 0x1413d2510; const P18: u64 = 0x142487a24;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 181>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x142487880, funcinfo_rva: 0x37eb4a4, handler: 1, shape: 2, frame_reg: 0, establisher_off: 0x5a8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((r9 as u8)) as u8); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x5a0u64)) as u64; rbx = (rdx) as u64; rdi = (rcx) as u64; rcx = (rdx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14248789b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rsp.wrapping_add(0x5c8u64), (0x0u32) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x4u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 2; continue; }
            2 => { r8 = ((rsp.wrapping_add(0x5c8u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424878c8); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 4; continue; }
            3 => { r8 = ((rsp.wrapping_add(0x5c8u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424878df); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rbx) as u64); rax = (0xffffffffffffffffu64) as u64; bb = 5; continue; }
            5 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(rax))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rax) as u64); rax = ((P3) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (rax) as u64); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x58u64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000020u32))) as u32 as u64; rax = ((((rax as u32)) | (0x20u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x5bu64), (0x20u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x5cu64), (0x0u32) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x60u64), (0x0u8) as u8); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); r8 = (0x1u32) as u32 as u64; rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142487949); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x142487949, (P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => l.unknown(0x142487880) }; rax = c_rax; xmm0 = c_x0; r8 = (((crate::simd_ops::ru32(rsp.wrapping_add(0x5c8u64))) as i32 as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142487961); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x142487961, (P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => l.unknown(0x142487880) }; rax = c_rax; xmm0 = c_x0; rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14248796b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x14248796b, (P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => l.unknown(0x142487880) }; rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(P7)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x168u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142487982); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x142487982, ({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0xa8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => l.unknown(0x142487880) }; rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142487991); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x142487991, (P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => l.unknown(0x142487880) }; rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424879a1); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x1424879a1, (P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => l.unknown(0x142487880) }; rax = c_rax; xmm0 = c_x0; bb = 9; continue; }
            9 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x5b0u64))) as u64; rsp = ((rsp).wrapping_add(0x5a0u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            10 => { rdx = ((P10) as u64) as u64; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424879c6); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424879cf); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 11; continue; }
            11 => { rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r8 = (0x400u32) as u32 as u64; rcx = ((rsp.wrapping_add(0x1a0u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424879e5); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (0x400u32) as u32 as u64; rcx = ((rsp.wrapping_add(0x1a0u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424879f7); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x1424879f7, (P14) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => l.unknown(0x142487880) }; rax = c_rax; xmm0 = c_x0; rcx = ((rsp.wrapping_add(0x58u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142487a01); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x142487a01, (P15) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => l.unknown(0x142487880) }; rax = c_rax; xmm0 = c_x0; rdx = (rax) as u64; r8 = ((rsp.wrapping_add(0x1a0u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142487a16); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x142487a16, (P16) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => l.unknown(0x142487880) }; rax = c_rax; xmm0 = c_x0; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142487a1f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x142487a1f, (P17) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => l.unknown(0x142487880) }; rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P18, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142487a30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14340e528; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x8; const P4: u64 = 0x20; const P5: u64 = 0x24; const P6: u64 = 0x1413bb8a0; const P7: u64 = 0x14183acbc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rbx = rcx; rax = P0; crate::simd_ops::wu64(rcx, rax); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx.wrapping_add(P1), rax); crate::simd_ops::wu64(rcx.wrapping_add(P2), rax); crate::simd_ops::wu8(rcx.wrapping_add(P3), ((rax as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(P4), rax); fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu8(rcx.wrapping_add(P3), (0x1u8) as u8); fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul((crate::simd_ops::ru32(rdx.wrapping_add(P5))) as i32) as u32) as u32 as u64; rdx = ((((rax as u32)) as i32 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(0x10u64)) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); r8 = (((crate::simd_ops::ru32(rdi.wrapping_add(P4))) as i32 as i64) as u64) as u64; rax = (((crate::simd_ops::ru32(rdi.wrapping_add(P5))) as i32 as i64) as u64) as u64; r8 = (((r8) as i64).wrapping_mul((rax) as i64) as u64) as u64; r8 = ((r8).wrapping_shl(2u32)) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P4), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P5), ((rax as u32)) as u32); bb = 3; continue; }
            3 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142487ad0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1418342b0; const P1: u64 = 0x14183acbc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r14 = (((crate::simd_ops::ru32(rcx.wrapping_add(0x20u64))) as i32 as i64) as u64) as u64; rbp = ((r8 as u32)) as u32 as u64; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x90u64))) as u32 as u64; rdi = rcx; r12 = (((crate::simd_ops::ru32(r9.wrapping_add(0x20u64))) as i32 as i64) as u64) as u64; rcx = rsp.wrapping_add(0x20); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); r15 = ((((rdx as u32)) as i32 as i64) as u64) as u64; rax = ((r14 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_sub((r15 as u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((r12 as u32)) as u32); rdx = rsp.wrapping_add(0x2c); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); rsi = r9; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rdx = rsp.wrapping_add(0x2c); rcx = rsp.wrapping_add(0x20); r13 = (((crate::simd_ops::ru32(rax)) as i32 as i64) as u64) as u64; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x98u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x24u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x24u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub((rbp as u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = 0; fl_l = 0; fl_r = 0; r8 = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; bb = 3; continue; }
            2 => { rdx = rbx; bb = 3; continue; }
            3 => { rax = ((r14 as u32)) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul(((rbp as u32)) as i32) as u32) as u32 as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(r15)) as u64; rdi = ((rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; bb = 5; continue; }
            5 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { rbp = r13; r14 = ((r14).wrapping_shl(2u32)) as u64; rbp = ((rbp).wrapping_shl(2u32)) as u64; r15 = r12; fl_l = ((r15).wrapping_shl(2u32)) as u64; r15 = ((fl_l as u64)) as u64; rsi = r8; bb = 7; continue; }
            7 => { r8 = rbp; rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdi = ((rdi).wrapping_add(r14)) as u64; rbx = ((rbx).wrapping_add(r15)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142487be0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14340e528; const P1: u64 = 0x8; const P2: u64 = 0x1401de830; const P3: u64 = 0x20; const P4: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = P0;
    crate::simd_ops::wu64(rcx, rax);
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rcx = ((rcx).wrapping_add(0x10u64)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rbx.wrapping_add(P1), (0x0u8) as u8); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx.wrapping_add(P3), rax); }
    rcx = ((rbx.wrapping_add(P4)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142487c30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14340e528; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x8; const P4: u64 = 0x20; const P5: u64 = 0x142487c80;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = P0;
    crate::simd_ops::wu64(rcx, rax);
    xmm0 = 0u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P1), (xmm0) as u128);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    crate::simd_ops::wu8(rcx.wrapping_add(P3), ((rax as u8)) as u8);
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rax);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142487c80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x24; const P2: u64 = 0x8; const P3: u64 = 0x1401de830; const P4: u64 = 0x18; const P5: u64 = 0x1c; const P6: u64 = 0x10; const P7: u64 = 0x140701ce0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = ((r8 as u32)) as u32 as u64; r12 = ((rdx as u32)) as u32 as u64; rbx = rcx; rsi = ((rcx.wrapping_add(P0)) as u64) as u64; r14 = ((rcx.wrapping_add(P1)) as u64) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rcx).wrapping_add(0x10u64)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsi, ((rax as u32)) as u32); crate::simd_ops::wu32(r14, ((rax as u32)) as u32); bb = 2; continue; }
            2 => { crate::simd_ops::wu8(rbx.wrapping_add(P2), (0x1u8) as u8); rdi = ((r12 as u32)) as u32 as u64; rdi = ((((rdi as u32)) as i32).wrapping_mul(((rbp as u32)) as i32) as u32) as u32 as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = ((rax as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = rdx; fl_r = rdi; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rdi } else { rdx }) as u64; rcx = ((rbx.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            4 => { fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu32(rbx.wrapping_add(P4), ((rdi as u32)) as u32); rcx = rdi; bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), (0xff000000u32) as u32); rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x58u64))) as u32 as u64; bb = 9; continue; }
            9 => { crate::simd_ops::wu32(rax, ((rdx as u32)) as u32); rax = ((rax).wrapping_add(0x4u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 10; continue; }
            10 => { rsi = ((rbx.wrapping_add(P0)) as u64) as u64; r14 = ((rbx.wrapping_add(P1)) as u64) as u64; bb = 11; continue; }
            11 => { crate::simd_ops::wu32(rsi, ((r12 as u32)) as u32); crate::simd_ops::wu32(r14, ((rbp as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142487db0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14101f7d0; const P2: u64 = 0x18; const P3: u64 = 0x10; const P4: u64 = 0x142488df0;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x80u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdi = rcx;
    rcx = rsp.wrapping_add(0x20);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    r8 = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64;
    r9 = rax;
    rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64;
    rsp = ((rsp).wrapping_add(0x80u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142487e40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426455a8; const P1: u64 = 0x38; const P2: u64 = 0x3c; const P3: u64 = 0x1; const P4: u64 = 0x30; const P5: u64 = 0x1413cef60; const P6: u64 = 0x8; const P7: u64 = 0xc; const P8: u64 = 0x10; const P9: u64 = 0x18; const P10: u64 = 0x1c; const P11: u64 = 0x20; const P12: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rdx; rdi = rcx; rcx = ((rcx).wrapping_add(0x40u64)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_add_u32(rdi, (1) as u32); r9 = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; r8 = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; rcx = ((r8.wrapping_add(r8.wrapping_mul(4))) as u64) as u64; rcx = ((rcx).wrapping_shl(3u32)) as u64; rax = ((r9.wrapping_add(r9.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(3u32)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((r9.wrapping_add(P3)) as u64) as u64; rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = ((rdi.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdx, rcx); crate::simd_ops::wu64(rdx.wrapping_add(P6), rcx); rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rdx, rax); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P6))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(P6), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P7))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(P7), ((rax as u32)) as u32); crate::simd_ops::wu64(rsi, rcx); crate::simd_ops::wu64(rsi.wrapping_add(P6), rcx); crate::simd_ops::wu64(rdx.wrapping_add(P8), rcx); crate::simd_ops::wu64(rdx.wrapping_add(P9), rcx); rax = (crate::simd_ops::ru64(rsi.wrapping_add(P8))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P8), rax); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P9))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(P9), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P10))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(P10), ((rax as u32)) as u32); crate::simd_ops::wu64(rsi.wrapping_add(P8), rcx); crate::simd_ops::wu64(rsi.wrapping_add(P9), rcx); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P11))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x20u64), ((rax as u32)) as u32); crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x38u64), (1) as u32); rcx = rdi.wrapping_add(0x40); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142487f20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a082b8; const P1: u64 = 0x142420c10; const P2: u64 = 0x14243d770; const P3: u64 = 0x1413aa840; const P4: u64 = 0x1424883c0; const P5: u64 = 0x14240a730; const P6: u64 = 0x14240ab10; const P7: u64 = 0x142489260;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 29>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x142487f20, funcinfo_rva: 0x37eb618, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0xe8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rsp.wrapping_sub(0x27u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0xb0u64)) as u64; r12 = (r8) as u64; r13 = (rdx) as u64; r15 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbp.wrapping_add(0x7fu64), ((r15 as u32)) as u32); rcx = (crate::simd_ops::ru64(P0)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), (rcx) as u64); rdi = (crate::simd_ops::ru64(r9.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru32(r9.wrapping_add(0x20u64))) as u32 as u64; rsi = ((rdi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rdi)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142487f77); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r14 = (rax) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142487f82); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = (rbx) as u64; rdx = ((rbp.wrapping_sub(0x31u64)) as u64) as u64; rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x60u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142487f92); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::ru8(rbp.wrapping_sub(0x21u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x31u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x49u64), (xmm0) as u128); rdx = ((rbp.wrapping_sub(0x49u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0x3u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142487faf); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (rax) as u64; fl_l = ((((r15 as u32)) | (0x1u32))) as u32 as u64; fl_r = fl_l; r15 = ((fl_l as u32)) as u32 as u64; bb = 5; continue; }
            4 => { crate::simd_ops::wu8(rbp.wrapping_sub(0x1u64), (0x0u8) as u8); rdx = ((rbp.wrapping_sub(0x19u64)) as u64) as u64; fl_l = ((((r15 as u32)) | (0x2u32))) as u32 as u64; fl_r = fl_l; r15 = ((fl_l as u32)) as u32 as u64; bb = 5; continue; }
            5 => { crate::simd_ops::wu32(rbp.wrapping_add(0x7fu64), ((r15 as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_sub(0x51u64), (0x0u8) as u8); fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x18u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = ((rbp.wrapping_sub(0x69u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x49u64), (rax) as u64); rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x69u64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000002u32))) as u32 as u64; rax = ((((rax as u32)) | (0x2u32))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x69u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_sub(0x66u64), (0x2u8) as u8); crate::simd_ops::wu32(rbp.wrapping_sub(0x65u64), (0x0u32) as u32); crate::simd_ops::wu8(rbp.wrapping_sub(0x61u64), (0x0u8) as u8); rcx = ((rbp.wrapping_sub(0x69u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142488000); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu8(rbp.wrapping_sub(0x51u64), (0x1u8) as u8); bb = 7; continue; }
            7 => { fl_l = ((r15 as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { r15 = ((((r15 as u32)) & (0xfffffffdu32))) as u32 as u64; fl_l = (crate::simd_ops::ru8(rbp.wrapping_sub(0x1u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = ((rbp.wrapping_sub(0x19u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14248801e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 10; continue; }
            10 => { fl_l = ((r15 as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 11; continue; }
            11 => { r15 = ((((r15 as u32)) & (0xfffffffeu32))) as u32 as u64; fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x1bu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = ((rbp.wrapping_add(0x3u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142488038); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_sub(0x51u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 19; continue; } bb = 14; continue; }
            14 => { xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x31u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x49u64), (xmm0) as u128); r9 = ((rbp.wrapping_sub(0x49u64)) as u64) as u64; r8 = (r12) as u64; rdx = (r13) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14248805b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::ru8(rbp.wrapping_sub(0x51u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = ((rbp.wrapping_sub(0x69u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14248806b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 16; continue; }
            16 => { rdi = ((rdi).wrapping_add(0x8u64)) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rsi) as u64 as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x39u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 17; continue; }
            17 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 18; continue; }
            18 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xf8u64))) as u64; rsp = ((rsp).wrapping_add(0xb0u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            19 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 18; continue; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424880b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401df850; const P1: u64 = 0x1424883c0; const P2: u64 = 0x142489260; const P3: u64 = 0x14240ab10;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1424880b0, funcinfo_rva: 0x37eb688, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x98, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rdi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = (rsp) as u64; rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rsi = (r9) as u64; r14 = (r8) as u64; r15 = (rdx) as u64; r12 = (rcx) as u64; rbx = (crate::simd_ops::ru64(r9.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru32(r9.wrapping_add(0x18u64))) as u32 as u64; rdi = (((rax) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rdx = ((rbx.wrapping_add(0x8u64)) as u64) as u64; rcx = ((crate::simd_ops::ru8(rbx.wrapping_add(0x3u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 5; continue; }
            4 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), (rdx) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x58u64), (rax) as u64); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x60u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x40u64), (xmm0) as u128); rdx = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142488150); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x30u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x40u64), (xmm0) as u128); rdx = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142488167); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::ru8(rbp.wrapping_sub(0x8u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 30; continue; } bb = 8; continue; }
            8 => { rdx = ((crate::simd_ops::ru8(rbp.wrapping_sub(0x1du64))) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = ((rdx.wrapping_mul(8)) as u32) as u32 as u64; bb = 11; continue; }
            10 => { rdx = (((rdx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x20u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; rcx = ((rbp.wrapping_sub(0x18u64)) as u64) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rcx = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rbp.wrapping_sub(0x18u64)) } else { rcx }) as u64; rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x1cu64))) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), (rcx) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), (rax) as u64); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x50u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x40u64), (xmm0) as u128); r9 = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; r8 = (r14) as u64; rdx = (r15) as u64; rcx = (r12) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424881cd); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::ru8(rbp.wrapping_sub(0x8u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = ((rbp.wrapping_sub(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424881dd); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 13; continue; }
            13 => { rbx = ((rbx).wrapping_add(0x108u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x28u64))) as u32 as u64; rdi = (((rax) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 28; continue; } bb = 15; continue; }
            15 => { bb = 16; continue; }
            16 => { rcx = ((crate::simd_ops::ru8(rbx.wrapping_add(0x3u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 18; continue; } bb = 17; continue; }
            17 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 19; continue; }
            18 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 19; continue; }
            19 => { rdx = ((rbx.wrapping_add(0x8u64)) as u64) as u64; rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 21; continue; } bb = 20; continue; }
            20 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), (rdx) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), (rax) as u64); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x50u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x30u64), (xmm0) as u128); rdx = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142488260); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x40u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x30u64), (xmm0) as u128); rdx = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142488277); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::ru8(rbp.wrapping_sub(0x8u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 30; continue; } bb = 22; continue; }
            22 => { rdx = ((crate::simd_ops::ru8(rbp.wrapping_sub(0x1du64))) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 24; continue; } bb = 23; continue; }
            23 => { rdx = ((rdx.wrapping_mul(8)) as u32) as u32 as u64; bb = 25; continue; }
            24 => { rdx = (((rdx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x20u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; rcx = ((rbp.wrapping_sub(0x18u64)) as u64) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rcx = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rbp.wrapping_sub(0x18u64)) } else { rcx }) as u64; rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x1cu64))) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), (rcx) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x58u64), (rax) as u64); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x60u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x30u64), (xmm0) as u128); r9 = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; r8 = (r14) as u64; rdx = (r15) as u64; rcx = (r12) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424882dd); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::ru8(rbp.wrapping_sub(0x8u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 27; continue; } bb = 26; continue; }
            26 => { rcx = ((rbp.wrapping_sub(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424882ed); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 27; continue; }
            27 => { rbx = ((rbx).wrapping_add(0x108u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 28; continue; }
            28 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 29; continue; }
            29 => { r11 = ((rsp.wrapping_add(0x80u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; r12 = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            30 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 29; continue; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142488330(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x1426455a8; const P2: u64 = 0x8; const P3: u64 = 0x1424880b0; const P4: u64 = 0x142487f20; const P5: u64 = 0x2c; const P6: u64 = 0x1; const P7: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x142488330, funcinfo_rva: 0x37eb6c0, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x38, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (r9) as u64; rbp = (r8) as u64; rdi = (rdx) as u64; r14 = (rcx) as u64; rbx = ((r9.wrapping_add(P0)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rbx) as u64); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14248835a); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = (crate::simd_ops::ru64(r9.wrapping_add(P2))) as u64; r8 = (rsi) as u64; rdx = (rbp) as u64; rcx = (r14) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { r9 = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14248837b); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 3; continue; }
            2 => { r9 = (crate::simd_ops::ru64(r9)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142488385); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 3; continue; }
            3 => { rax = (rax & !0xffu64) | ((((((rax as u8)) ^ (0x1u8))) as u8 as u64)); crate::simd_ops::wu8(rdi, ((rax as u8)) as u8); fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(P5))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; rax = (rax & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu8(rdi.wrapping_add(P6), ((rax as u8)) as u8); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14248839b); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P7)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424883c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1424badc0; const P1: u64 = 0x1401df850; const P2: u64 = 0x1401e01e0; const P3: u64 = 0x142408f50; const P4: u64 = 0x14240ab10; const P5: u64 = 0x143079458; const P6: u64 = 0x1426d01e0; const P7: u64 = 0x143078be8; const P8: u64 = 0x142491120; const P9: u64 = 0x14240aa00; const P10: u64 = 0x14240a730;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 67>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1424883c0, funcinfo_rva: 0x37eb700, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x218, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rsp.wrapping_sub(0xf0u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x1f0u64)) as u64; rbx = (rdx) as u64; r15 = (rcx) as u64; rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r14 = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rdi as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rdx)) as u128; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rcx = ((xmm0 as u64)) as u64; rax = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2fu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 58; continue; } bb = 2; continue; }
            2 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x5cu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 58; continue; } bb = 3; continue; }
            3 => { fl_l = (rdx) as u64 as u64; fl_r = (0x2u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x1u64))) as u8 as u64; fl_r = (0x3au8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 58; continue; } bb = 5; continue; }
            5 => { r13 = ((rsp.wrapping_add(0x38u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (0x1u64) as u64); crate::simd_ops::wu128(rbp.wrapping_sub(0x70u64), (xmm0) as u128); r12 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x68u64))) as u64; bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru32(r13)) as u32 as u64; rcx = ((rbp.wrapping_sub(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14248844a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (rax) as u64; r8 = ((rax.wrapping_add(0x8u64)) as u64) as u64; rcx = ((crate::simd_ops::ru8(rax.wrapping_add(0x3u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 9; continue; }
            8 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { r8 = (crate::simd_ops::ru64(r8)) as u64; bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), (r8) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), (rax) as u64); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x50u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424884a5); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x70u64))) as u64; fl_l = (r12) as u64 as u64; fl_r = (r12) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 12; continue; }
            12 => { rax = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2fu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 16; continue; } bb = 13; continue; }
            13 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x5cu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 16; continue; } bb = 14; continue; }
            14 => { fl_l = (r12) as u64 as u64; fl_r = (0x2u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 17; continue; } bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x1u64))) as u8 as u64; fl_r = (0x3au8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 17; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (rcx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (r12) as u64); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000002u32))) as u32 as u64; rax = ((((rax as u32)) | (0x2u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x23u64), (0x2u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((rdi as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x0u8) as u8); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142488508); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 34; continue; }
            17 => { fl_l = (r12) as u64 as u64; fl_r = (r12) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000002u32))) as u32 as u64; rax = ((((rax as u32)) | (0x2u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x23u64), (0x2u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((rdi as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x0u8) as u8); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14248854c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 34; continue; }
            19 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000002u32))) as u32 as u64; rax = ((((rax as u32)) | (0x2u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x23u64), (0x2u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((rdi as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x0u8) as u8); fl_l = ((((r14 as u32)) | (0x4u32))) as u32 as u64; fl_r = fl_l; r14 = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((r14 as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x40u64), (xmm0) as u128); crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rcx) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), (r12) as u64); rdx = (rdi) as u64; rsi = (rdi) as u64; rcx = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; r10 = (rdi) as u64; bb = 20; continue; }
            20 => { xmm0 = (crate::simd_ops::ru128(rcx)) as u128; r8 = (crate::simd_ops::ru64(rcx)) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = (r9) as u64 as u64; fl_r = (r9) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 21; continue; }
            21 => { rax = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2fu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 25; continue; } bb = 22; continue; }
            22 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x5cu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 25; continue; } bb = 23; continue; }
            23 => { fl_l = (r9) as u64 as u64; fl_r = (0x2u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 26; continue; } bb = 24; continue; }
            24 => { fl_l = (crate::simd_ops::ru8(r8.wrapping_add(0x1u64))) as u8 as u64; fl_r = (0x3au8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 26; continue; } bb = 25; continue; }
            25 => { rsi = (r10) as u64; rdx = (((xmm0 >> 64) as u64)) as u64; bb = 27; continue; }
            26 => { rax = (((xmm0 >> 64) as u64)) as u64; fl_l = ((rdx).wrapping_add(rax)) as u64 as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 27; continue; }
            27 => { r10 = ((r10).wrapping_add(1)) as u64; rcx = ((rcx).wrapping_add(0x10u64)) as u64; rax = ((rbp.wrapping_sub(0x20u64)) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 20; continue; } bb = 28; continue; }
            28 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 30; continue; } bb = 29; continue; }
            29 => { rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142488603); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 30; continue; }
            30 => { rax = (rsi) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rbx = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; rbx = ((rbx).wrapping_add(rax)) as u64; rdi = (0x2u32) as u32 as u64; rdi = ((rdi).wrapping_sub(rsi)) as u64; rdi = ((rdi).wrapping_shl(4u32)) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 33; continue; } bb = 31; continue; }
            31 => { bb = 32; continue; }
            32 => { xmm0 = (crate::simd_ops::ru128(rbx)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142488644); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 32; continue; } bb = 33; continue; }
            33 => { fl_l = ((((r14 as u32)) & (0xfffffffbu32))) as u32 as u64; fl_r = fl_l; r14 = ((fl_l as u32)) as u32 as u64; bb = 34; continue; }
            34 => { r14 = ((((r14 as u32)) | (0x2u32))) as u32 as u64; rcx = ((rbp.wrapping_sub(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14248865e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x23u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 36; continue; } bb = 35; continue; }
            35 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 37; continue; }
            36 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 37; continue; }
            37 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; r11 = ((rsp.wrapping_add(0x28u64)) as u64) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; r11 = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0x28u64)) } else { r11 }) as u64; rsi = (crate::simd_ops::ru32(rsp.wrapping_add(0x24u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(P5)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), (r11) as u64); rdi = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x58u64), (rsi) as u64); rcx = ((r11.wrapping_add(rsi)) as u64) as u64; rax = ((rsi.wrapping_add(r11)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (r11) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 45; continue; } bb = 38; continue; }
            38 => { r8 = (rax) as u64; bb = 39; continue; }
            39 => { r9 = (0x2u32) as u32 as u64; rdx = (rbx) as u64; r8 = ((crate::simd_ops::ru8(r8.wrapping_sub(0x1u64))) as u32) as u32 as u64; r10 = ((P6) as u64) as u64; r10 = ((crate::simd_ops::ru8(r8.wrapping_add(r10))) as u32) as u32 as u64; bb = 40; continue; }
            40 => { r9 = ((r9).wrapping_sub(1)) as u64; r8 = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; r12 = ((P6) as u64) as u64; fl_l = (crate::simd_ops::ru8(r8.wrapping_add(r12))) as u8 as u64; fl_r = ((r10 as u8)) as u8 as u64; r12 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x68u64))) as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 54; continue; } bb = 41; continue; }
            41 => { rdx = ((rdx).wrapping_add(1)) as u64; fl_l = (r9) as u64 as u64; fl_r = (r9) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 40; continue; } bb = 42; continue; }
            42 => { rcx = (rax) as u64; bb = 43; continue; }
            43 => { rax = ((rsi.wrapping_add(r11)) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 45; continue; } bb = 44; continue; }
            44 => { rax = ((rsi.wrapping_add(r11)) as u64) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rax) as u64 as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x58u64), (rdi) as u64); bb = 45; continue; }
            45 => { xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x60u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 50; continue; } bb = 46; continue; }
            46 => { fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 50; continue; } bb = 47; continue; }
            47 => { rax = ((crate::simd_ops::ru8(r11)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2fu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 49; continue; } bb = 48; continue; }
            48 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x5cu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 50; continue; } bb = 49; continue; }
            49 => { rax = ((P7) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (0x1u64) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; bb = 50; continue; }
            50 => { crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142488765); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 57; continue; } bb = 51; continue; }
            51 => { rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142488773); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r13 = ((r13).wrapping_add(0x4u64)) as u64; rax = ((rsp.wrapping_add(0x40u64)) as u64) as u64; fl_l = (r13) as u64 as u64; fl_r = (rax) as u64 as u64; rdi = (0x0u32) as u32 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 52; continue; }
            52 => { crate::simd_ops::wu8(r15.wrapping_add(0x18u64), (0x0u8) as u8); bb = 53; continue; }
            53 => { rax = (r15) as u64; r11 = ((rsp.wrapping_add(0x1f0u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x48u64))) as u64; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            54 => { rdx = ((rdx).wrapping_sub(rbx)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0xffffffffffffffffu64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 42; continue; } bb = 55; continue; }
            55 => { rax = ((rax).wrapping_sub(1)) as u64; r8 = (rax) as u64; rcx = ((rcx).wrapping_sub(1)) as u64; fl_l = (rax) as u64 as u64; fl_r = (r11) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 39; continue; } bb = 56; continue; }
            56 => { bb = 43; continue; }
            57 => { crate::simd_ops::wu64(rbp.wrapping_add(0x128u64), (r15) as u64); crate::simd_ops::lk_and_u32(r15, (0xff000002u32) as u32); crate::simd_ops::lk_or_u32(r15, (0x2u32) as u32); crate::simd_ops::wu8(r15.wrapping_add(0x3u64), (0x2u8) as u8); crate::simd_ops::wu32(r15.wrapping_add(0x4u64), (0x0u32) as u32); crate::simd_ops::wu8(r15.wrapping_add(0x8u64), (0x0u8) as u8); rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = (r15) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142488808); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu8(r15.wrapping_add(0x18u64), (0x1u8) as u8); rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142488818); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 53; continue; }
            58 => { crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142488832); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142488846); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 52; continue; } bb = 59; continue; }
            59 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x70u64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000002u32))) as u32 as u64; rax = ((((rax as u32)) | (0x2u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x73u64), (0x2u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x74u64), ((rdi as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x78u64), ((rdi as u8)) as u8); xmm0 = (crate::simd_ops::ru128(rbx)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142488885); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbp.wrapping_add(0x128u64), (r15) as u64); crate::simd_ops::lk_and_u32(r15, (0xff000002u32) as u32); crate::simd_ops::lk_or_u32(r15, (0x2u32) as u32); crate::simd_ops::wu8(r15.wrapping_add(0x3u64), (0x2u8) as u8); crate::simd_ops::wu32(r15.wrapping_add(0x4u64), ((rdi as u32)) as u32); crate::simd_ops::wu8(r15.wrapping_add(0x8u64), (0x0u8) as u8); rdx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; rcx = (r15) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424888b3); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu8(r15.wrapping_add(0x18u64), (0x1u8) as u8); rcx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424888c3); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 53; continue; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142488900(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1425371a0; const P1: u64 = 0x1413cea60; const P2: u64 = 0x14241c0a0; const P3: u64 = 0x14183acbc; const P4: u64 = 0x142481b20; const P5: u64 = 0x1413ced60; const P6: u64 = 0x14240ab10; const P7: u64 = 0x142442de0; const P8: u64 = 0x1413c6280; const P9: u64 = 0x14241c190; const P10: u64 = 0x142533200;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 91>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x142488900, funcinfo_rva: 0x37eb790, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x2d8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rsp.wrapping_sub(0x1b0u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x2b0u64)) as u64; rsi = (r8) as u64; r14 = (rdx) as u64; rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rdi as u32)) as u32); xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x38u64), (xmm0) as u128); rax = ((rsp.wrapping_add(0x38u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (r8) as u64); r15 = (r15 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru8(r8.wrapping_add(0x80u64))) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 9; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru64(r8.wrapping_add(0x50u64))) as u64; rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x48u64))) as u64; fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = (0x504b4d44u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 2; continue; }
            2 => { fl_l = ((((rdi as u32)) | (0x1u32))) as u32 as u64; fl_r = fl_l; rdi = ((fl_l as u32)) as u32 as u64; bb = 3; continue; }
            3 => { crate::simd_ops::wu8(r14.wrapping_add(0x10u64), (0x0u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rdi as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142488997); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r10 = ((rbx.wrapping_sub(0x1u64)) as u64) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64 as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = (rbx) as u64; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = ((r10.wrapping_add(0x108u64)) as u64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 25; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 24; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rbx, (rax) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), (rbx) as u64); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 27; continue; } bb = 7; continue; }
            7 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142488a00); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 27; continue; }
            8 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rax.wrapping_mul(8).wrapping_add(0x8u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (rax) as u64); rax = ((rdx.wrapping_add(0x8u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (rax) as u64); rax = ((rcx.wrapping_add(rdx)) as u64) as u64; r8 = ((r8).wrapping_sub(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r8) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm0) as u128); let y_addr: u64 = rsp.wrapping_add(0x60u64); xmm1 = core::ptr::read_unaligned(y_addr as *const u128); ymm1_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); let y_addr: u64 = rsp.wrapping_add(0x60u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm1); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm1_hi); crate::simd_ops::wu8(rbp.wrapping_sub(0x80u64), (0x1u8) as u8); rdx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x48u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142488a5d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r15 = (((rax as u8)) as u32) as u32 as u64; bb = 22; continue; }
            9 => { r13 = (crate::simd_ops::ru64(r8.wrapping_add(0x50u64))) as u64; rbx = (crate::simd_ops::ru64(r8.wrapping_add(0x48u64))) as u64; rdx = (0x10u32) as u32 as u64; rcx = (r13) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142488a7b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r12 = (rax) as u64; r8 = (r13) as u64; rdx = (rbx) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142488a8c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), (r13) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x68u64), (r12) as u64); rdx = ((rsi.wrapping_add(0x8u64)) as u64) as u64; rcx = ((crate::simd_ops::ru8(rsi.wrapping_add(0x3u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 12; continue; }
            11 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru32(rsi)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r13) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (r12) as u64); r8 = ((rsp.wrapping_add(0x60u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142488afd); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x1a0u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rdx = ((rbp.wrapping_sub(0x60u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x48u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142488b17); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r15 = (((rax as u8)) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x1a0u64))) as u32) as u32 as u64; bb = 16; continue; }
            16 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 20; continue; } bb = 17; continue; }
            17 => { rcx = ((rbp.wrapping_add(0xf0u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142488b32); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xd0u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142488b43); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 19; continue; }
            19 => { rcx = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142488b4d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rbp.wrapping_sub(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142488b56); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 21; continue; }
            20 => { rcx = ((rbp.wrapping_sub(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142488b61); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 21; continue; }
            21 => { rcx = (r12) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142488b6a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 22; continue; }
            22 => { rdi = ((((rdi as u32)) | (0x1u32))) as u32 as u64; fl_l = ((r15 as u8)) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(r14, (rax) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x8u64), (rax) as u64); crate::simd_ops::wu8(r14.wrapping_add(0x10u64), (0x1u8) as u8); bb = 35; continue; }
            24 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 26; continue; }
            25 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 26; continue; }
            26 => { rdx = (rcx) as u64; r9 = (rbx) as u64; rcx = (r10) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142488ba8); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 27; continue; }
            27 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 35; continue; } bb = 28; continue; }
            28 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142488bc4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r10 = ((rbx.wrapping_sub(0x1u64)) as u64) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64 as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = (rbx) as u64; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = ((r10.wrapping_add(0x108u64)) as u64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 33; continue; } bb = 29; continue; }
            29 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 32; continue; } bb = 30; continue; }
            30 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rbx, (rax) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), (rbx) as u64); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 35; continue; } bb = 31; continue; }
            31 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142488c21); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 35; continue; }
            32 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 34; continue; }
            33 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 34; continue; }
            34 => { rdx = (rcx) as u64; r9 = (rbx) as u64; rcx = (r10) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142488c39); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 35; continue; }
            35 => { rax = (r14) as u64; r11 = ((rsp.wrapping_add(0x2b0u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x48u64))) as u64; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142488c90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14241c190;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142488cb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x14241c0a0; const P3: u64 = 0x14183acbc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, r14);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = (crate::simd_ops::ru64(rdx)) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rsi = rdx;
    crate::simd_ops::wu64(rcx, rax);
    r14 = rcx;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    rcx = rdi;
    rdx = ((rax.wrapping_add(P1)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rdx = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64;
    r8 = rdi;
    rcx = rax;
    rbx = rax;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(r14, rdi);
    rax = r14;
    crate::simd_ops::wu64(r14.wrapping_add(P0), rbx);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    r14 = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142488d20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x50; const P1: u64 = 0x80; const P2: u64 = 0x20; const P3: u64 = 0x1424968a0; const P4: u64 = 0x18; const P5: u64 = 0x78; const P6: u64 = 0x30; const P7: u64 = 0x48; const P8: u64 = 0x40; const P9: u64 = 0x38; const P10: u64 = 0x10; const P11: u64 = 0x14240ab10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdi = rcx; rdx = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 2; continue; }
            2 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu8(rcx.wrapping_add(P1), ((rdx as u8)) as u8); bb = 5; continue; }
            5 => { fl_l = ((rcx).wrapping_add(0x48u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 7; continue; }
            6 => { fl_l = ((rcx).wrapping_add(0x18u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 7; continue; }
            7 => { rdx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rdx = ((rbx.wrapping_add(P4)) as u64) as u64; rcx = ((rdi.wrapping_add(P5)) as u64) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64; fl_r = rax; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { rdx } else { rcx }) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P5), rax); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P7))) as u64; r9 = ((crate::simd_ops::ru16(rbx.wrapping_add(P8))) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = (crate::simd_ops::ru64(rbx.wrapping_add(P9))) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P10))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rcx = rbx; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            11 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x48u64))) as u64; r9 = ((crate::simd_ops::ru16(rbx.wrapping_add(0x40u64))) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { rcx = rbx; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}
